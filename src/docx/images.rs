use super::{Error, paragraphs, resources::Resources, styles::Styles, values, xml};
use crate::{
    media,
    model::{Content, Element, Properties, Unit, Value},
};

pub(super) fn write(
    element: &Element,
    styles: &Styles<'_>,
    resources: &mut Resources,
) -> Result<String, Error> {
    let attributes = &element.attributes;
    let alignment = media::image_alignment(attributes);
    let mut paragraph = Properties::new();
    if !alignment.is_empty() {
        paragraph.insert("alignment".into(), Value::String(alignment));
    }
    if let Some(spacing) = attributes.get("spacing") {
        paragraph.insert("spacing".into(), spacing.clone());
    }
    match resources.images.get(element) {
        Ok(_) => {}
        Err(_) => {
            let label = values::first_text(attributes, &["alt", "src"]).unwrap_or_default();
            return paragraphs::write(
                &paragraph,
                &[Content::Text(format!("[Image: {label}]"))],
                styles,
                None,
                false,
                resources,
            );
        }
    };
    let paragraph_properties = super::paragraph_properties::write(&paragraph)?;
    let mut output = String::from("<w:p>");
    if !paragraph_properties.is_empty() {
        output.push_str(&format!("<w:pPr>{paragraph_properties}</w:pPr>"));
    }
    let (bookmarks, identifiers) = super::references::start_pending(resources)?;
    output.push_str(&bookmarks);
    output.push_str(&write_run(element, resources)?.unwrap_or_default());
    output.push_str(&super::references::end_pending(&identifiers));
    output.push_str("</w:p>");
    Ok(output)
}

