use super::{Error, paragraph_properties, values};
use crate::model::{Element, Properties, Value};

pub(super) fn level(element: &Element) -> Result<u8, Error> {
    let level = element
        .attributes
        .get("level")
        .map(values::number)
        .transpose()?
        .unwrap_or(1.0);
    if level.fract() != 0.0 || level < 1.0 {
        return Err(Error::InvalidValue(
            "heading level must be a positive integer".into(),
        ));
    }
    Ok(level.min(9.0) as u8)
}

pub(super) fn attributes(element: &Element, at_flow_start: bool) -> Result<Properties, Error> {
    let mut attributes = element.attributes.clone();
    let level = level(element)?;
    attributes.remove("level");
    if let Some(marker) = attributes.remove("marker")
        && marker.to_text() != "#"
    {
        return Err(Error::Unsupported("custom heading markers".into()));
    }
    attributes.remove("numbering");
    if !attributes.contains_key("style") {
        attributes.insert("style".into(), Value::String(format!("Heading{level}")));
    }
    if at_flow_start && !has_direct_spacing_before(&attributes)? {
        attributes.insert("spacing_before".into(), Value::Number(0.0));
    }
    // Validate the direct paragraph properties before applying an output style.
    let mut direct = attributes.clone();
    direct.remove("style");
    super::paragraphs::remove_run_attributes(&mut direct);
    paragraph_properties::write(&direct)?;
    Ok(attributes)
}

fn has_direct_spacing_before(attributes: &Properties) -> Result<bool, Error> {
    if attributes.contains_key("spacing_before") {
        return Ok(true);
    }
    Ok(attributes
        .get("spacing")
        .map(values::object)
        .transpose()?
        .is_some_and(|spacing| {
            spacing.contains_key("before") || spacing.contains_key("spacing_before")
        }))
}
