use super::{
    FontExtents, MeasurementError, TextMeasurement, kerning::Kerning, measured_size, prepare_text,
};
use ttf_parser::{Face, Tag};

/// A borrowed, static TrueType or OpenType/CFF face. The caller retains the exact
/// font bytes, allowing a painter to use the same data as the measurer.
pub struct OpenTypeFont<'font> {
    face: Face<'font>,
    kerning: Kerning<'font>,
    compact_outlines: bool,
}

impl<'font> OpenTypeFont<'font> {
    pub fn parse(bytes: &'font [u8]) -> Result<Self, MeasurementError> {
        if bytes.starts_with(b"ttcf") {
            return Err(MeasurementError::UnsupportedFont(
                "font collections are unsupported",
            ));
        }
        let face = Face::parse(bytes, 0).map_err(|_| MeasurementError::InvalidFont)?;
        for tag in [b"fvar", b"CFF2"] {
            if face.raw_face().table(Tag::from_bytes(tag)).is_some() {
                return Err(MeasurementError::UnsupportedFont(
                    "variable and CFF2 fonts are unsupported",
                ));
            }
        }
        if face.glyph_index('?').is_none() {
            return Err(MeasurementError::MissingReplacementGlyph);
        }
        let kerning = Kerning::new(&face);
        Ok(Self {
            face,
            kerning,
            compact_outlines: bytes.starts_with(b"OTTO"),
        })
    }

    pub fn measure(
        &self,
        text: &str,
        size_points: f64,
        layout_kerning: bool,
    ) -> Result<TextMeasurement, MeasurementError> {
        let size_points = measured_size(size_points)?;
        let scale = (size_points * 64.0 + 0.5) as i64;
        let units_per_em = i64::from(self.face.units_per_em());
        let (prepared_text, missing_codepoints) =
            prepare_text(text, |character| self.face.glyph_index(character).is_some());
        let mut advances = 0_i64;
        let mut adjustments = 0_i64;
        let mut previous = None;
        for character in prepared_text.chars() {
            let glyph = self
                .face
                .glyph_index(character)
                .ok_or(MeasurementError::MissingReplacementGlyph)?;
            let advance = self
                .face
                .glyph_hor_advance(glyph)
                .ok_or(MeasurementError::InvalidFont)?;
            advances += scaled_units(i64::from(advance), scale, units_per_em);
            if let Some(previous) = previous {
                // Keep the captured TrueType adjustment scale explicit. CFF uses
                // the requested point scale; both paths round each pair separately.
                let kerning_scale = if self.compact_outlines {
                    scale
                } else {
                    units_per_em
                };
                adjustments += scaled_units(
                    i64::from(self.kerning.adjustment(previous, glyph)),
                    kerning_scale,
                    units_per_em,
                );
            }
            previous = Some(glyph);
        }
        let header = self.face.tables().hhea;
        Ok(TextMeasurement {
            prepared_text,
            missing_codepoints,
            layout_width: (advances + if layout_kerning { adjustments } else { 0 }) as f64 / 64.0,
            paint_width: (advances + adjustments) as f64 / 64.0,
            extents: FontExtents {
                ascent: scaled_units(header.ascender.into(), scale, units_per_em) as f64 / 64.0,
                descent: -scaled_units(header.descender.into(), scale, units_per_em) as f64 / 64.0,
                height: scaled_units(
                    i64::from(header.ascender) - i64::from(header.descender)
                        + i64::from(header.line_gap),
                    scale,
                    units_per_em,
                ) as f64
                    / 64.0,
            },
        })
    }
}

fn scaled_units(value: i64, scale: i64, units_per_em: i64) -> i64 {
    let product = value * scale;
    let rounding = if product >= 0 {
        units_per_em / 2
    } else {
        -units_per_em / 2
    };
    (product + rounding) / units_per_em
}
