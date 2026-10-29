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
        "#blockquote[Quote]",
        "#paragraph(custom=true)[Text]",
        "#span(custom=true)[Text]",
        "#font(name=\"Body\", files={regular=\"font.ttf\"})",
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

#[test]
fn inline_formatting_matches_reference() {
    support::compare_fixture("inline-formatting");
}

#[test]
fn direct_attributes_without_a_word_mapping_fail_explicitly() {
    for source in [
        "#strong(size=pt(12))[Text]",
        "#emphasis(font=\"Arial\")[Text]",
        "#code(color=\"#123\")[Text]",
    ] {
        assert!(
            mote::docx::emit(&mote::compile(source).unwrap()).is_err(),
            "{source}"
        );
    }
}

#[test]
fn named_styles_and_font_mappings_match_reference() {
    support::compare_fixture("styles-and-fonts");
}

#[test]
fn unresolved_styles_cycles_and_unsupported_declarations_fail_explicitly() {
    for source in [
        "#paragraph(style=\"Missing\")[Text]",
        "#style(name=\"A\",based_on=\"B\")#style(name=\"B\",based_on=\"A\")",
        "#style(name=\"A\",based_on=\"Missing\")",
        "#style(name=\"A\",next=\"Missing\")",
        "#style(name=\"Table\",type=\"table\")",
        "#font(name=\"Body\",files={regular=\"font.ttf\"})",
    ] {
        assert!(
            mote::docx::emit(&mote::compile(source).unwrap()).is_err(),
            "{source}"
        );
    }
}

#[test]
fn declared_property_order_and_style_identity_match_reference() {
    support::compare_fixture("style-properties");
}

#[test]
fn builtin_style_dependencies_are_emitted_and_references_are_resolved() {
    let parts = support::package(
        r##"#style(name="Snippet",type="character",based_on="Code",color="#123")
#span(style="Snippet")[Text]"##,
    );
    let styles = roxmltree::Document::parse(&parts["word/styles.xml"]).unwrap();
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let identifiers: std::collections::BTreeSet<_> = styles
        .descendants()
        .filter(|node| node.has_tag_name((namespace, "style")))
        .map(|node| node.attribute((namespace, "styleId")).unwrap())
        .collect();
    assert!(identifiers.contains("Code"));
    for node in styles.descendants().filter(|node| {
        ["basedOn", "next", "link"]
            .iter()
            .any(|name| node.has_tag_name((namespace, *name)))
    }) {
        assert!(identifiers.contains(node.attribute((namespace, "val")).unwrap()));
    }
}

#[test]
fn empty_named_font_does_not_erase_an_inherited_direct_typeface() {
    let parts = support::package(
        r#"#style(name="Empty Font",type="character",font="")
#span(font="Arial")[#span(style="Empty Font")[Text]]"#,
    );
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let fonts = document
        .descendants()
        .find(|node| node.has_tag_name((namespace, "rFonts")))
        .unwrap();
    assert_eq!(fonts.attribute((namespace, "ascii")), Some("Arial"));
}

#[test]
fn headings_match_reference() {
    support::compare_fixture("headings");
    support::compare_fixture("heading-styles");
}

#[test]
fn heading_flow_start_preserves_explicit_spacing_only() {
    let namespace = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    for (attributes, expected) in [
        ("", "0"),
        ("spacing_before=pt(7)", "140"),
        ("spacing={before=pt(3)}", "60"),
        ("spacing={spacing_before=pt(4)}", "80"),
        ("spacing_before=pt(7),spacing={before=pt(3)}", "60"),
        ("style=\"Title\"", "0"),
    ] {
        let source = format!(
            "#style(name=\"Title\",paragraph={{spacing_before=pt(12)}})#heading({attributes})[Title]"
        );
        let parts = support::package(&source);
        let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        let spacing = document
            .descendants()
            .find(|node| node.has_tag_name((namespace, "spacing")))
            .unwrap();
        assert_eq!(
            spacing.attribute((namespace, "before")),
            Some(expected),
            "{source}"
        );
    }
    let parts = support::package("#paragraph[]\n\n#heading[Title]");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    assert!(
        !document
            .descendants()
            .any(|node| node.has_tag_name((namespace, "spacing")))
    );
}

#[test]
fn unsupported_heading_styles_and_numbering_fail_explicitly() {
    for source in [
        "#heading(level=7)[Missing default]",
        "#heading(style=\"Missing\")[Missing style]",
        "#heading(numbering={enabled=true})[Numbered]",
        "#heading(marker=\"custom\")[Marker]",
        "#heading(level=1, custom=true)[Unknown]",
    ] {
        assert!(
            mote::docx::emit(&mote::compile(source).unwrap()).is_err(),
            "{source}"
        );
    }
}
