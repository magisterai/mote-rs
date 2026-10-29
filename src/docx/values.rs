//! Output unit conversion. Unitless paragraph distances are twips.
use super::Error;
use crate::model::{Properties, Unit, Value};

pub(super) fn first<'value>(
    properties: &'value Properties,
    names: &[&str],
) -> Option<&'value Value> {
    names.iter().find_map(|name| properties.get(*name))
}

pub(super) fn object(value: &Value) -> Result<&Properties, Error> {
    match value {
        Value::Object(properties) => Ok(properties),
        _ => Err(Error::InvalidValue(
            "expected a property object for DOCX formatting".into(),
        )),
    }
}

pub(super) fn number(value: &Value) -> Result<f64, Error> {
    let number = match value {
        Value::Number(number) => *number,
        Value::String(text) => text
            .trim()
            .parse()
            .map_err(|_| Error::InvalidValue(format!("invalid number {text:?}")))?,
        _ => {
            return Err(Error::InvalidValue(
                "expected a numeric formatting value".into(),
            ));
        }
    };
    if !number.is_finite() {
        return Err(Error::InvalidValue(
            "formatting values must be finite".into(),
        ));
    }
    Ok(number)
}

pub(super) fn rounded(value: f64) -> Result<i64, Error> {
    if !value.is_finite() || value < 0.0 || value.round() > f64::from(i32::MAX) {
        return Err(Error::InvalidValue(
            "DOCX formatting value is outside the supported non-negative integer range".into(),
        ));
    }
    Ok(value.round() as i64)
}

pub(super) fn twips(value: &Value) -> Result<i64, Error> {
    let (magnitude, unit) = match value {
        Value::Length { value, unit } => (*value, *unit),
        Value::String(text) => {
            let text = text.trim();
            let mut parsed = None;
            for unit in [
                Unit::Point,
                Unit::Inch,
                Unit::Centimetre,
                Unit::Millimetre,
                Unit::Twip,
            ] {
                if let Some(magnitude) = text.strip_suffix(unit.as_text()) {
                    parsed = Some((
                        magnitude
                            .trim()
                            .parse::<f64>()
                            .map_err(|_| Error::InvalidValue(format!("invalid length {text:?}")))?,
                        unit,
                    ));
                    break;
                }
            }
            match parsed {
                Some(parsed) => parsed,
                None => (number(value)?, Unit::Twip),
            }
        }
        _ => (number(value)?, Unit::Twip),
    };
    rounded(match unit {
        Unit::Point => magnitude * 20.0,
        Unit::Inch => magnitude * 1440.0,
        Unit::Centimetre => magnitude * 1440.0 / 2.54,
        Unit::Millimetre => magnitude * 1440.0 / 25.4,
        Unit::Twip => magnitude,
    })
}

pub(super) fn boolean(value: &Value) -> Result<bool, Error> {
    match value {
        Value::Boolean(boolean) => Ok(*boolean),
        Value::String(text) => Ok(text.trim() == "true"),
        Value::Number(_) | Value::Nil => Ok(false),
        _ => Err(Error::InvalidValue(
            "expected a scalar boolean formatting value".into(),
        )),
    }
}

pub(super) fn color(value: &Value) -> Result<String, Error> {
    let text = value.to_text();
    let digits = text.trim().strip_prefix('#').unwrap_or(text.trim());
    if !matches!(digits.len(), 3 | 6)
        || !digits
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Err(Error::InvalidValue(format!("invalid DOCX color {text:?}")));
    }
    Ok(if digits.len() == 3 {
        digits
            .chars()
            .flat_map(|character| [character, character])
            .collect::<String>()
    } else {
        digits.into()
    }
    .to_uppercase())
}

pub(super) fn half_points(value: &Value) -> Result<i64, Error> {
    match value {
        Value::Length {
            value,
            unit: Unit::Point,
        } => rounded(value * 2.0),
        Value::Length { .. } => rounded(twips(value)? as f64 / 10.0),
        Value::String(text) if text.contains("pt") => rounded(twips(value)? as f64 / 10.0),
        _ => rounded(number(value)? * 2.0),
    }
}
