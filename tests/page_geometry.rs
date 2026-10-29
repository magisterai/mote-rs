#[path = "support/json_compare.rs"]
mod json_compare;
use mote::layout::PageGeometry;
use serde_json::{Value, json};
use std::{fs, path::Path};
#[test]
fn source_page_and_column_geometry_matches_captures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/page-geometry");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut cases = Vec::new();
    for case in requests["cases"].as_array().unwrap() {
        let document = mote::compile(case["source"].as_str().unwrap()).unwrap();
        let geometry = PageGeometry::from_document(&document).unwrap();
        cases.push(json!({"id":case["id"],"width":geometry.width,"height":geometry.height,"margin_top":geometry.margin_top,"margin_right":geometry.margin_right,"margin_bottom":geometry.margin_bottom,"margin_left":geometry.margin_left,"header_space":geometry.header_space,"footer_space":geometry.footer_space,"column_count":geometry.column_count,"column_gap":geometry.column_gap,"column_separator":geometry.column_separator,"content_width":geometry.content_width(),"content_bottom":geometry.content_bottom(),"column_width":geometry.column_width(),"column_left":(0..geometry.column_count).map(|index|geometry.column_left(index)).collect::<Vec<_>>()}));
    }
    assert_eq!(cases.len(), 23);
    let actual = json!({"schema_version":1,"cases":cases});
    let mut differences = Vec::new();
    json_compare::compare(&expected, &actual, "root", &mut differences);
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}
#[test]
fn geometry_rejects_nonfinite_or_empty_content_areas() {
    for geometry in [
        PageGeometry {
            width: f64::INFINITY,
            ..Default::default()
        },
        PageGeometry {
            height: 100.0,
            ..Default::default()
        },
        PageGeometry {
            column_count: 0,
            ..Default::default()
        },
        PageGeometry {
            width: 144.0,
            ..Default::default()
        },
    ] {
        assert!(geometry.validate().is_err());
    }
}
