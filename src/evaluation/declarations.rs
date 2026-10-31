//! Document-wide declarations; evaluating one does not produce visible content.

use super::{attributes, expressions::evaluate_named_arguments, values::*};
use crate::diagnostics::{Diagnostic, Span};
use crate::model::{ElementKind, FontDeclaration, Properties, StyleDeclaration, StyleKind, Value};
use crate::syntax::Call;

pub(super) fn evaluate_declaration(
    call: &Call,
    span: Span,
    document: &mut super::EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    if !call.bodies.is_empty()
        || call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
    {
        return Err(Diagnostic::new(
            span,
            format!("{} accepts named arguments and no content body", call.name),
        ));
    }
    let fields = evaluate_named_arguments(&call.arguments, document)?;
    apply(&call.name, fields, span, document)
}

pub(super) fn apply(
    directive: &str,
    mut fields: std::collections::BTreeMap<String, super::expressions::EvaluatedArgument>,
    span: Span,
    document: &mut super::EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    let name = fields
        .remove("name")
        .ok_or_else(|| Diagnostic::new(span, format!("{} requires name", directive)))?;
    let name_span = name.span;
    let numeric_name = matches!(name.value, Value::Number(_));
    let name = normalize_identifier(name.value, name_span, "name")?;
    if numeric_name {
        document.warn(Diagnostic::new(
            span,
            format!(
                "{} name was coerced from number to string {name:?}",
                directive
            ),
        ));
    }
    if directive == "font" && name.trim().is_empty() {
        return Err(Diagnostic::new(
            name_span,
            format!("{} name must not be empty", directive),
        ));
    }
    let mut properties = Properties::new();
    for (field, argument) in fields {
        let location = argument.span;
        let value = if directive == "font" {
            normalize_font_property(&field, argument.value, location)?
        } else {
            normalize_style_property(&field, argument.value, location)?
        };
        properties.insert(field, value);
    }
    if directive == "font" {
        let source_name = document.source_name().map(ToOwned::to_owned);
        document.fonts.insert(
            name.clone(),
            FontDeclaration {
                source_name,
                name,
                properties,
            },
        );
    } else {
        let kind = match properties.get("type") {
            Some(Value::String(value)) if value == "character" => StyleKind::Character,
            Some(Value::String(value)) if value == "table" => StyleKind::Table,
            _ => StyleKind::Paragraph,
        };
        document.styles.insert(
            name.clone(),
            StyleDeclaration {
                name,
                kind,
                properties,
                span,
            },
        );
    }
    Ok(())
}

fn normalize_font_property(name: &str, value: Value, span: Span) -> Result<Value, Diagnostic> {
    match name {
        "latin" | "east_asian" | "complex_script" => {
            let value = require_string(value, span, name)?;
            if matches!(&value, Value::String(text) if text.trim().is_empty()) {
                return Err(Diagnostic::new(
                    span,
                    format!("font {name} must not be empty"),
                ));
            }
            Ok(value)
        }
        "word_line_height" if matches!(&value, Value::Number(number) if number.is_finite() && *number > 0.0) => {
            Ok(value)
        }
        "word_line_height" => Err(Diagnostic::new(
            span,
            "font word_line_height must be a positive finite number",
        )),
        _ => Ok(value),
    }
}

pub(super) fn normalize_style_property(
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    match name {
        "type" => normalize_enum(
            value,
            span,
            "style type",
            &["paragraph", "character", "table"],
        ),
        "based_on" | "next" | "link" | "font" => require_string(value, span, name),
        "paragraph" | "run" => {
            let fields = require_object(value, span, name)?;
            fields
                .into_iter()
                .map(|(field, value)| {
                    let value = if name == "paragraph" {
                        normalize_paragraph_property(&field, value, span)?
                    } else {
                        normalize_run_property(&field, value, span)?
                    };
                    Ok((field, value))
                })
                .collect::<Result<Properties, Diagnostic>>()
                .map(Value::Object)
        }
        "table" => normalize_table_group(value, span, TableGroup::Table),
        "size" | "spacing_before" | "spacing_after" | "line_spacing" | "first_line_indent"
        | "hanging_indent" => normalize_length(value, span, name),
        "color" | "fill" | "background" | "highlight" => normalize_color(value, span, name),
        "bold" | "italic" | "underline" | "small_caps" | "keep_with_next" | "keep_together"
        | "page_break_before" | "widow_control" => normalize_boolean(value, span, name),
        _ => Ok(value),
    }
}

