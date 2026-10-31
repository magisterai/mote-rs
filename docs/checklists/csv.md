# CSV data checklist

[Checklist index](README.md)

## Language and attributes

- [x] One required string path; no content body.
- [x] heading: boolean or boolean string; skip a title row before the column headers.
- [x] Read rows as objects with string values, normalized names and duplicate-name suffixes.
- [x] Quoted separators/newlines, blank input, uneven rows and malformed input.
- [x] Source-relative resolution and compilation snapshots shared with other resources.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/csv.md),
[implementation tests](../../tests/csv.rs), and
[frozen captures](../../tests/fixtures/csv/README.md).

This is an evaluation facility with no direct OOXML element. Receiving directives
retain their own attribute inventories and output boundaries.
