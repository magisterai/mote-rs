# Iteration

[Directive index](README.md)

`for` is an authoring construct without an OOXML attribute family. Its output uses
the attribute inventories of the directives contained in its body.

- [x] `for name in expression` with a required braced markup body.
- [x] Arrays iterate in order; objects iterate values in sorted key order.
- [x] Fresh child scope for each iteration; the loop binding is local.
- [x] Normalize each iteration's block body while preserving inline iteration.
- [x] Empty collections produce no content; non-collections fail.
- [x] Nested loops and branches can use outer values without leaking local names.
- [x] Invalid forms and evaluation failures retain source locations.

Evidence: [language](../specification/control-flow.md),
[syntax](../../src/syntax/control.rs),
[evaluation](../../src/evaluation/control.rs),
[model/package tests](../../tests/control_flow.rs), and
[native geometry/raster tests](../../tests/control_rendering.rs).

Generated sections currently return their body into ordinary surrounding content
as well as registering the explicit section; see the language chapter's output
limit. Captured native output is separate from real-Word visual review.
