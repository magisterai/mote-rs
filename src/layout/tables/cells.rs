use super::*;
use crate::model::{Content, Document, ElementKind, Value};
use crate::tables::styles as table_styles;

pub(super) fn points(value: Option<&Value>) -> Option<f64> {
    value
        .and_then(super::super::style_values::points)
        .map(|points| (points * 20.0).round() / 20.0)
}
fn flatten(properties: &Properties) -> Properties {
    let mut output: Properties = properties
        .iter()
        .filter(|(name, _)| {
            matches!(
                name.as_str(),
                "style"
                    | "width"
                    | "fill"
                    | "vertical_alignment"
                    | "alignment"
                    | "margin"
                    | "borders"
                    | "text_direction"
                    | "no_wrap"
            )
        })
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    for group in ["cell", "paragraph", "text"] {
        if let Some(properties) = property_values::object(properties, group) {
            output.extend(properties.clone());
        }
    }
    output
}
pub(super) fn defaults(document: &Document, table: &Element) -> Properties {
    let base = table_styles::resolved(document, &property_values::text(&table.attributes, "style"));
    let mut defaults = flatten(&base);
    if property_values::object(&base, "cell").is_none_or(|cell| !cell.contains_key("borders")) {
        defaults.remove("borders");
    }
    defaults
        .entry("margin".into())
        .or_insert(Value::Object(tables::default_margins()));
    if let Some(actual) = property_values::object(&table.attributes, "cell") {
        defaults = tables::cell_defaults(actual, &defaults, false);
    }
    defaults
}
pub(super) fn attributes(
    document: &Document,
    table: &Element,
    cell: &Element,
    defaults: &Properties,
    row: usize,
    count: usize,
) -> Properties {
    let mut output = tables::cell_defaults(&cell.attributes, defaults, false);
    for region in table_styles::regions(document, table, cell, row, count)
        .iter()
        .skip(1)
    {
        output = tables::cell_defaults(&output, &flatten(region), false);
    }
    output
}
pub(super) fn margins(attributes: &Properties) -> [f64; 4] {
    let fields = property_values::object(attributes, "margin")
        .map(tables::margins)
        .unwrap_or_default();
    [("top", 4.0), ("right", 6.0), ("bottom", 4.0), ("left", 6.0)]
        .map(|(name, fallback)| points(fields.get(name)).unwrap_or(fallback))
}
fn explicit_spacing(attributes: &Properties, name: &str) -> bool {
    attributes.contains_key(&format!("spacing_{name}"))
        || property_values::object(attributes, "spacing").is_some_and(|spacing| {
            spacing.contains_key(name) || spacing.contains_key(&format!("spacing_{name}"))
        })
}
/// Apply cell inheritance only to paragraph content; embedded objects retain their own geometry.
pub(super) fn content(children: &[Content], attributes: &Properties) -> Vec<Content> {
    let mut output = Vec::new();
    let mut pending = Vec::new();
    fn flush(pending: &mut Vec<Content>, output: &mut Vec<Content>, attributes: &Properties) {
        if pending
            .iter()
            .all(|node| matches!(node,Content::Text(text) if text.trim().is_empty()))
        {
            pending.clear();
            return;
        }
        let paragraph = Element {
            kind: ElementKind::Paragraph,
            attributes: Properties::new(),
            children: std::mem::take(pending),
        };
        output.extend(content(&[Content::Element(paragraph)], attributes));
    }
    for node in children {
        let Content::Element(original) = node else {
            pending.push(node.clone());
            continue;
        };
        if !original.kind.is_block() {
            pending.push(node.clone());
            continue;
        }
        flush(&mut pending, &mut output, attributes);
        let mut element = original.clone();
        match element.kind {
            ElementKind::Paragraph | ElementKind::Heading => {
                let mut inherited = attributes.clone();
                // Cell shading is painted once over the complete cell rectangle.
                inherited.remove("fill");
                inherited.remove("width");
                element.attributes = tables::cell_defaults(&element.attributes, &inherited, false);
                if element.kind == ElementKind::Paragraph {
                    for name in ["before", "after"] {
                        if !explicit_spacing(&original.attributes, name) {
                            element
                                .attributes
                                .insert(format!("spacing_{name}"), Value::Number(0.0));
                            if let Some(Value::Object(spacing)) =
                                element.attributes.get_mut("spacing")
                            {
                                spacing.remove(name);
                                spacing.remove(&format!("spacing_{name}"));
                            }
                        }
                    }
                }
            }
            ElementKind::List | ElementKind::OrderedList => {
                for node in &mut element.children {
                    if let Content::Element(item) = node {
                        item.attributes =
                            tables::cell_defaults(&item.attributes, attributes, false);
                        item.attributes.remove("fill");
                        item.attributes
                            .insert("spacing_before".into(), Value::Number(0.0));
                    }
                }
            }
            ElementKind::Blockquote => {
                let mut inherited = tables::cell_defaults(&element.attributes, attributes, false);
                inherited
                    .entry("style".into())
                    .or_insert(Value::String("BlockQuote".into()));
                let mut children = content(&element.children, &inherited);
                for child in &mut children {
                    if let Content::Element(paragraph) = child {
                        for name in ["spacing_before", "spacing_after"] {
                            paragraph.attributes.remove(name);
                        }
                    }
                }
                output.extend(children);
                continue;
            }
            _ => {}
        }
        output.push(Content::Element(element));
    }
    flush(&mut pending, &mut output, attributes);
    if output.is_empty() {
        let mut inherited = attributes.clone();
        inherited.remove("fill");
        inherited.insert("spacing_before".into(), Value::Number(0.0));
        inherited.insert("spacing_after".into(), Value::Number(0.0));
        output.push(Content::Element(Element {
            kind: ElementKind::Paragraph,
            attributes: inherited,
            children: vec![Content::Text(String::new())],
        }));
    }
    output
}
pub(super) fn block_geometry(block: &Block) -> (f64, f64, f64) {
    match block {
        Block::Paragraph(paragraph) => (
            paragraph
                .text
                .lines
                .iter()
                .map(|line| line.height)
                .sum::<f64>()
                .max(paragraph.text.style.line_height),
            paragraph.text.style.spacing_before,
            paragraph.text.style.spacing_after,
        ),
        Block::Image(image) => (image.height(), image.before, image.after),
        Block::Table(table) => (table.height(), 0.0, 0.0),
        Block::Drawing(_) => (1.0, 0.0, 0.0),
        Block::Break(_) => (0.0, 0.0, 0.0),
    }
}

