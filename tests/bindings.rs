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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bindings")
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
fn bindings_preserve_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "values",
        "scopes",
        "declarations",
        "paths",
        "markdown",
        "notes",
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
fn bindings_reject_invalid_paths_and_declarations_with_source_locations() {
    for source in [
        "#let(1)",
        "#let(name=1)[]",
        "#missing",
        "#let(first=later, later=1)",
        "#paragraph[#let(local=1)]#local",
        "#let(value={field=1}, result=value.missing)",
        "#let(value={1}, result=value.1)",
        "#let(value={1}, result=value.index)",
        "#let(value=1, result=value.child)",
    ] {
        let error = mote::compile(source).expect_err(source);
        assert!(error.span.end <= source.len());
        assert!(error.span.start < error.span.end, "{source}: {error}");
    }
    assert_eq!(mote::compile("#let()").unwrap(), mote::compile("").unwrap());
}

#[test]
fn binding_capture_sources_remain_frozen() {
    use sha2::{Digest, Sha256};
    for entry in fs::read_dir(fixtures().join("captured")).unwrap() {
        let folder = entry.unwrap().path();
        let source = fs::read(fixtures().join(format!(
            "{}.mote",
            folder.file_name().unwrap().to_str().unwrap()
        )))
        .unwrap();
        let manifest: serde_json::Value =
            serde_json::from_slice(&fs::read(folder.join("capture.json")).unwrap()).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(source)),
            manifest["source_sha256"]
        );
    }
}
