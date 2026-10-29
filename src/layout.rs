//! Native text geometry shared by output painters.

mod inline;
mod lines;
mod style_values;
mod styles;
pub use inline::resolve_inline;
pub use styles::{LayoutStyleResolver, ParagraphStyle};
mod paragraphs;
mod placement;
mod wrapping;

use crate::measurement::{FontSelection, FontVariant, MeasurementError, MeasurementStyle};
pub use lines::plan_text;
pub use paragraphs::{LineFrame, ParagraphText, plan_paragraph_text};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    pub measurement: MeasurementStyle,
    pub word_line_height: Option<f64>,
    pub color: String,
    pub highlight: Option<String>,
    pub underline: bool,
    pub strike: bool,
    pub double_strike: bool,
    pub small_caps: bool,
    pub code: bool,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            measurement: MeasurementStyle {
                font: "Helvetica".into(),
                size_points: 11.0,
                variant: FontVariant::Regular,
            },
            word_line_height: None,
            color: "111827".into(),
            highlight: None,
            underline: false,
            strike: false,
            double_strike: false,
            small_caps: false,
            code: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextRun {
    pub text: String,
    pub style: TextStyle,
    pub preserve_space: bool,
    pub synthetic_tab: bool,
}

impl TextRun {
    pub fn new(text: impl Into<String>, style: TextStyle) -> Self {
        Self {
            text: text.into(),
            style,
            preserve_space: false,
            synthetic_tab: false,
        }
    }
    fn with_text(&self, text: &str) -> Self {
        Self {
            text: text.into(),
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineSpacing {
    Multiple(f64),
    Exact(f64),
    AtLeast(f64),
}

impl Default for LineSpacing {
    fn default() -> Self {
        Self::Multiple(1.15)
    }
}

impl LineSpacing {
    fn height(self, style: &TextStyle, fallback_multiplier: f64) -> f64 {
        let size = style.measurement.size_points;
        match self {
            Self::Multiple(multiple) => match style.word_line_height {
                Some(scale) if scale > 0.0 => size * scale * multiple,
                _ => size * 1.18 * multiple * 1.02,
            },
            Self::Exact(points) => points,
            Self::AtLeast(points) => points.max(
                size * style
                    .word_line_height
                    .filter(|scale| *scale > 0.0)
                    .unwrap_or(1.18)
                    * fallback_multiplier,
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineOptions {
    pub width: f64,
    pub first_indent: f64,
    pub alignment: Alignment,
    pub base_style: TextStyle,
    pub base_line_height: Option<f64>,
    pub spacing: LineSpacing,
    pub fallback_multiplier: f64,
    pub layout_kerning: bool,
    pub no_wrap: bool,
}

impl Default for LineOptions {
    fn default() -> Self {
        Self {
            width: 468.0,
            first_indent: 0.0,
            alignment: Alignment::Left,
            base_style: TextStyle::default(),
            base_line_height: None,
            spacing: LineSpacing::default(),
            fallback_multiplier: 1.15,
            layout_kerning: false,
            no_wrap: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineBreak {
    Final,
    Soft,
    Hard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextFragment {
    pub text: String,
    pub x: f64,
    pub baseline: f64,
    pub top: f64,
    pub height: f64,
    pub width: f64,
    pub advance: f64,
    pub style: TextStyle,
    pub font: FontSelection,
    pub missing_codepoints: Vec<char>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextLine {
    pub x: f64,
    pub baseline: f64,
    pub height: f64,
    pub width: f64,
    pub line_break: LineBreak,
    pub fragments: Vec<TextFragment>,
}

#[derive(Debug)]
pub enum LayoutError {
    InvalidGeometry(&'static str),
    Measurement(MeasurementError),
}

impl fmt::Display for LayoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidGeometry(message) => formatter.write_str(message),
            Self::Measurement(error) => error.fmt(formatter),
        }
    }
}
impl std::error::Error for LayoutError {}
impl From<MeasurementError> for LayoutError {
    fn from(error: MeasurementError) -> Self {
        Self::Measurement(error)
    }
}
