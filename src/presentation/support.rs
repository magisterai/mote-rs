//! Retained properties must not imply an output effect.
use crate::{
    model::{Content, Document, ElementKind, Properties},
    properties::object,
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
pub(super) fn warnings(document: &Document) -> BTreeSet<String> {
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
        let path = format!("style[{name:?}]");
        unknown(&style.properties, &allowed, &path, &mut warnings);
        formatting(&style.properties, &path, &mut warnings);
    }
    content(&document.content, "content", &mut warnings);
    warnings
}
fn content(nodes: &[Content], path: &str, warnings: &mut BTreeSet<String>) {
    for (index, node) in nodes.iter().enumerate() {
        let Content::Element(element) = node else {
            continue;
        };
        let path = format!("{path}[{index}].{}", element.kind.as_text());
        let attributes = &element.attributes;
        let mut allowed = Vec::new();
        match element.kind {
            ElementKind::Slide => allowed.extend([
                "name",
                "background",
                "notes",
                "footer",
                "date",
                "slide_number",
            ]),
            ElementKind::Box => {
                allowed.extend([
                    "name",
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
                allowed.extend_from_slice(RUN);
                allowed.extend_from_slice(PARAGRAPH);
                if attributes.contains_key("fit") {
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
                    ElementKind::Link => allowed.push("url"),
                    ElementKind::Code => allowed.push("preserve_space"),
                    _ => {}
                }
            }
        }
        unknown(attributes, &allowed, &path, warnings);
        formatting(attributes, &path, warnings);
        content(&element.children, &path, warnings);
    }
}
