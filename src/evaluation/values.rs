//! Shared validation for attribute values and typed helpers.

use crate::diagnostics::{Diagnostic, Span};
use crate::model::{Properties, Unit, Value};

pub(super) fn numeric_value(value: &Value) -> Option<f64> {
    let number = match value {
        Value::Number(number) => *number,
        Value::String(text) => text.parse::<f64>().ok()?,
        _ => return None,
    };
    number.is_finite().then_some(number)
}

pub(super) fn normalize_integer(
    value: Value,
    span: Span,
    property: &str,
    minimum: u32,
) -> Result<Value, Diagnostic> {
    let number = match &value {
        Value::Number(number) => Some(*number),
        Value::String(text) => text.parse::<i64>().ok().map(|number| number as f64),
        _ => None,
    };
    match number {
        Some(number)
            if number.is_finite()
                && number >= f64::from(minimum)
                && number < i64::MAX as f64
                && number.fract() == 0.0 =>
        {
            Ok(Value::Number(number))
        }
        _ => Err(Diagnostic::new(
            span,
            format!(
                "{property} must be a {} integer",
                if minimum == 0 {
                    "non-negative"
                } else {
                    "positive"
                }
            ),
        )),
    }
}

pub(super) fn normalize_boolean(
    value: Value,
    span: Span,
    property: &str,
) -> Result<Value, Diagnostic> {
    match value {
        Value::Boolean(_) => Ok(value),
        Value::String(text) => match text.as_str() {
            "true" | "True" | "TRUE" | "t" | "T" | "1" => Ok(Value::Boolean(true)),
            "false" | "False" | "FALSE" | "f" | "F" | "0" => Ok(Value::Boolean(false)),
            _ => Err(Diagnostic::new(
                span,
                format!("{property} must be a boolean"),
            )),
        },
        _ => Err(Diagnostic::new(
            span,
            format!("{property} must be a boolean"),
        )),
    }
}

pub(super) fn normalize_length(
    value: Value,
    span: Span,
    property: &str,
) -> Result<Value, Diagnostic> {
    if let Value::Length {
        value: magnitude, ..
    } = &value
    {
        if *magnitude >= 0.0 && magnitude.is_finite() {
            return Ok(value);
        }
    } else if let Some(magnitude) = numeric_value(&value) {
        if magnitude >= 0.0 {
            return Ok(value);
        }
    } else if let Value::String(text) = &value {
        for unit in [
            Unit::Twip,
            Unit::Point,
            Unit::Inch,
            Unit::Centimetre,
            Unit::Millimetre,
        ] {
            if let Some(magnitude) = text
                .strip_suffix(unit.as_text())
                .and_then(|text| text.trim().parse::<f64>().ok())
                .filter(|number| *number >= 0.0 && number.is_finite())
            {
                return Ok(Value::Length {
                    value: magnitude,
                    unit,
                });
            }
        }
    }
    Err(Diagnostic::new(
        span,
        format!("{property} must be a non-negative length"),
    ))
}

pub(super) fn normalize_color(
    value: Value,
    span: Span,
    property: &str,
) -> Result<Value, Diagnostic> {
    let text = match &value {
        Value::String(text) | Value::Color(text) => text.trim(),
        _ => {
            return Err(Diagnostic::new(
                span,
                format!("{property} must be a hex color"),
            ));
        }
    };
    let Some(digits) = text.strip_prefix('#').filter(|digits| {
        matches!(digits.len(), 3 | 6) && digits.bytes().all(|digit| digit.is_ascii_hexdigit())
    }) else {
        return Err(Diagnostic::new(
            span,
            format!("{property} must be a hex color like #fff or #ffffff"),
        ));
    };
    let digits = if digits.len() == 3 {
        digits
            .chars()
            .flat_map(|character| [character, character])
            .collect::<String>()
    } else {
        digits.to_owned()
    };
    Ok(Value::Color(format!("#{}", digits.to_ascii_lowercase())))
}

pub(super) fn require_string(
    value: Value,
    span: Span,
    property: &str,
) -> Result<Value, Diagnostic> {
    match value {
        Value::String(_) => Ok(value),
        _ => Err(Diagnostic::new(
            span,
            format!("{property} must be a string"),
        )),
    }
}

pub(super) fn normalize_identifier(
    value: Value,
    span: Span,
    property: &str,
) -> Result<String, Diagnostic> {
    match value {
        Value::String(text) => Ok(text),
        Value::Number(number) if number.is_finite() => Ok(number.to_string()),
        _ => Err(Diagnostic::new(
            span,
            format!("{property} must be a string or number"),
        )),
    }
}

pub(super) fn normalize_enum(
    value: Value,
    span: Span,
    property: &str,
    choices: &[&str],
) -> Result<Value, Diagnostic> {
    if matches!(&value, Value::String(text) if choices.contains(&text.as_str())) {
        return Ok(value);
    }
    Err(Diagnostic::new(
        span,
        format!("{property} must be one of {}", choices.join(", ")),
    ))
}

pub(super) fn require_object(
    value: Value,
    span: Span,
    property: &str,
) -> Result<Properties, Diagnostic> {
    match value {
        Value::Object(fields) => Ok(fields),
        _ => Err(Diagnostic::new(
            span,
            format!("{property} must be a property object"),
        )),
    }
}
