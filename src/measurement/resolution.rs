use super::{CoreFont, FontResolver, FontVariant, MeasurementError, OpenTypeFont, TextMeasurement};
use crate::{
    model::{Document, Value},
    source::{AssetRequest, AssetResolver, CachingAssetResolver, ResolvedAsset},
};
use std::{
    cell::RefCell,
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
    Installed,
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
    font_resolver: Option<&'source dyn FontResolver>,
    font_data: RefCell<BTreeMap<OsString, Arc<[u8]>>>,
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
            font_resolver: None,
            font_data: RefCell::new(BTreeMap::new()),
        }
    }

    /// Enable exact installed-face discovery for this measurement session.
    pub fn with_font_resolver(mut self, resolver: &'source dyn FontResolver) -> Self {
        self.font_resolver = Some(resolver);
        self.selections.clear();
        self
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
                return self.resolve_file(
                    name,
                    variant,
                    &reference,
                    declaration.source_name.as_deref().or(self.source_name),
                );
            }
            if let Some(Value::String(latin)) = declaration.properties.get("latin") {
                return self.resolve_named(name, latin, variant);
            }
        }
        if name.is_empty() {
            return core_selection(name, variant, CoreFont::Helvetica, None);
        }
        self.resolve_named(name, name, variant)
    }

    fn resolve_named(&self, name: &str, family: &str, variant: FontVariant) -> FontSelection {
        let mut fallback = named_selection(name, family, variant);
        if let Some(resolver) = self.font_resolver {
            let result = resolver
                .resolve_font(family, variant)
                .and_then(|asset| asset.map(|asset| self.snapshot_font(asset)).transpose());
            match result {
                Ok(Some((asset_name, bytes))) => {
                    return FontSelection {
                        requested_name: name.into(),
                        family: family.into(),
                        variant,
                        source: FontSource::Installed,
                        asset_name: Some(asset_name),
                        bytes: Some(bytes),
                        substitution: None,
                        core: CoreFont::Helvetica,
                    };
                }
                Err(reason) => fallback.substitution = Some(reason),
                Ok(None) => {}
            }
        }
        fallback
    }

    fn snapshot_font(&self, asset: ResolvedAsset) -> Result<(OsString, Arc<[u8]>), String> {
        if asset.name.is_empty() {
            return Err("font resolver supplied an empty logical name".into());
        }
        validate_embedding(&asset.bytes)?;
        OpenTypeFont::parse(&asset.bytes).map_err(|error| error.to_string())?;
        let mut data = self.font_data.borrow_mut();
        let bytes = if let Some(previous) = data.get(&asset.name) {
            if previous.as_ref() != asset.bytes {
                return Err(format!(
                    "font resolver supplied conflicting bytes for {:?}",
                    asset.name
                ));
            }
            previous.clone()
        } else {
            let bytes: Arc<[u8]> = asset.bytes.into();
            data.insert(asset.name.clone(), bytes.clone());
            bytes
        };
        Ok((asset.name, bytes))
    }

    fn resolve_file(
        &self,
        name: &str,
        variant: FontVariant,
        reference: &str,
        source_name: Option<&OsStr>,
    ) -> FontSelection {
        let loaded = self.resolver.resolve_asset(AssetRequest {
            reference: OsStr::new(reference),
            source_name,
        });
        let result = loaded
            .map_err(|_| "resolved font data could not be read".to_owned())
            .and_then(|asset| self.snapshot_font(asset));
        match result {
            Ok((asset_name, bytes)) => FontSelection {
                requested_name: name.into(),
                family: name.into(),
                variant,
                source: FontSource::Declared,
                asset_name: Some(asset_name),
                bytes: Some(bytes),
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

pub(super) fn validate_embedding(bytes: &[u8]) -> Result<(), String> {
    let header = bytes.get(..12).ok_or("invalid sfnt table directory")?;
    let table_count = usize::from(u16::from_be_bytes([header[4], header[5]]));
    if table_count == 0 || table_count > (bytes.len() - 12) / 16 {
        return Err("invalid sfnt table directory".into());
    }
    let directory_end = 12 + table_count * 16;
    let mut permission_table = None;
    for record in bytes[12..directory_end].as_chunks::<16>().0 {
        if &record[..4] != b"OS/2" {
            continue;
        }
        if permission_table.is_some() {
            return Err("duplicate OS/2 tables".into());
        }
        let offset = u32::from_be_bytes(record[8..12].try_into().unwrap()) as usize;
        let length = u32::from_be_bytes(record[12..16].try_into().unwrap()) as usize;
        if length < 10
            || offset < directory_end
            || offset > bytes.len()
            || length > bytes.len() - offset
        {
            return Err("invalid OS/2 table".into());
        }
        permission_table = Some(&bytes[offset..offset + length]);
    }
    let table = permission_table.ok_or("OS/2 embedding permissions are unavailable")?;
    let permissions = u16::from_be_bytes([table[8], table[9]]);
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
