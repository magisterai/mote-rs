use super::{Error, escape, package::Relationship};
use crate::{
    drawings::drawingml,
    measurement::TextMeasurer,
    model::{Document, Element},
    presentation::Theme,
    properties::{object, text},
};

pub(super) fn write(
    element: &Element,
    document: &Document,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
    next_shape: &mut usize,
    relationships: &mut Vec<Relationship>,
) -> Result<String, Error> {
    let identifier = *next_shape;
    *next_shape += 1;
    let name = text(&element.attributes, "name");
    let name = escape(&if name.is_empty() {
        format!("Shape {identifier}")
    } else {
        name
    })?;
    let mut bounds = crate::presentation::box_bounds(&element.attributes).map_err(Error)?;
    let attributes = crate::drawings::text_attributes(&element.attributes);
    let fitted = crate::layout::presentation::shapes::fit(document, element, theme, measurer)
        .map_err(|error| Error(error.to_string()))?;
    bounds[3] += (fitted.height_delta * 12700.0).round() as i64;
    let geometry = drawingml::shape(&element.attributes, bounds).map_err(Error)?;
    let mut output = format!(
        "<p:sp><p:nvSpPr><p:cNvPr id=\"{identifier}\" name=\"{name}\"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr><p:spPr>{geometry}</p:spPr>"
    );
    if !element.children.is_empty() {
        let body = super::boxes::body_properties(&attributes, fitted.font_scale)?;
        let content = super::text::write(
            document,
            &element.children,
            &attributes,
            theme,
            relationships,
        )?;
        output.push_str(&format!(
            "<p:txBody>{body}<a:lstStyle/>{content}</p:txBody>"
        ));
    }
    output.push_str("</p:sp>");
    Ok(output)
}
pub(super) fn connector(element: &Element, next_shape: &mut usize) -> Result<String, Error> {
    let identifier = *next_shape;
    *next_shape += 1;
    let attributes = &element.attributes;
    let name = text(attributes, "name");
    let name = escape(&if name.is_empty() {
        format!("Line {identifier}")
    } else {
        name
    })?;
    let endpoint = |name| {
        let fields = object(attributes, name);
        ["x", "y"].map(|field| {
            crate::presentation::units(fields.and_then(|fields| fields.get(field))).unwrap_or(0)
        })
    };
    let start = endpoint("from");
    let end = endpoint("to");
    let flip = if end[0] < start[0] {
        " flipH=\"1\""
    } else {
        ""
    };
    let vertical_flip = if end[1] < start[1] {
        " flipV=\"1\""
    } else {
        ""
    };
    let horizontal = start[0].min(end[0]);
    let vertical = start[1].min(end[1]);
    let width = (end[0] - start[0]).abs();
    let height = (end[1] - start[1]).abs();
    let mut border = object(attributes, "border").cloned().unwrap_or_default();
    if let Some(opacity) = attributes.get("opacity") {
        border
            .entry("opacity".into())
            .or_insert_with(|| opacity.clone());
    }
    let cap = text(attributes, "cap");
    let cap = if cap.is_empty() {
        text(&border, "cap")
    } else {
        cap
    };
    let line = drawingml::line(&border, &cap, object(attributes, "arrow")).map_err(Error)?;
    Ok(format!(
        "<p:cxnSp><p:nvCxnSpPr><p:cNvPr id=\"{identifier}\" name=\"{name}\"/><p:cNvCxnSpPr/><p:nvPr/></p:nvCxnSpPr><p:spPr><a:xfrm{flip}{vertical_flip}><a:off x=\"{horizontal}\" y=\"{vertical}\"/><a:ext cx=\"{width}\" cy=\"{height}\"/></a:xfrm><a:prstGeom prst=\"line\"><a:avLst/></a:prstGeom>{line}</p:spPr></p:cxnSp>"
    ))
}
