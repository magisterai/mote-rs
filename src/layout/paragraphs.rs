use super::{
    Alignment, LayoutError, LayoutStyleResolver, LineOptions, ParagraphBorders, ParagraphStyle,
    TextLine, TextRun, plan_text, resolve_inline,
};
use crate::{
    measurement::TextMeasurer,
    model::{Element, ElementKind},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineFrame {
    pub x: f64,
    pub width: f64,
}
/// The frames of the lines of a paragraph, relative to its container.
pub(super) struct LineFrames {
    pub first: LineFrame,
    pub continuation: LineFrame,
}
impl LineFrames {
    /// Returns the frames that the indents of `style` leave in a container of
    /// the given width. The frames keep a least width of twelve points, or
    /// the width of a narrower container.
    pub(super) fn within(style: &ParagraphStyle, container_width: f64) -> Self {
        let width = (container_width - style.indent_left - style.indent_right)
            .max(container_width.min(12.0));
        let origin = style.indent_left.min(container_width - width);
        Self {
            first: LineFrame {
                x: origin + style.first_line,
                width: width - style.first_line,
            },
            continuation: LineFrame { x: origin, width },
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphText {
    pub style: ParagraphStyle,
    pub borders: ParagraphBorders,
    pub is_empty: bool,
    pub first_frame: LineFrame,
    pub continuation_frame: LineFrame,
    /// Line geometry is paragraph-relative; vertical positions are not stacked.
    pub lines: Vec<TextLine>,
}

impl ParagraphText {
    /// Returns the box of the paragraph, which its shading and its borders
    /// follow on every line: the box begins at the first line or at the
    /// continuation lines, whichever begin further left, and ends at the right
    /// indent.
    pub fn box_frame(&self) -> LineFrame {
        let left = self.first_frame.x.min(self.continuation_frame.x);
        LineFrame {
            x: left,
            width: self.continuation_frame.x + self.continuation_frame.width - left,
        }
    }
    /// Returns the frame of a line of the paragraph, relative to its
    /// container.
    pub(super) fn line_frame(&self, line_index: usize) -> LineFrame {
        if line_index == 0 {
            self.first_frame
        } else {
            self.continuation_frame
        }
    }
}

/// Resolve and measure one paragraph or heading before pagination and decoration.
/// Borders, tab stops, numbering and page requests require the document planner.
pub fn plan_paragraph_text(
    element: &Element,
    container_width: f64,
    styles: &LayoutStyleResolver<'_>,
    measurer: &mut TextMeasurer<'_>,
    layout_kerning: bool,
) -> Result<ParagraphText, LayoutError> {
    if !matches!(element.kind, ElementKind::Paragraph | ElementKind::Heading) {
        return Err(LayoutError::InvalidGeometry(
            "paragraph text planning requires a paragraph or heading",
        ));
    }
    if !container_width.is_finite() || container_width <= 0.0 {
        return Err(LayoutError::InvalidGeometry(
            "paragraph width must be finite and positive",
        ));
    }
    let style = styles.paragraph(element);
    let runs = resolve_inline(&element.children, &style.text, styles);
    plan_resolved_paragraph(
        style,
        styles.borders(element),
        &runs,
        container_width,
        measurer,
        layout_kerning,
        true,
    )
}

pub(super) fn plan_resolved_paragraph(
    style: ParagraphStyle,
    borders: ParagraphBorders,
    runs: &[TextRun],
    container_width: f64,
    measurer: &mut TextMeasurer<'_>,
    layout_kerning: bool,
    resolve_tabs: bool,
) -> Result<ParagraphText, LayoutError> {
    let LineFrames {
        first: first_frame,
        continuation: continuation_frame,
    } = LineFrames::within(&style, container_width);
    let LineFrame { x: origin, width } = continuation_frame;
    let options = LineOptions {
        width,
        first_indent: style.first_line,
        base_style: style.text.clone(),
        base_line_height: Some(style.line_height),
        spacing: style.line_spacing,
        layout_kerning,
        alignment: match style.alignment.as_str() {
            "center" => Alignment::Center,
            "right" => Alignment::Right,
            "justify" => Alignment::Justify,
            _ => Alignment::Left,
        },
        tab_stops: resolve_tabs.then(|| style.tab_stops.clone()),
        tab_origin: origin,
        word_exact_baseline: style.word_exact_baseline,
        whole_point_baseline: style.whole_point_baseline,
        ..Default::default()
    };
    let mut lines = plan_text(runs, &options, measurer)?;
    for line in &mut lines {
        line.translate(origin, 0.0);
    }
    let top = borders.top_outset();
    if let Some(first) = lines.first_mut() {
        first.translate(0.0, top);
        first.height += top;
    }
    if let Some(last) = lines.last_mut() {
        last.height += borders.bottom_outset();
    }
    Ok(ParagraphText {
        borders,
        is_empty: runs.is_empty(),
        style,
        first_frame,
        continuation_frame,
        lines,
    })
}
