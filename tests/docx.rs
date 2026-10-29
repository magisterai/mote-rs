#[path = "docx/support.rs"]
mod support;

#[test]
fn plain_paragraph_package_matches_reference() {
    let parts = support::compare_fixture("plain-paragraph");
    let metadata = roxmltree::Document::parse(&parts["docProps/core.xml"]).unwrap();
    let timestamp = |name| {
        metadata
            .descendants()
            .find(|node| node.has_tag_name(("http://purl.org/dc/terms/", name)))
            .unwrap()
            .text()
            .unwrap()
    };
    assert_eq!(timestamp("created"), timestamp("modified"));
    assert_eq!(timestamp("created").len(), 20);
    assert!(timestamp("created").ends_with('Z'));
}

#[test]
fn paragraphs_generate_content_independently_of_the_reference() {
    let parts = support::package("First paragraph.\n\nSecond & <third>.");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let paragraphs: Vec<_> = document
        .descendants()
        .filter(|node| {
            node.has_tag_name((
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "p",
            ))
        })
        .collect();
    assert_eq!(paragraphs.len(), 2);
    let text: Vec<_> = document
        .descendants()
        .filter(|node| {
            node.has_tag_name((
                "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                "t",
            ))
        })
        .map(|node| node.text().unwrap())
        .collect();
    assert_eq!(text, ["First paragraph.", "Second & <third>."]);
}

#[test]
fn unsupported_content_and_attributes_fail_explicitly() {
    for source in [
        "#heading[Title]",
        "#list[#item[Item]]",
        "#paragraph(custom=true)[Text]",
        "#span[Text]",
        "#font(name=\"Body\", latin=\"Arial\")",
    ] {
        let error = mote::docx::emit(&mote::compile(source).unwrap()).unwrap_err();
        assert!(
            error.to_string().starts_with("DOCX does not yet support"),
            "{source}: {error}"
        );
    }
}
