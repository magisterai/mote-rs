//! Private, per-pass field values on cloned content. Authored models are never mutated.
use crate::model::{Content, ElementKind, Value};
use std::collections::BTreeMap;
pub(super) const RESOLVED_TEXT: &str = "__layout_field_text";
pub(super) const IDENTIFIER: &str = "__layout_field_identifier";

pub(super) fn resolve(
    content: &mut [Content],
    current: usize,
    total: usize,
    mut identifiers: Option<&mut usize>,
    values: &BTreeMap<usize, usize>,
) {
    for child in content {
        let Content::Element(element) = child else {
            continue;
        };
        if element.kind == ElementKind::PageNumber {
            let number = if crate::properties::text(&element.attributes, "kind") == "total" {
                total
            } else if let Some(next) = &mut identifiers {
                let identifier = **next;
                **next += 1;
                element
                    .attributes
                    .insert(IDENTIFIER.into(), Value::Number(identifier as f64));
                values.get(&identifier).copied().unwrap_or(current)
            } else {
                current
            };
            element
                .attributes
                .insert(RESOLVED_TEXT.into(), Value::String(number.to_string()));
        }
        resolve(
            &mut element.children,
            current,
            total,
            identifiers.as_deref_mut(),
            values,
        );
    }
}
