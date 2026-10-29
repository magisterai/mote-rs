//! Argument validation, property merging, and directive-specific normalization.

use super::{
    expressions::{evaluate_expression, evaluate_named_arguments},
    values::*,
};
use crate::diagnostics::{Diagnostic, Span};
use crate::model::{ElementKind, Properties, Value};
use crate::syntax::{Argument, ExpressionKind};
use std::collections::BTreeMap;

pub(super) const ALIGNMENTS: &[&str] = &["left", "center", "right", "justify"];
const NUMBER_FORMATS: &[&str] = &[
    "decimal",
    "lower_alpha",
    "upper_alpha",
    "lower_roman",
    "upper_roman",
];

pub(super) fn evaluate_attributes(
    kind: ElementKind,
    arguments: &[Argument],
) -> Result<Properties, Diagnostic> {
    let mut named_arguments = evaluate_named_arguments(arguments)?;
    // Repeated names use their final value. Outer types are checked before overrides.
    for (name, argument) in &named_arguments {
        validate_named_argument(kind, name, &argument.value, argument.source_expression.span)?;
    }
    let properties_argument = named_arguments.remove("properties");
    let mut attributes: BTreeMap<_, _> = named_arguments
        .into_iter()
        .map(|(name, argument)| (name, (argument.value, argument.source_expression.span)))
        .collect();
    if let Some(argument) = properties_argument {
        let expression = argument.source_expression;
        let fields = require_object(argument.value, expression.span, "properties")?;
        let locations: BTreeMap<_, _> = if let ExpressionKind::Object(fields) = &expression.kind {
            fields
                .iter()
                .filter_map(|field| field.name.as_deref().map(|name| (name, field.value.span)))
                .collect()
        } else {
            BTreeMap::new()
        };
        for (name, value) in fields {
            if name == "properties" {
                continue;
            }
            let location = locations
                .get(name.as_str())
                .copied()
                .unwrap_or(expression.span);
            attributes.insert(name, (value, location));
        }
    }
    if kind == ElementKind::Heading
        && let Some(argument) = arguments.iter().find(|argument| argument.name.is_none())
    {
        let value = evaluate_expression(&argument.value)?;
        if !matches!(value, Value::Number(_)) {
            return Err(Diagnostic::new(
                argument.value.span,
                "positional heading level must be a number",
            ));
        }
        attributes.insert("level".into(), (value, argument.value.span));
    }
    attributes
        .into_iter()
        .map(|(name, (value, span))| {
            Ok((name.clone(), normalize_attribute(kind, &name, value, span)?))
        })
        .collect()
}

#[derive(Clone, Copy)]
enum ArgumentType {
    String,
    NumberOrString,
    Length,
    Color,
    Boolean,
    Object,
    Collection,
}

impl ArgumentType {
    fn accepts(self, value: &Value) -> bool {
        match self {
            Self::String => matches!(value, Value::String(_)),
            Self::NumberOrString => matches!(value, Value::Number(_) | Value::String(_)),
            Self::Length => matches!(
                value,
                Value::Number(_) | Value::String(_) | Value::Length { .. }
            ),
            Self::Color => matches!(value, Value::String(_) | Value::Color(_)),
            Self::Boolean => matches!(value, Value::Boolean(_) | Value::String(_)),
            Self::Object => matches!(value, Value::Object(_)),
            Self::Collection => matches!(value, Value::Object(_) | Value::Array(_)),
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::NumberOrString => "number or string",
            Self::Length => "length",
            Self::Color => "color",
            Self::Boolean => "boolean",
            Self::Object => "object",
            Self::Collection => "collection",
        }
    }
}

