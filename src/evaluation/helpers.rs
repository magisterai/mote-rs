//! Scalar formatting and typed value helpers shared by static and Lua calls.
use super::{expressions::EvaluatedArguments, values::numeric_value};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Unit, Value},
};

/// The helpers that format or type a single value, evaluated alike in static
/// and Lua calls.
pub(super) const SCALAR_HELPERS: &[&str] = &[
    "upper",
    "lower",
    "format_money",
    "format_number",
    "length",
    "date",
];

pub(super) fn evaluate(
    name: &str,
    mut arguments: EvaluatedArguments,
    span: Span,
) -> Result<Value, Diagnostic> {
    let maximum = if matches!(name, "length" | "format_money" | "format_number") {
        2
    } else {
        1
    };
    if maximum == 1 && arguments.positional.len() != 1 {
        return Err(Diagnostic::new(
            span,
            format!("{name} requires one positional argument"),
        ));
    }
    // The second positional argument may instead be named.
    let (second, named): (&str, &[&str]) = match name {
        "format_money" => ("currency", &["currency"]),
        "format_number" => ("decimals", &["decimals", "grouping"]),
        _ => ("", &[]),
    };
    if arguments.positional.is_empty()
        || arguments.positional.len() > maximum
        || arguments.positional.len() == 2 && arguments.named.contains_key(second)
        || arguments
            .named
            .keys()
            .any(|key| !named.contains(&key.as_str()))
    {
        return Err(Diagnostic::new(
            span,
            format!("invalid arguments for {name}"),
        ));
    }
    let first = arguments.positional.remove(0);
    match name {
        "upper" | "lower" => Ok(Value::String(crate::casing::convert(
            &first.value.to_text(),
            name == "upper",
        ))),
        "date" => date(first.value, first.span),
        "length" => {
            let unit = arguments.positional.first().map(|argument| &argument.value);
            if unit.is_some_and(|value| !matches!(value, Value::String(_))) {
                return Err(Diagnostic::new(span, "length unit must be a string"));
            }
            let unit = unit.map(Value::to_text).unwrap_or_else(|| "pt".into());
            let unit = match unit.trim() {
                "pt" => Unit::Point,
                "in" => Unit::Inch,
                "cm" => Unit::Centimetre,
                "mm" => Unit::Millimetre,
                "twip" => Unit::Twip,
                _ => {
                    return Err(Diagnostic::new(
                        span,
                        "length unit must be pt, in, cm, mm or twip",
                    ));
                }
            };
            let value = number(&first.value).ok_or_else(|| {
                Diagnostic::new(first.span, "length magnitude must be a finite number")
            })?;
            Ok(Value::Length { value, unit })
        }
        "format_money" => {
            let amount = number(&first.value).ok_or_else(|| {
                Diagnostic::new(first.span, "format_money amount must be a finite number")
            })?;
            let currency = arguments
                .named
                .get("currency")
                .or_else(|| arguments.positional.first());
            if currency.is_some_and(|argument| !matches!(argument.value, Value::String(_))) {
                return Err(Diagnostic::new(span, "currency must be a string"));
            }
            let currency = currency
                .map(|argument| argument.value.to_text())
                .unwrap_or_else(|| "USD".into());
            let currency = crate::casing::convert(&currency, true);
            let formatted = format!("{:.2}", if amount < 0.0 { -amount } else { amount });
            let (whole, decimal) = formatted.split_once('.').unwrap();
            let grouped = group(whole);
            let sign = if amount < 0.0 { "-" } else { "" };
            Ok(Value::String(if currency == "USD" {
                format!("{sign}${grouped}.{decimal}")
            } else {
                format!("{sign}{currency} {grouped}.{decimal}")
            }))
        }
        "format_number" => {
            let value = number(&first.value).ok_or_else(|| {
                Diagnostic::new(first.span, "format_number value must be a finite number")
            })?;
            let decimals = match arguments
                .named
                .get("decimals")
                .or_else(|| arguments.positional.first())
            {
                None => 0,
                Some(argument) => match argument.value {
                    Value::Number(decimals)
                        if decimals.fract() == 0.0 && (0.0..=15.0).contains(&decimals) =>
                    {
                        decimals as usize
                    }
                    _ => {
                        return Err(Diagnostic::new(
                            argument.span,
                            "format_number decimals must be a whole number from 0 to 15",
                        ));
                    }
                },
            };
            let grouping = match arguments.named.get("grouping") {
                None => false,
                Some(argument) => match argument.value {
                    Value::Boolean(grouping) => grouping,
                    _ => {
                        return Err(Diagnostic::new(
                            argument.span,
                            "format_number grouping must be a boolean",
                        ));
                    }
                },
            };
            let formatted = format!("{:.decimals$}", value.abs());
            let (whole, decimal) = formatted
                .split_once('.')
                .unwrap_or((formatted.as_str(), ""));
            let mut text = if grouping {
                group(whole)
            } else {
                whole.to_owned()
            };
            if !decimal.is_empty() {
                text.push('.');
                text.push_str(decimal);
            }
            // A value that rounds to zero has no sign.
            if value < 0.0 && formatted.bytes().any(|byte| matches!(byte, b'1'..=b'9')) {
                text.insert(0, '-');
            }
            Ok(Value::String(text))
        }
        _ => Err(Diagnostic::new(span, format!("unsupported helper: {name}"))),
    }
}
/// Separates the thousands of a run of digits with commas.
fn group(digits: &str) -> String {
    let mut grouped = String::new();
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(character);
    }
    grouped
}
fn number(value: &Value) -> Option<f64> {
    if !matches!(value, Value::String(_) | Value::Number(_)) {
        return None;
    }
    // Numeric strings are parsed as supplied; whitespace is not a numeric token.
    if matches!(value,Value::String(text) if text.trim()!=text) {
        return None;
    }
    numeric_value(value)
}
pub(super) fn date(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let text = match value {
        Value::Date(value) => return Ok(Value::Date(value)),
        Value::String(value) => value.trim().to_owned(),
        _ => return Err(Diagnostic::new(span, "date must be an ISO date string")),
    };
    let bytes = text.as_bytes();
    let valid = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit());
    if valid {
        let year = text[..4].parse::<i32>().unwrap();
        let month = text[5..7].parse::<u8>().unwrap();
        let day = text[8..].parse::<u8>().unwrap();
        if time::Month::try_from(month)
            .ok()
            .is_some_and(|month| time::Date::from_calendar_date(year, month, day).is_ok())
        {
            return Ok(Value::Date(text));
        }
    }
    Err(Diagnostic::new(
        span,
        "date must be a valid ISO calendar date (YYYY-MM-DD)",
    ))
}