pub(super) fn write_run(
    element: &Element,
    resources: &mut Resources,
) -> Result<Option<String>, Error> {
    let attributes = &element.attributes;
    let placement = media::object(attributes, "placement");
    let Ok(asset) = resources.images.get(element).cloned() else {
        return Ok(None);
    };
    let dimension = |name: &str| {
        attributes
            .get(name)
            .or_else(|| placement.and_then(|fields| fields.get(name)))
            .map(length_in_drawing_units)
            .transpose()
    };
    let width = dimension("width")?;
    let height = dimension("height")?;
    let (width, height) = match (width, height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => (
            width,
            (width as f64 * f64::from(asset.natural_height) / f64::from(asset.natural_width))
                .round() as i64,
        ),
        (None, Some(height)) => (
            (height as f64 * f64::from(asset.natural_width) / f64::from(asset.natural_height))
                .round() as i64,
            height,
        ),
        _ => (
            i64::from(asset.natural_width) * 9525,
            i64::from(asset.natural_height) * 9525,
        ),
    };
    let (relationship, media_name, drawing_identifier) = resources.image(asset);
    let name = values::first_text(attributes, &["name", "title"])
        .unwrap_or_else(|| media_name.rsplit('/').next().unwrap().into());
    let description =
        values::first_text(attributes, &["description", "alt", "title", "src"]).unwrap_or_default();
    let hyperlink = if let Some(target) = values::first_text(attributes, &["link"]) {
        format!("<a:hlinkClick r:id=\"{}\"/>", resources.hyperlink(&target))
    } else {
        String::new()
    };
    let name = xml::escape(&name)?;
    let description = xml::escape(&description)?;
    let mut output = String::new();
    output.push_str(&format!("<w:r><w:drawing><wp:inline><wp:extent cx=\"{width}\" cy=\"{height}\"/><wp:docPr id=\"{drawing_identifier}\" name=\"{name}\" descr=\"{description}\">{hyperlink}</wp:docPr><wp:cNvGraphicFramePr><a:graphicFrameLocks noChangeAspect=\"1\"/></wp:cNvGraphicFramePr><a:graphic><a:graphicData uri=\"http://schemas.openxmlformats.org/drawingml/2006/picture\"><pic:pic><pic:nvPicPr><pic:cNvPr id=\"{drawing_identifier}\" name=\"{name}\" descr=\"{description}\">{hyperlink}</pic:cNvPr><pic:cNvPicPr/></pic:nvPicPr><pic:blipFill><a:blip r:embed=\"{relationship}\">{}</a:blip>",crate::drawings::pictures::effects(attributes)));
    output.push_str(&crop_xml(attributes));
    output.push_str("<a:stretch><a:fillRect/></a:stretch></pic:blipFill><pic:spPr><a:xfrm");
    if let Some(degrees) = media::number(attributes, "rotation") {
        output.push_str(&format!(" rot=\"{}\"", (degrees * 60000.0).round() as i64));
    }
    for (name, field) in [("flip_horizontal", "flipH"), ("flip_vertical", "flipV")] {
        if media::boolean(attributes, name) {
            output.push_str(&format!(" {field}=\"1\""));
        }
    }
    output.push_str(&format!("><a:off x=\"0\" y=\"0\"/><a:ext cx=\"{width}\" cy=\"{height}\"/></a:xfrm><a:prstGeom prst=\"rect\"><a:avLst/></a:prstGeom>{}</pic:spPr></pic:pic></a:graphicData></a:graphic></wp:inline></w:drawing></w:r>",border_xml(attributes, length_in_drawing_units)?));
    Ok(Some(output))
}
pub(super) fn length_in_drawing_units(value: &Value) -> Result<i64, Error> {
    if let Value::Length { value, unit } = value {
        values::rounded(
            value
                * match unit {
                    Unit::Point => 12700.0,
                    Unit::Inch => 914400.0,
                    Unit::Centimetre => 360000.0,
                    Unit::Millimetre => 36000.0,
                    Unit::Twip => 635.0,
                },
        )
    } else {
        values::rounded(values::number(value)? * 635.0)
    }
}
fn percent(properties: &Properties, name: &str) -> Option<i64> {
    media::number(properties, name).map(|number| (number * 1000.0).round() as i64)
}
fn crop_xml(attributes: &Properties) -> String {
    let Some(crop) = media::object(attributes, "crop") else {
        return String::new();
    };
    let mut fields = String::new();
    for (name, field) in [("left", "l"), ("top", "t"), ("right", "r"), ("bottom", "b")] {
        if let Some(value) = percent(crop, name) {
            fields.push_str(&format!(" {field}=\"{value}\""));
        }
    }
    if fields.is_empty() {
        String::new()
    } else {
        format!("<a:srcRect{fields}/>")
    }
}
pub(super) fn border_xml(
    attributes: &Properties,
    convert_length: fn(&Value) -> Result<i64, Error>,
) -> Result<String, Error> {
    let Some(border) = media::object(attributes, "border") else {
        return Ok(String::new());
    };
    let style = media::text(border, "style");
    if style == "none" {
        return Ok("<a:ln><a:noFill/></a:ln>".into());
    }
    let width = border
        .get("width")
        .map(convert_length)
        .transpose()?
        .unwrap_or(12700);
    let color = border
        .get("color")
        .map(values::color)
        .transpose()?
        .unwrap_or_else(|| "000000".into());
    let mut output = format!("<a:ln w=\"{width}\"><a:solidFill><a:srgbClr val=\"{color}\"");
    let opacity = media::number(border, "opacity")
        .map(|number| {
            if number <= 1.0 {
                number * 100000.0
            } else {
                number * 1000.0
            }
        })
        .map(|number| number.round().max(0.0) as i64);
    if let Some(opacity) = opacity.filter(|value| *value < 100000) {
        output.push_str(&format!("><a:alpha val=\"{opacity}\"/></a:srgbClr>"));
    } else {
        output.push_str("/>");
    }
    output.push_str("</a:solidFill>");
    if matches!(style.as_str(), "dashed" | "dotted") {
        output.push_str(&format!(
            "<a:prstDash val=\"{}\"/>",
            if style == "dashed" { "dash" } else { "dot" }
        ));
    }
    output.push_str("</a:ln>");
    Ok(output)
}
