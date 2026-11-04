//! Compares Mote's native layout with the layout of Microsoft Word and PowerPoint.
//!
//! A reference records the text lines of an Office PDF export of a review case or
//! a probe. An allowance records how far the native layout may currently lie from
//! its reference. The test below holds every native layout within its allowance on
//! any platform; recording a reference requires Office on macOS.
mod export;
pub(crate) mod lines;
mod matching;

use crate::{Outcome, arguments::Arguments, digest, repository, review::catalogue};
use export::Application;
use lines::{Line, Page};
use matching::Comparison;
use mote::{evaluation::EvaluationOptions, render, source::FileAssetResolver};
use serde_json::{Map, Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

const HELP: &str = "Usage: cargo xtask office-compare [--case NAME]... [--probe NAME]... [--record] [--allow] [--detail]

Compare the native layout of review cases and probes with their recorded Word or
PowerPoint references in tests/office/references/, and report each subject whose
line positions exceed its allowance in tests/office/allowances.json.
--case NAME selects a review case by identifier; --probe NAME selects a probe.
Without a selection, every case and probe is selected.
--record exports the selected subjects through Word or PowerPoint (macOS only)
and replaces their references.
--allow replaces the selected subjects' allowances with their present results.
--detail lists unmatched lines and every line more than half a point from Office.
";

/// A small margin above an allowance absorbs floating-point noise.
const SLACK: f64 = 0.05;

/// A review case or a probe: a source laid out natively and by an Office
/// application.
pub(crate) struct Subject {
    pub key: String,
    application: Application,
    source: PathBuf,
    enable_lua: bool,
}

pub(crate) fn subjects() -> Result<Vec<(String, Subject)>, String> {
    let root = repository();
    let mut subjects = Vec::new();
    for case in catalogue::load().map_err(|error| error.to_string())? {
        let application = match case.application {
            catalogue::Application::Word => Application::Word,
            catalogue::Application::PowerPoint => Application::PowerPoint,
        };
        subjects.push((
            case.identifier,
            Subject {
                key: format!("{}/{}", case.application.directory(), case.name),
                application,
                source: root.join("tests/cases").join(&case.source),
                enable_lua: case.enable_lua,
            },
        ));
    }
    let probes = root.join("tests/office/probes");
    let mut names: Vec<_> = fs::read_dir(&probes)
        .map_err(|error| format!("{}: {error}", probes.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "mote")
        })
        .collect();
    names.sort();
    for path in names {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        subjects.push((
            format!("probe:{name}"),
            Subject {
                key: format!("probes/{name}"),
                application: Application::Word,
                source: path,
                enable_lua: false,
            },
        ));
    }
    Ok(subjects)
}

/// Applies `work` to every subject on all available cores and returns the results
/// in the order of the subjects.
pub(crate) fn in_parallel<T: Send>(
    subjects: &[&Subject],
    work: impl Fn(&Subject) -> T + Sync,
) -> Vec<T> {
    let next = std::sync::atomic::AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(4, usize::from);
    let mut results: Vec<(usize, T)> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(|| {
                    let mut done = Vec::new();
                    loop {
                        let index = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(subject) = subjects.get(index) else {
                            break done;
                        };
                        done.push((index, work(subject)));
                    }
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().expect("a comparison thread panicked"))
            .collect()
    });
    results.sort_by_key(|(index, _)| *index);
    results.into_iter().map(|(_, result)| result).collect()
}

pub(crate) struct Native {
    pub package: Vec<u8>,
    pub plan: mote::layout::DocumentTextPlan,
}

/// Compiles a subject and returns its editable package and native layout.
pub(crate) fn native(subject: &Subject) -> Result<Native, String> {
    let source = fs::read_to_string(&subject.source)
        .map_err(|error| format!("{}: {error}", subject.source.display()))?;
    let resolver = FileAssetResolver::new(repository()).map_err(|error| error.to_string())?;
    let name = subject.source.as_os_str();
    let document = mote::compile_with_options(
        &source,
        EvaluationOptions {
            asset_resolver: Some(&resolver),
            source_name: Some(name),
            enable_lua: subject.enable_lua,
            ..Default::default()
        },
    )
    .map_err(|error| format!("{}: {}", subject.key, error.message))?;
    let failure = |error: &dyn std::fmt::Display| format!("{}: {error}", subject.key);
    let (package, layout) = match subject.application {
        Application::Word => (
            mote::docx::emit_with_assets(&document, &resolver, Some(name))
                .map_err(|error| failure(&error))?
                .bytes,
            render::LayoutMode::Document,
        ),
        Application::PowerPoint => (
            mote::pptx::emit_with_assets(&document, &resolver, Some(name))
                .map_err(|error| failure(&error))?
                .bytes,
            render::LayoutMode::Presentation,
        ),
    };
    let plan = render::plan_with_layout(&document, &resolver, Some(name), layout)
        .map_err(|error| failure(&error))?
        .output;
    Ok(Native { package, plan })
}

