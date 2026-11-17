# Functions and scripting

[Directive index](README.md)

`function` is an authoring construct without an OOXML attribute family. Returned
content follows the inventories of its receiving directives.

- [x] Named function definition with a parenthesized parameter list and a Lua body ending in `end`.
- [x] No parameter defaults in the declaration; omitted values become nil.
- [x] Positional and named calls, duplicate/unknown argument validation.
- [x] A final parameter named `body` receives evaluated content bodies.
- [x] Definition-time lexical snapshots; function names and value names remain distinct.
- [x] Scalar, typed length/color, object, array and content return conversions.
- [x] An array result is data where it is used as a value: in a binding, an
  argument, a path, a length, an iterable and chart data. An array result is a
  sequence of content where the function is called in content.
- [x] Lua expressions and one-based indexing are explicitly enabled; disabled by default.
- [x] A zero-based static numeric segment directly after a name keeps its
  meaning within a Lua expression. After any other expression, and within a
  function body, the error names the static path and suggests bracket indexing.
- [x] Only supported pure base globals, registered Mote helpers and the
  deterministic subset of the string and math libraries are available: no
  pattern functions, `string.dump` or `math.random`; `string.format` and
  `string.rep` are reimplemented with bounded widths and results.
- [x] A memory limit of 256 MiB per compilation, and globals restored when a
  binding that shadowed them leaves scope.
- [x] Existing document constructors are available through the `document` namespace.
  Inventory: all current text/list, table, media, note, field, anchor, caption and
  figure directives; document/font/style/header/footer declarations; registered
  value helpers. Section retains raw syntax and is excluded. Property maps precede
  positional values and body content; ordinary attribute normalization still applies.
- [x] One shared execution budget across a compilation, with located runtime errors.
- [x] Source-relative JSON uses the same resolver and snapshot from scripts.
- [x] Returned content observes the enclosing body mode and output rules.

This inventory does not authorize unrelated standard libraries, network access,
filesystem scripting, or unimplemented document/presentation directives.

Expression support: [language](../specification/lua-expressions.md),
[runtime](../../src/evaluation/scripting.rs),
[conversion and deadline tests](../../tests/scripting.rs), and
[captured models/packages](../../tests/scripting_captures.rs).
Function definitions and content returns: [language](../specification/functions.md),
[implementation](../../src/evaluation/functions.rs), [tests](../../tests/functions.rs),
[data results and static paths](../../tests/lua_data.rs),
and [captured models/packages](../../tests/function_captures.rs). Empty tagged content remains empty; an expression outside a function body
keeps its bindings across the functions it calls, and a call made by a function body retains the
explicit limits in the language chapter. Constructors: [language](../specification/document-constructors.md),
[shared validation](../../src/evaluation/constructors.rs),
[tests](../../tests/constructors.rs), and
[output captures](../../tests/constructor_captures.rs).
