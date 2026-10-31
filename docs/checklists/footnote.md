# Footnotes

[Directive index](README.md) · [Standard inventory](attributes/notes.md)

- [x] One required block body, inserted through an inline reference; named arguments only.
- [x] `id`: optional string or number metadata, independent of output numbering.
- [x] `properties`: retained attribute overrides; extra metadata does not establish formatting.


- [x] Markdown references and one-line definitions, forward lookup, cached bodies, and recursion diagnostics.

## Output limits

Editable DOCX uses independent note parts and reference numbering. Nested notes
and notes in headers or footers must retain visible content with a diagnostic.
Native placement supports stable single-paragraph note bodies at unambiguous
inline occurrences in one single-column section. Footnotes reserve page-bottom
space; endnotes enter document flow with continuation separators. Native markers
use decimal footnote and lower-Roman endnote numbers. Table cells, running stories,
figure descendants, multiple sections, multiple columns, rich block note bodies,
and dynamic note fields retain visible inline fallbacks with diagnostics.
Oversized footnotes warn and may extend above the page; they are not split.
Endnote continuation can overlap footer text when no explicit footer height is reserved.
Numbering, custom marks, separator authoring, positioning, and restart controls are
not authoring attributes. See the standard inventory for unsupported properties.

Evidence: [language](../specification/notes.md), [tests](../../tests/notes.rs),
and [captures](../../tests/fixtures/notes/README.md). Native geometry and PNG evidence: [rendering tests](../../tests/note_rendering.rs).

[Combined rendering captures](../../tests/fixtures/rendering-audit/README.md)
also cover note references inside semantic formatting and definition terms,
document kerning, note-band reservation and inherited editable run properties.
Word review is unreviewed.
