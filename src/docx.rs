//! Editable Word document generation. Layout and pagination are performed by Word.

mod block_quotes;
mod builtin_styles;
mod captions;
mod configuration;
mod controls;
mod document;
mod fonts;
mod headings;
mod images;
mod list_layout;
mod lists;
mod notes;
mod numbering;
mod package;
mod paragraph_properties;
mod paragraphs;
mod references;
mod resources;
mod run_properties;
mod runs;
mod sections;
mod styles;
mod tables;
mod values;
mod whitespace;
mod xml;

use crate::model::{Document, Properties};
use std::{error::Error as StandardError, fmt};

#[derive(Debug)]
pub enum Error {
    Unsupported(String),
    InvalidValue(String),
    Io(std::io::Error),
    Archive(zip::result::ZipError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(message) => write!(formatter, "DOCX does not yet support {message}"),
            Self::InvalidValue(message) => formatter.write_str(message),
            Self::Io(error) => error.fmt(formatter),
            Self::Archive(error) => error.fmt(formatter),
        }
    }
}

impl StandardError for Error {
    fn source(&self) -> Option<&(dyn StandardError + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Archive(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
impl From<zip::result::ZipError> for Error {
    fn from(error: zip::result::ZipError) -> Self {
        Self::Archive(error)
    }
}

/// Generate a complete package in memory. Unsupported content fails before a
/// caller needs to create or replace an output file. The authored model is unchanged.
pub fn emit(document: &Document) -> Result<Vec<u8>, Error> {
    let resolver = crate::source::FileAssetResolver::new(".")?;
    Ok(emit_with_assets(document, &resolver, None)?.bytes)
}

pub struct GeneratedPackage {
    pub bytes: Vec<u8>,
    pub warnings: Vec<String>,
}

/// Resolve media once and retain diagnostics separately from the generated package.
pub fn emit_with_assets(
    document: &Document,
    resolver: &dyn crate::source::AssetResolver,
    source_name: Option<&std::ffi::OsStr>,
) -> Result<GeneratedPackage, Error> {
    let styles = styles::Styles::new(document)?;
    let font_table = fonts::write_table(document)?;
    let styles_xml = styles.write()?;
    let mut state = document::DocumentState::default();
    state.resources.anchors = crate::references::Catalog::build(document);
    state.resources.images = crate::media::ImageCatalog::load(document, resolver, source_name);
    let body = document::write_document(document, &styles, &mut state)?;
    notes::write_parts(&styles, &mut state)?;
    let bytes = package::write(
        &body,
        &styles_xml,
        &font_table,
        &state.numbering.write()?,
        &state.resources.relationships()?,
        &state.resources.media,
        &state.resources.stories,
    )?;
    state
        .resources
        .warnings
        .extend(state.resources.images.warnings);
    state.resources.warnings.sort();
    state.resources.warnings.dedup();
    Ok(GeneratedPackage {
        bytes,
        warnings: state.resources.warnings,
    })
}

fn check_attributes(
    properties: &Properties,
    supported: &[&str],
    context: &str,
) -> Result<(), Error> {
    for name in properties.keys() {
        if !supported.contains(&name.as_str()) {
            return Err(Error::Unsupported(format!("{context} attribute {name:?}")));
        }
    }
    Ok(())
}
