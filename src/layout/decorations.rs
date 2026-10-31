use super::{LineOptions, LineSpacing, TextFragment, style_values};

#[derive(Debug, Clone, PartialEq)]
pub struct HighlightRectangle {
    pub x: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecorationKind {
    Underline,
    Strike,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TextDecoration {
    pub kind: DecorationKind,
    pub start: f64,
    pub end: f64,
    pub y: f64,
    pub width: f64,
    pub color: String,
    pub dash: Vec<f64>,
}

pub(super) fn plan(
    fragments: &[TextFragment],
    height: f64,
    base_height: f64,
    options: &LineOptions,
) -> (Vec<HighlightRectangle>, Vec<TextDecoration>) {
    let mut highlights = Vec::new();
    let mut underlines = Vec::new();
    let mut second_underlines = Vec::new();
    let mut strikes = Vec::new();
    for fragment in fragments {
        let style = &fragment.style;
        let size = style.measurement.size_points;
        let band_height = match options.spacing {
            LineSpacing::Exact(_) => base_height,
            _ if style.subscript || style.superscript => base_height,
            _ => options.spacing.height(style, options.fallback_multiplier),
        }
        .min(height);
        let band_top = fragment.baseline - 0.5 * band_height - 0.3 * size;
        if let Some(color) = style
            .highlight
            .as_deref()
            .and_then(style_values::parse_color)
        {
            let (top, fill_height) = if band_height > 2.0 {
                (band_top + 1.0, band_height - 2.0)
            } else {
                (band_top, band_height)
            };
            if fill_height > 0.0 {
                append_highlight(
                    &mut highlights,
                    HighlightRectangle {
                        x: fragment.x,
                        top,
                        width: fragment.advance,
                        height: fill_height,
                        color,
                    },
                );
            }
        }
        let color = style_values::parse_color(&style.color).unwrap_or_else(|| "111827".into());
        if style.underline {
            let double = style.underline_style == "double";
            let width = if double {
                (size * 0.025).max(0.35)
            } else {
                size * 0.05
            };
            let decoration = TextDecoration {
                kind: DecorationKind::Underline,
                start: fragment.x,
                end: fragment.x + fragment.advance,
                y: fragment.baseline + size * if double { 0.075 } else { 0.1 },
                width,
                color: style_values::parse_color(&style.underline_color)
                    .unwrap_or_else(|| color.clone()),
                dash: match style.underline_style.as_str() {
                    "dashed" => vec![3.0 * width, 3.0 * width],
                    "dotted" => vec![width, 2.0 * width],
                    _ => Vec::new(),
                },
            };
            append_decoration(&mut underlines, decoration.clone());
            if double {
                append_decoration(
                    &mut second_underlines,
                    TextDecoration {
                        y: decoration.y + (size * 0.08).max(1.1),
                        ..decoration
                    },
                );
            }
        }
        if style.strike {
            let decoration = TextDecoration {
                kind: DecorationKind::Strike,
                start: fragment.x,
                end: fragment.x + fragment.advance,
                y: band_top + band_height * 0.55,
                width: 0.2,
                color,
                dash: Vec::new(),
            };
            append_decoration(&mut strikes, decoration.clone());
            if style.double_strike {
                append_decoration(
                    &mut strikes,
                    TextDecoration {
                        y: decoration.y + (size * 0.08).max(1.0),
                        ..decoration
                    },
                );
            }
        }
    }
    underlines.extend(second_underlines);
    underlines.extend(strikes);
    (highlights, underlines)
}
fn append_highlight(rectangles: &mut Vec<HighlightRectangle>, rectangle: HighlightRectangle) {
    if let Some(previous) = rectangles.last_mut()
        && previous.color == rectangle.color
        && (previous.x + previous.width - rectangle.x).abs() < 0.001
        && (previous.top - rectangle.top).abs() < 0.001
        && (previous.height - rectangle.height).abs() < 0.001
    {
        previous.width += rectangle.width;
        return;
    }
    rectangles.push(rectangle);
}
fn append_decoration(decorations: &mut Vec<TextDecoration>, decoration: TextDecoration) {
    if let Some(previous) = decorations.last_mut()
        && previous.kind == decoration.kind
        && previous.color == decoration.color
        && previous.width == decoration.width
        && previous.dash == decoration.dash
        && (previous.end - decoration.start).abs() < 0.001
        && (previous.y - decoration.y).abs() < 0.001
    {
        previous.end = decoration.end;
        return;
    }
    decorations.push(decoration);
}
