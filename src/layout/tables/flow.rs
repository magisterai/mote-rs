use super::super::{
    FlowCursor, HighlightRectangle, LineFrame, LinePlacement, PageGeometry, PageOperation,
    TextPage,
    columns::ColumnSeparators,
    document::{ensure_space, next_area},
    drawings::Drawing,
    painting::ParagraphPainting,
};
use super::*;

struct RowPlacement {
    origin: [f64; 2],
    paint_rowspans: bool,
    outer_clip: Option<[f64; 4]>,
}

pub(in crate::layout) fn place(
    table: &TablePlan,
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
    pending: f64,
    boundary: Option<super::super::sections::FlowBoundary>,
) -> Result<Vec<String>, LayoutError> {
    let mut warnings = Vec::new();
    if table.rows.is_empty() {
        return Ok(warnings);
    }
    let border = table.top_border();
    let closing = table.closing_border_excess(0);
    let previous = (cursor.page_number, cursor.column_index);
    ensure_space(
        cursor,
        pages,
        columns,
        geometry,
        boundary,
        pending + border + table.rows[0].height + closing,
    );
    if previous == (cursor.page_number, cursor.column_index) {
        cursor.y += pending;
    }
    cursor.y += border;
    warn_of_cut_off_rows(table, cursor.page_number, false, &mut warnings);
    let header_count = table
        .rows
        .iter()
        .take_while(|row| row.repeat_header)
        .count();
    let header_height: f64 = table.rows[..header_count]
        .iter()
        .map(|row| row.height)
        .sum();
    let mut collector = borders::Collector::default();
    let mut start = 0;
    let mut anchored = false;
    while start < table.rows.len() {
        let end = pagination::block_end(table, start);
        let height: f64 = table.rows[start..end].iter().map(|row| row.height).sum();
        let closing = table.closing_border_excess(end - 1);
        let mut available = vec![
            (super::super::document::page_content_bottom(
                pages,
                geometry,
                boundary,
                cursor.page_number,
            ) - cursor.y
                - closing)
                .max(0.0),
        ];
        for _ in cursor.column_index + 1..geometry.column_count {
            available.push(
                (super::super::document::page_content_bottom(
                    pages,
                    geometry,
                    boundary,
                    cursor.page_number,
                ) - cursor.current_top
                    - header_height
                    - border
                    - closing)
                    .max(0.0),
            );
        }
        available.push(
            (geometry.content_bottom() - geometry.margin_top - header_height - border - closing)
                .max(0.0),
        );
        if let Some(slices) = pagination::plan(table, start, end, header_count, &available) {
            let mut frame = 0;
            for slice in slices {
                let advanced = slice.frame > frame;
                while frame < slice.frame {
                    next_area(cursor, pages, columns, geometry);
                    frame += 1;
                }
                let repeats_headers = advanced
                    && header_count > 0
                    && header_height + slice.height
                        <= super::super::document::page_content_bottom(
                            pages,
                            geometry,
                            boundary,
                            cursor.page_number,
                        ) - cursor.current_top;
                // Word sets the top border of the row that begins a page below the
                // top of the text area, as it does for the first row of the table.
                if advanced {
                    cursor.y += table.row_top_border(if repeats_headers { 0 } else { slice.row });
                }
                if repeats_headers {
                    for row in 0..header_count {
                        place_row(
                            table,
                            row,
                            RowPlacement {
                                origin: [
                                    table.left(
                                        geometry.column_left(cursor.column_index),
                                        geometry.column_width(),
                                    ),
                                    cursor.y,
                                ],
                                paint_rowspans: true,
                                outer_clip: None,
                            },
                            &mut pages[cursor.page_number - 1],
                            &mut collector,
                            cursor.page_number - 1,
                        )?;
                        cursor.y += table.rows[row].height;
                    }
                }
                let left = table.left(
                    geometry.column_left(cursor.column_index),
                    geometry.column_width(),
                );
                if !anchored {
                    pages[cursor.page_number - 1].place_anchors(&table.anchors, [left, cursor.y]);
                    anchored = true;
                }
                for span in &slice.spans {
                    let cell = &table.rows[span.row].cells[span.cell];
                    let bounds = cell_bounds(table, cell, left, cursor.y, span.height);
                    place_cell(
                        cell,
                        bounds,
                        span.window,
                        None,
                        &mut pages[cursor.page_number - 1],
                    )?;
                    collector.cell(
                        cursor.page_number - 1,
                        bounds,
                        &cell.borders,
                        span.first,
                        true,
                    );
                }
                for (index, cell) in table.rows[slice.row]
                    .cells
                    .iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.row_span == 1)
                {
                    let bounds = cell_bounds(table, cell, left, cursor.y, slice.height);
                    place_cell(
                        cell,
                        bounds,
                        slice.cells[index],
                        None,
                        &mut pages[cursor.page_number - 1],
                    )?;
                    collector.cell(
                        cursor.page_number - 1,
                        bounds,
                        &cell.borders,
                        slice.offset <= 0.01,
                        // Word closes the part of a split row on each page.
                        true,
                    );
                }
                cursor.y += slice.height;
                cursor.body_started = true;
                columns.note(cursor, geometry);
            }
        } else {
            let oversized = height > geometry.content_bottom() - geometry.margin_top;
            let degrade_rowspans = oversized && end > start + 1;
            if degrade_rowspans {
                warnings.push(format!("table row group {} exceeds a complete frame and has no safe continuation; rowspans are reduced to individual rows and content may be clipped", start + 1));
            } else if oversized {
                warnings.push(format!("table row {} exceeds a complete frame and has no safe continuation; atomic content may be clipped", start + 1));
            }
            for row in start..end {
                if degrade_rowspans || row == start {
                    let required = if degrade_rowspans {
                        table.rows[row].height
                    } else {
                        height
                    };
                    let before = (cursor.page_number, cursor.column_index);
                    ensure_space(
                        cursor,
                        pages,
                        columns,
                        geometry,
                        boundary,
                        required + closing,
                    );
                    let moved = before != (cursor.page_number, cursor.column_index);
                    let repeats_headers = moved
                        && row >= header_count
                        && header_count > 0
                        && header_height + required
                            <= super::super::document::page_content_bottom(
                                pages,
                                geometry,
                                boundary,
                                cursor.page_number,
                            ) - cursor.current_top;
                    if moved {
                        cursor.y += table.row_top_border(if repeats_headers { 0 } else { row });
                    }
                    if repeats_headers {
                        for header in 0..header_count {
                            place_row(
                                table,
                                header,
                                RowPlacement {
                                    origin: [
                                        table.left(
                                            geometry.column_left(cursor.column_index),
                                            geometry.column_width(),
                                        ),
                                        cursor.y,
                                    ],
                                    paint_rowspans: true,
                                    outer_clip: None,
                                },
                                &mut pages[cursor.page_number - 1],
                                &mut collector,
                                cursor.page_number - 1,
                            )?;
                            cursor.y += table.rows[header].height;
                        }
                    }
                }
                if !anchored {
                    pages[cursor.page_number - 1].place_anchors(
                        &table.anchors,
                        [
                            table.left(
                                geometry.column_left(cursor.column_index),
                                geometry.column_width(),
                            ),
                            cursor.y,
                        ],
                    );
                    anchored = true;
                }
                place_row(
                    table,
                    row,
                    RowPlacement {
                        origin: [
                            table.left(
                                geometry.column_left(cursor.column_index),
                                geometry.column_width(),
                            ),
                            cursor.y,
                        ],
                        paint_rowspans: !degrade_rowspans,
                        outer_clip: None,
                    },
                    &mut pages[cursor.page_number - 1],
                    &mut collector,
                    cursor.page_number - 1,
                )?;
                cursor.y += table.rows[row].height;
            }
        }
        cursor.body_started = true;
        columns.note(cursor, geometry);
        start = end;
    }
    collector.append(pages);
    cursor.body_started = true;
    Ok(warnings)
}
/// Places a row and returns the drawings of a nested table's cells, which the
/// enclosing cell places once its clip is lifted.
fn place_row<'plan>(
    table: &'plan TablePlan,
    row: usize,
    placement: RowPlacement,
    page: &mut TextPage,
    collector: &mut borders::Collector,
    page_index: usize,
) -> Result<Vec<&'plan Drawing>, LayoutError> {
    let [left, top] = placement.origin;
    let mut drawings = Vec::new();
    for cell in &table.rows[row].cells {
        let horizontal = left + table.widths[..cell.column].iter().sum::<f64>();
        let width = table.widths[cell.column..cell.column + cell.column_span]
            .iter()
            .sum();
        let row_span = if placement.paint_rowspans {
            cell.row_span
        } else {
            1
        };
        let height = table.rows[row..row + row_span]
            .iter()
            .map(|row| row.height)
            .sum();
        drawings.extend(place_cell(
            cell,
            [horizontal, top, width, height],
            pagination::Window {
                height,
                alignment: height,
                ..Default::default()
            },
            placement.outer_clip,
            page,
        )?);
        collector.cell(
            page_index,
            [horizontal, top, width, height],
            &cell.borders,
            true,
            true,
        );
    }
    Ok(drawings)
}
/// Reports each row of the table, and of the tables nested in its cells, whose
/// exact height cuts off its content.
fn warn_of_cut_off_rows(
    table: &TablePlan,
    page_number: usize,
    nested: bool,
    warnings: &mut Vec<String>,
) {
    for (index, row) in table.rows.iter().enumerate() {
        if row.cut_off {
            warnings.push(super::super::overflow::exact_row(
                index,
                page_number,
                nested,
            ));
        }
        for cell in &row.cells {
            for block in &cell.blocks {
                if let Block::Table(nested) = block {
                    warn_of_cut_off_rows(nested, page_number, true, warnings);
                }
            }
        }
    }
}
fn cell_bounds(table: &TablePlan, cell: &CellPlan, left: f64, top: f64, height: f64) -> [f64; 4] {
    [
        left + table.widths[..cell.column].iter().sum::<f64>(),
        top,
        table.widths[cell.column..cell.column + cell.column_span]
            .iter()
            .sum(),
        height,
    ]
}
/// Places a cell. A drawing lies at its own place on the page and not within
/// the cell, so a cell of a table in the body places its drawings once its clip
/// is lifted, and a cell of a nested table returns them to the enclosing cell.
fn place_cell<'plan>(
    cell: &'plan CellPlan,
    bounds: [f64; 4],
    window: pagination::Window,
    outer_clip: Option<[f64; 4]>,
    page: &mut TextPage,
) -> Result<Vec<&'plan Drawing>, LayoutError> {
    let mut drawings: Vec<&Drawing> = Vec::new();
    let [left, top, width, height] = bounds;
    if let Some(color) = super::super::style_values::color(&cell.attributes, "fill") {
        page.operations
            .push(PageOperation::Rectangle(HighlightRectangle {
                x: left,
                top,
                width,
                height,
                color,
            }));
    }
    let mut clip_top = top + window.inset;
    let mut clip_bottom = (clip_top + window.height).min(top + height);
    let mut clip_left = left;
    let mut clip_right = left + width;
    if let Some([outer_left, outer_top, outer_width, outer_height]) = outer_clip {
        clip_left = clip_left.max(outer_left);
        clip_right = clip_right.min(outer_left + outer_width);
        clip_top = clip_top.max(outer_top);
        clip_bottom = clip_bottom.min(outer_top + outer_height);
    }
    if clip_bottom <= clip_top + 0.01 || clip_right <= clip_left {
        return Ok(drawings);
    }
    let content_clip = [
        clip_left,
        clip_top,
        clip_right - clip_left,
        clip_bottom - clip_top,
    ];
    page.operations.push(PageOperation::PushClip(content_clip));
    let vertical = pagination::content_top(cell, window.alignment, window.top_aligned)
        - window.offset
        + window.inset;
    let content_left = left + cell.margins[3];
    let mut painting = ParagraphPainting::default();
    for (index, block) in cell.blocks.iter().enumerate() {
        let mut block_top = top + vertical + cell.content_tops[index];
        let object_left = content_left + cell.block_frames[index].x;
        let object_width = cell.block_frames[index].width;
        let block_height = cells::block_geometry(block).0;
        if block_top + block_height <= clip_top + 0.01 || block_top >= clip_bottom - 0.01 {
            continue;
        }
        match block {
            Block::Paragraph(paragraph) => {
                if paragraph.clear_fill_before {
                    painting.clear_fill();
                }
                for (line_index, line) in paragraph.text.lines.iter().enumerate() {
                    if block_top + line.height <= clip_top + 0.01 || block_top >= clip_bottom - 0.01
                    {
                        block_top += line.height;
                        continue;
                    }
                    let frame = if line_index == 0 {
                        paragraph.text.first_frame
                    } else {
                        paragraph.text.continuation_frame
                    };
                    painting.place_line(
                        page,
                        paragraph,
                        LinePlacement {
                            page_number: 1,
                            column_index: 0,
                            line_index,
                            top: block_top,
                            height: line.height,
                            frame: LineFrame {
                                x: content_left + frame.x,
                                width: frame.width,
                            },
                        },
                    );
                    block_top += line.height;
                }
                if paragraph.clear_fill_after {
                    painting.clear_fill();
                }
                continue;
            }
            Block::Divider(divider) => divider.place(page, object_left, block_top, object_width),
            Block::Image(image) => {
                page.operations
                    .extend(image.place(object_left, object_width, block_top)?)
            }
            Block::Table(table) => {
                let nested_left = table.left(object_left, object_width);
                let mut borders = borders::Collector::default();
                for row in 0..table.rows.len() {
                    drawings.extend(place_row(
                        table,
                        row,
                        RowPlacement {
                            origin: [nested_left, block_top],
                            paint_rowspans: true,
                            outer_clip: Some(content_clip),
                        },
                        page,
                        &mut borders,
                        0,
                    )?);
                    block_top += table.rows[row].height;
                }
                borders.append(std::slice::from_mut(page));
            }
            Block::Drawing(drawing) => drawings.push(drawing),
            // Cell content holds a chart as its fallback paragraph.
            Block::Chart(_) | Block::Break(_) => {}
        }
        painting.clear_fill();
    }
    page.operations.push(PageOperation::PopClip);
    if outer_clip.is_none() {
        for drawing in drawings.drain(..) {
            drawing.place(page);
        }
    }
    Ok(drawings)
}
