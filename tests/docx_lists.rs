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
        "#list[#item(checked=false)[Deferred task]]",
        "#list[#item[#paragraph(alignment=\"right\")[Deferred paragraph formatting]]]",
        "#list[#item[#heading[Deferred child block]]]",
    ] {
        let document = mote::compile(source).unwrap();
        let original = document.clone();
        assert!(mote::docx::emit(&document).is_err(), "{source}");
        assert_eq!(document, original);
    }
}