fn validate_named_argument(
    kind: ElementKind,
    name: &str,
    value: &Value,
    span: Span,
) -> Result<(), Diagnostic> {
    use ElementKind::*;
    let constraint = if name == "properties" {
        Some(ArgumentType::Object)
    } else {
        match kind {
            Paragraph => match name {
                "style" | "alignment" => Some(ArgumentType::String),
                "spacing_before" | "spacing_after" | "line_spacing" => Some(ArgumentType::Length),
                "spacing" | "indent" | "border" => Some(ArgumentType::Object),
                "fill" => Some(ArgumentType::Color),
                "tabs" => Some(ArgumentType::Collection),
                "keep_with_next" | "keep_together" | "page_break_before" | "widow_control" => {
                    Some(ArgumentType::Boolean)
                }
                "outline_level" => Some(ArgumentType::NumberOrString),
                _ => None,
            },
            Heading => match name {
                "level" => Some(ArgumentType::NumberOrString),
                "style" | "alignment" => Some(ArgumentType::String),
                "spacing_before" | "spacing_after" => Some(ArgumentType::Length),
                "spacing" | "indent" | "numbering" => Some(ArgumentType::Object),
                "keep_with_next" | "page_break_before" | "widow_control" => {
                    Some(ArgumentType::Boolean)
                }
                _ => None,
            },
            Span => match name {
                "style" | "font" => Some(ArgumentType::String),
                "size" => Some(ArgumentType::Length),
                "color" | "highlight" => Some(ArgumentType::Color),
                "bold" | "italic" | "underline" | "small_caps" | "strike" | "double_strike" => {
                    Some(ArgumentType::Boolean)
                }
                _ => None,
            },
            Code => match name {
                "style" | "language" => Some(ArgumentType::String),
                "preserve_space" => Some(ArgumentType::Boolean),
                _ => None,
            },
            List | OrderedList => match name {
                "id" | "start" | "level" => Some(ArgumentType::NumberOrString),
                "style" | "marker" | "format" => Some(ArgumentType::String),
                "restart" | "tight" => Some(ArgumentType::Boolean),
                "indent" | "spacing" => Some(ArgumentType::Object),
                _ => None,
            },
            ListItem => match name {
                "marker" => Some(ArgumentType::String),
                "level" => Some(ArgumentType::NumberOrString),
                "tight" | "checked" => Some(ArgumentType::Boolean),
                "checkbox" | "indent" | "spacing" => Some(ArgumentType::Object),
                _ => None,
            },
            Blockquote => match name {
                "style" => Some(ArgumentType::String),
                "border" => Some(ArgumentType::Object),
                "fill" => Some(ArgumentType::Color),
                _ => None,
            },
            Strong | Emphasis | LineBreak | Tab => None,
        }
    };
    if let Some(constraint) = constraint
        && !constraint.accepts(value)
    {
        return Err(Diagnostic::new(
            span,
            format!("{name} must have {} type", constraint.description()),
        ));
    }
    Ok(())
}

pub(super) fn normalize_attribute(
    kind: ElementKind,
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    use ElementKind::*;
    match kind {
        Paragraph | Heading => match name {
            "spacing_before" | "spacing_after" | "line_spacing" => {
                normalize_length(value, span, name)
            }
            "spacing" => normalize_spacing(value, span),
            "indent" => normalize_length_map(value, span, name),
            "keep_with_next" | "keep_together" | "page_break_before" | "widow_control" => {
                normalize_boolean(value, span, name)
            }
            "fill" => normalize_color(value, span, name),
            "outline_level" => normalize_integer(value, span, name, 0),
            "alignment" => normalize_enum(value, span, name, ALIGNMENTS),
            "level" if kind == Heading => normalize_integer(value, span, "heading level", 1),
            "numbering" if kind == Heading => normalize_numbering(value, span),
            _ => Ok(value),
        },
        Span => match name {
            "color" | "highlight" => normalize_color(value, span, name),
            "size" => normalize_length(value, span, name),
            "bold" | "italic" | "underline" | "small_caps" | "strike" | "double_strike" => {
                normalize_boolean(value, span, name)
            }
            _ => Ok(value),
        },
        List | OrderedList | ListItem => match name {
            "id" if kind != ListItem => Ok(Value::String(value.to_text())),
            "start" if kind != ListItem => normalize_integer(value, span, "list start", 1),
            "level" => normalize_integer(value, span, "list level", 1),
            "restart" if kind != ListItem => normalize_boolean(value, span, name),
            "checked" if kind == ListItem => normalize_boolean(value, span, name),
            "tight" => normalize_boolean(value, span, name),
            "indent" => normalize_length_map(value, span, name),
            "spacing" => normalize_spacing(value, span),
            "format" if kind != ListItem => normalize_enum(
                value,
                span,
                "list format",
                &[
                    "bullet",
                    "decimal",
                    "lower_alpha",
                    "upper_alpha",
                    "lower_roman",
                    "upper_roman",
                ],
            ),
            "checkbox" if kind == ListItem => normalize_checkbox(value, span),
            _ => Ok(value),
        },
        Blockquote if name == "fill" => normalize_color(value, span, name),
        Code if name == "preserve_space" => normalize_boolean(value, span, name),
        _ => Ok(value),
    }
}

