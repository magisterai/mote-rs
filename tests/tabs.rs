#[path = "docx/support.rs"]
mod support;

use mote::{compile, json::to_json};

#[test]
fn inline_tabs_match_the_captured_model_and_package() {
    support::compare_fixture("inline-tabs");
}

#[test]
fn tabs_are_inline_nodes_and_metadata_uses_shared_merging() {
    let document = compile(
        r#"#paragraph[Left#tab(note = "outer", properties = { note = "inner", extra = 2 })Right]"#,
    )
    .unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    let tab = &output["content"][0]["children"][1];
    assert_eq!(tab["kind"], "tab");
    assert_eq!(tab["category"], "inline");
    assert_eq!(tab["attributes"]["note"], "inner");
    assert_eq!(tab["attributes"]["extra"], 2.0);
    assert!(tab.get("children").is_none());
    assert!(mote::docx::emit(&document).is_err());
}

#[test]
fn tab_body_positionals_and_nonobject_properties_are_invalid() {
    for source in ["#tab(1)", "#tab()[text]", "#tab(properties = false)"] {
        assert!(compile(source).is_err(), "{source}");
    }
}

#[test]
fn ordinary_edge_tabs_trim_while_preserved_tabs_survive() {
    for (source, expected_tabs) in [
        ("#paragraph[#tab()  text  #tab()]", 0),
        ("#paragraph[before#tab()#tab()after]", 2),
        ("#paragraph[before#line_break()#tab()after]", 0),
        ("#paragraph[before#tab()#line_break()after]", 0),
        ("#paragraph[#code(preserve_space = true)[#tab()]  text]", 1),
        ("#paragraph[text  #code(preserve_space = true)[#tab()]]", 1),
    ] {
        let package = support::package(source);
        assert_eq!(
            package["word/document.xml"].matches("<w:tab/>").count(),
            expected_tabs,
            "{source}"
        );
    }
}

#[test]
fn edge_trimming_does_not_hide_unsupported_tab_metadata() {
    for source in [
        "#paragraph[#tab(note = 1)text]",
        "#list[#item[text#tab(note = 1)]]",
    ] {
        let document = compile(source).unwrap();
        assert!(
            mote::docx::emit(&document)
                .unwrap_err()
                .to_string()
                .contains("tab attribute")
        );
    }
}
