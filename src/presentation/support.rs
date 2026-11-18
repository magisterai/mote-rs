//! Retained properties must not imply an output effect.
use crate::{
    model::{Content, Document, Element, ElementKind, Properties},
    properties::{object, text},
};
use std::collections::BTreeSet;
const RUN: &[&str] = &[
    "style",
    "font",
    "size",
    "color",
    "highlight",
    "fill",
    "bold",
    "italic",
    "underline",
    "strike",
    "double_strike",
    "small_caps",
];
const PARAGRAPH: &[&str] = &[
    "alignment",
    "spacing",
    "spacing_before",
    "spacing_after",
    "line_spacing",
    "indent",
    "list",
];
const LIST: &[&str] = &["marker", "start", "format", "checked"];
fn unknown(properties: &Properties, allowed: &[&str], path: &str, warnings: &mut BTreeSet<String>) {
    for name in properties
        .keys()
        .filter(|name| !allowed.contains(&name.as_str()))
    {
        warnings.insert(format!(
            "{path}.{name} is retained without a presentation output effect"
        ));
    }
}
fn formatting(properties: &Properties, path: &str, warnings: &mut BTreeSet<String>) {
    for (name, allowed) in [
        (
            "spacing",
            &[
                "before",
                "after",
                "spacing_before",
                "spacing_after",
                "line",
                "line_spacing",
            ][..],
        ),
        ("indent", &["left", "right", "first_line", "hanging"][..]),
        ("margin", &["top", "right", "bottom", "left"][..]),
    ] {
        if let Some(nested) = object(properties, name) {
            unknown(nested, allowed, &format!("{path}.{name}"), warnings);
        }
    }
    for name in ["paragraph", "list", "run"] {
        if let Some(nested) = object(properties, name) {
            let mut allowed = RUN.to_vec();
            if name != "run" {
                allowed.extend_from_slice(PARAGRAPH);
            }
            if name == "list" {
                allowed.extend_from_slice(LIST);
            }
            let path = format!("{path}.{name}");
            unknown(nested, &allowed, &path, warnings);
            formatting(nested, &path, warnings);
        }
    }
}
pub(super) fn warnings(document: &Document, slides: &[Element]) -> BTreeSet<String> {
    let mut warnings = BTreeSet::new();
    unknown(
        &document.properties,
        &["title", "default_font"],
        "document",
        &mut warnings,
    );
    for (name, font) in &document.fonts {
        unknown(
            &font.properties,
            &["name", "latin", "files", "word_line_height"],
            &format!("font[{name:?}]"),
            &mut warnings,
        );
    }
    for (name, style) in &document.styles {
        let mut allowed = RUN.to_vec();
        allowed.extend_from_slice(PARAGRAPH);
        allowed.extend([
            "name",
            "type",
            "based_on",
            "next",
            "link",
            "run",
            "paragraph",
        ]);
        if text(&style.properties, "type") == "table" {
            allowed.extend([
                "table",
                "cell",
                "text",
                "borders",
                "banding",
                "first_row",
                "last_row",
                "first_column",
                "last_column",
                "style_id",
                "powerpoint_style_id",
            ]);
        }
        let path = format!("style[{name:?}]");
        unknown(&style.properties, &allowed, &path, &mut warnings);
        formatting(&style.properties, &path, &mut warnings);
    }
    if let Some(fields) = object(&document.presentation.properties, "field_text") {
        unknown(fields, FIELD_TEXT, "presentation.field_text", &mut warnings);
    }
    for (index, slide) in slides.iter().enumerate() {
        inspect(slide, &format!("content[{index}].slide"), &mut warnings);
    }
    warnings
}
fn content(nodes: &[Content], path: &str, warnings: &mut BTreeSet<String>) {
    for (index, node) in nodes.iter().enumerate() {
        if let Content::Element(element) = node {
            inspect(
                element,
                &format!("{path}[{index}].{}", element.kind.as_text()),
                warnings,
            );
        }
    }
}
const FIELD_TEXT: &[&str] = &["size", "color", "font", "bold", "italic"];

