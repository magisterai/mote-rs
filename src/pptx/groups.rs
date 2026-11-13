//! Group shapes: the frame of a group and the order of its objects on a slide.
use super::{Error, escape};
use crate::{
    model::{Content, Element, ElementKind},
    properties::{object, text},
};

/// A step in writing the objects of a slide in their order.
pub(super) enum Item<'content> {
    Open(&'content Element),
    Close,
    Object(&'content Content),
}

/// Lists the objects of a slide with each group opened before its objects and
/// closed after them. An empty group is left out.
pub(super) fn items<'content>(content: &'content [Content], output: &mut Vec<Item<'content>>) {
    for node in content {
        match node {
            Content::Element(element) if element.kind == ElementKind::Group => {
                if !element.children.is_empty() {
                    output.push(Item::Open(element));
                    items(&element.children, output);
                    output.push(Item::Close);
                }
            }
            _ => output.push(Item::Object(node)),
        }
    }
}

/// Returns the left, top, right and bottom of the smallest rectangle about the
/// frames of an object or of the objects of a group.
fn extent(element: &Element) -> Option<[i64; 4]> {
    match element.kind {
        ElementKind::Group => element
            .children
            .iter()
            .filter_map(|child| match child {
                Content::Element(child) => extent(child),
                Content::Text(_) => None,
            })
            .reduce(|first, second| {
                [
                    first[0].min(second[0]),
                    first[1].min(second[1]),
                    first[2].max(second[2]),
                    first[3].max(second[3]),
                ]
            }),
        ElementKind::Line => {
            let endpoint = |name| {
                let fields = object(&element.attributes, name);
                ["x", "y"].map(|field| {
                    crate::presentation::units(fields.and_then(|fields| fields.get(field)))
                        .unwrap_or(0)
                })
            };
            let (start, end) = (endpoint("from"), endpoint("to"));
            Some([
                start[0].min(end[0]),
                start[1].min(end[1]),
                start[0].max(end[0]),
                start[1].max(end[1]),
            ])
        }
        _ => crate::presentation::box_bounds(&element.attributes)
            .ok()
            .map(|[left, top, width, height]| [left, top, left + width, top + height]),
    }
}

/// Opens a group shape. The child frame equals the frame of the group, so the
/// objects within keep their slide coordinates.
pub(super) fn open(element: &Element, next_shape: &mut usize) -> Result<String, Error> {
    let identifier = *next_shape;
    *next_shape += 1;
    let name = text(&element.attributes, "name");
    let name = escape(&if name.is_empty() {
        format!("Group {identifier}")
    } else {
        name
    })?;
    let [left, top, right, bottom] = extent(element).unwrap_or_default();
    let (width, height) = (right - left, bottom - top);
    Ok(format!(
        "<p:grpSp><p:nvGrpSpPr><p:cNvPr id=\"{identifier}\" name=\"{name}\"/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x=\"{left}\" y=\"{top}\"/><a:ext cx=\"{width}\" cy=\"{height}\"/><a:chOff x=\"{left}\" y=\"{top}\"/><a:chExt cx=\"{width}\" cy=\"{height}\"/></a:xfrm></p:grpSpPr>"
    ))
}
