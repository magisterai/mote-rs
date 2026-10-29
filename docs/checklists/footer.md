# Footers

[Directive index](README.md) · [Standard inventory](attributes/sections-and-stories.md)

- [x] One required block body; no positional arguments.
- [x] `height`, `distance`: nonnegative lengths; direct arguments override the corresponding `properties` entries.
- [x] `properties`: retained property object; unknown fields do not imply a rendering effect.
- [x] Top-level declarations apply to subsequent implicit sections; section-local declarations replace inherited content.
- [x] An empty body clears the story.

Language and model behavior is covered by [section tests](../../tests/sections.rs)
and the [specification](../specification/sections-and-stories.md).
[Output captures](../../tests/fixtures/sections-stories/README.md) cover complete
DOCX packages and native geometry/raster regression tests. Microsoft Word review
remains unreviewed.

First-page and separate odd/even stories are inventoried standard capabilities;
they have no Mote authoring mapping in the current compatibility scope. Native
complex-story fallbacks and overflow must be reported explicitly.

Native support notes: current/total page fields are decimal. Number-format strings,
section vertical alignment, and odd/even or next-column section starts retain
editable DOCX mappings but warn about native limits. Header/footer tables and
images use visible native text fallbacks; overflow omits trailing lines with a
warning. Next-page and continuous sections, equal columns, numbering restarts,
local stories and mixed physical page sizes render natively.
