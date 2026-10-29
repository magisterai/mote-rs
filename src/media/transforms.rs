use super::{PreparedImage, RasterImage, boolean, number, object, sampling};
use crate::model::Properties;

pub(super) fn is_identity(attributes: &Properties) -> bool {
    let crop = object(attributes, "crop").is_none_or(|crop| {
        ["left", "top", "right", "bottom"]
            .iter()
            .all(|name| percent(crop, name, 0.0, 100.0) == 0.0)
    });
    let effects = object(attributes, "effects").is_none_or(|effects| {
        !boolean(effects, "grayscale")
            && percent(effects, "brightness", -100.0, 100.0) == 0.0
            && percent(effects, "contrast", -100.0, 100.0) == 0.0
            && percent(effects, "transparency", 0.0, 100.0) == 0.0
    });
    crop && effects
        && !boolean(attributes, "flip_horizontal")
        && !boolean(attributes, "flip_vertical")
        && (number(attributes, "rotation").unwrap_or(0.0) * 60000.0)
            .round()
            .rem_euclid(21600000.0)
            == 0.0
}

pub(super) fn prepare(
    source: &RasterImage,
    attributes: &Properties,
) -> Result<PreparedImage, String> {
    let mut raster = source.clone();
    if let Some(crop) = object(attributes, "crop") {
        let side = |name: &str, limit: u32| {
            (f64::from(limit) * percent(crop, name, 0.0, 100.0) / 100.0).round() as u32
        };
        let horizontal = interval(
            side("left", source.width),
            source.width - side("right", source.width),
            source.width,
        );
        let vertical = interval(
            side("top", source.height),
            source.height - side("bottom", source.height),
            source.height,
        );
        if horizontal != [0, source.width] || vertical != [0, source.height] {
            raster = sampling::scale(
                source,
                [horizontal[0], vertical[0], horizontal[1], vertical[1]],
                source.width,
                source.height,
            );
        }
    }
    let horizontal = boolean(attributes, "flip_horizontal");
    let vertical = boolean(attributes, "flip_vertical");
    if horizontal || vertical {
        let mut pixels = vec![0; raster.pixels.len()];
        for row in 0..raster.height {
            for column in 0..raster.width {
                let source_row = if vertical {
                    raster.height - 1 - row
                } else {
                    row
                };
                let source_column = if horizontal {
                    raster.width - 1 - column
                } else {
                    column
                };
                let source_offset =
                    (source_row as usize * raster.width as usize + source_column as usize) * 4;
                let target_offset = (row as usize * raster.width as usize + column as usize) * 4;
                pixels[target_offset..target_offset + 4]
                    .copy_from_slice(&raster.pixels[source_offset..source_offset + 4]);
            }
        }
        raster.pixels = pixels.into();
    }
    if let Some(effects) = object(attributes, "effects") {
        let brightness = percent(effects, "brightness", -100.0, 100.0) / 100.0;
        let contrast = percent(effects, "contrast", -100.0, 100.0) / 100.0;
        let alpha_scale = 1.0 - percent(effects, "transparency", 0.0, 100.0) / 100.0;
        let mut pixels = raster.pixels.to_vec();
        for pixel in pixels.as_chunks_mut::<4>().0 {
            let mut channels = [
                effect(f64::from(pixel[0]) / 255.0, brightness, contrast),
                effect(f64::from(pixel[1]) / 255.0, brightness, contrast),
                effect(f64::from(pixel[2]) / 255.0, brightness, contrast),
            ];
            if boolean(effects, "grayscale") {
                channels = [0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2]; 3];
            }
            for channel in 0..3 {
                pixel[channel] = (channels[channel].clamp(0.0, 1.0) * 255.0).round() as u8;
            }
            pixel[3] = (f64::from(pixel[3]) * alpha_scale).round() as u8;
        }
        raster.pixels = pixels.into();
    }
    let degrees = (number(attributes, "rotation").unwrap_or(0.0) * 60000.0).round() / 60000.0;
    let degrees = degrees.rem_euclid(360.0);
    let mut offset = [0.0; 2];
    if degrees > 0.0000001 && (degrees - 360.0).abs() > 0.0000001 {
        let radians = degrees.to_radians();
        let cosine = radians.cos();
        let sine = radians.sin();
        let width = (f64::from(source.width) * cosine.abs() + f64::from(source.height) * sine.abs())
            .ceil() as u32
            + 4;
        let height = (f64::from(source.width) * sine.abs()
            + f64::from(source.height) * cosine.abs())
        .ceil() as u32
            + 4;
        if u64::from(width) * u64::from(height) > 64_000_000 {
            return Err("rotated image exceeds the pixel allocation limit".into());
        }
        offset = [
            (f64::from(width) - f64::from(source.width)) / 2.0,
            (f64::from(height) - f64::from(source.height)) / 2.0,
        ];
        let mut pixels = vec![0; width as usize * height as usize * 4];
        for row in 0..height {
            for column in 0..width {
                let horizontal = f64::from(column) + 0.5 - f64::from(width) / 2.0;
                let vertical = f64::from(row) + 0.5 - f64::from(height) / 2.0;
                let source_horizontal =
                    cosine * horizontal + sine * vertical + f64::from(source.width) / 2.0;
                let source_vertical =
                    -sine * horizontal + cosine * vertical + f64::from(source.height) / 2.0;
                if source_horizontal < 0.0
                    || source_vertical < 0.0
                    || source_horizontal >= f64::from(source.width)
                    || source_vertical >= f64::from(source.height)
                {
                    continue;
                }
                let horizontal = sampling::weights(source_horizontal - 0.5, 1.0, 0, source.width);
                let vertical = sampling::weights(source_vertical - 0.5, 1.0, 0, source.height);
                let pixel = sampling::straight(sampling::sample(&raster, &horizontal, &vertical));
                let target = (row as usize * width as usize + column as usize) * 4;
                pixels[target..target + 4].copy_from_slice(&pixel);
            }
        }
        raster = RasterImage {
            width,
            height,
            pixels: pixels.into(),
        };
    }
    Ok(PreparedImage {
        raster_width: raster.width,
        raster_height: raster.height,
        raster,
        original_width: source.width,
        original_height: source.height,
        offset,
        jpeg: None,
    })
}
fn interval(start: u32, end: u32, limit: u32) -> [u32; 2] {
    if start < end {
        [start, end]
    } else if limit <= 1 {
        [0, limit]
    } else {
        [limit / 2, limit / 2 + 1]
    }
}
fn percent(properties: &Properties, name: &str, low: f64, high: f64) -> f64 {
    ((number(properties, name).unwrap_or(0.0) * 1000.0).round() / 1000.0).clamp(low, high)
}
fn effect(mut value: f64, brightness: f64, contrast: f64) -> f64 {
    value = if contrast >= 1.0 {
        if value < 0.5 { 0.0 } else { 1.0 }
    } else if contrast >= 0.0 {
        (value - 0.5) / (1.0 - contrast) + 0.5
    } else {
        (value - 0.5) * (1.0 + contrast) + 0.5
    };
    if brightness >= 0.0 {
        value += (1.0 - value) * brightness;
    } else {
        value *= 1.0 + brightness;
    }
    value.clamp(0.0, 1.0)
}
