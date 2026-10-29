//! Document-wide properties and normalized page geometry.

use super::{expressions::evaluate_named_arguments, values::*};
use crate::diagnostics::{Diagnostic, Span};
use crate::model::{Document, Properties, Value};
use crate::syntax::Call;

pub(super) fn evaluate(call: &Call, span: Span, document: &mut Document) -> Result<(), Diagnostic> {
    if !call.bodies.is_empty()
        || call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
    {
        return Err(Diagnostic::new(
            span,
            "document accepts named arguments and no content body",
        ));
    }
    for (name, argument) in evaluate_named_arguments(&call.arguments)? {
        let location = argument.source_expression.span;
        reject_flat_property(&name, location)?;
        match name.as_str() {
            "page" | "margin" => {
                let fields = normalize_geometry(&name, argument.value.clone(), location)?;
                if name == "page" {
                    document.page.extend(fields);
                } else {
                    document.margin.extend(fields);
                }
            }
            "columns" => {
                return Err(Diagnostic::new(
                    location,
                    "document columns are not supported yet",
                ));
            }
            _ => {}
        }
        if name.starts_with("default_") || name == "language" {
            document
                .defaults
                .insert(name.clone(), argument.value.clone());
        }
        document.properties.insert(name, argument.value);
    }
    Ok(())
}

fn reject_flat_property(name: &str, span: Span) -> Result<(), Diagnostic> {
    let replacement = if name == "orientation" {
        Some(("page", "orientation"))
    } else {
        ["page_", "margin_", "column_"]
            .iter()
            .find_map(|prefix| {
                name.strip_prefix(prefix)
                    .map(|field| (prefix.trim_end_matches('_'), field))
            })
            .filter(|_| name != "page_numbering")
    };
    if let Some((group, field)) = replacement {
        return Err(Diagnostic::new(
            span,
            format!("{name} is not supported; use {group} = {{ {field} = ... }}"),
        ));
    }
    Ok(())
}

fn normalize_geometry(group: &str, value: Value, span: Span) -> Result<Properties, Diagnostic> {
    require_object(value, span, group)?
        .into_iter()
        .map(|(name, value)| {
            let property = format!("{group}.{name}");
            let value = if group == "margin" || matches!(name.as_str(), "width" | "height") {
                normalize_length(value, span, &property)?
            } else if name == "orientation" {
                normalize_enum(value, span, &property, &["portrait", "landscape"])?
            } else {
                value
            };
            Ok((name, value))
        })
        .collect()
}
