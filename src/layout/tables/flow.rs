use super::super::{
    FlowCursor, HighlightRectangle, LineFrame, LinePlacement, PageGeometry, PageOperation,
    TextPage,
    columns::ColumnSeparators,
    document::{
        LeadingSpace, ensure_space, next_area, page_content_bottom, pass_block, place_leading,
    },
    drawings::Drawing,
    painting::ParagraphPainting,
    tolerances::{FIT_TOLERANCE, TABLE_POSITION_TOLERANCE},
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
    let headers = HeaderRows::of(table);
    let closing = table.closing_border_excess(table.opening_rows() - 1);
    let room = page_content_bottom(pages, geometry, boundary, cursor.page_number)
        - cursor.y
        - pending
        - border
        - closing;
    place_leading(
        cursor,
        pages,
        columns,
        geometry,
        boundary,
        LeadingSpace {
            height: pending,
            fit_height: pending + border + headers.opening_height(room) + closing,
        },
    );
    cursor.y += border;
    warn_of_cut_off_rows(table, cursor.page_number, false, &mut warnings);
    let mut collector = borders::Collector::default();
    let mut start = 0;
    let mut anchored = false;
    while start < table.rows.len() {
        let end = pagination::block_end(table, start);
        let height: f64 = table.rows[start..end].iter().map(|row| row.height).sum();
        let closing = table.closing_border_excess(end - 1);
        let mut available = vec![
            (page_content_bottom(pages, geometry, boundary, cursor.page_number)
                - cursor.y
                - closing)
                .max(0.0),
        ];
        for _ in cursor.column_index + 1..geometry.column_count {
            available.push(
                (page_content_bottom(pages, geometry, boundary, cursor.page_number)
                    - cursor.current_top
                    - headers.height
                    - border
                    - closing)
                    .max(0.0),
            );
        }
        available.push(
            (geometry.content_bottom() - geometry.margin_top - headers.height - border - closing)
                .max(0.0),
        );
        if let Some(slices) = pagination::plan(table, start, end, headers.count, &available) {
            let mut frame = 0;
            for slice in slices {
                let advanced = slice.frame > frame;
                while frame < slice.frame {
                    next_area(cursor, pages, columns, geometry);
                    frame += 1;
                }
                if advanced {
                    headers.begin_area(
                        AreaStart {
                            row: slice.row,
                            height: slice.height,
                        },
                        cursor,
                        pages,
                        geometry,
                        boundary,
                        &mut collector,
                    )?;
                }
                let left = left_edge(table, geometry, cursor);
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
                        slice.offset <= TABLE_POSITION_TOLERANCE,
                        // Word closes the part of a split row on each page.
                        true,
                    );
                }
                pass_block(cursor, columns, geometry, slice.height);
            }
        } else {
            let oversized =
                height > geometry.content_bottom() - geometry.margin_top + FIT_TOLERANCE;
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
                    let previous = *cursor;
                    ensure_space(
                        cursor,
                        pages,
                        columns,
                        geometry,
                        boundary,
                        required + closing,
                    );
                    if !cursor.is_in_area_of(&previous) {
                        headers.begin_area(
                            AreaStart {
                                row,
                                height: required,
                            },
                            cursor,
                            pages,
                            geometry,
                            boundary,
                            &mut collector,
                        )?;
                    }
                }
                let left = left_edge(table, geometry, cursor);
                if !anchored {
                    pages[cursor.page_number - 1].place_anchors(&table.anchors, [left, cursor.y]);
                    anchored = true;
                }
                place_row(
                    table,
                    row,
                    RowPlacement {
                        origin: [left, cursor.y],
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
fn left_edge(table: &TablePlan, geometry: PageGeometry, cursor: &FlowCursor) -> f64 {
    table.left(
        geometry.column_left(cursor.column_index),
        geometry.column_width(),
    )
}
/// The row that begins an area to which the table has moved, and the height
/// of the row, or of its first part, that the area must hold below the header
/// rows if they are to be repeated.
struct AreaStart {
    row: usize,
    height: f64,
}
/// The leading rows of a table that are repeated in each area to which the
/// table moves.
struct HeaderRows<'plan> {
    table: &'plan TablePlan,
    count: usize,
    height: f64,
}
impl<'plan> HeaderRows<'plan> {
    fn of(table: &'plan TablePlan) -> Self {
        let count = table
            .rows
            .iter()
            .take_while(|row| row.repeat_header)
            .count();
        Self {
            table,
            count,
            height: table.rows[..count].iter().map(|row| row.height).sum(),
        }
    }
    /// Returns the height that an area with `room` for rows must hold for the
    /// table to begin there. Word keeps the header rows on the page of the row
    /// that follows them, and of a row that can be divided the area need hold
    /// the first part only.
    fn opening_height(&self, room: f64) -> f64 {
        let table = self.table;
        // A table whose every row is a header row repeats none.
        let (first, headers) = if self.count == table.rows.len() {
            (0, 0.0)
        } else {
            (self.count, self.height)
        };
        let end = pagination::block_end(table, first);
        let first_part = pagination::plan(
            table,
            first,
            end,
            first,
            &[(room - headers).max(0.0), f64::MAX],
        )
        .and_then(|slices| slices.first().map(|slice| (slice.frame, slice.height)))
        .filter(|(frame, _)| *frame == 0)
        .map(|(_, height)| height);
        headers
            + first_part.unwrap_or_else(|| {
                if first == 0 {
                    table.rows[0].height
                } else {
                    table.rows[first..end].iter().map(|row| row.height).sum()
                }
            })
    }
    /// Begins an area to which the table has moved: sets the top border of the
    /// row at the top of the area, and repeats the header rows if the area
    /// holds them above the row that begins it.
    fn begin_area(
        &self,
        start: AreaStart,
        cursor: &mut FlowCursor,
        pages: &mut [TextPage],
        geometry: PageGeometry,
        boundary: Option<super::super::sections::FlowBoundary>,
        collector: &mut borders::Collector,
    ) -> Result<(), LayoutError> {
        // A header row that begins an area is not preceded by itself.
        let repeats_headers = start.row >= self.count
            && self.count > 0
            && self.height + start.height
                <= page_content_bottom(pages, geometry, boundary, cursor.page_number)
                    - cursor.current_top
                    + FIT_TOLERANCE;
        // Word sets the top border of the row that begins a page below the
        // top of the text area, as it does for the first row of the table.
        cursor.y += self
            .table
            .row_top_border(if repeats_headers { 0 } else { start.row });
        if repeats_headers {
            for row in 0..self.count {
                place_row(
                    self.table,
                    row,
                    RowPlacement {
                        origin: [left_edge(self.table, geometry, cursor), cursor.y],
                        paint_rowspans: true,
                        outer_clip: None,
                    },
                    &mut pages[cursor.page_number - 1],
                    collector,
                    cursor.page_number - 1,
                )?;
                cursor.y += self.table.rows[row].height;
            }
        }
        Ok(())
    }
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
    if clip_bottom <= clip_top + TABLE_POSITION_TOLERANCE || clip_right <= clip_left {
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
        if block_top + block_height <= clip_top + TABLE_POSITION_TOLERANCE
            || block_top >= clip_bottom - TABLE_POSITION_TOLERANCE
        {
            continue;
        }
        match block {
            Block::Paragraph(paragraph) => {
                if paragraph.clear_fill_before {
                    painting.clear_fill();
                }
                for (line_index, line) in paragraph.text.lines.iter().enumerate() {
                    if block_top + line.height <= clip_top + TABLE_POSITION_TOLERANCE
                        || block_top >= clip_bottom - TABLE_POSITION_TOLERANCE
                    {
                        block_top += line.height;
                        continue;
                    }
                    let frame = paragraph.text.line_frame(line_index);
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
            Block::Drawing(drawing) => match &drawing.flow {
                Some(flow) => {
                    drawing.place_in_flow(flow, page, object_left, object_width, block_top)
                }
                None => drawings.push(drawing),
            },
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
