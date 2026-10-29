use super::super::{Error, values, xml};
use crate::{
    model::{Properties, Value},
    properties as property_values, tables,
};

pub(super) fn margins(name: &str, properties: &Properties) -> Result<String, Error> {
    let properties = tables::margins(properties);
    let mut contents = String::new();
    for side in ["top", "right", "bottom", "left"] {
        if let Some(value) = properties.get(side) {
            contents.push_str(&format!(
                "<w:{side} w:w=\"{}\" w:type=\"dxa\"/>",
                values::twips(value)?
            ));
        }
    }
    Ok(wrap(name, contents))
}
pub(super) fn wrap(name: &str, contents: String) -> String {
    if contents.is_empty() {
        contents
    } else {
        format!("<w:{name}>{contents}</w:{name}>")
    }
}
pub(super) fn borders(name: &str, properties: &Properties, table: bool) -> Result<String, Error> {
    let sides: &[&str] = if table {
        &["top", "left", "bottom", "right", "insideH", "insideV"]
    } else {
        &["top", "left", "bottom", "right"]
    };
    let mut contents = String::new();
    for side in sides {
        let inside = side.starts_with("inside");
        let border = properties
            .get(*side)
            .or_else(|| {
                properties.get(if *side == "insideH" {
                    "inside_h"
                } else if *side == "insideV" {
                    "inside_v"
                } else {
                    *side
                })
            })
            .or_else(|| {
                if table {
                    properties.get(if inside { "inside" } else { "outside" })
                } else {
                    None
                }
            })
            .or_else(|| properties.get("all"));
        if let Some(value) = border {
            contents.push_str(&border_side(side, value)?);
        }
    }
    Ok(wrap(name, contents))
}
fn border_side(side: &str, value: &Value) -> Result<String, Error> {
    let attributes = match value {
        Value::Object(properties) => properties.clone(),
        _ => Properties::from([("style".into(), value.clone())]),
    };
    let style =
        values::first_text(&attributes, &["style", "val"]).unwrap_or_else(|| "single".into());
    if matches!(style.as_str(), "none" | "nil") {
        return Ok(format!("<w:{side} w:val=\"nil\"/>"));
    }
    let mut size = property_values::number(&attributes, "size")
        .unwrap_or(4.0)
        .trunc() as i64;
    if let Some(width) = attributes.get("width") {
        size = match width {
            Value::Number(number) => (number * 8.0).round() as i64,
            _ => (values::twips(width)? as f64 * 0.4).round() as i64,
        };
    }
    let color = attributes
        .get("color")
        .map(values::color)
        .transpose()?
        .unwrap_or_else(|| "BFBFBF".into());
    let space = property_values::number(&attributes, "space")
        .unwrap_or(0.0)
        .trunc() as i64;
    Ok(format!(
        "<w:{side} w:val=\"{}\" w:sz=\"{size}\" w:space=\"{space}\" w:color=\"{color}\"/>",
        xml::escape(&style)?
    ))
}
pub(super) fn default_borders() -> String {
    let mut contents = String::new();
    for side in ["top", "left", "bottom", "right", "insideH", "insideV"] {
        let color = if side.starts_with("inside") {
            "D9D9D9"
        } else {
            "BFBFBF"
        };
        contents.push_str(&format!(
            "<w:{side} w:val=\"single\" w:sz=\"4\" w:color=\"{color}\"/>"
        ));
    }
    wrap("tblBorders", contents)
}
pub(super) fn row(attributes: &Properties) -> Result<String, Error> {
    let mut contents = String::new();
    if property_values::boolean(attributes, "cant_split") {
        contents.push_str("<w:cantSplit/>");
    }
    if let Some(height) = attributes.get("height") {
        contents.push_str(&format!("<w:trHeight w:val=\"{}\"", values::twips(height)?));
        let rule = property_values::text(attributes, "height_rule");
        let rule = if rule == "at_least" { "atLeast" } else { &rule };
        if !rule.is_empty() {
            contents.push_str(&format!(" w:hRule=\"{}\"", xml::escape(rule)?));
        }
        contents.push_str("/>");
    }
    if property_values::boolean(attributes, "repeat_header") {
        contents.push_str("<w:tblHeader/>");
    }
    Ok(wrap("trPr", contents))
}
pub(super) fn cell(attributes: &Properties) -> Result<String, Error> {
    let mut contents = String::new();
    if let Some(properties) = property_values::object(attributes, "borders") {
        contents.push_str(&borders("tcBorders", properties, false)?);
    }
    if let Some(fill) = attributes.get("fill") {
        contents.push_str(&format!(
            "<w:shd w:val=\"clear\" w:color=\"auto\" w:fill=\"{}\"/>",
            values::color(fill)?
        ));
    }
    if property_values::boolean(attributes, "no_wrap") {
        contents.push_str("<w:noWrap/>");
    }
    if let Some(properties) = property_values::object(attributes, "margin") {
        contents.push_str(&margins("tcMar", properties)?);
    }
    let direction = match property_values::text(attributes, "text_direction").as_str() {
        "ltr" => "lrTb",
        "tb_rl" => "tbRl",
        "bt_lr" => "btLr",
        _ => "",
    };
    if !direction.is_empty() {
        contents.push_str(&format!("<w:textDirection w:val=\"{direction}\"/>"));
    }
    let alignment = property_values::text(attributes, "vertical_alignment");
    if !alignment.is_empty() {
        contents.push_str(&format!(
            "<w:vAlign w:val=\"{}\"/>",
            xml::escape(&alignment)?
        ));
    }
    Ok(contents)
}