fn native_pages(native: &Native, key: &str) -> Result<Vec<Page>, String> {
    let pdf = render::paint_pdf(&native.plan).map_err(|error| format!("{key}: {error}"))?;
    lines::read(pdf)
}

fn rounded(value: f64) -> Value {
    json!((value * 100.0).round() / 100.0)
}

fn reference_path(subject: &Subject) -> PathBuf {
    repository()
        .join("tests/office/references")
        .join(format!("{}.json", subject.key))
}

struct Reference {
    source_sha256: String,
    pages: Vec<Page>,
}

fn read_reference(path: &Path) -> Result<Reference, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    let number = |value: &Value| value.as_f64().unwrap_or_default();
    let pages = value["pages"]
        .as_array()
        .ok_or("reference without pages")?
        .iter()
        .map(|page| Page {
            width: number(&page["width"]),
            height: number(&page["height"]),
            lines: page["lines"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|line| Line {
                    text: line["text"].as_str().unwrap_or_default().to_owned(),
                    x: number(&line["x"]),
                    baseline: number(&line["baseline"]),
                })
                .collect(),
        })
        .collect();
    Ok(Reference {
        source_sha256: value["source_sha256"]
            .as_str()
            .unwrap_or_default()
            .to_owned(),
        pages,
    })
}

