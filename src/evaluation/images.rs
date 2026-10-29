use super::{
    attributes::{ALIGNMENTS, evaluate_attributes, normalize_length_map},
    expressions::evaluate_expression,
    values::*,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Element, ElementKind, Value},
    syntax::Call,
};

pub(super) fn evaluate(
    call: &Call,
    span: Span,
    state: &super::EvaluationState,
) -> Result<Content, Diagnostic> {
    if !call.bodies.is_empty() {
        return Err(Diagnostic::new(span, "image accepts no body"));
    }
    let positional: Vec<_> = call
        .arguments
        .iter()
        .filter(|argument| argument.name.is_none())
        .collect();
    if positional.len() > 1
        || (!positional.is_empty()
            && call
                .arguments
                .iter()
                .any(|argument| argument.name.as_deref() == Some("src")))
    {
        return Err(Diagnostic::new(span, "image source must be supplied once"));
    }
    let mut attributes = evaluate_attributes(ElementKind::Image, &call.arguments, state)?;
    if let Some(argument) = positional.first() {
        attributes.insert(
            "src".into(),
            require_string(
                evaluate_expression(&argument.value, state)?,
                argument.span,
                "image source",
            )?,
        );
    }
    if attributes
        .get("src")
        .is_none_or(|value| value.to_text().is_empty())
    {
        return Err(Diagnostic::new(span, "image requires a source"));
    }
    Ok(Content::Element(Element {
        kind: ElementKind::Image,
        attributes,
        children: Vec::new(),
    }))
}
pub(super) fn normalize(name: &str, value: Value, span: Span) -> Result<Value, Diagnostic> {
    match name {
        "width" | "height" => normalize_length(value, span, name),
        "alignment" => normalize_enum(value, span, name, ALIGNMENTS),
        "spacing" => normalize_length_map(value, span, name),
        "rotation" => Ok(Value::String(
            value.to_text().trim().trim_end_matches("deg").into(),
        )),
        "flip_horizontal" | "flip_vertical" => normalize_boolean(value, span, name),
        "placement" => placement(value, span),
        "crop" | "effects" | "border" => {
            // Extra fields are retained as metadata, without inferred output effects.
            let Value::Object(fields) = value else {
                return Ok(value);
            };
            fields
                .into_iter()
                .map(|(field, value)| {
                    let value = match (name, field.as_str()) {
                        ("crop", "top" | "right" | "bottom" | "left")
                        | ("effects", "brightness" | "contrast" | "transparency")
                        | ("border", "opacity") => percent_text(value),
                        ("effects", "grayscale") => normalize_boolean(value, span, &field)?,
                        ("border", "width") => normalize_length(value, span, &field)?,
                        ("border", "color") => normalize_color(value, span, &field)?,
                        ("border", "style") => normalize_enum(
                            value,
                            span,
                            &field,
                            &["single", "solid", "dashed", "dotted", "none"],
                        )?,
                        _ => value,
                    };
                    Ok((field, value))
                })
                .collect::<Result<_, Diagnostic>>()
                .map(Value::Object)
        }
        _ => Ok(value),
    }
}
fn percent_text(value: Value) -> Value {
    Value::String(value.to_text().trim().trim_end_matches('%').trim().into())
}
fn placement(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "placement")?
        .into_iter()
        .map(|(name, value)| {
            let value = match name.as_str() {
                "width" | "height" | "x" | "y" => normalize_length(value, span, &name)?,
                "margin" => normalize_length_map(value, span, &name)?,
                "align" => normalize_enum(value, span, &name, ALIGNMENTS)?,
                "type" => normalize_enum(value, span, &name, &["inline", "anchor", "floating"])?,
                "float" => normalize_enum(value, span, &name, &["none", "top", "bottom"])?,
                "wrap" => normalize_enum(
                    value,
                    span,
                    &name,
                    &[
                        "inline",
                        "square",
                        "tight",
                        "through",
                        "top_bottom",
                        "behind",
                        "in_front",
                    ],
                )?,
                "anchor" => normalize_enum(
                    value,
                    span,
                    &name,
                    &["page", "margin", "paragraph", "character"],
                )?,
                "allow_overlap" | "behind_text" | "layout_in_cell" | "locked" => {
                    normalize_boolean(value, span, &name)?
                }
                "relative_height" => normalize_integer(value, span, &name, 0)?,
                "horizontal" | "vertical" => axis(value, span, &name)?,
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<_, Diagnostic>>()
        .map(Value::Object)
}
fn axis(value: Value, span: Span, axis_name: &str) -> Result<Value, Diagnostic> {
    require_object(value, span, axis_name)?
        .into_iter()
        .map(|(name, value)| {
            let value = match name.as_str() {
                "offset" => normalize_length(value, span, &name)?,
                "alignment" => normalize_enum(
                    value,
                    span,
                    &name,
                    &[
                        "left", "center", "right", "top", "bottom", "inside", "outside",
                    ],
                )?,
                "relative_to" => normalize_enum(
                    value,
                    span,
                    &name,
                    if axis_name == "horizontal" {
                        &["margin", "page", "column", "character"]
                    } else {
                        &["margin", "page", "paragraph", "line"]
                    },
                )?,
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<_, Diagnostic>>()
        .map(Value::Object)
}
