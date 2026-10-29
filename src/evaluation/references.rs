//! Anchor declarations and references retain authoring intent until output resolution.
use super::EvaluationState;
use crate::diagnostics::{Diagnostic, Span};
/// Declaration provenance distinguishes inherited stories from discarded overrides.
pub(super) struct NamedLocation {
    pub name: String,
    pub span: Span,
    pub story_declaration: Option<usize>,
}
pub(super) fn validate(state: &EvaluationState) -> Result<(), Diagnostic> {
    let active_stories: std::collections::BTreeSet<_> = state
        .sections
        .iter()
        .flat_map(|section| [section.header.declaration_id, section.footer.declaration_id])
        .collect();
    let active = |location: &&NamedLocation| {
        location
            .story_declaration
            .is_none_or(|identifier| active_stories.contains(&identifier))
    };
    let mut labels = std::collections::BTreeSet::new();
    for declaration in state.labels.iter().filter(active) {
        if !labels.insert(&declaration.name) {
            return Err(Diagnostic::new(
                declaration.span,
                format!("label {:?} is declared more than once", declaration.name),
            ));
        }
    }
    for reference in state.references.iter().filter(active) {
        if !labels.contains(&reference.name) {
            return Err(Diagnostic::new(
                reference.span,
                format!(
                    "ref target {} is not labeled in this document",
                    reference.name
                ),
            ));
        }
    }
    Ok(())
}