/// Reports the arguments that an effect of an object does not take.
fn effects(attributes: &Properties, path: &str, warnings: &mut BTreeSet<String>) {
    let Some(crate::model::Value::Array(entries)) = attributes.get("animate") else {
        return;
    };
    for (index, entry) in entries.iter().enumerate() {
        let crate::model::Value::Object(fields) = entry else {
            continue;
        };
        let mut allowed = vec!["effect", "start", "easing"];
        allowed.extend(match text(fields, "effect").as_str() {
            "appear" | "disappear" => &[][..],
            "move" | "rotate" => &["duration", "by"][..],
            "scale" => &["duration", "to"][..],
            "wipe_in" | "wipe_out" => &["duration", "from"][..],
            _ => &["duration"][..],
        });
        let path = format!("{path}.animate[{index}]");
        unknown(fields, &allowed, &path, warnings);
        if let Some(by) = object(fields, "by") {
            unknown(by, &["x", "y"], &format!("{path}.by"), warnings);
        }
    }
}

fn inspect(element: &Element, path: &str, warnings: &mut BTreeSet<String>) {
    let attributes = &element.attributes;
    let mut allowed = Vec::new();
    effects(attributes, path, warnings);
    for (name, fields) in [
        ("position", &["x", "y", "width", "height"][..]),
        ("field_text", FIELD_TEXT),
    ] {
        if matches!(element.kind, ElementKind::Group | ElementKind::Slide)
            && let Some(nested) = object(attributes, name)
        {
            unknown(nested, fields, &format!("{path}.{name}"), warnings);
        }
    }
    match element.kind {
        ElementKind::Checkbox | ElementKind::Field | ElementKind::Comment => {
            allowed.extend([
                "checked", "kind", "name", "title", "code", "id", "author", "initials", "date",
            ]);
            warnings.insert(format!("{path} retains child text without an editable presentation control, field or comment"));
        }
        ElementKind::Math => {
            allowed.extend(["source", "format", "display", "size"]);
            match crate::math::convert(
                element,
                crate::math::Options {
                    presentation: true,
                    ..Default::default()
                },
            ) {
                Ok(equation) => warnings.extend(equation.warnings),
                Err(error) => {
                    warnings.insert(error.to_string());
                }
            }
        }

        ElementKind::Slide => allowed.extend([
            "name",
            "layout",
            "background",
            "notes",
            "footer",
            "date",
            "slide_number",
            "duration",
            "advance_after",
            "field_text",
        ]),
        ElementKind::Box | ElementKind::Shape => {
            allowed.extend([
                "animate",
                "name",
                "slot",
                "position",
                "fill",
                "border",
                "opacity",
                "shadow",
                "text",
                "paragraph",
                "list",
                "margin",
                "vertical_alignment",
            ]);
            if element.kind == ElementKind::Shape {
                allowed.extend([
                    "kind",
                    "corner_radius",
                    "rotation",
                    "flip_horizontal",
                    "flip_vertical",
                ]);
                if attributes.contains_key("corner_radius")
                    && text(attributes, "kind") != "round_rect"
                {
                    warnings.insert(format!(
                        "{path}.corner_radius has an effect only on a round_rect shape"
                    ));
                }
            }
            allowed.extend_from_slice(RUN);
            allowed.extend_from_slice(PARAGRAPH);
            if attributes.contains_key("fit")
                && !element.children.iter().any(|child| matches!(child, Content::Element(image) if image.kind == ElementKind::Image))
            {
                warnings.insert(format!(
                    "{path}.fit applies to image boxes, not text; use text.fit for text fitting"
                ));
            }
            allowed.push("fit");
            for (name, fields) in [
                ("position", &["x", "y", "width", "height"][..]),
                ("border", &["color", "width", "style", "opacity"][..]),
                (
                    "shadow",
                    &["color", "blur", "distance", "angle", "opacity"][..],
                ),
            ] {
                if let Some(nested) = object(attributes, name) {
                    unknown(nested, fields, &format!("{path}.{name}"), warnings);
                }
            }
            if let Some(nested) = object(attributes, "text") {
                let mut fields = RUN.to_vec();
                fields.extend_from_slice(PARAGRAPH);
                fields.extend(["fit", "margin", "vertical_alignment", "paragraph", "list"]);
                unknown(nested, &fields, &format!("{path}.text"), warnings);
                formatting(nested, &format!("{path}.text"), warnings);
            }
        }
        ElementKind::Group => {
            allowed.extend(["name", "animate", "rotation", "position", "alignment"]);
            if crate::drawings::ungrouped(&element.children).is_empty() {
                warnings.insert(format!("{path} is empty and is omitted"));
            }
        }
        ElementKind::Line => {
            allowed.extend([
                "animate", "name", "from", "to", "border", "opacity", "cap", "arrow",
            ]);
            for name in ["from", "to"] {
                if let Some(fields) = object(attributes, name) {
                    unknown(fields, &["x", "y"], &format!("{path}.{name}"), warnings);
                }
            }
        }
        ElementKind::Chart => allowed.extend_from_slice(crate::charts::support::warnings(
            attributes, path, false, false, warnings,
        )),
        ElementKind::Table => allowed.extend([
            "style",
            "width",
            "alignment",
            "cell",
            "borders",
            "style_id",
            "powerpoint_style_id",
        ]),
        // A slide has no second page upon which a header row could repeat.
        ElementKind::Row => allowed.extend(["height", "repeat_header"]),
        ElementKind::Cell => allowed.extend([
            "colspan",
            "rowspan",
            "style",
            "width",
            "fill",
            "alignment",
            "vertical_alignment",
            "margin",
            "borders",
            "text_direction",
            "no_wrap",
        ]),
        ElementKind::Image => {
            allowed.extend([
                "src",
                "name",
                "description",
                "alt",
                "title",
                "link",
                "crop",
                "rotation",
                "flip_horizontal",
                "flip_vertical",
                "effects",
                "opacity",
                "border",
            ]);
            for name in ["width", "height"] {
                if attributes.contains_key(name) {
                    allowed.push(name);
                    warnings.insert(format!(
                        "{path}.{name} has no effect on a slide: the box sets the size of its image"
                    ));
                }
            }
            for (name, fields) in [
                ("crop", &["left", "top", "right", "bottom"][..]),
                (
                    "effects",
                    &["brightness", "contrast", "grayscale", "transparency"][..],
                ),
                ("border", &["color", "width", "style", "opacity"][..]),
            ] {
                if let Some(nested) = object(attributes, name) {
                    unknown(nested, fields, &format!("{path}.{name}"), warnings);
                }
            }
        }
        _ => {
            allowed.extend_from_slice(RUN);
            if element.is_block() {
                allowed.extend_from_slice(PARAGRAPH);
            }
            if matches!(
                element.kind,
                ElementKind::List | ElementKind::OrderedList | ElementKind::ListItem
            ) {
                allowed.extend_from_slice(LIST);
            }
            match element.kind {
                ElementKind::Heading => {
                    allowed.push("level");
                    // A Markdown heading records its source syntax, which has no
                    // output to affect; only an authored custom marker is unsupported.
                    if matches!(text(attributes, "marker").as_str(), "#" | "setext") {
                        allowed.push("marker");
                    }
                }
                ElementKind::Strike => allowed.push("kind"),
                ElementKind::Link => allowed.extend(["url", "title", "tooltip"]),
                ElementKind::Code | ElementKind::Raw => allowed.push("preserve_space"),
                _ => {}
            }
        }
    }
    unknown(attributes, &allowed, path, warnings);
    formatting(attributes, path, warnings);
    content(&element.children, path, warnings);
}
