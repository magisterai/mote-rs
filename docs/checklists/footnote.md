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
inline occurrences in every single-column section. A footnote reserves space at
the foot of the page on which its reference falls, and is set in the text width
and at the left margin of the section that holds the reference. The foot of a
page that continuous sections share is that of the first section on the page.
Footnote numbering is continuous through all sections. Endnotes enter document
flow with continuation separators. Native markers use decimal footnote and
lower-Roman endnote numbers. Table cells, running stories, figure descendants,
sections with more than one column, rich block note bodies, and dynamic note
fields retain visible inline fallbacks, and the diagnostic names the context
that cannot hold the note.
Oversized footnotes warn and may extend above the page; they are not split.
Endnote continuation reserves measured visible footer content as well as any
larger declared footer height.
Numbering, custom marks, separator authoring, positioning, and restart controls are
not authoring attributes. See the standard inventory for unsupported properties.

Evidence: [language](../specification/notes.md), [tests](../../tests/notes.rs),
and [captures](../../tests/fixtures/notes/README.md). Native geometry and PNG evidence: [rendering tests](../../tests/note_rendering.rs).
Placement in documents with sections: [section note tests](../../tests/section_notes.rs).

[Combined rendering captures](../../tests/fixtures/rendering-audit/README.md)
also cover note references inside semantic formatting and definition terms,
document kerning, note-band reservation and inherited editable run properties.
Word review is unreviewed.
