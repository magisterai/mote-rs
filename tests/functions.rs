use mote::{evaluation::EvaluationOptions, model::Document};
use std::time::{Duration, Instant};
fn compile(source: &str) -> Result<Document, mote::diagnostics::Diagnostic> {
    mote::compile_with_options(
        source,
        EvaluationOptions {
            enable_lua: true,
            ..Default::default()
        },
    )
}
#[test]
fn function_definitions_require_explicit_lua_enablement() {
    let source = "#function identity(value) return value end\n#identity('ok')";
    assert!(
        mote::compile(source)
            .unwrap_err()
            .message
            .contains("require Lua")
    );
    assert_eq!(compile(source).unwrap(), mote::compile("ok").unwrap());
}
#[test]
fn function_signatures_validate_values_and_content_separately() {
    let definition = "#function example(first, second, body) return first end\n";
    for call in [
        "#example(1,2,3)",
        "#example(1,first=2)",
        "#example(unknown=1)",
        "#example(body='x')",
        "#example(1)[a][b]",
    ] {
        assert!(compile(&format!("{definition}{call}")).is_err(), "{call}");
    }
    assert!(compile("#function example(value) return value end\n#example(1)[body]").is_err());
    assert_eq!(compile("#function example(first,second) return tostring(first)..'/'..tostring(second) end\n#example(second=2)").unwrap(), mote::compile("nil/2").unwrap());
    // Repeated named fields keep the final value, as in ordinary directive calls.
    assert_eq!(
        compile(&format!("{definition}#example(first=1,first=2)")).unwrap(),
        mote::compile("2").unwrap()
    );
}
#[test]
fn content_returns_observe_inline_constraints_and_preserved_space() {
    let identity = "#function identity(body) return body end\n";
    assert!(
        compile(&format!(
            "{identity}#paragraph[#identity[#heading(1)[Block]]]"
        ))
        .unwrap_err()
        .message
        .contains("inline content")
    );
    assert_eq!(
        compile(&format!(
            "{identity}#code(preserve_space=true)[#identity[a\r\n  b]]"
        ))
        .unwrap(),
        mote::compile("#code(preserve_space=true)[a\n  b]").unwrap()
    );
    assert_eq!(
        compile(&format!("{identity}#identity[**text**]")).unwrap(),
        mote::compile("**text**").unwrap()
    );
}
#[test]
fn definitions_are_scoped_and_references_do_not_call_functions() {
    assert!(compile("#paragraph[#function inner() return 'yes' end\n#inner()]\n#inner()").is_err());
    assert_eq!(
        compile("#function value() return 'function' end\n#let(value='binding')#value / #value()")
            .unwrap(),
        mote::compile(r#"#let(first="binding",second="function")#first / #second"#).unwrap()
    );
    assert!(compile("#function example(value=1) return value end").is_err());
    assert!(compile("#function example(value) return value").is_err());
}
#[test]
fn runtime_errors_use_definition_locations_and_shared_budget() {
    let source = "Résumé\n\n#function failure() return 1 + nil end\n#failure()";
    let error = compile(source).unwrap_err();
    assert_eq!(error.location(source).0, 3);
    assert!(error.message.contains("arithmetic"));
    let start = Instant::now();
    let error = mote::compile_with_options(
        "#function forever() while true do end end\n#forever()",
        EvaluationOptions {
            enable_lua: true,
            lua_timeout: Some(Duration::from_millis(15)),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert!(error.message.contains("time budget exceeded"));
    assert!(start.elapsed() < Duration::from_secs(2));
}
#[test]
fn nested_callbacks_and_retained_aliases_share_the_host_without_borrowing_conflicts() {
    let source = "#function first(value) return upper(value) end\n#function second(value) return first(value)..upper('!') end\n#let(saved=(function() alias=second; return true end)())\n#let(result=alias('ok'))#result";
    assert_eq!(compile(source).unwrap(), mote::compile("OK!").unwrap());
}
