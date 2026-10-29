use super::{
    Alignment, LayoutError, LayoutStyleResolver, LineOptions, ParagraphStyle, TextLine, plan_text,
    resolve_inline,
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
#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphText {
    pub style: ParagraphStyle,
    pub is_empty: bool,
    pub first_frame: LineFrame,
    pub continuation_frame: LineFrame,
    /// Line geometry is paragraph-relative; vertical positions are not stacked.
    pub lines: Vec<TextLine>,
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
    let width =
        (container_width - style.indent_left - style.indent_right).max(container_width.min(12.0));
    let origin = style.indent_left.min(container_width - width);
    let continuation_frame = LineFrame { x: origin, width };
    let first_frame = LineFrame {
        x: origin + style.first_line,
        width: width - style.first_line,
    };
    let options = LineOptions {
        width,
        first_indent: style.first_line,
        base_style: style.text.clone(),
        base_line_height: Some(style.line_height),
        spacing: style.line_spacing,
        fallback_multiplier: style.line_multiplier,
        layout_kerning,
        alignment: match style.alignment.as_str() {
            "center" => Alignment::Center,
            "right" => Alignment::Right,
            "justify" => Alignment::Justify,
            _ => Alignment::Left,
        },
        ..Default::default()
    };
    let runs = resolve_inline(&element.children, &style.text, styles);
    let mut lines = plan_text(&runs, &options, measurer)?;
    for line in &mut lines {
        line.x += origin;
        for fragment in &mut line.fragments {
            fragment.x += origin;
        }
    }
    Ok(ParagraphText {
        is_empty: runs.is_empty(),
        style,
        first_frame,
        continuation_frame,
        lines,
    })
}
