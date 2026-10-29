#[path = "support/json_compare.rs"]
mod json_compare;
use json_compare::compare;
#[path = "../examples/support/line_capture.rs"]
mod line_capture;
use serde_json::Value;
use std::{fs, path::Path};

#[test]
fn captured_line_layout_matches_every_fragment_and_break() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lines");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let actual = line_capture::measure_cases(&root, &requests).unwrap();
    assert_eq!(requests["cases"].as_array().unwrap().len(), 342);
    let mut differences = Vec::new();
    compare(&expected, &actual, "root", &mut differences);
    assert!(
        differences.is_empty(),
        "{} differences:\n{}",
        differences.len(),
        differences
            .iter()
            .take(30)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn invalid_geometry_fails_even_without_visible_runs() {
    use mote::{
        layout::{LineOptions, LineSpacing, plan_text},
        measurement::TextMeasurer,
        model::Document,
        source::FileAssetResolver,
    };
    let document = Document::default();
    let resolver = FileAssetResolver::from_current_directory().unwrap();
    let mut measurer = TextMeasurer::new(&document, &resolver, None);
    let invalid = [
        LineOptions {
            width: f64::NAN,
            ..Default::default()
        },
        LineOptions {
            first_indent: f64::INFINITY,
            ..Default::default()
        },
        LineOptions {
            spacing: LineSpacing::Multiple(f64::MAX),
            ..Default::default()
        },
        LineOptions {
            fallback_multiplier: 0.0,
            ..Default::default()
        },
    ];
    for options in invalid {
        assert!(plan_text(&[], &options, &mut measurer).is_err());
    }
    let mut options = LineOptions::default();
    options.base_style.measurement.size_points = f64::NAN;
    assert!(plan_text(&[], &options, &mut measurer).is_err());
    options.base_style.measurement.size_points = 11.0;
    options.base_style.word_line_height = Some(f64::INFINITY);
    assert!(plan_text(&[], &options, &mut measurer).is_err());
}
