//! Typed shape geometry, drawing appearance and nested text properties.
use super::{attributes, declarations, images, values::*};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{ElementKind, Properties, Unit, Value},
};

pub(super) fn normalize(
    kind: ElementKind,
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    match name {
        "position" if kind == ElementKind::Shape => position(value, span),
        "from" | "to" if kind == ElementKind::Line => point(value, span, name),
        "name" => normalize_identifier(value, span, name).map(Value::String),
        "border" => images::normalize(name, value, span),
        "opacity" => Ok(percent(value)),
        "arrow" if kind == ElementKind::Line => arrows(value, span),
        "cap" if kind == ElementKind::Line => {
            normalize_enum(value, span, name, &["flat", "round", "square"])
        }
        _ if kind == ElementKind::Line => Ok(value),
        "kind" => aliases(
            value,
            span,
            name,
            &[
                ("rect", "rectangle"),
                ("rounded", "round_rect"),
                ("oval", "ellipse"),
            ],
            &[
                "rectangle",
                "round_rect",
                "ellipse",
                "triangle",
                "diamond",
                "hexagon",
                "chevron",
                "cloud",
                "parallelogram",
                "trapezoid",
            ],
        ),
        "fill" | "color" => normalize_color(value, span, name),
        "rotation" => Ok(Value::String(
            value.to_text().trim().trim_end_matches("deg").into(),
        )),
        "flip_horizontal" | "flip_vertical" => normalize_boolean(value, span, name),
        "alignment" => normalize_enum(value, span, name, attributes::ALIGNMENTS),
        "vertical_alignment" => vertical_alignment(value, span),
        "margin" => attributes::normalize_length_map(value, span, name),
        "shadow" => shadow(value, span),
        "text" => text_properties(value, span),
        "highlight" => Ok(value),
        _ => attributes::normalize_attribute(ElementKind::Span, name, value, span),
    }
}
fn aliases(
    value: Value,
    span: Span,
    name: &str,
    aliases: &[(&str, &str)],
    allowed: &[&str],
) -> Result<Value, Diagnostic> {
    let value = if let Value::String(text) = &value {
        aliases
            .iter()
            .find(|(alias, _)| text == alias)
            .map(|(_, name)| Value::String((*name).into()))
            .unwrap_or(value)
    } else {
        value
    };
    normalize_enum(value, span, name, allowed)
}
fn vertical_alignment(value: Value, span: Span) -> Result<Value, Diagnostic> {
    aliases(
        value,
        span,
        "vertical_alignment",
        &[("middle", "center")],
        &["top", "center", "bottom"],
    )
}
fn percent(value: Value) -> Value {
    Value::String(value.to_text().trim().trim_end_matches('%').trim().into())
}
fn position(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let mut fields = require_object(value, span, "position")?;
    for name in ["x", "y"] {
        fields.entry(name.into()).or_insert(Value::Length {
            value: 0.0,
            unit: Unit::Inch,
        });
    }
    for name in ["width", "height"] {
        if !fields.contains_key(name) {
            return Err(Diagnostic::new(
                span,
                format!("shape.position requires {name}"),
            ));
        }
    }
    for name in ["x", "y", "width", "height"] {
        let value = fields.remove(name).unwrap();
        fields.insert(
            name.into(),
            normalize_length(value, span, &format!("position.{name}"))?,
        );
    }
    Ok(Value::Object(fields))
}
fn point(value: Value, span: Span, name: &str) -> Result<Value, Diagnostic> {
    let mut fields = require_object(value, span, name)?;
    for coordinate in ["x", "y"] {
        let value = fields
            .remove(coordinate)
            .ok_or_else(|| Diagnostic::new(span, format!("line.{name} requires {coordinate}")))?;
        fields.insert(
            coordinate.into(),
            normalize_length(value, span, &format!("{name}.{coordinate}"))?,
        );
    }
    Ok(Value::Object(fields))
}
fn shadow(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "shadow")?
        .into_iter()
        .map(|(name, value)| {
            let value = match name.as_str() {
                "color" => normalize_color(value, span, &name)?,
                "blur" | "distance" => normalize_length(value, span, &name)?,
                "opacity" => percent(value),
                "angle" => Value::String(value.to_text().trim().trim_end_matches("deg").into()),
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
fn text_properties(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "text")?
        .into_iter()
        .map(|(name, value)| {
            let value = match name.as_str() {
                "fit" => aliases(
                    value,
                    span,
                    "text.fit",
                    &[
                        ("none", "clip"),
                        ("autofit", "shrink"),
                        ("resize", "resize_shape"),
                    ],
                    &["clip", "shrink", "resize_shape"],
                )?,
                "margin" | "indent" => attributes::normalize_length_map(value, span, &name)?,
                "alignment" => normalize_enum(value, span, &name, attributes::ALIGNMENTS)?,
                "vertical_alignment" => vertical_alignment(value, span)?,
                "font" => Value::String(normalize_identifier(value, span, &name)?),
                "paragraph" => declarations::normalize_style_property("paragraph", value, span)?,
                "list" => text_list(value, span)?,
                "spacing" | "spacing_before" | "spacing_after" | "line_spacing" => {
                    attributes::normalize_attribute(ElementKind::Paragraph, &name, value, span)?
                }
                _ => attributes::normalize_attribute(ElementKind::Span, &name, value, span)?,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
fn text_list(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "text.list")?
        .into_iter()
        .map(|(name, value)| {
            let value = if matches!(
                name.as_str(),
                "indent" | "spacing" | "spacing_before" | "spacing_after" | "line_spacing"
            ) {
                attributes::normalize_attribute(ElementKind::Paragraph, &name, value, span)?
            } else {
                value
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
fn arrows(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let mut fields = match value {
        Value::String(_) => Properties::from([("end".into(), value)]),
        Value::Object(fields) => fields,
        _ => {
            return Err(Diagnostic::new(
                span,
                "line arrow must be a string or property table",
            ));
        }
    };
    for name in ["start", "end", "from", "to"] {
        if let Some(value) = fields.remove(name) {
            fields.insert(
                name.into(),
                aliases(
                    value,
                    span,
                    name,
                    &[("arrow", "triangle")],
                    &["none", "triangle", "stealth", "diamond", "oval"],
                )?,
            );
        }
    }
    for name in [
        "size",
        "length",
        "start_size",
        "start_length",
        "end_size",
        "end_length",
        "from_size",
        "from_length",
        "to_size",
        "to_length",
    ] {
        if let Some(value) = fields.remove(name) {
            fields.insert(
                name.into(),
                aliases(
                    value,
                    span,
                    name,
                    &[("small", "sm"), ("medium", "med"), ("large", "lg")],
                    &["sm", "med", "lg"],
                )?,
            );
        }
    }
    for (alias, target) in [
        ("from", "start"),
        ("to", "end"),
        ("from_size", "start_size"),
        ("from_length", "start_length"),
        ("to_size", "end_size"),
        ("to_length", "end_length"),
    ] {
        if let Some(value) = fields.remove(alias) {
            fields.entry(target.into()).or_insert(value);
        }
    }
    Ok(Value::Object(fields))
}
