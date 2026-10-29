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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/captions")
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
fn captions_preserve_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "basic",
        "flow",
        "formatting",
        "grouping",
        "image",
        "metadata",
        "nested",
        "position",
        "sequences",
        "stories",
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
fn captions_and_figures_reject_invalid_structure_and_attributes() {
    for source in [
        "#figure",
        "#caption",
        "#figure(1)[]",
        "#caption(1)[]",
        "#figure[][]",
        "#caption[][]",
        "#figure[#caption[One]#caption[Two]]",
        "#caption(position=\"middle\")[Text]",
        "#caption(alignment=\"top\")[Text]",
        "#caption(numbering={start=0})[Text]",
        "#caption(numbering={start=1.5})[Text]",
        "#caption(numbering={format=\"unknown\"})[Text]",
        "#caption(numbering={enabled=\"maybe\"})[Text]",
        "#caption(numbering={restart=\"maybe\"})[Text]",
        "#caption(numbering=2)[Text]",
        "#figure(placement={width=-1})[Text]",
        "#figure(placement={type=\"unknown\"})[Text]",
        "#figure(placement={horizontal={relative_to=\"line\"}})[Text]",
        "#figure(placement={vertical={relative_to=\"column\"}})[Text]",
        "#figure(placement={relative_height=-1})[Text]",
        "#figure(style=2)[Text]",
        "#caption(style=2)[Text]",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
}
