#[path = "support/json_compare.rs"]
mod json_compare;
use mote::{
    layout::{LayoutStyleResolver, LineBreak, plan_paragraph_text},
    measurement::TextMeasurer,
    model::Content,
    source::FileAssetResolver,
};
use serde_json::{Value, json};
use std::{fs, path::Path};
#[test]
fn source_to_paragraph_lines_matches_all_captured_widths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("layout-styles/requests.json")).unwrap())
            .unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("paragraph-text/expected.json")).unwrap())
            .unwrap();
    let assets = FileAssetResolver::from_current_directory().unwrap();
    let mut cases = Vec::new();
    for case in requests["cases"].as_array().unwrap() {
        for width in [12.0, 72.0, 300.0] {
            let document = mote::compile(case["source"].as_str().unwrap()).unwrap();
            let styles = LayoutStyleResolver::new(&document);
            let mut measurer = TextMeasurer::new(&document, &assets, None);
            let mut paragraphs = Vec::new();
            for node in &document.content {
                let Content::Element(element) = node else {
                    continue;
                };
                let plan =
                    plan_paragraph_text(element, width, &styles, &mut measurer, true).unwrap();
                paragraphs.push(json!({"first_frame":{"x":plan.first_frame.x,"width":plan.first_frame.width},"continuation_frame":{"x":plan.continuation_frame.x,"width":plan.continuation_frame.width},"lines":plan.lines.iter().map(|line|json!({"x":line.x,"width":line.width,"height":line.height,"baseline":line.baseline,"break":match line.line_break{LineBreak::Final=>0,LineBreak::Soft=>1,LineBreak::Hard=>2},"fragments":line.fragments.iter().map(|fragment|json!({"text":fragment.text,"x":fragment.x,"baseline":fragment.baseline,"top":fragment.top,"height":fragment.height,"width":fragment.width,"advance":fragment.advance,"size_points":fragment.style.measurement.size_points,"missing_codepoints":fragment.missing_codepoints.iter().map(|character|*character as u32).collect::<Vec<_>>()})).collect::<Vec<_>>()})).collect::<Vec<_>>()}));
            }
            cases.push(json!({"id":format!("{}-{width}",case["id"].as_str().unwrap()),"paragraphs":paragraphs}));
        }
    }
    assert_eq!(cases.len(), 273);
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
