//! Shared logical table structure, independent of package emission and page layout.
pub(crate) mod styles;
use crate::{
    model::{Content, Element, ElementKind, Properties, Unit, Value},
    properties as property_values,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub(crate) struct GridCell {
    pub element: Element,
    pub origin_row: usize,
    pub origin_column: usize,
    pub column_span: usize,
    pub row_span: usize,
    pub continuation: bool,
}
pub(crate) struct TableGrid {
    pub rows: Vec<Vec<Option<GridCell>>>,
    pub column_count: usize,
}
pub(crate) fn rows(table: &Element) -> Vec<&Element> {
    table
        .children
        .iter()
        .filter_map(|child| match child {
            Content::Element(element) if element.kind == ElementKind::Row => Some(element),
            _ => None,
        })
        .collect()
}
pub(crate) fn span(cell: &Element, name: &str) -> Result<usize, String> {
    let span = property_values::number(&cell.attributes, name)
        .unwrap_or(1.0)
        .max(1.0);
    if span > 16384.0 {
        return Err("table span exceeds the supported grid limit".into());
    }
    Ok(span as usize)
}
pub(crate) fn grid(table: &Element) -> Result<TableGrid, String> {
    let mut output: Vec<Vec<Option<GridCell>>> = Vec::new();
    let mut active: BTreeMap<usize, (usize, GridCell)> = BTreeMap::new();
    let mut column_count = 0;
    for (row_index, row) in rows(table).into_iter().enumerate() {
        let mut slots = Vec::new();
        for (column, (_, cell)) in &active {
            slots.resize(slots.len().max(column + cell.column_span), None);
            let mut continuation = cell.clone();
            continuation.continuation = true;
            for slot in &mut slots[*column..column + cell.column_span] {
                *slot = Some(continuation.clone());
            }
        }
        active.retain(|_, (remaining, _)| {
            *remaining -= 1;
            *remaining > 0
        });
        let mut column = 0;
        for child in &row.children {
            let Content::Element(cell) = child else {
                continue;
            };
            if cell.kind != ElementKind::Cell {
                continue;
            }
            while slots.get(column).is_some_and(Option::is_some) {
                column += 1;
            }
            let column_span = span(cell, "colspan")?;
            let row_span = span(cell, "rowspan")?;
            if column + column_span > 16384 {
                return Err("table exceeds the supported grid limit".into());
            }
            let placed = GridCell {
                element: cell.clone(),
                origin_row: row_index,
                origin_column: column,
                column_span,
                row_span,
                continuation: false,
            };
            slots.resize(slots.len().max(column + column_span), None);
            for slot in &mut slots[column..column + column_span] {
                *slot = Some(placed.clone());
            }
            if row_span > 1 {
                active.insert(column, (row_span - 1, placed));
            }
            column += column_span;
        }
        column_count = column_count.max(slots.len());
        output.push(slots);
    }
    Ok(TableGrid {
        rows: output,
        column_count,
    })
}
pub(crate) fn cell_defaults(
    actual: &Properties,
    defaults: &Properties,
    skip_margin: bool,
) -> Properties {
    let mut properties: Properties = defaults
        .iter()
        .filter(|(name, _)| !skip_margin || name.as_str() != "margin")
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    properties.extend(actual.clone());
    if let (Some(defaults), Some(actual)) = (
        property_values::object(defaults, "borders"),
        property_values::object(actual, "borders"),
    ) {
        let mut borders = defaults.clone();
        borders.extend(actual.clone());
        properties.insert("borders".into(), Value::Object(borders));
    }
    properties
}
pub(crate) fn margins(properties: &Properties) -> Properties {
    let mut margins = Properties::new();
    for (name, sides) in [
        ("all", &["top", "right", "bottom", "left"][..]),
        ("vertical", &["top", "bottom"][..]),
        ("horizontal", &["right", "left"][..]),
    ] {
        if let Some(value) = properties.get(name) {
            for side in sides {
                margins.insert((*side).into(), value.clone());
            }
        }
    }
    for side in ["top", "right", "bottom", "left"] {
        if let Some(value) = properties.get(side) {
            margins.insert(side.into(), value.clone());
        }
    }
    margins
}
pub(crate) fn default_margins() -> Properties {
    [("top", 4.0), ("right", 6.0), ("bottom", 4.0), ("left", 6.0)]
        .into_iter()
        .map(|(name, value)| {
            (
                name.into(),
                Value::Length {
                    value,
                    unit: Unit::Point,
                },
            )
        })
        .collect()
}
pub(crate) fn plain_text(content: &[Content]) -> String {
    content
        .iter()
        .map(|child| match child {
            Content::Text(text) => text.clone(),
            Content::Element(element) => plain_text(&element.children),
        })
        .collect()
}

/// Keep directly authored paragraph boundaries when a cell wrapper supplies its style.
pub(crate) fn paragraph_units(children: &[Content]) -> Vec<(Properties, Vec<Content>)> {
    fn flush(output: &mut Vec<(Properties, Vec<Content>)>, pending: &mut Vec<Content>) {
        while matches!(pending.first(),Some(Content::Text(text)) if text.trim().is_empty()) {
            pending.remove(0);
        }
        while matches!(pending.last(),Some(Content::Text(text)) if text.trim().is_empty()) {
            pending.pop();
        }
        if !pending.is_empty() {
            output.push((Properties::new(), std::mem::take(pending)));
        }
    }
    let mut output = Vec::new();
    let mut pending = Vec::new();
    for child in children {
        if let Content::Element(element) = child
            && element.kind == ElementKind::Paragraph
        {
            flush(&mut output, &mut pending);
            output.push((element.attributes.clone(), element.children.clone()));
        } else {
            pending.push(child.clone());
        }
    }
    flush(&mut output, &mut pending);
    if output.is_empty() {
        output.push((Properties::new(), Vec::new()));
    }
    output
}
