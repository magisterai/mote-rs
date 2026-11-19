use super::{
    DocumentTextPlan, FlowArea, FlowCursor, FlowOptions, FlowStep, LayoutError, PageGeometry,
    TextLine, TextPage, plan_paragraph_flow, widows::minimum_start_line_count,
};
use super::{
    blocks::{Block, PreparedParagraph},
    columns::ColumnSeparators,
    spacing::collapsed_spacing,
    tolerances::{AREA_START_TOLERANCE, FIT_TOLERANCE, LEAST_AREA_HEIGHT},
};
use crate::{
    measurement::TextMeasurer,
    model::{Document, ElementKind},
};

/// Resolve block content and assemble ordered native painting operations.
pub fn plan_document_text(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
) -> Result<DocumentTextPlan, LayoutError> {
    fn has_images(content: &[crate::model::Content]) -> bool {
        content.iter().any(|node|matches!(node,crate::model::Content::Element(element) if element.kind==ElementKind::Image || has_images(&element.children)))
    }
    if has_images(&document.content) {
        return Err(LayoutError::Unsupported(
            "image layout requires asset-aware render planning".into(),
        ));
    }
    plan_document_with_images(document, measurer, &crate::media::ImageCatalog::default())
}

pub(crate) fn plan_document_with_images(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
    images: &crate::media::ImageCatalog,
) -> Result<DocumentTextPlan, LayoutError> {
    if !document.presentation.properties.is_empty() {
        return Err(LayoutError::Unsupported(
            "presentation output is not yet supported".into(),
        ));
    }
    super::sections::plan(document, measurer, images)
}

