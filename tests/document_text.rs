#[path = "../examples/support/document_capture.rs"]
mod document_capture;
#[path = "support/json_compare.rs"]
mod json_compare;
use mote::{layout::plan_document_text, measurement::TextMeasurer, source::FileAssetResolver};
use serde_json::Value;
use std::{fs, path::Path};
#[test]
fn complete_sources_match_captured_page_text_geometry() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/document-text");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let actual = document_capture::capture_sources(&requests).unwrap();
    let mut differences = Vec::new();
    json_compare::compare(&expected, &actual, "root", &mut differences);
    assert!(
        differences.is_empty(),
        "{} differences:\n{}",
        differences.len(),
        differences
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn document_text_scope_rejects_unplanned_structure() {
    let assets = FileAssetResolver::from_current_directory().unwrap();
    {
        let source = "#document(columns={count=2})\nText";
        let document = mote::compile(source).unwrap();
        let mut measurer = TextMeasurer::new(&document, &assets, None);
        assert!(
            plan_document_text(&document, &mut measurer).is_err(),
            "{source}"
        );
    }
}
