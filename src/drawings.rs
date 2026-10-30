//! Drawing text defaults shared by editable and native output.
use crate::{
    model::{Properties, Unit, Value},
    properties::object,
};

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
