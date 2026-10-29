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
images use visible native text fallbacks; overflow omits trailing lines with a
warning. Next-page and continuous sections, equal columns, numbering restarts,
local stories and mixed physical page sizes render natively.
