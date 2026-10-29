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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/control-flow")
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
fn control_flow_preserves_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "branches",
        "iterations",
        "structures",
        "truthiness",
        "markdown",
        "stories",
        "boundaries",
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
fn control_capture_sources_remain_frozen() {
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

#[test]
fn controls_keep_scopes_local_and_evaluate_only_selected_bodies() {
    for source in [
        "#if(true)[#let(local=1)]#local",
        "#for local in {1} {}#local",
        "#let(outer=1)#for value in {1,2} {#if(value)[#let(hidden=1)]#hidden}",
        "#for value in 1 {}",
        "#for value in nil {}",
        "#for value in \"text\" {}",
        "#paragraph[#if(true)[#paragraph[Block]]]",
        "#paragraph[#for value in {1} {#paragraph[Block]}]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
    let resolver =
        |_: mote::source::AssetRequest<'_>| -> std::io::Result<mote::source::ResolvedAsset> {
            panic!("unselected branch loaded data")
        };
    let source = "#if(false)[#json(\"unread.json\")] else [Selected]#for value in {} {#missing}";
    let document = mote::compile_with_assets(source, &resolver, None).unwrap();
    assert_eq!(document, mote::compile("Selected").unwrap());
    let source =
        "#let(value=7)#for value in {1,2} {#paragraph[#value]#let(value=99)}#paragraph[#value]";
    assert_eq!(
        mote::compile(source).unwrap(),
        mote::compile("#paragraph[1]#paragraph[2]#paragraph[7]").unwrap()
    );
}

#[test]
fn malformed_controls_fail_with_bounded_source_ranges() {
    for source in [
        "#if",
        "#if()[]",
        "#if(true)",
        "#if(true)[] else",
        "#for in {} {}",
        "#for item of {} {}",
        "#for item in {}",
        "#for item in {1} {missing",
        "#if(false)[#paragraph[",
    ] {
        let error = mote::syntax::parse(source).expect_err(source);
        assert!(
            error.span.start <= error.span.end && error.span.end <= source.len(),
            "{source}: {error}"
        );
    }
}
