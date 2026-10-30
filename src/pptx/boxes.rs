use super::{Error, escape, package::Relationship};
use crate::{
    model::{Document, Element, Properties},
    presentation::{color, units},
    properties::{number, object, text},
};

pub(super) fn write(
    element: &Element,
    document: &Document,
    theme: &crate::presentation::Theme,
    measurer: &mut crate::measurement::TextMeasurer<'_>,
    warnings: &mut Vec<String>,
    next_shape: &mut usize,
    relationships: &mut Vec<Relationship>,
) -> Result<String, Error> {
    let attributes = &element.attributes;
    if attributes.contains_key("shadow") {
        return Err(Error("PPTX box shadows are not yet supported".into()));
    }
    let [horizontal, vertical, width, mut height] =
        crate::presentation::images::box_bounds(attributes).map_err(Error)?;
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
    let mut font_scale = 1.0;
    if matches!(
        text(&text_attributes, "fit").as_str(),
        "shrink" | "resize_shape"
    ) {
        let prepared = crate::layout::presentation::prepare_box(document, element, theme, measurer)
            .map_err(|error| Error(error.to_string()))?;
        font_scale = prepared.fitted.font_scale;
        let delta = units(Some(&crate::model::Value::Length {
            value: prepared.fitted.height_delta,
            unit: crate::model::Unit::Point,
        }))
        .ok_or_else(|| Error("fitted box height exceeds the drawing-unit range".into()))?;
        height = height
            .checked_add(delta)
            .ok_or_else(|| Error("fitted box height exceeds the drawing-unit range".into()))?;
        if font_scale < 0.999995 || prepared.fitted.height_delta > 0.0 {
            for line in prepared.fitted.page.text_lines() {
                for fragment in &line.fragments {
                    if let Some(reason) = &fragment.font.substitution {
                        warnings.push(format!(
                            "PPTX text fitting substituted font {:?}: {reason}",
                            fragment.style.measurement.font
                        ));
                    }
                    for character in &fragment.missing_codepoints {
                        warnings.push(format!(
                            "PPTX text fitting replaces missing glyph U+{:04X} with '?'",
                            *character as u32
                        ));
                    }
                }
            }
        }
    }
    let body = body_properties(&text_attributes, font_scale)?;
    let content = super::text::write(
        document,
        &element.children,
        &text_attributes,
        theme,
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
pub(super) fn border(attributes: &Properties) -> Result<String, Error> {
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
fn body_properties(attributes: &Properties, font_scale: f64) -> Result<String, Error> {
    let anchor = match text(attributes, "vertical_alignment").as_str() {
        "center" | "middle" => "ctr",
        "bottom" => "b",
        _ => "t",
    };
    let fit = text(attributes, "fit");
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
    let fitting = match fit.as_str() {
        "clip" => "<a:noAutofit/>".into(),
        "shrink" if font_scale > 0.0 && font_scale < 0.999995 => format!(
            "<a:normAutofit fontScale=\"{}\"/>",
            (font_scale * 100000.0).round() as i64
        ),
        "shrink" => "<a:normAutofit/>".into(),
        "resize_shape" => "<a:spAutoFit/>".into(),
        _ => String::new(),
    };
    if fitting.is_empty() {
        output.push_str("/>");
    } else {
        output.push_str(&format!(">{fitting}</a:bodyPr>"));
    }
    Ok(output)
}
