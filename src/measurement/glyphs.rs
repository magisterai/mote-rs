use super::{MeasurementError, kerning::Kerning};
use ttf_parser::{Face, GlyphId};

pub(crate) struct GlyphPlacement {
    pub glyph: GlyphId,
    pub character: char,
    pub offset: f64,
}

/// Painter positions use the same per-glyph fixed-point rounding as measurements.
/// The supplied size is in points for PDF and device pixels for raster output.
pub(crate) fn placements(
    face: &Face<'_>,
    text: &str,
    size: f64,
) -> Result<Vec<GlyphPlacement>, MeasurementError> {
    let scale = (size * 64.0).round();
    let units = f64::from(face.units_per_em());
    let kerning = Kerning::new(face);
    let kerning_scale = if face.tables().cff.is_some() {
        scale
    } else {
        units
    };
    let mut position = 0.0;
    let mut previous = None;
    let mut output = Vec::new();
    for character in text.chars() {
        let glyph = face
            .glyph_index(character)
            .ok_or(MeasurementError::MissingReplacementGlyph)?;
        if let Some(previous) = previous {
            position +=
                (f64::from(kerning.adjustment(previous, glyph)) * kerning_scale / units).round();
        }
        output.push(GlyphPlacement {
            glyph,
            character,
            offset: position / 64.0,
        });
        let advance = face
            .glyph_hor_advance(glyph)
            .ok_or(MeasurementError::InvalidFont)?;
        position += (f64::from(advance) * scale / units).round();
        previous = Some(glyph);
    }
    Ok(output)
}
