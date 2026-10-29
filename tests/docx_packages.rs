#[path = "docx/support.rs"]
mod support;

use std::collections::BTreeSet;

const WORD_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const RELATIONSHIP_NAMESPACE: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

#[test]
fn every_captured_package_has_resolved_styles_numbering_and_package_relationships() {
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/docx");
    let mut names: Vec<_> = std::fs::read_dir(fixtures)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    for name in names {
        let parts = support::compare_fixture(&name);
        for (part_name, contents) in &parts {
            let document = roxmltree::Document::parse(contents).unwrap();
            if part_name.ends_with(".rels") {
                let mut identifiers = BTreeSet::new();
                let prefix = if part_name.starts_with("word/") {
                    "word/"
                } else {
                    ""
                };
                for relationship in document
                    .descendants()
                    .filter(|node| node.has_tag_name((RELATIONSHIP_NAMESPACE, "Relationship")))
                {
                    let identifier = relationship.attribute("Id").unwrap();
                    assert!(
                        identifiers.insert(identifier),
                        "{name}: duplicate relationship {identifier}"
                    );
                    if relationship.attribute("TargetMode") == Some("External") {
                        assert_eq!(
                            relationship.attribute("Type"),
                            Some(
                                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
                            )
                        );
                        assert!(!relationship.attribute("Target").unwrap().is_empty());
                        continue;
                    }
                    let target = format!("{prefix}{}", relationship.attribute("Target").unwrap());
                    assert!(
                        parts.contains_key(&target),
                        "{name}: missing relationship target {target}"
                    );
                }
            }
            if part_name == "[Content_Types].xml" {
                for declaration in document
                    .descendants()
                    .filter(|node| node.tag_name().name() == "Override")
                {
                    let target = declaration
                        .attribute("PartName")
                        .unwrap()
                        .strip_prefix('/')
                        .unwrap();
                    assert!(
                        parts.contains_key(target),
                        "{name}: content type points to missing {target}"
                    );
                }
            }
        }
        let styles = roxmltree::Document::parse(&parts["word/styles.xml"]).unwrap();
        let mut style_identifiers = BTreeSet::new();
        for style in styles
            .descendants()
            .filter(|node| node.has_tag_name((WORD_NAMESPACE, "style")))
        {
            let identifier = style.attribute((WORD_NAMESPACE, "styleId")).unwrap();
            assert!(!identifier.is_empty());
            assert!(
                style_identifiers.insert(identifier),
                "{name}: duplicate style {identifier}"
            );
        }
        for relationship in styles.descendants().filter(|node| {
            ["basedOn", "next", "link"]
                .iter()
                .any(|tag| node.has_tag_name((WORD_NAMESPACE, *tag)))
        }) {
            let identifier = relationship.attribute((WORD_NAMESPACE, "val")).unwrap();
            assert!(
                style_identifiers.contains(identifier),
                "{name}: missing related style {identifier}"
            );
        }
        let numbering = roxmltree::Document::parse(&parts["word/numbering.xml"]).unwrap();
        let mut abstract_identifiers = BTreeSet::new();
        let mut numbering_identifiers = BTreeSet::new();
        for definition in numbering
            .descendants()
            .filter(|node| node.has_tag_name((WORD_NAMESPACE, "abstractNum")))
        {
            let identifier = definition
                .attribute((WORD_NAMESPACE, "abstractNumId"))
                .unwrap();
            assert!(
                abstract_identifiers.insert(identifier),
                "{name}: duplicate numbering definition {identifier}"
            );
        }
        for instance in numbering
            .descendants()
            .filter(|node| node.has_tag_name((WORD_NAMESPACE, "num")))
        {
            let identifier = instance.attribute((WORD_NAMESPACE, "numId")).unwrap();
            assert!(
                numbering_identifiers.insert(identifier),
                "{name}: duplicate numbering instance {identifier}"
            );
            let definition = instance
                .children()
                .find(|node| node.has_tag_name((WORD_NAMESPACE, "abstractNumId")))
                .unwrap()
                .attribute((WORD_NAMESPACE, "val"))
                .unwrap();
            assert!(
                abstract_identifiers.contains(definition),
                "{name}: missing numbering definition {definition}"
            );
        }
        let document = roxmltree::Document::parse(&parts["word/document.xml"]).unwrap();
        let relationships =
            roxmltree::Document::parse(&parts["word/_rels/document.xml.rels"]).unwrap();
        let identifiers: BTreeSet<_> = relationships
            .descendants()
            .filter_map(|node| node.attribute("Id"))
            .collect();
        for node in document.descendants() {
            if let Some(identifier) = node.attribute((
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                "id",
            )) {
                assert!(
                    identifiers.contains(identifier),
                    "{name}: unresolved relationship {identifier}"
                );
            }
        }
        for reference in document.descendants().filter(|node| {
            ["pStyle", "rStyle", "numId"]
                .iter()
                .any(|tag| node.has_tag_name((WORD_NAMESPACE, *tag)))
        }) {
            let identifier = reference.attribute((WORD_NAMESPACE, "val")).unwrap();
            let target_set = if reference.tag_name().name() == "numId" {
                &numbering_identifiers
            } else {
                &style_identifiers
            };
            assert!(
                target_set.contains(identifier),
                "{name}: unresolved {} {identifier}",
                reference.tag_name().name()
            );
        }
    }
}
