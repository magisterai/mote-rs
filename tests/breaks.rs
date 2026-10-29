#[path = "docx/support.rs"]
mod support;

use mote::{compile, json::to_json};
const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[test]
fn block_breaks_match_captured_models_and_packages() {
    for name in ["block-breaks", "column-breaks", "consecutive-breaks"] {
        support::compare_fixture(name);
    }
}

#[test]
fn break_defaults_and_property_precedence_are_visible_in_the_model() {
    let document = compile(
        r#"#break()#break(type = "page", properties = { type = "column", note = "kept" })"#,
    )
    .unwrap();
    let output: serde_json::Value = serde_json::from_str(&to_json(&document).unwrap()).unwrap();
    assert_eq!(output["content"][0]["kind"], "break");
    assert_eq!(output["content"][0]["category"], "block");
    assert_eq!(output["content"][0]["attributes"]["type"], "page");
    assert_eq!(output["content"][1]["attributes"]["type"], "column");
    assert_eq!(output["content"][1]["attributes"]["note"], "kept");
    assert!(mote::docx::emit(&document).is_err());
}

#[test]
fn invalid_break_calls_and_inline_placement_are_rejected() {
    for source in [
        "#break(1)",
        "#break()[text]",
        "#break(type = false)",
        "#break(type = false, properties = { type = \"page\" })",
        "#break(type = \"unknown\")",
        "#break(type = \"section\")",
        "#break(properties = { type = 1 })",
        "#paragraph[Text#break()]",
        "#span[Text#break(type = \"line\")]",
        "#list[#break()]",
    ] {
        assert!(compile(source).is_err(), "{source}");
    }
}

#[test]
fn pending_column_breaks_survive_empty_consecutive_and_terminal_positions() {
    for (source, paragraphs, breaks) in [
        ("#break(type = \"column\")", 1, 1),
        ("#break(type = \"column\")#paragraph[]", 1, 1),
        ("#break(type = \"column\")#break(type = \"column\")", 2, 2),
        ("#break(type = \"column\")#break()#paragraph[Text]", 3, 2),
        ("#paragraph[Text]#break(type = \"column\")", 2, 1),
    ] {
        let package = support::package(source);
        let document = roxmltree::Document::parse(&package["word/document.xml"]).unwrap();
        assert_eq!(
            document
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "p")))
                .count(),
            paragraphs,
            "{source}"
        );
        assert_eq!(
            document
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "br")))
                .count(),
            breaks,
            "{source}"
        );
    }
}

#[test]
fn column_start_suppresses_only_implicit_heading_spacing() {
    for (attributes, expected) in [
        ("1", Some("0")),
        ("1, spacing_before = pt(8)", Some("160")),
        ("1, spacing = { before = pt(9) }", Some("180")),
    ] {
        let package = support::package(&format!(
            "#paragraph[Before]#break(type = \"column\")#heading({attributes})[Heading]"
        ));
        let document = roxmltree::Document::parse(&package["word/document.xml"]).unwrap();
        let heading = document
            .descendants()
            .filter(|node| node.has_tag_name((WORD_NAMESPACE, "p")))
            .nth(1)
            .unwrap();
        let spacing = heading
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "spacing")))
            .unwrap();
        assert_eq!(spacing.attribute((WORD_NAMESPACE, "before")), expected);
        assert_eq!(
            heading
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "br")))
                .count(),
            1
        );
    }
}

#[test]
fn nested_break_output_fails_instead_of_disappearing() {
    for source in [
        "#blockquote[Before#break()After]",
        "#list[#item[Before#break()After]]",
    ] {
        let document = compile(source).unwrap();
        assert!(mote::docx::emit(&document).is_err(), "{source}");
    }
}
