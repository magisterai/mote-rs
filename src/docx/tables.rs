mod grid;
mod properties;
mod styles;
use super::{
    Error,
    document::{self, DocumentState},
    paragraphs,
    styles::Styles,
    values, xml,
};
use crate::{
    model::{Content, Element, ElementKind, Properties, Unit, Value},
    properties as property_values, tables,
};
pub(super) use styles::write as write_style;

pub(super) fn write(
    table: &Element,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    let grid = tables::grid(table).map_err(Error::InvalidValue)?;
    let attributes = &table.attributes;
    let defaults = property_values::object(attributes, "cell")
        .cloned()
        .unwrap_or_default();
    let table_style =
        tables::styles::resolved(styles.document, &property_values::text(attributes, "style"));
    let fixed = attributes
        .get("layout")
        .or_else(|| table_style.get("layout"))
        .is_some_and(|value| value.to_text() == "fixed");
    let grid_defaults = tables::cell_defaults(
        &defaults,
        &property_values::object(&table_style, "cell")
            .cloned()
            .unwrap_or_default(),
        false,
    );
    let widths = grid::widths(table, &grid, &grid_defaults, fixed)?;
    let mut output = String::from("<w:tbl><w:tblPr>");
    let style = property_values::text(attributes, "style");
    if !style.is_empty() {
        output.push_str(&format!(
            "<w:tblStyle w:val=\"{}\"/>",
            xml::escape(styles.identifier(&style)?)?
        ));
    }
    output.push_str(&self::styles::look(&table_style));
    if let Some(width) = attributes.get("width") {
        output.push_str(&format!(
            "<w:tblW w:w=\"{}\" w:type=\"dxa\"/>",
            values::twips(width)?
        ));
    } else {
        output.push_str("<w:tblW w:w=\"0\" w:type=\"auto\"/>");
    }
    let alignment = property_values::text(attributes, "alignment");
    let alignment = if matches!(alignment.as_str(), "center" | "right") {
        alignment.as_str()
    } else {
        "left"
    };
    output.push_str(&format!("<w:jc w:val=\"{alignment}\"/>"));
    for (name, tag) in [("cell_spacing", "tblCellSpacing"), ("indent", "tblInd")] {
        if let Some(value) = attributes.get(name) {
            output.push_str(&format!(
                "<w:{tag} w:w=\"{}\" w:type=\"dxa\"/>",
                values::twips(value)?
            ));
        }
    }
    let borders = property_values::object(attributes, "borders")
        .map(|borders| properties::borders("tblBorders", borders, true))
        .transpose()?
        .unwrap_or_default();
    let style_borders = property_values::object(&table_style, "borders")
        .map(|borders| properties::borders("tblBorders", borders, true))
        .transpose()?
        .unwrap_or_default();
    let style_cell_borders = property_values::object(&table_style, "cell")
        .and_then(|cell| property_values::object(cell, "borders"))
        .map(|borders| properties::borders("tcBorders", borders, false))
        .transpose()?
        .unwrap_or_default();
    output.push_str(&if borders.is_empty()
        && style_borders.is_empty()
        && style_cell_borders.is_empty()
    {
        properties::default_borders()
    } else {
        borders
    });
    if fixed {
        output.push_str("<w:tblLayout w:type=\"fixed\"/>");
    }
    let margins = property_values::object(&defaults, "margin")
        .cloned()
        .unwrap_or_else(|| {
            if style.is_empty() {
                tables::default_margins()
            } else {
                Properties::new()
            }
        });
    output.push_str(&properties::margins("tblCellMar", &margins)?);
    output.push_str("</w:tblPr>");
    if !widths.is_empty() {
        output.push_str("<w:tblGrid>");
        for width in &widths {
            output.push_str(&format!("<w:gridCol w:w=\"{width}\"/>"));
        }
        output.push_str("</w:tblGrid>");
    }
    for (row, slots) in tables::rows(table).iter().zip(&grid.rows) {
        output.push_str("<w:tr>");
        output.push_str(&properties::row(&row.attributes)?);
        for (column, cell) in slots.iter().enumerate() {
            if cell
                .as_ref()
                .is_some_and(|cell| column != cell.origin_column)
            {
                continue;
            }
            let span = cell.as_ref().map_or(1, |cell| cell.column_span);
            let grid_width: i64 = widths.iter().skip(column).take(span).sum();
            output.push_str("<w:tc><w:tcPr>");
            let attributes = cell
                .as_ref()
                .filter(|cell| !cell.continuation)
                .map(|cell| tables::cell_defaults(&cell.element.attributes, &defaults, true))
                .unwrap_or_default();
            let width = attributes
                .get("width")
                .map(values::twips)
                .transpose()?
                .or_else(|| (grid_width > 0).then_some(grid_width));
            if let Some(width) = width {
                output.push_str(&format!("<w:tcW w:w=\"{width}\" w:type=\"dxa\"/>"));
            }
            if span > 1 {
                output.push_str(&format!("<w:gridSpan w:val=\"{span}\"/>"));
            }
            if let Some(cell) = cell {
                if cell.continuation {
                    output.push_str("<w:vMerge/>");
                } else if cell.row_span > 1 {
                    output.push_str("<w:vMerge w:val=\"restart\"/>");
                }
            }
            output.push_str(&properties::cell(&attributes)?);
            output.push_str("</w:tcPr>");
            if let Some(cell) = cell.as_ref().filter(|cell| !cell.continuation) {
                let text = tables::styles::text(
                    styles.document,
                    table,
                    &cell.element,
                    cell.origin_row,
                    grid.rows.len(),
                );
                let formatting = self::styles::text_formatting(text, styles)?;
                let previous = std::mem::replace(&mut state.resources.active_text, formatting);
                state.table_cell_depth += 1;
                let result = write_cell_content(&cell.element.children, &attributes, styles, state);
                state.table_cell_depth -= 1;
                state.resources.active_text = previous;
                output.push_str(&result?);
            } else {
                output.push_str(
                    "<w:p><w:pPr><w:spacing w:before=\"0\" w:after=\"0\"/></w:pPr></w:p>",
                );
            }
            output.push_str("</w:tc>");
        }
        output.push_str("</w:tr>");
    }
    output.push_str("</w:tbl>");
    Ok(output)
}
fn paragraph_attributes(actual: &Properties, cell: &Properties) -> Properties {
    let mut attributes = actual.clone();
    if !attributes.contains_key("alignment")
        && let Some(alignment) = cell.get("alignment")
    {
        attributes.insert("alignment".into(), alignment.clone());
    }
    if let Some(direction) = cell.get("text_direction") {
        attributes.insert("text_direction".into(), direction.clone());
    }
    for name in ["before", "after"] {
        let flat = format!("spacing_{name}");
        if !attributes.contains_key(&flat)
            && property_values::object(&attributes, "spacing")
                .is_none_or(|spacing| !spacing.contains_key(name) && !spacing.contains_key(&flat))
        {
            attributes.insert(
                flat,
                Value::Length {
                    value: 0.0,
                    unit: Unit::Point,
                },
            );
        }
    }
    attributes
}
fn write_cell_content(
    children: &[Content],
    attributes: &Properties,
    styles: &Styles<'_>,
    state: &mut DocumentState,
) -> Result<String, Error> {
    if children
        .iter()
        .any(|child| matches!(child,Content::Element(element) if element.kind.is_block()))
    {
        let mut children = children.to_vec();
        for child in &mut children {
            if let Content::Element(element) = child
                && element.kind == ElementKind::Paragraph
            {
                element.attributes = paragraph_attributes(&element.attributes, attributes);
            }
        }
        document::write_blocks(&children, styles, state)
    } else {
        paragraphs::write(
            &paragraph_attributes(&Properties::new(), attributes),
            children,
            styles,
            None,
            false,
            &mut state.resources,
        )
    }
}
