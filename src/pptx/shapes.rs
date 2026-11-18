use super::{Error, escape, package::Relationship};
use crate::{
    drawings::drawingml,
    measurement::TextMeasurer,
    model::{Document, Element},
    presentation::Theme,
    properties::text,
};

pub(super) struct WrittenShape {
    pub markup: String,
    pub text_is_cut_off: bool,
}

pub(super) fn write(
    element: &Element,
    document: &Document,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
    next_shape: &mut usize,
    relationships: &mut Vec<Relationship>,
) -> Result<WrittenShape, Error> {
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
    Ok(WrittenShape {
        markup: output,
        text_is_cut_off: fitted.is_cut_off(),
    })
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
    let properties = drawingml::connector(attributes).map_err(Error)?;
    Ok(format!(
        "<p:cxnSp><p:nvCxnSpPr><p:cNvPr id=\"{identifier}\" name=\"{name}\"/><p:cNvCxnSpPr/><p:nvPr/></p:nvCxnSpPr><p:spPr>{properties}</p:spPr></p:cxnSp>"
    ))
}
