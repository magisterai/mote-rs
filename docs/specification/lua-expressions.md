# Lua expressions

[Specification](README.md) · [Scripting inventory](../checklists/function.md)

Lua expression evaluation is opt-in. Enable it with the CLI's `--lua` option or
`EvaluationOptions.enable_lua` in the library. Ordinary static evaluation remains
the default. With Lua enabled, directive argument values, binding values,
conditions, and iterable expressions can use Lua 5.1 expression syntax.

Arithmetic (`+`, `-`, `*`, `/`), comparisons (`==`, `~=`, `<`, `>`), logical
operators (`and`, `or`, `not`), and string concatenation (`..`) are evaluated by
Lua. Square-bracket array indexing is one-based.

A zero-based static path from the bindings chapter keeps its meaning with Lua
enabled, so that a source that compiles without Lua compiles with it. A numeric
segment that directly follows a name or a named segment, as in `rows.0.name` or
`inch(rows.0.width)`, selects by the rules of that chapter wherever a Lua
expression is accepted: in a binding value, a directive or helper argument, a
condition, an iterable, and an entry of a table constructor. No valid Lua
expression contains a name followed by a period and a digit, so that the
recognition changes no Lua expression. A segment that selects nothing is an
error.

A numeric segment that follows any other expression, as in `(rows).0` or
`rows[1].0`, is not recognized, and neither is one within a function body. The
error states that a zero-based static path is not Lua and suggests `rows[1]`. A
numeric segment in content is text with Lua enabled as without it: `#rows.0`
inserts `rows` followed by `.0`.

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
are available. Additional formatting helpers are defined in [value helpers](value-helpers.md).

A deterministic subset of the string and math libraries is available. The string
functions are `byte`, `char`, `format`, `len`, `lower`, `rep`, `reverse`, `sub`, and
`upper`, and strings accept them as methods, as in `("total"):upper()`. The
pattern functions `find`, `match`, `gmatch`, and `gsub` are not exposed, since a
pattern search cannot be interrupted by the time budget, and neither is
`string.dump`. The math library is complete except for `random` and
`randomseed`, so that a document evaluates alike on every run.

`string.format` supports the conversions `d`, `i`, `x`, `X`, `e`, `E`, `f`, `g`,
`G`, `s`, and `%%`, with the flags `-`, `0`, `+`, and space, and widths and
precisions of at most two digits. Its numbers match the C library on every
platform; `%s` widths and precisions count characters rather than bytes. Other
conversions are errors. `string.rep` refuses a result longer than 16 MiB.

```mote
#let(row=string.format("%-10s %8.1f", "Revenue", 171.456))
#paragraph[#row]
```

The standard libraries `table`, `os`, `io`, `debug`, and `coroutine`, and the
filesystem and module loaders, are not exposed. A compilation's Lua state may
allocate at most 256 MiB.

A bare content reference can resolve a Lua base global when no Mote binding
matches. An unknown Lua name evaluates to nil and produces no content. With
static evaluation, an unknown binding remains an error.

The CLI gives a compilation a shared ten-second wall-clock budget beginning at
its first Lua evaluation. `--lua-timeout` accepts durations in `ms`, `s`, `m`, or
`h`; a negative duration disables the budget and zero selects the default. The
library uses `EvaluationOptions.lua_timeout`, where `None` disables the budget.
The clock is shared across expressions, including time spent in resource helpers.
A script cannot outrun the budget by catching its error: once the budget is
spent, `pcall` and `xpcall` raise the error again when the protected call
returns. The handler of `xpcall` is called after the protected call has
returned, with the error as its argument.
A blocking custom resolver remains responsible for its own I/O deadline.

Errors identify the containing source expression. Parsing Lua-enabled source is
available separately through `syntax::parse_with_lua`; parsing alone does not
execute it. Library callers can use `compile_with_options` to select both the
parser and evaluator settings together.
