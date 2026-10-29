//! Literal evaluation, named arguments, and value helpers shared by directives.

use super::values::{normalize_color, numeric_value};
use crate::diagnostics::{Diagnostic, Span};
use crate::model::{Properties, Unit, Value};
use crate::syntax::{Argument, Expression, ExpressionKind};
use std::collections::BTreeMap;

#[derive(Clone)]
pub(super) struct EvaluatedArgument {
    pub value: Value,
    pub span: Span,
    pub fields: BTreeMap<String, Span>,
}
impl EvaluatedArgument {
    pub fn new(value: Value, span: Span) -> Self {
        Self {
            value,
            span,
            fields: BTreeMap::new(),
        }
    }
}
#[derive(Default)]
pub(super) struct EvaluatedArguments {
    pub named: BTreeMap<String, EvaluatedArgument>,
    pub positional: Vec<EvaluatedArgument>,
}

pub(super) fn evaluate_arguments(
    arguments: &[Argument],
    state: &mut super::EvaluationState<'_>,
) -> Result<EvaluatedArguments, Diagnostic> {
    let mut evaluated = EvaluatedArguments::default();
    for argument in arguments {
        let mut value = EvaluatedArgument::new(
            evaluate_expression(&argument.value, state)?,
            argument.value.span,
        );
        if let ExpressionKind::Object(fields) = &argument.value.kind {
            value.fields = fields
                .iter()
                .filter_map(|field| {
                    field
                        .name
                        .as_ref()
                        .map(|name| (name.clone(), field.value.span))
                })
                .collect();
        }
        if let Some(name) = &argument.name {
            evaluated.named.insert(name.clone(), value);
        } else {
            evaluated.positional.push(value);
        }
    }
    Ok(evaluated)
}
pub(super) fn evaluate_named_arguments(
    arguments: &[Argument],
    state: &mut super::EvaluationState<'_>,
) -> Result<BTreeMap<String, EvaluatedArgument>, Diagnostic> {
    Ok(evaluate_arguments(arguments, state)?.named)
}

pub(super) fn evaluate_expression(
    expression: &Expression,
    state: &mut super::EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    match &expression.kind {
        ExpressionKind::Boolean(value) => state
            .bindings
            .resolve(if *value { "true" } else { "false" }, expression.span),
        ExpressionKind::Nil => state.bindings.resolve("nil", expression.span),
        _ => evaluate_static_expression(expression, state),
    }
}

fn evaluate_static_expression(
    expression: &Expression,
    state: &mut super::EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    Ok(match &expression.kind {
        ExpressionKind::String(text) => Value::String(text.clone()),
        ExpressionKind::Number(number) => Value::Number(*number),
        ExpressionKind::Boolean(boolean) => Value::Boolean(*boolean),
        ExpressionKind::Nil => Value::Nil,
        ExpressionKind::Array(entries) => Value::Array(
            entries
                .iter()
                .map(|entry| evaluate_static_expression(entry, state))
                .collect::<Result<_, _>>()?,
        ),
        ExpressionKind::Object(fields) => {
            let mut object = Properties::new();
            for field in fields {
                let Some(name) = &field.name else {
                    return Err(Diagnostic::new(field.span, "object fields must be named"));
                };
                object.insert(
                    name.clone(),
                    evaluate_static_expression(&field.value, state)?,
                );
            }
            Value::Object(object)
        }
        ExpressionKind::Call { name, arguments } => {
            evaluate_helper_arguments(name, arguments, expression.span, state, true)?
        }
        ExpressionKind::Path(name) => state.bindings.resolve(name, expression.span)?,
        ExpressionKind::Script(source) => {
            super::scripting::evaluate(source, expression.span, state)?
        }
    })
}

pub(super) fn evaluate_helper(
    name: &str,
    arguments: &[Argument],
    span: Span,
    state: &mut super::EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    evaluate_helper_arguments(name, arguments, span, state, false)
}

fn evaluate_helper_arguments(
    name: &str,
    arguments: &[Argument],
    span: Span,
    state: &mut super::EvaluationState<'_>,
    static_arguments: bool,
) -> Result<Value, Diagnostic> {
    let unit = Unit::from_name(name);
    if unit.is_none() && !matches!(name, "upper" | "color" | "json") {
        return Err(Diagnostic::new(span, format!("unsupported helper: {name}")));
    }
    let argument = match arguments {
        [argument] if argument.name.is_none() => argument,
        _ => {
            return Err(Diagnostic::new(
                span,
                format!("{name} requires one positional argument"),
            ));
        }
    };
    let value = if static_arguments {
        evaluate_static_expression(&argument.value, state)?
    } else {
        evaluate_expression(&argument.value, state)?
    };
    evaluate_helper_value(name, value, argument.value.span, state)
}

pub(super) fn evaluate_helper_value(
    name: &str,
    value: Value,
    span: Span,
    state: &mut super::EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    let unit = Unit::from_name(name);
    if name == "json" {
        return super::data::load_json(value, span, state);
    }
    if name == "color" {
        return normalize_color(value, span, "color");
    }
    if let Some(unit) = unit {
        let Some(magnitude) = numeric_value(&value) else {
            return Err(Diagnostic::new(
                span,
                "length magnitude must be a finite number",
            ));
        };
        return Ok(Value::Length {
            value: magnitude,
            unit,
        });
    }
    let text = value_as_text(&value, span)?;
    if !text.is_ascii() {
        return Err(Diagnostic::new(
            span,
            "upper currently supports ASCII text only",
        ));
    }
    Ok(Value::String(text.to_ascii_uppercase()))
}

pub(super) fn value_as_text(value: &Value, span: Span) -> Result<String, Diagnostic> {
    if matches!(value, Value::Object(_) | Value::Array(_)) {
        return Err(Diagnostic::new(
            span,
            "property objects and arrays cannot be inserted as text",
        ));
    }
    Ok(value.to_text())
}
