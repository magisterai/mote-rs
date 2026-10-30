use super::super::{
    FlowCursor, HighlightRectangle, LineFrame, LinePlacement, PageGeometry, PageOperation,
    TextPage,
    columns::ColumnSeparators,
    document::{ensure_space, next_area},
    painting::ParagraphPainting,
};
use super::*;
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
    let leading = pending.max(3.0);
    let previous = (cursor.page_number, cursor.column_index);
    ensure_space(
        cursor,
        pages,
        columns,
        geometry,
        boundary,
        leading + table.rows[0].height,
    );
    if previous == (cursor.page_number, cursor.column_index) {
        cursor.y += leading;
    }
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
        let mut available = vec![
            (super::super::document::page_content_bottom(
                pages,
                geometry,
                boundary,
                cursor.page_number,
            ) - cursor.y)
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
                    - header_height)
                    .max(0.0),
            );
        }
        available.push((geometry.content_bottom() - geometry.margin_top - header_height).max(0.0));
        if let Some(slices) = pagination::plan(table, start, end, header_count, &available) {
            let mut frame = 0;
            for slice in slices {
                let advanced = slice.frame > frame;
                while frame < slice.frame {
                    next_area(cursor, pages, columns, geometry);
                    frame += 1;
                }
                if advanced
                    && header_height + slice.height
                        <= super::super::document::page_content_bottom(
                            pages,
                            geometry,
                            boundary,
                            cursor.page_number,
                        ) - cursor.current_top
                {
                    for row in 0..header_count {
                        place_row(
                            table,
                            row,
                            [
                                table.left(
                                    geometry.column_left(cursor.column_index),
                                    geometry.column_width(),
                                ),
                                cursor.y,
                            ],
                            true,
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
                        &mut pages[cursor.page_number - 1],
                    )?;
                    collector.cell(
                        cursor.page_number - 1,
                        bounds,
                        &cell.borders,
                        span.first,
                        span.last,
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
                        &mut pages[cursor.page_number - 1],
                    )?;
                    collector.cell(
                        cursor.page_number - 1,
                        bounds,
                        &cell.borders,
                        slice.offset <= 0.01,
                        slice.offset + slice.height >= table.rows[slice.row].height - 0.01,
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
                    ensure_space(cursor, pages, columns, geometry, boundary, required);
                    if before != (cursor.page_number, cursor.column_index)
                        && row >= header_count
                        && header_height + required
                            <= super::super::document::page_content_bottom(
                                pages,
                                geometry,
                                boundary,
                                cursor.page_number,
                            ) - cursor.current_top
                    {
                        for header in 0..header_count {
                            place_row(
                                table,
                                header,
                                [
                                    table.left(
                                        geometry.column_left(cursor.column_index),
                                        geometry.column_width(),
                                    ),
                                    cursor.y,
                                ],
                                true,
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
                    [
                        table.left(
                            geometry.column_left(cursor.column_index),
                            geometry.column_width(),
                        ),
                        cursor.y,
                    ],
                    !degrade_rowspans,
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
    let before = (cursor.page_number, cursor.column_index);
    ensure_space(cursor, pages, columns, geometry, boundary, 1.0);
    if before == (cursor.page_number, cursor.column_index) {
        cursor.y += 1.0;
    }
    cursor.body_started = true;
    Ok(warnings)
}
fn place_row(
    table: &TablePlan,
    row: usize,
    origin: [f64; 2],
    paint_rowspans: bool,
    page: &mut TextPage,
    collector: &mut borders::Collector,
    page_index: usize,
) -> Result<(), LayoutError> {
    let [left, top] = origin;
    for cell in &table.rows[row].cells {
        let horizontal = left + table.widths[..cell.column].iter().sum::<f64>();
        let width = table.widths[cell.column..cell.column + cell.column_span]
            .iter()
            .sum();
        let row_span = if paint_rowspans { cell.row_span } else { 1 };
        let height = table.rows[row..row + row_span]
            .iter()
            .map(|row| row.height)
            .sum();
        place_cell(
            cell,
            [horizontal, top, width, height],
            pagination::Window {
                height,
                alignment: height,
                ..Default::default()
            },
            page,
        )?;
        collector.cell(
            page_index,
            [horizontal, top, width, height],
            &cell.borders,
            true,
            true,
        );
    }
    Ok(())
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
fn place_cell(
    cell: &CellPlan,
    bounds: [f64; 4],
    window: pagination::Window,
    page: &mut TextPage,
) -> Result<(), LayoutError> {
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
    let clip_top = top + window.inset;
    let clip_bottom = (clip_top + window.height).min(top + height);
    if clip_bottom <= clip_top + 0.01 {
        return Ok(());
    }
    page.operations.push(PageOperation::PushClip([
        left,
        clip_top,
        width,
        clip_bottom - clip_top,
    ]));
    let vertical = pagination::content_top(cell, window.alignment, window.top_aligned)
        - window.offset
        + window.inset;
    let content_left = left + cell.margins[3];
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
                let mut painting = ParagraphPainting::default();
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
            }
            Block::Image(image) => {
                page.operations
                    .extend(image.place(object_left, object_width, block_top)?)
            }
            Block::Table(table) => {
                let nested_left = table.left(object_left, object_width);
                let mut borders = borders::Collector::default();
                for row in 0..table.rows.len() {
                    place_row(
                        table,
                        row,
                        [nested_left, block_top],
                        true,
                        page,
                        &mut borders,
                        0,
                    )?;
                    block_top += table.rows[row].height;
                }
                borders.append(std::slice::from_mut(page));
            }
            Block::Chart(_) | Block::Drawing(_) => {
                return Err(LayoutError::Unsupported(
                    "native shapes in table cells are not yet supported".into(),
                ));
            }
            Block::Break(_) => {}
        }
    }
    page.operations.push(PageOperation::PopClip);
    Ok(())
}
