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
    if json_compare::refresh(&root, &actual) {
        return;
    }
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
fn flat_first_line_and_hanging_indents_of_a_style_move_the_first_line() {
    let source = "#style(name = \"Lead\", type = \"paragraph\", first_line_indent = pt(40))\n#style(name = \"Hang\", type = \"paragraph\", paragraph = { indent = { left = pt(40) } }, hanging_indent = pt(40))\n\n#paragraph \"Lead\"[The first line is indented, and the paragraph runs on for long enough to reach a second line of the text area.]\n\n#paragraph \"Hang\"[The first line hangs at the margin, and the paragraph runs on for long enough to reach a second line of the text area.]\n";
    let document = mote::compile(source).unwrap();
    let resolver = mote::source::FileAssetResolver::new("tests/fixtures/images/assets").unwrap();
    let planned = mote::render::plan(&document, &resolver, None).unwrap();
    assert_eq!(planned.warnings, Vec::<String>::new());
    let lefts: Vec<f64> = planned.output.pages[0]
        .lines
        .iter()
        .map(|line| line.line.x)
        .collect();
    assert_eq!(lefts, [112.0, 72.0, 72.0, 112.0]);
}
