//! Mote's source language core. See `docs/implementation.md` for coverage.

pub mod diagnostics;
pub mod docx;
pub mod evaluation;
pub mod formatting;
pub mod json;
pub mod layout;
pub mod measurement;
pub mod media;
pub mod model;
pub mod render;
pub mod source;
pub mod syntax;

/// Parse and evaluate source without selecting an output format or laying out pages.
pub fn compile(source: &str) -> Result<model::Document, diagnostics::Diagnostic> {
    evaluation::evaluate(&syntax::parse(source)?)
}

mod tables;

mod properties;

mod captions;
mod numbering;
mod references;

/// Parse and evaluate with a selected resolver and optional logical source name.
/// Resource requests are cached for this compilation. Custom failures never
/// fall back to the filesystem; use `FileAssetResolver` for local files.
pub fn compile_with_assets(
    source: &str,
    resolver: &dyn source::AssetResolver,
    source_name: Option<&std::ffi::OsStr>,
) -> Result<model::Document, diagnostics::Diagnostic> {
    evaluation::evaluate_with_assets(&syntax::parse(source)?, resolver, source_name)
}
