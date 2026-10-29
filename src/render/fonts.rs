use super::RenderError;
use crate::measurement::{FontSelection, FontVariant};
use std::{collections::BTreeMap, sync::Arc};
use ttf_parser::Face;

pub(super) struct RasterFont {
    pub bytes: Arc<[u8]>,
    pub face_index: u32,
    pub fallback: bool,
}
#[derive(Default)]
pub(super) struct RasterFonts {
    fonts: BTreeMap<(String, FontVariant), RasterFont>,
}
impl RasterFonts {
    pub fn resolve(&mut self, selection: &FontSelection) -> Result<&RasterFont, RenderError> {
        let key = (
            selection
                .asset_name
                .as_ref()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| selection.family.clone()),
            selection.variant,
        );
        if !self.fonts.contains_key(&key) {
            let font = match &selection.bytes {
                Some(bytes) => RasterFont {
                    bytes: bytes.clone(),
                    face_index: 0,
                    fallback: false,
                },
                None => core_raster_font(&selection.family, selection.variant),
            };
            Face::parse(&font.bytes, font.face_index)
                .map_err(|_| RenderError::Invalid("invalid raster font".into()))?;
            self.fonts.insert(key.clone(), font);
        }
        Ok(&self.fonts[&key])
    }
}
fn core_raster_font(family: &str, variant: FontVariant) -> RasterFont {
    let face_index = match variant {
        FontVariant::Regular => 0,
        FontVariant::Bold => 1,
        FontVariant::Italic => 2,
        FontVariant::BoldItalic => 3,
    };
    let collection = format!("/System/Library/Fonts/{family}.ttc");
    if let Ok(bytes) = std::fs::read(collection)
        && Face::parse(&bytes, face_index).is_ok()
    {
        return RasterFont {
            bytes: bytes.into(),
            face_index,
            fallback: false,
        };
    }
    let family = match family {
        "Times" => "Times New Roman",
        "Courier" => "Courier New",
        _ => "Arial",
    };
    let variant_name = match variant {
        FontVariant::Regular => "",
        FontVariant::Bold => " Bold",
        FontVariant::Italic => " Italic",
        FontVariant::BoldItalic => " Bold Italic",
    };
    if let Ok(bytes) = std::fs::read(format!(
        "/System/Library/Fonts/Supplemental/{family}{variant_name}.ttf"
    )) && Face::parse(&bytes, 0).is_ok()
    {
        return RasterFont {
            bytes: bytes.into(),
            face_index: 0,
            fallback: false,
        };
    }
    RasterFont {
        bytes: Arc::from(bundled_font(variant)),
        face_index: 0,
        fallback: true,
    }
}
pub(super) fn bundled_font(variant: FontVariant) -> &'static [u8] {
    match variant {
        FontVariant::Regular => include_bytes!("../../assets/fonts/Go-Regular.ttf"),
        FontVariant::Bold => include_bytes!("../../assets/fonts/Go-Bold.ttf"),
        FontVariant::Italic => include_bytes!("../../assets/fonts/Go-Italic.ttf"),
        FontVariant::BoldItalic => include_bytes!("../../assets/fonts/Go-Bold-Italic.ttf"),
    }
}
pub(super) fn core_pdf_name(selection: &FontSelection) -> Result<&'static str, RenderError> {
    use FontVariant::*;
    Ok(match (selection.family.as_str(), selection.variant) {
        ("Helvetica", Regular) => "Helvetica",
        ("Helvetica", Bold) => "Helvetica-Bold",
        ("Helvetica", Italic) => "Helvetica-Oblique",
        ("Helvetica", BoldItalic) => "Helvetica-BoldOblique",
        ("Times", Regular) => "Times-Roman",
        ("Times", Bold) => "Times-Bold",
        ("Times", Italic) => "Times-Italic",
        ("Times", BoldItalic) => "Times-BoldItalic",
        ("Courier", Regular) => "Courier",
        ("Courier", Bold) => "Courier-Bold",
        ("Courier", Italic) => "Courier-Oblique",
        ("Courier", BoldItalic) => "Courier-BoldOblique",
        _ => {
            return Err(RenderError::Invalid(format!(
                "unknown core font {:?}",
                selection.family
            )));
        }
    })
}
