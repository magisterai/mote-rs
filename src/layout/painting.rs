use super::{HighlightRectangle, TextLine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleRole {
    Generic,
    ColumnSeparator,
    TableBorder,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub role: RuleRole,
    pub start: [f64; 2],
    pub end: [f64; 2],
    pub width: f64,
    pub color: String,
}

/// Ordered page operations refer to text lines stored on the same page.
#[derive(Debug, Clone, PartialEq)]
pub enum PageOperation {
    Text(usize),
    PushClip([f64; 4]),
    PopClip,
    Stroke {
        rule: Rule,
        dash: Vec<f64>,
        round_cap: bool,
    },
    Image(Box<super::PositionedImage>),
    OutlineRectangle {
        bounds: [f64; 4],
        width: f64,
        color: String,
    },
    Rectangle(HighlightRectangle),
    Rule(Rule),
    ListMarker(Box<ListMarker>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListMarkerKind {
    Bullet,
    Ordered,
    Task,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListMarker {
    pub kind: ListMarkerKind,
    pub checked: bool,
    pub slot: [f64; 4],
    pub baseline: f64,
    pub font_size: f64,
    pub text: Option<TextLine>,
}

#[derive(Default)]
pub(super) struct ParagraphPainting {
    pending_fill: Option<(usize, usize, HighlightRectangle, f64)>,
}
impl ParagraphPainting {
    pub fn clear_fill(&mut self) {
        self.pending_fill = None;
    }
    pub fn place_line(
        &mut self,
        page: &mut super::TextPage,
        prepared: &super::blocks::PreparedParagraph,
        placement: super::LinePlacement,
    ) {
        let paragraph = &prepared.text;
        if placement.line_index == 0 {
            page.place_anchors(&prepared.anchors, [placement.frame.x, placement.top]);
        }
        let mut line = paragraph.lines[placement.line_index].clone();
        let relative_frame = if placement.line_index == 0 {
            paragraph.first_frame
        } else {
            paragraph.continuation_frame
        };
        let horizontal = placement.frame.x - relative_frame.x;
        line.translate(horizontal, placement.top);
        if !paragraph.is_empty {
            if let Some(color) = &paragraph.style.fill {
                if let Some((previous_page, previous_column, previous, spacing_after)) =
                    self.pending_fill.take()
                    && previous_page == placement.page_number
                    && previous_column == placement.column_index
                {
                    let gap = placement.top - previous.top - previous.height;
                    if gap > 0.001 && spacing_after > 0.0 {
                        page.operations
                            .push(PageOperation::Rectangle(HighlightRectangle {
                                top: previous.top + previous.height,
                                height: gap.min(spacing_after),
                                ..previous
                            }));
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
                self.pending_fill = Some((
                    placement.page_number,
                    placement.column_index,
                    rectangle,
                    paragraph.style.spacing_after,
                ));
            } else {
                self.pending_fill = None;
            }
            if let Some(color) = &paragraph.border_color {
                let bottom = placement.top + placement.height;
                page.operations.push(PageOperation::Rule(Rule {
                    role: crate::layout::RuleRole::Generic,
                    start: [placement.frame.x, bottom],
                    end: [placement.frame.x + placement.frame.width, bottom],
                    width: 0.35,
                    color: color.clone(),
                }));
            }
        } else {
            self.pending_fill = None;
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
                    slot_left + text.width.min(marker.slot[2]) / 2.0 - text.width / 2.0
                } else {
                    slot_left
                };
                text.translate(horizontal - text.x, line.baseline - text.baseline);
            }
            page.operations
                .push(PageOperation::ListMarker(Box::new(marker)));
        }
        page.push_line(super::PlacedTextLine {
            top: placement.top,
            line,
        });
    }
}
