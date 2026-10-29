//! Lexical values; output declarations remain in the document state.
use super::{EvaluationState, expressions::evaluate_expression};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Properties, Value},
    syntax::Call,
};

#[derive(Clone, Default)]
struct Scope {
    values: Properties,
    functions: std::collections::BTreeMap<String, std::rc::Rc<super::functions::DocumentFunction>>,
}
#[derive(Clone)]
pub(super) struct Bindings {
    scopes: Vec<Scope>,
}
impl Default for Bindings {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::default()],
        }
    }
}
impl Bindings {
    pub fn visible_values(&self) -> Properties {
        let mut values = Properties::new();
        for scope in &self.scopes {
            values.extend(scope.values.clone());
        }
        values
    }
    pub fn resolve_bound_path(&self, path: &str) -> Option<Value> {
        if path.is_empty()
            || !path.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '_' | '.')
            })
        {
            return None;
        }
        let root = path.split('.').next()?;
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.values.get(root))?;
        self.resolve(path, Span { start: 0, end: 0 }).ok()
    }
    pub fn enter(&mut self) {
        self.scopes.push(Scope::default());
    }
    pub fn leave(&mut self) {
        assert!(self.scopes.len() > 1);
        self.scopes.pop();
    }
    pub fn set(&mut self, name: String, value: Value) {
        self.scopes.last_mut().unwrap().values.insert(name, value);
    }
    pub fn set_function(
        &mut self,
        name: String,
        function: std::rc::Rc<super::functions::DocumentFunction>,
    ) {
        self.scopes
            .last_mut()
            .unwrap()
            .functions
            .insert(name, function);
    }
    pub fn function(&self, name: &str) -> Option<std::rc::Rc<super::functions::DocumentFunction>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.functions.get(name))
            .cloned()
    }
    pub fn visible_functions(
        &self,
    ) -> std::collections::BTreeMap<String, std::rc::Rc<super::functions::DocumentFunction>> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.functions.clone())
            .collect()
    }
    pub fn resolve(&self, path: &str, span: Span) -> Result<Value, Diagnostic> {
        let mut components = path.split('.');
        let root = components.next().unwrap_or_default();
        let Some(mut value) = self
            .scopes
            .iter()
            .rev()
            .find_map(|scope| scope.values.get(root))
        else {
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
    state: &mut EvaluationState<'_>,
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