pub(super) fn normalize_length_map(
    value: Value,
    span: Span,
    property: &str,
) -> Result<Value, Diagnostic> {
    require_object(value, span, property)?
        .into_iter()
        .map(|(name, value)| {
            let value = normalize_length(value, span, &format!("{property}.{name}"))?;
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}

fn normalize_spacing(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "spacing")?
        .into_iter()
        .map(|(name, value)| {
            let value = if matches!(name.as_str(), "line" | "line_spacing") {
                normalize_line_spacing(value, span)?
            } else {
                normalize_length(value, span, "spacing")?
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}

fn normalize_line_spacing(value: Value, span: Span) -> Result<Value, Diagnostic> {
    if !matches!(value, Value::Object(_)) {
        return normalize_length(value, span, "spacing.line");
    }
    let mut fields = require_object(value, span, "spacing.line")?;
    let rule = fields
        .remove("rule")
        .ok_or_else(|| Diagnostic::new(span, "spacing.line.rule is required"))?;
    let rule = normalize_enum(
        rule,
        span,
        "spacing.line.rule",
        &["multiple", "exact", "at_least"],
    )?;
    let value = fields
        .remove("value")
        .ok_or_else(|| Diagnostic::new(span, "spacing.line.value is required"))?;
    let value = if rule == Value::String("multiple".into()) {
        Value::Number(
            numeric_value(&value)
                .filter(|number| *number > 0.0)
                .ok_or_else(|| {
                    Diagnostic::new(
                        span,
                        "spacing.line.value must be a positive number for multiple",
                    )
                })?,
        )
    } else {
        normalize_length(value, span, "spacing.line.value")?
    };
    fields.insert("rule".into(), rule);
    fields.insert("value".into(), value);
    Ok(Value::Object(fields))
}

fn normalize_numbering(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "numbering")?
        .into_iter()
        .map(|(name, value)| {
            let property = format!("numbering.{name}");
            let value = match name.as_str() {
                "enabled" | "restart" => normalize_boolean(value, span, &property)?,
                "start" => normalize_integer(value, span, &property, 1)?,
                "format" => normalize_enum(
                    Value::String(value.to_text()),
                    span,
                    &property,
                    NUMBER_FORMATS,
                )?,
                "id" | "label" | "prefix" | "separator" => Value::String(value.to_text()),
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}

fn normalize_checkbox(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "checkbox")?
        .into_iter()
        .map(|(name, value)| {
            let property = format!("checkbox.{name}");
            let value = match name.as_str() {
                "checked" => normalize_boolean(value, span, &property)?,
                "kind" => normalize_enum(
                    value,
                    span,
                    &property,
                    &["content_control", "legacy_form_field"],
                )?,
                "name" | "title" => Value::String(value.to_text()),
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
