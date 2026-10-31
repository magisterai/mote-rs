//! Scalar formatting and typed value helpers shared by static and Lua calls.
use super::{expressions::EvaluatedArguments, values::numeric_value};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Unit, Value},
};

pub(super) fn evaluate(
    name: &str,
    mut arguments: EvaluatedArguments,
    span: Span,
) -> Result<Value, Diagnostic> {
    let maximum = if matches!(name, "length" | "format_money") {
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
    if arguments.positional.is_empty()
        || arguments.positional.len() > maximum
        || name == "format_money"
            && arguments.positional.len() == 2
            && arguments.named.contains_key("currency")
        || arguments
            .named
            .keys()
            .any(|key| name != "format_money" || key != "currency")
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
            let mut grouped = String::new();
            for (index, character) in whole.chars().enumerate() {
                if index > 0 && (whole.len() - index).is_multiple_of(3) {
                    grouped.push(',');
                }
                grouped.push(character);
            }
            let sign = if amount < 0.0 { "-" } else { "" };
            Ok(Value::String(if currency == "USD" {
                format!("{sign}${grouped}.{decimal}")
            } else {
                format!("{sign}{currency} {grouped}.{decimal}")
            }))
        }
        _ => Err(Diagnostic::new(span, format!("unsupported helper: {name}"))),
    }
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
