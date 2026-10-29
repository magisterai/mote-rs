#[path = "support/json_compare.rs"]
mod json_compare;
use mote::{layout::plan_document_text, measurement::TextMeasurer, source::FileAssetResolver};
use serde_json::{Value, json};
use std::{fs, path::Path};
#[test]
fn complete_sources_match_captured_page_text_geometry() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/document-text");
    let requests: Value =
        serde_json::from_slice(&fs::read(root.join("requests.json")).unwrap()).unwrap();
    let expected: Value =
        serde_json::from_slice(&fs::read(root.join("expected.json")).unwrap()).unwrap();
    let mut cases = Vec::new();
    let assets = FileAssetResolver::from_current_directory().unwrap();
    for case in requests["cases"].as_array().unwrap() {
        let document = mote::compile(case["source"].as_str().unwrap()).unwrap();
        let mut measurer = TextMeasurer::new(&document, &assets, None);
        let result = plan_document_text(&document, &mut measurer);
        let plan = match result {
            Ok(plan) => plan,
            Err(mote::layout::LayoutError::InvalidGeometry(message))
                if message.contains("line height must be positive") =>
            {
                cases.push(json!({"id":case["id"],"invalid_line_height":true}));
                continue;
            }
            Err(error) => panic!("{}: {error}", case["id"]),
        };
        cases.push(json!({"id":case["id"],"pages":plan.pages.iter().map(|page|json!({"lines":page.lines.iter().map(|placed|{let line=&placed.line;json!({"x":line.x,"top":placed.top,"width":line.width,"height":line.height,"baseline":line.baseline,"fragments":line.fragments.iter().map(|fragment|json!({"text":fragment.text,"x":fragment.x,"baseline":fragment.baseline,"top":fragment.top,"width":fragment.width,"height":fragment.height,"advance":fragment.advance,"font_size":fragment.style.measurement.size_points,"font":fragment.font.family,"variant":fragment.font.variant.as_text(),"color":color(&fragment.style.color),"missing_codepoints":fragment.missing_codepoints.iter().map(|character|*character as u32).collect::<Vec<_>>()})).collect::<Vec<_>>()})}).collect::<Vec<_>>()})).collect::<Vec<_>>()}));
    }
    let actual = json!({"schema_version":1,"cases":cases});
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
fn color(text: &str) -> Value {
    let value = u32::from_str_radix(text, 16).unwrap();
    json!({"r":(value>>16)&255,"g":(value>>8)&255,"b":value&255,"a":255})
}

#[test]
fn document_text_scope_rejects_unplanned_structure() {
    let assets = FileAssetResolver::from_current_directory().unwrap();
    for source in [
        "#list[#item[One]]",
        "#blockquote[Quote]",
        "#document(columns={count=2})\nText",
        "#heading(numbering={enabled=true})[Numbered]",
    ] {
        let document = mote::compile(source).unwrap();
        let mut measurer = TextMeasurer::new(&document, &assets, None);
        assert!(
            plan_document_text(&document, &mut measurer).is_err(),
            "{source}"
        );
    }
}
