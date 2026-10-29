# Bindings

[Directive index](README.md)

`let` is an authoring construct without an OOXML attribute family. Its values
feed the already inventoried attributes of the receiving directives.

- [x] Named bindings evaluated in source order; later bindings can use earlier ones.
- [x] No body; empty binding lists allowed; positional arguments rejected.
- [x] Scalar, length, color, object, and array values retain their types.
- [x] Rebinding replaces the current local value; bodies have child scopes.
- [x] Dotted object paths and zero-based array paths in expressions.
- [x] Value insertion in content and sentence punctuation after paths.
- [x] Unknown names, missing properties, invalid indices and scalar traversal fail with source locations.
- [x] Bindings remain evaluation state, separate from the emitted document model.

Evidence: [language](../specification/bindings.md),
[evaluation](../../src/evaluation/bindings.rs),
[tests and captured models/packages](../../tests/bindings.rs).
Bindings feed the existing output paths; these checks do not establish Office
visual review.
