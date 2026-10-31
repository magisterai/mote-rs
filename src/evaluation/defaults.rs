//! Progressive element defaults share ordinary attribute normalization.
use super::{EvaluationState, attributes, constructors::Signature, expressions};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::ElementKind,
    syntax::Call,
};

pub(super) fn evaluate(
    call: &Call,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    let kind = if call.name == "list_item" {
        ElementKind::ListItem
    } else {
        Signature::find(&call.name)
            .map(|signature| signature.kind)
            .ok_or_else(|| {
                Diagnostic::new(
                    span,
                    format!("set target {:?} is not a supported element", call.name),
                )
            })?
    };
    let arguments = expressions::evaluate_arguments(&call.arguments, state)?;
    if !arguments.positional.is_empty() {
        return Err(Diagnostic::new(span, "set accepts named properties only"));
    }
    let properties = attributes::normalize_defaults(kind, arguments)?;
    state.bindings.set_defaults(kind, &properties);
    Ok(())
}
