//! Ordered layout declarations, using the normal slide and box property contracts.
use super::{
    ContentMode, EvaluationContext, EvaluationState, attributes, evaluate_body,
    expressions::{EvaluatedArgument, EvaluatedArguments, evaluate_named_arguments},
    values::normalize_identifier,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, ElementKind, PresentationLayout, PresentationLayoutSlot, Value},
    syntax::Call,
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn evaluate(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    if call.bodies.len() > 1
        || call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
    {
        return Err(Diagnostic::new(
            span,
            "layout accepts named arguments and at most one block body",
        ));
    }
    let arguments = evaluate_named_arguments(&call.arguments, state)?;
    let children = if let Some(body) = call.bodies.first() {
        evaluate_body(body, ContentMode::Block, context, state)?
    } else {
        Vec::new()
    };
    apply(arguments, children, span, state)
}
pub(super) fn apply(
    mut arguments: BTreeMap<String, EvaluatedArgument>,
    children: Vec<Content>,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    let name = arguments
        .get("name")
        .ok_or_else(|| Diagnostic::new(span, "layout requires name"))?;
    let name = normalize_identifier(name.value.clone(), name.span, "layout name")?
        .trim()
        .to_owned();
    if name.is_empty() {
        return Err(Diagnostic::new(span, "layout name must not be empty"));
    }
    if crate::presentation::layouts::is_builtin(&name) {
        return Err(Diagnostic::new(
            span,
            format!("layout name {name:?} is reserved for a built-in presentation layout"),
        ));
    }
    let based_on = if let Some(argument) = arguments.remove("based_on") {
        let Value::String(name) = argument.value else {
            return Err(Diagnostic::new(
                argument.span,
                "layout based_on must be a string",
            ));
        };
        name.trim().to_owned()
    } else {
        String::new()
    };
    let mut properties = attributes::normalize_arguments(
        ElementKind::Slide,
        EvaluatedArguments {
            named: arguments,
            positional: Vec::new(),
        },
        &Default::default(),
    )?;
    properties.remove("name");
    properties.remove("based_on");
    let mut slots = Vec::new();
    let mut names = BTreeSet::new();
    for child in children {
        if matches!(&child, Content::Text(text) if text.trim().is_empty()) {
            continue;
        }
        let Content::Element(element) = child else {
            return Err(Diagnostic::new(
                span,
                "layout can only contain box slot declarations",
            ));
        };
        if element.kind != ElementKind::Box {
            return Err(Diagnostic::new(
                span,
                "layout can only contain box slot declarations",
            ));
        }
        let name = crate::properties::text(&element.attributes, "slot");
        if name.is_empty() {
            return Err(Diagnostic::new(
                span,
                "layout box requires a non-empty slot name",
            ));
        }
        if !names.insert(name.clone()) {
            return Err(Diagnostic::new(
                span,
                format!("layout declares slot {name:?} more than once"),
            ));
        }
        if element
            .children
            .iter()
            .any(|child| !matches!(child, Content::Text(text) if text.trim().is_empty()))
        {
            return Err(Diagnostic::new(
                span,
                format!("layout slot {name:?} is a declaration and must not contain content"),
            ));
        }
        let mut properties = element.attributes;
        let kind = properties
            .remove("kind")
            .map(|value| value.to_text())
            .unwrap_or_default();
        let required = properties
            .remove("required")
            .map(|value| value == Value::Boolean(true));
        let remove = properties.remove("remove") == Some(Value::Boolean(true));
        properties.remove("slot");
        if remove && (!properties.is_empty() || !kind.is_empty() || required.is_some()) {
            return Err(Diagnostic::new(
                span,
                format!("layout slot {name:?} cannot set properties when remove = true"),
            ));
        }
        slots.push(PresentationLayoutSlot {
            name,
            kind,
            required,
            remove,
            properties,
        });
    }
    let declaration = PresentationLayout {
        name,
        based_on,
        properties,
        slots,
    };
    let layouts = &mut state.presentation.layouts;
    if let Some(existing) = layouts
        .iter_mut()
        .find(|existing| existing.name == declaration.name)
    {
        *existing = declaration;
    } else {
        layouts.push(declaration);
    }
    Ok(())
}
