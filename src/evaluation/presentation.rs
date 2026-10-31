//! Deck declarations and explicit slide/box attribute contracts.
use super::{
    attributes, drawings,
    expressions::{EvaluatedArgument, evaluate_named_arguments},
    values::*,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, ElementKind, Properties, Value},
    syntax::Call,
};
use std::collections::BTreeMap;

pub(super) fn evaluate(
    call: &Call,
    span: Span,
    state: &mut super::EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    if !call.bodies.is_empty()
        || call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
    {
        return Err(Diagnostic::new(
            span,
            "presentation accepts named arguments and no content body",
        ));
    }
    let arguments = evaluate_named_arguments(&call.arguments, state)?;
    apply(arguments, state)
}

pub(super) fn apply(
    mut arguments: BTreeMap<String, EvaluatedArgument>,
    state: &mut super::EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    for (name, argument) in &arguments {
        let valid = match name.as_str() {
            "size" => matches!(argument.value, Value::String(_) | Value::Object(_)),
            "width" | "height" => matches!(
                argument.value,
                Value::Number(_) | Value::String(_) | Value::Length { .. }
            ),
            "background" => matches!(
                argument.value,
                Value::String(_) | Value::Color(_) | Value::Object(_)
            ),
            "footer" | "date" | "slide_number" => {
                matches!(argument.value, Value::String(_) | Value::Boolean(_))
            }
            "theme" | "properties" => matches!(argument.value, Value::Object(_)),
            _ => true,
        };
        if !valid {
            return Err(Diagnostic::new(
                argument.span,
                format!("invalid presentation {name} argument type"),
            ));
        }
    }
    let overrides = arguments.remove("properties");
    let mut fields: BTreeMap<_, _> = arguments
        .into_iter()
        .map(|(name, argument)| (name, (argument.value, argument.span)))
        .collect();
    if let Some(argument) = overrides {
        for (name, value) in require_object(argument.value, argument.span, "properties")? {
            if name != "properties" {
                let span = argument.fields.get(&name).copied().unwrap_or(argument.span);
                fields.insert(name, (value, span));
            }
        }
    }
    let normalized: Properties = fields
        .into_iter()
        .map(|(name, (value, span))| {
            let value = match name.as_str() {
                "width" | "height" => normalize_length(value, span, &name)?,
                "size" => size(value, span)?,
                "background" => background(value, span)?,
                "theme" => theme(value, span)?,
                "footer" | "date" => text_or_boolean(value, span, &name)?,
                "slide_number" => normalize_boolean(value, span, &name)?,
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<_, Diagnostic>>()?;
    if normalized.contains_key("background") {
        state.presentation.background_source_name = state.source_name().map(ToOwned::to_owned);
    }
    let presentation = &mut state.presentation;
    if let Some(value) = normalized.get("size") {
        presentation
            .size_properties
            .insert("size".into(), value.clone());
        if let Value::Object(fields) = value {
            for name in ["width", "height"] {
                if let Some(value) = fields.get(name) {
                    presentation.dimensions.insert(name.into(), value.clone());
                }
            }
        }
    }
    for name in ["width", "height"] {
        if let Some(value) = normalized.get(name) {
            presentation.dimensions.insert(name.into(), value.clone());
            presentation
                .size_properties
                .insert(name.into(), value.clone());
        }
    }
    presentation.properties.extend(normalized);
    Ok(())
}
fn size(value: Value, span: Span) -> Result<Value, Diagnostic> {
    match value {
        Value::Object(_) => attributes::normalize_length_map(value, span, "presentation.size"),
        Value::String(text) => Ok(Value::String(
            match text.trim() {
                "wide" | "widescreen" | "16:9" => "wide",
                "standard" | "4:3" => "standard",
                _ => {
                    return Err(Diagnostic::new(
                        span,
                        "presentation size must be wide, standard or a dimension table",
                    ));
                }
            }
            .into(),
        )),
        _ => Err(Diagnostic::new(
            span,
            "presentation size must be a string or property table",
        )),
    }
}
fn background(value: Value, span: Span) -> Result<Value, Diagnostic> {
    if let Value::Object(mut fields) = value {
        for name in ["color", "src", "fit"] {
            if let Some(value) = fields.remove(name) {
                let value = match name {
                    "color" => normalize_color(value, span, name)?,
                    "src" => Value::String(normalize_identifier(value, span, name)?),
                    _ => normalize_enum(
                        value,
                        span,
                        "background fit",
                        &["stretch", "cover", "contain"],
                    )?,
                };
                fields.insert(name.into(), value);
            }
        }
        Ok(Value::Object(fields))
    } else {
        normalize_color(value, span, "background")
    }
}
fn theme(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let mut fields = require_object(value, span, "theme")?;
    if let Some(Value::Object(colors)) = fields.get_mut("colors") {
        for name in [
            "background",
            "text",
            "dark1",
            "light1",
            "dark2",
            "light2",
            "accent1",
            "accent2",
            "accent3",
            "accent4",
            "accent5",
            "accent6",
            "hyperlink",
            "followed_hyperlink",
        ] {
            if let Some(value) = colors.remove(name) {
                colors.insert(name.into(), normalize_color(value, span, name)?);
            }
        }
    }
    if let Some(Value::Object(fonts)) = fields.get_mut("fonts") {
        for name in ["heading", "body", "major", "minor"] {
            if let Some(value) = fonts.remove(name) {
                fonts.insert(
                    name.into(),
                    Value::String(normalize_identifier(value, span, name)?),
                );
            }
        }
    }
    Ok(Value::Object(fields))
}
fn text_or_boolean(value: Value, span: Span, name: &str) -> Result<Value, Diagnostic> {
    if matches!(value, Value::String(_) | Value::Boolean(_)) {
        Ok(value)
    } else {
        Err(Diagnostic::new(
            span,
            format!("{name} must be a string or boolean"),
        ))
    }
}

pub(super) fn normalize(
    kind: ElementKind,
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    if kind == ElementKind::Slide {
        return match name {
            "name" => normalize_identifier(value, span, name).map(Value::String),
            "background" => background(value, span),
            "footer" | "date" => text_or_boolean(value, span, name),
            "slide_number" => normalize_boolean(value, span, name),
            "layout" => {
                let name = normalize_identifier(value, span, name)?;
                if name.trim().is_empty() {
                    return Err(Diagnostic::new(span, "slide layout must not be empty"));
                }
                Ok(Value::String(name.trim().into()))
            }
            _ => Ok(value),
        };
    }
    match name {
        "slot" => {
            let name = normalize_identifier(value, span, name)?;
            Ok(Value::String(name.trim().into()))
        }
        "kind" => {
            let kind = normalize_enum(
                value,
                span,
                "layout slot kind",
                &["content", "any", "text", "image", "table", "chart"],
            )?;
            Ok(if kind.to_text() == "any" {
                Value::String("content".into())
            } else {
                kind
            })
        }
        "required" | "remove" => normalize_boolean(value, span, name),
        "position" => drawings::position(value, span, "box.position"),
        "fit" => normalize_enum(value, span, name, &["contain", "cover", "stretch"]),
        "rotation" | "flip_horizontal" | "flip_vertical" => Ok(value),
        _ => drawings::normalize(ElementKind::Shape, name, value, span),
    }
}

pub(super) fn validate_children(
    kind: ElementKind,
    children: &[Content],
    span: Span,
) -> Result<(), Diagnostic> {
    let positioned = |child: &Content| matches!(child, Content::Element(element) if matches!(element.kind, ElementKind::Box | ElementKind::Shape | ElementKind::Line));
    if kind == ElementKind::Slide {
        if children.iter().any(positioned) && children.iter().any(|child| !positioned(child)) {
            return Err(Diagnostic::new(
                span,
                "slide cannot mix positioned objects and loose content",
            ));
        }
    } else {
        if matches!(children, [Content::Element(element)] if matches!(element.kind, ElementKind::Image | ElementKind::Table | ElementKind::Chart))
        {
            return Ok(());
        }
        if children.iter().any(|child| matches!(child, Content::Element(element) if (element.kind.is_block() || element.kind.is_table_part()) && !matches!(element.kind, ElementKind::Paragraph | ElementKind::Heading | ElementKind::List | ElementKind::OrderedList | ElementKind::ListItem | ElementKind::Blockquote | ElementKind::Raw | ElementKind::DefinitionList | ElementKind::Term | ElementKind::Definition | ElementKind::Divider))) {
            return Err(Diagnostic::new(span, "box accepts text flow or exactly one image, table or chart"));
        }
    }
    Ok(())
}
