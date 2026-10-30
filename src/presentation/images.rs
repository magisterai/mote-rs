//! Picture fitting in either drawing units or native page points.
use crate::{
    model::{Content, Element, ElementKind, Properties},
    properties::{number, object},
};

pub(crate) fn child(element: &Element) -> Option<&Element> {
    match element.children.as_slice() {
        [Content::Element(image)] if image.kind == ElementKind::Image => Some(image),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Crop {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl Crop {
    pub fn resolve(attributes: &Properties) -> Self {
        let fraction = |name| {
            object(attributes, "crop")
                .and_then(|crop| number(crop, name))
                .map(|value| ((value * 1000.0).round() / 100000.0).clamp(0.0, 1.0))
                .unwrap_or(0.0)
        };
        Self {
            left: fraction("left"),
            top: fraction("top"),
            right: fraction("right"),
            bottom: fraction("bottom"),
        }
    }
    pub fn dimensions(self, width: f64, height: f64) -> [f64; 2] {
        [
            width * (1.0 - self.left - self.right).max(0.0),
            height * (1.0 - self.top - self.bottom).max(0.0),
        ]
    }
    pub fn compose(self, relative: Self) -> Self {
        let [width, height] = self.dimensions(1.0, 1.0);
        Self {
            left: (self.left + relative.left * width).clamp(0.0, 1.0),
            top: (self.top + relative.top * height).clamp(0.0, 1.0),
            right: (self.right + relative.right * width).clamp(0.0, 1.0),
            bottom: (self.bottom + relative.bottom * height).clamp(0.0, 1.0),
        }
    }
}
pub(crate) struct ImagePlacement {
    pub object: [f64; 4],
    pub paint: [f64; 4],
    pub crop: Crop,
}
impl ImagePlacement {
    pub fn resolve(container: [f64; 4], dimensions: [f64; 2], fit: &str) -> Self {
        let mut result = Self {
            object: container,
            paint: container,
            crop: Crop::default(),
        };
        if container[2] <= 0.0
            || container[3] <= 0.0
            || dimensions.iter().any(|value| *value <= 0.0)
        {
            return result;
        }
        let image_ratio = dimensions[0] / dimensions[1];
        let container_ratio = container[2] / container[3];
        let mut frame = container;
        match fit {
            "contain" => {
                if image_ratio > container_ratio {
                    frame[3] = container[2] / image_ratio;
                    frame[1] += (container[3] - frame[3]) / 2.0;
                } else {
                    frame[2] = container[3] * image_ratio;
                    frame[0] += (container[2] - frame[2]) / 2.0;
                }
                result.object = frame.map(round_coordinate);
                result.paint = result.object;
            }
            "cover" => {
                if image_ratio > container_ratio {
                    frame[2] = container[3] * image_ratio;
                    frame[0] -= (frame[2] - container[2]) / 2.0;
                    result.crop.left = (1.0 - container[2] / frame[2]) / 2.0;
                    result.crop.right = result.crop.left;
                } else if image_ratio < container_ratio {
                    frame[3] = container[2] / image_ratio;
                    frame[1] -= (frame[3] - container[3]) / 2.0;
                    result.crop.top = (1.0 - container[3] / frame[3]) / 2.0;
                    result.crop.bottom = result.crop.top;
                }
                result.paint = frame.map(round_coordinate);
            }
            _ => {}
        }
        result
    }
}
fn round_coordinate(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

/// Select direct image boxes without resolving unrelated document assets.
pub(crate) fn elements(deck: &super::Deck) -> impl Iterator<Item = &Element> {
    deck.slides.iter().flat_map(|slide| {
        slide
            .background
            .image
            .iter()
            .chain(
                slide
                    .element
                    .children
                    .iter()
                    .filter_map(|content| match content {
                        Content::Element(element) if element.kind == ElementKind::Box => {
                            child(element)
                        }
                        _ => None,
                    }),
            )
    })
}
