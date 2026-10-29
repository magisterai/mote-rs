//! Argument validation, property merging, and directive-specific normalization.

use super::{
    expressions::{EvaluatedArguments, evaluate_arguments},
    values::*,
};
use crate::diagnostics::{Diagnostic, Span};
use crate::model::{ElementKind, Properties, Value};
use crate::syntax::Argument;
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
    state: &mut super::EvaluationState<'_>,
) -> Result<Properties, Diagnostic> {
    normalize_arguments(kind, evaluate_arguments(arguments, state)?)
}

pub(super) fn normalize_arguments(
    kind: ElementKind,
    mut arguments: EvaluatedArguments,
) -> Result<Properties, Diagnostic> {
    for (name, argument) in &arguments.named {
        validate_named_argument(kind, name, &argument.value, argument.span)?;
    }
    let properties_argument = arguments.named.remove("properties");
    let mut attributes: BTreeMap<_, _> = arguments
        .named
        .into_iter()
        .map(|(name, argument)| (name, (argument.value, argument.span)))
        .collect();
    if let Some(argument) = properties_argument {
        for (name, value) in require_object(argument.value, argument.span, "properties")? {
            if name == "properties" {
                continue;
            }
            let location = argument.fields.get(&name).copied().unwrap_or(argument.span);
            attributes.insert(name, (value, location));
        }
    }
    if let Some(argument) = arguments.positional.into_iter().next() {
        let (primary, valid, message) = match kind {
            ElementKind::Heading => (
                "level",
                matches!(argument.value, Value::Number(_)),
                "positional heading level must be a number",
            ),
            ElementKind::Link => (
                "url",
                matches!(argument.value, Value::String(_)),
                "link URL must be a string",
            ),
            ElementKind::Image => (
                "src",
                matches!(argument.value, Value::String(_)),
                "image source must be a string",
            ),
            ElementKind::Bookmark | ElementKind::Label => (
                "name",
                matches!(argument.value, Value::String(_)),
                "positional name or target must be a string",
            ),
            ElementKind::Reference => (
                "target",
                matches!(argument.value, Value::String(_)),
                "positional name or target must be a string",
            ),
            _ => ("", true, ""),
        };
        if !valid {
            return Err(Diagnostic::new(argument.span, message));
        }
        if !primary.is_empty() {
            attributes.insert(primary.into(), (argument.value, argument.span));
        }
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
    StringOrObject,
}

impl ArgumentType {
    fn accepts(self, value: &Value) -> bool {
        match self {
            Self::StringOrObject => matches!(value, Value::String(_) | Value::Object(_)),
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
            Self::StringOrObject => "string or object",
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
            Footnote | Endnote => match name {
                "id" => Some(ArgumentType::NumberOrString),
                _ => None,
            },
            Figure | Caption => match name {
                "kind" | "style" | "alignment" | "position" => Some(ArgumentType::String),
                "numbering" | "placement" => Some(ArgumentType::Object),
                _ => None,
            },
            Bookmark | Label => match name {
                "name" | "kind" | "title" => Some(ArgumentType::String),
                "number" if kind == Label => Some(ArgumentType::NumberOrString),
                _ => None,
            },
            Reference => match name {
                "target" => Some(ArgumentType::StringOrObject),
                "kind" | "display" | "fallback" => Some(ArgumentType::String),
                _ => None,
            },
            PageNumber => match name {
                "kind" | "format" => Some(ArgumentType::String),
                _ => None,
            },
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
            Table => match name {
                "style" | "layout" | "alignment" => Some(ArgumentType::String),
                "width" | "indent" | "cell_spacing" => Some(ArgumentType::Length),
                "cell" | "borders" => Some(ArgumentType::Object),
                _ => None,
            },
            Row => match name {
                "repeat_header" | "cant_split" => Some(ArgumentType::Boolean),
                "height" => Some(ArgumentType::Length),
                "height_rule" => Some(ArgumentType::String),
                _ => None,
            },
            Cell => match name {
                "style" | "alignment" | "vertical_alignment" | "text_direction" => {
                    Some(ArgumentType::String)
                }
                "width" => Some(ArgumentType::Length),
                "colspan" | "rowspan" => Some(ArgumentType::NumberOrString),
                "fill" => Some(ArgumentType::Color),
                "margin" | "borders" => Some(ArgumentType::Object),
                "no_wrap" => Some(ArgumentType::Boolean),
                _ => None,
            },
            Image => match name {
                "src" | "name" | "alt" | "description" | "style" | "title" | "link"
                | "alignment" => Some(ArgumentType::String),
                "width" | "height" => Some(ArgumentType::Length),
                "rotation" => Some(ArgumentType::NumberOrString),
                "flip_horizontal" | "flip_vertical" => Some(ArgumentType::Boolean),
                "placement" | "spacing" | "crop" | "border" | "effects" => {
                    Some(ArgumentType::Object)
                }
                _ => None,
            },
            Link => match name {
                "url" | "title" | "tooltip" | "target" => Some(ArgumentType::String),
                _ => None,
            },
            Strong | Emphasis | LineBreak | Tab => None,
            Break if name == "type" => Some(ArgumentType::String),
            Break => None,
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
        Figure | Caption => match name {
            "alignment" => normalize_enum(value, span, name, ALIGNMENTS),
            "position" if kind == Caption => normalize_enum(value, span, name, &["above", "below"]),
            "numbering" => normalize_numbering(value, span),
            "placement" if kind == Figure => super::images::normalize(name, value, span),
            _ => Ok(value),
        },
        Reference if name == "kind" => {
            normalize_enum(value, span, name, &["title", "number", "page", "text"])
        }
        Reference if name == "display" => normalize_enum(
            value,
            span,
            name,
            &["auto", "title", "number", "page", "text"],
        ),
        PageNumber if name == "kind" => normalize_enum(value, span, name, &["current", "total"]),
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
        Image => super::images::normalize(name, value, span),
        Table | Row | Cell => super::tables::normalize(kind, name, value, span),
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
        Break if name == "type" => {
            normalize_enum(value, span, "break type", &["page", "column", "line"])
        }
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
