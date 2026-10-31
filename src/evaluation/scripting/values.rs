//! Explicit conversions retain tagged lengths/colors and distinguish arrays/maps.
use crate::model::{Content, Element, ElementKind, Properties, Unit, Value};
use mlua::{Lua, Value as LuaValue};
use std::collections::BTreeMap;

pub(super) fn to_lua(lua: &Lua, value: &Value) -> mlua::Result<LuaValue> {
    Ok(match value {
        Value::Nil => LuaValue::Nil,
        Value::Content(content) => {
            let table = lua.create_table()?;
            table.raw_set("__mote_kind", "content")?;
            table.raw_set("nodes", content_to_lua(lua, content)?)?;
            LuaValue::Table(table)
        }
        Value::Element(element) => {
            let table = lua.create_table()?;
            table.raw_set("__mote_kind", "element")?;
            table.raw_set("name", element.kind.as_text())?;
            table.raw_set(
                "category",
                if element.kind.is_table_part() {
                    "table_part"
                } else if element.kind.is_block() {
                    "block"
                } else {
                    "inline"
                },
            )?;
            table.raw_set(
                "attributes",
                to_lua(lua, &Value::Object(element.attributes.clone()))?,
            )?;
            table.raw_set("children", content_to_lua(lua, &element.children)?)?;
            LuaValue::Table(table)
        }
        Value::Boolean(value) => LuaValue::Boolean(*value),
        Value::Number(value) => LuaValue::Number(*value),
        Value::String(value) => LuaValue::String(lua.create_string(value)?),
        Value::Array(values) => {
            let table = lua.create_table()?;
            for (index, value) in values.iter().enumerate() {
                table.raw_set(index + 1, to_lua(lua, value)?)?;
            }
            LuaValue::Table(table)
        }
        Value::Object(fields) => {
            let table = lua.create_table()?;
            for (name, value) in fields {
                table.raw_set(name.as_str(), to_lua(lua, value)?)?;
            }
            LuaValue::Table(table)
        }
        Value::Length { value, unit } => {
            let table = lua.create_table()?;
            table.raw_set("__mote_kind", "length")?;
            table.raw_set("value", *value)?;
            table.raw_set("unit", unit.as_text())?;
            LuaValue::Table(table)
        }
        Value::Color(value) => {
            let table = lua.create_table()?;
            table.raw_set("__mote_kind", "color")?;
            table.raw_set("hex", value.as_str())?;
            LuaValue::Table(table)
        }
    })
}

