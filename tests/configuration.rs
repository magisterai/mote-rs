use mote::{compile, json::to_json, model::Value};
use serde_json::json;

#[test]
fn geometry_merges_but_authored_arguments_replace_whole_groups() {
    let document = compile(
        r#"#document(page = { size = "a4", width = "6in" }, margin = { left = 20 })
#paragraph[Before]
#document(page = { height = "9in" }, margin = { top = 0 })"#,
    )
    .unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    assert_eq!(output["document"]["page"], json!({"height":"9in"}));
    assert_eq!(output["page"]["properties"]["width"]["text"], "6in");
    assert_eq!(output["page"]["properties"]["size"], "a4");
    assert_eq!(output["margin"]["left"], 20.0);
    assert_eq!(output["margin"]["top"], 0.0);
    assert_eq!(output["sections"][0]["page"], output["page"]);
    assert_eq!(output["sections"][0]["margin"], output["margin"]);
    assert_eq!(document.content.len(), 1);
}

#[test]
fn declarations_produce_no_content_and_apply_inside_evaluated_bodies() {
    let empty = compile("#document()\n#document(page = { size = \"a5\" })").unwrap();
    assert!(empty.content.is_empty());
    let output: serde_json::Value = serde_json::from_str(&to_json(&empty).unwrap()).unwrap();
    assert!(output.get("sections").is_none());
    let nested = compile("#paragraph[Text#document(margin = { top = 0 })]").unwrap();
    assert_eq!(nested.margin["top"], Value::Number(0.0));
    assert_eq!(nested.content.len(), 1);
}

#[test]
fn extra_metadata_is_retained_without_inventing_geometry() {
    let document = compile(r#"#document(note = "draft", default_size = 12, language = "en-GB", page = { custom = true }, margin = { custom = "2pt" }, properties = { page = { size = "a3" } })"#).unwrap();
    assert_eq!(document.defaults["default_size"], Value::Number(12.0));
    assert_eq!(document.defaults["language"], Value::String("en-GB".into()));
    assert!(!document.page.contains_key("size"));
    assert!(matches!(document.margin["custom"], Value::Length { .. }));
    assert!(mote::docx::emit(&document).is_err());
}

#[test]
fn invalid_configuration_reports_source_located_diagnostics() {
    for source in [
        "#document(1)",
        "#document()[Body]",
        "#document(page = 1)",
        "#document(margin = false)",
        "#document(page = {})",
        "#document(page = { width = -1 })",
        "#document(page = { height = true })",
        "#document(page = { orientation = \"Landscape\" })",
        "#document(margin = { top = \"-1pt\" })",
        "#document(margin = { unknown = true })",
        "#document(page_size = \"a4\")",
        "#document(orientation = \"portrait\")",
        "#document(page_width = 10)",
        "#document(margin_top = 10)",
        "#document(column_gap = 10)",
        "#document(columns = { count = 2 })",
    ] {
        let error = compile(source).expect_err(source);
        assert!(error.span.end > error.span.start, "{source}: {error}");
        assert!(error.span.end <= source.len(), "{source}: {error}");
    }
}
