# Imported declarations checklist

[Checklist index](README.md)

## Language and attributes

- [x] One required string path; no named arguments or body.
- [x] Resolve from the importing source through the selected asset resolver.
- [x] Export local bindings, functions and element defaults; discard ordinary imported content.
- [x] Seek a path without the `.mote` extension as written and then with it.
- [x] Seek a source beside the importing source, then in each directory of the
  stated import path, then in the library that Mote carries; end the search at
  a failure other than absence.
- [x] Library content: `components/gantt` draws a Gantt chart of dated rows as
  a group of shapes, with bars, milestones, an axis of days, weeks, months,
  quarters or years, and a rule at a stated day; see the
  [component tests](../../tests/gantt.rs).
- [ ] `components/timeline` is a placeholder, whose function `timeline()`
  returns a paragraph that says so and draws nothing.
- [ ] The Gantt component draws no dependency, progress or grouping of rows,
  and leaves a long label to the text fit of its shape, which Word and
  PowerPoint do not apply as native output does.
- [x] Import each resolved logical source once and reject active import cycles.
- [x] Keep source provenance for deferred JSON/CSV requests, font/background declarations and diagnostics.
- [x] Preserve element source locations through Lua conversion, including empty children and images returned by imported functions.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/imports.md),
[implementation tests](../../tests/imports.rs), and
[frozen captures](../../tests/fixtures/imports/README.md).

This is an evaluation facility with no direct OOXML element. Receiving directives
retain their own attribute inventories and output boundaries.
