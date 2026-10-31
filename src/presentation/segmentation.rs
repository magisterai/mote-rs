//! Explicit slides and top-level Markdown boundaries, used only in presentation mode.
use crate::model::{Content, Element, ElementKind, Properties};

pub(super) fn slides(content: &[Content]) -> Vec<Element> {
    let mut slides = Vec::new();
    let mut pending = Vec::new();
    for node in content
        .iter()
        .filter(|node| super::automatic::meaningful(node))
    {
        match node {
            Content::Element(element) if element.kind == ElementKind::Slide => {
                flush(&mut pending, &mut slides);
                slides.push(element.clone());
            }
            Content::Element(element) if element.kind == ElementKind::Divider => {
                flush(&mut pending, &mut slides);
            }
            Content::Element(element)
                if element.kind == ElementKind::Heading
                    && matches!(
                        crate::properties::text(&element.attributes, "level").as_str(),
                        "" | "1"
                    ) =>
            {
                flush(&mut pending, &mut slides);
                pending.push(node.clone());
            }
            _ => pending.push(node.clone()),
        }
    }
    flush(&mut pending, &mut slides);
    slides
}
fn flush(pending: &mut Vec<Content>, slides: &mut Vec<Element>) {
    if !pending.is_empty() {
        slides.push(Element {
            source_name: None,
            kind: ElementKind::Slide,
            attributes: Properties::new(),
            children: std::mem::take(pending),
        });
    }
}
