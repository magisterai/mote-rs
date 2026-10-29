use crate::{layout::PositionedImage, media::sampling};
use tiny_skia::Pixmap;

pub(super) fn paint(pixels: &mut Pixmap, image: &PositionedImage, scale: f64) {
    let raster = &image.image.raster;
    let left = (image.bounds[0] * scale).round() as i64;
    let top = (image.bounds[1] * scale).round() as i64;
    let right = ((image.bounds[0] + image.bounds[2]) * scale).round() as i64;
    let bottom = ((image.bounds[1] + image.bounds[3]) * scale).round() as i64;
    if right <= left || bottom <= top {
        return;
    }
    let clip_left = (image.clip[0] * scale).round() as i64;
    let clip_top = (image.clip[1] * scale).round() as i64;
    let clip_right = ((image.clip[0] + image.clip[2]) * scale).round() as i64;
    let clip_bottom = ((image.clip[1] + image.clip[3]) * scale).round() as i64;
    let start_column = left.max(clip_left).max(0);
    let end_column = right.min(clip_right).min(i64::from(pixels.width()));
    let start_row = top.max(clip_top).max(0);
    let end_row = bottom.min(clip_bottom).min(i64::from(pixels.height()));
    let horizontal_scale = f64::from(raster.width) / (right - left) as f64;
    let vertical_scale = f64::from(raster.height) / (bottom - top) as f64;
    let horizontal: Vec<_> = (start_column..end_column)
        .map(|column| {
            sampling::weights(
                ((column - left) as f64 + 0.5) * horizontal_scale - 0.5,
                horizontal_scale,
                0,
                raster.width,
            )
        })
        .collect();
    let width = pixels.width() as usize;
    for row in start_row..end_row {
        let vertical = sampling::weights(
            ((row - top) as f64 + 0.5) * vertical_scale - 0.5,
            vertical_scale,
            0,
            raster.height,
        );
        for (column, contributions) in (start_column..end_column).zip(&horizontal) {
            let sampled = sampling::sample(raster, contributions, &vertical);
            let offset = (row as usize * width + column as usize) * 4;
            let pixel = &mut pixels.data_mut()[offset..offset + 4];
            for channel in 0..3 {
                pixel[channel] = ((u32::from(sampled[channel])
                    + (65535 - u32::from(sampled[3])) * u32::from(pixel[channel]) / 255)
                    .min(65535)
                    >> 8) as u8;
            }
            pixel[3] = 255;
        }
    }
}
