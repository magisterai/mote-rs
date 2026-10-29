#[allow(dead_code)]
#[path = "docx/support.rs"]
mod xml_support;
use mote::source::FileAssetResolver;
use std::{
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/notes")
}
fn normalize_numbers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            *number = serde_json::Number::from_f64(number.as_f64().unwrap()).unwrap()
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        serde_json::Value::Object(fields) => fields.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}
#[test]
fn notes_preserve_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "basic",
        "markdown",
        "rich",
        "stories",
        "pressure",
        "multiple",
        "endnote-flow",
        "endnote-footer",
        "fallbacks",
        "fields",
        "overflow",
    ] {
        let source = fs::read_to_string(fixtures().join(format!("{case}.mote"))).unwrap();
        let document = mote::compile(&source).unwrap();
        let original = document.clone();
        let capture = fixtures().join("captured").join(case);
        let mut actual: serde_json::Value =
            serde_json::from_str(&mote::json::to_json(&document).unwrap()).unwrap();
        let mut expected: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("model.json")).unwrap()).unwrap();
        normalize_numbers(&mut actual);
        normalize_numbers(&mut expected);
        assert_eq!(actual, expected, "{case}: model");
        let generated = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
        let mut archive = zip::ZipArchive::new(Cursor::new(generated.bytes)).unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(capture.join("capture.json")).unwrap()).unwrap();
        let names = manifest["parts"].as_array().unwrap();
        assert_eq!(archive.len(), names.len());
        for name in names {
            let name = name.as_str().unwrap();
            let mut actual = Vec::new();
            archive
                .by_name(name)
                .unwrap()
                .read_to_end(&mut actual)
                .unwrap();
            let expected = fs::read(capture.join("package").join(name)).unwrap();
            if name.ends_with(".xml") || name.ends_with(".rels") {
                xml_support::compare_xml(
                    std::str::from_utf8(&actual).unwrap(),
                    std::str::from_utf8(&expected).unwrap(),
                    &format!("{case}/{name}"),
                );
            } else {
                assert_eq!(actual, expected, "{case}/{name}");
            }
        }
        assert_eq!(document, original);
    }
}

#[test]
fn notes_validate_body_and_identifier_contracts() {
    for source in [
        "#footnote()",
        "#endnote(1)[]",
        "#footnote[][]",
        "#footnote(id=true)[]",
        "#endnote(id={value=1})[]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    assert!(mote::compile("#paragraph[Text#footnote(id=\"\")[#paragraph[Note]]]").is_ok());
    assert!(mote::compile("#paragraph[Text#endnote(id=3.5)[]]").is_ok());
}
#[test]
fn markdown_note_definitions_are_lazy_and_reject_recursion() {
    assert!(mote::compile("Visible.\n\n[^unused]: #unknown[]").is_ok());
    for source in [
        "Reference[^a].\n\n[^a]: Recursive[^a]",
        "Reference[^a].\n\n[^a]: Nested[^b]\n[^b]: Cycle[^a]",
    ] {
        let error = mote::compile(source).unwrap_err();
        assert!(error.message.contains("recursively"), "{error}");
    }
}
#[test]
fn note_placement_fallbacks_are_reported() {
    let document =
        mote::compile("#header[Header#footnote[Note]]\nBody#endnote[Outer#footnote[Inner]]")
            .unwrap();
    let resolver = FileAssetResolver::new(".").unwrap();
    let package = mote::docx::emit_with_assets(&document, &resolver, None).unwrap();
    assert_eq!(package.warnings.len(), 1);
    assert!(package.warnings[0].contains("flattened footnote"));
}

#[test]
fn note_capture_hashes_bind_sources_and_native_artifacts() {
    use sha2::{Digest, Sha256};
    let digest = |bytes: &[u8]| format!("{:x}", Sha256::digest(bytes));
    for entry in fs::read_dir(fixtures().join("captured")).unwrap() {
        let folder = entry.unwrap().path();
        let name = folder.file_name().unwrap().to_str().unwrap();
        let source = fs::read(fixtures().join(format!("{name}.mote"))).unwrap();
        for manifest_name in ["capture.json", "native-capture.json"] {
            let manifest: serde_json::Value =
                serde_json::from_slice(&fs::read(folder.join(manifest_name)).unwrap()).unwrap();
            assert_eq!(
                digest(&source),
                manifest["source_sha256"],
                "{name}/{manifest_name}"
            );
            if let Some(files) = manifest["files"].as_object() {
                for (path, expected) in files {
                    assert_eq!(
                        digest(&fs::read(folder.join(path)).unwrap()),
                        expected.as_str().unwrap(),
                        "{name}/{path}"
                    );
                }
            }
        }
    }
}
