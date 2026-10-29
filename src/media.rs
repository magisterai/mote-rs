//! Immutable image assets shared by editable packages and native painters.
mod decoding;
pub(crate) mod sampling;
mod transforms;
use crate::{
    model::{Content, Document, Element, ElementKind, Properties},
    source::{AssetRequest, AssetResolver, CachingAssetResolver},
};
use std::{collections::BTreeMap, ffi::OsStr, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub struct RasterImage {
    pub width: u32,
    pub height: u32,
    /// Straight (unassociated) RGBA, eight bits per channel.
    pub pixels: Arc<[u8]>,
}
#[derive(Debug, Clone)]
pub(crate) struct ImageAsset {
    pub original: Arc<[u8]>,
    pub extension: &'static str,
    pub content_type: &'static str,
    pub raster: RasterImage,
    pub natural_width: u32,
    pub natural_height: u32,
    pub jpeg_components: Option<u8>,
}
#[derive(Default)]
pub(crate) struct ImageCatalog {
    entries: BTreeMap<String, Result<ImageAsset, String>>,
    pub warnings: Vec<String>,
    prepared: Vec<(Properties, Result<Arc<PreparedImage>, String>)>,
}
impl ImageCatalog {
    pub fn load(
        document: &Document,
        resolver: &dyn AssetResolver,
        source_name: Option<&OsStr>,
    ) -> Self {
        let resolver = CachingAssetResolver::new(resolver);
        let mut catalog = Self::default();
        catalog.collect(&document.content, &resolver, source_name);
        catalog
    }
    fn collect(
        &mut self,
        content: &[Content],
        resolver: &dyn AssetResolver,
        source_name: Option<&OsStr>,
    ) {
        for child in content {
            let Content::Element(element) = child else {
                continue;
            };
            if element.kind == ElementKind::Image {
                let source = text(&element.attributes, "src");
                if !self.entries.contains_key(&source) {
                    let loaded = resolver
                        .resolve_asset(AssetRequest {
                            reference: OsStr::new(&source),
                            source_name,
                        })
                        .map_err(|error| {
                            let reason = if error.kind() == std::io::ErrorKind::Unsupported {
                                "is unsupported"
                            } else {
                                "could not be read"
                            };
                            format!("image {source:?} {reason}: {error}")
                        })
                        .and_then(|asset| decoding::decode(asset.bytes, &source));
                    if let Err(error) = &loaded {
                        self.warnings
                            .push(format!("{error}; emitted placeholder text"));
                    }
                    self.entries.insert(source, loaded);
                }
            }
            self.collect(&element.children, resolver, source_name);
        }
    }
    pub fn prepare_all(&mut self, content: &[Content]) {
        for child in content {
            let Content::Element(element) = child else {
                continue;
            };
            if element.kind == ElementKind::Image
                && !self.prepared.iter().any(|(attributes, _)| {
                    attributes == &transformation_properties(&element.attributes)
                })
            {
                let prepared = self
                    .get(element)
                    .and_then(|asset| prepare(asset, &element.attributes))
                    .map(Arc::new);
                if let Err(error) = &prepared
                    && self.get(element).is_ok()
                {
                    self.warnings
                        .push(format!("{error}; emitted placeholder text"));
                }
                self.prepared
                    .push((transformation_properties(&element.attributes), prepared));
            }
            self.prepare_all(&element.children);
        }
    }
    pub fn prepared(&self, element: &Element) -> Result<Arc<PreparedImage>, String> {
        self.prepared
            .iter()
            .find(|(attributes, _)| attributes == &transformation_properties(&element.attributes))
            .map(|(_, image)| image.clone())
            .unwrap_or_else(|| Err("image assets were not prepared".into()))
    }
    pub fn get(&self, element: &Element) -> Result<&ImageAsset, String> {
        self.entries
            .get(&text(&element.attributes, "src"))
            .ok_or_else(|| "image assets were not supplied to layout".to_owned())?
            .as_ref()
            .map_err(Clone::clone)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedImage {
    pub raster: RasterImage,
    /// Geometry extent can differ from decoded pixels for a GIF subframe.
    pub raster_width: u32,
    pub raster_height: u32,
    pub original_width: u32,
    pub original_height: u32,
    pub offset: [f64; 2],
    pub(crate) jpeg: Option<(Arc<[u8]>, u8)>,
}
pub(crate) fn prepare(
    asset: &ImageAsset,
    attributes: &Properties,
) -> Result<PreparedImage, String> {
    let mut prepared = transforms::prepare(&asset.raster, attributes)?;
    if transforms::is_identity(attributes) {
        prepared.original_width = asset.natural_width;
        prepared.original_height = asset.natural_height;
        prepared.raster_width = asset.natural_width;
        prepared.raster_height = asset.natural_height;
        if let Some(components) = asset.jpeg_components {
            prepared.jpeg = Some((asset.original.clone(), components));
        }
    }
    Ok(prepared)
}
pub(crate) use crate::properties::{boolean, number, object, text};

pub(crate) fn image_alignment(attributes: &Properties) -> String {
    let direct = text(attributes, "alignment");
    if !direct.is_empty() {
        return direct;
    }
    if let Some(placement) = object(attributes, "placement") {
        let compact = text(placement, "align");
        if !compact.is_empty() {
            return compact;
        }
        if let Some(horizontal) = object(placement, "horizontal") {
            let alignment = text(horizontal, "alignment");
            if matches!(alignment.as_str(), "left" | "center" | "right") {
                return alignment;
            }
        }
    }
    String::new()
}

fn transformation_properties(attributes: &Properties) -> Properties {
    attributes
        .iter()
        .filter(|(name, _)| {
            matches!(
                name.as_str(),
                "src" | "crop" | "rotation" | "flip_horizontal" | "flip_vertical" | "effects"
            )
        })
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect()
}
