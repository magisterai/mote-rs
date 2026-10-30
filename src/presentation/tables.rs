//! Drawing-unit table geometry and formatting shared by slide outputs.
mod formatting;
use super::units;
use crate::{
    model::{Content, Document, Element, ElementKind, Properties},
    properties::{object, text},
    tables::{self, GridCell, TableGrid},
};
pub(crate) use formatting::{CellFormat, border_for_side, option_enabled};

pub(crate) struct SlideTable {
    pub grid: TableGrid,
    pub bounds: [i64; 4],
    pub widths: Vec<i64>,
    pub heights: Vec<i64>,
    pub style: Properties,
    pub borders: Properties,
    pub style_identifier: String,
    pub neutral_style: bool,
    pub formats: Vec<Vec<Option<CellFormat>>>,
}
impl SlideTable {
    pub fn resolve(
        document: &Document,
        container: &Element,
        table: &Element,
    ) -> Result<Self, String> {
        let mut bounds = super::box_bounds(&container.attributes)?;
        if let Some(width) = units(table.attributes.get("width"))
            && width > 0
            && width < bounds[2]
        {
            bounds[0] += match text(&table.attributes, "alignment").as_str() {
                "center" => (bounds[2] - width) / 2,
                "right" => bounds[2] - width,
                _ => 0,
            };
            bounds[2] = width;
        }
        let grid = tables::grid(table)?;
        let widths = column_widths(table, bounds[2], grid.column_count)?;
        let heights = if grid.rows.is_empty() {
            Vec::new()
        } else {
            let divided = bounds[3] / grid.rows.len() as i64;
            let default = if divided > 0 { divided } else { bounds[3] };
            tables::rows(table)
                .iter()
                .map(|row| units(row.attributes.get("height")).unwrap_or(default))
                .collect()
        };
        let style = formatting::style(document, table);
        let mut borders = object(&style, "borders").cloned().unwrap_or_default();
        borders.extend(
            object(&table.attributes, "borders")
                .cloned()
                .unwrap_or_default(),
        );
        let mut formats = Vec::new();
        let mut authored = !borders.is_empty();
        for row in &grid.rows {
            let mut cells = Vec::new();
            for (column, cell) in row.iter().enumerate() {
                let format = cell
                    .as_ref()
                    .filter(|cell| !cell.continuation && cell.origin_column == column)
                    .map(|cell| {
                        formatting::resolve(
                            document,
                            table,
                            &style,
                            cell,
                            grid.rows.len(),
                            grid.column_count,
                        )
                    });
                authored |= format.as_ref().is_some_and(|format| {
                    object(&format.cell, "borders").is_some_and(|borders| !borders.is_empty())
                });
                cells.push(format);
            }
            formats.push(cells);
        }
        let mut properties = style.clone();
        properties.extend(table.attributes.clone());
        let explicit = ["style_id", "powerpoint_style_id"]
            .iter()
            .map(|name| text(&properties, name))
            .find(|value| !value.is_empty())
            .unwrap_or_default();
        let neutral_style = authored && explicit.is_empty();
        let style_identifier = if !explicit.is_empty() {
            explicit
        } else if neutral_style {
            "{2D5ABB26-0587-4C30-8999-92F81FD0307C}".into()
        } else {
            "{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}".into()
        };
        Ok(Self {
            grid,
            bounds,
            widths,
            heights,
            style,
            borders,
            style_identifier,
            neutral_style,
            formats,
        })
    }
    pub fn border(
        &self,
        cell: &GridCell,
        attributes: &Properties,
        side: &str,
    ) -> Option<Properties> {
        border_for_side(
            object(attributes, "borders"),
            &self.borders,
            side,
            cell,
            self.grid.rows.len(),
            self.grid.column_count,
        )
    }
}
fn column_widths(table: &Element, total: i64, count: usize) -> Result<Vec<i64>, String> {
    for row in tables::rows(table) {
        let mut widths = Vec::new();
        for child in &row.children {
            let Content::Element(cell) = child else {
                continue;
            };
            if cell.kind != ElementKind::Cell {
                continue;
            }
            let Some(width) = units(cell.attributes.get("width")).filter(|width| *width > 0) else {
                widths.clear();
                break;
            };
            let span = tables::span(cell, "colspan")?;
            widths.extend(std::iter::repeat_n(width / span as i64, span));
        }
        if widths.len() == count {
            return Ok(widths);
        }
    }
    Ok(if count == 0 {
        Vec::new()
    } else {
        vec![total / count as i64; count]
    })
}
