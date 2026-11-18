//! Refresh of the captured editable packages under tests/fixtures. A stored part
//! is replaced only when its canonical XML changes, so that formatting, generation
//! timestamps and advisory page-break markers never produce a refresh.
use crate::{
    Outcome,
    arguments::{Arguments, unknown},
    compare_outputs::{canonical_xml, read_package},
    digest::digest,
    json::{Json, pretty},
    package, repository,
};
use mote::{
    evaluation::EvaluationOptions,
    source::{CachingAssetResolver, FileAssetResolver},
};
use std::{
    collections::BTreeSet,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};

const HELP: &str = "Usage: cargo xtask refresh-captures [--group NAME]... [--check]

Regenerate every captured DOCX and PPTX under tests/fixtures and replace each
stored part whose canonical XML differs. Part files that no manifest references
any longer are removed.
--group NAME  refresh one fixture group, such as tables or docx; repeatable
--check       report the parts that would change, write nothing, and fail if any
";

/// The fixture groups whose test harnesses always evaluate with Lua. Evaluation
/// differs with Lua enabled, so a capture is refreshed under its harness's setting.
pub(crate) const LUA_GROUPS: [&str; 10] = [
    "charts",
    "constructors",
    "drawings",
    "functions",
    "presentation-fields",
    "presentation-images",
    "presentation-text",
    "presentations",
    "rendering-audit",
    "scripting",
];

/// The three storage layouts of captured packages.
enum Layout {
    /// Content-addressed part files named by an object of entries in the manifest.
    Addressed { group_root: PathBuf },
    /// A package tree beside the manifest, written exactly as emitted.
    Package,
    /// A package tree beside the manifest, written as formatted XML.
    FormattedPackage,
}

struct Capture {
    manifest: PathBuf,
    group: String,
    source: PathBuf,
    layout: Layout,
}

struct Refresh {
    write: bool,
    changes: Vec<String>,
}

pub(crate) fn run(mut arguments: Arguments) -> Outcome {
    let mut groups = Vec::new();
    let mut write = true;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(ExitCode::SUCCESS);
            }
            "--group" => groups.push(arguments.value("--group")?),
            "--check" => write = false,
            option => return Err(unknown(option)),
        }
    }
    let fixtures = repository().join("tests/fixtures");
    let mut refresh = Refresh {
        write,
        changes: Vec::new(),
    };
    let mut skipped = Vec::new();
    for capture in captures(&fixtures)? {
        if !groups.is_empty() && !groups.contains(&capture.group) {
            continue;
        }
        if let Err(error) = refresh.capture(&capture) {
            skipped.push(format!("{}: {error}", capture.manifest.display()));
        }
    }
    let orphans = unreferenced_parts(&fixtures)?;
    for orphan in &orphans {
        refresh
            .changes
            .push(format!("{} is unreferenced", orphan.display()));
        if write {
            fs::remove_file(orphan).map_err(|error| format!("{}: {error}", orphan.display()))?;
        }
    }
    for change in &refresh.changes {
        println!("{change}");
    }
    for skip in &skipped {
        eprintln!("skipped {skip}");
    }
    println!(
        "{} changes {}",
        refresh.changes.len(),
        if write { "written" } else { "found" }
    );
    Ok(
        if skipped.is_empty() && (write || refresh.changes.is_empty()) {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        },
    )
}

fn manifests(directory: &Path, found: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(directory).map_err(|error| format!("{}: {error}", directory.display()))?;
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            manifests(&path, found)?;
        } else if path.file_name().is_some_and(|name| name == "capture.json") {
            found.push(path);
        }
    }
    Ok(())
}

