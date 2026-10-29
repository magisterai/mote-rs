//! Separable Catmull-Rom reconstruction in premultiplied color space.
use super::RasterImage;

pub(crate) fn weights(center: f64, scale: f64, start: u32, end: u32) -> Vec<(u32, f64)> {
    let support = 2.0 * scale.max(1.0);
    let first = (center - support).floor().max(f64::from(start)) as u32;
    let last = (center + support).ceil().min(f64::from(end)) as u32;
    let mut result = Vec::new();
    let mut total = 0.0;
    for coordinate in first..last {
        let distance = ((center - f64::from(coordinate)) / scale.max(1.0)).abs();
        let weight = if distance < 1.0 {
            (1.5 * distance - 2.5) * distance * distance + 1.0
        } else if distance < 2.0 {
            ((-0.5 * distance + 2.5) * distance - 4.0) * distance + 2.0
        } else {
            0.0
        };
        if weight != 0.0 {
            result.push((coordinate, weight));
            total += weight;
        }
    }
    if total != 0.0 {
        for (_, weight) in &mut result {
            *weight /= total;
        }
    }
    result
}
pub(crate) fn sample(
    source: &RasterImage,
    horizontal: &[(u32, f64)],
    vertical: &[(u32, f64)],
) -> [u16; 4] {
    let mut output = [0.0; 4];
    for &(row, row_weight) in vertical {
        let mut intermediate = [0.0; 4];
        for &(column, column_weight) in horizontal {
            let offset = (row as usize * source.width as usize + column as usize) * 4;
            let pixel = &source.pixels[offset..offset + 4];
            let alpha = u32::from(pixel[3]) * 257;
            for channel in 0..3 {
                // Straight bytes are converted through the 16-bit color contract.
                let premultiplied = u32::from(pixel[channel]) * 257 * alpha / 65535;
                intermediate[channel] += f64::from(premultiplied) * column_weight;
            }
            intermediate[3] += f64::from(alpha) * column_weight;
        }
        for channel in 0..4 {
            output[channel] += intermediate[channel] * row_weight;
        }
    }
    let alpha = output[3].round().clamp(0.0, 65535.0) as u16;
    [
        output[0].round().clamp(0.0, f64::from(alpha)) as u16,
        output[1].round().clamp(0.0, f64::from(alpha)) as u16,
        output[2].round().clamp(0.0, f64::from(alpha)) as u16,
        alpha,
    ]
}
pub(crate) fn straight(pixel: [u16; 4]) -> [u8; 4] {
    if pixel[3] == 0 {
        return [0; 4];
    }
    let alpha = u32::from(pixel[3]);
    [
        ((u32::from(pixel[0]) * 65535 / alpha) >> 8) as u8,
        ((u32::from(pixel[1]) * 65535 / alpha) >> 8) as u8,
        ((u32::from(pixel[2]) * 65535 / alpha) >> 8) as u8,
        (alpha >> 8) as u8,
    ]
}
pub(crate) fn scale(
    source: &RasterImage,
    bounds: [u32; 4],
    width: u32,
    height: u32,
) -> RasterImage {
    let [left, top, right, bottom] = bounds;
    let horizontal_scale = f64::from(right - left) / f64::from(width);
    let vertical_scale = f64::from(bottom - top) / f64::from(height);
    let horizontal: Vec<_> = (0..width)
        .map(|column| {
            weights(
                f64::from(left) + (f64::from(column) + 0.5) * horizontal_scale - 0.5,
                horizontal_scale,
                left,
                right,
            )
        })
        .collect();
    let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
    for row in 0..height {
        let vertical = weights(
            f64::from(top) + (f64::from(row) + 0.5) * vertical_scale - 0.5,
            vertical_scale,
            top,
            bottom,
        );
        for contributions in &horizontal {
            pixels.extend(straight(sample(source, contributions, &vertical)));
        }
    }
    RasterImage {
        width,
        height,
        pixels: pixels.into(),
    }
}
