//! Attribute contracts for the existing content directives.

use super::{evaluate_expression, values::*};
use crate::model::{ElementKind, Properties, Value};
use crate::syntax::{Argument, Diagnostic, ExpressionKind, Span};
use std::collections::BTreeMap;

const ALIGNMENTS: &[&str] = &["left", "center", "right", "justify"];
const NUMBER_FORMATS: &[&str] = &[
    "decimal",
    "lower_alpha",
    "upper_alpha",
    "lower_roman",
    "upper_roman",
];
const LIST_FORMATS: &[&str] = &[
    "bullet",
    "decimal",
    "lower_alpha",
    "upper_alpha",
    "lower_roman",
    "upper_roman",
];
const PARAGRAPH_ATTRIBUTES: &[&str] = &[
    "style",
    "alignment",
    "spacing",
    "spacing_before",
    "spacing_after",
    "line_spacing",
    "indent",
    "border",
    "fill",
    "tabs",
    "keep_with_next",
    "keep_together",
    "page_break_before",
    "widow_control",
    "outline_level",
];
const HEADING_ATTRIBUTES: &[&str] = &[
    "level",
    "style",
    "alignment",
    "spacing",
    "spacing_before",
    "spacing_after",
    "line_spacing",
    "indent",
    "keep_with_next",
    "keep_together",
    "page_break_before",
    "widow_control",
    "numbering",
];
const INLINE_ATTRIBUTES: &[&str] = &["style", "font", "size", "color"];
const CODE_ATTRIBUTES: &[&str] = &[
    "style",
    "font",
    "size",
    "color",
    "language",
    "preserve_space",
];
const LIST_ATTRIBUTES: &[&str] = &[
    "id", "style", "start", "marker", "format", "restart", "tight", "level", "indent", "spacing",
];
const ITEM_ATTRIBUTES: &[&str] = &[
    "style", "marker", "level", "tight", "checked", "checkbox", "indent", "spacing",
];
const QUOTE_ATTRIBUTES: &[&str] = &[
    "style",
    "border",
    "fill",
    "alignment",
    "spacing",
    "spacing_before",
    "spacing_after",
    "line_spacing",
    "indent",
];