fn captures(fixtures: &Path) -> Result<Vec<Capture>, String> {
    let mut paths = Vec::new();
    manifests(fixtures, &mut paths)?;
    paths.sort();
    let mut captures = Vec::new();
    for manifest in paths {
        let document = Json::read(&manifest)?;
        let directory = manifest.parent().expect("a manifest lies in a directory");
        let group = manifest
            .strip_prefix(fixtures)
            .ok()
            .and_then(|relative| relative.components().next())
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .unwrap_or_default();
        let (source, layout) = match document.get("parts") {
            Some(Json::Object(_)) => {
                let group_root = directory
                    .parent()
                    .and_then(Path::parent)
                    .ok_or("an addressed capture lies two levels below its group")?;
                let case = directory.file_name().unwrap_or_default().to_string_lossy();
                (
                    group_root.join(format!("{case}.mote")),
                    Layout::Addressed {
                        group_root: group_root.to_path_buf(),
                    },
                )
            }
            Some(Json::Array(_)) => match document.get("source").and_then(Json::as_str) {
                Some(source)
                    if directory
                        .parent()
                        .is_some_and(|parent| parent.ends_with("captured")) =>
                {
                    (directory.join(source), Layout::Package)
                }
                Some(source) => (directory.join(source), Layout::FormattedPackage),
                None => continue,
            },
            _ => continue,
        };
        captures.push(Capture {
            manifest,
            group,
            source,
            layout,
        });
    }
    Ok(captures)
}

/// Sources are rendered as the tests render them: from their own directory, with
/// Lua where the group's harness enables it, and otherwise with Lua only when
/// evaluation without it fails.
pub(crate) fn render(source: &Path, lua: bool, presentation: bool) -> Result<Vec<u8>, String> {
    let text =
        fs::read_to_string(source).map_err(|error| format!("{}: {error}", source.display()))?;
    let directory = source.parent().expect("a source lies in a directory");
    let files = FileAssetResolver::new(directory).map_err(|error| error.to_string())?;
    let resolver = CachingAssetResolver::new(&files);
    let compile = |enable_lua| {
        mote::compile_with_options(
            &text,
            EvaluationOptions {
                asset_resolver: Some(&resolver),
                source_name: Some(source.as_os_str()),
                enable_lua,
                ..Default::default()
            },
        )
    };
    let document = compile(lua)
        .or_else(|_| compile(true))
        .map_err(|error| error.message)?;
    let name = Some(source.as_os_str());
    if presentation {
        mote::pptx::emit_with_assets(&document, &resolver, name)
            .map(|package| package.bytes)
            .map_err(|error| error.to_string())
    } else {
        mote::docx::emit_with_assets(&document, &resolver, name)
            .map(|package| package.bytes)
            .map_err(|error| error.to_string())
    }
}

fn equivalent(name: &str, stored: &[u8], emitted: &[u8]) -> Result<bool, String> {
    if name.ends_with(".xml") || name.ends_with(".rels") {
        let parse = |bytes: &[u8]| -> Result<_, String> {
            let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
            let text = text.trim_start_matches('\u{feff}').to_owned();
            roxmltree::Document::parse(&text)
                .map(|document| canonical_xml(document.root_element(), false))
                .map_err(|error| format!("{name}: {error}"))
        };
        Ok(parse(stored)? == parse(emitted)?)
    } else if name.ends_with(".xlsx") {
        Ok(read_package(stored, 1)?.0 == read_package(emitted, 1)?.0)
    } else {
        Ok(stored == emitted)
    }
}

