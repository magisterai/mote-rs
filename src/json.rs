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
    let mut output = json!({"page":{}, "margin":{}, "columns":{}, "content": content});
    copy_nonempty_properties(&mut output["page"], &document.page)?;
    copy_nonempty_properties(&mut output["margin"], &document.margin)?;
    copy_nonempty_properties(&mut output["columns"], &document.columns.properties)?;
    if let Some(count) = document.columns.count {
        output["columns"]["count"] = json!(count);
    }
    for name in ["gap", "equal_width"] {
        copy_nonempty_field(&mut output["columns"], &document.columns.properties, name)?;
    }
    if document.columns.properties.get("separator") == Some(&Value::Boolean(true)) {
        output["columns"]["separator"] = JsonValue::Bool(true);
    }
    for name in [
        "top", "right", "bottom", "left", "header", "footer", "gutter",
    ] {
        copy_nonempty_field(&mut output["margin"], &document.margin, name)?;
    }
    if !document.properties.is_empty() {
        output["document"] = properties_as_json(&document.properties)?;
    }
    if !document.defaults.is_empty() {
        output["defaults"] = properties_as_json(&document.defaults)?;
    }
    if !document.content.is_empty() {
        output["sections"] = json!([{
            "section_properties":{}, "page_numbering":{}, "page":output["page"],
            "margin":output["margin"], "columns":output["columns"], "header_properties":{},
            "footer_properties":{}, "content":output["content"],
        }]);
    }
    if !document.fonts.is_empty() {
        let fonts: Vec<JsonValue> = document
            .fonts
            .values()
            .map(|font| {
                let mut output = json!({"name":font.name});
                copy_nonempty_properties(&mut output, &font.properties)?;
                for name in ["latin", "east_asian", "complex_script"] {
                    copy_nonempty_field(&mut output, &font.properties, name)?;
                }
                Ok(output)
            })
            .collect::<Result<_, serde_json::Error>>()?;
        output["fonts"] = JsonValue::Array(fonts);
    }
    if !document.styles.is_empty() {
        let styles: Vec<JsonValue> = document
            .styles
            .values()
            .map(|style| {
                let mut output = json!({"name":style.name, "type":style.kind.as_text()});
                copy_nonempty_properties(&mut output, &style.properties)?;
                for name in [
                    "based_on",
                    "next",
                    "link",
                    "font",
                    "paragraph",
                    "run",
                    "table",
                ] {
                    copy_nonempty_field(&mut output, &style.properties, name)?;
                }
                Ok(output)
            })
            .collect::<Result<_, serde_json::Error>>()?;
        output["styles"] = JsonValue::Array(styles);
    }
    serde_json::to_string_pretty(&output)
}

fn content_as_json(content: &Content) -> Result<JsonValue, serde_json::Error> {
    Ok(match content {
        Content::Text(text) if text.is_empty() => json!({"kind":"text"}),
        Content::Text(text) => json!({"kind": "text", "text": text}),
        Content::Element(element) => {
            let children = element
                .children
                .iter()
                .map(content_as_json)
                .collect::<Result<Vec<_>, _>>()?;
            let mut object = json!({
                "kind": element.kind.as_text(),
                "category": if element.kind.is_table_part() { "table_part" } else if element.kind.is_block() { "block" } else { "inline" },
            });
            if !children.is_empty() {
                object["children"] = JsonValue::Array(children);
            }
            if !element.attributes.is_empty() {
                object["attributes"] = properties_as_json(&element.attributes)?;
            }
            object
        }
    })
}

fn copy_nonempty_properties(
    output: &mut JsonValue,
    properties: &Properties,
) -> Result<(), serde_json::Error> {
    if !properties.is_empty() {
        output["properties"] = properties_as_json(properties)?;
    }
    Ok(())
}

fn copy_nonempty_field(
    output: &mut JsonValue,
    properties: &Properties,
    name: &str,
) -> Result<(), serde_json::Error> {
    if let Some(value) = properties.get(name) {
        let empty = match value {
            Value::String(text) => text.is_empty(),
            Value::Object(fields) => fields.is_empty(),
            _ => false,
        };
        if !empty {
            output[name] = value_as_json(value)?;
        }
    }
    Ok(())
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
        Value::Array(values) => {
            JsonValue::Array(values.iter().map(value_as_json).collect::<Result<_, _>>()?)
        }
        Value::Color(color) => JsonValue::String(color.clone()),
        Value::Length { value, unit } => json!({
            "value": number_as_json(*value)?, "unit": unit.as_text(),
            "text": format!("{value}{}", unit.as_text()),
        }),
    })
}
