//! Mote's source language core. See `docs/implementation.md` for coverage.

pub mod diagnostics;
pub mod docx;
pub mod evaluation;
pub mod formatting;
pub mod json;
pub mod layout;
pub mod measurement;
pub mod model;
pub mod render;
pub mod source;
pub mod syntax;

/// Parse and evaluate source without selecting an output format or laying out pages.
pub fn compile(source: &str) -> Result<model::Document, diagnostics::Diagnostic> {
    evaluation::evaluate(&syntax::parse(source)?)
}
