//! Line-safe row fragments. Each cell advances independently within a shared physical row.
use super::*;
#[derive(Clone, Copy, Default)]
pub(super) struct Window {
    pub offset: f64,
    pub height: f64,
    pub inset: f64,
    pub alignment: f64,
    pub top_aligned: bool,
}
pub(super) struct Slice {
    pub row: usize,
    pub frame: usize,
    pub offset: f64,
    pub height: f64,
    pub cells: Vec<Window>,
    pub spans: Vec<SpanSlice>,
}
pub(super) struct SpanSlice {
    pub row: usize,
    pub cell: usize,
    pub height: f64,
    pub window: Window,
    pub first: bool,
    pub last: bool,
}
#[derive(Clone, Copy)]
struct Segment {
    top: f64,
    bottom: f64,
    break_after: bool,
}
fn segments(cell: &CellPlan) -> Vec<Segment> {
    let mut output = Vec::new();
    for (index, block) in cell.blocks.iter().enumerate() {
        let mut top = cell.content_tops[index];
        let final_block = index + 1 == cell.blocks.len();
        match block {
            Block::Paragraph(paragraph) => {
                let count = paragraph.text.lines.len();
                let style = &paragraph.text.style;
                for (index, line) in paragraph.text.lines.iter().enumerate() {
                    let break_after = if index + 1 == count {
                        final_block || !style.keep_with_next
                    } else {
                        !style.keep_together
                            && (!style.widow_control || (index >= 1 && count - index > 2))
                    };
                    output.push(Segment {
                        top,
                        bottom: top + line.height,
                        break_after,
                    });
                    top += line.height;
                }
            }
            Block::Table(table) => {
                let mut row = 0;
                while row < table.rows.len() {
                    let end = block_end(table, row);
                    let height: f64 = table.rows[row..end].iter().map(|row| row.height).sum();
                    output.push(Segment {
                        top,
                        bottom: top + height,
                        break_after: true,
                    });
                    top += height;
                    row = end;
                }
            }
            _ => output.push(Segment {
                top,
                bottom: top + cells::block_geometry(block).0,
                break_after: true,
            }),
        }
    }
    output
}
pub(super) fn content_top(cell: &CellPlan, height: f64, force_top: bool) -> f64 {
    if force_top {
        return cell.margins[0];
    }
    match property_values::text(&cell.attributes, "vertical_alignment").as_str() {
        "center" | "middle" => ((height - cell.content_height) / 2.0).max(cell.margins[0]),
        "bottom" => height - cell.content_height - cell.margins[2],
        _ => cell.margins[0],
    }
}
fn break_allowed(cell: &CellPlan, height: f64, cut: f64) -> bool {
    let offset = content_top(cell, height, false);
    let mut previous = None;
    let mut following = false;
    for segment in segments(cell) {
        let top = offset + segment.top;
        let bottom = offset + segment.bottom;
        if cut > top + 0.01 && cut < bottom - 0.01 {
            return false;
        }
        if bottom <= cut + 0.01 {
            previous = Some(segment.break_after);
        } else {
            following = true;
        }
    }
    !following || previous.unwrap_or(true)
}
fn offsets(cell: &CellPlan, row_height: f64) -> (Vec<f64>, f64) {
    let segments = segments(cell);
    let mut offsets: Vec<f64> = segments
        .iter()
        .take(segments.len().saturating_sub(1))
        .filter(|segment| segment.break_after)
        .map(|segment| (cell.margins[0] + segment.bottom).max(0.0).min(row_height))
        .filter(|offset| *offset > 0.01)
        .collect();
    let end = (cell.margins[0] + cell.content_height + cell.margins[2] + 0.5)
        .max(0.0)
        .min(row_height);
    if end > 0.01 {
        offsets.push(end);
    }
    offsets.sort_by(f64::total_cmp);
    offsets.dedup_by(|left, right| (*left - *right).abs() <= 0.01);
    (offsets, end)
}
fn capacity(available: &[f64], frame: usize) -> f64 {
    available[frame.min(available.len() - 1)].max(0.0)
}
fn remaining_frames(available: &[f64], frame: usize, remaining: f64) -> Vec<f64> {
    let mut output = vec![remaining.max(0.0)];
    output.extend_from_slice(&available[(frame + 1).min(available.len())..]);
    if output.len() == 1 {
        output.push(capacity(available, frame + 1));
    }
    output
}
fn row_slices(row: &RowPlan, row_index: usize, available: &[f64]) -> Option<Vec<Slice>> {
    let ordinary: Vec<_> = row
        .cells
        .iter()
        .enumerate()
        .filter(|(_, cell)| cell.row_span == 1)
        .collect();
    if ordinary.is_empty()
        || ordinary
            .iter()
            .any(|(_, cell)| cells::points(cell.attributes.get("height")).unwrap_or(0.0) > 0.01)
    {
        return None;
    }
    let plans: Vec<_> = ordinary
        .iter()
        .map(|(_, cell)| offsets(cell, row.height))
        .collect();
    let mut cursors = vec![0.0; ordinary.len()];
    let mut output: Vec<Slice> = Vec::new();
    let mut frame = 0;
    let mut physical = 0.0;
    while plans
        .iter()
        .zip(&cursors)
        .any(|((_, end), cursor)| end - cursor > 0.01)
    {
        let available_height = capacity(available, frame);
        if output.is_empty() && row.height <= available_height + 0.01 {
            return Some(vec![Slice {
                row: row_index,
                frame,
                offset: 0.0,
                height: row.height,
                cells: row
                    .cells
                    .iter()
                    .map(|_| Window {
                        height: row.height,
                        alignment: row.height,
                        ..Default::default()
                    })
                    .collect(),
                spans: Vec::new(),
            }]);
        }
        let mut windows = vec![Window::default(); row.cells.len()];
        let mut next = cursors.clone();
        let mut height: f64 = 0.0;
        let mut blocked = false;
        for (index, ((column, cell), (offsets, end))) in ordinary.iter().zip(&plans).enumerate() {
            let cursor = cursors[index];
            if end - cursor <= 0.01 {
                continue;
            }
            let inset = if cursor > 0.01 {
                cell.margins[0].max(0.0)
            } else {
                0.0
            };
            let mut cut = cursor;
            let mut required = 0.0;
            for candidate in offsets {
                if *candidate <= cursor + 0.01 {
                    continue;
                }
                let candidate_height = candidate - cursor
                    + inset
                    + if *candidate < end - 0.01 {
                        cell.margins[2] + 0.5
                    } else {
                        0.0
                    };
                if candidate_height > available_height + 0.01 {
                    break;
                }
                cut = *candidate;
                required = candidate_height;
            }
            if cut <= cursor + 0.01 {
                blocked = true;
                continue;
            }
            windows[*column] = Window {
                offset: cursor,
                height: cut - cursor,
                inset,
                ..Default::default()
            };
            next[index] = cut;
            height = height.max(required);
        }
        if blocked || height <= 0.01 {
            if frame < available.len() - 1 {
                frame += 1;
                continue;
            }
            return None;
        }
        output.push(Slice {
            row: row_index,
            frame,
            offset: physical,
            height,
            cells: windows,
            spans: Vec::new(),
        });
        physical += height;
        cursors = next;
        frame += 1;
    }
    if let Some(first) = output.first() {
        let alignment = first.height;
        let counts: Vec<_> = (0..row.cells.len())
            .map(|index| {
                output
                    .iter()
                    .filter(|slice| slice.cells[index].height > 0.01)
                    .count()
            })
            .collect();
        for slice in &mut output {
            for (index, window) in slice.cells.iter_mut().enumerate() {
                if window.height <= 0.01 {
                    continue;
                }
                window.alignment = alignment;
                window.top_aligned = counts[index] > 1;
                if counts[index] == 1 {
                    window.height = alignment;
                }
            }
        }
    }
    Some(output)
}
pub(super) fn block_end(table: &TablePlan, start: usize) -> usize {
    let mut end = start + 1;
    let mut scan = start;
    while scan < end {
        for cell in &table.rows[scan].cells {
            end = end.max(scan + cell.row_span);
        }
        scan += 1;
    }
    end
}
pub(super) fn plan(
    table: &TablePlan,
    start: usize,
    end: usize,
    headers: usize,
    available: &[f64],
) -> Option<Vec<Slice>> {
    if end == start + 1 {
        if start < headers || table.rows[start].cant_split {
            return None;
        }
        return row_slices(&table.rows[start], start, available);
    }
    let mut output = Vec::new();
    let mut frame = 0;
    let mut remaining = capacity(available, 0);
    for row_index in start..end {
        let row = &table.rows[row_index];
        let slices = if row.cant_split {
            None
        } else {
            row_slices(
                row,
                row_index,
                &remaining_frames(available, frame, remaining),
            )
        };
        if let Some(mut slices) = slices {
            let mut painted = 0.0;
            let initial_frame = frame;
            for slice in &mut slices {
                slice.frame += initial_frame;
                if frame != slice.frame {
                    frame = slice.frame;
                    remaining = capacity(available, frame);
                }
                remaining = (remaining - slice.height).max(0.0);
                painted += slice.height;
            }
            output.extend(slices);
            while row.height - painted > 0.01 {
                if remaining <= 0.01 {
                    frame += 1;
                    remaining = capacity(available, frame);
                }
                if remaining <= 0.01 {
                    return None;
                }
                let height = (row.height - painted).min(remaining);
                output.push(Slice {
                    row: row_index,
                    frame,
                    offset: painted,
                    height,
                    cells: vec![Window::default(); row.cells.len()],
                    spans: Vec::new(),
                });
                painted += height;
                remaining -= height;
            }
        } else {
            if row.height > remaining + 0.01 {
                if row.height > capacity(available, available.len() - 1) + 0.01 {
                    return None;
                }
                loop {
                    frame += 1;
                    remaining = capacity(available, frame);
                    if row.height <= remaining + 0.01 {
                        break;
                    }
                }
            }
            output.push(Slice {
                row: row_index,
                frame,
                offset: 0.0,
                height: row.height,
                cells: row
                    .cells
                    .iter()
                    .map(|cell| {
                        if cell.row_span == 1 {
                            Window {
                                height: row.height,
                                alignment: row.height,
                                ..Default::default()
                            }
                        } else {
                            Window::default()
                        }
                    })
                    .collect(),
                spans: Vec::new(),
            });
            remaining -= row.height;
        }
    }
    for row_index in start..end {
        for (cell_index, cell) in table.rows[row_index]
            .cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.row_span > 1)
        {
            let mut groups: Vec<(usize, usize, f64)> = Vec::new();
            for (index, slice) in output.iter().enumerate().filter(|(_, slice)| {
                slice.row >= row_index && slice.row < row_index + cell.row_span
            }) {
                if let Some((_, frame, height)) = groups.last_mut()
                    && *frame == slice.frame
                {
                    *height += slice.height;
                } else {
                    groups.push((index, slice.frame, slice.height));
                }
            }
            let alignment = groups.first()?.2;
            let top_aligned =
                cell.margins[0] + cell.content_height + cell.margins[2] + 0.5 > alignment + 0.01;
            let mut offset = 0.0;
            for (index, (slice, _, height)) in groups.iter().enumerate() {
                if index > 0 && !break_allowed(cell, alignment, offset) {
                    return None;
                }
                output[*slice].spans.push(SpanSlice {
                    row: row_index,
                    cell: cell_index,
                    height: *height,
                    window: Window {
                        offset,
                        height: *height,
                        alignment,
                        top_aligned,
                        ..Default::default()
                    },
                    first: index == 0,
                    last: index + 1 == groups.len(),
                });
                offset += height;
            }
        }
    }
    Some(output)
}
