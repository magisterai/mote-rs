use super::{CoreFont, FontVariant, MeasurementError, OpenTypeFont, TextMeasurement};
use crate::{
    model::{Document, Value},
    source::{AssetRequest, AssetResolver, CachingAssetResolver},
};
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementStyle {
    pub font: String,
    pub size_points: f64,
    pub variant: FontVariant,
}

impl Default for MeasurementStyle {
    fn default() -> Self {
        Self {
            font: String::new(),
            size_points: 11.0,
            variant: FontVariant::Regular,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSource {
    Core,
    Declared,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontSelection {
    pub requested_name: String,
    pub family: String,
    pub variant: FontVariant,
    pub source: FontSource,
    pub asset_name: Option<OsString>,
    /// Exact immutable bytes used for measurement and available to painters.
    pub bytes: Option<Arc<[u8]>>,
    pub substitution: Option<String>,
    core: CoreFont,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasuredText {
    pub font: FontSelection,
    pub measurement: TextMeasurement,
}

pub struct TextMeasurer<'source> {
    document: &'source Document,
    source_name: Option<&'source OsStr>,
    resolver: CachingAssetResolver<'source>,
    selections: BTreeMap<(String, FontVariant), FontSelection>,
}

impl<'source> TextMeasurer<'source> {
    pub fn new(
        document: &'source Document,
        resolver: &'source dyn AssetResolver,
        source_name: Option<&'source OsStr>,
    ) -> Self {
        Self {
            document,
            source_name,
            resolver: CachingAssetResolver::new(resolver),
            selections: BTreeMap::new(),
        }
    }

    pub fn resolve(&mut self, name: &str, variant: FontVariant) -> FontSelection {
        let key = (name.to_owned(), variant);
        if let Some(selection) = self.selections.get(&key) {
            return selection.clone();
        }
        let selection = self.resolve_uncached(name, variant);
        self.selections.insert(key, selection.clone());
        selection
    }

    pub fn measure(
        &mut self,
        style: &MeasurementStyle,
        text: &str,
        layout_kerning: bool,
    ) -> Result<MeasuredText, MeasurementError> {
        let font = self.resolve(&style.font, style.variant);
        let measurement = match &font.bytes {
            Some(bytes) => {
                OpenTypeFont::parse(bytes)?.measure(text, style.size_points, layout_kerning)?
            }
            None => font.core.measure(text, style.size_points, style.variant)?,
        };
        Ok(MeasuredText { font, measurement })
    }

    fn resolve_uncached(&self, name: &str, variant: FontVariant) -> FontSelection {
        if let Some(declaration) = self.document.fonts.get(name) {
            if let Some(Value::Object(files)) = declaration.properties.get("files") {
                let reference = files
                    .get(variant.as_text())
                    .map(Value::to_text)
                    .unwrap_or_default();
                if reference.is_empty() {
                    return core_selection(
                        name,
                        variant,
                        CoreFont::Helvetica,
                        Some(format!("missing {} font file", variant.as_text())),
                    );
                }
                return self.resolve_file(name, variant, &reference);
            }
            if let Some(Value::String(latin)) = declaration.properties.get("latin") {
                return named_selection(name, latin, variant);
            }
        }
        if name.is_empty() {
            return core_selection(name, variant, CoreFont::Helvetica, None);
        }
        named_selection(name, name, variant)
    }

    fn resolve_file(&self, name: &str, variant: FontVariant, reference: &str) -> FontSelection {
        let loaded = self.resolver.resolve_asset(AssetRequest {
            reference: OsStr::new(reference),
            source_name: self.source_name,
        });
        let result = loaded
            .map_err(|_| "resolved font data could not be read".to_owned())
            .and_then(|asset| {
                validate_embedding(&asset.bytes)?;
                OpenTypeFont::parse(&asset.bytes).map_err(|error| error.to_string())?;
                Ok(asset)
            });
        match result {
            Ok(asset) => FontSelection {
                requested_name: name.into(),
                family: name.into(),
                variant,
                source: FontSource::Declared,
                asset_name: Some(asset.name),
                bytes: Some(asset.bytes.into()),
                substitution: None,
                core: CoreFont::Helvetica,
            },
            Err(reason) => core_selection(
                name,
                variant,
                CoreFont::Helvetica,
                Some(format!(
                    "declared font {name:?} {} is unavailable: {reason}",
                    variant.as_text()
                )),
            ),
        }
    }
}

fn core_selection(
    name: &str,
    variant: FontVariant,
    core: CoreFont,
    substitution: Option<String>,
) -> FontSelection {
    FontSelection {
        requested_name: name.into(),
        family: core.family().into(),
        variant,
        source: FontSource::Core,
        asset_name: None,
        bytes: None,
        substitution,
        core,
    }
}

fn named_selection(name: &str, family: &str, variant: FontVariant) -> FontSelection {
    let normalized = family.trim().to_lowercase();
    let exact = match normalized.as_str() {
        "helvetica" => Some(CoreFont::Helvetica),
        "times" => Some(CoreFont::Times),
        "courier" | "mono" => Some(CoreFont::Courier),
        _ => None,
    };
    if let Some(core) = exact {
        return core_selection(name, variant, core, None);
    }
    let compatible = if ["courier", "mono", "consolas", "cascadia"]
        .iter()
        .any(|part| normalized.contains(part))
    {
        Some(CoreFont::Courier)
    } else if normalized.contains("times")
        || normalized.contains("georgia")
        || normalized == "serif"
        || normalized.ends_with(" serif")
    {
        Some(CoreFont::Times)
    } else if normalized == "arial"
        || normalized.contains("aptos")
        || normalized.contains("calibri")
        || normalized == "sans"
        || normalized.contains("sans serif")
        || normalized.contains("sans-serif")
    {
        Some(CoreFont::Helvetica)
    } else {
        None
    };
    match compatible {
        Some(core) => core_selection(
            name,
            variant,
            core,
            Some(format!(
                "exact installed font {family:?} {} is unavailable",
                variant.as_text()
            )),
        ),
        None => core_selection(
            name,
            variant,
            CoreFont::Helvetica,
            Some(format!("unsupported font {family:?}")),
        ),
    }
}

fn validate_embedding(bytes: &[u8]) -> Result<(), String> {
    let face = ttf_parser::Face::parse(bytes, 0)
        .map_err(|_| "font data could not be parsed as OpenType")?;
    let permissions = face
        .raw_face()
        .table(ttf_parser::Tag::from_bytes(b"OS/2"))
        .and_then(|table| table.get(8..10))
        .ok_or("OS/2 embedding permissions are unavailable")?;
    let permissions = u16::from_be_bytes(permissions.try_into().unwrap());
    let reason = if permissions & 2 != 0 {
        "OS/2 fsType restricts embedding"
    } else if permissions & 0x100 != 0 {
        "OS/2 fsType forbids subsetting"
    } else if permissions & 0x200 != 0 {
        "OS/2 fsType permits bitmap embedding only"
    } else if permissions & !0x30f != 0 {
        "OS/2 fsType contains reserved permission bits"
    } else {
        match permissions & 15 {
            0 | 8 => return Ok(()),
            4 => "OS/2 fsType permits preview/print embedding only",
            _ => "OS/2 fsType contains ambiguous embedding permissions",
        }
    };
    Err(reason.into())
}
