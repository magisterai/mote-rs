//! Chart argument normalization; data interpretation belongs to the chart model.
use super::values::*;
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Properties, Value},
};

const KINDS: &[&str] = &[
    "column",
    "bar",
    "line",
    "pie",
    "doughnut",
    "stacked_column",
    "stacked_bar",
    "percent_stacked_column",
    "percent_stacked_bar",
    "area",
    "stacked_area",
    "percent_stacked_area",
];
const LEGEND_POSITIONS: &[(&str, &str)] =
    &[("right", "r"), ("left", "l"), ("top", "t"), ("bottom", "b")];
const LABEL_POSITIONS: &[(&str, &str)] = &[
    ("center", "ctr"),
    ("inside_base", "inBase"),
    ("inside_end", "inEnd"),
    ("outside_end", "outEnd"),
    ("best_fit", "bestFit"),
    ("left", "l"),
    ("right", "r"),
    ("top", "t"),
    ("bottom", "b"),
    ("ctr", "ctr"),
    ("inBase", "inBase"),
    ("inEnd", "inEnd"),
    ("outEnd", "outEnd"),
    ("bestFit", "bestFit"),
    ("l", "l"),
    ("r", "r"),
    ("t", "t"),
    ("b", "b"),
];

pub(super) fn normalize(name: &str, value: Value, span: Span) -> Result<Value, Diagnostic> {
    match name {
        "kind" => normalize_enum(value, span, name, KINDS),
        "title" => normalize_identifier(value, span, name).map(Value::String),
        "width" | "height" => normalize_length(value, span, name),
        "legend" => legend(value, span),
        "colors" => colors(value, span),
        "background" => normalize_color(value, span, name),
        "border" => border(value, span),
        "plot_area" | "gridlines" | "data_labels" => appearance(name, value, span),
        "gap_width" | "overlap" => integer(value, span, name),
        _ => Ok(value),
    }
}

fn choice(
    value: Value,
    span: Span,
    name: &str,
    choices: &[(&str, &str)],
) -> Result<Value, Diagnostic> {
    choices
        .iter()
        .find(|(input, _)| *input == value.to_text())
        .map(|(_, output)| Value::String((*output).into()))
        .ok_or_else(|| {
            Diagnostic::new(
                span,
                format!(
                    "{name} must be one of {}",
                    choices
                        .iter()
                        .map(|(input, _)| *input)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
        })
}

fn integer(value: Value, span: Span, name: &str) -> Result<Value, Diagnostic> {
    let number = match value {
        Value::Number(number)
            if number.is_finite()
                && number.fract() == 0.0
                && number >= i64::MIN as f64
                && number < i64::MAX as f64 =>
        {
            Some(number)
        }
        Value::String(text) => text.parse::<i64>().ok().map(|number| number as f64),
        _ => None,
    };
    number
        .map(Value::Number)
        .ok_or_else(|| Diagnostic::new(span, format!("{name} must be an integer")))
}

fn legend(value: Value, span: Span) -> Result<Value, Diagnostic> {
    match value {
        Value::Boolean(_) => Ok(value),
        Value::Object(mut fields) => {
            if let Some(value) = fields.remove("enabled") {
                fields.insert(
                    "enabled".into(),
                    normalize_boolean(value, span, "legend.enabled")?,
                );
            }
            if let Some(value) = fields.remove("position") {
                fields.insert(
                    "position".into(),
                    choice(value, span, "legend.position", LEGEND_POSITIONS)?,
                );
            }
            Ok(Value::Object(fields))
        }
        Value::String(ref text) if text == "true" || text == "false" => {
            Ok(Value::Boolean(text == "true"))
        }
        _ => Ok(Value::Object(Properties::from([(
            "position".into(),
            choice(value, span, "legend", LEGEND_POSITIONS)?,
        )]))),
    }
}

fn colors(value: Value, span: Span) -> Result<Value, Diagnostic> {
    match value {
        Value::Array(values) => values
            .into_iter()
            .map(|value| normalize_color(value, span, "chart color"))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        Value::Object(fields) => fields
            .into_iter()
            .map(|(name, value)| Ok((name.clone(), normalize_color(value, span, &name)?)))
            .collect::<Result<Properties, Diagnostic>>()
            .map(Value::Object),
        _ => Err(Diagnostic::new(
            span,
            "chart colors must be an array or property table",
        )),
    }
}

fn border(value: Value, span: Span) -> Result<Value, Diagnostic> {
    match value {
        Value::Boolean(_) => Ok(value),
        Value::String(_) | Value::Color(_) => normalize_color(value, span, "border"),
        Value::Object(fields) => fields
            .into_iter()
            .map(|(name, value)| {
                let normalized = match name.as_str() {
                    "enabled" => normalize_boolean(value, span, "border.enabled"),
                    "color" => normalize_color(value, span, "border.color"),
                    "width" => normalize_length(value, span, "border.width"),
                    _ => Ok(value),
                }?;
                Ok((name, normalized))
            })
            .collect::<Result<Properties, Diagnostic>>()
            .map(Value::Object),
        _ => Err(Diagnostic::new(
            span,
            "border must be a boolean, color or property table",
        )),
    }
}

fn appearance(group: &str, value: Value, span: Span) -> Result<Value, Diagnostic> {
    if matches!(value, Value::Boolean(_)) {
        return Ok(value);
    }
    let fields = require_object(value, span, group)?;
    fields
        .into_iter()
        .map(|(name, value)| {
            let path = format!("{group}.{name}");
            let value = match (group, name.as_str()) {
                ("plot_area", "background" | "fill" | "color") | ("gridlines", "color") => {
                    normalize_color(value, span, &path)
                }
                ("plot_area", "border") => border(value, span),
                ("gridlines", "width") => normalize_length(value, span, &path),
                ("gridlines", "major" | "minor")
                | (
                    "data_labels",
                    "enabled" | "value" | "category" | "series" | "percent" | "percentage",
                ) => normalize_boolean(value, span, &path),
                ("data_labels", "position") => choice(value, span, &path, LABEL_POSITIONS),
                _ => Ok(value),
            }?;
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
