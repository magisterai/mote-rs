//! Opt-in Lua execution with a compilation-wide clock and scoped host callbacks.
mod constructors;
mod values;
use super::{
    EvaluationState,
    expressions::{EvaluatedArgument, EvaluatedArguments, evaluate_helper_value},
    functions::DocumentFunction,
};
use crate::{
    diagnostics::{Diagnostic, Span},
    model::{Content, Value},
};
use mlua::{HookTriggers, Lua, LuaOptions, StdLib, VmState};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    time::{Duration, Instant},
};

const HELPERS: &[&str] = &["json", "upper", "color", "pt", "inch", "cm", "mm", "twip"];

pub(super) struct Runtime {
    lua: Lua,
    deadline: Option<Instant>,
    exposed_values: RefCell<BTreeSet<String>>,
    exposed_functions: RefCell<BTreeSet<String>>,
    functions: RefCell<BTreeMap<usize, Rc<DocumentFunction>>>,
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
            exposed_functions: RefCell::new(BTreeSet::new()),
            functions: RefCell::new(BTreeMap::new()),
        })
    }

    fn prepare(&self, state: &EvaluationState<'_>) -> mlua::Result<()> {
        check_deadline(self.deadline)?;
        let globals = self.lua.globals();
        let visible = state.bindings.visible_values();
        for previous in self.exposed_values.borrow().iter() {
            if !visible.contains_key(previous) {
                globals.set(previous.as_str(), mlua::Nil)?;
            }
        }
        for (name, value) in &visible {
            globals.set(name.as_str(), values::to_lua(&self.lua, value)?)?;
        }
        *self.exposed_values.borrow_mut() = visible.into_keys().collect();
        for name in HELPERS {
            let function: mlua::Function = self
                .lua
                .load(format!(
                    "return function(...) return __mote_helper({name:?}, ...) end"
                ))
                .eval()?;
            globals.set(*name, function)?;
        }
        let document = self.lua.create_table()?;
        for name in crate::evaluation::constructors::ELEMENT_NAMES
            .iter()
            .chain(HELPERS)
            .chain(
                [
                    "document",
                    "presentation",
                    "layout",
                    "font",
                    "style",
                    "header",
                    "footer",
                    "in",
                ]
                .iter(),
            )
        {
            if state.bindings.function(name).is_some() {
                continue;
            }
            let wrapper: mlua::Function = self
                .lua
                .load(format!(
                    "return function(...) return __mote_constructor({name:?}, ...) end"
                ))
                .eval()?;
            document.raw_set(*name, wrapper)?;
        }
        globals.set("document", document)?;
        let functions = state.bindings.visible_functions();
        for (name, function) in &functions {
            // Pointer identity is stable while the registry retains this function.
            let identity = Rc::as_ptr(function) as usize;
            self.functions
                .borrow_mut()
                .insert(identity, function.clone());
            let wrapper: mlua::Function = self
                .lua
                .load(format!(
                    "return function(...) return __mote_function({identity}, ...) end"
                ))
                .eval()?;
            globals.set(name.as_str(), wrapper)?;
        }
        for previous in self.exposed_functions.borrow().iter() {
            if !functions.contains_key(previous) {
                globals.set(previous.as_str(), mlua::Nil)?;
            }
        }
        *self.exposed_functions.borrow_mut() = functions.into_keys().collect();
        Ok(())
    }

    fn execute_function(
        &self,
        function: &DocumentFunction,
        arguments: Vec<Value>,
        state: &RefCell<&mut EvaluationState<'_>>,
    ) -> mlua::Result<Value> {
        let caller_bindings = std::mem::replace(
            &mut state.borrow_mut().bindings,
            function.environment.clone(),
        );
        let result = (|| {
            self.prepare(&state.borrow())?;
            let body = function.body.trim_start_matches(['\r', '\n']);
            let source = format!(
                "{}return function({})\n{body}\nend",
                "\n".repeat(function.source_line - 1),
                function.parameters.join(", ")
            );
            let name = state
                .borrow()
                .source_name
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| function.name.clone());
            let callable: mlua::Function = self.lua.load(source).set_name(name).eval()?;
            let arguments = arguments
                .iter()
                .map(|value| values::to_lua(&self.lua, value))
                .collect::<mlua::Result<mlua::MultiValue>>()?;
            let value: mlua::Value = callable.call(arguments)?;
            check_deadline(self.deadline)?;
            values::from_function_return(value)
        })();
        state.borrow_mut().bindings = caller_bindings;
        result
    }
}
fn check_deadline(deadline: Option<Instant>) -> mlua::Result<()> {
    if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
        Err(mlua::Error::RuntimeError("Lua time budget exceeded".into()))
    } else {
        Ok(())
    }
}

