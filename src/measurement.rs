//! Font metrics shared by native layout and painting, expressed in points.
//!
//! This module measures character advances and pair kerning. It does not shape
//! text, choose fonts from the operating system, or decide where lines break.

mod core;
pub(crate) use core::win_ansi;
pub(crate) mod glyphs;
mod kerning;
mod opentype;
mod resolution;

pub use core::CoreFont;
pub use opentype::OpenTypeFont;
pub use resolution::{FontSelection, FontSource, MeasuredText, MeasurementStyle, TextMeasurer};

use std::{collections::BTreeSet, fmt};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontVariant {
    #[default]
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl FontVariant {
    pub fn as_text(self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::BoldItalic => "bold_italic",
        }
    }

    pub fn from_emphasis(bold: bool, italic: bool) -> Self {
        match (bold, italic) {
            (false, false) => Self::Regular,
            (true, false) => Self::Bold,
            (false, true) => Self::Italic,
            (true, true) => Self::BoldItalic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontExtents {
    pub ascent: f64,
    /// Positive distance below the baseline.
    pub descent: f64,
    /// Natural font height including the font's line gap.
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextMeasurement {
    pub prepared_text: String,
    /// Unique missing characters in order of first occurrence.
    pub missing_codepoints: Vec<char>,
    pub layout_width: f64,
    pub paint_width: f64,
    pub extents: FontExtents,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementError {
    InvalidSize,
    InvalidFont,
    UnsupportedFont(&'static str),
    MissingReplacementGlyph,
}

impl fmt::Display for MeasurementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidSize => "font size is non-finite or exceeds the measurement range",
            Self::InvalidFont => "font data could not be parsed as OpenType",
            Self::UnsupportedFont(reason) => reason,
            Self::MissingReplacementGlyph => "replacement glyph U+003F is missing",
        })
    }
}

impl std::error::Error for MeasurementError {}

fn measured_size(size_points: f64) -> Result<f64, MeasurementError> {
    if !size_points.is_finite() || size_points * 64.0 > f64::from(i32::MAX) {
        return Err(MeasurementError::InvalidSize);
    }
    Ok(if size_points <= 0.0 {
        11.0
    } else {
        size_points
    })
}

fn prepare_text(text: &str, supported: impl Fn(char) -> bool) -> (String, Vec<char>) {
    let mut prepared = String::with_capacity(text.len());
    let mut missing = Vec::new();
    let mut seen = BTreeSet::new();
    for character in text.chars() {
        if supported(character) {
            prepared.push(character);
        } else {
            prepared.push('?');
            if seen.insert(character) {
                missing.push(character);
            }
        }
    }
    (prepared, missing)
}
