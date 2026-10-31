//! Paragraph projections for definition lists, without changing authored content.
use crate::model::{Content, Element, ElementKind, Properties, Value};

pub(crate) fn definition_paragraphs(list: &Element, preserve_paragraphs: bool) -> Vec<Element> {
    let mut paragraphs = Vec::new();
    for child in &list.children {
        let Content::Element(item) = child else {
            continue;
        };
        let style = match item.kind {
            ElementKind::Term => "DefinitionTerm",
            ElementKind::Definition => "Definition",
            _ => continue,
        };
        let structured = preserve_paragraphs && item.children.iter().any(|child| {
            matches!(child, Content::Element(element) if element.kind == ElementKind::Paragraph)
        });
        let units = if structured {
            crate::tables::paragraph_units(&item.children)
        } else {
            let children = item
                .children
                .iter()
                .flat_map(|child| match child {
                    Content::Element(element) if element.kind == ElementKind::Paragraph => {
                        element.children.clone()
                    }
                    child => vec![child.clone()],
                })
                .collect();
            vec![(Properties::new(), children)]
        };
        for (explicit, children) in units {
            let mut attributes = item.attributes.clone();
            attributes
                .entry("style".into())
                .or_insert(Value::String(style.into()));
            attributes.extend(explicit);
            paragraphs.push(Element {
                source_name: item.source_name.clone(),
                kind: ElementKind::Paragraph,
                attributes,
                children,
            });
        }
    }
    paragraphs
}
