//! Drawing text defaults shared by editable and native output.
pub(crate) mod drawingml;
pub(crate) mod paragraphs;
pub(crate) mod pictures;
pub(crate) mod presets;
pub(crate) mod sites;
use crate::{
    model::{Content, Element, ElementKind, Properties, Unit, Value},
    properties::object,
};

/// Returns the content with each group replaced by the objects it holds, in
/// their order, however deeply the groups are nested.
pub(crate) fn ungrouped(content: &[Content]) -> Vec<Content> {
    let mut output = Vec::new();
    for node in content {
        match node {
            Content::Element(element) if element.kind == ElementKind::Group => {
                output.extend(ungrouped(&element.children));
            }
            _ => output.push(node.clone()),
        }
    }
    output
}

/// Reports whether a document sets an object in the text: a shape or a group
/// whose position states no place. An object that states a place is fixed on
/// its page.
pub(crate) fn flows(element: &Element) -> bool {
    matches!(element.kind, ElementKind::Shape | ElementKind::Group)
        && object(&element.attributes, "position")
            .is_none_or(|position| !position.contains_key("x") && !position.contains_key("y"))
}

/// Returns the corner of a group within its container, in drawing units.
pub(crate) fn group_corner(element: &Element) -> [i64; 2] {
    let offset = object(&element.attributes, "position");
    ["x", "y"].map(|name| {
        length_in_drawing_units(offset.and_then(|offset| offset.get(name))).unwrap_or(0)
    })
}

/// Returns the width and height of the frame of a group, in drawing units:
/// those its position states, or else the reach of its objects to the right
/// of and below its corner.
pub(crate) fn group_size(element: &Element) -> [i64; 2] {
    let position = object(&element.attributes, "position");
    let reach = element
        .children
        .iter()
        .filter_map(|child| match child {
            Content::Element(child) => Some(reach(child)),
            Content::Text(_) => None,
        })
        .fold([0, 0], |first, second| {
            [first[0].max(second[0]), first[1].max(second[1])]
        });
    let stated = |name| length_in_drawing_units(position.and_then(|position| position.get(name)));
    [
        stated("width").unwrap_or(reach[0]),
        stated("height").unwrap_or(reach[1]),
    ]
}

/// Returns the right and bottom edges of an object within its container.
fn reach(element: &Element) -> [i64; 2] {
    match element.kind {
        ElementKind::Group => {
            let (corner, size) = (group_corner(element), group_size(element));
            [corner[0] + size[0], corner[1] + size[1]]
        }
        _ => crate::presentation::extent(element)
            .map_or([0, 0], |[_, _, right, bottom]| [right, bottom]),
    }
}

pub(crate) fn text_properties(attributes: &Properties) -> Properties {
    let mut merged = attributes.clone();
    merged.remove("text");
    if let Some(text) = object(attributes, "text") {
        merged.extend(text.clone());
    }
    for (name, value) in [
        ("font", Value::String("Aptos".into())),
        (
            "size",
            Value::Length {
                value: 18.0,
                unit: Unit::Point,
            },
        ),
        ("color", Value::Color("#000000".into())),
    ] {
        merged.entry(name.into()).or_insert(value);
    }
    merged
}

pub(crate) fn paragraph_properties(text: &Properties) -> Properties {
    let mut paragraph = object(text, "paragraph").cloned().unwrap_or_default();
    for name in [
        "alignment",
        "spacing",
        "spacing_before",
        "spacing_after",
        "line_spacing",
        "indent",
    ] {
        if let Some(value) = text.get(name) {
            paragraph
                .entry(name.into())
                .or_insert_with(|| value.clone());
        }
    }
    for name in ["spacing_before", "spacing_after"] {
        paragraph.entry(name.into()).or_insert(Value::Length {
            value: 0.0,
            unit: Unit::Point,
        });
    }
    paragraph
}

pub(crate) const TEXT_ATTRIBUTES: &[&str] = &[
    "alignment",
    "background",
    "bold",
    "color",
    "double_strike",
    "fit",
    "font",
    "highlight",
    "indent",
    "italic",
    "line_spacing",
    "list",
    "margin",
    "paragraph",
    "size",
    "small_caps",
    "spacing",
    "spacing_after",
    "spacing_before",
    "strike",
    "style",
    "underline",
    "vertical_alignment",
];

/// Extract text values without importing a shape's fill, border or placement.
pub(crate) fn text_attributes(attributes: &Properties) -> Properties {
    let mut text: Properties = attributes
        .iter()
        .filter(|(name, _)| TEXT_ATTRIBUTES.contains(&name.as_str()))
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    if let Some(nested) = object(attributes, "text") {
        text.extend(nested.clone());
    }
    text
}

pub(crate) fn length_in_drawing_units(value: Option<&Value>) -> Option<i64> {
    let text = value.map(Value::to_text).unwrap_or_default();
    let (number, units) = ["twip", "pt", "in", "cm", "mm"]
        .into_iter()
        .find_map(|unit| text.strip_suffix(unit).map(|number| (number, unit)))
        .unwrap_or((&text, ""));
    let number: f64 = number.trim().parse().ok()?;
    let converted = match units {
        "twip" => number * 635.0,
        "pt" => number * 12700.0,
        "in" => number * 914400.0,
        "cm" => number * 914400.0 / 2.54,
        "mm" => number * 914400.0 / 25.4,
        _ => number,
    };
    (converted.is_finite() && converted >= i64::MIN as f64 && converted < i64::MAX as f64)
        .then(|| converted.round() as i64)
}