pub(super) struct PlacementContext<'scope> {
    pub geometry: PageGeometry,
    pub boundary: Option<super::sections::FlowBoundary>,
    pub notes: &'scope mut super::notes::NoteLayout,
    /// Holds, for each block, whether the block stays on the page of the
    /// block after it.
    pub kept_with_next: &'scope [bool],
}
/// Places the blocks of a flow, and returns the page on which the flow last
/// left a column together with the lowest point that such a column of the page
/// reaches with the space after its last block.
pub(super) fn place_blocks(
    blocks: &[Block],
    context: PlacementContext<'_>,
    pages: &mut Vec<TextPage>,
    cursor: &mut FlowCursor,
    columns: &mut ColumnSeparators,
    pending_spacing: &mut f64,
    warnings: &mut Vec<String>,
) -> Result<Option<(usize, f64)>, LayoutError> {
    let PlacementContext {
        geometry,
        boundary,
        notes,
        kept_with_next,
    } = context;
    let mut painting = super::painting::ParagraphPainting::default();
    let mut heads_page = false;
    let mut left_columns: Option<(usize, f64)> = None;
    let mut note_left_column = |before: &(FlowCursor, f64), cursor: &FlowCursor| {
        if !cursor.is_in_area_of(&before.0) {
            let reach = before.0.y + before.1;
            left_columns = Some(match left_columns {
                Some((page, lowest)) if page == before.0.page_number => (page, lowest.max(reach)),
                _ => (before.0.page_number, reach),
            });
        }
    };
    let mut before_block = (*cursor, *pending_spacing);
    for (index, block) in blocks.iter().enumerate() {
        note_left_column(&before_block, cursor);
        before_block = (*cursor, *pending_spacing);
        // Word leaves out the space before a block that an explicit page break
        // sets at the head of a page.
        let mut after_page_break = std::mem::take(&mut heads_page);
        let kept_height = |leading: f64| {
            kept_blocks_height(
                &blocks[index..],
                &kept_with_next[index..],
                leading,
                geometry.content_bottom() - geometry.margin_top,
            )
        };
        match block {
            Block::Divider(divider) => {
                painting.clear_fill();
                if divider.clamped {
                    warnings.push("divider width was clamped to one point".into());
                }
                if divider.width <= 0.0 {
                    continue;
                }
                let before = if after_page_break {
                    0.0
                } else {
                    divider.before
                };
                let leading = collapsed_spacing(*pending_spacing, before);
                let required = divider.width.max(kept_height(leading));
                seek_room_below_leading(
                    cursor, pages, columns, geometry, boundary, leading, required,
                );
                divider.place(
                    &mut pages[cursor.page_number - 1],
                    geometry.column_left(cursor.column_index),
                    cursor.y,
                    geometry.column_width(),
                );
                pass_block(cursor, columns, geometry, divider.width);
                *pending_spacing = divider.after;
            }
            Block::Chart(chart) => {
                painting.clear_fill();
                if chart.height <= 0.0 {
                    continue;
                }
                let before = if after_page_break { 0.0 } else { chart.before };
                let leading = collapsed_spacing(*pending_spacing, before);
                let required = chart.height.max(kept_height(leading));
                seek_room_below_leading(
                    cursor, pages, columns, geometry, boundary, leading, required,
                );
                chart.place(
                    &mut pages[cursor.page_number - 1],
                    geometry.column_left(cursor.column_index),
                    cursor.y,
                );
                warnings.extend(chart.reduced_labels_warning(cursor.page_number));
                pass_block(cursor, columns, geometry, chart.height);
                *pending_spacing = chart.after;
            }
            Block::Drawing(drawing) if drawing.flow.is_some() => {
                let flow = drawing.flow.as_ref().unwrap();
                painting.clear_fill();
                let before = if after_page_break { 0.0 } else { flow.before };
                let leading = collapsed_spacing(*pending_spacing, before);
                let required = flow.height.max(kept_height(leading));
                seek_room_below_leading(
                    cursor, pages, columns, geometry, boundary, leading, required,
                );
                drawing.place_in_flow(
                    flow,
                    &mut pages[cursor.page_number - 1],
                    geometry.column_left(cursor.column_index),
                    geometry.column_width(),
                    cursor.y,
                );
                warnings.extend(
                    drawing
                        .cut_off_text
                        .iter()
                        .map(|cut_off| cut_off.warning(cursor.page_number)),
                );
                pass_block(cursor, columns, geometry, flow.height);
                *pending_spacing = flow.after;
            }
            Block::Drawing(drawing) => {
                painting.clear_fill();
                let leading = std::mem::take(pending_spacing);
                let required = DRAWING_HEIGHT.max(kept_height(leading));
                seek_room_below_leading(
                    cursor, pages, columns, geometry, boundary, leading, required,
                );
                drawing.place(&mut pages[cursor.page_number - 1]);
                warnings.extend(
                    drawing
                        .cut_off_text
                        .iter()
                        .map(|cut_off| cut_off.warning(cursor.page_number)),
                );
                pass_block(cursor, columns, geometry, DRAWING_HEIGHT);
            }

            Block::Break(kind) => {
                painting.clear_fill();
                if kind == "column" || kind == "next_column" {
                    next_area(cursor, pages, columns, geometry);
                } else {
                    new_page(cursor, pages, columns, geometry);
                    heads_page = true;
                }
                *pending_spacing = 0.0;
            }
            Block::Image(image) => {
                painting.clear_fill();
                let before = if after_page_break { 0.0 } else { image.before };
                let leading = collapsed_spacing(*pending_spacing, before);
                let required = image.height().max(kept_height(leading));
                seek_room_below_leading(
                    cursor, pages, columns, geometry, boundary, leading, required,
                );
                pages[cursor.page_number - 1].place_anchors(
                    &image.anchors,
                    [geometry.column_left(cursor.column_index), cursor.y],
                );
                pages[cursor.page_number - 1].operations.extend(image.place(
                    geometry.column_left(cursor.column_index),
                    geometry.column_width(),
                    cursor.y,
                )?);
                pass_block(cursor, columns, geometry, image.height());
                *pending_spacing = image.after;
            }
            Block::Table(table) => {
                painting.clear_fill();
                let required = kept_height(*pending_spacing);
                if required > 0.0 {
                    let previous = *cursor;
                    ensure_space(
                        cursor,
                        pages,
                        columns,
                        geometry,
                        boundary,
                        *pending_spacing + required,
                    );
                    // The table sets the space above it once it has sought room
                    // for its first row, so the space is here only dropped.
                    if !cursor.is_in_area_of(&previous) {
                        *pending_spacing = 0.0;
                    }
                }
                warnings.extend(super::tables::flow::place(
                    table,
                    cursor,
                    pages,
                    columns,
                    geometry,
                    *pending_spacing,
                    boundary,
                )?);
                *pending_spacing = 0.0;
            }
            Block::Paragraph(prepared) => {
                let paragraph = &prepared.text;
                if paragraph.lines.is_empty() {
                    pages[cursor.page_number - 1].place_anchors(
                        &prepared.anchors,
                        [geometry.column_left(cursor.column_index), cursor.y],
                    );
                    heads_page = after_page_break;
                    continue;
                }
                if prepared.clear_fill_before {
                    painting.clear_fill();
                }
                if paragraph.style.page_break_before
                    && (cursor.body_started
                        || cursor.column_index > 0
                        || cursor.y > geometry.margin_top)
                {
                    new_page(cursor, pages, columns, geometry);
                    *pending_spacing = 0.0;
                    painting.clear_fill();
                    after_page_break = true;
                }
                let before = if after_page_break {
                    0.0
                } else {
                    paragraph.style.spacing_before
                };
                let leading = collapsed_spacing(*pending_spacing, before);
                let minimum = keep_height(
                    &blocks[index..],
                    geometry.content_bottom() - geometry.margin_top,
                )
                .max(kept_height(leading));
                if paragraph
                    .lines
                    .iter()
                    .flat_map(|line| &line.fragments)
                    .any(|fragment| fragment.footnote_number.is_some())
                {
                    notes.place_paragraph(
                        prepared,
                        leading,
                        minimum,
                        super::notes::NoteFlowContext {
                            pages,
                            cursor,
                            columns,
                            geometry,
                            boundary,
                            warnings,
                        },
                        &mut painting,
                    )?;
                    *pending_spacing = paragraph.style.spacing_after;
                    if prepared.clear_fill_after {
                        painting.clear_fill();
                    }
                    continue;
                }
                let heights: Vec<_> = paragraph.lines.iter().map(|line| line.height).collect();
                let frames: Vec<_> = (0..heights.len())
                    .map(|index| paragraph.line_frame(index))
                    .collect();
                let flow = plan_paragraph_flow(
                    *cursor,
                    &heights,
                    &frames,
                    &FlowOptions {
                        leading_space: leading,
                        minimum_start_space: minimum,
                        keep_together: paragraph.style.keep_together,
                        widow_control: paragraph.style.widow_control,
                        trailing_spaces: paragraph
                            .lines
                            .iter()
                            .map(|line| line.trailing_space)
                            .collect(),
                    },
                    |page_number| {
                        let mut area = FlowArea::from(geometry);
                        area.content_bottom =
                            page_content_bottom(pages, geometry, boundary, page_number);
                        area.whole_lines = boundary.is_some_and(|boundary| {
                            boundary.page == page_number
                                && boundary.bottom < geometry.content_bottom()
                        });
                        if page_number == cursor.page_number {
                            // A section that begins at the foot of a full page
                            // leaves no room there, so its first line moves on.
                            let top = cursor
                                .current_top
                                .min(area.content_bottom - LEAST_AREA_HEIGHT);
                            area.content_top = top;
                            area.page_start = top;
                        }
                        area
                    },
                )?;
                for step in flow.steps {
                    if let FlowStep::Line(placement) = step {
                        columns.note(
                            &FlowCursor {
                                page_number: placement.page_number,
                                column_index: placement.column_index,
                                y: placement.top + placement.height,
                                ..Default::default()
                            },
                            geometry,
                        );
                        while pages.len() < placement.page_number {
                            pages.push(TextPage::default());
                        }
                        painting.place_line(
                            &mut pages[placement.page_number - 1],
                            prepared,
                            placement,
                        );
                    }
                }
                *cursor = flow.final_cursor;
                *pending_spacing = paragraph.style.spacing_after;
                if prepared.clear_fill_after {
                    painting.clear_fill();
                }
            }
        }
    }
    note_left_column(&before_block, cursor);
    Ok(left_columns)
}
fn new_page(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
) {
    columns.note(cursor, geometry);
    pages.push(TextPage::default());
    *cursor = FlowCursor {
        page_number: pages.len(),
        current_top: geometry.margin_top,
        y: geometry.margin_top,
        ..Default::default()
    };
}
pub(super) fn ensure_space(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
    boundary: Option<super::sections::FlowBoundary>,
    height: f64,
) {
    // A block that a whole page cannot hold gains nothing by a move from the
    // head of a page, and stays there as it does in Word.
    let heads_whole_page = cursor.y <= geometry.margin_top + AREA_START_TOLERANCE
        && boundary.is_none_or(|boundary| boundary.page != cursor.page_number);
    if !heads_whole_page
        && cursor.y + height
            > page_content_bottom(pages, geometry, boundary, cursor.page_number) + FIT_TOLERANCE
    {
        next_area(cursor, pages, columns, geometry);
    }
}
/// The space above a block of the body.
pub(super) struct LeadingSpace {
    pub height: f64,
    /// The height that the area must hold below the cursor if the block is to
    /// stay in the area: the space and whatever must share the area with it.
    pub fit_height: f64,
}
/// Seeks room for the space above a block, and sets the space unless the move
/// of the block drops it.
pub(super) fn place_leading(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
    boundary: Option<super::sections::FlowBoundary>,
    leading: LeadingSpace,
) {
    let previous = *cursor;
    ensure_space(
        cursor,
        pages,
        columns,
        geometry,
        boundary,
        leading.fit_height,
    );
    cursor.add_leading(&previous, leading.height);
}
/// Seeks room for a divider, a chart or an image: for the space above it
/// together with the height `required`, and then for that height alone.
fn seek_room_below_leading(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
    boundary: Option<super::sections::FlowBoundary>,
    leading: f64,
    required: f64,
) {
    if leading > 0.0 {
        place_leading(
            cursor,
            pages,
            columns,
            geometry,
            boundary,
            LeadingSpace {
                height: leading,
                fit_height: leading + required,
            },
        );
    }
    ensure_space(cursor, pages, columns, geometry, boundary, required);
}
/// Moves the cursor past a block of the given height that has been placed at
/// the cursor.
pub(super) fn pass_block(
    cursor: &mut FlowCursor,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
    height: f64,
) {
    cursor.y += height;
    cursor.body_started = true;
    columns.note(cursor, geometry);
}
pub(super) fn next_area(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    columns: &mut ColumnSeparators,
    geometry: PageGeometry,
) {
    columns.note(cursor, geometry);
    if cursor.column_index + 1 < geometry.column_count {
        cursor.column_index += 1;
        cursor.y = cursor.current_top;
    } else {
        new_page(cursor, pages, columns, geometry);
    }
}
/// Returns the height that a page must hold of `lines` if they are the last
/// lines on it: their whole height less the trailing space of the last line.
pub(super) fn fitting_height(lines: &[TextLine]) -> f64 {
    lines.iter().map(|line| line.height).sum::<f64>()
        - lines.last().map_or(0.0, |line| line.trailing_space)
}
/// Returns the height that the page on which a paragraph starts must hold.
fn start_height(prepared: &PreparedParagraph) -> f64 {
    let paragraph = &prepared.text;
    let count = minimum_start_line_count(
        paragraph.lines.len(),
        paragraph.style.widow_control,
        paragraph.style.keep_together,
    );
    fitting_height(&paragraph.lines[..count])
}
fn keep_height(blocks: &[Block], maximum: f64) -> f64 {
    let Some(Block::Paragraph(first)) = blocks.first() else {
        return 0.0;
    };
    let first = &first.text;
    if !first.style.keep_with_next {
        return 0.0;
    }
    let mut count = 1;
    let mut required: f64 = first.lines.iter().map(|line| line.height).sum();
    let mut previous_after = first.style.spacing_after;
    for block in &blocks[1..] {
        let Block::Paragraph(prepared) = block else {
            break;
        };
        let paragraph = &prepared.text;
        if paragraph.style.page_break_before {
            break;
        }
        count += 1;
        required += collapsed_spacing(previous_after, paragraph.style.spacing_before);
        if paragraph.style.keep_with_next {
            required += paragraph.lines.iter().map(|line| line.height).sum::<f64>();
            previous_after = paragraph.style.spacing_after;
        } else {
            required += start_height(prepared);
            break;
        }
    }
    if count < 2 || first.style.spacing_before + required > maximum + FIT_TOLERANCE {
        0.0
    } else {
        required
    }
}