fn record(subject: &Subject) -> Result<(), String> {
    let native = native(subject)?;
    let stem = subject.key.replace('/', "-");
    let pdf = subject.application.export(&stem, &native.package)?;
    let pages = lines::read(pdf)?;
    let source = fs::read(&subject.source).map_err(|error| error.to_string())?;
    let value = json!({
        "source_sha256": digest::digest(&source),
        "renderer": subject.application.version()?,
        "pages": pages.iter().map(|page| json!({
            "width": rounded(page.width),
            "height": rounded(page.height),
            "lines": page.lines.iter().map(|line| json!({
                "text": line.text,
                "x": rounded(line.x),
                "baseline": rounded(line.baseline),
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    });
    let path = reference_path(subject);
    fs::create_dir_all(path.parent().unwrap()).map_err(|error| error.to_string())?;
    fs::write(&path, serde_json::to_string_pretty(&value).unwrap() + "\n")
        .map_err(|error| format!("{}: {error}", path.display()))
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Allowance {
    page_moves: usize,
    unmatched: usize,
    vertical: f64,
    median: f64,
    horizontal: f64,
}

impl Allowance {
    fn measured(comparison: &Comparison) -> Self {
        let ceiling = |value: f64| (value * 10.0).ceil() / 10.0;
        Self {
            page_moves: comparison.page_moves,
            unmatched: comparison.unmatched,
            vertical: ceiling(comparison.maximum_vertical),
            median: ceiling(comparison.median_vertical),
            horizontal: ceiling(comparison.maximum_horizontal),
        }
    }

    fn exceeded_by(&self, comparison: &Comparison) -> Vec<String> {
        let mut failures = Vec::new();
        if comparison.office_pages != comparison.native_pages {
            failures.push(format!(
                "{} Office pages, {} native",
                comparison.office_pages, comparison.native_pages
            ));
        }
        if comparison.page_moves > self.page_moves {
            failures.push(format!("{} lines on another page", comparison.page_moves));
        }
        if comparison.unmatched > self.unmatched {
            failures.push(format!("{} unmatched lines", comparison.unmatched));
        }
        for (name, measured, allowed) in [
            ("vertical", comparison.maximum_vertical, self.vertical),
            ("median", comparison.median_vertical, self.median),
            ("horizontal", comparison.maximum_horizontal, self.horizontal),
        ] {
            if measured > allowed + SLACK {
                failures.push(format!("{name} {measured:.2}pt exceeds {allowed:.1}pt"));
            }
        }
        failures
    }
}

fn allowances_path() -> PathBuf {
    repository().join("tests/office/allowances.json")
}

fn read_allowances() -> Result<BTreeMap<String, Allowance>, String> {
    let path = allowances_path();
    let Ok(text) = fs::read_to_string(&path) else {
        return Ok(BTreeMap::new());
    };
    let value: Map<String, Value> =
        serde_json::from_str(&text).map_err(|error| error.to_string())?;
    Ok(value
        .into_iter()
        .map(|(key, entry)| {
            let number = |name: &str| entry[name].as_f64().unwrap_or_default();
            (
                key,
                Allowance {
                    page_moves: number("page_moves") as usize,
                    unmatched: number("unmatched") as usize,
                    vertical: number("vertical"),
                    median: number("median"),
                    horizontal: number("horizontal"),
                },
            )
        })
        .collect())
}

fn write_allowances(allowances: &BTreeMap<String, Allowance>) -> Result<(), String> {
    let value: Map<String, Value> = allowances
        .iter()
        .map(|(key, allowance)| {
            (
                key.clone(),
                json!({
                    "page_moves": allowance.page_moves,
                    "unmatched": allowance.unmatched,
                    "vertical": allowance.vertical,
                    "median": allowance.median,
                    "horizontal": allowance.horizontal,
                }),
            )
        })
        .collect();
    fs::write(
        allowances_path(),
        serde_json::to_string_pretty(&value).unwrap() + "\n",
    )
    .map_err(|error| error.to_string())
}

/// Compares a subject's native layout with its reference, which must have been
/// recorded from the present source.
fn compare(subject: &Subject) -> Result<Option<Comparison>, String> {
    let path = reference_path(subject);
    if !path.is_file() {
        return Ok(None);
    }
    let reference = read_reference(&path)?;
    let source = fs::read(&subject.source).map_err(|error| error.to_string())?;
    if digest::digest(&source) != reference.source_sha256 {
        return Err(format!(
            "{}: the source changed after its Office reference was recorded; record it again",
            subject.key
        ));
    }
    let pages = native_pages(&native(subject)?, &subject.key)?;
    Ok(Some(matching::compare(&reference.pages, &pages)))
}

pub(crate) fn run(mut arguments: Arguments) -> Outcome {
    let all = subjects()?;
    let mut selected = Vec::new();
    let (mut recording, mut allowing, mut detailed) = (false, false, false);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(ExitCode::SUCCESS);
            }
            "--case" | "--probe" => {
                let name = arguments.value(&argument)?;
                let identifier = if argument == "--probe" {
                    format!("probe:{name}")
                } else {
                    name
                };
                if !all.iter().any(|(candidate, _)| *candidate == identifier) {
                    return Err(format!("unknown subject: {identifier}"));
                }
                selected.push(identifier);
            }
            "--record" => recording = true,
            "--allow" => allowing = true,
            "--detail" => detailed = true,
            other => return Err(format!("unknown option: {other}; use --help")),
        }
    }
    let subjects: Vec<&Subject> = all
        .iter()
        .filter(|(identifier, _)| selected.is_empty() || selected.contains(identifier))
        .map(|(_, subject)| subject)
        .collect();
    if recording {
        for subject in &subjects {
            record(subject)?;
            println!("recorded {}", subject.key);
        }
    }
    let mut allowances = read_allowances()?;
    let mut failed = false;
    println!(
        "subject | pages | moved | unmatched | vertical | median | horizontal | first difference"
    );
    let comparisons = in_parallel(&subjects, compare);
    for (subject, comparison) in subjects.iter().zip(comparisons) {
        let Some(comparison) = comparison? else {
            println!("{} | no reference", subject.key);
            continue;
        };
        println!(
            "{} | {}/{} | {} | {} | {:.2} | {:.2} | {:.2} | {}",
            subject.key,
            comparison.office_pages,
            comparison.native_pages,
            comparison.page_moves,
            comparison.unmatched,
            comparison.maximum_vertical,
            comparison.median_vertical,
            comparison.maximum_horizontal,
            comparison.first_difference.as_deref().unwrap_or("")
        );
        if detailed {
            for detail in &comparison.details {
                println!("  {detail}");
            }
        }
        if allowing {
            allowances.insert(subject.key.clone(), Allowance::measured(&comparison));
            continue;
        }
        let failures = allowances.get(&subject.key).map_or_else(
            || vec!["no allowance".to_owned()],
            |allowance| allowance.exceeded_by(&comparison),
        );
        if !failures.is_empty() {
            failed = true;
            println!("  exceeds its allowance: {}", failures.join("; "));
        }
    }
    if allowing {
        write_allowances(&allowances)?;
    }
    Ok(if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_recorded_subject_stays_within_its_office_allowance() {
        let allowances = read_allowances().unwrap();
        let mut failures = Vec::new();
        let mut compared = 0;
        let all = subjects().unwrap();
        let selected: Vec<&Subject> = all.iter().map(|(_, subject)| subject).collect();
        for (subject, comparison) in selected.iter().zip(in_parallel(&selected, compare)) {
            let Some(comparison) = comparison.unwrap() else {
                continue;
            };
            compared += 1;
            let exceeded = allowances.get(&subject.key).map_or_else(
                || vec!["no allowance".to_owned()],
                |allowance| allowance.exceeded_by(&comparison),
            );
            if !exceeded.is_empty() {
                failures.push(format!("{}: {}", subject.key, exceeded.join("; ")));
            }
        }
        assert!(compared > 0, "no Office references are recorded");
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn every_allowance_belongs_to_a_recorded_subject() {
        let keys: Vec<String> = subjects()
            .unwrap()
            .into_iter()
            .filter(|(_, subject)| reference_path(subject).is_file())
            .map(|(_, subject)| subject.key)
            .collect();
        for key in read_allowances().unwrap().keys() {
            assert!(keys.contains(key), "{key} has no reference");
        }
    }
}
