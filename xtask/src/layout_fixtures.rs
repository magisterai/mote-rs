//! Integrity of the frozen native layout captures, verified without generating
//! new expectations. Agreement of the implementation is the business of `cargo test`.
use crate::{
    Outcome,
    arguments::{Arguments, unknown},
    json::Json,
    repository,
};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path, process::ExitCode};

const HELP: &str = "Usage: cargo xtask verify-layout [--fixtures PATH]

Verify the frozen native layout collections: file fingerprints, strict JSON,
unique case identities and exact case coverage. No renderer is run.
";

const COLLECTIONS: [&str; 9] = [
    "lines",
    "layout-styles",
    "paragraph-text",
    "paragraph-flow",
    "page-geometry",
    "document-text",
    "text-decorations",
    "native-blocks",
    "native-columns",
];

pub(crate) fn run(mut arguments: Arguments) -> Outcome {
    let mut fixtures = repository().join("tests/fixtures");
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(ExitCode::SUCCESS);
            }
            "--fixtures" => fixtures = arguments.path("--fixtures")?,
            option => return Err(unknown(option)),
        }
    }
    let mut cases = 0;
    for name in COLLECTIONS {
        cases += verify_fixture(&fixtures.join(name), &fixtures)
            .map_err(|error| format!("{name}: {error}"))?;
    }
    println!(
        "{}",
        serde_json::json!({
            "status": "fixture_verified",
            "collections": COLLECTIONS.len(),
            "cases": cases,
        })
    );
    Ok(ExitCode::SUCCESS)
}

fn case_identities(document: &Json) -> Result<Vec<String>, String> {
    if document.get("schema_version").and_then(Json::as_integer) != Some(1) {
        return Err("unsupported capture schema".into());
    }
    let cases = document
        .get("cases")
        .and_then(Json::as_array)
        .ok_or("cases must be an array")?;
    let mut identities = Vec::with_capacity(cases.len());
    for case in cases {
        case.as_object().ok_or("cases must contain objects")?;
        let identity = case
            .get("id")
            .and_then(Json::as_str)
            .filter(|identity| !identity.is_empty())
            .ok_or("case identities must be nonempty strings")?;
        identities.push(identity.to_owned());
    }
    if identities.iter().collect::<BTreeSet<_>>().len() != identities.len() {
        return Err("duplicate case identity".into());
    }
    Ok(identities)
}

/// The capture names each file with its length and SHA-256, so that a changed
/// byte fails before any case is compared.
pub(crate) fn verify_fixture(directory: &Path, fixture_root: &Path) -> Result<usize, String> {
    let capture = Json::read(&directory.join("capture.json"))?;
    if capture.get("schema_version").and_then(Json::as_integer) != Some(1) {
        return Err("unsupported capture metadata schema".into());
    }
    let case_count = capture
        .get("case_count")
        .and_then(Json::as_integer)
        .filter(|count| *count > 0)
        .ok_or("case count must be a positive integer")?;
    if capture.get("units").and_then(Json::as_str) != Some("points")
        || capture.get("absolute_tolerance").and_then(Json::as_number) != Some(1e-9)
    {
        return Err("unexpected measurement units or tolerance".into());
    }
    let root = fixture_root
        .canonicalize()
        .map_err(|error| format!("{}: {error}", fixture_root.display()))?;
    let mut verified = Vec::new();
    for (name, identity) in capture
        .get("files")
        .and_then(Json::as_object)
        .ok_or("files must be an object")?
    {
        let path = directory
            .join(name)
            .canonicalize()
            .map_err(|error| format!("{name}: {error}"))?;
        if !path.starts_with(&root) {
            return Err(format!("{name} lies outside the fixtures"));
        }
        let bytes = fs::read(&path).map_err(|error| format!("{name}: {error}"))?;
        let length = identity.get("bytes").and_then(Json::as_integer);
        let digest = identity.get("sha256").and_then(Json::as_str);
        if length != Some(bytes.len() as i128)
            || digest != Some(format!("{:x}", Sha256::digest(&bytes)).as_str())
        {
            return Err(format!("fingerprint differs: {name}"));
        }
        let text = String::from_utf8(bytes).map_err(|error| format!("{name}: {error}"))?;
        verified.push((
            name.as_str(),
            Json::parse(&text).map_err(|error| format!("{name}: {error}"))?,
        ));
    }
    let requests: Vec<_> = verified
        .iter()
        .filter(|(name, _)| name.ends_with("requests.json"))
        .map(|(_, document)| document)
        .collect();
    let expected = verified
        .iter()
        .find(|(name, _)| *name == "expected.json")
        .map(|(_, document)| document);
    let ([requests], Some(expected)) = (requests.as_slice(), expected) else {
        return Err("one request document and expected.json are required".into());
    };
    let mut requested = case_identities(requests)?;
    let captured = case_identities(expected)?;
    if let Some(widths) = capture
        .get("widths_points")
        .filter(|widths| **widths != Json::Null)
    {
        let widths: Vec<f64> = widths
            .as_array()
            .filter(|widths| !widths.is_empty())
            .and_then(|widths| widths.iter().map(Json::as_number).collect::<Option<_>>())
            .filter(|widths: &Vec<f64>| widths.iter().all(|width| *width > 0.0))
            .ok_or("widths must be positive numbers")?;
        if widths
            .iter()
            .map(|width| width.to_bits())
            .collect::<BTreeSet<_>>()
            .len()
            != widths.len()
        {
            return Err("duplicate width".into());
        }
        requested = requested
            .iter()
            .flat_map(|identity| {
                widths
                    .iter()
                    .map(move |width| format!("{identity}-{width}"))
            })
            .collect();
    }
    if requested.iter().collect::<BTreeSet<_>>() != captured.iter().collect::<BTreeSet<_>>()
        || captured.len() as i128 != case_count
    {
        return Err("captured case coverage differs from the requests".into());
    }
    if let Some(rejected_plans) = capture.get("rejected_plans") {
        let rejected = expected
            .get("cases")
            .and_then(Json::as_array)
            .into_iter()
            .flatten()
            .filter(|case| case.get("invalid_line_height") == Some(&Json::Boolean(true)))
            .count() as i128;
        let successful_plans = capture.get("successful_plans").and_then(Json::as_integer);
        if rejected_plans.as_integer() != Some(rejected)
            || successful_plans != Some(captured.len() as i128 - rejected)
        {
            return Err("successful and rejected plan counts differ".into());
        }
    }
    Ok(captured.len())
}

