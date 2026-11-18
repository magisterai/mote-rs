//! Compares Mote's native layout with the layout of Microsoft Word and PowerPoint.
//!
//! A reference records the text lines of an Office PDF export of a review case or
//! a probe. An allowance records how far the native layout may currently lie from
//! its reference. The test below holds every native layout within its allowance on
//! any platform; recording a reference requires Office on macOS.
mod export;
pub(crate) mod lines;
mod matching;

use crate::{
    Outcome,
    allowance::{Field, Ledger, Scale},
    arguments::Arguments,
    digest, repository,
    review::catalogue,
};
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

const HELP: &str = "Usage: cargo xtask office-compare [--case NAME]... [--probe NAME]... [--record] [--allow [--raise SUBJECT]...] [--detail]

Compare the native layout of review cases and probes with their recorded Word or
PowerPoint references in tests/office/references/, and report each subject whose
line positions exceed its allowance in tests/office/allowances.json.
--case NAME selects a review case by identifier; --probe NAME selects a probe.
Without a selection, every case and probe is selected.
--record exports the selected subjects through Word or PowerPoint (macOS only)
and replaces their references.
--allow lowers the selected subjects' allowances to their present results, lists
every value that changed, and records a subject that has no allowance. A subject
whose present result exceeds its allowance is refused, and the command fails.
--raise SUBJECT, with --allow, lets the allowance of one subject rise to its
present result; SUBJECT is a case identifier or probe:NAME.
--detail lists unmatched lines and every line more than half a point from Office.
";

