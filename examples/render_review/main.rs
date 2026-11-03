//! Generate the native review catalogue directly through the Mote library.
mod catalogue;
mod inputs;
mod publication;
mod rendering;

use catalogue::{Application, ReviewCase};
use std::{env, error::Error, path::PathBuf, process::ExitCode};

type ReviewResult<T> = Result<T, Box<dyn Error>>;

const HELP: &str = "Usage: cargo review [--case NAME] [--group NAME] [--output PATH] [--list]

Generate editable DOCX/PPTX, native PDF and native PNG pages at 144 DPI.
The default output root is target/review/, with concise/ and complete/ trees.
--case NAME selects one case; repeat it to select several. Omit for all cases.
--group NAME selects fundamentals, text, tables, flow or media.
--output PATH selects another review root.
--list lists the available case names without generating files.
No Office application is opened. Office visual review remains separate.
";

fn run() -> ReviewResult<()> {
    let cases = catalogue::load()?;
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut output = repository.join("target/review");
    let mut selected = Vec::new();
    let mut groups = Vec::new();
    let mut arguments = env::args_os().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.to_str() {
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
                    .ok_or_else(|| {
                        format!("unknown case: {}; use --list", identifier.to_string_lossy())
                    })?;
                if !selected.contains(&case.identifier) {
                    selected.push(case.identifier.clone());
                }
            }
            Some("--group") => {
                let group = arguments.next().ok_or("--group requires a name")?;
                let group = group
                    .to_str()
                    .filter(|group| catalogue::GROUPS.contains(group))
                    .ok_or("unknown group; use fundamentals, text, tables, flow or media")?;
                groups.push(group.to_owned());
            }
            Some("--output") => output = arguments.next().ok_or("--output requires a path")?.into(),
            _ => {
                return Err(
                    format!("unknown option: {}; use --help", argument.to_string_lossy()).into(),
                );
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

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
