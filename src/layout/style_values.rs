use super::LineSpacing;
use crate::model::{Properties, Unit, Value};

pub(super) fn text(properties: &Properties, name: &str) -> String {
    properties.get(name).map(Value::to_text).unwrap_or_default()
}
pub(super) fn boolean(properties: &Properties, name: &str) -> bool {
    match properties.get(name) {
        Some(Value::Boolean(value)) => *value,
        Some(Value::String(value)) => value.trim() == "true",
        _ => false,
    }
}
pub(super) fn color(properties: &Properties, name: &str) -> Option<String> {
    let text = text(properties, name);
    let text = text.trim().trim_start_matches('#');
    if !matches!(text.len(), 3 | 6) || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    Some(
        if text.len() == 3 {
            text.chars()
                .flat_map(|character| [character, character])
                .collect::<String>()
        } else {
            text.into()
        }
        .to_uppercase(),
    )
}
pub(super) fn number(value: &Value) -> Option<f64> {
    let number = match value {
        Value::Number(number) => *number,
        _ => value.to_text().trim().parse().ok()?,
    };
    number.is_finite().then_some(number)
}
/// Native style distances resolve through integer twips, including unitless sizes.
pub(super) fn points(value: &Value) -> Option<f64> {
    let (magnitude, unit) = match value {
        Value::Length { value, unit } => (*value, *unit),
        Value::Number(value) => (*value, Unit::Twip),
        _ => {
            let text = value.to_text();
            let text = text.trim();
            let suffix = [
                Unit::Point,
                Unit::Inch,
                Unit::Centimetre,
                Unit::Millimetre,
                Unit::Twip,
            ]
            .into_iter()
            .find(|unit| text.ends_with(unit.as_text()));
            match suffix {
                Some(unit) => (
                    text.strip_suffix(unit.as_text())?.trim().parse().ok()?,
                    unit,
                ),
                None => (text.parse().ok()?, Unit::Twip),
            }
        }
    };
    let twips = match unit {
        Unit::Point => magnitude * 20.0,
        Unit::Inch => magnitude * 1440.0,
        Unit::Centimetre => magnitude * 1440.0 / 2.54,
        Unit::Millimetre => magnitude * 1440.0 / 25.4,
        Unit::Twip => magnitude,
    };
    (twips.is_finite() && twips.abs() <= i32::MAX as f64).then(|| twips.round() / 20.0)
}
pub(super) fn spacing(value: &Value) -> Option<LineSpacing> {
    if let Value::Object(properties) = value {
        let value = properties.get("value")?;
        return match text(properties, "rule").trim() {
            "multiple" => number(value)
                .filter(|value| *value > 0.0)
                .map(LineSpacing::Multiple),
            "exact" => points(value)
                .filter(|value| *value >= 0.0)
                .map(LineSpacing::Exact),
            "at_least" => points(value)
                .filter(|value| *value >= 0.0)
                .map(LineSpacing::AtLeast),
            _ => None,
        };
    }
    points(value).map(LineSpacing::Exact)
}
pub(super) fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect()
}
