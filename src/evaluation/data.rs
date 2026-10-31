//! Typed external values through the compilation's selected resource resolver.
mod csv;
use super::{EvaluationState, expressions::EvaluatedArguments};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::Value,
    source::{AssetRequest, resolve_asset},
};
use std::ffi::OsStr;

pub(super) fn load_json(
    path: Value,
    span: Span,
    state: &EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    let Value::String(path) = path else {
        return Err(Diagnostic::new(span, "json path must be a string"));
    };
    let resolver = state.assets.ok_or_else(|| {
        Diagnostic::new(
            span,
            "json requires an asset resolver; use compile_with_assets",
        )
    })?;
    let asset = resolve_asset(
        resolver,
        AssetRequest {
            reference: OsStr::new(&path),
            source_name: state.source_name(),
        },
    )
    .map_err(|error| Diagnostic::new(span, format!("cannot load JSON {path:?}: {error}")))?;
    let decoded = serde_json::from_slice(&asset.bytes).map_err(|error| {
        Diagnostic::new(span, format!("invalid JSON in {:?}: {error}", asset.name))
    })?;
    from_json(decoded).map_err(|message| {
        Diagnostic::new(span, format!("invalid JSON in {:?}: {message}", asset.name))
    })
}

fn from_json(value: serde_json::Value) -> Result<Value, &'static str> {
    Ok(match value {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(value) => Value::Boolean(value),
        serde_json::Value::String(value) => Value::String(value),
        serde_json::Value::Number(value) => Value::Number(
            value
                .as_f64()
                .filter(|number| number.is_finite())
                .ok_or("number must be finite")?,
        ),
        serde_json::Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(from_json)
                .collect::<Result<_, _>>()?,
        ),
        serde_json::Value::Object(fields) => Value::Object(
            fields
                .into_iter()
                .map(|(name, value)| Ok((name, from_json(value)?)))
                .collect::<Result<_, &'static str>>()?,
        ),
    })
}

pub(super) fn evaluate(
    name: &str,
    mut arguments: EvaluatedArguments,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    if arguments.positional.len() != 1 {
        return Err(Diagnostic::new(
            span,
            format!("{name} requires one positional string path"),
        ));
    }
    if let Some((argument, value)) = arguments
        .named
        .iter()
        .find(|(argument, _)| name != "csv" || argument.as_str() != "heading")
    {
        return Err(Diagnostic::new(
            value.span,
            format!("unknown {name} argument {argument:?}"),
        ));
    }
    let Value::String(path) = arguments.positional.remove(0).value else {
        return Err(Diagnostic::new(
            span,
            format!("{name} path must be a string"),
        ));
    };
    if name == "import" {
        super::imports::load(&path, span, state)?;
        return Ok(Value::Nil);
    }
    let heading = arguments
        .named
        .remove("heading")
        .map(|argument| {
            super::values::normalize_boolean(argument.value, argument.span, "csv heading")
        })
        .transpose()?
        == Some(Value::Boolean(true));
    let resolver = state.assets.ok_or_else(|| {
        Diagnostic::new(
            span,
            "csv requires an asset resolver; use compile_with_assets",
        )
    })?;
    let asset = resolve_asset(
        resolver,
        AssetRequest {
            reference: OsStr::new(&path),
            source_name: state.source_name(),
        },
    )
    .map_err(|error| Diagnostic::new(span, format!("cannot load CSV {path:?}: {error}")))?;
    let text = std::str::from_utf8(&asset.bytes).map_err(|error| {
        Diagnostic::new(span, format!("invalid CSV in {:?}: {error}", asset.name))
    })?;
    csv::decode(text, heading).map_err(|message| {
        Diagnostic::new(span, format!("invalid CSV in {:?}: {message}", asset.name))
    })
}
