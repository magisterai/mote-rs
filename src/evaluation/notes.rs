//! Inline note references retain block content in a separate story.
use super::{ContentMode, EvaluationContext, EvaluationState, evaluate_body};
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
            format!(
                "{} requires one block body and named arguments only",
                call.name
            ),
        ));
    }
    let kind = if call.name == "footnote" {
        ElementKind::Footnote
    } else {
        ElementKind::Endnote
    };
    let attributes = super::attributes::evaluate_attributes(kind, &call.arguments, state)?;
    let children = evaluate_body(&call.bodies[0], ContentMode::Block, context, state)?;
    Ok(Content::Element(Element {
        kind,
        attributes,
        children,
    }))
}

#[derive(Clone)]
pub(super) struct Definition {
    body: Vec<crate::syntax::Node>,
    content: Option<Vec<Content>>,
    evaluating: bool,
}
pub(super) fn collect_definitions(nodes: &[crate::syntax::Node], state: &mut EvaluationState) {
    use crate::syntax::NodeKind;
    for node in nodes {
        match &node.kind {
            NodeKind::FootnoteDefinition { name, body } => {
                state.markdown_notes.insert(
                    name.clone(),
                    Definition {
                        body: body.clone(),
                        content: None,
                        evaluating: false,
                    },
                );
            }
            NodeKind::Call(call) => {
                for body in &call.bodies {
                    collect_definitions(body, state);
                }
            }
            NodeKind::Heading { body, .. }
            | NodeKind::ListItem { body, .. }
            | NodeKind::Image { label: body, .. }
            | NodeKind::Strong(body)
            | NodeKind::Emphasis(body)
            | NodeKind::Blockquote(body) => collect_definitions(body, state),
            NodeKind::Table(rows) => {
                for row in rows {
                    for cell in &row.cells {
                        collect_definitions(&cell.body, state);
                    }
                }
            }
            _ => {}
        }
    }
}
pub(super) fn markdown_reference(
    name: &str,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState,
) -> Result<Content, Diagnostic> {
    use crate::model::{Properties, Value};
    let children = if let Some(definition) = state.markdown_notes.get(name).cloned() {
        if definition.evaluating {
            return Err(Diagnostic::new(
                span,
                format!("markdown footnote {name:?} references itself recursively"),
            ));
        }
        if let Some(content) = definition.content {
            content
        } else {
            state.markdown_notes.get_mut(name).unwrap().evaluating = true;
            let result = evaluate_body(&definition.body, ContentMode::Block, context, state);
            let definition = state.markdown_notes.get_mut(name).unwrap();
            definition.evaluating = false;
            let content = result?;
            definition.content = Some(content.clone());
            content
        }
    } else {
        vec![Content::Text(name.to_owned())]
    };
    Ok(Content::Element(Element {
        kind: ElementKind::Footnote,
        attributes: Properties::from([("id".into(), Value::String(name.to_owned()))]),
        children,
    }))
}
