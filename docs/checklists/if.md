# Conditional content

[Directive index](README.md)

`if` is an authoring construct without an OOXML attribute family. Its output uses
the attribute inventories of the directives contained in the selected branch.

- [x] Parenthesized condition followed by a required bracketed body.
- [x] Optional `else` bracketed body, including a newline before `else`.
- [x] Defined truthiness for each supported value kind.
- [x] Only the selected branch evaluates, in a child scope.
- [x] Preserve surrounding inline/block flow and declaration whitespace rules.
- [x] No output when false without an else; empty bodies are allowed.
- [x] Condition and body errors retain source locations.

Evidence: [language](../specification/control-flow.md),
[syntax](../../src/syntax/control.rs),
[evaluation](../../src/evaluation/control.rs),
[model/package tests](../../tests/control_flow.rs), and
[native geometry/raster tests](../../tests/control_rendering.rs).

Generated sections currently return their body into ordinary surrounding content
as well as registering the explicit section; see the language chapter's output
limit. Captured native output is separate from real-Word visual review.
