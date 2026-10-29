use super::{
    ContentMode, EvaluationContext, EvaluationState, attributes::evaluate_attributes,
    evaluate_nodes,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Element, ElementKind},
    syntax::Call,
};
pub(super) fn evaluate(
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState,
) -> Result<Content, Diagnostic> {
    if call.bodies.len() != 1
        || call
            .arguments
            .iter()
            .any(|argument| argument.name.is_none())
    {
        return Err(Diagnostic::new(
            span,
            format!("{} requires one body and named arguments only", call.name),
        ));
    }
    let kind = if call.name == "figure" {
        ElementKind::Figure
    } else {
        ElementKind::Caption
    };
    let attributes = evaluate_attributes(kind, &call.arguments)?;
    let children = evaluate_nodes(
        &call.bodies[0],
        if kind == ElementKind::Figure {
            ContentMode::Block
        } else {
            ContentMode::Inline
        },
        context,
        state,
    )?;
    if kind == ElementKind::Figure && children.iter().filter(|child| matches!(child, Content::Element(element) if element.kind == ElementKind::Caption)).count() > 1 {
        return Err(Diagnostic::new(span, "figure can contain at most one caption"));
    }
    Ok(Content::Element(Element {
        kind,
        attributes,
        children,
    }))
}
