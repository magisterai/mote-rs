use super::{Objects, compress, rounded};
use crate::{layout::PositionedImage, render::RenderError};
use pdf_writer::{Content, Filter, Name, Pdf, Rect, Ref};

pub(super) fn write(
    image: &PositionedImage,
    document: &mut Pdf,
    objects: &mut Objects,
) -> Result<Ref, RenderError> {
    let raster = &image.image.raster;
    let reference = objects.allocate();
    if let Some((bytes, components)) = &image.image.jpeg {
        let mut stream = document.image_xobject(reference, bytes);
        stream
            .width(raster.width as i32)
            .height(raster.height as i32)
            .bits_per_component(8)
            .filter(Filter::DctDecode);
        match components {
            1 => {
                stream.color_space().device_gray();
            }
            4 => {
                stream.color_space().device_cmyk();
                stream.decode([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
            }
            _ => {
                stream.color_space().device_rgb();
            }
        }
        return Ok(reference);
    }
    let mut colors = Vec::with_capacity(raster.width as usize * raster.height as usize * 3);
    let mut alpha = Vec::with_capacity(raster.width as usize * raster.height as usize);
    for pixel in raster.pixels.as_chunks::<4>().0 {
        colors.extend_from_slice(&pixel[..3]);
        alpha.push(pixel[3]);
    }
    let mask = if alpha.iter().any(|value| *value != 255) {
        let mask = objects.allocate();
        let compressed = compress(&alpha)?;
        let mut stream = document.image_xobject(mask, &compressed);
        stream
            .width(raster.width as i32)
            .height(raster.height as i32)
            .bits_per_component(8)
            .filter(Filter::FlateDecode);
        stream.color_space().device_gray();
        Some(mask)
    } else {
        None
    };
    let compressed = compress(&colors)?;
    let mut stream = document.image_xobject(reference, &compressed);
    stream
        .width(raster.width as i32)
        .height(raster.height as i32)
        .bits_per_component(8)
        .filter(Filter::FlateDecode);
    stream.color_space().device_rgb();
    if let Some(mask) = mask {
        stream.s_mask(mask);
    }
    Ok(reference)
}
pub(super) fn paint(image: &PositionedImage, content: &mut Content, name: &str, page_height: f64) {
    let [left, top, width, height] = image.bounds;
    let [clip_left, clip_top, clip_width, clip_height] = image.clip;
    content
        .save_state()
        .rect(
            rounded(clip_left),
            rounded(page_height - clip_top),
            rounded(clip_width),
            -rounded(clip_height),
        )
        .clip_nonzero()
        .end_path()
        .transform([
            rounded(width),
            0.0,
            0.0,
            rounded(height),
            rounded(left),
            rounded(page_height - top - height),
        ])
        .x_object(Name(name.as_bytes()))
        .restore_state();
}
pub(super) fn annotation_bounds(image: &PositionedImage, page_height: f64) -> Rect {
    let [left, top, width, height] = image.clip;
    Rect::new(
        rounded(left),
        rounded(page_height - top - height),
        rounded(left + width),
        rounded(page_height - top),
    )
}
