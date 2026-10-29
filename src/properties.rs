//! Scalar and object access for retained authoring properties.
use crate::model::{Properties, Value};

pub(crate) fn text(properties: &Properties, name: &str) -> String {
    properties.get(name).map(Value::to_text).unwrap_or_default()
}
pub(crate) fn object<'value>(
    properties: &'value Properties,
    name: &str,
) -> Option<&'value Properties> {
    match properties.get(name) {
        Some(Value::Object(fields)) => Some(fields),
        _ => None,
    }
}
pub(crate) fn number(properties: &Properties, name: &str) -> Option<f64> {
    text(properties, name)
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|number| number.is_finite())
}
pub(crate) fn boolean(properties: &Properties, name: &str) -> bool {
    properties.get(name) == Some(&Value::Boolean(true)) || text(properties, name) == "true"
}
