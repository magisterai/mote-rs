//! Structural comparison of the evaluated models and DOCX packages of two
//! explicitly selected Mote executables.
use crate::{
    Outcome,
    arguments::{Arguments, unknown},
    digest::{digest, fingerprint},
    json::{Json, compact, object, pretty, set, string},
    package,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
    io::Read,
    iter::Peekable,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub(crate) const WORD_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const DATE_NAMESPACE: &str = "http://purl.org/dc/terms/";
const XML_NAMESPACE: &str = "http://www.w3.org/XML/1998/namespace";
const MAXIMUM_NESTING: usize = 4;
const NORMALIZATION: [&str; 6] = [
    "JSON numbers compare numerically; booleans retain their type.",
    "XML names compare by namespace URI, with attribute order ignored.",
    "Generation created/modified timestamps are ignored.",
    "Formatting whitespace is ignored outside text and xml:space=\"preserve\".",
    "Advisory lastRenderedPageBreak elements are counted and ignored; text split by those markers is joined within the same run.",
    "All other element order, values, text, and package parts are compared.",
];

const HELP: &str = "Usage: cargo xtask compare-outputs --reference PATH --candidate PATH --sources PATH --output PATH [OPTIONS]

Compare evaluated JSON and DOCX packages from two explicit Mote executables.
--reference PATH   the reference executable
--candidate PATH   the candidate executable
--sources PATH     the directory of .mote sources
--output PATH      a new directory for source snapshots, packages and report.json
--recursive        include sources in subdirectories
--workers N        concurrent cases (default 4)
--timeout SECONDS  deadline for each emitter process (default 30)
";

fn read_model(path: &Path) -> Result<Json, String> {
    Json::read(path)
}

fn kind(value: &Json) -> u8 {
    match value {
        Json::Null => 0,
        Json::Boolean(_) => 1,
        Json::Integer(_) | Json::Decimal(_) => 2,
        Json::String(_) => 3,
        Json::Array(_) => 4,
        Json::Object(_) => 5,
    }
}

/// Returns the first changed field. Numbers compare by value as `f64`, and
/// booleans never compare equal to numbers.
fn model_difference(reference: &Json, candidate: &Json, location: &str) -> Option<String> {
    if kind(reference) != kind(candidate) {
        return Some(format!("{location}: value type"));
    }
    match (reference, candidate) {
        (Json::Object(first), Json::Object(second)) => {
            let names: BTreeSet<_> = first.iter().map(|(name, _)| name).collect();
            if names != second.iter().map(|(name, _)| name).collect() {
                return Some(format!("{location}: object fields"));
            }
            names.into_iter().find_map(|name| {
                model_difference(
                    reference.get(name)?,
                    candidate.get(name)?,
                    &format!("{location}.{name}"),
                )
            })
        }
        (Json::Array(first), Json::Array(second)) => {
            if first.len() != second.len() {
                return Some(format!("{location}: array length"));
            }
            first
                .iter()
                .zip(second)
                .enumerate()
                .find_map(|(index, (first, second))| {
                    model_difference(first, second, &format!("{location}[{index}]"))
                })
        }
        _ if reference.as_number().is_some() => {
            (reference.as_number() != candidate.as_number()).then(|| format!("{location}: value"))
        }
        _ => (reference != candidate).then(|| format!("{location}: value")),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Canonical {
    Element {
        name: String,
        attributes: Vec<(String, String)>,
        text: String,
        children: Vec<Canonical>,
    },
    Tail(String),
}

fn expanded_name(namespace: Option<&str>, local: &str) -> String {
    match namespace {
        Some(namespace) => format!("{{{namespace}}}{local}"),
        None => local.to_owned(),
    }
}

fn is_word(node: roxmltree::Node, local: &str) -> bool {
    node.tag_name().namespace() == Some(WORD_NAMESPACE) && node.tag_name().name() == local
}

fn gap_text<'a, 'input>(nodes: &mut Peekable<roxmltree::Children<'a, 'input>>) -> String {
    let mut text = String::new();
    while let Some(node) = nodes.next_if(|node| !node.is_element()) {
        if node.is_text() {
            text.push_str(node.text().unwrap_or_default());
        }
    }
    text
}

fn joined_text(previous: &Canonical, next: &Canonical) -> Option<Canonical> {
    let word_text = expanded_name(Some(WORD_NAMESPACE), "t");
    match (previous, next) {
        (
            Canonical::Element {
                name,
                attributes,
                text,
                children,
            },
            Canonical::Element {
                name: next_name,
                attributes: next_attributes,
                text: next_text,
                children: next_children,
            },
        ) if *name == word_text
            && *next_name == word_text
            && attributes == next_attributes
            && children.is_empty()
            && next_children.is_empty() =>
        {
            Some(Canonical::Element {
                name: word_text,
                attributes: attributes.clone(),
                text: format!("{text}{next_text}"),
                children: Vec::new(),
            })
        }
        _ => None,
    }
}

/// Mirrors the element tree of the Python reference: element text precedes
/// the first child, and the text after each child is its tail.
pub(crate) fn canonical_xml(node: roxmltree::Node, inherited_preserve: bool) -> Canonical {
    let preserve = node
        .attribute((XML_NAMESPACE, "space"))
        .map_or(inherited_preserve, |space| space == "preserve");
    let name = expanded_name(node.tag_name().namespace(), node.tag_name().name());
    let mut attributes: Vec<_> = node
        .attributes()
        .map(|attribute| {
            (
                expanded_name(attribute.namespace(), attribute.name()),
                attribute.value().to_owned(),
            )
        })
        .collect();
    attributes.sort();
    let mut nodes = node.children().peekable();
    let mut text = gap_text(&mut nodes);
    if node.tag_name().namespace() == Some(DATE_NAMESPACE)
        && matches!(node.tag_name().name(), "created" | "modified")
    {
        text = "<generation timestamp>".into();
    } else if !preserve && text.trim().is_empty() && !is_word(node, "t") {
        text.clear();
    }
    let mut children: Vec<Canonical> = Vec::new();
    let mut advisory_pending = false;
    while let Some(child) = nodes.next() {
        if is_word(child, "lastRenderedPageBreak") {
            advisory_pending = true;
        } else {
            let canonical = canonical_xml(child, preserve);
            let joined = children
                .last()
                .filter(|_| advisory_pending && is_word(node, "r"))
                .and_then(|previous| joined_text(previous, &canonical));
            match joined {
                Some(joined) => *children.last_mut().expect("a previous child") = joined,
                None => children.push(canonical),
            }
            advisory_pending = false;
        }
        let tail = gap_text(&mut nodes);
        if !tail.trim().is_empty() || (preserve && !tail.is_empty()) {
            children.push(Canonical::Tail(tail));
        }
    }
    Canonical::Element {
        name,
        attributes,
        text,
        children,
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum PartContent {
    Xml(Canonical),
    Package(BTreeMap<String, PartContent>),
    Bytes(String),
}

type AdvisoryCounts = BTreeMap<String, usize>;

pub(crate) fn read_package(
    bytes: &[u8],
    depth: usize,
) -> Result<(BTreeMap<String, PartContent>, AdvisoryCounts), String> {
    if depth > MAXIMUM_NESTING {
        return Err("embedded package nesting exceeds four levels".into());
    }
    let mut parts = BTreeMap::new();
    let mut advisory_counts = BTreeMap::new();
    for part in package::parts(bytes)? {
        let content = if part.name.ends_with(".xml") || part.name.ends_with(".rels") {
            let text = std::str::from_utf8(&part.bytes)
                .map_err(|error| format!("{}: {error}", part.name))?;
            let document = roxmltree::Document::parse(text.trim_start_matches('\u{feff}'))
                .map_err(|error| format!("{}: {error}", part.name))?;
            let root = document.root_element();
            let count = root
                .descendants()
                .filter(|node| is_word(*node, "lastRenderedPageBreak"))
                .count();
            if count > 0 {
                advisory_counts.insert(part.name.clone(), count);
            }
            PartContent::Xml(canonical_xml(root, false))
        } else if part.name.ends_with(".xlsx") {
            let (nested, counts) = read_package(&part.bytes, depth + 1)?;
            advisory_counts.extend(
                counts
                    .into_iter()
                    .map(|(name, count)| (format!("{}/{name}", part.name), count)),
            );
            PartContent::Package(nested)
        } else {
            PartContent::Bytes(digest(&part.bytes))
        };
        parts.insert(part.name, content);
    }
    Ok((parts, advisory_counts))
}

pub(crate) struct Emission {
    pub exit_code: i32,
    pub stderr: String,
}

/// An emitter runs one executable with arguments under a deadline.
pub(crate) type Emitter<'a> =
    &'a (dyn Fn(&Path, &[OsString], Duration) -> Result<Emission, String> + Sync);

pub(crate) fn emit(
    executable: &Path,
    arguments: &[OsString],
    timeout: Duration,
) -> Result<Emission, String> {
    let mut child = Command::new(executable)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("{}: {error}", executable.display()))?;
    let mut stdout = child.stdout.take().expect("piped standard output");
    let mut stderr = child.stderr.take().expect("piped standard error");
    let output = thread::spawn(move || {
        let mut ignored = Vec::new();
        stdout.read_to_end(&mut ignored)
    });
    let errors = thread::spawn(move || {
        let mut text = String::new();
        stderr.read_to_string(&mut text).map(|_| text)
    });
    let deadline = Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child.try_wait().map_err(|error| error.to_string())? {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!(
                "{} timed out after {} seconds",
                executable.display(),
                timeout.as_secs_f64()
            ));
        }
        thread::sleep(Duration::from_millis(10));
    };
    let _ = output.join();
    let stderr = errors
        .join()
        .map_err(|_| "standard error reader failed".to_owned())?
        .map_err(|error| error.to_string())?;
    Ok(Emission {
        exit_code: status.code().unwrap_or(-1),
        stderr,
    })
}

enum Failure {
    Emitter {
        label: String,
        format: &'static str,
        exit_code: i32,
    },
    Message(String),
}

impl From<String> for Failure {
    fn from(message: String) -> Self {
        Self::Message(message)
    }
}

struct Comparison {
    differences: Vec<Json>,
    advisory: Json,
}

fn counts_object(counts: AdvisoryCounts) -> Json {
    Json::Object(
        counts
            .into_iter()
            .map(|(name, count)| (name, Json::Integer(count as i128)))
            .collect(),
    )
}

fn run_case(
    source: &Path,
    destination: &Path,
    executables: &[(&str, PathBuf)],
    timeout: Duration,
    emitter: Emitter,
    source_digest: &mut String,
    diagnostics: &mut Vec<(String, Json)>,
) -> Result<Comparison, Failure> {
    let io = |error: std::io::Error| format!("{}: {error}", destination.display());
    fs::create_dir(destination).map_err(io)?;
    let snapshot = destination.join("source.mote");
    fs::write(&snapshot, fs::read(source).map_err(io)?).map_err(io)?;
    *source_digest = fingerprint(&snapshot)?;
    for (label, executable) in executables {
        diagnostics.push(((*label).to_owned(), Json::Object(Vec::new())));
        for format in ["json", "docx"] {
            let output = destination.join(format!("{label}.{format}"));
            let arguments: Vec<OsString> = vec![
                "-format".into(),
                format.into(),
                "-o".into(),
                output.clone().into(),
                source.into(),
            ];
            let emission = emitter(executable, &arguments, timeout)?;
            if !emission.stderr.is_empty()
                && let Some((_, Json::Object(messages))) = diagnostics.last_mut()
            {
                messages.push((format.to_owned(), string(emission.stderr.trim())));
            }
            if emission.exit_code != 0 {
                return Err(Failure::Emitter {
                    label: (*label).to_owned(),
                    format,
                    exit_code: emission.exit_code,
                });
            }
            if !output.is_file() {
                return Err(format!("{label} did not create {format} output").into());
            }
            if fingerprint(source)? != *source_digest {
                return Err("source changed during comparison".to_owned().into());
            }
        }
    }
    let mut differences = Vec::new();
    if let Some(location) = model_difference(
        &read_model(&destination.join("reference.json"))?,
        &read_model(&destination.join("candidate.json"))?,
        "$",
    ) {
        differences.push(object([
            ("part", string("model.json")),
            ("location", string(location)),
        ]));
    }
    let read = |name: &str| {
        let path = destination.join(name);
        let bytes = fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        read_package(&bytes, 0)
    };
    let (reference, reference_advisory) = read("reference.docx")?;
    let (candidate, candidate_advisory) = read("candidate.docx")?;
    let names: BTreeSet<_> = reference.keys().chain(candidate.keys()).collect();
    for name in names {
        if reference.get(name) != candidate.get(name) {
            differences.push(object([("part", string(name))]));
        }
    }
    Ok(Comparison {
        differences,
        advisory: object([
            ("reference", counts_object(reference_advisory)),
            ("candidate", counts_object(candidate_advisory)),
        ]),
    })
}

pub(crate) fn compare_case(
    source: &Path,
    destination: &Path,
    executables: &[(&str, PathBuf)],
    timeout: Duration,
    emitter: Emitter,
) -> Json {
    let mut source_digest = String::new();
    let mut diagnostics = Vec::new();
    let outcome = run_case(
        source,
        destination,
        executables,
        timeout,
        emitter,
        &mut source_digest,
        &mut diagnostics,
    );
    let status = match &outcome {
        Ok(comparison) if comparison.differences.is_empty() => "matched",
        Ok(_) => "different",
        Err(_) => "error",
    };
    let mut result = vec![
        ("source".to_owned(), string(source.display())),
        ("source_sha256".to_owned(), string(source_digest)),
        ("artifacts".to_owned(), string(destination.display())),
        ("status".to_owned(), string(status)),
        ("diagnostics".to_owned(), Json::Object(diagnostics)),
    ];
    match outcome {
        Ok(comparison) => {
            set(&mut result, "ignored_advisory_markers", comparison.advisory);
            set(
                &mut result,
                "differences",
                Json::Array(comparison.differences),
            );
        }
        Err(Failure::Emitter {
            label,
            format,
            exit_code,
        }) => set(
            &mut result,
            "error",
            object([
                ("executable", string(label)),
                ("format", string(format)),
                ("exit_code", Json::Integer(exit_code.into())),
            ]),
        ),
        Err(Failure::Message(message)) => {
            set(&mut result, "error", object([("message", string(message))]))
        }
    }
    Json::Object(result)
}

struct Options {
    reference: PathBuf,
    candidate: PathBuf,
    sources: PathBuf,
    output: PathBuf,
    recursive: bool,
    workers: usize,
    timeout: f64,
}

impl Options {
    fn parse(mut arguments: Arguments) -> Result<Option<Self>, String> {
        let (mut reference, mut candidate, mut sources, mut output) = (None, None, None, None);
        let (mut recursive, mut workers, mut timeout) = (false, 4, 30.0);
        while let Some(argument) = arguments.next() {
            match argument.as_str() {
                "--help" | "-h" => {
                    print!("{HELP}");
                    return Ok(None);
                }
                "--reference" => reference = Some(arguments.path(&argument)?),
                "--candidate" => candidate = Some(arguments.path(&argument)?),
                "--sources" => sources = Some(arguments.path(&argument)?),
                "--output" => output = Some(arguments.path(&argument)?),
                "--recursive" => recursive = true,
                "--workers" => workers = arguments.number(&argument)?,
                "--timeout" => timeout = arguments.number(&argument)?,
                _ => return Err(unknown(&argument)),
            }
        }
        let required = |value: Option<PathBuf>, option: &str| {
            value.ok_or_else(|| format!("{option} is required"))
        };
        Ok(Some(Self {
            reference: required(reference, "--reference")?,
            candidate: required(candidate, "--candidate")?,
            sources: required(sources, "--sources")?,
            output: required(output, "--output")?,
            recursive,
            workers,
            timeout,
        }))
    }
}

fn find_sources(
    directory: &Path,
    recursive: bool,
    sources: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let entries =
        fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        if kind.is_dir() {
            if recursive {
                find_sources(&path, recursive, sources)?;
            }
        } else if path
            .extension()
            .is_some_and(|extension| extension == "mote")
        {
            sources.push(path);
        }
    }
    Ok(())
}

fn parallel<T: Sync, R: Send>(
    items: &[T],
    workers: usize,
    work: impl Fn(&T) -> R + Sync,
) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let results = Mutex::new((0..items.len()).map(|_| None).collect::<Vec<_>>());
    thread::scope(|scope| {
        for _ in 0..workers.min(items.len()) {
            scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(item) = items.get(index) else { break };
                    let result = work(item);
                    results.lock().expect("an unpoisoned result list")[index] = Some(result);
                }
            });
        }
    });
    results
        .into_inner()
        .expect("an unpoisoned result list")
        .into_iter()
        .flatten()
        .collect()
}

