//! Editable DrawingML tables backed by shared slide table resolution.
use super::{Error, escape, package::Relationship};
use crate::{
    model::{Document, Element, Properties, Value},
    presentation::{
        Theme, color,
        tables::{SlideTable, option_enabled},
        units,
    },
    properties::{boolean, number, object, text},
};

pub(super) fn write(
    document: &Document,
    container: &Element,
    table: &Element,
    theme: &Theme,
    next_shape: &mut usize,
    relationships: &mut Vec<Relationship>,
    warnings: &mut Vec<String>,
) -> Result<String, Error> {
    let identifier = *next_shape;
    *next_shape += 1;
    let resolved = SlideTable::resolve(document, container, table).map_err(Error)?;
    if resolved.grid.column_count == 0 {
        warnings.push("PPTX omits empty tables".into());
        return Ok(String::new());
    }
    let mut name = text(&container.attributes, "name");
    if name.is_empty() {
        name = format!("Table {identifier}");
    }
    let name = escape(&name)?;
    let [left, top, width, height] = resolved.bounds;
    let mut output = format!(
        r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="{identifier}" name="{name}"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr><p:xfrm><a:off x="{left}" y="{top}"/><a:ext cx="{width}" cy="{height}"/></p:xfrm><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table"><a:tbl>"#
    );
    output.push_str(&table_properties(&resolved, table)?);
    output.push_str("<a:tblGrid>");
    for width in &resolved.widths {
        output.push_str(&format!(r#"<a:gridCol w="{width}"/>"#));
    }
    output.push_str("</a:tblGrid>");
    for (row_index, row) in resolved.grid.rows.iter().enumerate() {
        output.push_str(&format!(r#"<a:tr h="{}">"#, resolved.heights[row_index]));
        for column in 0..resolved.grid.column_count {
            let cell = row.get(column).and_then(Option::as_ref);
            let original = cell.filter(|cell| !cell.continuation && cell.origin_column == column);
            output.push_str("<a:tc");
            if let Some(cell) = original {
                if cell.row_span > 1 {
                    output.push_str(&format!(r#" rowSpan="{}""#, cell.row_span));
                }
                if cell.column_span > 1 {
                    output.push_str(&format!(r#" gridSpan="{}""#, cell.column_span));
                }
            }
            if let Some(cell) = cell {
                if cell.origin_column < column {
                    output.push_str(r#" hMerge="1""#);
                }
                if cell.continuation {
                    output.push_str(r#" vMerge="1""#);
                }
            }
            output.push_str("><a:txBody><a:bodyPr/><a:lstStyle/>");
            if let Some(cell) = original {
                let format = resolved.formats[row_index][column].as_ref().unwrap();
                let mut attributes = format.text.clone();
                attributes.insert("paragraph".into(), Value::Object(format.paragraph.clone()));
                output.push_str(&super::text::write(
                    document,
                    &cell.element.children,
                    &attributes,
                    theme,
                    relationships,
                )?);
                output.push_str("</a:txBody>");
                output.push_str(&cell_properties(&resolved, cell, &format.cell)?);
                if boolean(&format.cell, "no_wrap") {
                    warnings.push("PowerPoint always wraps table-cell text; cell.no_wrap has no presentation effect".into());
                }
            } else {
                output.push_str("<a:p/></a:txBody><a:tcPr/>");
            }
            output.push_str("</a:tc>");
        }
        output.push_str("</a:tr>");
    }
    output.push_str("</a:tbl></a:graphicData></a:graphic></p:graphicFrame>");
    Ok(output)
}
fn table_properties(resolved: &SlideTable, table: &Element) -> Result<String, Error> {
    let mut properties = resolved.style.clone();
    properties.extend(table.attributes.clone());
    let mut output = String::from("<a:tblPr");
    for (name, field) in [
        ("first_row", "firstRow"),
        ("last_row", "lastRow"),
        ("first_column", "firstCol"),
        ("last_column", "lastCol"),
        ("banded_rows", "bandRow"),
        ("banded_columns", "bandCol"),
    ] {
        if option_enabled(&properties, name) {
            output.push_str(&format!(r#" {field}="1""#));
        }
    }
    if let Some(banding) = object(&properties, "banding") {
        for (names, field) in [
            (["rows", "odd", "even"], "bandRow"),
            (["columns", "odd_column", "even_column"], "bandCol"),
        ] {
            if names.iter().any(|name| option_enabled(banding, name)) {
                output.push_str(&format!(r#" {field}="1""#));
            }
        }
    }
    output.push_str(&format!(
        "><a:tableStyleId>{}</a:tableStyleId></a:tblPr>",
        escape(&resolved.style_identifier)?
    ));
    Ok(output)
}
fn cell_properties(
    resolved: &SlideTable,
    cell: &crate::tables::GridCell,
    attributes: &Properties,
) -> Result<String, Error> {
    let mut output = String::from("<a:tcPr");
    let direction = match text(attributes, "text_direction").as_str() {
        "tb_rl" => "vert270",
        "bt_lr" => "vert",
        _ => "",
    };
    if !direction.is_empty() {
        output.push_str(&format!(r#" vert="{direction}""#));
    }
    if let Some(margin) = object(attributes, "margin") {
        for (side, field) in [
            ("left", "marL"),
            ("right", "marR"),
            ("top", "marT"),
            ("bottom", "marB"),
        ] {
            if let Some(value) = units(margin.get(side)) {
                output.push_str(&format!(r#" {field}="{value}""#));
            }
        }
    }
    let anchor = match text(attributes, "vertical_alignment").as_str() {
        "top" => "t",
        "center" | "middle" => "ctr",
        "bottom" => "b",
        _ => "",
    };
    if !anchor.is_empty() {
        output.push_str(&format!(r#" anchor="{anchor}""#));
    }
    output.push('>');
    for (side, tag) in [
        ("left", "a:lnL"),
        ("right", "a:lnR"),
        ("top", "a:lnT"),
        ("bottom", "a:lnB"),
    ] {
        if let Some(border) = resolved.border(cell, attributes, side) {
            output.push_str(&line(&border, tag));
        }
    }
    if let Some(fill) = color(attributes.get("fill")) {
        output.push_str(&super::boxes::solid_fill(&fill, None));
    }
    output.push_str("</a:tcPr>");
    Ok(output)
}
fn line(border: &Properties, tag: &str) -> String {
    if text(border, "style") == "none" {
        return format!("<{tag}><a:noFill/></{tag}>");
    }
    let width = units(border.get("width")).unwrap_or(12700);
    let fill = super::boxes::solid_fill(
        &color(border.get("color")).unwrap_or_else(|| "D9D9D9".into()),
        number(border, "opacity"),
    );
    let dash = match text(border, "style").as_str() {
        "dashed" => "<a:prstDash val=\"dash\"/>",
        "dotted" => "<a:prstDash val=\"dot\"/>",
        _ => "",
    };
    format!("<{tag} w=\"{width}\">{fill}{dash}</{tag}>")
}
