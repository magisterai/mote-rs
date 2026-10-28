//! Deterministic JSON for inspecting evaluated content, not a layout export.

use crate::model::{Content, Document, Properties, Value};
use serde_json::{Map, Number, Value as Json, json};
use std::io;

pub fn to_json(document: &Document) -> Result<String, serde_json::Error> {
    let content = document
        .content
        .iter()
        .map(content_json)
        .collect::<Result<Vec<_>, _>>()?;
    serde_json::to_string_pretty(&json!({"content": content}))
}

fn content_json(content: &Content) -> Result<Json, serde_json::Error> {
    Ok(match content {
        Content::Text(text) => json!({"kind": "text", "text": text}),
        Content::Element(element) => {
            let children = element
                .children
                .iter()
                .map(content_json)
                .collect::<Result<Vec<_>, _>>()?;
            let mut object = json!({
                "kind": element.kind.as_str(),
                "category": if element.kind.is_block() { "block" } else { "inline" },
                "children": children,
            });
            if !element.attributes.is_empty() {
                object["attributes"] = properties_json(&element.attributes)?;
            }
            object
        }
    })
}

fn properties_json(properties: &Properties) -> Result<Json, serde_json::Error> {
    let fields: Map<String, Json> = properties
        .iter()
        .map(|(name, value)| Ok((name.clone(), value_json(value)?)))
        .collect::<Result<_, serde_json::Error>>()?;
    Ok(Json::Object(fields))
}

fn finite_number(n: f64) -> Result<Json, serde_json::Error> {
    Number::from_f64(n).map(Json::Number).ok_or_else(|| {
        serde_json::Error::io(io::Error::new(
            io::ErrorKind::InvalidData,
            "cannot emit a non-finite number",
        ))
    })
}

fn value_json(value: &Value) -> Result<Json, serde_json::Error> {
    Ok(match value {
        Value::String(text) => Json::String(text.clone()),
        Value::Number(n) => finite_number(*n)?,
        Value::Boolean(b) => Json::Bool(*b),
        Value::Nil => Json::Null,
        Value::Object(properties) => properties_json(properties)?,
        Value::Length { value, unit } => json!({
            "value": finite_number(*value)?, "unit": unit.as_str(),
            "text": format!("{value}{}", unit.as_str()),
        }),
    })
}
