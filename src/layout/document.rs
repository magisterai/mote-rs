use super::blocks::{Block, prepare_blocks};
use super::{
    FlowArea, FlowCursor, FlowOptions, FlowStep, HighlightRectangle, LayoutError,
    LayoutStyleResolver, LineBreak, LineFrame, ListMarkerKind, PageGeometry, PageOperation, Rule,
    TextLine, minimum_start_height, plan_paragraph_flow,
};
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
    pub lines: Vec<PlacedTextLine>,
    pub operations: Vec<PageOperation>,
}
impl TextPage {
    fn push_line(&mut self, line: PlacedTextLine) {
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
}

/// Resolve block content and assemble ordered native painting operations.
pub fn plan_document_text(
    document: &Document,
    measurer: &mut TextMeasurer<'_>,
) -> Result<DocumentTextPlan, LayoutError> {
    let geometry = PageGeometry::from_document(document)?;
    if geometry.column_count != 1 {
        return Err(LayoutError::Unsupported(
            "document planning does not yet balance multiple columns".into(),
        ));
    }
    let styles = LayoutStyleResolver::new(document);
    let blocks = prepare_blocks(document, geometry.column_width(), &styles, measurer)?;
    if blocks.is_empty() {
        return Ok(DocumentTextPlan {
            geometry,
            pages: Vec::new(),
        });
    }
    let mut pages = vec![TextPage::default()];
    let mut cursor = FlowCursor {
        page_number: 1,
        current_top: geometry.margin_top,
        y: geometry.margin_top,
        ..Default::default()
    };
    let mut pending_spacing: f64 = 0.0;
    let mut pending_fill: Option<(usize, usize, HighlightRectangle, f64)> = None;
    for (index, block) in blocks.iter().enumerate() {
        match block {
            Block::Break(kind) => {
                pending_fill = None;
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
                    if pending_spacing > 0.0 {
                        ensure_space(&mut cursor, &mut pages, geometry, pending_spacing);
                        cursor.y += pending_spacing;
                    }
                    ensure_space(&mut cursor, &mut pages, geometry, style.line_height);
                    let frame = frame_for(&style, geometry.column_width());
                    pages[cursor.page_number - 1].push_line(PlacedTextLine {
                        top: cursor.y,
                        line: TextLine {
                            x: geometry.margin_left + frame.x,
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
                } else {
                    new_page(&mut cursor, &mut pages, geometry);
                }
                pending_spacing = 0.0;
            }
            Block::Paragraph(prepared) => {
                let paragraph = &prepared.text;
                if prepared.clear_fill_before {
                    pending_fill = None;
                }
                if paragraph.style.page_break_before
                    && (cursor.body_started || cursor.y > geometry.margin_top)
                {
                    new_page(&mut cursor, &mut pages, geometry);
                    pending_spacing = 0.0;
                    pending_fill = None;
                }
                let leading = pending_spacing.max(paragraph.style.spacing_before);
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
                    cursor,
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
                    |_| FlowArea::from(geometry),
                )?;
                for step in flow.steps {
                    if let FlowStep::Line(placement) = step {
                        while pages.len() < placement.page_number {
                            pages.push(TextPage::default());
                        }
                        let mut line = paragraph.lines[placement.line_index].clone();
                        let relative_frame = if placement.line_index == 0 {
                            paragraph.first_frame
                        } else {
                            paragraph.continuation_frame
                        };
                        let horizontal = placement.frame.x - relative_frame.x;
                        line.translate(horizontal, placement.top);
                        let page = &mut pages[placement.page_number - 1];
                        if !paragraph.is_empty {
                            if let Some(color) = &paragraph.style.fill {
                                if let Some((
                                    previous_page,
                                    previous_column,
                                    previous,
                                    spacing_after,
                                )) = pending_fill.take()
                                    && previous_page == placement.page_number
                                    && previous_column == placement.column_index
                                {
                                    let gap = placement.top - previous.top - previous.height;
                                    if gap > 0.001 && spacing_after > 0.0 {
                                        page.operations.push(PageOperation::Rectangle(
                                            HighlightRectangle {
                                                top: previous.top + previous.height,
                                                height: gap.min(spacing_after),
                                                ..previous
                                            },
                                        ));
                                    }
                                }
                                let rectangle = HighlightRectangle {
                                    x: placement.frame.x,
                                    top: placement.top,
                                    width: placement.frame.width,
                                    height: placement.height,
                                    color: color.clone(),
                                };
                                page.operations
                                    .push(PageOperation::Rectangle(rectangle.clone()));
                                pending_fill = Some((
                                    placement.page_number,
                                    placement.column_index,
                                    rectangle,
                                    paragraph.style.spacing_after,
                                ));
                            } else {
                                pending_fill = None;
                            }
                            if let Some(color) = &paragraph.border_color {
                                let bottom = placement.top + placement.height;
                                page.operations.push(PageOperation::Rule(Rule {
                                    start: [placement.frame.x, bottom],
                                    end: [placement.frame.x + placement.frame.width, bottom],
                                    width: 0.35,
                                    color: color.clone(),
                                }));
                            }
                        } else {
                            pending_fill = None;
                        }
                        if placement.line_index == 0
                            && let Some(prepared_marker) = &prepared.marker
                        {
                            let mut marker = prepared_marker.marker.clone();
                            let slot_left = placement.frame.x - prepared_marker.hanging;
                            marker.slot = [
                                slot_left,
                                placement.top,
                                prepared_marker.hanging.max(0.0),
                                placement.height,
                            ];
                            marker.baseline = line.baseline;
                            if let Some(text) = &mut marker.text {
                                let horizontal = if marker.kind == ListMarkerKind::Bullet {
                                    slot_left + text.width.min(marker.slot[2]) / 2.0
                                        - text.width / 2.0
                                } else {
                                    slot_left
                                };
                                text.translate(horizontal - text.x, line.baseline - text.baseline);
                            }
                            page.operations
                                .push(PageOperation::ListMarker(Box::new(marker)));
                        }
                        page.push_line(PlacedTextLine {
                            top: placement.top,
                            line,
                        });
                    }
                }
                cursor = flow.final_cursor;
                pending_spacing = paragraph.style.spacing_after.max(0.0);
                if prepared.clear_fill_after {
                    pending_fill = None;
                }
            }
        }
    }
    Ok(DocumentTextPlan { geometry, pages })
}
fn frame_for(style: &super::ParagraphStyle, width: f64) -> LineFrame {
    let available = (width - style.indent_left - style.indent_right).max(width.min(12.0));
    let origin = style.indent_left.min(width - available);
    LineFrame {
        x: origin + style.first_line,
        width: available - style.first_line,
    }
}
fn new_page(cursor: &mut FlowCursor, pages: &mut Vec<TextPage>, geometry: PageGeometry) {
    pages.push(TextPage::default());
    *cursor = FlowCursor {
        page_number: pages.len(),
        current_top: geometry.margin_top,
        y: geometry.margin_top,
        ..Default::default()
    };
}
fn ensure_space(
    cursor: &mut FlowCursor,
    pages: &mut Vec<TextPage>,
    geometry: PageGeometry,
    height: f64,
) {
    if cursor.y + height > geometry.content_bottom() {
        new_page(cursor, pages, geometry);
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
