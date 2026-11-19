# Sections

[Directive index](README.md) · [Standard inventory](attributes/sections-and-stories.md)

- [x] One required block body, named arguments only, implicit content before and after explicit sections.
- [x] `page`, `margin`, `columns`: inherit document geometry and override named fields.
- [x] `properties.start`: next_page, continuous, even_page, odd_page, next_column.
- [x] `properties.vertical_alignment`: top, center, bottom, justified (both normalizes to justified).
- [x] `page_numbering.restart`, `start`, `format`: boolean, positive integer and retained number-format string.
- [x] Section-local header/footer declarations, inherited defaults and explicit empty-story resets.
- [x] Retain unknown nested properties; reject legacy flat section and page-numbering aliases.

Language and model behavior is covered by [section tests](../../tests/sections.rs)
and the [specification](../specification/sections-and-stories.md).
[Output captures](../../tests/fixtures/sections-stories/README.md) cover complete
DOCX packages and native geometry/raster regression tests. Microsoft Word review
remains unreviewed.

Native support notes: current/total page fields are decimal. Number-format strings,
section vertical alignment, and odd/even or next-column section starts retain
editable DOCX mappings but warn about native limits. Header/footer tables and
images use visible native text fallbacks. A header or footer taller than its
margin pushes the body, as Word does: the body begins at the foot of the header
and ends at the top of the footer, as the
[headers-and-footers probe](../../tests/office/probes/headers-and-footers.mote)
records and the [story tests](../../tests/tall_stories.rs) verify. A story whose
lines on a later page outgrow its height on the first page of its section omits
its trailing lines with a warning. Next-page and continuous sections, equal columns, numbering restarts,
local stories and mixed physical page sizes render natively. A footnote in a
single-column section is placed at the foot of its page in the geometry of that
section, and endnotes follow the last section; a note in a section with more
than one column retains an inline fallback with a warning, as the
[section note tests](../../tests/section_notes.rs) verify.
