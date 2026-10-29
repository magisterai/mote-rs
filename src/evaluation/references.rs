//! Anchor declarations and references retain authoring intent until output resolution.
use super::{
    ContentMode, EvaluationContext, EvaluationState, attributes::evaluate_attributes,
    evaluate_body, expressions::evaluate_expression,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Element, ElementKind, Value},
    syntax::Call,
};
pub(super) fn evaluate(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState,
) -> Result<Content, Diagnostic> {
    let kind = match call.name.as_str() {
        "bookmark" => ElementKind::Bookmark,
        "label" => ElementKind::Label,
        _ => ElementKind::Reference,
    };
    let primary = if kind == ElementKind::Reference {
        "target"
    } else {
        "name"
    };
    let positional: Vec<_> = call
        .arguments
        .iter()
        .filter(|argument| argument.name.is_none())
        .collect();
    if positional.len() > 1
        || (!positional.is_empty()
            && call
                .arguments
                .iter()
                .any(|argument| argument.name.as_deref() == Some(primary)))
    {
        return Err(Diagnostic::new(
            span,
            "supply the primary argument once, positionally or by name",
        ));
    }
    let body_allowed = kind == ElementKind::Bookmark || call.from_markdown;
    if call.bodies.len() > usize::from(body_allowed) {
        return Err(Diagnostic::new(
            span,
            format!("{} does not accept this content body", call.name),
        ));
    }
    let mut attributes = evaluate_attributes(kind, &call.arguments, state)?;
    if let Some(argument) = positional.first() {
        let value = evaluate_expression(&argument.value, state)?;
        if !matches!(value, Value::String(_)) {
            return Err(Diagnostic::new(
                argument.span,
                "positional name or target must be a string",
            ));
        }
        attributes.insert(primary.into(), value);
    }
    if !attributes.contains_key(primary) {
        return Err(Diagnostic::new(
            span,
            format!("{} requires {primary}", call.name),
        ));
    }
    if kind == ElementKind::Reference {
        if !matches!(
            attributes.get(primary),
            Some(Value::String(_) | Value::Object(_))
        ) || crate::references::target(&attributes).is_empty()
        {
            return Err(Diagnostic::new(span, "ref requires a target label"));
        }
    } else if kind == ElementKind::Label && attributes[primary].to_text().trim().is_empty() {
        return Err(Diagnostic::new(span, "label requires a non-empty name"));
    }
    if kind == ElementKind::Bookmark && attributes[primary].to_text().is_empty() {
        return Err(Diagnostic::new(span, "bookmark requires a name"));
    }
    if kind == ElementKind::Label {
        let name = attributes[primary].to_text();
        state.labels.push(NamedLocation {
            name,
            span,
            story_declaration: state.story_declaration,
        });
    } else if kind == ElementKind::Reference {
        let target = match &attributes[primary] {
            Value::Object(fields) => fields
                .get("label")
                .or_else(|| fields.get("name"))
                .map(Value::to_text)
                .unwrap_or_default(),
            value => value.to_text(),
        };
        state.references.push(NamedLocation {
            name: target,
            span,
            story_declaration: state.story_declaration,
        });
    }
    let children = call
        .bodies
        .first()
        .map(|body| evaluate_body(body, ContentMode::Inline, context, state))
        .transpose()?
        .unwrap_or_default();
    Ok(Content::Element(Element {
        kind,
        attributes,
        children,
    }))
}

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
