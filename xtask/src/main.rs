//! Repository tasks: the review catalogue, README generation, package validation, fixture verification,
//! output comparison and the specification attribute inventory.
mod arguments;
mod compare_measurements;
mod compare_outputs;
mod digest;
mod inventory;
mod json;
mod layout_fixtures;
mod package;
mod readme;
mod review;
#[cfg(test)]
mod testing;
mod validate;

use arguments::Arguments;
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

const HELP: &str = "Usage: cargo xtask COMMAND [OPTIONS]

Commands:
  review                 Render the review catalogue to target/review/.
  readme                 Regenerate the README examples and their previews.
  validate PACKAGE...    Validate DOCX and PPTX parts against ECMA-376.
  inventory [--write]    Check or refresh the attribute inventory.
  verify-layout          Verify the frozen native layout captures.
  compare-outputs        Compare the models and packages of two executables.
  compare-measurements   Verify measurement fixtures and compare candidates.

Use cargo xtask COMMAND --help for the options of a command.
";

/// A command reports a failure that is not an error, such as a schema
/// violation or a comparison difference, through its exit code.
pub(crate) type Outcome = Result<ExitCode, String>;

pub(crate) fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lies inside the repository")
        .to_path_buf()
}

fn main() -> ExitCode {
    let mut arguments = Arguments::from_environment();
    let outcome = match arguments.next().as_deref() {
        Some("readme") => readme::run(arguments),
        Some("review") => review::run(arguments),
        Some("validate") => validate::run(arguments),
        Some("inventory") => inventory::run(arguments),
        Some("verify-layout") => layout_fixtures::run(arguments),
        Some("compare-outputs") => compare_outputs::run(arguments),
        Some("compare-measurements") => compare_measurements::run(arguments),
        None | Some("--help" | "-h" | "help") => {
            print!("{HELP}");
            Ok(ExitCode::SUCCESS)
        }
        Some(command) => Err(format!("unknown command: {command}; use --help")),
    };
    outcome.unwrap_or_else(|message| {
        eprintln!("{message}");
        ExitCode::FAILURE
    })
}
