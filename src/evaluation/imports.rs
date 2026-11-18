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
    let source = Arc::new(resolve(path, span, state)?);
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

/// Finds the source that a path names: beside the importing source, then in
/// each directory of the import path, then in the library that Mote carries.
/// A path without the `.mote` extension is tried as written and then with it.
fn resolve(
    path: &str,
    span: Span,
    state: &EvaluationState<'_>,
) -> Result<source::SourceFile, Diagnostic> {
    let mut names = vec![path.to_owned()];
    if !path.ends_with(".mote") {
        names.push(format!("{path}.mote"));
    }
    // A source of the library reaches only the library, since no resolver knows its name.
    if let Some(directory) = crate::library::directory(&state.source.name) {
        return names
            .iter()
            .flat_map(|name| [format!("{directory}{name}"), name.clone()])
            .find_map(|name| crate::library::source(&name))
            .ok_or_else(|| {
                Diagnostic::new(
                    span,
                    format!("cannot import {path:?}: the Mote library holds no such source"),
                )
            });
    }
    let mut missing = None;
    if let Some(resolver) = state.assets {
        let beside = names.iter().map(|name| (name.clone(), state.source_name()));
        let on_path = state.import_paths.iter().flat_map(|directory| {
            names
                .iter()
                .map(move |name| (format!("{}/{name}", directory.trim_end_matches('/')), None))
        });
        for (reference, source_name) in beside.chain(on_path) {
            match source::load_source(
                resolver,
                AssetRequest {
                    reference: OsStr::new(&reference),
                    source_name,
                },
            ) {
                Ok(source) => return Ok(source),
                Err(error) if error.cause.kind() == std::io::ErrorKind::NotFound => {
                    missing.get_or_insert(error);
                }
                Err(error) => {
                    return Err(Diagnostic::new(
                        span,
                        format!("cannot import {path:?}: {error}"),
                    ));
                }
            }
        }
    }
    if let Some(source) = names.iter().find_map(|name| crate::library::source(name)) {
        return Ok(source);
    }
    Err(Diagnostic::new(
        span,
        match missing {
            Some(error) => format!("cannot import {path:?}: {error}"),
            None => "import requires an asset resolver; use compile_with_assets".into(),
        },
    ))
}
