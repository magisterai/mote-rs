# Endnotes

[Directive index](README.md) · [Standard inventory](attributes/notes.md)

- [x] One required block body, inserted through an inline reference; named arguments only.
- [x] `id`: optional string or number metadata, independent of output numbering.
- [x] `properties`: retained attribute overrides; extra metadata does not establish formatting.


## Output limits

Editable DOCX uses independent note parts and reference numbering. Nested notes
and notes in headers or footers must retain visible content with a diagnostic.
Native placement supports stable single-paragraph note bodies at unambiguous
inline occurrences in every single-column section. Footnotes reserve page-bottom
space. Endnotes cited in any section enter document flow after the content of
the last section, in the geometry of that section, with continuation
separators; their numbering is continuous through all sections. Native markers
use decimal footnote and lower-Roman endnote numbers. Table cells, running stories,
figure descendants, sections with more than one column, rich block note bodies,
and dynamic note fields retain visible inline fallbacks, and the diagnostic
names the context that cannot hold the note. If the last section has more than
one column, every endnote retains its inline fallback.
Oversized footnotes warn and may extend above the page; they are not split.
Endnote continuation reserves the measured visible footer lines or the declared
footer height, whichever is larger. Page-dependent footer measurement participates
in pagination stabilization.
Numbering, custom marks, separator authoring, positioning, and restart controls are
not authoring attributes. See the standard inventory for unsupported properties.

Evidence: [language](../specification/notes.md), [tests](../../tests/notes.rs),
and [captures](../../tests/fixtures/notes/README.md). Native geometry and PNG evidence: [rendering tests](../../tests/note_rendering.rs).
Placement in documents with sections: [section note tests](../../tests/section_notes.rs),
and the [section-endnotes probe](../../tests/office/probes/section-endnotes.mote),
in which every endnote lies within 0.3 point of Word.

[Combined rendering captures](../../tests/fixtures/rendering-audit/README.md)
also cover definition endnotes alongside formatted footnotes and document kerning.
Word review is unreviewed.
