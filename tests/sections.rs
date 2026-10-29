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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/sections-stories")
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
fn sections_preserve_models_and_complete_editable_packages() {
    let resolver = FileAssetResolver::new(fixtures()).unwrap();
    for case in [
        "balanced",
        "basic",
        "continuous",
        "default-changes",
        "empty-story-lines",
        "field-flow",
        "field-formats",
        "geometry",
        "local-stories",
        "overflow",
        "page-sizes",
        "rich-stories",
        "starts",
        "story-media",
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
fn section_and_story_validation_is_explicit() {
    for source in [
        "#section",
        "#section()",
        "#section(1)[]",
        "#section[][]",
        "#header",
        "#footer()",
        "#header(2)[]",
        "#header(unknown=1)[]",
        "#header(height=-1)[]",
        "#footer(distance=-1)[]",
        "#header(properties=1)[]",
        "#section(page_width=1)[]",
        "#section(properties={start=\"invalid\"})[]",
        "#section(properties={break=\"page\"})[]",
        "#section(vertical_alignment=\"top\")[]",
        "#section(page_numbering={start=0})[]",
        "#section(page_numbering={start=1.5})[]",
        "#section(page_numbering={restart=\"invalid\"})[]",
        "#section(page_number_start=1)[]",
        "#page_number(1)",
        "#page_number[]",
        "#page_number(kind=\"invalid\")",
    ] {
        assert!(mote::compile(source).is_err(), "{source}");
    }
}
#[test]
fn story_declarations_and_empty_sections_do_not_duplicate_body_content() {
    let document = mote::compile("#header[Unused] #footer[Unused]").unwrap();
    assert!(document.content.is_empty());
    assert!(document.sections.is_empty());
    let document = mote::compile("#section[]").unwrap();
    assert_eq!(document.sections.len(), 1);
    assert!(document.sections[0].explicit);
    assert!(document.content.is_empty());
}
