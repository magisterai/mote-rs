#[path = "docx/support.rs"]
mod support;

const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[test]
fn bullet_layout_and_instances_match_reference() {
    support::compare_fixture("bullet-lists");
    support::compare_fixture("bullet-instances");
}

#[test]
fn numbering_references_resolve_to_nine_complete_levels() {
    let parts = support::compare_fixture("bullet-instances");
    let definitions = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
    let instances: std::collections::BTreeMap<_, _> = definitions
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "num")))
        .map(|node| {
            (
                node.attribute((WORD_NAMESPACE, "numId")).unwrap(),
                node.children()
                    .find(|child| child.has_tag_name((WORD_NAMESPACE, "abstractNumId")))
                    .unwrap()
                    .attribute((WORD_NAMESPACE, "val"))
                    .unwrap(),
            )
        })
        .collect();
    let abstract_definitions: std::collections::BTreeSet<_> = definitions
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "abstractNum")))
        .map(|node| {
            let levels: Vec<_> = node
                .children()
                .filter(|child| child.has_tag_name((WORD_NAMESPACE, "lvl")))
                .collect();
            assert_eq!(levels.len(), 9);
            for (index, level) in levels.iter().enumerate() {
                assert_eq!(
                    level.attribute((WORD_NAMESPACE, "ilvl")).unwrap(),
                    index.to_string()
                );
                assert_eq!(
                    level
                        .descendants()
                        .find(|child| child.has_tag_name((WORD_NAMESPACE, "numFmt")))
                        .unwrap()
                        .attribute((WORD_NAMESPACE, "val")),
                    Some("bullet")
                );
            }
            node.attribute((WORD_NAMESPACE, "abstractNumId")).unwrap()
        })
        .collect();
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    for reference in document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "numId")))
    {
        let identifier = reference.attribute((WORD_NAMESPACE, "val")).unwrap();
        assert!(abstract_definitions.contains(instances[identifier]));
    }
}

#[test]
fn empty_lists_create_no_numbering_instances_but_empty_items_do() {
    for (source, expected_paragraphs, expected_instances) in [
        ("#list[]", 0, 0),
        ("#list[#item[]]", 1, 1),
        ("#list[]\n#list[#item[One]]\n#list[]", 1, 1),
    ] {
        let parts = support::package(source);
        let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        let numbering = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
        assert_eq!(
            document
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "p")))
                .count(),
            expected_paragraphs
        );
        assert_eq!(
            numbering
                .descendants()
                .filter(|node| node.has_tag_name((WORD_NAMESPACE, "num")))
                .count(),
            expected_instances
        );
    }
}

#[test]
fn list_limits_and_unsupported_properties_are_explicit() {
    for source in [
        "#list(level=10)[#item[Too deep]]",
        "#list[#item(level=10)[Too deep]]",
        "#list(indent={level=twip(2147483647)})[#item[Overflow]]",
        "#list(indent={left=pt(4), custom=pt(3)})[#item[Unknown]]",
        "#list(spacing={before=pt(4)})[#item[Unsupported]]",
        "#list(style=\"Normal\")[#item[Unsupported]]",
        "#list[#item(checkbox={custom=true})[Unsupported task property]]",
        "#list[#item[#paragraph(alignment=\"right\")[Deferred paragraph formatting]]]",
        "#list[#item[#heading[Deferred child block]]]",
    ] {
        let document = mote::compile(source).unwrap();
        let original = document.clone();
        assert!(mote::docx::emit(&document).is_err(), "{source}");
        assert_eq!(document, original);
    }
}

#[test]
fn ordered_lists_and_sequence_continuation_match_reference() {
    support::compare_fixture("ordered-lists");
    support::compare_fixture("list-continuation");
}

#[test]
fn restarts_replace_the_continuation_instance_and_preserve_starts() {
    let parts = support::compare_fixture("list-continuation");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let identifiers: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "numId")))
        .map(|node| node.attribute((WORD_NAMESPACE, "val")).unwrap())
        .collect();
    assert_eq!(
        identifiers,
        ["1", "1", "2", "2", "3", "4", "4", "5", "5", "6", "7", "2"]
    );
    let numbering = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
    let starts: Vec<_> = numbering
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "startOverride")))
        .map(|node| node.attribute((WORD_NAMESPACE, "val")).unwrap())
        .collect();
    assert_eq!(starts, ["3", "7", "1"]);
}

#[test]
fn source_list_markers_set_only_the_initial_counter() {
    for (marker, expected) in [
        ("9.", "9"),
        (" 003. ", "3"),
        ("4. [x]", "4"),
        ("0.", "1"),
        ("17x", "1"),
        ("999999999999999999999999.", "1"),
    ] {
        let source = format!(
            "#ordered_list[#item(marker=\"{marker}\")[First]#item(marker=\"99.\")[Second]]"
        );
        let parts = support::package(&source);
        let numbering = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
        let starts: Vec<_> = numbering
            .descendants()
            .filter(|node| node.has_tag_name((WORD_NAMESPACE, "startOverride")))
            .map(|node| node.attribute((WORD_NAMESPACE, "val")).unwrap())
            .collect();
        assert_eq!(starts, [expected], "{marker}");
    }
}

#[test]
fn ordered_lists_reject_unrepresentable_starts_without_mutating_the_model() {
    let document = mote::compile("#ordered_list(start=2147483648)[#item[Text]]").unwrap();
    let original = document.clone();
    assert!(mote::docx::emit(&document).is_err());
    assert_eq!(document, original);
}

#[test]
fn heading_numbering_matches_reference() {
    support::compare_fixture("heading-numbering");
}

#[test]
fn heading_numbering_keeps_sequences_separate_and_disabled_headings_unnumbered() {
    let parts = support::compare_fixture("heading-numbering");
    let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
    let identifiers: Vec<_> = document
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "numId")))
        .map(|node| node.attribute((WORD_NAMESPACE, "val")).unwrap())
        .collect();
    assert_eq!(
        identifiers,
        ["1", "1", "1", "2", "2", "3", "4", "5", "6", "7", "8", "2"]
    );
    let disabled = document
        .descendants()
        .find(|node| {
            node.has_tag_name((WORD_NAMESPACE, "p"))
                && node
                    .descendants()
                    .any(|child| child.text() == Some("Unnumbered heading"))
        })
        .unwrap();
    assert!(
        !disabled
            .descendants()
            .any(|node| node.has_tag_name((WORD_NAMESPACE, "numPr")))
    );
    let numbering = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
    let starts: Vec<_> = numbering
        .descendants()
        .filter(|node| node.has_tag_name((WORD_NAMESPACE, "startOverride")))
        .map(|node| node.attribute((WORD_NAMESPACE, "val")).unwrap())
        .collect();
    assert_eq!(starts, ["3", "7", "2"]);
    assert!(
        numbering
            .descendants()
            .any(|node| node.has_tag_name((WORD_NAMESPACE, "lvlText"))
                && node.attribute((WORD_NAMESPACE, "val")) == Some("%1 & %2 & %3 & "))
    );
}

#[test]
fn a_disabled_heading_does_not_allocate_numbering() {
    let parts = support::package("#heading(numbering={enabled=false,start=9,restart=true})[Title]");
    let numbering = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
    assert_eq!(
        numbering
            .root_element()
            .children()
            .filter(|node| node.is_element())
            .count(),
        0
    );
}
