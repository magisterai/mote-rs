use super::super::{Error, values};
use crate::{
    model::{Element, Properties},
    properties as property_values,
    tables::{self, TableGrid},
};

pub(super) fn widths(
    table: &Element,
    grid: &TableGrid,
    defaults: &Properties,
    fixed: bool,
) -> Result<Vec<i64>, Error> {
    if grid.column_count == 0 {
        return Ok(Vec::new());
    }
    let mut widths = vec![0; grid.column_count];
    let mut explicit = false;
    for row in &grid.rows {
        for (column, cell) in row.iter().enumerate() {
            let Some(cell) = cell else { continue };
            if cell.continuation || column != cell.origin_column {
                continue;
            }
            let properties = tables::cell_defaults(&cell.element.attributes, defaults, true);
            if let Some(width) = properties.get("width") {
                let width = values::twips(width)?;
                if width > 0 {
                    explicit = true;
                    for value in &mut widths[column..column + cell.column_span] {
                        *value = (*value).max(width / cell.column_span as i64);
                    }
                }
            }
        }
    }
    if !explicit {
        if let Some(width) = table.attributes.get("width") {
            let width = values::twips(width)?;
            if width > 0 {
                return Ok(vec![width / grid.column_count as i64; grid.column_count]);
            }
        }
        if !fixed {
            return Ok(Vec::new());
        }
    }
    let mut natural = vec![720; grid.column_count];
    for row in &grid.rows {
        for (column, cell) in row.iter().enumerate() {
            let Some(cell) = cell else { continue };
            if cell.continuation || column != cell.origin_column {
                continue;
            }
            let text = tables::plain_text(&cell.element.children);
            let text_width = text
                .split('\n')
                .map(|line| {
                    line.chars()
                        .map(|character| match character {
                            '\r' => 0,
                            '\t' => 320,
                            ' ' | 'i' | 'l' | 'I' | '|' | '!' | '.' | ',' => 65,
                            '0'..='9' => 115,
                            character if !character.is_ascii() => 160,
                            _ => 125,
                        })
                        .sum::<i64>()
                })
                .max()
                .unwrap_or(0);
            let margin = property_values::object(&cell.element.attributes, "margin")
                .or_else(|| property_values::object(defaults, "margin"))
                .cloned()
                .unwrap_or_else(|| {
                    if property_values::text(&table.attributes, "style").is_empty() {
                        tables::default_margins()
                    } else {
                        Properties::new()
                    }
                });
            let margin = tables::margins(&margin);
            let horizontal = ["left", "right"]
                .into_iter()
                .filter_map(|side| margin.get(side))
                .map(values::twips)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .sum::<i64>();
            let width = (if text_width == 0 { 360 } else { text_width }) + horizontal + 160;
            for value in &mut natural[column..column + cell.column_span] {
                *value = (*value).max(width / cell.column_span as i64);
            }
        }
    }
    fit(&mut natural);
    for (width, natural) in widths.iter_mut().zip(natural) {
        if *width <= 0 {
            *width = natural;
        }
    }
    fit(&mut widths);
    Ok(widths)
}
fn fit(widths: &mut [i64]) {
    let total: i64 = widths.iter().sum();
    if total <= 9360 {
        return;
    }
    let mut remaining = 9360;
    let last = widths.len() - 1;
    for (index, width) in widths.iter_mut().enumerate() {
        *width = if index == last {
            remaining.max(720)
        } else {
            (*width * 9360 / total).max(720)
        };
        remaining -= *width;
    }
}
