//! Diagnostics for retained properties with a defined, non-fatal Word fallback.
use crate::model::{Content, Document, Element, ElementKind, Properties, StyleKind, Value};

pub(super) fn collect(document: &Document) -> Vec<String> {
    let mut warnings = Vec::new();
    if document.sections.is_empty() {
        visit(&document.content, 0, false, &mut warnings);
    } else {
        for section in &document.sections {
            visit(&section.content, 0, false, &mut warnings);
            visit(&section.header.content, 0, false, &mut warnings);
            visit(&section.footer.content, 0, false, &mut warnings);
        }
    }
    for style in document.styles.values() {
        if style.kind != StyleKind::Table && style.properties.contains_key("table") {
            let location = style
                .source_location
                .as_ref()
                .map(|location| format!(" ({})", location.describe()))
                .unwrap_or_default();
            warnings.push(format!(
                "DOCX ignores style {:?}.table on a {} style; declare type=\"table\" for table formatting{location}",
                style.name, style.kind.as_text(),
            ));
        }
    }
    warnings
}

fn warn(element: &Element, property: &str, advice: &str, warnings: &mut Vec<String>) {
    let name = if element.kind == ElementKind::ListItem {
        "item"
    } else {
        element.kind.as_text()
    };
    report(
        element,
        format!("DOCX ignores {name}.{property}; {advice}"),
        warnings,
    );
}

fn report(element: &Element, message: String, warnings: &mut Vec<String>) {
    let location = element
        .source_location
        .as_ref()
        .map(|location| format!(" ({})", location.describe()))
        .unwrap_or_default();
    warnings.push(format!("{message}{location}"));
}

fn nested<'properties>(
    attributes: &'properties Properties,
    name: &str,
) -> Option<&'properties Properties> {
    match attributes.get(name) {
        Some(Value::Object(properties)) => Some(properties),
        _ => None,
    }
}

fn visit(content: &[Content], cell_depth: usize, flattened: bool, warnings: &mut Vec<String>) {
    for node in content {
        let Content::Element(element) = node else {
            continue;
        };
        let attributes = &element.attributes;
        match element.kind {
            ElementKind::Strike
            | ElementKind::Highlight
            | ElementKind::SmallCaps
            | ElementKind::Subscript
            | ElementKind::Superscript => {
                for property in ["style", "font", "size", "color"] {
                    if attributes.contains_key(property) {
                        warn(
                            element,
                            property,
                            "the semantic effect is retained; put this formatting on a surrounding span",
                            warnings,
                        );
                    }
                }
            }
            ElementKind::List | ElementKind::OrderedList | ElementKind::ListItem => {
                for (group, field, advice) in [
                    (
                        "indent",
                        "right",
                        "list indentation uses left, hanging and level",
                    ),
                    (
                        "spacing",
                        "before",
                        "list spacing uses after; place before-spacing on a surrounding paragraph",
                    ),
                ] {
                    if nested(attributes, group)
                        .is_some_and(|properties| properties.contains_key(field))
                    {
                        warn(element, &format!("{group}.{field}"), advice, warnings);
                    }
                }
                if element.kind != ElementKind::ListItem {
                    for property in ["marker", "style"] {
                        if attributes.contains_key(property) {
                            warn(
                                element,
                                property,
                                "the standard list appearance is retained; use inline spans for text formatting",
                                warnings,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        if flattened
            && matches!(
                element.kind,
                ElementKind::Paragraph | ElementKind::Blockquote | ElementKind::Heading
            )
        {
            for property in attributes.keys() {
                warn(
                    element,
                    property,
                    "nested blocks contribute inline text here; use spans for text formatting",
                    warnings,
                );
            }
        } else if matches!(
            element.kind,
            ElementKind::Paragraph | ElementKind::Blockquote
        ) && let Some(border) = nested(attributes, "border")
        {
            for property in ["width", "color"] {
                if border.contains_key(property) {
                    warn(
                        element,
                        &format!("border.{property}"),
                        "put border properties inside all or a side such as bottom",
                        warnings,
                    );
                }
            }
        }
        if flattened && element.kind == ElementKind::Divider {
            report(
                element,
                "DOCX omits a divider inside inline block content; place it outside the list item or quote to emit a rule".into(),
                warnings,
            );
        }
        let separate_story = matches!(
            element.kind,
            ElementKind::Footnote | ElementKind::Endnote | ElementKind::Comment
        );
        let child_depth = if separate_story {
            0
        } else {
            cell_depth + usize::from(element.kind == ElementKind::Cell)
        };
        let structured_cell_content = cell_depth > 0 && element.children.iter().any(|child| matches!(child, Content::Element(child) if child.kind == ElementKind::Paragraph));
        let child_flattened = !separate_story
            && (flattened
                || !structured_cell_content
                    && matches!(
                        element.kind,
                        ElementKind::Blockquote | ElementKind::ListItem
                    ));
        visit(&element.children, child_depth, child_flattened, warnings);
    }
}
