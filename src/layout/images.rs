use super::{LayoutError, PageOperation, style_values};
use crate::{
    media::{self, ImageCatalog, PreparedImage},
    model::{Element, Properties},
};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct PositionedImage {
    pub image: Arc<PreparedImage>,
    pub bounds: [f64; 4],
    pub clip: [f64; 4],
    pub link: Option<String>,
}
pub(super) struct ImageBlock {
    pub anchors: Vec<String>,
    pub image: Arc<PreparedImage>,
    pub width: f64,
    pub height: f64,
    pub before: f64,
    pub after: f64,
    pub alignment: String,
    border: Option<(f64, String)>,
    link: Option<String>,
}
impl ImageBlock {
    pub fn prepare(
        element: &Element,
        assets: &ImageCatalog,
        maximum_width: f64,
    ) -> Result<Self, String> {
        let image = assets.prepared(element)?;
        let attributes = &element.attributes;
        let dimension = |name: &str| {
            attributes
                .get(name)
                .or_else(|| {
                    media::object(attributes, "placement").and_then(|placement| placement.get(name))
                })
                .and_then(style_values::points)
                .unwrap_or(0.0)
        };
        let mut width = dimension("width");
        let mut height = dimension("height");
        let natural_width = f64::from(image.original_width) * 0.75;
        let natural_height = f64::from(image.original_height) * 0.75;
        if width > 0.0 && height <= 0.0 {
            height = width * natural_height / natural_width;
        } else if height > 0.0 && width <= 0.0 {
            width = height * natural_width / natural_height;
        } else if width <= 0.0 && height <= 0.0 {
            width = natural_width;
            height = natural_height;
        }
        if width > maximum_width {
            height *= maximum_width / width;
            width = maximum_width;
        }
        let spacing = media::object(attributes, "spacing");
        let before = spacing
            .and_then(|spacing| spacing.get("before"))
            .and_then(style_values::points)
            .unwrap_or(0.0);
        let after = spacing
            .and_then(|spacing| spacing.get("after"))
            .and_then(style_values::points)
            .unwrap_or(8.0);
        let border = media::object(attributes, "border")
            .filter(|border| media::text(border, "style") != "none")
            .map(|border| {
                (
                    border
                        .get("width")
                        .and_then(style_values::points)
                        .unwrap_or(1.0),
                    style_values::color(border, "color").unwrap_or_else(|| "D1D5DB".into()),
                )
            });
        let link = attributes
            .get("link")
            .map(crate::model::Value::to_text)
            .filter(|value| !value.is_empty());
        Ok(Self {
            anchors: Vec::new(),
            image,
            width,
            height,
            before,
            after,
            alignment: media::image_alignment(attributes),
            border,
            link,
        })
    }
    pub fn height(&self) -> f64 {
        self.height + 1.0
    }
    pub fn place(
        &self,
        left: f64,
        column_width: f64,
        top: f64,
    ) -> Result<Vec<PageOperation>, LayoutError> {
        let horizontal = left
            + match self.alignment.as_str() {
                "center" => (column_width - self.width) / 2.0,
                "right" => column_width - self.width,
                _ => 0.0,
            }
            .max(0.0);
        let horizontal_scale = self.width / f64::from(self.image.original_width);
        let vertical_scale = self.height / f64::from(self.image.original_height);
        let bounds = [
            horizontal - self.image.offset[0] * horizontal_scale,
            top - self.image.offset[1] * vertical_scale,
            f64::from(self.image.raster_width) * horizontal_scale,
            f64::from(self.image.raster_height) * vertical_scale,
        ];
        let clip = [bounds[0], top, bounds[2], self.height];
        let mut operations = vec![PageOperation::Image(Box::new(PositionedImage {
            image: self.image.clone(),
            bounds,
            clip,
            link: self.link.clone(),
        }))];
        if let Some((width, color)) = &self.border {
            if *width <= 0.0 {
                return Err(LayoutError::InvalidGeometry(
                    "image border width must be positive",
                ));
            }
            operations.push(PageOperation::OutlineRectangle {
                bounds: [horizontal, top, self.width, self.height],
                width: *width,
                color: color.clone(),
            });
        }
        Ok(operations)
    }
}
pub(super) fn fallback(element: &Element, error: &str) -> Element {
    let source = media::text(&element.attributes, "src");
    let kind = if error.contains("could not be read") {
        "missing"
    } else {
        "unsupported"
    };
    let mut attributes = Properties::new();
    if let Some(spacing) = element.attributes.get("spacing") {
        attributes.insert("spacing".into(), spacing.clone());
    }
    Element {
        source_name: None,
        kind: crate::model::ElementKind::Paragraph,
        attributes,
        children: vec![crate::model::Content::Text(format!(
            "[{kind} image: {source}]"
        ))],
    }
}
