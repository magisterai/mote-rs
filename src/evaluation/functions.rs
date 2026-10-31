//! Document functions capture lexical values and accept evaluated content.
use super::{
    ContentMode, EvaluationContext, EvaluationState,
    bindings::Bindings,
    evaluate_body,
    expressions::{EvaluatedArguments, evaluate_arguments},
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Value},
    syntax::Call,
};
use std::rc::Rc;

pub(super) struct DocumentFunction {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: String,
    pub environment: Bindings,
    pub span: Span,
    pub source_line: usize,
    pub source: std::sync::Arc<crate::source::SourceFile>,
}
impl DocumentFunction {
    pub fn accepts_body(&self) -> bool {
        self.parameters.last().is_some_and(|name| name == "body")
    }
    pub fn bind_arguments(
        &self,
        mut arguments: EvaluatedArguments,
        body: Option<Vec<Content>>,
        span: Span,
    ) -> Result<Vec<Value>, Diagnostic> {
        let count = self.parameters.len() - usize::from(self.accepts_body());
        if arguments.positional.len() > count {
            return Err(Diagnostic::new(
                span,
                "too many positional function arguments",
            ));
        }
        if body.is_some() && !self.accepts_body() {
            return Err(Diagnostic::new(span, "function does not accept a body"));
        }
        let mut values = Vec::new();
        let mut positional = arguments.positional.into_iter();
        for name in &self.parameters[..count] {
            let positional_value = positional.next();
            let named_value = arguments.named.remove(name);
            if positional_value.is_some() && named_value.is_some() {
                return Err(Diagnostic::new(
                    span,
                    format!("parameter {name:?} was supplied both positionally and by name"),
                ));
            }
            values.push(
                positional_value
                    .or(named_value)
                    .map(|argument| argument.value)
                    .unwrap_or(Value::Nil),
            );
        }
        if let Some((name, argument)) = arguments.named.first_key_value() {
            return Err(Diagnostic::new(
                argument.span,
                format!("unknown function parameter {name:?}"),
            ));
        }
        if self.accepts_body() {
            values.push(Value::Content(body.unwrap_or_default()));
        }
        Ok(values)
    }
}

pub(super) fn define(
    name: &str,
    parameters: &[String],
    body: &str,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    if !state.enable_lua {
        return Err(Diagnostic::new(
            span,
            "function definitions require Lua to be enabled",
        ));
    }
    let function = DocumentFunction {
        name: name.to_owned(),
        parameters: parameters.to_vec(),
        body: body.to_owned(),
        environment: state.bindings.clone(),
        span,
        source: state.source.clone(),
        source_line: context.source[..span.start]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count()
            + 1,
    };
    state
        .bindings
        .set_function(name.to_owned(), Rc::new(function));
    Ok(())
}

pub(super) fn evaluate(
    function: &DocumentFunction,
    call: &Call,
    span: Span,
    context: EvaluationContext<'_>,
    state: &mut EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    if call.bodies.len() > 1 {
        return Err(Diagnostic::new(
            span,
            "function accepts at most one content body",
        ));
    }
    let arguments = evaluate_arguments(&call.arguments, state)?;
    let body = call
        .bodies
        .first()
        .map(|body| evaluate_body(body, ContentMode::Any, context, state))
        .transpose()?;
    let arguments = function.bind_arguments(arguments, body, span)?;
    super::scripting::call(function, arguments, state)
}
