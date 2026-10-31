//! Deterministic visible text for block objects without geometry in a context.
use crate::{
    model::{Element, ElementKind},
    properties,
};

pub(super) fn text(element: &Element) -> String {
    let condensed = |text: String| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut body = condensed(crate::references::text(&element.children));
    if element.kind == ElementKind::Math {
        let source = properties::text(&element.attributes, "source")
            .trim()
            .to_owned();
        if !source.is_empty() {
            body = source;
        }
    }
    let label = element.kind.as_text();
    if body.is_empty() {
        format!("[{label}]")
    } else {
        format!("[{label}] {body}")
    }
}
