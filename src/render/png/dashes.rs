//! Bounded dash traversal shared by raster paths, rectangles and rules.
use super::RenderError;

// Bound work independently of page pixels: a tiny period can otherwise require
// arbitrarily many segments even on a short line.
const MAXIMUM_DASH_SEGMENTS: usize = 1_000_000;

pub(super) fn segments(
    length: f64,
    pattern: &[f64],
) -> Result<impl Iterator<Item = (f64, f64)> + '_, RenderError> {
    let period: f64 = pattern.iter().sum();
    let segment_limit = (length / period).ceil() * pattern.len() as f64;
    if !length.is_finite()
        || length < 0.0
        || !period.is_finite()
        || period <= 0.0
        || pattern
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
        || !segment_limit.is_finite()
        || segment_limit > MAXIMUM_DASH_SEGMENTS as f64
    {
        return Err(RenderError::Invalid(
            "PNG dash pattern exceeds the segment limit or has invalid geometry".into(),
        ));
    }
    let mut position = 0.0;
    let mut index = 0usize;
    Ok(std::iter::from_fn(move || {
        while position < length {
            let start = position;
            position = (position + pattern[index % pattern.len()]).min(length);
            let painted = index.is_multiple_of(2);
            index += 1;
            if painted && position > start {
                return Some((start, position));
            }
        }
        None
    }))
}