/// The height by which a drawing anchored in the flow advances the flow.
pub(super) const DRAWING_HEIGHT: f64 = 1.0;

/// Returns the height that the first block needs below its leading space if
/// that block and the blocks kept with it are to share a page: the whole of
/// each kept block, and the start of the first block that is not kept. The
/// result is zero if the first block is not kept, or if a full page cannot
/// hold the blocks, in which case each block breaks as it would alone.
fn kept_blocks_height(
    blocks: &[Block],
    kept_with_next: &[bool],
    leading: f64,
    maximum: f64,
) -> f64 {
    if !kept_with_next.first().copied().unwrap_or(false) {
        return 0.0;
    }
    let mut required: f64 = 0.0;
    let mut previous_after: Option<f64> = None;
    for (index, (block, kept)) in blocks.iter().zip(kept_with_next).enumerate() {
        let (height, before, after) = match block {
            Block::Break(_) => return 0.0,
            Block::Paragraph(prepared) if index > 0 && prepared.text.style.page_break_before => {
                return 0.0;
            }
            Block::Paragraph(prepared) => {
                let paragraph = &prepared.text;
                let height = if *kept {
                    paragraph.lines.iter().map(|line| line.height).sum()
                } else {
                    start_height(prepared)
                };
                (
                    height,
                    paragraph.style.spacing_before,
                    paragraph.style.spacing_after,
                )
            }
            Block::Divider(divider) => (divider.width, divider.before, divider.after),
            Block::Chart(chart) => (chart.height, chart.before, chart.after),
            Block::Drawing(drawing) => match &drawing.flow {
                Some(flow) => (flow.height, flow.before, flow.after),
                None => (DRAWING_HEIGHT, 0.0, 0.0),
            },
            Block::Image(image) => (image.height(), image.before, image.after),
            Block::Table(table) => (
                table.top_border()
                    + if *kept {
                        table.height()
                    } else {
                        table.opening_height()
                    },
                0.0,
                0.0,
            ),
        };
        if height > 0.0 {
            if let Some(previous_after) = previous_after {
                required += collapsed_spacing(previous_after, before);
            }
            required += height;
            previous_after = Some(after);
        }
        if !kept {
            break;
        }
    }
    if leading + required > maximum + FIT_TOLERANCE {
        0.0
    } else {
        required
    }
}

pub(super) fn page_content_bottom(
    pages: &[TextPage],
    geometry: PageGeometry,
    boundary: Option<super::sections::FlowBoundary>,
    page_number: usize,
) -> f64 {
    super::sections::content_bottom(geometry, boundary, page_number).min(
        pages
            .get(page_number.wrapping_sub(1))
            .and_then(|page| page.note_body_bottom)
            .unwrap_or(f64::INFINITY),
    )
}
