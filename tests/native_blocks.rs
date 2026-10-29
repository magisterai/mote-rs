#[path = "../examples/support/document_capture.rs"]
mod document_capture;
#[path = "support/json_compare.rs"]
mod json_compare;
use mote::{render, source::FileAssetResolver};
use serde_json::Value;
use std::{fs, path::Path};

#[test]
fn block_flow_and_ordered_paint_operations_match_independent_captures() {
    for name in ["native-blocks", "native-columns"] {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{name}"));
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
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
#[test]
fn unpaintable_marker_geometry_and_excessive_counters_fail_explicitly() {
    let resolver = FileAssetResolver::from_current_directory().unwrap();
    for source in [
        "#list(indent={hanging=pt(0)})[#item[No marker slot]]",
        "#ordered_list(format=\"upper_alpha\",start=1000000)[#item[Excessive counter]]",
    ] {
        let document = mote::compile(source).unwrap();
        assert!(
            render::plan(&document, &resolver, None).is_err(),
            "{source}"
        );
        assert!(
            render::emit_pdf(&document, &resolver, None).is_err(),
            "{source}"
        );
        assert!(
            render::emit_png(&document, &resolver, None, &Default::default()).is_err(),
            "{source}"
        );
    }
}
