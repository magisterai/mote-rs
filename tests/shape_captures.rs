#[allow(dead_code)]
#[path = "docx/support.rs"]
mod xml_support;
use mote::source::FileAssetResolver;
use std::{
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};
fn fixtures(directory: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(directory)
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
fn shapes_preserve_models_and_complete_editable_packages() {
    for (directory, cases) in [
        (
            "shape-output",
            &["properties", "presets", "defaults", "text", "media", "lua"][..],
        ),
        (
            "shape-text",
            &["fitting", "formatting", "presets", "rotation"][..],
        ),
    ] {
        let root = fixtures(directory);
        let resolver = FileAssetResolver::new(&root).unwrap();
        for case in cases {
            let source = fs::read_to_string(root.join(format!("{case}.mote"))).unwrap();
            let source_name = root.join(format!("{case}.mote"));
            let document = mote::compile_with_options(
                &source,
                mote::evaluation::EvaluationOptions {
                    enable_lua: true,
                    asset_resolver: Some(&resolver),
                    source_name: Some(source_name.as_os_str()),
                    ..Default::default()
                },
            )
            .unwrap();
            let original = document.clone();
            let capture = root.join("captured").join(case);
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
            use sha2::{Digest, Sha256};
            assert_eq!(
                format!("{:x}", Sha256::digest(source.as_bytes())),
                manifest["source_sha256"]
            );
            for (path, expected) in manifest["assets"].as_object().unwrap() {
                assert_eq!(
                    format!("{:x}", Sha256::digest(fs::read(root.join(path)).unwrap())),
                    expected.as_str().unwrap()
                );
            }
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
}
