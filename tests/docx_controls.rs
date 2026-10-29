#[path = "docx/support.rs"]
mod support;

const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const WORD_2010_NAMESPACE: &str = "http://schemas.microsoft.com/office/word/2010/wordml";

#[test]
fn task_items_match_reference() {
    support::compare_fixture("task-items");
}

#[test]
fn modern_control_state_matches_fallback_text_and_has_unique_identifiers() {
    let parts = support::compare_fixture("task-items");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let mut identifiers = std::collections::BTreeSet::new();
    for control in document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "sdt")))
    {
        let identifier = control
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "id")))
            .unwrap()
            .attribute((WORD_NAMESPACE, "val"))
            .unwrap();
        assert!(identifiers.insert(identifier));
        let checked = control
            .descendants()
            .find(|node| node.has_tag_name((WORD_2010_NAMESPACE, "checked")))
            .unwrap()
            .attribute((WORD_2010_NAMESPACE, "val"))
            .unwrap();
        let fallback = control
            .descendants()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "t")))
            .unwrap()
            .text()
            .unwrap();
        assert_eq!(fallback, if checked == "1" { "☒" } else { "☐" });
        let paragraph = control
            .ancestors()
            .find(|node| node.has_tag_name((WORD_NAMESPACE, "p")))
            .unwrap();
        assert!(
            !paragraph
                .descendants()
                .any(|node| node.has_tag_name((WORD_NAMESPACE, "numPr")))
        );
    }
    assert_eq!(identifiers.len(), 7);
    let legacy_name = document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "ffData")))
        .flat_map(|node| node.children())
        .find(|node| {
            node.has_tag_name((WORD_NAMESPACE, "name"))
                && node.attribute((WORD_NAMESPACE, "val")) == Some("Check1003")
        });
    assert!(legacy_name.is_some());
    assert!(
        !document
            .descendants()
            .any(|node| node.attribute((WORD_NAMESPACE, "dirty")).is_some())
    );
}

#[test]
fn task_state_presence_is_distinct_from_nested_checkbox_metadata() {
    for (attributes, expected_controls, expected_numbering) in [
        ("checked=false,checkbox={checked=true}", 1, 0),
        ("checked=true,checkbox={checked=false}", 1, 0),
        ("checkbox={checked=true}", 0, 1),
    ] {
        let source = format!("#list[#item({attributes})[Task]]");
        let parts = support::package(&source);
        let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        assert_eq!(
            document
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "sdt")))
                .count(),
            expected_controls
        );
        assert_eq!(
            document
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "numPr")))
                .count(),
            expected_numbering
        );
    }
}

#[test]
fn unsupported_control_metadata_fails_explicitly() {
    for source in [
        "#list[#item(checked=true,checkbox={kind=\"legacy_form_field\",title=\"Title\"})[Text]]",
        "#list[#item(checked=true,checkbox={custom=true})[Text]]",
        "#list[#item(checked=true,checkbox={title=\"Invalid\u{1}\"})[Text]]",
    ] {
        let document = mote::compile(source).unwrap();
        assert!(mote::docx::emit(&document).is_err(), "{source}");
    }
}
