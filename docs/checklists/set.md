# Scoped defaults checklist

[Checklist index](README.md)

## Language and attributes

- [x] A target element name and named properties; no content body.
- [x] Accept supported element targets and normalize their optional properties; required arguments remain construction-time checks.
- [x] Progressive lexical defaults, recursive property merging and explicit overrides.
- [x] Apply defaults to directive, Markdown and Lua constructor content.
- [x] Bodies, branches, loops and functions preserve lexical scope.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/scoped-defaults.md),
[implementation tests](../../tests/scoped_defaults.rs), and
[frozen captures](../../tests/fixtures/scoped-defaults/README.md).

This is an evaluation facility with no direct OOXML element. Receiving directives
retain their own attribute inventories and output boundaries.
