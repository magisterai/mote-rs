use super::{HighlightRectangle, TextLine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleRole {
    Generic,
    Divider,
    ChartAxis,
    ChartGridline,
    ColumnSeparator,
    NoteSeparator,
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
    PushRotation {
        degrees: f64,
        center: [f64; 2],
    },
    PopRotation,
    Path(PaintedPath),
    PaintedRectangle {
        bounds: [f64; 4],
        fill: Option<PathPaint>,
        stroke: Option<PathStroke>,
    },
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

impl PageOperation {
    pub(super) fn translate(&mut self, [horizontal, vertical]: [f64; 2]) {
        let move_point = |point: &mut [f64; 2]| {
            point[0] += horizontal;
            point[1] += vertical;
        };
        let move_bounds = |bounds: &mut [f64; 4]| {
            bounds[0] += horizontal;
            bounds[1] += vertical;
        };
        match self {
            Self::PushRotation { center, .. } => move_point(center),
            Self::Path(path) => path.points.iter_mut().for_each(move_point),
            Self::PaintedRectangle { bounds, .. }
            | Self::OutlineRectangle { bounds, .. }
            | Self::PushClip(bounds) => move_bounds(bounds),
            Self::Stroke { rule, .. } | Self::Rule(rule) => {
                move_point(&mut rule.start);
                move_point(&mut rule.end);
            }
            Self::Image(image) => {
                move_bounds(&mut image.bounds);
                move_bounds(&mut image.clip);
            }
            Self::Rectangle(rectangle) => {
                rectangle.x += horizontal;
                rectangle.top += vertical;
            }
            Self::ListMarker(marker) => {
                move_bounds(&mut marker.slot);
                marker.baseline += vertical;
                if let Some(text) = &mut marker.text {
                    text.translate(horizontal, vertical);
                }
            }
            Self::Text(_) | Self::PopClip | Self::PopRotation => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathPaint {
    pub color: String,
    pub alpha: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineCap {
    #[default]
    Flat,
    Round,
    Square,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PathStroke {
    pub paint: PathPaint,
    pub width: f64,
    pub dash: Vec<f64>,
    pub cap: LineCap,
}
/// A painter-neutral polygon or open polyline in page coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct PaintedPath {
    pub points: Vec<[f64; 2]>,
    pub closed: bool,
    pub fill: Option<PathPaint>,
    pub stroke: Option<PathStroke>,
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

/// The MS Gothic ballot box glyphs, U+2610 and U+2612, that Word and PowerPoint
/// draw for a task marker, in thousandths of an em from the glyph origin with y
/// upward. The glyph advance is one em.
const UNCHECKED_TASK: &[&[[f64; 2]]] = &[
    &[[132.8125, 0.0], [132.8125, 734.375], [871.09375, 734.375], [871.09375, 0.0]],
    &[[171.875, 46.875], [828.125, 46.875], [828.125, 687.5], [171.875, 687.5]],
];
const CHECKED_TASK: &[&[[f64; 2]]] = &[
    &[
        [132.8125, -11.71875],
        [132.8125, 722.65625],
        [867.1875, 722.65625],
        [867.1875, -11.71875],
    ],
    &[
        [171.875, 35.15625],
        [824.21875, 35.15625],
        [824.21875, 675.78125],
        [171.875, 675.78125],
    ],
    &[
        [769.53125, 652.34375],
        [800.78125, 621.09375],
        [535.15625, 351.5625],
        [800.78125, 85.9375],
        [769.53125, 54.6875],
        [500.0, 320.3125],
        [234.375, 54.6875],
        [203.125, 85.9375],
        [468.75, 351.5625],
        [203.125, 621.09375],
        [234.375, 652.34375],
        [500.0, 386.71875],
    ],
];

impl ListMarker {
    /// Returns the closed contours of a task marker's glyph in page coordinates,
    /// to be filled with the even-odd rule. The glyph origin is the left edge of
    /// the slot on the marker's baseline.
    pub fn task_contours(&self) -> Vec<Vec<[f64; 2]>> {
        let scale = self.font_size / 1000.0;
        let glyph = if self.checked {
            CHECKED_TASK
        } else {
            UNCHECKED_TASK
        };
        glyph
            .iter()
            .map(|contour| {
                contour
                    .iter()
                    .map(|[x, y]| [self.slot[0] + x * scale, self.baseline - y * scale])
                    .collect()
            })
            .collect()
    }
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
            paint_borders(page, paragraph, &placement, horizontal);
        } else {
            self.pending_fill = None;
        }
        if placement.line_index == 0
            && let Some(prepared_marker) = &prepared.marker
        {
            let mut marker = prepared_marker.marker.clone();
            let slot_width = if prepared_marker.hanging == 0.0 {
                marker
                    .text
                    .as_ref()
                    .map_or(marker.font_size, |text| text.width.max(1.0))
            } else {
                prepared_marker.hanging
            };
            let slot_left = placement.frame.x - slot_width;
            marker.slot = if marker.kind == ListMarkerKind::Task {
                [line.x, placement.top, marker.font_size, placement.height]
            } else {
                [slot_left, placement.top, slot_width, placement.height]
            };
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

/// Side rules sit outside the text by their spacing; top and bottom rules occupy
/// the height that paragraph planning reserved for them.
fn paint_borders(
    page: &mut super::TextPage,
    paragraph: &super::paragraphs::ParagraphText,
    placement: &super::LinePlacement,
    horizontal: f64,
) {
    let borders = &paragraph.borders;
    let first = placement.line_index == 0;
    let last = placement.line_index + 1 == paragraph.lines.len();
    let left = horizontal + paragraph.continuation_frame.x;
    let right = left + paragraph.continuation_frame.width;
    let top = placement.top;
    let bottom = placement.top + placement.height;
    let mut rule = |start: [f64; 2], end: [f64; 2], side: &crate::layout::BorderSide| {
        page.operations.push(PageOperation::Rule(Rule {
            role: crate::layout::RuleRole::Generic,
            start,
            end,
            width: side.width,
            color: side.color.clone(),
        }));
    };
    let left_edge = left - borders.left.as_ref().map_or(0.0, |side| side.outset());
    let right_edge = right + borders.right.as_ref().map_or(0.0, |side| side.outset());
    if let Some(side) = &borders.left {
        let x = left - side.space - side.width / 2.0;
        rule([x, top], [x, bottom], side);
    }
    if let Some(side) = &borders.right {
        let x = right + side.space + side.width / 2.0;
        rule([x, top], [x, bottom], side);
    }
    if first && let Some(side) = &borders.top {
        let y = top + side.width / 2.0;
        rule([left_edge, y], [right_edge, y], side);
    }
    if last && let Some(side) = &borders.bottom {
        let y = bottom - side.width / 2.0;
        rule([left_edge, y], [right_edge, y], side);
    }
}
