# Footnotes

[Directive index](README.md) · [Standard inventory](attributes/notes.md)

- [x] One required block body, inserted through an inline reference; named arguments only.
- [x] `id`: optional string or number metadata, independent of output numbering.
- [x] `properties`: retained attribute overrides; extra metadata does not establish formatting.


- [x] Markdown references and one-line definitions, forward lookup, cached bodies, and recursion diagnostics.

## Output limits

Editable DOCX uses independent note parts and reference numbering. Nested notes
and notes in headers or footers must retain visible content with a diagnostic.
Native placement remains pending; native output currently flattens note text
inline with a diagnostic.
Numbering, custom marks, separator authoring, positioning, and restart controls are
not authoring attributes. See the standard inventory for unsupported properties.

Evidence: [language](../specification/notes.md), [tests](../../tests/notes.rs),
and [captures](../../tests/fixtures/notes/README.md). Word review is unreviewed.