pub(super) fn evaluate_attributes(
    kind: ElementKind,
    arguments: &[Argument],
) -> Result<Properties, Diagnostic> {
    let mut attributes = BTreeMap::new();
    let mut property_fields = BTreeMap::new();
    for argument in arguments {
        let Some(name) = &argument.name else { continue };
        let value = evaluate_expression(&argument.value)?;
        if name == "properties" {
            let fields = require_object(value, argument.value.span, "properties")?;
            property_fields.clear();
            for (name, value) in fields {
                let span = if let ExpressionKind::Object(expressions) = &argument.value.kind {
                    expressions
                        .iter()
                        .rev()
                        .find(|field| field.name.as_ref() == Some(&name))
                        .map_or(argument.value.span, |field| field.value.span)
                } else {
                    argument.value.span
                };
                property_fields.insert(name, (value, span));
            }
        } else {
            attributes.insert(name.clone(), (value, argument.value.span));
        }
    }
    attributes.extend(property_fields);
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

fn normalize_attribute(
    kind: ElementKind,
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    let allowed = match kind {
        ElementKind::Paragraph => PARAGRAPH_ATTRIBUTES,
        ElementKind::Heading => HEADING_ATTRIBUTES,
        ElementKind::Strong | ElementKind::Emphasis => INLINE_ATTRIBUTES,
        ElementKind::Code => CODE_ATTRIBUTES,
        ElementKind::List | ElementKind::OrderedList => LIST_ATTRIBUTES,
        ElementKind::ListItem => ITEM_ATTRIBUTES,
        ElementKind::Blockquote => QUOTE_ATTRIBUTES,
        ElementKind::LineBreak => &[],
    };
    if !allowed.contains(&name) {
        return Err(Diagnostic::new(
            span,
            format!("unsupported {} attribute: {name}", kind.as_text()),
        ));
    }
    let is_list = matches!(
        kind,
        ElementKind::List | ElementKind::OrderedList | ElementKind::ListItem
    );
    match name {
        "style" | "font" | "language" | "marker" => require_string(
            value,
            span,
            if kind == ElementKind::ListItem && name == "marker" {
                "item marker"
            } else {
                name
            },
        ),
        "id" => normalize_identifier(value, span, name),
        "level" => normalize_integer(
            value,
            span,
            if kind == ElementKind::Heading {
                "heading level"
            } else {
                "list level"
            },
            1,
        ),
        "start" => normalize_integer(value, span, "list start", 1),
        "outline_level" => normalize_integer(value, span, name, 0),
        "alignment" => normalize_enum(value, span, name, ALIGNMENTS).map_err(|_| {
            Diagnostic::new(span, "alignment must be left, center, right, or justify")
        }),
        "keep_with_next" | "keep_together" | "page_break_before" | "widow_control" | "restart"
        | "tight" | "checked" | "preserve_space" => normalize_boolean(value, span, name),
        "fill" => normalize_color(value, span, name),
        "color" => {
            normalize_color(value.clone(), span, name)?;
            Ok(value)
        }
        "size" => {
            normalize_length(value.clone(), span, name)?;
            Ok(value)
        }
        "spacing_before" | "spacing_after" | "line_spacing" => {
            let normalized = normalize_length(value.clone(), span, name)?;
            Ok(if kind == ElementKind::Blockquote {
                value
            } else {
                normalized
            })
        }
        "spacing" => {
            let normalized = normalize_spacing(value.clone(), span)?;
            Ok(if kind == ElementKind::Blockquote {
                value
            } else {
                normalized
            })
        }
        "indent" => {
            let fields = require_object(value.clone(), span, name)?;
            require_fields(
                &fields,
                if is_list {
                    &["left", "hanging", "level"]
                } else {
                    &["left", "right", "first_line", "hanging"]
                },
                span,
                name,
            )?;
            let normalized = fields
                .into_iter()
                .map(|(name, value)| {
                    Ok((
                        name.clone(),
                        normalize_length(value, span, &format!("indent.{name}"))?,
                    ))
                })
                .collect::<Result<Properties, Diagnostic>>()?;
            Ok(if kind == ElementKind::Blockquote {
                value
            } else {
                Value::Object(normalized)
            })
        }
        "format" => normalize_enum(value, span, "list format", LIST_FORMATS),
        "numbering" => normalize_numbering(value, span),
        "checkbox" => normalize_checkbox(value, span),
        "border" => {
            validate_border(&value, span)?;
            Ok(value)
        }
        "tabs" => {
            validate_tabs(&value, span)?;
            Ok(value)
        }
        _ => unreachable!("attribute inventory and normalizers must agree"),
    }
}

fn normalize_spacing(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let fields = require_object(value, span, "spacing")?;
    require_fields(
        &fields,
        &[
            "before",
            "after",
            "line",
            "spacing_before",
            "spacing_after",
            "line_spacing",
        ],
        span,
        "spacing",
    )?;
    fields
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
    require_fields(&fields, &["rule", "value"], span, "spacing.line")?;
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
    Ok(Value::Object(Properties::from([
        ("rule".into(), rule),
        ("value".into(), value),
    ])))
}

fn normalize_numbering(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let fields = require_object(value, span, "numbering")?;
    require_fields(
        &fields,
        &["enabled", "id", "format", "separator", "start", "restart"],
        span,
        "numbering",
    )?;
    fields
        .into_iter()
        .map(|(name, value)| {
            let property = format!("numbering.{name}");
            let value = match name.as_str() {
                "enabled" | "restart" => normalize_boolean(value, span, &property)?,
                "start" => normalize_integer(value, span, &property, 1)?,
                "format" => normalize_enum(value, span, &property, NUMBER_FORMATS)?,
                _ => normalize_identifier(value, span, &property)?,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}

fn normalize_checkbox(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let fields = require_object(value, span, "checkbox")?;
    require_fields(
        &fields,
        &["checked", "kind", "name", "title"],
        span,
        "checkbox",
    )?;
    fields
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
                _ => normalize_identifier(value, span, &property)?,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}

fn validate_border(value: &Value, span: Span) -> Result<(), Diagnostic> {
    let fields = require_object(value.clone(), span, "border")?;
    require_fields(
        &fields,
        &["top", "right", "bottom", "left", "all"],
        span,
        "border",
    )?;
    for (side, value) in fields {
        if matches!(value, Value::String(_)) {
            continue;
        }
        let property = format!("border.{side}");
        let fields = require_object(value, span, &property)?;
        require_fields(
            &fields,
            &["style", "val", "size", "width", "space", "color"],
            span,
            &property,
        )?;
        for (name, value) in fields {
            let property = format!("border.{side}.{name}");
            match name.as_str() {
                "style" | "val" => {
                    require_string(value, span, &property)?;
                }
                "color" => {
                    normalize_color(value, span, &property)?;
                }
                "width" => {
                    normalize_length(value, span, &property)?;
                }
                _ => {
                    normalize_integer(value, span, &property, 0)?;
                }
            }
        }
    }
    Ok(())
}

fn validate_tabs(value: &Value, span: Span) -> Result<(), Diagnostic> {
    let stops = match value {
        Value::Array(stops) => stops.as_slice(),
        Value::Object(_) => std::slice::from_ref(value),
        _ => {
            return Err(Diagnostic::new(
                span,
                "tabs must be a property object or array",
            ));
        }
    };
    for stop in stops {
        let fields = require_object(stop.clone(), span, "tab stop")?;
        if fields.len() == 1 {
            let (alignment, position) = fields.first_key_value().unwrap();
            if [
                "left", "center", "right", "decimal", "bar", "clear", "start", "end",
            ]
            .contains(&alignment.as_str())
            {
                normalize_length(position.clone(), span, "tab stop position")?;
                continue;
            }
        }
        require_fields(
            &fields,
            &[
                "position",
                "pos",
                "alignment",
                "align",
                "leader",
                "left",
                "center",
                "right",
            ],
            span,
            "tab stop",
        )?;
        if !["position", "pos", "left", "center", "right"]
            .iter()
            .any(|name| fields.contains_key(*name))
        {
            return Err(Diagnostic::new(span, "tab stop position is required"));
        }
        for (name, value) in fields {
            if matches!(name.as_str(), "alignment" | "align" | "leader") {
                require_string(value, span, &format!("tab stop {name}"))?;
            } else {
                normalize_length(value, span, "tab stop position")?;
            }
        }
    }
    Ok(())
}
