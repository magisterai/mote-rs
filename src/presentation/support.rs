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
fn inspect(element: &Element, path: &str, warnings: &mut BTreeSet<String>) {
    let attributes = &element.attributes;
    let mut allowed = Vec::new();
    match element.kind {
        ElementKind::Slide => allowed.extend([
            "name",
            "layout",
            "background",
            "notes",
            "footer",
            "date",
            "slide_number",
        ]),
        ElementKind::Box | ElementKind::Shape => {
            allowed.extend([
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
                allowed.extend(["kind", "rotation", "flip_horizontal", "flip_vertical"]);
            }
            allowed.extend_from_slice(RUN);
            allowed.extend_from_slice(PARAGRAPH);
            if attributes.contains_key("fit") && super::images::child(element).is_none() {
                warnings.insert(format!(
                    "{path}.fit applies to image boxes, not text; use text.fit for text fitting"
                ));
            }
            allowed.push("fit");
            for (name, fields) in [
                ("position", &["x", "y", "width", "height"][..]),
                ("border", &["color", "width", "style", "opacity"][..]),
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
        ElementKind::Line => {
            allowed.extend(["name", "from", "to", "border", "opacity", "cap", "arrow"]);
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
        ElementKind::Row => allowed.extend(["height"]),
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
                "border",
            ]);
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
            if element.kind.is_block() {
                allowed.extend_from_slice(PARAGRAPH);
            }
            if matches!(
                element.kind,
                ElementKind::List | ElementKind::OrderedList | ElementKind::ListItem
            ) {
                allowed.extend_from_slice(LIST);
            }
            match element.kind {
                ElementKind::Heading => allowed.push("level"),
                ElementKind::Strike => allowed.push("kind"),
                ElementKind::Link => allowed.push("url"),
                ElementKind::Code => allowed.push("preserve_space"),
                _ => {}
            }
        }
    }
    unknown(attributes, &allowed, path, warnings);
    formatting(attributes, path, warnings);
    content(&element.children, path, warnings);
}

/// Native underline painting currently retains two patterns as solid strokes.
pub(crate) fn native_text_warnings(content: &[Content]) -> BTreeSet<String> {
    let mut warnings = BTreeSet::new();
    for node in content {
        if let Content::Element(element) = node {
            if element.kind == ElementKind::Underline
                && matches!(text(&element.attributes, "style").as_str(), "none" | "wave")
            {
                warnings
                    .insert("native underline styles none and wave use a solid underline".into());
            }
            warnings.extend(native_text_warnings(&element.children));
        }
    }
    warnings
}
