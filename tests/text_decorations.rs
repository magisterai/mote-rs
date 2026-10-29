#[path = "support/json_compare.rs"]
mod json_compare;
use mote::{
    layout::{DecorationKind, LayoutStyleResolver, plan_paragraph_text},
    measurement::TextMeasurer,
    model::Content,
    source::FileAssetResolver,
};
use serde_json::{Value, json};
use std::{fs, path::Path};
#[test]
fn text_decoration_geometry_matches_captured_operations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/text-decorations");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let assets = FileAssetResolver::from_current_directory().unwrap();
    let mut cases = Vec::new();
    for case in requests["cases"].as_array().unwrap() {
        let document = mote::compile(case["source"].as_str().unwrap()).unwrap();
        let styles = LayoutStyleResolver::new(&document);
        let mut measurer = TextMeasurer::new(&document, &assets, None);
        let Content::Element(element) = &document.content[0] else {
            panic!("paragraph missing")
        };
        let plan = plan_paragraph_text(
            element,
            case["width"].as_f64().unwrap(),
            &styles,
            &mut measurer,
            true,
        )
        .unwrap();
        cases.push(json!({"id":case["id"],"lines":plan.lines.iter().map(|line|json!({"highlights":line.highlights.iter().map(|rectangle|json!({"x":rectangle.x,"top":rectangle.top,"width":rectangle.width,"height":rectangle.height,"color":rectangle.color})).collect::<Vec<_>>(),"decorations":line.decorations.iter().map(|decoration|json!({"kind":match decoration.kind{DecorationKind::Underline=>"underline",DecorationKind::Strike=>"strike"},"start":decoration.start,"end":decoration.end,"y":decoration.y,"width":decoration.width,"color":decoration.color})).collect::<Vec<_>>()})).collect::<Vec<_>>()}));
    }
    assert_eq!(cases.len(), 90);
    let actual = json!({"schema_version":1,"cases":cases});
    let mut differences = Vec::new();
    json_compare::compare(&expected, &actual, "root", &mut differences);
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
fn decorations_follow_text_into_absolute_document_positions() {
    use mote::layout::plan_document_text;
    let source = "#paragraph(indent={left=pt(12)})[#span(highlight=\"#ff8\",underline=true,double_strike=true)[Marked words]]";
    let document = mote::compile(source).unwrap();
    let assets = FileAssetResolver::from_current_directory().unwrap();
    let styles = LayoutStyleResolver::new(&document);
    let mut measurer = TextMeasurer::new(&document, &assets, None);
    let Content::Element(element) = &document.content[0] else {
        panic!()
    };
    let relative = plan_paragraph_text(element, 468.0, &styles, &mut measurer, true).unwrap();
    let absolute = plan_document_text(&document, &mut measurer).unwrap();
    let line = &absolute.pages[0].lines[0].line;
    assert_eq!(line.highlights.len(), 1);
    assert_eq!(line.decorations.len(), 5);
    for (relative, absolute) in relative.lines[0].highlights.iter().zip(&line.highlights) {
        assert!((absolute.x - relative.x - 72.0).abs() < 1e-9);
        assert!((absolute.top - relative.top - 72.0).abs() < 1e-9);
        assert_eq!(relative.color, absolute.color);
    }
    for (relative, absolute) in relative.lines[0].decorations.iter().zip(&line.decorations) {
        assert!((absolute.start - relative.start - 72.0).abs() < 1e-9);
        assert!((absolute.end - relative.end - 72.0).abs() < 1e-9);
        assert!((absolute.y - relative.y - 72.0).abs() < 1e-9);
        assert_eq!(relative.width, absolute.width);
    }
}
