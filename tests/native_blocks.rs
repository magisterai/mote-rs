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
        if !differences.is_empty() && json_compare::refresh(&root, &actual) {
            continue;
        }
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
fn excessive_list_counters_fail_explicitly() {
    let resolver = FileAssetResolver::from_current_directory().unwrap();
    let source = "#ordered_list(format=\"upper_alpha\",start=1000000)[#item[Excessive counter]]";
    let document = mote::compile(source).unwrap();
    assert!(render::plan(&document, &resolver, None).is_err());
    assert!(render::emit_pdf(&document, &resolver, None).is_err());
    assert!(render::emit_png(&document, &resolver, None, &Default::default()).is_err());
}

#[test]
fn paragraph_borders_paint_each_side_outside_the_text_and_reserve_their_height() {
    let paragraph = "#paragraph(border = {left = {width = pt(3), color = \"#e0902f\", space = 6}, top = {width = pt(1), space = 4}, bottom = \"single\"})[Callout]";
    let plan = |source: &str| {
        let document = mote::compile(&format!(
            "#document(margin = {{left = inch(1), top = inch(1)}})\n{source}"
        ))
        .unwrap();
        let resolver = FileAssetResolver::from_current_directory().unwrap();
        render::plan(&document, &resolver, None).unwrap().output
    };
    let bordered = plan(paragraph);
    let page = &bordered.pages[0];
    let rules: Vec<_> = page
        .operations
        .iter()
        .filter_map(|operation| match operation {
            mote::layout::PageOperation::Rule(rule) => Some(rule),
            _ => None,
        })
        .collect();
    assert_eq!(rules.len(), 3);
    let left = rules.iter().find(|rule| rule.color == "E0902F").unwrap();
    assert_eq!(left.start[0], 72.0 - 6.0 - 1.5);
    assert_eq!(left.width, 3.0);
    let top = rules.iter().find(|rule| rule.width == 1.0).unwrap();
    assert_eq!(top.start[1], 72.0 + 0.5);
    let bottom = rules.iter().find(|rule| rule.width == 0.5).unwrap();
    assert_eq!(bottom.color, "BFBFBF");

    let plain = plan("#paragraph[Callout]");
    let text_top =
        |plan: &mote::layout::DocumentTextPlan| plan.pages[0].lines[0].line.fragments[0].top;
    assert!((text_top(&bordered) - text_top(&plain) - 5.0).abs() < 1e-9);
}
