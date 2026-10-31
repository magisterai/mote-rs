//! Declaration imports export one local scope and retain the resolved source identity.
use super::{ContentMode, EvaluationContext, EvaluationState, evaluate_nodes, expressions};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::Value,
    source::{self, AssetRequest},
    syntax::Call,
};
use std::{ffi::OsStr, sync::Arc};

pub(super) fn evaluate(
    call: &Call,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    if !call.bodies.is_empty() {
        return Err(Diagnostic::new(span, "import does not accept a body"));
    }
    let mut arguments = expressions::evaluate_arguments(&call.arguments, state)?;
    if !arguments.named.is_empty() || arguments.positional.len() != 1 {
        return Err(Diagnostic::new(
            span,
            "import requires one positional string path",
        ));
    }
    let Value::String(path) = arguments.positional.remove(0).value else {
        return Err(Diagnostic::new(span, "import path must be a string"));
    };
    load(&path, span, state)
}

pub(super) fn load(
    path: &str,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> Result<(), Diagnostic> {
    let resolver = state.assets.ok_or_else(|| {
        Diagnostic::new(
            span,
            "import requires an asset resolver; use compile_with_assets",
        )
    })?;
    let source = Arc::new(
        source::load_source(
            resolver,
            AssetRequest {
                reference: OsStr::new(path),
                source_name: state.source_name(),
            },
        )
        .map_err(|error| Diagnostic::new(span, format!("cannot import {path:?}: {error}")))?,
    );
    if state.importing.contains(&source.name) {
        return Err(Diagnostic::new(
            span,
            format!("import cycle involving {:?}", source.name),
        ));
    }
    if state.imported.contains(&source.name) {
        return Ok(());
    }
    if state.importing.len() >= 64 {
        return Err(Diagnostic::new(span, "import nesting limit exceeded"));
    }
    let tree = if state.enable_lua {
        crate::syntax::parse_with_lua(&source.text)
    } else {
        crate::syntax::parse(&source.text)
    }
    .map_err(|error| error.with_source(source.clone()))?;
    state.imported.insert(source.name.clone());
    state.imports.push(source.name.clone());
    state.importing.push(source.name.clone());
    let caller_source = std::mem::replace(&mut state.source, source.clone());
    state.bindings.enter();
    let warnings_start = state.warnings.len();
    let result = evaluate_nodes(
        &tree.nodes,
        ContentMode::Block,
        EvaluationContext {
            source: &tree.source,
            preserve_space: false,
        },
        state,
    );
    for warning in &mut state.warnings[warnings_start..] {
        *warning = warning.clone().with_source(source.clone());
    }
    if result.is_ok() {
        state.bindings.export_and_leave();
    } else {
        state.bindings.leave();
    }
    state.source = caller_source;
    state.importing.pop();
    result
        .map(|_| ())
        .map_err(|error| error.with_source(source))
}
