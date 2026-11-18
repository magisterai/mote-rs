#[path = "docx/support.rs"]
mod support;

const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[test]
fn block_quote_content_and_properties_match_reference() {
    support::compare_fixture("block-quotes");
    support::compare_fixture("quote-styles");
}

#[test]
fn quote_formatting_does_not_leak_into_following_paragraphs() {
    let parts =
        support::package("#blockquote(fill=\"#abc\",border={left=\"single\"})[Quote]\n\nOutside.");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let paragraphs: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "p")))
        .collect();
    assert_eq!(paragraphs.len(), 2);
    assert!(
        paragraphs[0]
            .descendants()
            .any(|node| node.has_tag_name((WORD_NAMESPACE, "pStyle")))
    );
    assert!(
        !paragraphs[1]
            .descendants()
            .any(|node| node.has_tag_name((WORD_NAMESPACE, "pPr")))
    );
}

#[test]
fn quote_lists_supply_text_without_allocating_numbering() {
    let parts = support::package("#blockquote[#ordered_list(start=3)[#item[First]#item[Second]]]");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let text: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "t")))
        .map(|node| node.text().unwrap())
        .collect();
    assert_eq!(text, ["First", "Second"]);
    let numbering = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
    assert!(
        !numbering
            .descendants()
            .any(|node| node.has_tag_name((WORD_NAMESPACE, "num")))
    );
}

#[test]
fn quote_names_without_a_word_mapping_warn() {
    for (source, name, without) in [
        (
            "#blockquote[#list[#item(checked=true)[Task]]]",
            "item.checked",
            "#blockquote[#list[#item[Task]]]",
        ),
        (
            "#blockquote(custom=true)[Unknown]",
            "blockquote.custom",
            "#blockquote[Unknown]",
        ),
    ] {
        let actual = support::assert_ignored(source, name);
        let expected = support::package(without);
        support::compare_xml(
            &actual["word/document.xml"],
            &expected["word/document.xml"],
            "word/document.xml",
        );
    }
}
