use super::{ImageAsset, RasterImage};
use std::{io::Cursor, sync::Arc};
const MAXIMUM_PIXELS: u64 = 64_000_000;
fn dimensions(width: u32, height: u32) -> Result<usize, String> {
    if width == 0 || height == 0 || u64::from(width) * u64::from(height) > MAXIMUM_PIXELS {
        return Err("image exceeds the 64-million-pixel allocation limit".into());
    }
    Ok(width as usize * height as usize * 4)
}
pub(super) fn decode(bytes: Vec<u8>, source: &str) -> Result<ImageAsset, String> {
    let mut jpeg_components = None;
    let (raster, extension, content_type) = if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        (decode_png(&bytes)?, ".png", "image/png")
    } else if bytes.starts_with(b"\xff\xd8") {
        let (raster, components) = decode_jpeg(&bytes)?;
        jpeg_components = Some(components);
        (
            raster,
            if source.to_lowercase().ends_with(".jpeg") {
                ".jpeg"
            } else {
                ".jpg"
            },
            "image/jpeg",
        )
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        (decode_gif(&bytes)?, ".gif", "image/gif")
    } else {
        return Err(format!("image {source:?} has unsupported format"));
    };
    let (natural_width, natural_height) = if content_type == "image/gif" {
        (
            u32::from(u16::from_le_bytes([bytes[6], bytes[7]])),
            u32::from(u16::from_le_bytes([bytes[8], bytes[9]])),
        )
    } else {
        (raster.width, raster.height)
    };
    Ok(ImageAsset {
        original: bytes.into(),
        extension,
        content_type,
        raster,
        natural_width,
        natural_height,
        jpeg_components,
    })
}
fn decode_png(bytes: &[u8]) -> Result<RasterImage, String> {
    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder
        .read_info()
        .map_err(|error| format!("PNG image header: {error}"))?;
    let capacity = dimensions(reader.info().width, reader.info().height)?;
    let mut buffer = vec![
        0;
        reader
            .output_buffer_size()
            .ok_or("PNG image is too large")?
    ];
    let output = reader
        .next_frame(&mut buffer)
        .map_err(|error| format!("PNG image pixels: {error}"))?;
    let mut pixels = Vec::with_capacity(capacity);
    let channels = output.color_type.samples();
    for sample in buffer[..output.buffer_size()].chunks_exact(channels) {
        match output.color_type {
            png::ColorType::Rgba => pixels.extend_from_slice(sample),
            png::ColorType::Rgb => {
                pixels.extend_from_slice(&[sample[0], sample[1], sample[2], 255])
            }
            png::ColorType::Grayscale => {
                pixels.extend_from_slice(&[sample[0], sample[0], sample[0], 255])
            }
            png::ColorType::GrayscaleAlpha => {
                pixels.extend_from_slice(&[sample[0], sample[0], sample[0], sample[1]])
            }
            _ => return Err("PNG palette was not expanded".into()),
        }
    }
    Ok(RasterImage {
        width: output.width,
        height: output.height,
        pixels: pixels.into(),
    })
}
fn decode_jpeg(bytes: &[u8]) -> Result<(RasterImage, u8), String> {
    let mut decoder = zune_jpeg::JpegDecoder::new(Cursor::new(bytes));
    decoder
        .decode_headers()
        .map_err(|error| format!("JPEG image header: {error}"))?;
    let info = decoder.info().ok_or("JPEG has no dimensions")?;
    let width = u32::from(info.width);
    let height = u32::from(info.height);
    let capacity = dimensions(width, height)?;
    let buffer = decoder
        .decode()
        .map_err(|error| format!("JPEG image pixels: {error}"))?;
    let channels = decoder
        .output_colorspace()
        .ok_or("JPEG has no color space")?
        .num_components();
    let mut pixels = Vec::with_capacity(capacity);
    for sample in buffer.chunks_exact(channels) {
        match channels {
            1 => pixels.extend_from_slice(&[sample[0], sample[0], sample[0], 255]),
            3 => pixels.extend_from_slice(&[sample[0], sample[1], sample[2], 255]),
            _ => return Err("JPEG decoder returned an unsupported color space".into()),
        }
    }
    Ok((
        RasterImage {
            width,
            height,
            pixels: pixels.into(),
        },
        info.components,
    ))
}
fn decode_gif(bytes: &[u8]) -> Result<RasterImage, String> {
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = options
        .read_info(Cursor::new(bytes))
        .map_err(|error| format!("GIF image header: {error}"))?;
    dimensions(u32::from(decoder.width()), u32::from(decoder.height()))?;
    let frame = decoder
        .read_next_frame()
        .map_err(|error| format!("GIF image pixels: {error}"))?
        .ok_or("GIF has no frame")?;
    dimensions(u32::from(frame.width), u32::from(frame.height))?;
    Ok(RasterImage {
        width: u32::from(frame.width),
        height: u32::from(frame.height),
        pixels: Arc::from(frame.buffer.as_ref()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dimensions_are_bounded_before_pixel_allocation() {
        assert!(dimensions(0, 1).is_err());
        assert!(dimensions(u32::MAX, u32::MAX).is_err());
        assert!(dimensions(8001, 8000).is_err());
        assert_eq!(dimensions(8000, 8000).unwrap(), 256_000_000);
    }
    #[test]
    fn first_gif_frame_preserves_logical_canvas_dimensions() {
        let mut bytes = Vec::new();
        {
            let mut encoder = gif::Encoder::new(&mut bytes, 10, 8, &[0, 0, 0, 255, 0, 0]).unwrap();
            let frame = gif::Frame {
                width: 2,
                height: 3,
                left: 4,
                top: 2,
                buffer: std::borrow::Cow::Owned(vec![1; 6]),
                ..Default::default()
            };
            encoder.write_frame(&frame).unwrap();
        }
        let asset = decode(bytes, "frame.gif").unwrap();
        assert_eq!((asset.natural_width, asset.natural_height), (10, 8));
        assert_eq!((asset.raster.width, asset.raster.height), (2, 3));
        assert_eq!(
            asset.raster.pixels.as_chunks::<4>().0,
            &[[255, 0, 0, 255]; 6]
        );
    }
}
