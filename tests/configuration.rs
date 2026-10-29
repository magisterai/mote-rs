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
        "#document(columns = { count = 0 })",
    ] {
        let error = compile(source).expect_err(source);
        assert!(error.span.end > error.span.start, "{source}: {error}");
        assert!(error.span.end <= source.len(), "{source}: {error}");
    }
}

#[test]
fn column_fields_merge_and_preserve_authored_count_and_explicit_false() {
    let document = compile(
        r#"#document(columns = { count = "3", gap = "12pt", separator = true, equal_width = true })
#document(columns = { separator = "false", equal_width = false })
#paragraph[Text]"#,
    )
    .unwrap();
    assert_eq!(document.columns.count, Some(3));
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    assert_eq!(output["columns"]["count"], 3);
    assert_eq!(output["columns"]["properties"]["count"], "3");
    assert_eq!(output["columns"]["gap"]["text"], "12pt");
    assert_eq!(output["columns"]["equal_width"], false);
    assert_eq!(output["columns"]["properties"]["separator"], false);
    assert!(output["columns"].get("separator").is_none());
    assert_eq!(output["sections"][0]["columns"], output["columns"]);
    assert!(
        mote::docx::emit(&document)
            .unwrap_err()
            .to_string()
            .contains("unequal-width")
    );
}

#[test]
fn invalid_columns_are_rejected_before_model_construction() {
    for properties in [
        "2",
        "{}",
        "{ count = 0 }",
        "{ count = -1 }",
        "{ count = 1.5 }",
        "{ count = true }",
        "{ count = \"2.5\" }",
        "{ count = \"2.0\" }",
        "{ gap = -1 }",
        "{ separator = 1 }",
        "{ separator = \"yes\" }",
        "{ equal_width = 0 }",
        "{ equal_width = \"yes\" }",
    ] {
        let source = format!("#document(columns = {properties})");
        assert!(compile(&source).is_err(), "{source}");
    }
}

#[test]
fn integer_column_strings_remain_exact_before_output_range_validation() {
    for count in [9_007_199_254_740_993_i64, i64::MAX] {
        let document = compile(&format!("#document(columns = {{ count = \"{count}\" }})")).unwrap();
        assert_eq!(document.columns.count, Some(count));
        let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
        assert_eq!(output["columns"]["count"].as_i64(), Some(count));
        assert_eq!(output["columns"]["properties"]["count"], count.to_string());
        assert!(mote::docx::emit(&document).is_err());
    }
    for count in [
        "\"9223372036854775808\"",
        "9223372036854775808",
        "\"-1\"",
        "\"0\"",
    ] {
        assert!(compile(&format!("#document(columns = {{ count = {count} }})")).is_err());
    }
}