/// Formats the current time as an ISO 8601 UTC timestamp with microseconds.
fn utc_timestamp() -> String {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let seconds = elapsed.as_secs() as i64;
    let (days, time) = (seconds.div_euclid(86_400), seconds.rem_euclid(86_400));
    let era = (days + 719_468).div_euclid(146_097);
    let day_of_era = (days + 719_468).rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:06}+00:00",
        time / 3600,
        time % 3600 / 60,
        time % 60,
        elapsed.subsec_micros()
    )
}

fn execute(options: &Options, emitter: Emitter) -> Result<bool, String> {
    if options.workers < 1 || options.timeout.is_nan() || options.timeout <= 0.0 {
        return Err("workers and timeout must be positive".into());
    }
    let timeout = Duration::try_from_secs_f64(options.timeout)
        .map_err(|_| "timeout is too large".to_owned())?;
    let mut executables = Vec::new();
    for (label, path) in [
        ("reference", &options.reference),
        ("candidate", &options.candidate),
    ] {
        let resolved = fs::canonicalize(path)
            .ok()
            .filter(|path| path.is_file())
            .ok_or_else(|| format!("{label} executable is not a file: {}", path.display()))?;
        let digest = fingerprint(&resolved)?;
        executables.push((label, resolved, digest));
    }
    let source_root = std::path::absolute(&options.sources).map_err(|error| error.to_string())?;
    let mut sources = Vec::new();
    find_sources(&source_root, options.recursive, &mut sources)?;
    sources.sort();
    if sources.is_empty() {
        return Err("no .mote sources found".into());
    }
    let destination = std::path::absolute(&options.output).map_err(|error| error.to_string())?;
    if destination.exists() {
        return Err("output directory already exists; choose a new path".into());
    }
    fs::create_dir_all(&destination)
        .map_err(|error| format!("{}: {error}", destination.display()))?;
    let jobs: Vec<_> = sources
        .iter()
        .enumerate()
        .map(|(index, source)| {
            let stem = source.file_stem().unwrap_or_default().to_string_lossy();
            (source, destination.join(format!("{index:05}-{stem}")))
        })
        .collect();
    let selected: Vec<_> = executables
        .iter()
        .map(|(label, path, _)| (*label, path.clone()))
        .collect();
    let results = parallel(&jobs, options.workers, |(source, case)| {
        compare_case(source, case, &selected, timeout, emitter)
    });
    let count = |status: &str| {
        results
            .iter()
            .filter(|result| result.get("status").and_then(Json::as_str) == Some(status))
            .count()
    };
    let (matched, different, errors) = (count("matched"), count("different"), count("error"));
    let integrity_errors: Vec<_> = executables
        .iter()
        .filter(|(_, path, digest)| fingerprint(path).as_ref() != Ok(digest))
        .map(|(label, _, _)| string(format!("{label} executable changed during comparison")))
        .collect();
    let failed = different > 0 || errors > 0 || !integrity_errors.is_empty();
    let summary = [
        ("cases", Json::Integer(results.len() as i128)),
        ("matched", Json::Integer(matched as i128)),
        ("different", Json::Integer(different as i128)),
        ("error", Json::Integer(errors as i128)),
    ];
    let report = object([
        ("created_at_utc", string(utc_timestamp())),
        (
            "executables",
            Json::Object(
                executables
                    .iter()
                    .map(|(label, path, digest)| {
                        (
                            (*label).to_owned(),
                            object([("path", string(path.display())), ("sha256", string(digest))]),
                        )
                    })
                    .collect(),
            ),
        ),
        (
            "normalization",
            Json::Array(NORMALIZATION.into_iter().map(string).collect()),
        ),
        ("word_review_status", string("unreviewed")),
        ("input_integrity_errors", Json::Array(integrity_errors)),
        ("summary", object(summary.clone())),
        ("cases", Json::Array(results)),
    ]);
    let path = destination.join("report.json");
    fs::write(&path, pretty(&report) + "\n")
        .map_err(|error| format!("{}: {error}", path.display()))?;
    let mut line = vec![("report".to_owned(), string(path.display()))];
    line.extend(summary.map(|(name, value)| (name.to_owned(), value)));
    println!("{}", compact(&Json::Object(line)));
    Ok(!failed)
}

