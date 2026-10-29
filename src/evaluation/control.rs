//! Branches preserve their surrounding flow; each loop iteration owns its scope.
use super::{
    ContentMode, EvaluationContext, EvaluationState, evaluate_body, evaluate_nodes,
    expressions::evaluate_expression,
};
use crate::{
    diagnostics::Diagnostic,
    model::{Content, Value},
    syntax::{Node, NodeKind},
};

pub(super) fn evaluate(
    node: &Node,
    mode: ContentMode,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<Option<Vec<Content>>, Diagnostic> {
    match &node.kind {
        NodeKind::Conditional {
            condition,
            consequence,
            alternative,
        } => {
            let value = evaluate_expression(condition, state)?;
            let body = if is_truthy(&value) {
                Some(consequence)
            } else {
                alternative.as_ref()
            };
            let mode = if mode.is_block() {
                ContentMode::Flow
            } else {
                mode
            };
            body.map(|body| evaluate_body(body, mode, context, state))
                .transpose()
        }
        NodeKind::Iteration {
            binding,
            iterable,
            body,
        } => {
            let values = match evaluate_expression(iterable, state)? {
                Value::Array(values) => values,
                Value::Object(fields) => fields.into_values().collect(),
                _ => {
                    return Err(Diagnostic::new(
                        iterable.span,
                        "for iterable must be an array or object",
                    ));
                }
            };
            let mode = if mode.is_block() || mode == ContentMode::Flow {
                ContentMode::Block
            } else {
                mode
            };
            let mut content = Vec::new();
            for value in values {
                state.bindings.enter();
                state.bindings.set(binding.clone(), value);
                let result = evaluate_nodes(body, mode, context, state);
                state.bindings.leave();
                content.extend(result?);
            }
            Ok(Some(content))
        }
        _ => unreachable!(),
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Content(content) => !content.is_empty(),
        Value::Element(_) => true,
        Value::Boolean(value) => *value,
        Value::String(value) | Value::Color(value) => !value.is_empty(),
        Value::Number(value) | Value::Length { value, .. } => *value != 0.0,
        Value::Array(values) => !values.is_empty(),
        Value::Object(fields) => !fields.is_empty(),
    }
}
