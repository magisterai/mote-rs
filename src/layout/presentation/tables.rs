//! Fixed slide grids share formatting with editable DrawingML tables.
mod borders;
use crate::{
    layout::{HighlightRectangle, LayoutError, PageOperation, TextPage, drawing_text},
    measurement::TextMeasurer,
    model::{Document, Element, Properties},
    presentation::{Theme, color, tables::SlideTable, text::TextDefaults, units},
    properties::{boolean, object, text},
};

pub(super) fn place(
    document: &Document,
    container: &Element,
    element: &Element,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
    page: &mut TextPage,
    warnings: &mut Vec<String>,
) -> Result<(), LayoutError> {
    let table =
        SlideTable::resolve(document, container, element).map_err(LayoutError::Unsupported)?;
    if table.bounds[2] <= 0 || table.bounds[3] <= 0 {
        return Ok(());
    }
    let columns = offsets(&table.widths);
    let rows = offsets(&table.heights);
    let origin = [
        table.bounds[0] as f64 / 12700.0,
        table.bounds[1] as f64 / 12700.0,
    ];
    let default_fill = tinted(&theme.colors[4]);
    for (row_index, row) in table.grid.rows.iter().enumerate() {
        for (column, cell) in row.iter().enumerate() {
            let Some(cell) = cell
                .as_ref()
                .filter(|cell| !cell.continuation && cell.origin_column == column)
            else {
                continue;
            };
            let format = table.formats[row_index][column]
                .as_ref()
                .expect("origin format");
            let bounds = [
                origin[0] + columns[column],
                origin[1] + rows[row_index],
                columns[(column + cell.column_span).min(columns.len() - 1)] - columns[column],
                rows[(row_index + cell.row_span).min(rows.len() - 1)] - rows[row_index],
            ];
            let fill = color(format.cell.get("fill")).or_else(|| {
                (!table.neutral_style).then(|| {
                    if column == 0 && table.style.contains_key("first_column") {
                        "2E67E8".into()
                    } else {
                        default_fill.clone()
                    }
                })
            });
            if let Some(color) = fill {
                page.operations
                    .push(PageOperation::Rectangle(HighlightRectangle {
                        x: bounds[0],
                        top: bounds[1],
                        width: bounds[2],
                        height: bounds[3],
                        color,
                    }));
            }
            let content = content_bounds(bounds, &format.cell);
            let mut paragraphs = crate::presentation::text::resolve(
                document,
                &cell.element.children,
                &format.text,
                theme,
                TextDefaults::Resolved,
            )?;
            let alignment = text(&format.paragraph, "alignment");
            if !alignment.is_empty() {
                for paragraph in &mut paragraphs {
                    paragraph.style.alignment = alignment.clone();
                }
            }
            if boolean(&format.cell, "no_wrap") {
                warnings.push("PowerPoint always wraps table-cell text; cell.no_wrap has no presentation effect".into());
            }
            let rotation = match text(&format.cell, "text_direction").as_str() {
                "tb_rl" => -90.0,
                "bt_lr" => 90.0,
                _ => 0.0,
            };
            let center = [content[0] + content[2] / 2.0, content[1] + content[3] / 2.0];
            let text_bounds = if rotation == 0.0 {
                content
            } else {
                [
                    center[0] - content[3] / 2.0,
                    center[1] - content[2] / 2.0,
                    content[3],
                    content[2],
                ]
            };
            let mut attributes = Properties::new();
            if let Some(value) = format.cell.get("vertical_alignment") {
                attributes.insert("vertical_alignment".into(), value.clone());
            }
            let fitted = drawing_text::fit_with_frame(
                &paragraphs,
                text_bounds,
                &attributes,
                drawing_text::TextFrame::TableCell,
                measurer,
            )?;
            if rotation != 0.0 {
                page.operations.push(PageOperation::PushRotation {
                    degrees: rotation,
                    center,
                });
            }
            page.append_content(fitted.page);
            if rotation != 0.0 {
                page.operations.push(PageOperation::PopRotation);
            }
        }
    }
    page.operations
        .extend(borders::operations(&table, origin, &columns, &rows));
    Ok(())
}
fn offsets(lengths: &[i64]) -> Vec<f64> {
    let mut output = vec![0.0];
    for length in lengths {
        output.push(output.last().unwrap() + *length as f64 / 12700.0);
    }
    output
}
fn content_bounds(bounds: [f64; 4], attributes: &Properties) -> [f64; 4] {
    let margin = object(attributes, "margin");
    let side = |name, fallback| {
        margin
            .and_then(|margin| units(margin.get(name)))
            .map(|value| value as f64 / 12700.0)
            .unwrap_or(fallback)
    };
    let left = side("left", 7.2);
    let top = side("top", 3.6);
    [
        bounds[0] + left,
        bounds[1] + top,
        (bounds[2] - left - side("right", 7.2)).max(1.0),
        (bounds[3] - top - side("bottom", 3.6)).max(1.0),
    ]
}
fn tinted(color: &str) -> String {
    let Ok(value) = u32::from_str_radix(color.trim_start_matches('#'), 16) else {
        return "E8EAFB".into();
    };
    [16, 8, 0]
        .iter()
        .map(|shift| {
            let encoded = ((value >> shift) & 255) as f64 / 255.0;
            let linear = if encoded <= 0.04045 {
                encoded / 12.92
            } else {
                ((encoded + 0.055) / 1.055).powf(2.4)
            };
            let tinted = linear * 0.2 + 0.8;
            let encoded = if tinted <= 0.0031308 {
                tinted * 12.92
            } else {
                1.055 * tinted.powf(1.0 / 2.4) - 0.055
            };
            format!("{:02X}", (encoded * 255.0).round() as u8)
        })
        .collect()
}
