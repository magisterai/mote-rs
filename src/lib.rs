//! Mote's source language core. See `docs/implementation.md` for coverage.

pub mod evaluation;
pub mod json;
pub mod model;
pub mod syntax;

/// Parse and evaluate source without selecting an output format or laying out pages.
pub fn compile(source: &str) -> Result<model::Document, syntax::Diagnostic> {
    evaluation::evaluate(&syntax::parse(source)?)
}
