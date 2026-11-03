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
    content_with_spacing(children, attributes, false)
}

fn content_with_spacing(
    children: &[Content],
    attributes: &Properties,
    preserve_style_spacing: bool,
) -> Vec<Content> {
    let mut output = Vec::new();
    let mut pending = Vec::new();
    fn flush(
        pending: &mut Vec<Content>,
        output: &mut Vec<Content>,
        attributes: &Properties,
        preserve_style_spacing: bool,
    ) {
        if pending
            .iter()
            .all(|node| matches!(node,Content::Text(text) if text.trim().is_empty()))
        {
            pending.clear();
            return;
        }
        let paragraph = Element {
            source_name: None,
            source_location: None,
            kind: ElementKind::Paragraph,
            attributes: Properties::new(),
            children: std::mem::take(pending),
        };
        output.extend(content_with_spacing(
            &[Content::Element(paragraph)],
            attributes,
            preserve_style_spacing,
        ));
    }
    for node in children {
        let Content::Element(original) = node else {
            pending.push(node.clone());
            continue;
        };
        if !original.is_block() {
            pending.push(node.clone());
            continue;
        }
        flush(
            &mut pending,
            &mut output,
            attributes,
            preserve_style_spacing,
        );
        let mut element = original.clone();
        if matches!(element.kind, ElementKind::Chart | ElementKind::Math) {
            element.kind = ElementKind::Paragraph;
            element.children = vec![Content::Text(super::super::fallbacks::text(original))];
        }
        match element.kind {
            ElementKind::Paragraph | ElementKind::Heading => {
                let mut inherited = attributes.clone();
                // Cell shading is painted once over the complete cell rectangle.
                inherited.remove("fill");
                inherited.remove("width");
                element.attributes = tables::cell_defaults(&element.attributes, &inherited, false);
                if original.kind == ElementKind::Paragraph && !preserve_style_spacing {
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
                inherit_list_items(&mut element, attributes);
            }
            ElementKind::Blockquote => {
                let mut inherited = tables::cell_defaults(&element.attributes, attributes, false);
                inherited
                    .entry("style".into())
                    .or_insert(Value::String("BlockQuote".into()));
                output.extend(content_with_spacing(&element.children, &inherited, true));
                continue;
            }
            ElementKind::DefinitionList => {
                for paragraph in crate::block_content::definition_paragraphs(&element, true) {
                    output.extend(content_with_spacing(
                        &[Content::Element(paragraph)],
                        attributes,
                        true,
                    ));
                }
                continue;
            }
            _ => {}
        }
        output.push(Content::Element(element));
    }
    flush(
        &mut pending,
        &mut output,
        attributes,
        preserve_style_spacing,
    );
    if output.is_empty() {
        let mut inherited = attributes.clone();
        inherited.remove("fill");
        if !preserve_style_spacing {
            inherited.insert("spacing_before".into(), Value::Number(0.0));
            inherited.insert("spacing_after".into(), Value::Number(0.0));
        }
        output.push(Content::Element(Element {
            source_name: None,
            source_location: None,
            kind: ElementKind::Paragraph,
            attributes: inherited,
            children: vec![Content::Text(String::new())],
        }));
    }
    output
}
fn inherit_list_items(list: &mut Element, attributes: &Properties) {
    let geometry = super::super::lists::ListGeometry::default().apply(&list.attributes);
    let mut inherited = attributes.clone();
    // A list's item spacing is independent of the cell's paragraph
    // defaults. Keep authored item/paragraph spacing overrides.
    inherited.remove("spacing_before");
    inherited.remove("spacing_after");
    if let Some(Value::Object(spacing)) = inherited.get_mut("spacing") {
        for name in ["before", "after", "spacing_before", "spacing_after"] {
            spacing.remove(name);
        }
    }
    for node in &mut list.children {
        if let Content::Element(item) = node {
            let item_geometry = geometry.apply(&item.attributes);
            let paragraph_defaults = tables::cell_defaults(&item.attributes, attributes, false);
            for child in &mut item.children {
                if let Content::Element(paragraph) = child
                    && paragraph.kind == ElementKind::Paragraph
                {
                    let authored =
                        tables::cell_defaults(&paragraph.attributes, &item.attributes, false);
                    paragraph.attributes =
                        tables::cell_defaults(&paragraph.attributes, &paragraph_defaults, false);
                    paragraph.attributes.remove("fill");
                    paragraph.attributes.remove("width");
                    for (name, fallback) in [("before", 0.0), ("after", item_geometry.after)] {
                        if !explicit_spacing(&authored, name) {
                            if let Some(Value::Object(spacing)) =
                                paragraph.attributes.get_mut("spacing")
                            {
                                spacing.remove(name);
                                spacing.remove(&format!("spacing_{name}"));
                            }
                            paragraph.attributes.insert(
                                format!("spacing_{name}"),
                                Value::Length {
                                    value: fallback,
                                    unit: crate::model::Unit::Point,
                                },
                            );
                        }
                    }
                }
            }
            item.attributes = tables::cell_defaults(&item.attributes, &inherited, false);
            item.attributes.remove("fill");
            if !explicit_spacing(&item.attributes, "before") {
                item.attributes
                    .insert("spacing_before".into(), Value::Number(0.0));
            }
        }
    }
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
        Block::Divider(divider) => (divider.width, divider.before, divider.after),
        Block::Image(image) => (image.height(), image.before, image.after),
        Block::Table(table) => (table.height(), 0.0, 0.0),
        Block::Chart(chart) => (chart.height, chart.before, chart.after),
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
            source_name: None,
            source_location: None,
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
