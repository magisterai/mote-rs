//! Immutable image assets shared by editable packages and native painters.
mod decoding;
pub(crate) mod sampling;
mod transforms;
use crate::{
    model::{Content, Document, Element, ElementKind, Properties},
    source::{AssetRequest, AssetResolver, CachingAssetResolver},
};
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    sync::Arc,
};

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
    entries: BTreeMap<(Option<OsString>, String), Result<ImageAsset, String>>,
    pub warnings: Vec<String>,
    prepared: Vec<PreparedEntry>,
}
struct PreparedEntry {
    source_name: Option<OsString>,
    properties: Properties,
    image: Result<Arc<PreparedImage>, String>,
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
        for section in &document.sections {
            catalog.collect(&section.header.content, &resolver, source_name);
            catalog.collect(&section.footer.content, &resolver, source_name);
        }
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
                self.load_image(element, resolver, source_name);
            }
            self.collect(&element.children, resolver, source_name);
        }
    }
    /// Load only the image elements selected by a layout or package writer.
    pub fn load_selected<'a>(
        elements: impl IntoIterator<Item = &'a Element>,
        resolver: &dyn AssetResolver,
        source_name: Option<&OsStr>,
    ) -> Self {
        let resolver = CachingAssetResolver::new(resolver);
        let mut catalog = Self::default();
        for element in elements {
            catalog.load_image(element, &resolver, source_name);
        }
        catalog
    }
    fn load_image(
        &mut self,
        element: &Element,
        resolver: &dyn AssetResolver,
        source_name: Option<&OsStr>,
    ) {
        let source = text(&element.attributes, "src");
        let key = (element.source_name.clone(), source.clone());
        if !self.entries.contains_key(&key) {
            let loaded = resolver
                .resolve_asset(AssetRequest {
                    reference: OsStr::new(&source),
                    source_name: element.source_name.as_deref().or(source_name),
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
                self.warnings.push(error.clone());
            }
            self.entries.insert(key, loaded);
        }
    }
    pub fn prepare_all(&mut self, content: &[Content]) {
        for child in content {
            let Content::Element(element) = child else {
                continue;
            };
            if element.kind == ElementKind::Image {
                self.prepare_element(element);
            }
            self.prepare_all(&element.children);
        }
    }
    pub fn prepare_element(&mut self, element: &Element) {
        let properties = transformation_properties(&element.attributes);
        if self
            .prepared
            .iter()
            .any(|entry| entry.source_name == element.source_name && entry.properties == properties)
        {
            return;
        }
        let prepared = self
            .get(element)
            .and_then(|asset| prepare(asset, &element.attributes))
            .map(Arc::new);
        if let Err(error) = &prepared
            && self.get(element).is_ok()
        {
            self.warnings.push(error.clone());
        }
        self.prepared.push(PreparedEntry {
            source_name: element.source_name.clone(),
            properties,
            image: prepared,
        });
    }
    pub fn prepared(&self, element: &Element) -> Result<Arc<PreparedImage>, String> {
        let properties = transformation_properties(&element.attributes);
        self.prepared
            .iter()
            .find(|entry| {
                entry.source_name == element.source_name && entry.properties == properties
            })
            .map(|entry| entry.image.clone())
            .unwrap_or_else(|| Err("image assets were not prepared".into()))
    }
    pub fn get(&self, element: &Element) -> Result<&ImageAsset, String> {
        self.entries
            .get(&(
                element.source_name.clone(),
                text(&element.attributes, "src"),
            ))
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
