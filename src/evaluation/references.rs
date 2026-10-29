//! Anchor declarations and references retain authoring intent until output resolution.
use super::{
    ContentMode, EvaluationContext, EvaluationState, attributes::evaluate_attributes,
    evaluate_nodes, expressions::evaluate_expression,
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
    let mut attributes = evaluate_attributes(kind, &call.arguments)?;
    if let Some(argument) = positional.first() {
        let value = evaluate_expression(&argument.value)?;
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
        if state.labels.insert(name.clone(), span).is_some() {
            return Err(Diagnostic::new(
                span,
                format!("label {name:?} is declared more than once"),
            ));
        }
    } else if kind == ElementKind::Reference {
        let target = match &attributes[primary] {
            Value::Object(fields) => fields
                .get("label")
                .or_else(|| fields.get("name"))
                .map(Value::to_text)
                .unwrap_or_default(),
            value => value.to_text(),
        };
        state.references.push((target, span));
    }
    let children = call
        .bodies
        .first()
        .map(|body| evaluate_nodes(body, ContentMode::Inline, context, state))
        .transpose()?
        .unwrap_or_default();
    Ok(Content::Element(Element {
        kind,
        attributes,
        children,
    }))
}
