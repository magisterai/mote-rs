//! Opt-in Lua execution with a compilation-wide clock and scoped host callbacks.
mod values;
use super::{EvaluationState, expressions::evaluate_helper_value};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::Value,
};
use mlua::{HookTriggers, Lua, LuaOptions, StdLib, VmState};
use std::{
    cell::RefCell,
    collections::BTreeSet,
    time::{Duration, Instant},
};

pub(super) struct Runtime {
    lua: Lua,
    deadline: Option<Instant>,
    exposed_values: RefCell<BTreeSet<String>>,
}
impl Runtime {
    fn new(timeout: Option<Duration>) -> mlua::Result<Self> {
        let lua = Lua::new_with(StdLib::NONE, LuaOptions::default())?;
        for name in [
            "_printregs",
            "collectgarbage",
            "dofile",
            "getfenv",
            "load",
            "loadfile",
            "loadstring",
            "print",
            "require",
            "setfenv",
        ] {
            lua.globals().set(name, mlua::Nil)?;
        }
        let deadline = timeout.and_then(|duration| Instant::now().checked_add(duration));
        lua.set_hook(
            HookTriggers::new().every_nth_instruction(1000),
            move |_, _| {
                check_deadline(deadline)?;
                Ok(VmState::Continue)
            },
        )?;
        Ok(Self {
            lua,
            deadline,
            exposed_values: RefCell::new(BTreeSet::new()),
        })
    }
}
fn check_deadline(deadline: Option<Instant>) -> mlua::Result<()> {
    if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        Err(mlua::Error::RuntimeError("Lua time budget exceeded".into()))
    } else {
        Ok(())
    }
}

pub(super) fn evaluate(
    source: &str,
    span: Span,
    state: &EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    if let Some(value) = state.bindings.resolve_bound_path(source) {
        return Ok(value);
    }
    if !state.enable_lua {
        return Err(Diagnostic::new(
            span,
            "Lua expressions are disabled; enable Lua explicitly",
        ));
    }
    if state.lua.get().is_none() {
        let runtime = Runtime::new(state.lua_timeout)
            .map_err(|error| Diagnostic::new(span, error.to_string()))?;
        let _ = state.lua.set(runtime);
    }
    let runtime = state.lua.get().unwrap();
    let result = runtime.lua.scope(|scope| {
        check_deadline(runtime.deadline)?;
        let globals = runtime.lua.globals();
        let visible = state.bindings.visible_values();
        for previous in runtime.exposed_values.borrow().iter() {
            if !visible.contains_key(previous) {
                globals.set(previous.as_str(), mlua::Nil)?;
            }
        }
        for (name, value) in &visible {
            globals.set(name.as_str(), values::to_lua(&runtime.lua, value)?)?;
        }
        *runtime.exposed_values.borrow_mut() = visible.into_keys().collect();
        globals.set(
            "__mote_helper",
            scope.create_function(
                move |lua, (name, arguments): (String, mlua::Variadic<mlua::Value>)| {
                    if !matches!(
                        name.as_str(),
                        "json" | "upper" | "color" | "pt" | "inch" | "cm" | "mm" | "twip"
                    ) || arguments.len() != 1
                    {
                        return Err(mlua::Error::RuntimeError(format!(
                            "{name} requires one positional argument"
                        )));
                    }
                    let argument = values::from_lua(arguments.into_iter().next().unwrap())?;
                    let value = evaluate_helper_value(&name, argument, span, state)
                        .map_err(mlua::Error::external)?;
                    values::to_lua(lua, &value)
                },
            )?,
        )?;
        for name in ["json", "upper", "color", "pt", "inch", "cm", "mm", "twip"] {
            let function: mlua::Function = runtime
                .lua
                .load(format!(
                    "return function(...) return __mote_helper({name:?}, ...) end"
                ))
                .eval()?;
            globals.set(name, function)?;
        }
        let name = state
            .source_name
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "expression".into());
        let results: mlua::MultiValue = runtime
            .lua
            .load(format!("return {source}"))
            .set_name(name)
            .eval()?;
        check_deadline(runtime.deadline)?;
        values::from_lua(results.into_iter().last().unwrap_or(mlua::Nil))
    });
    result.map_err(|error| Diagnostic::new(span, error.to_string()))
}
