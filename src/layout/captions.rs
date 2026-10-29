//! Native figure ordering; numbering advances later, when each block is prepared.
use crate::{
    model::{Content, Element, ElementKind, Properties, Value},
    properties::text,
};
pub(super) const BEFORE: &str = "__layout_figure_before";
pub(super) const AFTER: &str = "__layout_figure_after";
pub(super) const KIND: &str = "__layout_figure_kind";
pub(super) const PARENT: &str = "__layout_figure_properties";

pub(super) fn expand(content: &[Content]) -> Vec<Content> {
    let mut output = Vec::new();
    let mut pending = Vec::new();
    for node in content {
        if crate::references::standalone_labels(node).is_some() {
            pending.push(node.clone());
            continue;
        }
        let Content::Element(element) = node else {
            output.append(&mut pending);
            output.push(node.clone());
            continue;
        };
        if element.kind == ElementKind::Figure {
            let start = output.len();
            let mut labels = std::mem::take(&mut pending);
            let alignment = text(&element.attributes, "alignment");
            let mut body = Vec::new();
            let mut caption = None;
            for child in &element.children {
                if crate::references::standalone_labels(child).is_some() {
                    labels.push(child.clone());
                    continue;
                }
                if let Content::Element(child) = child {
                    let child = crate::captions::aligned_child(child, &alignment);
                    if child.kind == ElementKind::Caption {
                        caption = Some(child);
                    } else {
                        body.push(Content::Element(child));
                    }
                } else {
                    body.push(child.clone());
                }
            }
            let above = caption
                .as_ref()
                .is_some_and(|caption| text(&caption.attributes, "position") == "above");
            if above {
                output.append(&mut labels);
                output.push(Content::Element(prepare(
                    caption.as_ref().unwrap(),
                    &text(&element.attributes, "kind"),
                    &element.attributes,
                )));
            }
            output.extend(expand(&body));
            if !above && let Some(caption) = caption {
                output.append(&mut labels);
                output.push(Content::Element(prepare(
                    &caption,
                    &text(&element.attributes, "kind"),
                    &element.attributes,
                )));
            }
            for marker in [BEFORE, AFTER] {
                let index = if marker == BEFORE {
                    (start..output.len()).find(|index| {
                        crate::references::standalone_labels(&output[*index]).is_none()
                    })
                } else {
                    (start..output.len()).rev().find(|index| {
                        crate::references::standalone_labels(&output[*index]).is_none()
                    })
                };
                if let Some(index) = index
                    && let Content::Element(element) = &mut output[index]
                {
                    element
                        .attributes
                        .insert(marker.into(), Value::Boolean(true));
                }
            }
        } else {
            output.append(&mut pending);
            output.push(if element.kind == ElementKind::Caption {
                Content::Element(prepare(element, "figure", &Properties::new()))
            } else {
                node.clone()
            });
        }
    }
    output.append(&mut pending);
    output
}
fn prepare(caption: &Element, kind: &str, parent: &Properties) -> Element {
    let mut caption = caption.clone();
    caption
        .attributes
        .insert(KIND.into(), Value::String(kind.into()));
    caption
        .attributes
        .insert(PARENT.into(), Value::Object(parent.clone()));
    caption
        .attributes
        .entry("style".into())
        .or_insert(Value::String("Caption".into()));
    caption
}