#[cfg(test)]
mod tests {
    use super::verify_fixture;
    use sha2::{Digest, Sha256};
    use std::{fs, path::PathBuf};

    struct Collection {
        root: PathBuf,
        directory: PathBuf,
    }

    impl Collection {
        fn new(name: &str) -> Self {
            let root =
                std::env::temp_dir().join(format!("xtask-layout-{name}-{}", std::process::id()));
            let directory = root.join("lines");
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(&directory).unwrap();
            let collection = Self { root, directory };
            let document = r#"{"schema_version": 1, "cases": [{"id": "example"}]}"#;
            collection.write("requests.json", document);
            collection.write("expected.json", document);
            collection
        }
        fn write(&self, name: &str, text: &str) {
            fs::write(self.directory.join(name), text).unwrap();
            let files: Vec<String> = ["requests.json", "expected.json"]
                .iter()
                .map(|name| {
                    let bytes = fs::read(self.directory.join(name)).unwrap_or_default();
                    format!(
                        r#""{name}": {{"bytes": {}, "sha256": "{:x}"}}"#,
                        bytes.len(),
                        Sha256::digest(&bytes)
                    )
                })
                .collect();
            fs::write(
                self.directory.join("capture.json"),
                format!(
                    r#"{{"schema_version": 1, "case_count": 1, "units": "points", "absolute_tolerance": 1e-9, "files": {{{}}}}}"#,
                    files.join(", ")
                ),
            )
            .unwrap();
        }
        fn verify(&self) -> Result<usize, String> {
            verify_fixture(&self.directory, &self.root)
        }
    }

    impl Drop for Collection {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn unchanged_capture_verifies() {
        assert_eq!(Collection::new("unchanged").verify(), Ok(1));
    }

    #[test]
    fn changed_bytes_fail_before_comparison() {
        let collection = Collection::new("changed");
        let path = collection.directory.join("expected.json");
        let mut text = fs::read_to_string(&path).unwrap();
        text.push(' ');
        fs::write(path, text).unwrap();
        assert!(collection.verify().unwrap_err().contains("fingerprint"));
    }

    #[test]
    fn duplicate_fields_are_rejected_even_with_matching_fingerprint() {
        let collection = Collection::new("duplicate");
        collection.write(
            "expected.json",
            r#"{"schema_version":1,"schema_version":1,"cases":[{"id":"example"}]}"#,
        );
        assert!(collection.verify().unwrap_err().contains("duplicate JSON"));
    }

    #[test]
    fn case_identity_must_cover_the_request() {
        let collection = Collection::new("coverage");
        collection.write(
            "expected.json",
            r#"{"schema_version": 1, "cases": [{"id": "different"}]}"#,
        );
        assert!(collection.verify().unwrap_err().contains("coverage"));
    }
}
