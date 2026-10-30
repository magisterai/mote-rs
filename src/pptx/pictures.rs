//! Editable pictures and their package media relationships.
use super::{Error, escape, package::Relationship};
use crate::{
    media::ImageCatalog,
    model::Element,
    presentation::{
        box_bounds,
        images::{Crop, ImagePlacement},
    },
    properties::{boolean, number, object, text},
};
use std::sync::Arc;

pub(super) struct Pictures<'a> {
    images: &'a ImageCatalog,
    pub media: Vec<(String, Arc<[u8]>)>,
}
impl<'a> Pictures<'a> {
    pub fn new(images: &'a ImageCatalog) -> Self {
        Self {
            images,
            media: Vec::new(),
        }
    }
    pub fn write(
        &mut self,
        container: &Element,
        image: &Element,
        next_shape: &mut usize,
        relationships: &mut Vec<Relationship>,
    ) -> Result<Option<String>, Error> {
        let Ok(asset) = self.images.get(image) else {
            return Ok(None);
        };
        let media_name = format!("media/image{}{}", self.media.len() + 1, asset.extension);
        self.media
            .push((format!("ppt/{media_name}"), asset.original.clone()));
        relationships.push(Relationship {
            kind: "image",
            target: format!("../{media_name}"),
            external: false,
        });
        let relationship = relationships.len();
        let identifier = *next_shape;
        *next_shape += 1;
        let attributes = &image.attributes;
        let first_text = |names: &[&str]| {
            names
                .iter()
                .map(|name| text(attributes, name))
                .find(|value| !value.is_empty())
        };
        let mut name = text(&container.attributes, "name");
        if name.is_empty() {
            name = first_text(&["name", "title", "alt", "src"])
                .unwrap_or_else(|| format!("Picture {identifier}"));
        }
        let name = escape(&name)?;
        let description =
            escape(&first_text(&["description", "alt", "title", "src"]).unwrap_or_default())?;
        let mut nonvisual =
            format!(r#"<p:cNvPr id="{identifier}" name="{name}" descr="{description}""#);
        let link = text(attributes, "link");
        if link.is_empty() {
            nonvisual.push_str("/>");
        } else {
            relationships.push(Relationship {
                kind: "hyperlink",
                target: link,
                external: true,
            });
            nonvisual.push_str(&format!(
                r#"><a:hlinkClick r:id="rId{}"/></p:cNvPr>"#,
                relationships.len()
            ));
        }
        let crop = Crop::resolve(attributes);
        let dimensions = crop.dimensions(
            f64::from(asset.natural_width),
            f64::from(asset.natural_height),
        );
        let placement = ImagePlacement::resolve(
            box_bounds(&container.attributes)
                .map_err(Error)?
                .map(|value| value as f64),
            dimensions,
            &text(&container.attributes, "fit"),
        );
        let crop = crop_xml(crop.compose(placement.crop));
        let [left, top, width, height] = placement.object.map(|value| value.round() as i64);
        let mut transform = String::from("<a:xfrm");
        if let Some(rotation) = number(attributes, "rotation") {
            transform.push_str(&format!(
                r#" rot="{}""#,
                (rotation * 60000.0).round() as i64
            ));
        }
        for (name, field) in [("flip_horizontal", "flipH"), ("flip_vertical", "flipV")] {
            if boolean(attributes, name) {
                transform.push_str(&format!(r#" {field}="1""#));
            }
        }
        transform.push_str(&format!(
            r#"><a:off x="{left}" y="{top}"/><a:ext cx="{width}" cy="{height}"/></a:xfrm>"#
        ));
        let border = super::boxes::border(if object(attributes, "border").is_some() {
            attributes
        } else {
            &container.attributes
        })?;
        let effects = crate::drawings::pictures::effects(attributes);
        Ok(Some(format!(
            r#"<p:pic><p:nvPicPr>{nonvisual}<p:cNvPicPr><a:picLocks noChangeAspect="1"/></p:cNvPicPr><p:nvPr/></p:nvPicPr><p:blipFill><a:blip r:embed="rId{relationship}">{effects}</a:blip>{crop}<a:stretch><a:fillRect/></a:stretch></p:blipFill><p:spPr>{transform}<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>{border}</p:spPr></p:pic>"#
        )))
    }
}
fn crop_xml(crop: Crop) -> String {
    let mut fields = String::new();
    for (first_name, first, second_name, second) in [
        ("l", crop.left, "r", crop.right),
        ("t", crop.top, "b", crop.bottom),
    ] {
        if first > 0.0 || second > 0.0 {
            fields.push_str(&format!(
                r#" {first_name}="{}" {second_name}="{}""#,
                (first * 100000.0).round() as i64,
                (second * 100000.0).round() as i64
            ));
        }
    }
    if fields.is_empty() {
        fields
    } else {
        format!("<a:srcRect{fields}/>")
    }
}
