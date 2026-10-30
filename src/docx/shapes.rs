//! Absolute Word shapes with editable text and DrawingML visual properties.
use super::{
    Error,
    document::{self, DocumentState},
    images, paragraphs, run_properties,
    styles::Styles,
    values, xml,
};
use crate::{
    drawings,
    model::{Content, Element, ElementKind, Properties},
    properties::{self, object, text},
};

pub(super) fn write(
    element: &Element,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let attributes = &element.attributes;
    let position = object(attributes, "position").cloned().unwrap_or_default();
    let coordinate = |name: &str, fallback| length(&position, name, fallback);
    let horizontal = coordinate("x", 0)?;
    let vertical = coordinate("y", 0)?;
    let width = coordinate("width", 3657600)?;
    let height = coordinate("height", 1828800)?;
    let identifier = state.resources.next_drawing();
    let mut name = text(attributes, "name");
    if name.is_empty() {
        name = format!("Shape {identifier}");
    }
    let name = xml::escape(&name)?;
    let (bookmarks, bookmark_identifiers) = super::references::start_pending(&mut state.resources)?;
    let geometry = shape_properties(attributes, width, height)?;
    let content = text_box(element, styles, state)?;
    let body_properties = body_properties(attributes)?;
    let bookmark_ends = super::references::end_pending(&bookmark_identifiers);
    Ok(format!(
        r#"<w:p><w:pPr><w:spacing w:before="0" w:after="0" w:line="20" w:lineRule="exact"/><w:rPr><w:sz w:val="2"/><w:szCs w:val="2"/></w:rPr></w:pPr>{bookmarks}<w:r><w:drawing><wp:anchor distT="0" distB="0" distL="0" distR="0" simplePos="0" relativeHeight="251658240" behindDoc="0" locked="0" layoutInCell="1" allowOverlap="1"><wp:simplePos x="0" y="0"/><wp:positionH relativeFrom="page"><wp:posOffset>{horizontal}</wp:posOffset></wp:positionH><wp:positionV relativeFrom="page"><wp:posOffset>{vertical}</wp:posOffset></wp:positionV><wp:extent cx="{width}" cy="{height}"/><wp:effectExtent l="0" t="0" r="0" b="0"/><wp:wrapNone/><wp:docPr id="{identifier}" name="{name}"/><wp:cNvGraphicFramePr/><a:graphic><a:graphicData uri="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"><wps:wsp><wps:cNvSpPr/><wps:spPr>{geometry}</wps:spPr>{content}{body_properties}</wps:wsp></a:graphicData></a:graphic></wp:anchor></w:drawing></w:r>{bookmark_ends}</w:p>"#
    ))
}

fn length(properties: &Properties, name: &str, fallback: i64) -> Result<i64, Error> {
    properties
        .get(name)
        .map(drawing_units)
        .transpose()
        .map(|value| value.unwrap_or(fallback))
}

fn drawing_units(value: &crate::model::Value) -> Result<i64, Error> {
    if matches!(value, crate::model::Value::Length { .. }) {
        images::length_in_drawing_units(value)
    } else {
        values::rounded(values::number(value)?)
    }
}

fn shape_properties(attributes: &Properties, width: i64, height: i64) -> Result<String, Error> {
    let mut transform = String::new();
    if let Some(degrees) = properties::number(attributes, "rotation") {
        transform.push_str(&format!(" rot=\"{}\"", rotation_units(degrees)?));
    }
    for (name, field) in [("flip_horizontal", "flipH"), ("flip_vertical", "flipV")] {
        if properties::boolean(attributes, name) {
            transform.push_str(&format!(" {field}=\"1\""));
        }
    }
    let kind = text(attributes, "kind");
    let preset = match kind.as_str() {
        "round_rect" => "roundRect",
        "ellipse" | "triangle" | "diamond" | "hexagon" | "chevron" | "cloud" | "parallelogram"
        | "trapezoid" => &kind,
        _ => "rect",
    };
    let fill = if let Some(value) = attributes.get("fill") {
        let color = values::color(value)?;
        let alpha = alpha(attributes, "opacity").filter(|alpha| *alpha < 100000);
        let color = if let Some(alpha) = alpha {
            format!("<a:srgbClr val=\"{color}\"><a:alpha val=\"{alpha}\"/></a:srgbClr>")
        } else {
            format!("<a:srgbClr val=\"{color}\"/>")
        };
        format!("<a:solidFill>{color}</a:solidFill>")
    } else {
        "<a:noFill/>".into()
    };
    let border = if object(attributes, "border").is_some() {
        images::border_xml(attributes, drawing_units)?
    } else {
        "<a:ln><a:noFill/></a:ln>".into()
    };
    let shadow = shadow_xml(attributes)?;
    Ok(format!(
        "<a:xfrm{transform}><a:off x=\"0\" y=\"0\"/><a:ext cx=\"{width}\" cy=\"{height}\"/></a:xfrm><a:prstGeom prst=\"{preset}\"><a:avLst/></a:prstGeom>{fill}{border}{shadow}"
    ))
}

