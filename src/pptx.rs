//! Editable PresentationML packages for explicit slides and positioned content.
mod boxes;
mod charts;
mod fields;
mod groups;
mod notes;
mod package;
mod pictures;
mod shapes;
mod tables;
mod text;
use crate::{model::Document, presentation::Deck};
use std::fmt;

#[derive(Debug)]
pub struct Error(String);
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
impl std::error::Error for Error {}
impl From<crate::xml::InvalidCharacter> for Error {
    fn from(error: crate::xml::InvalidCharacter) -> Self {
        Self(error.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}
impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Self(error.to_string())
    }
}
pub struct GeneratedPackage {
    pub bytes: Vec<u8>,
    pub warnings: Vec<String>,
}
pub fn emit(document: &Document) -> Result<Vec<u8>, Error> {
    Ok(emit_with_warnings(document)?.bytes)
}
pub fn emit_with_warnings(document: &Document) -> Result<GeneratedPackage, Error> {
    let resolver = crate::source::FileAssetResolver::from_current_directory()?;
    emit_with_assets(document, &resolver, None)
}
/// Font assets used for shared text fitting resolve relative to the source.
pub fn emit_with_assets(
    document: &Document,
    resolver: &dyn crate::source::AssetResolver,
    source_name: Option<&std::ffi::OsStr>,
) -> Result<GeneratedPackage, Error> {
    let fonts = crate::measurement::SystemFontResolver::new();
    emit_with_fonts(document, resolver, source_name, Some(&fonts))
}
pub fn emit_with_fonts(
    document: &Document,
    resolver: &dyn crate::source::AssetResolver,
    source_name: Option<&std::ffi::OsStr>,
    fonts: Option<&dyn crate::measurement::FontResolver>,
) -> Result<GeneratedPackage, Error> {
    crate::charts::validate_document(document).map_err(|error| Error(error.to_string()))?;
    let mut measurer = crate::measurement::TextMeasurer::new(document, resolver, source_name);
    if let Some(fonts) = fonts {
        measurer = measurer.with_font_resolver(fonts);
    }
    let mut deck = Deck::resolve(document, &mut measurer).map_err(Error)?;
    let images = crate::media::ImageCatalog::load_selected(
        crate::presentation::images::elements(&deck),
        resolver,
        source_name,
    );
    let bytes = package::write(document, &mut deck, &mut measurer, &images)?;
    deck.warnings.extend(images.warnings);
    deck.warnings.sort();
    deck.warnings.dedup();
    Ok(GeneratedPackage {
        bytes,
        warnings: deck.warnings,
    })
}
fn escape(value: &str) -> Result<String, Error> {
    Ok(crate::xml::escape(value)?)
}
use crate::xml::PRESENTATION_ROOT_ATTRIBUTES as ROOT_ATTRIBUTES;
const GROUP: &str = r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr>"#;