fn normalize_paragraph_property(name: &str, value: Value, span: Span) -> Result<Value, Diagnostic> {
    if name == "background" {
        return normalize_color(value, span, name);
    }
    attributes::normalize_attribute(ElementKind::Paragraph, name, value, span)
}

fn normalize_run_property(name: &str, value: Value, span: Span) -> Result<Value, Diagnostic> {
    if matches!(name, "fill" | "background") {
        return normalize_color(value, span, name);
    }
    attributes::normalize_attribute(ElementKind::Span, name, value, span)
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum TableGroup {
    Table,
    Region,
    Cell,
    Banding,
}

pub(super) fn normalize_table_group(
    value: Value,
    span: Span,
    group: TableGroup,
) -> Result<Value, Diagnostic> {
    let fields = require_object(value, span, "table style group")?;
    if group == TableGroup::Table && fields.contains_key("cell_padding") {
        return Err(Diagnostic::new(
            span,
            "cell_padding is not supported; use cell.margin",
        ));
    }
    fields
        .into_iter()
        .map(|(name, value)| {
            let value = match (group, name.as_str()) {
                (TableGroup::Table, "width" | "indent" | "cell_spacing") => {
                    normalize_length(value, span, &name)?
                }
                (TableGroup::Table, "layout") => {
                    normalize_enum(value, span, &name, &["auto", "autofit", "fixed"])?
                }
                (TableGroup::Table | TableGroup::Region, "cell") => {
                    normalize_table_group(value, span, TableGroup::Cell)?
                }
                (TableGroup::Table, "first_row" | "last_row" | "first_column" | "last_column") => {
                    normalize_table_group(value, span, TableGroup::Region)?
                }
                (TableGroup::Table, "banding") => {
                    normalize_table_group(value, span, TableGroup::Banding)?
                }
                (TableGroup::Banding, "rows" | "columns") => normalize_boolean(value, span, &name)?,
                (TableGroup::Banding, "odd" | "even" | "odd_column" | "even_column") => {
                    normalize_table_group(value, span, TableGroup::Region)?
                }
                (TableGroup::Table | TableGroup::Region, "paragraph" | "text") => {
                    let nested = require_object(value, span, &name)?;
                    Value::Object(
                        nested
                            .into_iter()
                            .map(|(field, value)| {
                                let value = if name == "paragraph" {
                                    normalize_paragraph_property(&field, value, span)?
                                } else {
                                    normalize_run_property(&field, value, span)?
                                };
                                Ok((field, value))
                            })
                            .collect::<Result<Properties, Diagnostic>>()?,
                    )
                }
                (TableGroup::Table | TableGroup::Region | TableGroup::Cell, "alignment") => {
                    normalize_enum(value, span, &name, attributes::ALIGNMENTS)?
                }
                (TableGroup::Region | TableGroup::Cell, "fill") => {
                    normalize_color(value, span, &name)?
                }
                (TableGroup::Region | TableGroup::Cell, "margin") => {
                    attributes::normalize_length_map(value, span, &name)?
                }
                (TableGroup::Region | TableGroup::Cell, "no_wrap") => {
                    normalize_boolean(value, span, &name)?
                }
                (TableGroup::Region | TableGroup::Cell, "vertical_alignment") => {
                    let value =
                        normalize_enum(value, span, &name, &["top", "center", "middle", "bottom"])?;
                    if value == Value::String("middle".into()) {
                        Value::String("center".into())
                    } else {
                        value
                    }
                }
                (TableGroup::Region | TableGroup::Cell, "text_direction") => {
                    normalize_enum(value, span, &name, &["ltr", "rtl", "tb_rl", "bt_lr"])?
                }
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
