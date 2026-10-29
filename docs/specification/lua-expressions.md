# Lua expressions

[Specification](README.md) · [Scripting inventory](../checklists/function.md)

Lua expression evaluation is opt-in. Enable it with the CLI's `--lua` option or
`EvaluationOptions.enable_lua` in the library. Ordinary static evaluation remains
the default. With Lua enabled, directive argument values, binding values,
conditions, and iterable expressions can use Lua 5.1 expression syntax.

Arithmetic (`+`, `-`, `*`, `/`), comparisons (`==`, `~=`, `<`, `>`), logical
operators (`and`, `or`, `not`), and string concatenation (`..`) are evaluated by
Lua. Square-bracket array indexing is one-based. Dotted Mote paths that resolve
directly still use the zero-based path rules from the bindings chapter.

This example requires Lua:

```mote
#let(items={{name="Analysis",amount=12},{name="Review",amount=8}}, total=items[1].amount+items[2].amount)
#paragraph[Total: #total]
#if(total > 10)[#paragraph[Review required.]]
```

Lua's own logical expressions treat only false and nil as false. After an
expression returns, Mote's `if` uses the truthiness rules from the previous
chapter, including zero and empty collections being false. Keep that distinction
in mind when combining `and` or `or` with a Mote condition.

Bindings are copied into the Lua environment before evaluation. Values declared
in a directive body disappear from that environment when their lexical scope
ends. Lua table indices must be contiguous positive integers starting at one to
become a Mote array; sparse or mixed tables become objects with string keys. An
empty Lua table becomes an empty object. Nil entries are absent from Lua tables.
Finite numbers, strings, and booleans retain their ordinary value kinds. Cyclic
or excessively nested tables cannot become document values.

The existing `json`, `upper`, `color`, `pt`, `inch`, `cm`, `mm`, and `twip` helpers
are available in Lua. Length and color values retain their types through tagged
Lua tables. JSON uses the same source-relative resolver and resource snapshot.
Basic Lua functions such as `type`, `tostring`, `tonumber`, `pairs`, and `ipairs`
are available. Additional standard libraries and filesystem/module loaders are
not exposed. `upper` retains its currently documented ASCII restriction.

A bare content reference can resolve a Lua base global when no Mote binding
matches. An unknown Lua name evaluates to nil and produces no content. With
static evaluation, an unknown binding remains an error.

The CLI gives a compilation a shared ten-second wall-clock budget beginning at
its first Lua evaluation. `--lua-timeout` accepts durations in `ms`, `s`, `m`, or
`h`; a negative duration disables the budget and zero selects the default. The
library uses `EvaluationOptions.lua_timeout`, where `None` disables the budget.
The clock is shared across expressions, including time spent in resource helpers.
A blocking custom resolver remains responsible for its own I/O deadline.

Errors identify the containing source expression. Parsing Lua-enabled source is
available separately through `syntax::parse_with_lua`; parsing alone does not
execute it. Library callers can use `compile_with_options` to select both the
parser and evaluator settings together.
