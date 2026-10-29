//! Lexical values; output declarations remain in the document state.
use super::{EvaluationState, expressions::evaluate_expression};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Properties, Value},
    syntax::Call,
};

pub(super) struct Bindings {
    scopes: Vec<Properties>,
}
impl Default for Bindings {
    fn default() -> Self {
        Self {
            scopes: vec![Properties::new()],
        }
    }
}
impl Bindings {
    pub fn enter(&mut self) {
        self.scopes.push(Properties::new());
    }
    pub fn leave(&mut self) {
        assert!(self.scopes.len() > 1);
        self.scopes.pop();
    }
    pub fn set(&mut self, name: String, value: Value) {
        self.scopes.last_mut().unwrap().insert(name, value);
    }
    pub fn resolve(&self, path: &str, span: Span) -> Result<Value, Diagnostic> {
        let mut components = path.split('.');
        let root = components.next().unwrap_or_default();
        let Some(mut value) = self.scopes.iter().rev().find_map(|scope| scope.get(root)) else {
            return match path {
                "true" => Ok(Value::Boolean(true)),
                "false" => Ok(Value::Boolean(false)),
                "nil" => Ok(Value::Nil),
                _ => Err(Diagnostic::new(span, format!("unknown binding: {root}"))),
            };
        };
        for component in components {
            value = match value {
                Value::Object(fields) => fields.get(component),
                Value::Array(entries) => component
                    .parse::<usize>()
                    .ok()
                    .and_then(|index| entries.get(index)),
                _ => None,
            }
            .ok_or_else(|| Diagnostic::new(span, format!("cannot resolve binding path: {path}")))?;
        }
        Ok(value.clone())
    }
}
pub(super) fn evaluate(
    call: &Call,
    span: Span,
    state: &mut EvaluationState,
) -> Result<(), Diagnostic> {
    if !call.bodies.is_empty() {
        return Err(Diagnostic::new(span, "let does not accept a body"));
    }
    for argument in &call.arguments {
        let Some(name) = &argument.name else {
            return Err(Diagnostic::new(argument.span, "let bindings must be named"));
        };
        let value = evaluate_expression(&argument.value, state)?;
        state.bindings.set(name.clone(), value);
    }
    Ok(())
}