pub(crate) fn run(arguments: Arguments) -> Outcome {
    let Some(options) = Options::parse(arguments)? else {
        return Ok(ExitCode::SUCCESS);
    };
    Ok(if execute(&options, &emit)? {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TemporaryDirectory;
    use std::io::{Cursor, Write};
    use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

    fn canonical(text: &str) -> Canonical {
        canonical_xml(
            roxmltree::Document::parse(text).unwrap().root_element(),
            false,
        )
    }

    fn archive(entries: &[(&str, &[u8])], method: CompressionMethod) -> Vec<u8> {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        for (name, contents) in entries {
            writer
                .start_file(
                    *name,
                    SimpleFileOptions::default().compression_method(method),
                )
                .unwrap();
            writer.write_all(contents).unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    fn crc32(bytes: &[u8]) -> u32 {
        let mut crc = !0u32;
        for byte in bytes {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = if crc & 1 == 1 {
                    (crc >> 1) ^ 0xEDB8_8320
                } else {
                    crc >> 1
                };
            }
        }
        !crc
    }

    /// A stored archive written by hand, because the zip writer refuses
    /// the duplicate names that a hostile package may contain.
    fn stored_archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let (mut bytes, mut directory) = (Vec::new(), Vec::new());
        for (name, contents) in entries {
            let offset = bytes.len() as u32;
            let header = |signature: u32| {
                let mut header = signature.to_le_bytes().to_vec();
                if signature == 0x0201_4b50 {
                    header.extend(20u16.to_le_bytes());
                }
                header.extend([20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
                header.extend(crc32(contents).to_le_bytes());
                header.extend((contents.len() as u32).to_le_bytes());
                header.extend((contents.len() as u32).to_le_bytes());
                header.extend((name.len() as u16).to_le_bytes());
                header.extend(0u16.to_le_bytes());
                header
            };
            bytes.extend(header(0x0403_4b50));
            bytes.extend(name.as_bytes());
            bytes.extend(*contents);
            directory.extend(header(0x0201_4b50));
            directory.extend([0; 10]);
            directory.extend(offset.to_le_bytes());
            directory.extend(name.as_bytes());
        }
        let start = bytes.len() as u32;
        bytes.extend(&directory);
        bytes.extend(0x0605_4b50u32.to_le_bytes());
        bytes.extend([0, 0, 0, 0]);
        bytes.extend((entries.len() as u16).to_le_bytes());
        bytes.extend((entries.len() as u16).to_le_bytes());
        bytes.extend((directory.len() as u32).to_le_bytes());
        bytes.extend(start.to_le_bytes());
        bytes.extend(0u16.to_le_bytes());
        bytes
    }

    fn executables() -> Vec<(&'static str, PathBuf)> {
        let executable = std::env::current_exe().unwrap();
        vec![("reference", executable.clone()), ("candidate", executable)]
    }

    fn error_message(result: &Json) -> &str {
        result
            .get("error")
            .and_then(|error| error.get("message"))
            .and_then(Json::as_str)
            .unwrap_or_default()
    }

    #[test]
    fn numeric_spelling_matches_but_boolean_types_do_not() {
        let directory = TemporaryDirectory::new();
        let path = directory.0.join("model.json");
        fs::write(&path, r#"{"value": 2.00}"#).unwrap();
        let first = read_model(&path).unwrap();
        fs::write(&path, r#"{"value": 2}"#).unwrap();
        assert_eq!(
            model_difference(&first, &read_model(&path).unwrap(), "$"),
            None
        );
        let boolean = Json::parse(r#"{"value": true}"#).unwrap();
        let number = Json::parse(r#"{"value": 1}"#).unwrap();
        assert!(model_difference(&boolean, &number, "$").is_some());
        fs::write(&path, r#"{"value": NaN}"#).unwrap();
        assert!(read_model(&path).is_err());
    }

    #[test]
    fn xml_prefixes_and_indentation_do_not_hide_changed_geometry() {
        let first = canonical(r#"<w:p xmlns:w="urn:word"><w:r width="20"/></w:p>"#);
        let second = canonical("<a:p xmlns:a=\"urn:word\">\n  <a:r width=\"20\"/>\n</a:p>");
        assert_eq!(first, second);
        let changed = canonical("<a:p xmlns:a=\"urn:word\">\n  <a:r width=\"21\"/>\n</a:p>");
        assert_ne!(first, changed);
    }

    #[test]
    fn preserved_whitespace_order_and_tail_text_remain_significant() {
        for (first, second) in [
            (
                r#"<p xml:space="preserve"> </p>"#,
                r#"<p xml:space="preserve">  </p>"#,
            ),
            ("<p><first/><second/></p>", "<p><second/><first/></p>"),
            ("<p><run/>tail</p>", "<p><run/>changed</p>"),
        ] {
            assert_ne!(canonical(first), canonical(second));
        }
    }

    #[test]
    fn timestamp_normalization_does_not_ignore_other_metadata() {
        let core = |created: &str, title: &str| {
            canonical(&format!(
                r#"<core xmlns:d="http://purl.org/dc/terms/"><d:created>{created}</d:created><title>{title}</title></core>"#
            ))
        };
        assert_eq!(core("one", "Title"), core("two", "Title"));
        assert_ne!(core("one", "Title"), core("two", "Changed"));
    }

    #[test]
    fn only_text_split_by_an_advisory_marker_is_joined() {
        let run =
            |body: &str| canonical(&format!(r#"<w:r xmlns:w="{WORD_NAMESPACE}">{body}</w:r>"#));
        let whole = run(r#"<w:t xml:space="preserve">one two</w:t>"#);
        let split = run(
            r#"<w:t xml:space="preserve">one </w:t><w:lastRenderedPageBreak/><w:t xml:space="preserve">two</w:t>"#,
        );
        assert_eq!(whole, split);
        for separator in ["", "<w:br/>", "<w:tab/>"] {
            assert_ne!(
                whole,
                run(&format!(
                    r#"<w:t xml:space="preserve">one </w:t>{separator}<w:t xml:space="preserve">two</w:t>"#
                ))
            );
        }
        assert_ne!(
            whole,
            run(r#"<w:t>one </w:t><w:lastRenderedPageBreak/><w:t xml:space="preserve">two</w:t>"#)
        );
    }

    #[test]
    fn packages_count_advisory_breaks_and_compare_binary_parts() {
        let document = format!(
            r#"<w:document xmlns:w="{WORD_NAMESPACE}"><w:lastRenderedPageBreak/><w:br w:type="page"/></w:document>"#
        );
        let entries: [(&str, &[u8]); 2] = [
            ("word/document.xml", document.as_bytes()),
            ("word/media/image.png", b"pixels"),
        ];
        let (parts, counts) =
            read_package(&archive(&entries, CompressionMethod::Stored), 0).unwrap();
        assert_eq!(
            counts,
            BTreeMap::from([("word/document.xml".to_owned(), 1)])
        );
        assert!(matches!(
            parts["word/media/image.png"],
            PartContent::Bytes(_)
        ));
        assert!(format!("{:?}", parts["word/document.xml"]).contains("br"));
        let duplicated =
            stored_archive(&[entries[0], entries[1], ("word/document.xml", b"<changed/>")]);
        let error = read_package(&duplicated, 0).unwrap_err();
        assert!(error.contains("duplicate package part"), "{error}");
    }

    #[test]
    fn embedded_workbooks_compare_contents_not_archive_encoding() {
        let package = |value: &str, reverse: bool, duplicate: bool| {
            let sheet = format!("<sheet><cell>{value}</cell></sheet>");
            let mut entries: Vec<(&str, &[u8])> = vec![
                ("xl/worksheets/sheet1.xml", sheet.as_bytes()),
                ("xl/styles.xml", b"<styles/>"),
            ];
            if reverse {
                entries.reverse();
            }
            let workbook = if duplicate {
                entries.push(("xl/styles.xml", b"<other/>"));
                stored_archive(&entries)
            } else if reverse {
                archive(&entries, CompressionMethod::Deflated)
            } else {
                archive(&entries, CompressionMethod::Stored)
            };
            archive(
                &[("word/embeddings/chart1.xlsx", &workbook)],
                CompressionMethod::Stored,
            )
        };
        let original = read_package(&package("4", false, false), 0).unwrap();
        assert_eq!(
            original,
            read_package(&package("4", true, false), 0).unwrap()
        );
        assert_ne!(
            original,
            read_package(&package("5", false, false), 0).unwrap()
        );
        let duplicate = read_package(&package("4", false, true), 0).unwrap_err();
        assert!(duplicate.contains("duplicate package part"));
        assert!(
            read_package(&package("4", false, false), 5)
                .unwrap_err()
                .contains("nesting")
        );
    }

    #[test]
    fn failed_emitters_and_timeouts_cannot_be_reported_as_matches() {
        let directory = TemporaryDirectory::new();
        let source = directory.0.join("case.mote");
        fs::write(&source, "Text").unwrap();
        let failing = vec![
            ("reference", PathBuf::from("/usr/bin/false")),
            ("candidate", PathBuf::from("/usr/bin/false")),
        ];
        let result = compare_case(
            &source,
            &directory.0.join("failure"),
            &failing,
            Duration::from_secs(5),
            &emit,
        );
        assert_eq!(result.get("status").and_then(Json::as_str), Some("error"));
        let executable = result
            .get("error")
            .and_then(|error| error.get("executable"));
        assert_eq!(executable.and_then(Json::as_str), Some("reference"));
        let timeout = emit(
            Path::new("/bin/sleep"),
            &["5".into()],
            Duration::from_millis(100),
        )
        .err()
        .unwrap();
        assert!(timeout.contains("timed out"));
        let result = compare_case(
            &source,
            &directory.0.join("timeout"),
            &executables(),
            Duration::from_millis(100),
            &|_, _, _| Err("emitter timed out after 0.1 seconds".into()),
        );
        assert_eq!(result.get("status").and_then(Json::as_str), Some("error"));
        assert!(error_message(&result).contains("timed out"));
    }

    #[test]
    fn command_writes_a_traceable_report_and_refuses_existing_destinations() {
        let directory = TemporaryDirectory::new();
        let sources = directory.0.join("sources");
        fs::create_dir(&sources).unwrap();
        let source = sources.join("case.mote");
        fs::write(&source, "Text").unwrap();
        let produce_output = |_: &Path, arguments: &[OsString], _: Duration| {
            assert_eq!(Path::new(arguments.last().unwrap()), source);
            let output = Path::new(&arguments[3]);
            if arguments[1] == "json" {
                fs::write(output, r#"{"content": []}"#).unwrap();
            } else {
                fs::write(
                    output,
                    archive(
                        &[("word/document.xml", b"<document/>")],
                        CompressionMethod::Stored,
                    ),
                )
                .unwrap();
            }
            Ok(Emission {
                exit_code: 0,
                stderr: String::new(),
            })
        };
        let executable = std::env::current_exe().unwrap();
        let options = Options {
            reference: executable.clone(),
            candidate: executable,
            sources,
            output: directory.0.join("report"),
            recursive: false,
            workers: 4,
            timeout: 30.0,
        };
        assert!(execute(&options, &produce_output).unwrap());
        let report = Json::read(&directory.0.join("report/report.json")).unwrap();
        let summary = report.get("summary").unwrap();
        for (name, count) in [("cases", 1), ("matched", 1), ("different", 0), ("error", 0)] {
            assert_eq!(summary.get(name), Some(&Json::Integer(count)));
        }
        assert_eq!(
            report.get("word_review_status").and_then(Json::as_str),
            Some("unreviewed")
        );
        let case = &report.get("cases").and_then(Json::as_array).unwrap()[0];
        assert_eq!(
            case.get("source_sha256").and_then(Json::as_str),
            Some(fingerprint(&source).unwrap().as_str())
        );
        let snapshot = directory.0.join("report/00000-case/source.mote");
        assert_eq!(fs::read_to_string(snapshot).unwrap(), "Text");
        assert!(execute(&options, &produce_output).is_err());
    }

    #[test]
    fn changed_sources_are_errors_even_when_the_emitter_succeeds() {
        let directory = TemporaryDirectory::new();
        let source = directory.0.join("case.mote");
        fs::write(&source, "Before").unwrap();
        let change_source = |_: &Path, arguments: &[OsString], _: Duration| {
            fs::write(&arguments[3], "{}").unwrap();
            fs::write(&source, "After").unwrap();
            Ok(Emission {
                exit_code: 0,
                stderr: String::new(),
            })
        };
        let result = compare_case(
            &source,
            &directory.0.join("result"),
            &executables(),
            Duration::from_secs(5),
            &change_source,
        );
        assert_eq!(result.get("status").and_then(Json::as_str), Some("error"));
        assert!(error_message(&result).contains("source changed"));
    }
}
