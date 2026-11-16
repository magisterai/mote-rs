//! Group shapes: the frame of a group and the order of its objects on a slide.
use super::{Error, escape};
use crate::{
    model::{Content, Element, ElementKind},
    properties::{number, object, text},
};

/// A step in writing the objects of a slide in their order.
pub(super) enum Item<'content> {
    Open(&'content Element),
    Close,
    /// An object and its path within the slide.
    Object(&'content Content, String),
}

/// Lists the objects of a slide with each group opened before its objects and
/// closed after them. A group that holds no object at any depth is left out.
pub(super) fn items<'content>(
    content: &'content [Content],
    path: &str,
    output: &mut Vec<Item<'content>>,
) {
    for (index, node) in content.iter().enumerate() {
        let path = match node {
            Content::Element(element) => format!("{path}[{index}].{}", element.kind.as_text()),
            Content::Text(_) => format!("{path}[{index}]"),
        };
        match node {
            Content::Element(element) if element.kind == ElementKind::Group => {
                if !crate::drawings::ungrouped(&element.children).is_empty() {
                    output.push(Item::Open(element));
                    items(&element.children, &path, output);
                    output.push(Item::Close);
                }
            }
            _ => output.push(Item::Object(node, path)),
        }
    }
}

/// Opens a group shape. The child frame is the frame about the objects, so
/// that the objects within keep their slide coordinates; the frame of the
/// group is that frame displaced by the `offset` of the group.
pub(super) fn open(element: &Element, next_shape: &mut usize) -> Result<String, Error> {
    let identifier = *next_shape;
    *next_shape += 1;
    let name = text(&element.attributes, "name");
    let name = escape(&if name.is_empty() {
        format!("Group {identifier}")
    } else {
        name
    })?;
    let [left, top, right, bottom] = crate::presentation::extent(element).unwrap_or_default();
    let (width, height) = (right - left, bottom - top);
    let displacement = |name| {
        object(&element.attributes, "offset")
            .and_then(|offset| crate::presentation::units(offset.get(name)))
            .unwrap_or(0)
    };
    let rotation = match number(&element.attributes, "rotation") {
        Some(degrees) if degrees != 0.0 => {
            format!(
                " rot=\"{}\"",
                (degrees.rem_euclid(360.0) * 60000.0).round() as i64
            )
        }
        _ => String::new(),
    };
    Ok(format!(
        "<p:grpSp><p:nvGrpSpPr><p:cNvPr id=\"{identifier}\" name=\"{name}\"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm{rotation}><a:off x=\"{}\" y=\"{}\"/><a:ext cx=\"{width}\" cy=\"{height}\"/><a:chOff x=\"{left}\" y=\"{top}\"/><a:chExt cx=\"{width}\" cy=\"{height}\"/></a:xfrm></p:grpSpPr>",
        left + displacement("x"),
        top + displacement("y"),
    ))
}
