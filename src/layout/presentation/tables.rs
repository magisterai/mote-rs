//! Fixed slide grids share formatting with editable DrawingML tables.
mod borders;
use crate::{
    layout::{HighlightRectangle, LayoutError, PageOperation, TextPage, drawing_text, overflow},
    measurement::TextMeasurer,
    model::{Document, Element, Properties},
    presentation::{Theme, color, tables::SlideTable, text::TextDefaults, units},
    properties::{boolean, object, text},
};

/// The rotation in degrees that a text direction gives the text of a cell.
fn rotation(cell: &Properties) -> f64 {
    match text(cell, "text_direction").as_str() {
        "tb_rl" => -90.0,
        "bt_lr" => 90.0,
        _ => 0.0,
    }
}

/// Returns the heights of the rows in drawing units, which the native and
/// the editable output share. PowerPoint treats the height of a row as a
/// minimum and grows the row to hold the text of its cells, so that no row is
/// lower than its text; a cell of rotated text does not grow its row here.
///
/// The rows that state no height share a part of the box. A row whose text
/// needs more than an equal share takes what the text needs, and the rest of
/// that part is divided equally among the other rows, until no row needs more
/// than its share. The heights are those of the table itself where no row
/// needs more than it has.
pub(crate) fn row_heights(
    document: &Document,
    table: &SlideTable,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
) -> Result<Vec<i64>, LayoutError> {
    let mut required = vec![0_i64; table.heights.len()];
    let columns = offsets(&table.widths);
    let mut spanning = Vec::new();
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
            if rotation(&format.cell) != 0.0 {
                continue;
            }
            let width =
                columns[(column + cell.column_span).min(columns.len() - 1)] - columns[column];
            let [left, top, right, bottom] = margins(&format.cell);
            let paragraphs = crate::presentation::text::resolve(
                document,
                &cell.element.children,
                &format.text_attributes(),
                theme,
                TextDefaults::Resolved,
            )?;
            let fitted = drawing_text::fit_with_frame(
                &paragraphs,
                [0.0, 0.0, (width - left - right).max(1.0), 1.0],
                &Properties::new(),
                drawing_text::TextFrame::TableCell,
                measurer,
            )?;
            let needed = ((fitted.text_height + top + bottom) * 12700.0).ceil() as i64;
            let last = (row_index + cell.row_span).min(required.len());
            if last - row_index > 1 {
                spanning.push((row_index, last, needed));
            } else {
                required[row_index] = required[row_index].max(needed);
            }
        }
    }
    let mut heights = table.heights.clone();
    let mut unsettled: Vec<usize> = (0..heights.len())
        .filter(|row| !table.stated_heights[*row])
        .collect();
    let mut remaining: i64 = unsettled.iter().map(|row| heights[*row]).sum();
    let mut redistributed = false;
    while !unsettled.is_empty() {
        let share = (remaining / unsettled.len() as i64).max(0);
        let (exceeding, within): (Vec<usize>, Vec<usize>) =
            unsettled.iter().partition(|row| required[**row] > share);
        if exceeding.is_empty() {
            if redistributed {
                for row in within {
                    heights[row] = share;
                }
            }
            break;
        }
        for row in exceeding {
            heights[row] = required[row];
            remaining -= required[row];
        }
        redistributed = true;
        unsettled = within;
    }
    for (row, height) in heights.iter_mut().enumerate() {
        *height = (*height).max(required[row]);
    }
    for (first, last, needed) in spanning {
        let height: i64 = heights[first..last].iter().sum();
        if needed > height {
            heights[last - 1] += needed - height;
        }
    }
    Ok(heights)
}

/// Returns whether the rows, grown to hold their text, make the table taller
/// than its frame.
pub(crate) fn grows_beyond_frame(table: &SlideTable, heights: &[i64]) -> bool {
    // Half a point, below which a difference is not visible.
    const TOLERANCE: i64 = 6350;
    let grown: i64 = heights.iter().sum();
    grown > table.heights.iter().sum::<i64>() + TOLERANCE && grown > table.bounds[3] + TOLERANCE
}

pub(crate) fn place(
    document: &Document,
    container: &Element,
    element: &Element,
    theme: &Theme,
    measurer: &mut TextMeasurer<'_>,
    page: &mut TextPage,
    described: &str,
) -> Result<Vec<String>, LayoutError> {
    let mut warnings = Vec::new();
    let table =
        SlideTable::resolve(document, container, element).map_err(LayoutError::Unsupported)?;
    if table.bounds[2] <= 0 || table.bounds[3] <= 0 {
        return Ok(warnings);
    }
    let columns = offsets(&table.widths);
    let heights = row_heights(document, &table, theme, measurer)?;
    if grows_beyond_frame(&table, &heights) {
        warnings.push(overflow::table_rows(described));
    }
    let rows = offsets(&heights);
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
            let paragraphs = crate::presentation::text::resolve(
                document,
                &cell.element.children,
                &format.text_attributes(),
                theme,
                TextDefaults::Resolved,
            )?;
            if boolean(&format.cell, "no_wrap") {
                warnings.push("PowerPoint always wraps table-cell text; cell.no_wrap has no presentation effect".into());
            }
            let rotation = rotation(&format.cell);
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
            if fitted.is_cut_off() {
                warnings.push(overflow::cell_text(described, row_index, column));
            }
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
    Ok(warnings)
}
fn offsets(lengths: &[i64]) -> Vec<f64> {
    let mut output = vec![0.0];
    for length in lengths {
        output.push(output.last().unwrap() + *length as f64 / 12700.0);
    }
    output
}
/// Returns the left, top, right and bottom margins of a cell in points.
fn margins(attributes: &Properties) -> [f64; 4] {
    let margin = object(attributes, "margin");
    let side = |name, fallback| {
        margin
            .and_then(|margin| units(margin.get(name)))
            .map(|value| value as f64 / 12700.0)
            .unwrap_or(fallback)
    };
    [
        side("left", 7.2),
        side("top", 3.6),
        side("right", 7.2),
        side("bottom", 3.6),
    ]
}
fn content_bounds(bounds: [f64; 4], attributes: &Properties) -> [f64; 4] {
    let [left, top, right, bottom] = margins(attributes);
    [
        bounds[0] + left,
        bounds[1] + top,
        (bounds[2] - left - right).max(1.0),
        (bounds[3] - top - bottom).max(1.0),
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
