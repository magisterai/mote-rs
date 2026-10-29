use super::{
    FlowArea, FlowCursor, FlowOptions, FlowStep, LayoutError, LayoutStyleResolver, LineBreak,
    LineFrame, PageGeometry, PageOperation, TextLine, minimum_start_height, plan_paragraph_flow,
};
use super::{blocks::Block, columns::ColumnSeparators};
use crate::{
    measurement::TextMeasurer,
    model::{Document, Element, ElementKind, Properties},
};

/// Absolute text positions. Painting and non-text block layout are separate stages.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedTextLine {
    pub top: f64,
    pub line: TextLine,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextPage {
    /// Section geometry of the physical page; absent on manually assembled plans.
    pub geometry: Option<PageGeometry>,
    pub section_index: usize,
    pub page_number: usize,
    pub lines: Vec<PlacedTextLine>,
    pub anchors: std::collections::BTreeMap<String, [f64; 2]>,
    pub operations: Vec<PageOperation>,
}
impl TextPage {
    pub(super) fn place_anchors(&mut self, names: &[String], position: [f64; 2]) {
        for name in names {
            self.anchors.entry(name.clone()).or_insert(position);
        }
    }
    pub(super) fn push_line(&mut self, line: PlacedTextLine) {
        for fragment in &line.line.fragments {
            self.place_anchors(&fragment.anchors, [fragment.x, line.top]);
        }
        self.operations.push(PageOperation::Text(self.lines.len()));
        self.lines.push(line);
    }
    pub fn text_lines(&self) -> impl Iterator<Item = &TextLine> {
        self.lines
            .iter()
            .map(|placed| &placed.line)
            .chain(
                self.operations
                    .iter()
                    .filter_map(|operation| match operation {
                        PageOperation::ListMarker(marker) => marker.text.as_ref(),
                        _ => None,
                    }),
            )
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentTextPlan {
    pub geometry: PageGeometry,
    pub pages: Vec<TextPage>,
    /// Observable layout fallbacks shared by every painter.
    pub warnings: Vec<String>,
}

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
    super::sections::plan(document, measurer, images)
}

pub(super) struct PlacementContext<'scope, 'document> {
    pub styles: &'scope LayoutStyleResolver<'document>,
    pub geometry: PageGeometry,
    pub boundary: Option<super::sections::FlowBoundary>,
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
    } = context;
    let mut painting = super::painting::ParagraphPainting::default();
    for (index, block) in blocks.iter().enumerate() {
        match block {
            Block::Break(kind) => {
                painting.clear_fill();
                if kind == "line" {
                    let style = styles.paragraph(&Element {
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
                        ensure_space(cursor, pages, columns, geometry, boundary, *pending_spacing);
                        cursor.y += *pending_spacing;
                    }
                    ensure_space(
                        cursor,
                        pages,
                        columns,
                        geometry,
                        boundary,
                        style.line_height,
                    );
                    let frame = frame_for(&style, geometry.column_width());
                    pages[cursor.page_number - 1].push_line(PlacedTextLine {
                        top: cursor.y,
                        line: TextLine {
                            x: geometry.column_left(cursor.column_index) + frame.x,
                            baseline: cursor.y + style.line_height * 0.8,
                            height: style.line_height,
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
                let leading = (*pending_spacing).max(image.before);
                if leading > 0.0 {
                    let previous = (cursor.page_number, cursor.column_index);
                    ensure_space(
                        cursor,
                        pages,
                        columns,
                        geometry,
                        boundary,
                        leading + image.height(),
                    );
                    if previous == (cursor.page_number, cursor.column_index) {
                        cursor.y += leading;
                    }
                }
                ensure_space(cursor, pages, columns, geometry, boundary, image.height());
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
                *pending_spacing = image.after.max(0.0);
            }
            Block::Table(table) => {
                painting.clear_fill();
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
                let leading = (*pending_spacing).max(paragraph.style.spacing_before);
                let minimum = keep_height(
                    &blocks[index..],
                    geometry.content_bottom() - geometry.margin_top,
                )
                .max(prepared.minimum_start());
                let heights: Vec<_> = paragraph.lines.iter().map(|line| line.height).collect();
                let frames: Vec<_> = (0..heights.len())
                    .map(|index| {
                        if index == 0 {
                            paragraph.first_frame
                        } else {
                            paragraph.continuation_frame
                        }
                    })
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
                        ..Default::default()
                    },
                    |page_number| {
                        let mut area = FlowArea::from(geometry);
                        area.content_bottom =
                            super::sections::content_bottom(geometry, boundary, page_number);
                        if page_number == cursor.page_number {
                            area.content_top = cursor.current_top;
                            area.page_start = cursor.current_top;
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
                *pending_spacing = paragraph.style.spacing_after.max(0.0);
                if prepared.clear_fill_after {
                    painting.clear_fill();
                }
            }
        }
    }
    Ok(())
}
fn frame_for(style: &super::ParagraphStyle, width: f64) -> LineFrame {
    let available = (width - style.indent_left - style.indent_right).max(width.min(12.0));
    let origin = style.indent_left.min(width - available);
    LineFrame {
        x: origin + style.first_line,
        width: available - style.first_line,
    }
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
    if cursor.y + height > super::sections::content_bottom(geometry, boundary, cursor.page_number) {
        next_area(cursor, pages, columns, geometry);
    }
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
        required += previous_after.max(paragraph.style.spacing_before);
        let heights: Vec<_> = paragraph.lines.iter().map(|line| line.height).collect();
        if paragraph.style.keep_with_next {
            required += heights.iter().sum::<f64>();
            previous_after = paragraph.style.spacing_after;
        } else {
            required += minimum_start_height(
                &heights,
                paragraph.style.widow_control,
                paragraph.style.keep_together,
            )
            .max(prepared.minimum_start());
            break;
        }
    }
    if count < 2 || first.style.spacing_before + required > maximum {
        0.0
    } else {
        required
    }
}
