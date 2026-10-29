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
fn unsupported_nested_quote_formatting_is_explicit() {
    for source in [
        "#blockquote[#paragraph(alignment=\"right\")[Nested paragraph formatting]]",
        "#blockquote[#blockquote(fill=\"#abc\")[Nested quote formatting]]",
        "#blockquote[#heading[Heading]]",
        "#blockquote[#list[#item(checked=true)[Task]]]",
        "#blockquote(custom=true)[Unknown]",
    ] {
        let document = mote::compile(source).unwrap();
        assert!(mote::docx::emit(&document).is_err(), "{source}");
    }
}
