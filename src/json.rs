//! Deterministic JSON for inspecting evaluated content, not a layout export.

use crate::model::{Content, Document, Properties, Value};
use serde_json::{Map, Number, Value as JsonValue, json};
use std::io;

pub fn to_json(document: &Document) -> Result<String, serde_json::Error> {
    let content = document
        .content
        .iter()
        .map(content_as_json)
        .collect::<Result<Vec<_>, _>>()?;
    serde_json::to_string_pretty(&json!({"content": content}))
}

fn content_as_json(content: &Content) -> Result<JsonValue, serde_json::Error> {
    Ok(match content {
        Content::Text(text) => json!({"kind": "text", "text": text}),
        Content::Element(element) => {
            let children = element
                .children
                .iter()
                .map(content_as_json)
                .collect::<Result<Vec<_>, _>>()?;
            let mut object = json!({
                "kind": element.kind.as_text(),
                "category": if element.kind.is_block() { "block" } else { "inline" },
                "children": children,
            });
            if !element.attributes.is_empty() {
                object["attributes"] = properties_as_json(&element.attributes)?;
            }
            object
        }
    })
}

fn properties_as_json(properties: &Properties) -> Result<JsonValue, serde_json::Error> {
    let fields: Map<String, JsonValue> = properties
        .iter()
        .map(|(name, value)| Ok((name.clone(), value_as_json(value)?)))
        .collect::<Result<_, serde_json::Error>>()?;
    Ok(JsonValue::Object(fields))
}

fn number_as_json(number: f64) -> Result<JsonValue, serde_json::Error> {
    Number::from_f64(number)
        .map(JsonValue::Number)
        .ok_or_else(|| {
            serde_json::Error::io(io::Error::new(
                io::ErrorKind::InvalidData,
                "cannot emit a non-finite number",
            ))
        })
}

fn value_as_json(value: &Value) -> Result<JsonValue, serde_json::Error> {
    Ok(match value {
        Value::String(text) => JsonValue::String(text.clone()),
        Value::Number(number) => number_as_json(*number)?,
        Value::Boolean(boolean) => JsonValue::Bool(*boolean),
        Value::Nil => JsonValue::Null,
        Value::Object(properties) => properties_as_json(properties)?,
        Value::Length { value, unit } => json!({
            "value": number_as_json(*value)?, "unit": unit.as_text(),
            "text": format!("{value}{}", unit.as_text()),
        }),
    })
}
