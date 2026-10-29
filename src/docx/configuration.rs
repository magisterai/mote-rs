//! Single-section page geometry. This does not perform pagination.

use super::{Error, check_attributes, values};
use crate::model::{ColumnConfiguration, Document, Properties, Value};

pub(super) fn section_properties(document: &Document) -> Result<String, Error> {
    check_attributes(
        &document.properties,
        &["page", "margin", "columns"],
        "document",
    )?;
    check_attributes(
        &document.page,
        &["size", "width", "height", "orientation"],
        "page",
    )?;
    check_attributes(
        &document.margin,
        &[
            "top", "right", "bottom", "left", "header", "footer", "gutter",
        ],
        "margin",
    )?;
    let (default_width, default_height) = match document
        .page
        .get("size")
        .map(Value::to_text)
        .unwrap_or_default()
        .to_lowercase()
        .as_str()
    {
        "legal" | "us_legal" => (12240, 20160),
        "tabloid" | "ledger" => (15840, 24480),
        "a3" => (16838, 23811),
        "a4" => (11906, 16838),
        "a5" => (8391, 11906),
        _ => (12240, 15840),
    };
    let mut width = distance(&document.page, "width", default_width)?;
    let mut height = distance(&document.page, "height", default_height)?;
    let orientation = document
        .page
        .get("orientation")
        .map(Value::to_text)
        .unwrap_or_default();
    if !matches!(orientation.as_str(), "" | "portrait" | "landscape") {
        return Err(Error::InvalidValue(
            "page orientation must be portrait or landscape".into(),
        ));
    }
    let orientation_attribute = if orientation == "landscape" {
        if height > width {
            std::mem::swap(&mut width, &mut height);
        }
        " w:orient=\"landscape\""
    } else {
        ""
    };
    let mut output = format!(
        "<w:sectPr><w:pgSz w:w=\"{width}\" w:h=\"{height}\"{orientation_attribute}/><w:pgMar"
    );
    for (name, fallback) in [
        ("top", 1440),
        ("right", 1440),
        ("bottom", 1440),
        ("left", 1440),
        ("header", 720),
        ("footer", 720),
        ("gutter", 0),
    ] {
        output.push_str(&format!(
            " w:{name}=\"{}\"",
            distance(&document.margin, name, fallback)?
        ));
    }
    output.push_str("/>");
    output.push_str(&column_properties(&document.columns)?);
    output.push_str("</w:sectPr>");
    Ok(output)
}

fn column_properties(columns: &ColumnConfiguration) -> Result<String, Error> {
    check_attributes(
        &columns.properties,
        &["count", "gap", "separator", "equal_width"],
        "columns",
    )?;
    if let Some(equal_width) = columns.properties.get("equal_width")
        && !values::boolean(equal_width)?
    {
        return Err(Error::Unsupported("unequal-width columns".into()));
    }
    let Some(count) = columns.count else {
        return Ok(String::new());
    };
    if count < 1 || count > i64::from(i32::MAX) {
        return Err(Error::InvalidValue(
            "column count is outside the supported positive integer range".into(),
        ));
    }
    let mut output = format!("<w:cols w:num=\"{count}\"");
    if let Some(gap) = columns.properties.get("gap") {
        output.push_str(&format!(" w:space=\"{}\"", values::twips(gap)?));
    }
    if let Some(separator) = columns.properties.get("separator")
        && values::boolean(separator)?
    {
        output.push_str(" w:sep=\"1\"");
    }
    output.push_str("/>");
    Ok(output)
}

fn distance(properties: &Properties, name: &str, fallback: i64) -> Result<i64, Error> {
    properties
        .get(name)
        .map(values::twips)
        .transpose()
        .map(|value| value.unwrap_or(fallback))
}