/// The stored formatting is xmllint's, with the standalone declaration omitted.
fn formatted(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut xmllint = Command::new("xmllint")
        .args(["--format", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("xmllint is required to format captures: {error}"))?;
    xmllint
        .stdin
        .take()
        .expect("standard input is piped")
        .write_all(bytes)
        .map_err(|error| error.to_string())?;
    let output = xmllint
        .wait_with_output()
        .map_err(|error| error.to_string())?;
    let text = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    Ok(text.replacen(" standalone=\"yes\"?>", "?>", 1).into_bytes())
}

impl Refresh {
    fn capture(&mut self, capture: &Capture) -> Result<(), String> {
        let text = fs::read_to_string(&capture.manifest).map_err(|error| error.to_string())?;
        let mut manifest = Json::parse(&text)?;
        let part_names: Vec<&str> = match manifest.get("parts") {
            Some(Json::Object(entries)) => entries.iter().map(|(name, _)| name.as_str()).collect(),
            Some(Json::Array(names)) => names.iter().filter_map(Json::as_str).collect(),
            _ => Vec::new(),
        };
        let presentation = manifest.get("format").and_then(Json::as_str) == Some("pptx")
            || part_names.iter().any(|name| name.starts_with("ppt/"));
        let emitted = package::parts(&render(
            &capture.source,
            LUA_GROUPS.contains(&capture.group.as_str()),
            presentation,
        )?)?;
        let directory = capture
            .manifest
            .parent()
            .expect("a manifest lies in a directory");
        match &capture.layout {
            Layout::Addressed { group_root } => {
                let Some(Json::Object(entries)) = field(&mut manifest, "parts") else {
                    unreachable!("an addressed capture has an object of parts");
                };
                let before = self.changes.len();
                self.addressed(
                    entries,
                    &emitted,
                    group_root,
                    &capture.manifest.display().to_string(),
                )?;
                if self.write && self.changes.len() > before {
                    let ending = if text.ends_with('\n') { "\n" } else { "" };
                    fs::write(&capture.manifest, pretty(&manifest) + ending)
                        .map_err(|error| error.to_string())?;
                }
            }
            Layout::Package | Layout::FormattedPackage => {
                for part in &emitted {
                    let stored_path = directory.join("package").join(&part.name);
                    if part.name == "docProps/core.xml" || !stored_path.is_file() {
                        continue;
                    }
                    let stored = fs::read(&stored_path).map_err(|error| error.to_string())?;
                    if equivalent(&part.name, &stored, &part.bytes)? {
                        continue;
                    }
                    self.changes
                        .push(format!("{}: {}", capture.manifest.display(), part.name));
                    if self.write {
                        let bytes = match capture.layout {
                            Layout::FormattedPackage if part.name.ends_with(".xml") => {
                                formatted(&part.bytes)?
                            }
                            _ => part.bytes.clone(),
                        };
                        fs::write(&stored_path, bytes).map_err(|error| error.to_string())?;
                    }
                }
            }
        }
        Ok(())
    }

    fn addressed(
        &mut self,
        entries: &mut [(String, Json)],
        emitted: &[package::Part],
        group_root: &Path,
        location: &str,
    ) -> Result<(), String> {
        for (name, entry) in entries {
            let Some(part) = emitted.iter().find(|part| part.name == *name) else {
                continue;
            };
            if name == "docProps/core.xml" {
                continue;
            }
            if let Some(Json::Object(nested)) = field(entry, "parts") {
                let nested_parts = package::parts(&part.bytes)?;
                self.addressed(
                    nested,
                    &nested_parts,
                    group_root,
                    &format!("{location}: {name}"),
                )?;
                continue;
            }
            let stored_path = entry
                .get("path")
                .and_then(Json::as_str)
                .ok_or_else(|| format!("{location}: {name} has no path"))?
                .to_owned();
            let stored = fs::read(group_root.join(&stored_path))
                .map_err(|error| format!("{stored_path}: {error}"))?;
            if equivalent(name, &stored, &part.bytes)? {
                continue;
            }
            self.changes.push(format!("{location}: {name}"));
            let sha256 = digest(&part.bytes);
            let stored_path = Path::new(&stored_path);
            let file_name = match stored_path.extension() {
                Some(extension) => format!("{sha256}.{}", extension.to_string_lossy()),
                None => sha256.clone(),
            };
            let path = stored_path.with_file_name(file_name);
            if self.write {
                fs::write(group_root.join(&path), &part.bytes)
                    .map_err(|error| error.to_string())?;
            }
            *field(entry, "path").expect("the path was read above") =
                Json::String(path.to_string_lossy().into_owned());
            if let Some(field) = field(entry, "sha256") {
                *field = Json::String(sha256);
            }
        }
        Ok(())
    }
}

fn field<'a>(value: &'a mut Json, name: &str) -> Option<&'a mut Json> {
    match value {
        Json::Object(fields) => fields
            .iter_mut()
            .find(|(field, _)| field == name)
            .map(|(_, value)| value),
        _ => None,
    }
}