pub(super) fn object_frame(
    node: &Content,
    attributes: &Properties,
    width: f64,
    styles: &LayoutStyleResolver<'_>,
) -> super::super::LineFrame {
    let Content::Element(element) = node else {
        return super::super::LineFrame { x: 0.0, width };
    };
    if !matches!(element.kind, ElementKind::Image | ElementKind::Table) {
        return super::super::LineFrame { x: 0.0, width };
    }
    let mut element = element.clone();
    element.attributes = tables::cell_defaults(&element.attributes, attributes, false);
    let style = styles.paragraph(&element);
    let available = (width - style.indent_left - style.indent_right).max(width.min(12.0));
    let origin = style.indent_left.min(width - available);
    super::super::LineFrame {
        x: origin + style.first_line,
        width: available - style.first_line,
    }
}
pub(super) fn object_inset(
    element: &Element,
    attributes: &Properties,
    styles: &LayoutStyleResolver<'_>,
) -> (f64, f64) {
    let mut element = element.clone();
    element.attributes = tables::cell_defaults(&element.attributes, attributes, false);
    let style = styles.paragraph(&element);
    (
        style.indent_left.max(0.0) + style.indent_right.max(0.0) + style.first_line.max(0.0),
        style.text.measurement.size_points * 2.0,
    )
}
pub(super) fn inherit_image_alignment(
    nodes: &mut [Content],
    document: &Document,
    table: &Element,
    row: usize,
    count: usize,
) {
    let regions = table_styles::regions(
        document,
        table,
        &Element {
            kind: ElementKind::Cell,
            attributes: Properties::new(),
            children: Vec::new(),
        },
        row,
        count,
    );
    let mut alignment = String::new();
    for (index, region) in regions.iter().enumerate() {
        let value = property_values::object(region, "paragraph")
            .map(|paragraph| property_values::text(paragraph, "alignment"))
            .filter(|value| !value.is_empty())
            .or_else(|| {
                (index > 0)
                    .then(|| property_values::text(region, "alignment"))
                    .filter(|value| !value.is_empty())
            })
            .or_else(|| {
                property_values::object(region, "cell")
                    .map(|cell| property_values::text(cell, "alignment"))
                    .filter(|value| !value.is_empty())
            });
        if let Some(value) = value {
            alignment = value;
        }
    }
    for node in nodes {
        if let Content::Element(element) = node
            && element.kind == ElementKind::Image
            && crate::media::image_alignment(&element.attributes).is_empty()
            && !alignment.is_empty()
        {
            element
                .attributes
                .insert("alignment".into(), Value::String(alignment.clone()));
        }
    }
}
