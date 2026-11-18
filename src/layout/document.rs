use super::{
    DocumentTextPlan, FlowArea, FlowCursor, FlowOptions, FlowStep, LayoutError,
    LayoutStyleResolver, LineBreak, PageGeometry, PlacedTextLine, TextLine, TextPage,
    plan_paragraph_flow, widows::minimum_start_line_count,
};
use super::{
    blocks::{Block, PreparedParagraph},
    columns::ColumnSeparators,
    paragraphs::LineFrames,
    spacing::{LeadingDrop, NegativeSpaceAfter, carried_spacing, collapsed_spacing},
    tolerances::{BLOCK_FIT_TOLERANCE, LEAST_AREA_HEIGHT},
};
use crate::{
    measurement::TextMeasurer,
    model::{Document, Element, ElementKind, Properties},
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

pub(super) struct PlacementContext<'scope, 'document> {
    pub styles: &'scope LayoutStyleResolver<'document>,
    pub geometry: PageGeometry,
    pub boundary: Option<super::sections::FlowBoundary>,
    pub notes: &'scope mut super::notes::NoteLayout,
    /// Holds, for each block, whether the block stays on the page of the
    /// block after it.
    pub kept_with_next: &'scope [bool],
}
pub(super) fn place_blocks(
    blocks: &[Block],
    context: PlacementContext<'_, '_>,
    pages: &mut Vec<TextPage>,
    cursor: &mut FlowCursor,
    columns: &mut ColumnSeparators,
    pending_spacing: &mut f64,
    warnings: &mut Vec<String>,
) -> Result<(), LayoutError> {
    let PlacementContext {
        styles,
        geometry,
        boundary,
        notes,
        kept_with_next,
    } = context;
    let mut painting = super::painting::ParagraphPainting::default();
    for (index, block) in blocks.iter().enumerate() {
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
                let leading = collapsed_spacing(*pending_spacing, divider.before);
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
                cursor.y += divider.width;
                cursor.body_started = true;
                columns.note(cursor, geometry);
                *pending_spacing = carried_spacing(divider.after, NegativeSpaceAfter::TakenAsNone);
            }
            Block::Chart(chart) => {
                painting.clear_fill();
                if chart.height <= 0.0 {
                    continue;
                }
                let leading = collapsed_spacing(*pending_spacing, chart.before);
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
                cursor.y += chart.height;
                cursor.body_started = true;
                columns.note(cursor, geometry);
                *pending_spacing = carried_spacing(chart.after, NegativeSpaceAfter::TakenAsNone);
            }
            Block::Drawing(drawing) => {
                painting.clear_fill();
                if *pending_spacing > 0.0 {
                    place_leading(
                        cursor,
                        pages,
                        columns,
                        geometry,
                        boundary,
                        LeadingSpace {
                            height: *pending_spacing,
                            fit_height: *pending_spacing,
                            dropped: LeadingDrop::Never,
                        },
                    );
                }
                *pending_spacing = 0.0;
                ensure_space(
                    cursor,
                    pages,
                    columns,
                    geometry,
                    boundary,
                    DRAWING_HEIGHT.max(kept_height(0.0)),
                );
                drawing.place(&mut pages[cursor.page_number - 1]);
                if let Some(cut_off) = &drawing.cut_off_text {
                    warnings.push(cut_off.warning(cursor.page_number));
                }
                cursor.y += DRAWING_HEIGHT;
                cursor.body_started = true;
                columns.note(cursor, geometry);
            }

            Block::Break(kind) => {
                painting.clear_fill();
                if kind == "line" {
                    let style = styles.paragraph(&Element {
                        source_name: None,
                        source_location: None,
                        kind: ElementKind::Paragraph,
                        attributes: Properties::new(),
                        children: Vec::new(),
                    });
                    if !style.line_height.is_finite() || style.line_height <= 0.0 {
                        return Err(LayoutError::InvalidGeometry(
                            "hard-break line height must be positive and finite",
                        ));
                    }
                    if *pending_spacing > 0.0 {
                        place_leading(
                            cursor,
                            pages,
                            columns,
                            geometry,
                            boundary,
                            LeadingSpace {
                                height: *pending_spacing,
                                fit_height: *pending_spacing,
                                dropped: LeadingDrop::Never,
                            },
                        );
                    }
                    ensure_space(
                        cursor,
                        pages,
                        columns,
                        geometry,
                        boundary,
                        style.line_height,
                    );
                    let frame = LineFrames::within(&style, geometry.column_width()).first;
                    pages[cursor.page_number - 1].push_line(PlacedTextLine {
                        top: cursor.y,
                        line: TextLine {
                            x: geometry.column_left(cursor.column_index) + frame.x,
                            baseline: cursor.y + style.line_height * 0.8,
                            height: style.line_height,
                            trailing_space: 0.0,
                            width: frame.width,
                            line_break: LineBreak::Final,
                            fragments: Vec::new(),
                            highlights: Vec::new(),
                            decorations: Vec::new(),
                        },
                    });
                    cursor.y += style.line_height;
                    cursor.body_started = true;
                    columns.note(cursor, geometry);
                } else if kind == "column" || kind == "next_column" {
                    next_area(cursor, pages, columns, geometry);
                } else {
                    new_page(cursor, pages, columns, geometry);
                }
                *pending_spacing = 0.0;
            }
            Block::Image(image) => {
                painting.clear_fill();
                let leading = collapsed_spacing(*pending_spacing, image.before);
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
                cursor.y += image.height();
                cursor.body_started = true;
                columns.note(cursor, geometry);
                *pending_spacing = carried_spacing(image.after, NegativeSpaceAfter::TakenAsNone);
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
                    if !cursor.keeps_leading(&previous, LeadingDrop::InNewPageOrColumn) {
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
                }
                let leading = collapsed_spacing(*pending_spacing, paragraph.style.spacing_before);
                let minimum = keep_height(
                    &blocks[index..],
                    geometry.content_bottom() - geometry.margin_top,
                )
                .max(prepared.minimum_start())
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
                    *pending_spacing = carried_spacing(
                        paragraph.style.spacing_after,
                        NegativeSpaceAfter::TakenAsNone,
                    );
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
                        keep_fit_leading_space: leading,
                        minimum_start_space: minimum,
                        keep_together: paragraph.style.keep_together,
                        widow_control: paragraph.style.widow_control,
                        trailing_spaces: paragraph
                            .lines
                            .iter()
                            .map(|line| line.trailing_space)
                            .collect(),
                        ..Default::default()
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
                *pending_spacing = carried_spacing(
                    paragraph.style.spacing_after,
                    NegativeSpaceAfter::TakenAsNone,
                );
                if prepared.clear_fill_after {
                    painting.clear_fill();
                }
            }
        }
    }
    Ok(())
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
    if cursor.y + height
        > page_content_bottom(pages, geometry, boundary, cursor.page_number) + BLOCK_FIT_TOLERANCE
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
    pub dropped: LeadingDrop,
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
    cursor.add_leading(&previous, leading.height, leading.dropped);
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
                dropped: LeadingDrop::InNewPageOrColumn,
            },
        );
    }
    ensure_space(cursor, pages, columns, geometry, boundary, required);
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
    fitting_height(&paragraph.lines[..count]).max(prepared.minimum_start())
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
    let mut previous_after = carried_spacing(first.style.spacing_after, NegativeSpaceAfter::Kept);
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
            previous_after =
                carried_spacing(paragraph.style.spacing_after, NegativeSpaceAfter::Kept);
        } else {
            required += start_height(prepared);
            break;
        }
    }
    if count < 2 || first.style.spacing_before + required > maximum + BLOCK_FIT_TOLERANCE {
        0.0
    } else {
        required
    }
}

/// The height by which a drawing anchored in the flow advances the flow.
const DRAWING_HEIGHT: f64 = 1.0;

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
            Block::Drawing(_) => (DRAWING_HEIGHT, 0.0, 0.0),
            Block::Image(image) => (image.height(), image.before, image.after),
            Block::Table(table) => (
                table.top_border()
                    + if *kept {
                        table.height()
                    } else {
                        table.rows.first().map_or(0.0, |row| row.height)
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
            previous_after = Some(carried_spacing(after, NegativeSpaceAfter::TakenAsNone));
        }
        if !kept {
            break;
        }
    }
    if leading + required > maximum + BLOCK_FIT_TOLERANCE {
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
