#[path = "support/json_compare.rs"]
mod json_compare;
use json_compare::compare;
use mote::{
    layout::{LayoutStyleResolver, LineSpacing, TextStyle, resolve_inline},
    measurement::FontVariant,
    model::Content,
};
use serde_json::{Value, json};
use std::{fs, path::Path};
fn text_style(style: &TextStyle) -> Value {
    let (bold, italic) = match style.measurement.variant {
        FontVariant::Regular => (false, false),
        FontVariant::Bold => (true, false),
        FontVariant::Italic => (false, true),
        FontVariant::BoldItalic => (true, true),
    };
    json!({"font":style.measurement.font,"size":style.measurement.size_points,"word_line_height":style.word_line_height.unwrap_or(0.0),"bold":bold,"italic":italic,"underline":style.underline,"strike":style.strike,"double_strike":style.double_strike,"small_caps":style.small_caps,"code":style.code,"color":style.color,"fill":style.highlight.clone().unwrap_or_default()})
}
#[test]
fn native_styles_match_captured_source_cases() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/layout-styles");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut cases = Vec::new();
    for case in requests["cases"].as_array().unwrap() {
        let document = mote::compile(case["source"].as_str().unwrap()).unwrap();
        let resolver = LayoutStyleResolver::new(&document);
        let mut paragraphs = Vec::new();
        for node in &document.content {
            let Content::Element(element) = node else {
                continue;
            };
            let style = resolver.paragraph(element);
            let runs = resolve_inline(&element.children, &style.text, &resolver);
            let (rule, value) = match style.line_spacing {
                LineSpacing::Multiple(value) => ("multiple", value),
                LineSpacing::Exact(value) => ("exact", value),
                LineSpacing::AtLeast(value) => ("at_least", value),
            };
            paragraphs.push(json!({"text":text_style(&style.text),"alignment":style.alignment,"spacing_before":style.spacing_before,"spacing_after":style.spacing_after,"keep_with_next":style.keep_with_next,"keep_together":style.keep_together,"page_break_before":style.page_break_before,"widow_control":style.widow_control,"line_height":style.line_height,"line_rule":rule,"line_value":value,"line_multiplier":style.line_multiplier,"indent_left":style.indent_left,"indent_right":style.indent_right,"first_line":style.first_line,"fill":style.fill.unwrap_or_default(),"runs":runs.iter().map(|run|json!({"text":run.text,"style":text_style(&run.style),"preserve_space":run.preserve_space})).collect::<Vec<_>>()}));
        }
        cases.push(json!({"id":case["id"],"paragraphs":paragraphs}));
    }
    let actual = json!({"schema_version":1,"cases":cases});
    assert_eq!(cases.len(), 91);
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
