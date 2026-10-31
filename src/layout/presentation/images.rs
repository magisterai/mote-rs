//! Place prepared image blocks inside slide frames, then rotate the whole object.
use crate::{
    layout::{LayoutError, PageOperation, images::ImageBlock},
    media::ImageCatalog,
    model::{Element, Properties, Unit, Value},
    presentation::{
        box_bounds,
        images::{Crop, ImagePlacement},
    },
    properties::{number, text},
};

pub(super) enum ImageContainer {
    Background,
    Box,
}

pub(super) fn place(
    container: &Element,
    image: &Element,
    context: ImageContainer,
    images: &mut ImageCatalog,
    operations: &mut Vec<PageOperation>,
    warnings: &mut Vec<String>,
) -> Result<(), LayoutError> {
    let bounds = box_bounds(&container.attributes)
        .map_err(LayoutError::Unsupported)?
        .map(|value| value as f64 / 12700.0);
    if bounds[2] <= 0.0 || bounds[3] <= 0.0 {
        return Ok(());
    }
    let fit = text(&container.attributes, "fit");
    if !matches!(fit.as_str(), "" | "stretch" | "contain" | "cover") {
        return Err(LayoutError::Unsupported(format!(
            "unsupported presentation image fit {fit:?}"
        )));
    }
    let mut resolved = image.clone();
    let rotation = number(&resolved.attributes, "rotation").unwrap_or(0.0);
    resolved.attributes.remove("rotation");
    resolved.attributes.remove("border");
    resolved
        .attributes
        .insert("alignment".into(), Value::String("left".into()));
    resolved
        .attributes
        .insert("width".into(), points(bounds[2]));
    if matches!(fit.as_str(), "contain" | "cover") {
        resolved.attributes.remove("height");
    } else {
        resolved
            .attributes
            .insert("height".into(), points(bounds[3]));
    }
    images.prepare_element(&resolved);
    let mut block = match ImageBlock::prepare(&resolved, images, bounds[2]) {
        Ok(block) => block,
        Err(error) => {
            warnings.push(format!("{error}; omitted picture"));
            return Ok(());
        }
    };
    // Prepared pixels retain their storage extent; fit their cropped source
    // aspect ratio and scale that raster into the same object as editable output.
    let dimensions = Crop::resolve(&image.attributes).dimensions(block.width, block.height);
    let placement = ImagePlacement::resolve(bounds, dimensions, &fit);
    // These are resolved page coordinates, not authored document distances.
    // Keep their precision instead of converting them through integer twips.
    block.width = placement.paint[2];
    block.height = placement.paint[3];
    if rotation != 0.0 {
        operations.push(PageOperation::PushRotation {
            degrees: rotation,
            center: [
                placement.object[0] + placement.object[2] / 2.0,
                placement.object[1] + placement.object[3] / 2.0,
            ],
        });
    }
    operations.push(PageOperation::PushClip(bounds));
    for mut operation in block.place(placement.paint[0], placement.paint[2], placement.paint[1])? {
        if matches!(context, ImageContainer::Box)
            && let PageOperation::Image(image) = &mut operation
        {
            let Some(clip) = intersection(image.clip, bounds) else {
                continue;
            };
            image.clip = clip;
        }
        operations.push(operation);
    }
    operations.push(PageOperation::PopClip);
    let mut border = Properties::new();
    if let Some(value) = image
        .attributes
        .get("border")
        .or_else(|| container.attributes.get("border"))
    {
        border.insert("border".into(), value.clone());
    }
    if let Some(rectangle) = super::rectangle(placement.object, &border) {
        operations.push(rectangle);
    }
    if rotation != 0.0 {
        operations.push(PageOperation::PopRotation);
    }
    Ok(())
}
fn points(value: f64) -> Value {
    Value::Length {
        value,
        unit: Unit::Point,
    }
}
fn intersection(first: [f64; 4], second: [f64; 4]) -> Option<[f64; 4]> {
    let left = first[0].max(second[0]);
    let top = first[1].max(second[1]);
    let right = (first[0] + first[2]).min(second[0] + second[2]);
    let bottom = (first[1] + first[3]).min(second[1] + second[3]);
    (right > left && bottom > top).then_some([left, top, right - left, bottom - top])
}
