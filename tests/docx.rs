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
    assert_eq!(text, ["First paragraph.", "Second & ", "<", "third>."]);
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

#[test]
fn text_breaks_and_tabs_match_reference() {
    support::compare_fixture("text-fidelity");
}

#[test]
fn illegal_xml_characters_fail_without_mutating_the_model() {
    let document = mote::compile("text\u{1}more").unwrap();
    let original = document.clone();
    let error = mote::docx::emit(&document).unwrap_err();
    assert!(error.to_string().contains("U+0001"));
    assert_eq!(document, original);
}

#[test]
fn paragraph_formatting_matches_reference() {
    support::compare_fixture("paragraph-formatting");
}

#[test]
fn unimplemented_nested_properties_and_invalid_output_ranges_are_explicit() {
    for source in [
        "#paragraph(indent={start=pt(4)})[Text]",
        "#paragraph(outline_level=10)[Text]",
        "#paragraph(spacing_after=pt(1000000000000))[Text]",
    ] {
        assert!(
            mote::docx::emit(&mote::compile(source).unwrap()).is_err(),
            "{source}"
        );
    }
}
