use super::{Error, escape, package::Relationship};
use crate::{
    model::{Document, Element, Properties},
    presentation::{color, units},
    properties::{number, object, text},
};

pub(super) fn write(
    element: &Element,
    document: &Document,
    hyperlink_color: &str,
    next_shape: &mut usize,
    relationships: &mut Vec<Relationship>,
) -> Result<String, Error> {
    let attributes = &element.attributes;
    if attributes.contains_key("shadow") {
        return Err(Error("PPTX box shadows are not yet supported".into()));
    }
    let position =
        object(attributes, "position").ok_or_else(|| Error("PPTX box requires position".into()))?;
    let coordinate = |name: &str, fallback| -> Result<i64, Error> {
        match position.get(name) {
            Some(value) => {
                units(Some(value)).ok_or_else(|| Error(format!("invalid box position.{name}")))
            }
            None => Ok(fallback),
        }
    };
    let horizontal = coordinate("x", 0)?;
    let vertical = coordinate("y", 0)?;
    let width = coordinate("width", 3657600)?;
    let height = coordinate("height", 1828800)?;
    let identifier = *next_shape;
    *next_shape += 1;
    let mut name = text(attributes, "name");
    if name.is_empty() {
        name = format!("Text Box {identifier}");
    }
    let name = escape(&name)?;
    let fill = if let Some(color) = color(attributes.get("fill")) {
        solid_fill(&color, number(attributes, "opacity"))
    } else {
        "<a:noFill/>".into()
    };
    let line = border(attributes)?;
    let text_attributes = crate::drawings::text_attributes(attributes);
    let body = body_properties(&text_attributes)?;
    let content = super::text::write(
        document,
        &element.children,
        &text_attributes,
        hyperlink_color,
        relationships,
    )?;
    Ok(format!(
        r#"<p:sp><p:nvSpPr><p:cNvPr id="{identifier}" name="{name}"/><p:cNvSpPr txBox="1"/><p:nvPr/></p:nvSpPr><p:spPr><a:xfrm><a:off x="{horizontal}" y="{vertical}"/><a:ext cx="{width}" cy="{height}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom>{fill}{line}</p:spPr><p:txBody>{body}<a:lstStyle/>{content}</p:txBody></p:sp>"#
    ))
}
fn solid_fill(color: &str, opacity: Option<f64>) -> String {
    let alpha = opacity
        .map(|value| {
            if value <= 1.0 {
                value * 100000.0
            } else {
                value * 1000.0
            }
        })
        .map(|value| value.round().max(0.0) as i64)
        .filter(|value| *value < 100000);
    let color = if let Some(alpha) = alpha {
        format!("<a:srgbClr val=\"{color}\"><a:alpha val=\"{alpha}\"/></a:srgbClr>")
    } else {
        format!("<a:srgbClr val=\"{color}\"/>")
    };
    format!("<a:solidFill>{color}</a:solidFill>")
}
fn border(attributes: &Properties) -> Result<String, Error> {
    let Some(border) = object(attributes, "border") else {
        return Ok("<a:ln><a:noFill/></a:ln>".into());
    };
    let style = text(border, "style");
    if style == "none" {
        return Ok("<a:ln><a:noFill/></a:ln>".into());
    }
    let width = if border.contains_key("width") {
        units(border.get("width")).ok_or_else(|| Error("invalid box border width".into()))?
    } else {
        12700
    };
    let fill = solid_fill(
        &color(border.get("color")).unwrap_or_else(|| "000000".into()),
        number(border, "opacity"),
    );
    let dash = match style.as_str() {
        "dashed" => "<a:prstDash val=\"dash\"/>",
        "dotted" => "<a:prstDash val=\"dot\"/>",
        _ => "",
    };
    Ok(format!("<a:ln w=\"{width}\">{fill}{dash}</a:ln>"))
}
fn body_properties(attributes: &Properties) -> Result<String, Error> {
    let anchor = match text(attributes, "vertical_alignment").as_str() {
        "center" | "middle" => "ctr",
        "bottom" => "b",
        _ => "t",
    };
    let fit = text(attributes, "fit");
    if matches!(fit.as_str(), "shrink" | "resize_shape") {
        return Err(Error("PPTX text fitting requires shared presentation measurement, which is not yet supported".into()));
    }
    let mut output = format!("<a:bodyPr wrap=\"square\" anchor=\"{anchor}\"");
    if fit == "clip" {
        output.push_str(" vertOverflow=\"clip\"");
    }
    let margin = object(attributes, "margin");
    for (name, attribute) in [
        ("left", "lIns"),
        ("right", "rIns"),
        ("top", "tIns"),
        ("bottom", "bIns"),
    ] {
        let value = margin.and_then(|margin| margin.get(name));
        let value = if value.is_some() {
            units(value).ok_or_else(|| Error(format!("invalid text margin.{name}")))?
        } else {
            0
        };
        output.push_str(&format!(" {attribute}=\"{value}\""));
    }
    if fit == "clip" {
        output.push_str("><a:noAutofit/></a:bodyPr>");
    } else {
        output.push_str("/>");
    }
    Ok(output)
}