fn referenced(entries: &[(String, Json)], group_root: &Path, paths: &mut BTreeSet<PathBuf>) {
    for (_, entry) in entries {
        if let Some(path) = entry.get("path").and_then(Json::as_str) {
            paths.insert(group_root.join(path));
        }
        if let Some(Json::Object(nested)) = entry.get("parts") {
            referenced(nested, group_root, paths);
        }
    }
}

/// Part files are content-addressed and shared between manifests of one group,
/// so a file is removed only when no manifest in the fixtures still names it.
pub(crate) fn unreferenced_parts(fixtures: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = BTreeSet::new();
    let mut part_directories = BTreeSet::new();
    for capture in captures(fixtures)? {
        if let Layout::Addressed { group_root } = &capture.layout {
            let manifest = Json::read(&capture.manifest)?;
            if let Some(Json::Object(entries)) = manifest.get("parts") {
                referenced(entries, group_root, &mut paths);
            }
            part_directories.insert(group_root.join("parts"));
            part_directories.insert(group_root.join("captured/parts"));
        }
    }
    let mut orphans = Vec::new();
    for directory in part_directories
        .iter()
        .filter(|directory| directory.is_dir())
    {
        for entry in fs::read_dir(directory).map_err(|error| error.to_string())? {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path.is_file() && !paths.contains(&path) {
                orphans.push(path);
            }
        }
    }
    Ok(orphans)
}

#[cfg(test)]
mod tests {
    use super::{captures, unreferenced_parts};
    use crate::{
        digest::digest as digest_of,
        json::{Json, pretty},
        native_capture, repository,
        testing::TemporaryDirectory,
    };

    #[test]
    fn every_capture_manifest_is_rewritten_without_change() {
        for capture in captures(&repository().join("tests/fixtures")).unwrap() {
            let text = std::fs::read_to_string(&capture.manifest).unwrap();
            let ending = if text.ends_with('\n') { "\n" } else { "" };
            let rewritten = pretty(&Json::parse(&text).unwrap()) + ending;
            let difference = text
                .lines()
                .zip(rewritten.lines())
                .find(|(stored, written)| stored != written);
            assert!(
                rewritten == text,
                "{}: {difference:?}",
                capture.manifest.display()
            );
        }
    }

