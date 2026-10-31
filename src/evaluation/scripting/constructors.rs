//! Lua argument tables and content values use the shared element constructors.
use super::values;
use crate::{
    diagnostics::Span,
    evaluation::{
        EvaluationState,
        constructors::Signature,
        expressions::{EvaluatedArgument, EvaluatedArguments},
    },
    model::Value,
};

pub(super) fn call(
    name: &str,
    arguments: mlua::Variadic<mlua::Value>,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> mlua::Result<Value> {
    if name == "import" {
        let mut arguments = arguments
            .into_iter()
            .map(values::from_lua)
            .collect::<mlua::Result<Vec<_>>>()?
            .into_iter()
            .peekable();
        let named = if matches!(arguments.peek(), Some(Value::Object(_))) {
            let Some(Value::Object(fields)) = arguments.next() else {
                unreachable!()
            };
            fields
                .into_iter()
                .map(|(name, value)| (name, EvaluatedArgument::new(value, span)))
                .collect()
        } else {
            Default::default()
        };
        let arguments = EvaluatedArguments {
            named,
            positional: arguments
                .map(|value| EvaluatedArgument::new(value, span))
                .collect(),
        };
        return crate::evaluation::data::evaluate(name, arguments, span, state)
            .map_err(mlua::Error::external);
    }
    if super::HELPERS.contains(&name) || name == "in" {
        if arguments.len() != 1 {
            return Err(mlua::Error::RuntimeError(format!(
                "{name} requires one positional argument"
            )));
        }
        let argument = values::from_lua(arguments.into_iter().next().unwrap())?;
        return crate::evaluation::expressions::evaluate_helper_value(name, argument, span, state)
            .map_err(mlua::Error::external);
    }
    if matches!(
        name,
        "document" | "presentation" | "layout" | "font" | "style" | "header" | "footer"
    ) {
        return declaration(name, arguments, span, state);
    }
    let signature = Signature::find(name).ok_or_else(|| {
        mlua::Error::RuntimeError(format!("unsupported document constructor: {name}"))
    })?;
    let mut values = arguments
        .into_iter()
        .map(values::from_lua)
        .collect::<mlua::Result<Vec<_>>>()?
        .into_iter()
        .peekable();
    let mut arguments = EvaluatedArguments::default();
    if matches!(values.peek(), Some(Value::Object(_))) {
        let Value::Object(fields) = values.next().unwrap() else {
            unreachable!()
        };
        arguments.named = fields
            .into_iter()
            .map(|(name, value)| (name, EvaluatedArgument::new(value, span)))
            .collect();
    }
    if !signature.body_allowed() {
        arguments.positional = values
            .map(|value| EvaluatedArgument::new(value, span))
            .collect();
        return signature
            .prepare(
                arguments,
                0,
                false,
                span,
                &state.bindings.element_defaults(signature.kind),
            )
            .and_then(|prepared| prepared.finish(Vec::new(), span, state))
            .map(|element| Value::Element(Box::new(element)))
            .map_err(mlua::Error::external);
    }
    if let Some(primary) = signature.primary()
        && !arguments.named.contains_key(primary)
        && let Some(value) = values.next()
    {
        arguments
            .positional
            .push(EvaluatedArgument::new(value, span));
    }
    let has_body = values.peek().is_some();
    let children = values.flat_map(super::values::into_content).collect();
    let prepared = signature
        .prepare(
            arguments,
            usize::from(has_body),
            false,
            span,
            &state.bindings.element_defaults(signature.kind),
        )
        .map_err(mlua::Error::external)?;
    let preserve_space = prepared.preserve_space(false);
    let mut element = prepared
        .finish(children, span, state)
        .map_err(mlua::Error::external)?;
    if preserve_space {
        crate::evaluation::normalize_preserved_content(&mut element.children);
    }
    Ok(Value::Element(Box::new(element)))
}

fn declaration(
    name: &str,
    arguments: mlua::Variadic<mlua::Value>,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> mlua::Result<Value> {
    let mut arguments = arguments
        .into_iter()
        .map(values::from_lua)
        .collect::<mlua::Result<Vec<_>>>()?
        .into_iter()
        .peekable();
    let named = if matches!(arguments.peek(), Some(Value::Object(_))) {
        let Value::Object(fields) = arguments.next().unwrap() else {
            unreachable!()
        };
        fields
            .into_iter()
            .map(|(name, value)| (name, EvaluatedArgument::new(value, span)))
            .collect()
    } else {
        Default::default()
    };
    if name == "layout" {
        crate::evaluation::presentation_layouts::apply(
            named,
            arguments.flat_map(values::into_content).collect(),
            span,
            state,
        )
        .map_err(mlua::Error::external)?;
    } else if matches!(name, "header" | "footer") {
        if arguments.peek().is_none() {
            return Err(mlua::Error::RuntimeError(format!(
                "{name} requires one content body"
            )));
        }
        let properties = crate::evaluation::sections::story_properties(name, named, span)
            .map_err(mlua::Error::external)?;
        state.set_story(
            name == "header",
            crate::model::HeaderFooter {
                declaration_id: span.start + 1,
                properties,
                content: arguments.flat_map(values::into_content).collect(),
            },
        );
    } else {
        if arguments.next().is_some() {
            return Err(mlua::Error::RuntimeError(format!(
                "{name} accepts named arguments only"
            )));
        }
        if name == "presentation" {
            crate::evaluation::presentation::apply(named, state).map_err(mlua::Error::external)?;
        } else if name == "document" {
            crate::evaluation::configuration::apply(named, state).map_err(mlua::Error::external)?;
        } else {
            crate::evaluation::declarations::apply(name, named, span, state)
                .map_err(mlua::Error::external)?;
        }
    }
    Ok(Value::Nil)
}

/// Global helpers pass positional Lua values; only document constructors interpret a property table.
pub(super) fn helper(
    name: &str,
    arguments: mlua::Variadic<mlua::Value>,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> mlua::Result<Value> {
    if name == "import" {
        let positional = arguments
            .into_iter()
            .map(|value| values::from_lua(value).map(|value| EvaluatedArgument::new(value, span)))
            .collect::<mlua::Result<_>>()?;
        return crate::evaluation::data::evaluate(
            name,
            EvaluatedArguments {
                positional,
                named: Default::default(),
            },
            span,
            state,
        )
        .map_err(mlua::Error::external);
    }
    call(name, arguments, span, state)
}
