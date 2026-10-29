# Functions and scripting

[Directive index](README.md)

`function` is an authoring construct without an OOXML attribute family. Returned
content follows the inventories of its receiving directives.

- [ ] Named function definition with a parenthesized parameter list and a Lua body ending in `end`.
- [ ] No parameter defaults in the declaration; omitted values become nil.
- [ ] Positional and named calls, duplicate/unknown argument validation.
- [ ] A final parameter named `body` receives evaluated content bodies.
- [ ] Definition-time lexical snapshots; function names and value names remain distinct.
- [ ] Scalar, typed length/color, object, array and content return conversions.
- [x] Lua expressions and one-based indexing are explicitly enabled; disabled by default.
- [x] Only supported pure base globals and registered Mote helpers are available.
- [ ] Existing document constructors are available through the `document` namespace.
- [x] One shared execution budget across a compilation, with located runtime errors.
- [x] Source-relative JSON uses the same resolver and snapshot from scripts.
- [ ] Returned content observes the enclosing body mode and output rules.

This inventory does not authorize unrelated standard libraries, network access,
filesystem scripting, or unimplemented document/presentation directives.

Expression support: [language](../specification/lua-expressions.md),
[runtime](../../src/evaluation/scripting.rs),
[conversion and deadline tests](../../tests/scripting.rs), and
[captured models/packages](../../tests/scripting_captures.rs).
Scalar, length/color, object and array expression values are implemented; content
return values and function definitions remain unchecked.