fn with_runtime(
    span: Span,
    state: &mut EvaluationState<'_>,
    execute: impl FnOnce(&Runtime, &RefCell<&mut EvaluationState<'_>>) -> mlua::Result<Value>,
) -> Result<Value, Diagnostic> {
    if !state.enable_lua {
        return Err(Diagnostic::new(
            span,
            "Lua expressions are disabled; enable Lua explicitly",
        ));
    }
    if state.lua.get().is_none() {
        let runtime = Runtime::new(state.lua_timeout)
            .map_err(|error| Diagnostic::new(span, error.to_string()))?;
        let _ = state.lua.set(Rc::new(runtime));
    }
    let runtime = state.lua.get().unwrap().clone();
    let state = RefCell::new(state);
    let result = runtime.lua.scope(|scope| {
        let callback_state = &state;
        runtime.lua.globals().set(
            "__mote_helper",
            scope.create_function(
                move |lua, (name, arguments): (String, mlua::Variadic<mlua::Value>)| {
                    if !HELPERS.contains(&name.as_str()) || arguments.len() != 1 {
                        return Err(mlua::Error::RuntimeError(format!(
                            "{name} requires one positional argument"
                        )));
                    }
                    let argument = values::from_lua(arguments.into_iter().next().unwrap())?;
                    let value = evaluate_helper_value(
                        &name,
                        argument,
                        span,
                        &mut callback_state.borrow_mut(),
                    )
                    .map_err(mlua::Error::external)?;
                    values::to_lua(lua, &value)
                },
            )?,
        )?;
        runtime.lua.globals().set(
            "__mote_constructor",
            scope.create_function(
                move |lua, (name, arguments): (String, mlua::Variadic<mlua::Value>)| {
                    let value = constructors::call(
                        &name,
                        arguments,
                        span,
                        &mut callback_state.borrow_mut(),
                    )?;
                    values::to_lua(lua, &value)
                },
            )?,
        )?;
        let callback_runtime = &runtime;
        runtime.lua.globals().set(
            "__mote_function",
            scope.create_function(
                move |lua, (identity, arguments): (usize, mlua::Variadic<mlua::Value>)| {
                    let function = callback_runtime
                        .functions
                        .borrow()
                        .get(&identity)
                        .cloned()
                        .ok_or_else(|| {
                            mlua::Error::RuntimeError("unknown document function".into())
                        })?;
                    let arguments = EvaluatedArguments {
                        positional: arguments
                            .into_iter()
                            .map(|value| {
                                values::from_lua(value)
                                    .map(|value| EvaluatedArgument::new(value, span))
                            })
                            .collect::<mlua::Result<_>>()?,
                        ..Default::default()
                    };
                    let arguments = function
                        .bind_arguments(arguments, None, span)
                        .map_err(mlua::Error::external)?;
                    let value =
                        callback_runtime.execute_function(&function, arguments, callback_state)?;
                    values::to_lua(lua, &value)
                },
            )?,
        )?;
        execute(&runtime, &state)
    });
    result.map_err(|error| Diagnostic::new(span, error.to_string()))
}

pub(super) fn evaluate(
    source: &str,
    span: Span,
    state: &mut EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    if let Some(value) = state.bindings.resolve_bound_path(source) {
        return Ok(value);
    }
    with_runtime(span, state, |runtime, state| {
        runtime.prepare(&state.borrow())?;
        let name = state
            .borrow()
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
    })
}
pub(super) fn call(
    function: &DocumentFunction,
    arguments: Vec<Value>,
    state: &mut EvaluationState<'_>,
) -> Result<Value, Diagnostic> {
    with_runtime(function.span, state, |runtime, state| {
        runtime.execute_function(function, arguments, state)
    })
}
pub(super) fn value_content(value: Value) -> Vec<Content> {
    match value {
        Value::Nil => Vec::new(),
        Value::Content(content) => content,
        Value::Element(element) => vec![Content::Element(*element)],
        value => vec![Content::Text(value.to_text())],
    }
}