/// An offset is stored in tenths of a point. A small margin above the stored
/// value absorbs floating-point noise.
const OFFSET: Scale = Scale::new(10.0, 0.05);

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
        // A probe of PowerPoint's layout names the application in its file name.
        let application = if name.starts_with("powerpoint-") {
            Application::PowerPoint
        } else {
            Application::Word
        };
        subjects.push((
            format!("probe:{name}"),
            Subject {
                key: format!("probes/{name}"),
                application,
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
    // A review case may name a font of the system by its absolute path.
    let resolver =
        FileAssetResolver::unconfined(repository()).map_err(|error| error.to_string())?;
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
    /// The measurements that an allowance bounds, in the order of `values`.
    fn fields(comparison: &Comparison) -> [Field; 5] {
        let field = |name, scale, measured| Field {
            name,
            scale,
            measured,
        };
        [
            field("page_moves", Scale::COUNT, comparison.page_moves as f64),
            field("unmatched", Scale::COUNT, comparison.unmatched as f64),
            field("vertical", OFFSET, comparison.maximum_vertical),
            field("median", OFFSET, comparison.median_vertical),
            field("horizontal", OFFSET, comparison.maximum_horizontal),
        ]
    }

    fn values(&self) -> [f64; 5] {
        [
            self.page_moves as f64,
            self.unmatched as f64,
            self.vertical,
            self.median,
            self.horizontal,
        ]
    }

    fn from_values(values: &[f64]) -> Self {
        Self {
            page_moves: values[0] as usize,
            unmatched: values[1] as usize,
            vertical: values[2],
            median: values[3],
            horizontal: values[4],
        }
    }

    fn exceeded_by(&self, comparison: &Comparison) -> Vec<String> {
        let mut failures: Vec<String> = different_page_counts(comparison).into_iter().collect();
        for (field, allowed) in Self::fields(comparison).iter().zip(self.values()) {
            if field.scale.admits(allowed, field.measured) {
                continue;
            }
            failures.push(match field.name {
                "page_moves" => format!("{} lines on another page", comparison.page_moves),
                "unmatched" => format!("{} unmatched lines", comparison.unmatched),
                name => format!("{name} {:.2}pt exceeds {allowed:.1}pt", field.measured),
            });
        }
        failures
    }
}

/// No allowance admits a native layout whose number of pages differs from that
/// of Office.
fn different_page_counts(comparison: &Comparison) -> Option<String> {
    (comparison.office_pages != comparison.native_pages).then(|| {
        format!(
            "{} Office pages, {} native",
            comparison.office_pages, comparison.native_pages
        )
    })
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
    let mut raised = Vec::new();
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
            "--raise" => {
                let identifier = arguments.value("--raise")?;
                if !all.iter().any(|(candidate, _)| *candidate == identifier) {
                    return Err(format!("unknown subject: {identifier}"));
                }
                raised.push(identifier);
            }
            "--record" => recording = true,
            "--allow" => allowing = true,
            "--detail" => detailed = true,
            other => return Err(format!("unknown option: {other}; use --help")),
        }
    }
    let (identifiers, subjects): (Vec<&String>, Vec<&Subject>) = all
        .iter()
        .filter(|(identifier, _)| selected.is_empty() || selected.contains(identifier))
        .map(|(identifier, subject)| (identifier, subject))
        .unzip();
    crate::allowance::check_raised(&raised, &identifiers, allowing)?;
    if recording {
        for subject in &subjects {
            record(subject)?;
            println!("recorded {}", subject.key);
        }
    }
    let mut allowances = read_allowances()?;
    let mut ledger = Ledger::default();
    let mut failed = false;
    println!(
        "subject | pages | moved | unmatched | vertical | median | horizontal | first difference"
    );
    let comparisons = in_parallel(&subjects, compare);
    for ((subject, identifier), comparison) in subjects.iter().zip(identifiers).zip(comparisons) {
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
            let stored = allowances.get(&subject.key).map(Allowance::values);
            if let Some(values) = ledger.allow(
                &subject.key,
                identifier,
                &Allowance::fields(&comparison),
                stored.as_ref().map(<[f64; 5]>::as_slice),
                raised.contains(identifier),
            ) {
                allowances.insert(subject.key.clone(), Allowance::from_values(&values));
            }
            if let Some(difference) = different_page_counts(&comparison) {
                failed = true;
                println!("  no allowance admits {difference}");
            }
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
        print!("{}", ledger.report());
        failed |= ledger.refused();
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

    fn comparison(counts: [usize; 2], offsets: [f64; 3]) -> Comparison {
        Comparison {
            page_moves: counts[0],
            unmatched: counts[1],
            maximum_vertical: offsets[0],
            median_vertical: offsets[1],
            maximum_horizontal: offsets[2],
            ..Default::default()
        }
    }

    #[test]
    fn allowing_refuses_exactly_the_results_that_exceed_an_allowance() {
        let allowance = Allowance {
            page_moves: 1,
            unmatched: 2,
            vertical: 0.3,
            median: 0.1,
            horizontal: 0.5,
        };
        for (result, exceeds) in [
            (comparison([1, 2], [0.3, 0.1, 0.5]), false),
            (comparison([1, 2], [0.34, 0.14, 0.54]), false),
            (comparison([0, 0], [0.0, 0.0, 0.0]), false),
            (comparison([2, 2], [0.3, 0.1, 0.5]), true),
            (comparison([1, 3], [0.3, 0.1, 0.5]), true),
            (comparison([1, 2], [0.36, 0.1, 0.5]), true),
            (comparison([1, 2], [0.3, 0.16, 0.5]), true),
            (comparison([1, 2], [0.3, 0.1, 0.56]), true),
        ] {
            assert_eq!(
                !allowance.exceeded_by(&result).is_empty(),
                exceeds,
                "{result:?}"
            );
            let allowed = Ledger::default().allow(
                "subject",
                "subject",
                &Allowance::fields(&result),
                Some(&allowance.values()),
                false,
            );
            assert_eq!(allowed.is_none(), exceeds, "{result:?}");
            let Some(values) = allowed else {
                continue;
            };
            let lowered = Allowance::from_values(&values);
            assert!(
                values
                    .iter()
                    .zip(allowance.values())
                    .all(|(after, before)| *after <= before),
                "{result:?}: {lowered:?}"
            );
            assert!(lowered.exceeded_by(&result).is_empty(), "{result:?}");
        }
    }

    #[test]
    fn a_result_within_the_slack_of_its_allowance_leaves_the_allowance_as_it_is() {
        let allowance = Allowance {
            page_moves: 0,
            unmatched: 0,
            vertical: 0.3,
            median: 0.1,
            horizontal: 0.5,
        };
        let allowed = Ledger::default()
            .allow(
                "subject",
                "subject",
                &Allowance::fields(&comparison([0, 0], [0.31, 0.11, 0.51])),
                Some(&allowance.values()),
                false,
            )
            .unwrap();
        assert_eq!(Allowance::from_values(&allowed), allowance);
    }

    #[test]
    fn a_different_number_of_pages_exceeds_every_allowance() {
        let result = Comparison {
            office_pages: 2,
            native_pages: 3,
            ..Default::default()
        };
        assert_eq!(
            Allowance::default().exceeded_by(&result),
            ["2 Office pages, 3 native"]
        );
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