    #[test]
    fn every_native_capture_manifest_fingerprints_its_files() {
        fn manifests(folder: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(folder).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    manifests(&path, found);
                } else if path
                    .file_name()
                    .is_some_and(|name| name == "native-capture.json")
                {
                    found.push(path);
                }
            }
        }
        let mut found = Vec::new();
        manifests(&repository().join("tests/fixtures"), &mut found);
        assert!(!found.is_empty());
        let mut stale = Vec::new();
        for manifest in found {
            let folder = manifest.parent().unwrap();
            let files = Json::read(&manifest).unwrap();
            for (name, digest) in files.get("files").and_then(Json::as_object).unwrap_or(&[]) {
                let Some(digest) = digest.as_str() else {
                    continue;
                };
                if crate::digest::fingerprint(&folder.join(name))
                    .ok()
                    .as_deref()
                    != Some(digest)
                {
                    stale.push(folder.join(name));
                }
            }
        }
        assert!(stale.is_empty(), "{stale:?}");
    }

    /// A capture folder that holds the given files and a manifest of the given
    /// text, in which `{NAME}` stands for the digest of the file of that name.
    fn capture_folder(files: &[(&str, &[u8])], manifest: &str) -> TemporaryDirectory {
        let folder = TemporaryDirectory::new();
        let mut manifest = manifest.to_owned();
        for (name, bytes) in files {
            std::fs::write(folder.0.join(name), bytes).unwrap();
            manifest = manifest.replace(&format!("{{{name}}}"), &digest_of(bytes));
        }
        std::fs::write(folder.0.join("capture.json"), manifest).unwrap();
        folder
    }

    fn manifest_of(folder: &TemporaryDirectory) -> String {
        std::fs::read_to_string(folder.0.join("capture.json")).unwrap()
    }

    fn refresh(folder: &TemporaryDirectory, name: &str, bytes: &[u8]) {
        native_capture::replace_recorded(&folder.0, "capture.json", name, bytes);
    }

    #[test]
    fn a_refreshed_capture_replaces_its_own_fingerprint_alone() {
        let text = r#"{
  "geometry_sha256": "{geometry.json}",
  "native_png_files": {
    "reference-1.png": "{reference-1.png}",
    "reference-2.png": "{reference-2.png}"
  }
}"#;
        let folder = capture_folder(
            &[
                ("geometry.json", b"{}"),
                ("reference-1.png", b"page"),
                ("reference-2.png", b"page"),
            ],
            text,
        );
        refresh(&folder, "reference-1.png", b"moved");
        refresh(&folder, "geometry.json", b"[]");
        let expected = text
            .replace("{geometry.json}", &digest_of(b"[]"))
            .replace("{reference-1.png}", &digest_of(b"moved"))
            .replace("{reference-2.png}", &digest_of(b"page"));
        assert_eq!(manifest_of(&folder), expected);
        let stored = |name: &str| std::fs::read(folder.0.join(name)).unwrap();
        assert_eq!(stored("reference-1.png"), b"moved");
        assert_eq!(stored("reference-2.png"), b"page");
        assert_eq!(stored("geometry.json"), b"[]");
    }

    #[test]
    fn a_refresh_fails_when_the_manifest_records_another_digest() {
        let stale = digest_of(b"an earlier page");
        for text in [
            format!(r#"{{"files": {{"reference-1.png": "{stale}"}}}}"#),
            format!(r#"{{"reference_1_sha256": "{stale}"}}"#),
            r#"{"pages": [{"path": "reference-1.png", "sha256": "{reference-1.png}"}]}"#.to_owned(),
        ] {
            let folder = capture_folder(&[("reference-1.png", b"page")], &text);
            let recorded = manifest_of(&folder);
            let refreshed =
                std::panic::catch_unwind(|| refresh(&folder, "reference-1.png", b"moved"));
            assert!(refreshed.is_err(), "{text}");
            assert_eq!(manifest_of(&folder), recorded);
            assert_eq!(
                std::fs::read(folder.0.join("reference-1.png")).unwrap(),
                b"page"
            );
        }
    }

    #[test]
    fn a_refresh_leaves_a_manifest_without_a_fingerprint_of_the_file() {
        let text = r#"{"pages": 1, "native_png_dpi": 72}"#;
        let folder = capture_folder(&[("reference-1.png", b"page")], text);
        refresh(&folder, "reference-1.png", b"moved");
        assert_eq!(manifest_of(&folder), text);
        assert_eq!(
            std::fs::read(folder.0.join("reference-1.png")).unwrap(),
            b"moved"
        );
    }

    #[test]
    fn concurrent_refreshes_of_one_manifest_keep_every_fingerprint() {
        let names: Vec<String> = (1..=16)
            .map(|number| format!("reference-{number}.png"))
            .collect();
        let files: Vec<(&str, &[u8])> = names
            .iter()
            .map(|name| (name.as_str(), name.as_bytes()))
            .collect();
        let members: Vec<String> = names
            .iter()
            .map(|name| format!(r#""{name}": "{{{name}}}""#))
            .collect();
        for round in 0..20 {
            let folder = capture_folder(&files, &format!("{{{}}}", members.join(", ")));
            let start = std::sync::Barrier::new(names.len());
            std::thread::scope(|scope| {
                for name in &names {
                    let (folder, start) = (&folder, &start);
                    scope.spawn(move || {
                        start.wait();
                        refresh(folder, name, format!("{name} moved").as_bytes());
                    });
                }
            });
            let manifest = manifest_of(&folder);
            for name in &names {
                let expected = digest_of(format!("{name} moved").as_bytes());
                assert!(
                    manifest.contains(&format!(r#""{name}": "{expected}""#)),
                    "round {round}: the fingerprint of {name} was lost"
                );
            }
        }
    }

    #[test]
    fn every_captured_part_file_is_referenced() {
        let orphans = unreferenced_parts(&repository().join("tests/fixtures")).unwrap();
        assert!(orphans.is_empty(), "{orphans:?}");
    }
}
