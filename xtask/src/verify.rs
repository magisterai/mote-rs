//! Runs every check that a change must pass before it is pushed: the formatting
//! check, the lints, and the tests of the library and of the repository tasks.
use crate::{
    Outcome,
    arguments::{Arguments, unknown},
    repository,
};
use std::{
    io::{BufRead, BufReader},
    process::{Command, ExitCode, Stdio},
    time::{Duration, Instant},
};

const HELP: &str = "Usage: cargo xtask verify

Run, in order, the formatting check, the lints and every test of the workspace:
  cargo fmt --all --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace --no-fail-fast
A check runs even if an earlier one failed. A summary follows the last check,
and the exit status is nonzero unless every check passed. The command has no
option that omits a check.
";

const CHECKS: [&[&str]; 3] = [
    &["fmt", "--all", "--check"],
    &[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ],
    &["test", "--workspace", "--no-fail-fast"],
];

/// The totals of the result lines that the test harness prints, one for each
/// test binary and one for the documentation tests of each library.
#[derive(Debug, Default, PartialEq)]
struct Tally {
    runs: usize,
    passed: usize,
    failed: usize,
    ignored: usize,
    failures: Vec<String>,
}

impl Tally {
    fn read_line(&mut self, line: &str) {
        if let Some(name) = line
            .strip_prefix("test ")
            .and_then(|rest| rest.strip_suffix(" ... FAILED"))
        {
            self.failures.push(name.to_owned());
            return;
        }
        let Some(counts) = line.strip_prefix("test result: ") else {
            return;
        };
        self.runs += 1;
        for part in counts.split(';') {
            let mut words = part.split_whitespace().rev();
            let (Some(label), Some(count)) = (words.next(), words.next()) else {
                continue;
            };
            let Ok(count) = count.parse::<usize>() else {
                continue;
            };
            match label {
                "passed" => self.passed += count,
                "failed" => self.failed += count,
                "ignored" => self.ignored += count,
                _ => {}
            }
        }
    }
}

struct Report {
    command: String,
    passed: bool,
    elapsed: Duration,
    tally: Tally,
}

fn run_check(cargo_arguments: &[&str]) -> Result<Report, String> {
    let command = format!("cargo {}", cargo_arguments.join(" "));
    let failure = |error: std::io::Error| format!("{command}: {error}");
    println!("\n$ {command}");
    let started = Instant::now();
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut child = Command::new(cargo)
        .args(cargo_arguments)
        .current_dir(repository())
        // A capture harness that finds this variable writes its output in place of
        // a failing capture and passes, so the variable would defeat the check.
        .env_remove("MOTE_REFRESH_CAPTURES")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(failure)?;
    let mut output = BufReader::new(child.stdout.take().expect("the output is piped"));
    let mut tally = Tally::default();
    let mut bytes = Vec::new();
    while output.read_until(b'\n', &mut bytes).map_err(failure)? > 0 {
        let line = String::from_utf8_lossy(&bytes);
        let line = line.trim_end_matches(['\n', '\r']);
        println!("{line}");
        tally.read_line(line);
        bytes.clear();
    }
    let passed = child.wait().map_err(failure)?.success();
    Ok(Report {
        command,
        passed,
        elapsed: started.elapsed(),
        tally,
    })
}

fn summary(reports: &[Report]) -> String {
    let mut text = String::from("\nVerification summary\n");
    for report in reports {
        text += &format!(
            "  {}  {} ({:.0} s)",
            if report.passed { "passed" } else { "FAILED" },
            report.command,
            report.elapsed.as_secs_f64()
        );
        let tally = &report.tally;
        if tally.runs > 0 {
            text += &format!(
                ": {} tests passed, {} failed, {} ignored in {} test runs",
                tally.passed, tally.failed, tally.ignored, tally.runs
            );
        }
        text.push('\n');
        for name in &tally.failures {
            text += &format!("    failed: {name}\n");
        }
    }
    text += if reports.iter().all(|report| report.passed) {
        "The verification passed.\n"
    } else {
        "The verification FAILED.\n"
    };
    text
}

pub(crate) fn run(mut arguments: Arguments) -> Outcome {
    if let Some(argument) = arguments.next() {
        return match argument.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                Ok(ExitCode::SUCCESS)
            }
            option => Err(unknown(option)),
        };
    }
    let reports = CHECKS
        .iter()
        .map(|check| run_check(check))
        .collect::<Result<Vec<_>, _>>()?;
    print!("{}", summary(&reports));
    Ok(if reports.iter().all(|report| report.passed) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tally_adds_the_result_lines_of_every_test_run() {
        let mut tally = Tally::default();
        for line in [
            "running 3 tests",
            "test tests::first ... ok",
            "test tests::second ... FAILED",
            "test tests::third ... ignored",
            "test result: FAILED. 1 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.52s",
            "test result: ok. 40 passed; 0 failed; 2 ignored; 0 measured; 5 filtered out; finished in 1.00s",
        ] {
            tally.read_line(line);
        }
        assert_eq!(
            tally,
            Tally {
                runs: 2,
                passed: 41,
                failed: 1,
                ignored: 3,
                failures: vec!["tests::second".to_owned()],
            }
        );
    }

    #[test]
    fn a_summary_fails_unless_every_check_passed() {
        let report = |command: &str, passed| Report {
            command: command.to_owned(),
            passed,
            elapsed: Duration::from_secs(2),
            tally: Tally::default(),
        };
        let passing = summary(&[report("cargo fmt", true), report("cargo test", true)]);
        assert!(passing.ends_with("The verification passed.\n"), "{passing}");
        let failing = summary(&[report("cargo fmt", false), report("cargo test", true)]);
        assert!(failing.contains("  FAILED  cargo fmt (2 s)\n"), "{failing}");
        assert!(
            failing.contains("  passed  cargo test (2 s)\n"),
            "{failing}"
        );
        assert!(failing.ends_with("The verification FAILED.\n"), "{failing}");
    }
}
