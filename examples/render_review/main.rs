//! Generate the native review catalogue directly through the Mote library.
mod publication;
mod rendering;

use mote::render::LayoutMode;
use std::{env, error::Error, path::PathBuf, process::ExitCode};

type ReviewResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, Copy)]
enum Application {
    Word,
    PowerPoint,
}

impl Application {
    fn directory(self) -> &'static str {
        match self {
            Self::Word => "word",
            Self::PowerPoint => "powerpoint",
        }
    }

    fn package_format(self) -> &'static str {
        match self {
            Self::Word => "docx",
            Self::PowerPoint => "pptx",
        }
    }

    fn layout(self) -> LayoutMode {
        match self {
            Self::Word => LayoutMode::Document,
            Self::PowerPoint => LayoutMode::Presentation,
        }
    }

    fn layout_name(self) -> &'static str {
        match self {
            Self::Word => "document",
            Self::PowerPoint => "presentation",
        }
    }

    fn review_status(self) -> &'static str {
        match self {
            Self::Word => "word_status",
            Self::PowerPoint => "powerpoint_reference_status",
        }
    }
}

struct ReviewCase {
    identifier: &'static str,
    application: Application,
    name: &'static str,
    source_name: &'static str,
    page_count: usize,
}

const CASES: &[ReviewCase] = &[
    ReviewCase {
        identifier: "markdown-table-document",
        application: Application::Word,
        name: "01_markdown_table_alignment",
        source_name: "markdown-table-alignment",
        page_count: 1,
    },
    ReviewCase {
        identifier: "markdown-table-slides",
        application: Application::PowerPoint,
        name: "01_markdown_table_alignment",
        source_name: "markdown-table-alignment",
        page_count: 1,
    },
    ReviewCase {
        identifier: "slide-table-alignment",
        application: Application::PowerPoint,
        name: "02_table_alignment",
        source_name: "slide-table-alignment",
        page_count: 2,
    },
    ReviewCase {
        identifier: "slide-table-spacing",
        application: Application::PowerPoint,
        name: "03_table_spacing",
        source_name: "slide-table-spacing",
        page_count: 2,
    },
    ReviewCase {
        identifier: "slide-table-margins",
        application: Application::PowerPoint,
        name: "04_table_margins",
        source_name: "slide-table-margins",
        page_count: 1,
    },
];

const HELP: &str = "Usage: cargo review [--case NAME] [--output PATH] [--list]

Generate editable DOCX/PPTX, native PDF and native PNG pages at 144 DPI.
The default output root is target/review/, with concise/ and complete/ trees.
--case NAME selects one case; repeat it to select several. Omit for all cases.
--output PATH selects another review root.
--list lists the available case names without generating files.
No Office application is opened. Office visual review remains separate.
";

fn run() -> ReviewResult<()> {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut output = repository.join("target/review");
    let mut selected = Vec::new();
    let mut arguments = env::args_os().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.to_str() {
            Some("--help" | "-h") => {
                print!("{HELP}");
                return Ok(());
            }
            Some("--list") => {
                for case in CASES {
                    println!(
                        "{}: {}/{}",
                        case.identifier,
                        case.application.directory(),
                        case.name
                    );
                }
                return Ok(());
            }
            Some("--case") => {
                let identifier = arguments.next().ok_or("--case requires a case name")?;
                let case = CASES
                    .iter()
                    .find(|case| identifier == case.identifier)
                    .ok_or_else(|| {
                        format!("unknown case: {}; use --list", identifier.to_string_lossy())
                    })?;
                if !selected.contains(&case.identifier) {
                    selected.push(case.identifier);
                }
            }
            Some("--output") => output = arguments.next().ok_or("--output requires a path")?.into(),
            _ => {
                return Err(
                    format!("unknown option: {}; use --help", argument.to_string_lossy()).into(),
                );
            }
        }
    }
    for case in CASES
        .iter()
        .filter(|case| selected.is_empty() || selected.contains(&case.identifier))
    {
        publication::assert_case_closed(&output, case)?;
        let artifacts = rendering::generate(&repository, case)?;
        publication::publish(&output, case, &artifacts)?;
        println!(
            "{}/{}: {} pages, {}, native PDF and PNG",
            case.application.directory(),
            case.name,
            case.page_count,
            case.application.package_format().to_uppercase()
        );
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
