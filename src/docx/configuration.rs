//! Single-section page geometry. This does not perform pagination.

use super::{Error, check_attributes, values};
use crate::model::{Document, Properties, Value};

pub(super) fn section_properties(document: &Document) -> Result<String, Error> {
    check_attributes(&document.properties, &["page", "margin"], "document")?;
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
    output.push_str("/></w:sectPr>");
    Ok(output)
}

fn distance(properties: &Properties, name: &str, fallback: i64) -> Result<i64, Error> {
    properties
        .get(name)
        .map(values::twips)
        .transpose()
        .map(|value| value.unwrap_or(fallback))
}
