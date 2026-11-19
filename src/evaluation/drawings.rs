//! Typed shape geometry, drawing appearance and nested text properties.
use super::{attributes, declarations, images, values::*};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Element, ElementKind, Properties, Unit, Value},
};
use std::collections::BTreeMap;

pub(super) fn normalize(
    kind: ElementKind,
    name: &str,
    value: Value,
    span: Span,
) -> Result<Value, Diagnostic> {
    match name {
        "position" if kind == ElementKind::Shape => position(value, span, "shape.position"),
        "from" | "to" if kind == ElementKind::Line => point(value, span, name),
        "name" => normalize_identifier(value, span, name).map(Value::String),
        "animate" => super::presentation::animation(value, span),
        "border" => images::normalize(name, value, span),
        "opacity" => Ok(percent(value)),
        "corner_radius" if kind == ElementKind::Shape => normalize_length(value, span, name),
        "arrow" if kind == ElementKind::Line => arrows(value, span),
        "cap" if kind == ElementKind::Line => {
            normalize_enum(value, span, name, &["flat", "round", "square"])
        }
        _ if kind == ElementKind::Line => Ok(value),
        "kind" => aliases(
            value,
            span,
            name,
            &[
                ("rect", "rectangle"),
                ("rounded", "round_rect"),
                ("oval", "ellipse"),
            ],
            &[
                "rectangle",
                "round_rect",
                "ellipse",
                "triangle",
                "diamond",
                "hexagon",
                "chevron",
                "cloud",
                "parallelogram",
                "trapezoid",
            ],
        ),
        "fill" | "color" => normalize_color(value, span, name),
        "rotation" => Ok(Value::String(
            value.to_text().trim().trim_end_matches("deg").into(),
        )),
        "flip_horizontal" | "flip_vertical" => normalize_boolean(value, span, name),
        "alignment" => normalize_enum(value, span, name, attributes::ALIGNMENTS),
        "vertical_alignment" => vertical_alignment(value, span),
        "margin" => attributes::normalize_length_map(value, span, name),
        "shadow" => shadow(value, span),
        "text" => text_properties(value, span),
        "highlight" => Ok(value),
        _ => attributes::normalize_attribute(ElementKind::Span, name, value, span),
    }
}
fn aliases(
    value: Value,
    span: Span,
    name: &str,
    aliases: &[(&str, &str)],
    allowed: &[&str],
) -> Result<Value, Diagnostic> {
    let value = if let Value::String(text) = &value {
        aliases
            .iter()
            .find(|(alias, _)| text == alias)
            .map(|(_, name)| Value::String((*name).into()))
            .unwrap_or(value)
    } else {
        value
    };
    normalize_enum(value, span, name, allowed)
}
fn vertical_alignment(value: Value, span: Span) -> Result<Value, Diagnostic> {
    aliases(
        value,
        span,
        "vertical_alignment",
        &[("middle", "center")],
        &["top", "center", "bottom"],
    )
}
fn percent(value: Value) -> Value {
    Value::String(value.to_text().trim().trim_end_matches('%').trim().into())
}
pub(super) fn position(value: Value, span: Span, owner: &str) -> Result<Value, Diagnostic> {
    let mut fields = require_object(value, span, "position")?;
    for name in ["width", "height"] {
        if !fields.contains_key(name) {
            return Err(Diagnostic::new(span, format!("{owner} requires {name}")));
        }
    }
    // A box stands on a slide alone, where an omitted coordinate is zero.
    if owner == "box.position" {
        for name in ["x", "y"] {
            fields.entry(name.into()).or_insert(Value::Length {
                value: 0.0,
                unit: Unit::Inch,
            });
        }
    }
    place(&mut fields, span)?;
    Ok(Value::Object(fields))
}
/// Normalizes the lengths of a position. A position that states neither `x`
/// nor `y` keeps neither, since a document sets such an object in the text; a
/// position that states one of them takes zero for the other.
fn place(fields: &mut Properties, span: Span) -> Result<(), Diagnostic> {
    if fields.contains_key("x") || fields.contains_key("y") {
        for name in ["x", "y"] {
            fields.entry(name.into()).or_insert(Value::Length {
                value: 0.0,
                unit: Unit::Inch,
            });
        }
    }
    for name in ["x", "y", "width", "height"] {
        if let Some(value) = fields.remove(name) {
            fields.insert(
                name.into(),
                normalize_length(value, span, &format!("position.{name}"))?,
            );
        }
    }
    Ok(())
}
/// Normalizes the position of a group, of which every member is optional.
/// The corner may lie on either side of the origin of the container, since
/// the objects of the group are measured from it.
pub(super) fn group_position(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let mut fields = require_object(value, span, "position")?;
    let corner: Properties = ["x", "y"]
        .into_iter()
        .filter_map(|name| Some((name.to_owned(), fields.remove(name)?)))
        .collect();
    place(&mut fields, span)?;
    if !corner.is_empty() {
        let Value::Object(mut corner) =
            super::presentation::displacement(Value::Object(corner), span)?
        else {
            unreachable!("a displacement is an object")
        };
        for name in ["x", "y"] {
            corner.entry(name.into()).or_insert(Value::Length {
                value: 0.0,
                unit: Unit::Inch,
            });
        }
        fields.extend(corner);
    }
    Ok(Value::Object(fields))
}
fn point(value: Value, span: Span, name: &str) -> Result<Value, Diagnostic> {
    let mut fields = require_object(value, span, name)?;
    if let Some(shape) = fields.remove("shape") {
        if fields.contains_key("x") || fields.contains_key("y") {
            return Err(Diagnostic::new(
                span,
                format!("line.{name} takes a shape or x and y, not both"),
            ));
        }
        fields.insert(
            "shape".into(),
            Value::String(normalize_identifier(shape, span, &format!("{name}.shape"))?),
        );
        match (fields.remove("side"), fields.remove("site")) {
            (Some(side), None) => {
                fields.insert(
                    "side".into(),
                    normalize_enum(
                        side,
                        span,
                        &format!("{name}.side"),
                        &["top", "bottom", "left", "right"],
                    )?,
                );
            }
            (None, Some(site)) => {
                fields.insert(
                    "site".into(),
                    normalize_integer(site, span, &format!("{name}.site"), 0)?,
                );
            }
            _ => {
                return Err(Diagnostic::new(
                    span,
                    format!("line.{name} requires a side or a site of its shape"),
                ));
            }
        }
        return Ok(Value::Object(fields));
    }
    for coordinate in ["x", "y"] {
        let value = fields
            .remove(coordinate)
            .ok_or_else(|| Diagnostic::new(span, format!("line.{name} requires {coordinate}")))?;
        fields.insert(
            coordinate.into(),
            normalize_length(value, span, &format!("{name}.{coordinate}"))?,
        );
    }
    Ok(Value::Object(fields))
}
fn shadow(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "shadow")?
        .into_iter()
        .map(|(name, value)| {
            let value = match name.as_str() {
                "color" => normalize_color(value, span, &name)?,
                "blur" | "distance" => normalize_length(value, span, &name)?,
                "opacity" => percent(value),
                "angle" => Value::String(value.to_text().trim().trim_end_matches("deg").into()),
                _ => value,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
fn text_properties(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "text")?
        .into_iter()
        .map(|(name, value)| {
            let value = match name.as_str() {
                "fit" => aliases(
                    value,
                    span,
                    "text.fit",
                    &[
                        ("none", "clip"),
                        ("autofit", "shrink"),
                        ("resize", "resize_shape"),
                    ],
                    &["clip", "shrink", "resize_shape"],
                )?,
                "margin" | "indent" => attributes::normalize_length_map(value, span, &name)?,
                "alignment" => normalize_enum(value, span, &name, attributes::ALIGNMENTS)?,
                "vertical_alignment" => vertical_alignment(value, span)?,
                "font" => Value::String(normalize_identifier(value, span, &name)?),
                "paragraph" => declarations::normalize_style_property("paragraph", value, span)?,
                "list" => text_list(value, span)?,
                "spacing" | "spacing_before" | "spacing_after" | "line_spacing" => {
                    attributes::normalize_attribute(ElementKind::Paragraph, &name, value, span)?
                }
                _ => attributes::normalize_attribute(ElementKind::Span, &name, value, span)?,
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
fn text_list(value: Value, span: Span) -> Result<Value, Diagnostic> {
    require_object(value, span, "text.list")?
        .into_iter()
        .map(|(name, value)| {
            let value = if matches!(
                name.as_str(),
                "indent" | "spacing" | "spacing_before" | "spacing_after" | "line_spacing"
            ) {
                attributes::normalize_attribute(ElementKind::Paragraph, &name, value, span)?
            } else {
                value
            };
            Ok((name, value))
        })
        .collect::<Result<Properties, Diagnostic>>()
        .map(Value::Object)
}
fn arrows(value: Value, span: Span) -> Result<Value, Diagnostic> {
    let mut fields = match value {
        Value::String(_) => Properties::from([("end".into(), value)]),
        Value::Object(fields) => fields,
        _ => {
            return Err(Diagnostic::new(
                span,
                "line arrow must be a string or property table",
            ));
        }
    };
    for name in ["start", "end", "from", "to"] {
        if let Some(value) = fields.remove(name) {
            fields.insert(
                name.into(),
                aliases(
                    value,
                    span,
                    name,
                    &[("arrow", "triangle")],
                    &["none", "triangle", "stealth", "diamond", "oval"],
                )?,
            );
        }
    }
    for name in [
        "size",
        "length",
        "start_size",
        "start_length",
        "end_size",
        "end_length",
        "from_size",
        "from_length",
        "to_size",
        "to_length",
    ] {
        if let Some(value) = fields.remove(name) {
            fields.insert(
                name.into(),
                aliases(
                    value,
                    span,
                    name,
                    &[("small", "sm"), ("medium", "med"), ("large", "lg")],
                    &["sm", "med", "lg"],
                )?,
            );
        }
    }
    for (alias, target) in [
        ("from", "start"),
        ("to", "end"),
        ("from_size", "start_size"),
        ("from_length", "start_length"),
        ("to_size", "end_size"),
        ("to_length", "end_length"),
    ] {
        if let Some(value) = fields.remove(alias) {
            fields.entry(target.into()).or_insert(value);
        }
    }
    Ok(Value::Object(fields))
}

/// Places each end of a line that names a shape at a connection site of that
/// shape. A line joins the shapes of its own slide or group, and the name must
/// belong to one shape alone of its slide, or of the outermost group of a
/// document, so that an editable connector may record the shape it joins.
pub(super) fn connect(content: &mut [Content]) -> Result<(), Diagnostic> {
    for node in content {
        let Content::Element(element) = node else {
            continue;
        };
        if matches!(element.kind, ElementKind::Slide | ElementKind::Group) {
            let mut names = BTreeMap::new();
            count_names(&element.children, &mut names);
            join(element, &names)?;
        } else {
            connect(&mut element.children)?;
        }
    }
    Ok(())
}

fn count_names(content: &[Content], names: &mut BTreeMap<String, usize>) {
    for node in content {
        let Content::Element(element) = node else {
            continue;
        };
        match element.kind {
            ElementKind::Shape => {
                let name = crate::properties::text(&element.attributes, "name");
                if !name.is_empty() {
                    *names.entry(name).or_default() += 1;
                }
            }
            ElementKind::Group => count_names(&element.children, names),
            _ => {}
        }
    }
}

fn join(container: &mut Element, names: &BTreeMap<String, usize>) -> Result<(), Diagnostic> {
    let shapes: BTreeMap<String, Properties> = container
        .children
        .iter()
        .filter_map(|node| match node {
            Content::Element(element) if element.kind == ElementKind::Shape => Some((
                crate::properties::text(&element.attributes, "name"),
                element.attributes.clone(),
            )),
            _ => None,
        })
        .collect();
    for node in &mut container.children {
        let Content::Element(element) = node else {
            continue;
        };
        if element.kind == ElementKind::Group {
            join(element, names)?;
            continue;
        }
        if element.kind != ElementKind::Line {
            continue;
        }
        let location = element.source_location.clone();
        for end in ["from", "to"] {
            let Some(Value::Object(endpoint)) = element.attributes.get_mut(end) else {
                continue;
            };
            let name = crate::properties::text(endpoint, "shape");
            if name.is_empty() {
                continue;
            }
            let failure = |message: String| {
                let location = location.as_ref();
                let diagnostic = Diagnostic::new(
                    location.map_or(Span { start: 0, end: 0 }, |location| location.span),
                    message,
                );
                match location {
                    Some(location) => diagnostic.with_source(location.source.clone()),
                    None => diagnostic,
                }
            };
            let shape = match (shapes.get(&name), names.get(&name)) {
                (Some(shape), Some(1)) => shape,
                (Some(_), _) => {
                    return Err(failure(format!(
                        "line.{end} names shape {name:?}, which more than one shape bears"
                    )));
                }
                (None, _) => {
                    return Err(failure(format!(
                        "line.{end} names shape {name:?}, which its slide or group does not hold"
                    )));
                }
            };
            let (site, point) = crate::drawings::sites::locate(shape, endpoint)
                .map_err(|message| failure(format!("line.{end}: {message}")))?;
            for (coordinate, value) in ["x", "y"].into_iter().zip(point) {
                endpoint.insert(
                    coordinate.into(),
                    Value::Length {
                        value: (value * 12700.0).round() / 12700.0,
                        unit: Unit::Point,
                    },
                );
            }
            endpoint.insert("site".into(), Value::Number(site as f64));
        }
    }
    Ok(())
}
