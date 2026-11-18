//! Schema validation of the packages that Mote generates for the review cases,
//! the Office probes and the fixture sources. A golden capture compares generated
//! XML only with an earlier generation, so that a schema violation would
//! otherwise be recorded as an expectation.
use super::validate_package;
use crate::{
    captures, repository,
    review::catalogue::{self, Application},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Word,
    PowerPoint,
}

impl Format {
    fn extension(self) -> &'static str {
        match self {
            Self::Word => "docx",
            Self::PowerPoint => "pptx",
        }
    }
}

struct Subject {
    source: PathBuf,
    enable_lua: bool,
    /// A review case or a probe states its format and must be generated in it. A
    /// fixture source states none: both formats are attempted, and a format that
    /// refuses the source is passed over provided that the other accepts it.
    formats: &'static [Format],
}

struct Report {
    packages: usize,
    failures: Vec<String>,
}

fn sources(directory: &Path, recursive: bool, found: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(directory).unwrap_or_else(|error| panic!("{}: {error}", directory.display()));
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            if recursive {
                sources(&path, recursive, found);
            }
        } else if path
            .extension()
            .is_some_and(|extension| extension == "mote")
        {
            found.push(path);
        }
    }
}

fn subjects() -> Vec<Subject> {
    let root = repository();
    let mut subjects: Vec<_> = catalogue::load()
        .unwrap()
        .into_iter()
        .map(|case| Subject {
            source: root.join("tests/cases").join(&case.source),
            enable_lua: case.enable_lua,
            formats: match case.application {
                Application::Word => &[Format::Word],
                Application::PowerPoint => &[Format::PowerPoint],
            },
        })
        .collect();
    let mut probes = Vec::new();
    sources(&root.join("tests/office/probes"), false, &mut probes);
    probes.sort();
    assert!(!probes.is_empty(), "no Office probe was found");
    subjects.extend(probes.into_iter().map(|source| {
        // A probe of PowerPoint's layout names the application in its file name.
        let presentation = source
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with("powerpoint-"));
        Subject {
            source,
            enable_lua: false,
            formats: if presentation {
                &[Format::PowerPoint]
            } else {
                &[Format::Word]
            },
        }
    }));
    let fixtures = root.join("tests/fixtures");
    let mut fixture_sources = Vec::new();
    sources(&fixtures, true, &mut fixture_sources);
    fixture_sources.sort();
    assert!(!fixture_sources.is_empty(), "no fixture source was found");
    subjects.extend(fixture_sources.into_iter().map(|source| {
        let group = source
            .strip_prefix(&fixtures)
            .ok()
            .and_then(|relative| relative.components().next())
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .unwrap_or_default();
        Subject {
            source,
            enable_lua: captures::LUA_GROUPS.contains(&group.as_str()),
            formats: &[Format::Word, Format::PowerPoint],
        }
    }));
    subjects
}

fn validate(subject: &Subject) -> Report {
    let name = subject
        .source
        .strip_prefix(repository())
        .unwrap_or(&subject.source)
        .display()
        .to_string();
    let mut report = Report {
        packages: 0,
        failures: Vec::new(),
    };
    let mut refusals = Vec::new();
    for format in subject.formats {
        let origin = format!("{name} as {}", format.extension());
        let package = captures::render(
            &subject.source,
            subject.enable_lua,
            *format == Format::PowerPoint,
        );
        match package {
            Ok(bytes) => {
                report.packages += 1;
                match validate_package(&bytes, &origin) {
                    Ok(failures) => report.failures.extend(
                        failures
                            .into_iter()
                            .map(|failure| format!("{}: {}", failure.location, failure.error)),
                    ),
                    Err(error) => report.failures.push(error),
                }
            }
            Err(error) => refusals.push(format!("{origin} is not generated: {error}")),
        }
    }
    if report.packages == 0 || subject.formats.len() == 1 {
        report.failures.extend(refusals);
    }
    report
}

#[test]
fn generated_packages_conform_to_the_schemas() {
    if let Err(error) = Command::new("xmllint")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        panic!("xmllint is required to validate the generated packages: {error}");
    }
    let subjects = subjects();
    let next = AtomicUsize::new(0);
    let workers = thread::available_parallelism().map_or(4, usize::from);
    let reports: Vec<Report> = thread::scope(|scope| {
        let workers: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut reports = Vec::new();
                    while let Some(subject) = subjects.get(next.fetch_add(1, Ordering::Relaxed)) {
                        reports.push(validate(subject));
                    }
                    reports
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().expect("validation does not panic"))
            .collect()
    });
    let packages: usize = reports.iter().map(|report| report.packages).sum();
    let mut failures: Vec<_> = reports
        .into_iter()
        .flat_map(|report| report.failures)
        .collect();
    failures.sort();
    println!(
        "{packages} packages of {} sources were validated",
        subjects.len()
    );
    assert!(
        failures.is_empty(),
        "{} schema errors in {packages} generated packages:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
