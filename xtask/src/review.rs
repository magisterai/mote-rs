//! Generate the native review catalogue directly through the Mote library.
pub(crate) mod catalogue;
mod inputs;
mod publication;
mod rendering;

use crate::{Outcome, arguments::Arguments};
use catalogue::{Application, ReviewCase};
use std::{error::Error, path::PathBuf, process::ExitCode};

pub(crate) type ReviewResult<T> = Result<T, Box<dyn Error>>;

const HELP: &str = "Usage: cargo review [--case NAME] [--group NAME] [--output PATH] [--list]

Generate editable DOCX/PPTX, native PDF and native PNG pages at 144 DPI.
The default output root is target/review/, with concise/ and complete/ trees.
--case NAME selects one case; repeat it to select several. Omit for all cases.
--group NAME selects fundamentals, text, tables, flow or media.
--output PATH selects another review root.
--list lists the available case names without generating files.
No Office application is opened. Office visual review remains separate.
";

fn generate(arguments: Arguments) -> ReviewResult<()> {
    let cases = catalogue::load()?;
    let repository = crate::repository();
    let mut output = repository.join("target/review");
    let mut selected = Vec::new();
    let mut groups = Vec::new();
    let mut arguments = arguments;
    while let Some(argument) = arguments.next() {
        match Some(argument.as_str()) {
            Some("--help" | "-h") => {
                print!("{HELP}");
                return Ok(());
            }
            Some("--list") => {
                for case in &cases {
                    println!(
                        "{}: {}/{} [{}]",
                        case.identifier,
                        case.application.directory(),
                        case.name,
                        case.group
                    );
                }
                return Ok(());
            }
            Some("--case") => {
                let identifier = arguments.next().ok_or("--case requires a case name")?;
                let case = cases
                    .iter()
                    .find(|case| identifier == case.identifier.as_str())
                    .ok_or_else(|| format!("unknown case: {identifier}; use --list"))?;
                if !selected.contains(&case.identifier) {
                    selected.push(case.identifier.clone());
                }
            }
            Some("--group") => {
                let group = arguments.next().ok_or("--group requires a name")?;
                if !catalogue::GROUPS.contains(&group.as_str()) {
                    return Err(
                        "unknown group; use fundamentals, text, tables, flow or media".into(),
                    );
                }
                groups.push(group);
            }
            Some("--output") => {
                output = PathBuf::from(arguments.next().ok_or("--output requires a path")?)
            }
            _ => {
                return Err(format!("unknown option: {argument}; use --help").into());
            }
        }
    }
    let mut failures = Vec::new();
    let mut summaries = Vec::new();
    for case in cases.iter().filter(|case| {
        (selected.is_empty() || selected.contains(&case.identifier))
            && (groups.is_empty() || groups.contains(&case.group))
    }) {
        let result = publication::assert_case_closed(&output, case)
            .and_then(|()| rendering::generate(&repository, case))
            .and_then(|artifacts| publication::publish(&output, case, &artifacts));
        match result {
            Ok(()) => {
                println!(
                    "{}/{}: {} pages, {}, native PDF and PNG",
                    case.application.directory(),
                    case.name,
                    case.page_count,
                    case.application.package_format().to_uppercase()
                );
                summaries.push(serde_json::json!({"case":case.identifier,"status":"generated","pages":case.page_count}));
            }
            Err(error) => {
                eprintln!("{}: {error}", case.identifier);
                summaries.push(serde_json::json!({"case":case.identifier,"status":"failed","error":error.to_string()}));
                failures.push(case.identifier.clone());
            }
        }
    }
    if summaries.is_empty() {
        return Err("no cases match the selected filters".into());
    }
    std::fs::create_dir_all(output.join("complete"))?;
    std::fs::write(
        output.join("complete/last-run.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"office_review":"unreviewed","cases":summaries}),
        )?,
    )?;
    if !failures.is_empty() {
        return Err(format!(
            "{} review cases failed: {}",
            failures.len(),
            failures.join(", ")
        )
        .into());
    }
    println!("Review files: {}", output.join("concise").display());
    println!("Diagnostics: {}", output.join("complete").display());
    Ok(())
}

pub(crate) fn run(arguments: Arguments) -> Outcome {
    generate(arguments)
        .map(|()| ExitCode::SUCCESS)
        .map_err(|error| error.to_string())
}