pub(super) fn from_lua(value: LuaValue) -> mlua::Result<Value> {
    convert(value, 0)
}
fn convert(value: LuaValue, depth: usize) -> mlua::Result<Value> {
    if depth > 64 {
        return Err(mlua::Error::RuntimeError(
            "Lua value nesting limit exceeded; cyclic values cannot become document values".into(),
        ));
    }
    Ok(match value {
        LuaValue::Nil => Value::Nil,
        LuaValue::Boolean(value) => Value::Boolean(value),
        LuaValue::Integer(value) => Value::Number(value as f64),
        LuaValue::Number(value) if value.is_finite() => Value::Number(value),
        LuaValue::Number(_) => {
            return Err(mlua::Error::RuntimeError(
                "Lua number must be finite".into(),
            ));
        }
        LuaValue::String(value) => Value::String(value.to_string_lossy()),
        LuaValue::Table(table) => {
            let tag = match table.raw_get::<LuaValue>("__mote_kind")? {
                LuaValue::String(value) => Some(value.to_string_lossy()),
                _ => None,
            };
            match tag.as_deref() {
                Some("content") => {
                    return Ok(Value::Content(into_content(convert(
                        table.raw_get("nodes")?,
                        depth + 1,
                    )?)));
                }
                Some("element") => {
                    let name: String = table.raw_get("name")?;
                    let kind = ElementKind::from_name(&name).ok_or_else(|| {
                        mlua::Error::RuntimeError(format!("unsupported document element: {name}"))
                    })?;
                    let category: Option<String> = table.raw_get("category")?;
                    let expected = if kind.is_table_part() {
                        "table_part"
                    } else if kind.is_block() {
                        "block"
                    } else {
                        "inline"
                    };
                    if category
                        .as_deref()
                        .is_some_and(|category| !category.is_empty() && category != expected)
                    {
                        return Err(mlua::Error::RuntimeError(format!(
                            "invalid category for {name}"
                        )));
                    }
                    let attributes = match convert(table.raw_get("attributes")?, depth + 1)? {
                        Value::Object(fields) => fields,
                        _ => Properties::new(),
                    };
                    let children = into_content(convert(table.raw_get("children")?, depth + 1)?);
                    return Ok(Value::Element(Box::new(Element {
                        source_name: None,
                        kind,
                        attributes,
                        children,
                    })));
                }
                Some("length") => {
                    let magnitude: f64 = table.raw_get("value")?;
                    let unit: String = table.raw_get("unit")?;
                    if !magnitude.is_finite() {
                        return Err(mlua::Error::RuntimeError("length must be finite".into()));
                    }
                    return Ok(Value::Length {
                        value: magnitude,
                        unit: Unit::from_name(&unit).ok_or_else(|| {
                            mlua::Error::RuntimeError(format!("unknown length unit: {unit}"))
                        })?,
                    });
                }
                Some("color") => return Ok(Value::Color(table.raw_get("hex")?)),
                _ => {}
            }
            let mut fields = Properties::new();
            let mut indexed = BTreeMap::new();
            for pair in table.pairs::<LuaValue, LuaValue>() {
                let (key, value) = pair?;
                let number = match &key {
                    LuaValue::Integer(value) => Some(*value as f64),
                    LuaValue::Number(value) => Some(*value),
                    _ => None,
                };
                if let Some(number) = number.filter(|value| {
                    value.is_finite()
                        && value.fract() == 0.0
                        && *value >= 1.0
                        && *value < usize::MAX as f64
                }) {
                    indexed.insert(number as usize, convert(value, depth + 1)?);
                } else {
                    let name = match key {
                        LuaValue::String(value) => value.to_string_lossy(),
                        LuaValue::Number(value) => value.to_string(),
                        LuaValue::Integer(value) => value.to_string(),
                        LuaValue::Boolean(value) => value.to_string(),
                        _ => {
                            return Err(mlua::Error::RuntimeError(
                                "Lua table key must be a scalar".into(),
                            ));
                        }
                    };
                    fields.insert(name, convert(value, depth + 1)?);
                }
            }
            if fields.is_empty()
                && !indexed.is_empty()
                && indexed
                    .last_key_value()
                    .is_some_and(|(index, _)| *index == indexed.len())
            {
                Value::Array(indexed.into_values().collect())
            } else {
                fields.extend(
                    indexed
                        .into_iter()
                        .map(|(index, value)| (index.to_string(), value)),
                );
                Value::Object(fields)
            }
        }
        _ => {
            return Err(mlua::Error::RuntimeError(
                "Lua value cannot become a document value".into(),
            ));
        }
    })
}

fn content_to_lua(lua: &Lua, content: &[Content]) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;
    for (index, node) in content.iter().enumerate() {
        let value = match node {
            Content::Text(text) => Value::String(text.clone()),
            Content::Element(element) => Value::Element(Box::new(element.clone())),
        };
        table.raw_set(index + 1, to_lua(lua, &value)?)?;
    }
    Ok(table)
}

pub(super) fn into_content(value: Value) -> Vec<Content> {
    match value {
        Value::Nil => Vec::new(),
        Value::Content(content) => content,
        Value::Element(element) => vec![Content::Element(*element)],
        Value::Array(values) => values.into_iter().flat_map(into_content).collect(),
        value => vec![Content::Text(value.to_text())],
    }
}

pub(super) fn from_function_return(value: LuaValue) -> mlua::Result<Value> {
    let value = from_lua(value)?;
    Ok(if matches!(value, Value::Array(_)) {
        Value::Content(into_content(value))
    } else {
        value
    })
}
