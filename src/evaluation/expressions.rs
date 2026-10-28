//! Literal evaluation, named arguments, and value helpers shared by directives.

use super::values::{normalize_color, numeric_value};
use crate::diagnostics::{Diagnostic, Span};
use crate::model::{Properties, Unit, Value};
use crate::syntax::{Argument, Expression, ExpressionKind};
use std::collections::BTreeMap;

pub(super) struct EvaluatedArgument<'expression> {
    pub value: Value,
    pub source_expression: &'expression Expression,
}

pub(super) fn evaluate_named_arguments(
    arguments: &[Argument],
) -> Result<BTreeMap<String, EvaluatedArgument<'_>>, Diagnostic> {
    let mut evaluated = BTreeMap::new();
    for argument in arguments {
        if let Some(name) = &argument.name {
            evaluated.insert(
                name.clone(),
                EvaluatedArgument {
                    value: evaluate_expression(&argument.value)?,
                    source_expression: &argument.value,
                },
            );
        }
    }
    Ok(evaluated)
}

pub(super) fn evaluate_expression(expression: &Expression) -> Result<Value, Diagnostic> {
    Ok(match &expression.kind {
        ExpressionKind::String(text) => Value::String(text.clone()),
        ExpressionKind::Number(number) => Value::Number(*number),
        ExpressionKind::Boolean(boolean) => Value::Boolean(*boolean),
        ExpressionKind::Nil => Value::Nil,
        ExpressionKind::Array(entries) => Value::Array(
            entries
                .iter()
                .map(evaluate_expression)
                .collect::<Result<_, _>>()?,
        ),
        ExpressionKind::Object(fields) => {
            let mut object = Properties::new();
            for field in fields {
                let Some(name) = &field.name else {
                    return Err(Diagnostic::new(field.span, "object fields must be named"));
                };
                object.insert(name.clone(), evaluate_expression(&field.value)?);
            }
            Value::Object(object)
        }
        ExpressionKind::Call { name, arguments } => {
            evaluate_helper(name, arguments, expression.span)?
        }
        ExpressionKind::Path(name) => {
            return Err(Diagnostic::new(
                expression.span,
                format!("binding references are not supported yet: {name}"),
            ));
        }
    })
}

pub(super) fn evaluate_helper(
    name: &str,
    arguments: &[Argument],
    span: Span,
) -> Result<Value, Diagnostic> {
    let unit = Unit::from_name(name);
    if unit.is_none() && !matches!(name, "upper" | "color") {
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
    let value = evaluate_expression(&argument.value)?;
    if name == "color" {
        return normalize_color(value, argument.value.span, "color");
    }
    if let Some(unit) = unit {
        let Some(magnitude) = numeric_value(&value) else {
            return Err(Diagnostic::new(
                argument.value.span,
                "length magnitude must be a finite number",
            ));
        };
        return Ok(Value::Length {
            value: magnitude,
            unit,
        });
    }
    let text = value_as_text(&value, argument.value.span)?;
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
