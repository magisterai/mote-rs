use crate::media::{RasterImage, sampling};
use tiny_skia::{Mask, Pixmap};

pub(super) fn paint(
    destination: &mut Pixmap,
    layer: Pixmap,
    degrees: f64,
    center: [f64; 2],
    scale: f64,
    clip: Option<&Mask>,
) {
    let mut bounds = [layer.width(), layer.height(), 0, 0];
    for (index, pixel) in layer.data().as_chunks::<4>().0.iter().enumerate() {
        if pixel[3] == 0 {
            continue;
        }
        let column = index as u32 % layer.width();
        let row = index as u32 / layer.width();
        bounds[0] = bounds[0].min(column);
        bounds[1] = bounds[1].min(row);
        bounds[2] = bounds[2].max(column + 1);
        bounds[3] = bounds[3].max(row + 1);
    }
    if bounds[0] >= bounds[2] || bounds[1] >= bounds[3] {
        return;
    }
    let (sine, cosine) = degrees.to_radians().sin_cos();
    let center = center.map(|value| value * scale);
    let mut transformed = [
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    for [horizontal, vertical] in [
        [bounds[0], bounds[1]],
        [bounds[0], bounds[3]],
        [bounds[2], bounds[1]],
        [bounds[2], bounds[3]],
    ] {
        let horizontal = f64::from(horizontal) - center[0];
        let vertical = f64::from(vertical) - center[1];
        let point = [
            center[0] + horizontal * cosine - vertical * sine,
            center[1] + horizontal * sine + vertical * cosine,
        ];
        for axis in 0..2 {
            transformed[axis] = transformed[axis].min(point[axis]);
            transformed[axis + 2] = transformed[axis + 2].max(point[axis]);
        }
    }
    let source = RasterImage {
        width: layer.width(),
        height: layer.height(),
        pixels: layer.take().into(),
    };
    let left = (transformed[0] - 3.0).floor().max(0.0) as u32;
    let top = (transformed[1] - 3.0).floor().max(0.0) as u32;
    let right = (transformed[2] + 3.0)
        .ceil()
        .min(f64::from(destination.width())) as u32;
    let bottom = (transformed[3] + 3.0)
        .ceil()
        .min(f64::from(destination.height())) as u32;
    for row in top..bottom {
        for column in left..right {
            let offset = (row as usize * destination.width() as usize + column as usize) * 4;
            if clip.is_some_and(|mask| mask.data()[offset / 4] == 0) {
                continue;
            }
            let horizontal = f64::from(column) + 0.5 - center[0];
            let vertical = f64::from(row) + 0.5 - center[1];
            let source_horizontal = center[0] + horizontal * cosine + vertical * sine - 0.5;
            let source_vertical = center[1] - horizontal * sine + vertical * cosine - 0.5;
            let horizontal = sampling::weights(source_horizontal, 1.0, 0, source.width);
            let vertical = sampling::weights(source_vertical, 1.0, 0, source.height);
            let sampled = sampling::sample_premultiplied(&source, &horizontal, &vertical);
            let target = &mut destination.data_mut()[offset..offset + 4];
            for channel in 0..4 {
                target[channel] = ((u32::from(sampled[channel])
                    + (65535 - u32::from(sampled[3])) * u32::from(target[channel]) / 255)
                    .min(65535)
                    >> 8) as u8;
            }
        }
    }
}