fn alpha(properties: &Properties, name: &str) -> Option<i64> {
    properties::number(properties, name).map(|value| {
        (value * if value <= 1.0 { 100000.0 } else { 1000.0 })
            .round()
            .clamp(0.0, 100000.0) as i64
    })
}

fn shadow_xml(attributes: &Properties) -> Result<String, Error> {
    let Some(shadow) = object(attributes, "shadow") else {
        return Ok(String::new());
    };
    let color = shadow
        .get("color")
        .map(values::color)
        .transpose()?
        .unwrap_or_else(|| "000000".into());
    let opacity = alpha(shadow, "opacity").unwrap_or(35000);
    let blur = length(shadow, "blur", 50800)?;
    let distance = length(shadow, "distance", 25400)?;
    let direction = rotation_units(properties::number(shadow, "angle").unwrap_or(270.0))?;
    Ok(format!(
        "<a:effectLst><a:outerShdw blurRad=\"{blur}\" dist=\"{distance}\" dir=\"{direction}\" algn=\"tl\" rotWithShape=\"0\"><a:srgbClr val=\"{color}\"><a:alpha val=\"{opacity}\"/></a:srgbClr></a:outerShdw></a:effectLst>"
    ))
}

fn text_box(
    element: &Element,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let mut children: Vec<_> = element
        .children
        .iter()
        .filter(|child| !matches!(child,Content::Text(text) if text.trim().is_empty()))
        .cloned()
        .collect();
    if children.is_empty() {
        return Ok(String::new());
    }
    let text = drawings::text_properties(&element.attributes);
    let paragraph = drawings::paragraph_properties(&text);
    let previous = state.resources.active_text.clone();
    state.resources.active_text.properties.extend(
        text.into_iter()
            .filter(|(name, _)| run_properties::ATTRIBUTES.contains(&name.as_str())),
    );
    let inline = children
        .iter()
        .all(|child| !matches!(child,Content::Element(element) if element.kind.is_block()));
    let content = if inline {
        paragraphs::write(
            &paragraph,
            &children,
            styles,
            None,
            false,
            &mut state.resources,
        )
    } else {
        for child in &mut children {
            if let Content::Element(element) = child
                && matches!(
                    element.kind,
                    ElementKind::Paragraph | ElementKind::Heading | ElementKind::Blockquote
                )
            {
                for (name, value) in &paragraph {
                    element
                        .attributes
                        .entry(name.clone())
                        .or_insert_with(|| value.clone());
                }
            }
        }
        document::write_blocks(&children, styles, state)
    };
    state.resources.active_text = previous;
    Ok(format!(
        "<wps:txbx><w:txbxContent>{}</w:txbxContent></wps:txbx>",
        content?
    ))
}

fn body_properties(attributes: &Properties) -> Result<String, Error> {
    let text = drawings::text_properties(attributes);
    let anchor = match properties::text(&text, "vertical_alignment").as_str() {
        "center" | "middle" => "ctr",
        "bottom" => "b",
        _ => "t",
    };
    let margin = object(&text, "margin").cloned().unwrap_or_default();
    let mut output = format!("<wps:bodyPr wrap=\"square\" anchor=\"{anchor}\"");
    for (name, field) in [
        ("left", "lIns"),
        ("right", "rIns"),
        ("top", "tIns"),
        ("bottom", "bIns"),
    ] {
        output.push_str(&format!(" {field}=\"{}\"", length(&margin, name, 0)?));
    }
    match properties::text(&text, "fit").as_str() {
        "clip" => output.push_str("><a:noAutofit/></wps:bodyPr>"),
        "shrink" => output.push_str("><a:normAutofit/></wps:bodyPr>"),
        "resize_shape" => output.push_str("><a:spAutoFit/></wps:bodyPr>"),
        _ => output.push_str("/>"),
    }
    Ok(output)
}

fn rotation_units(degrees: f64) -> Result<i64, Error> {
    let value = (degrees * 60000.0).round();
    if !value.is_finite() || value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        return Err(Error::InvalidValue(
            "drawing angle exceeds the signed integer range".into(),
        ));
    }
    Ok(value as i64)
}
