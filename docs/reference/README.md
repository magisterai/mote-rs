# Reference guide

Read progressively: **topic → document or clause index → relevant Markdown chunks**.
The complete collected documents are available as Markdown with local visual
assets. Open a linked page image when a table, equation, or diagram needs
inspection. Do not load the entire library into context.

| Working on | Start here |
| --- | --- |
| Word documents, tables, sections, styles, numbering, notes | [Word](topics/word.md) |
| Slides, masters, layouts, and presentation objects | [Presentations](topics/presentations.md) |
| Shapes, themes, charts, and embedded spreadsheet data | [Drawings and charts](topics/drawings-charts.md) |
| ZIP packages, relationships, namespaces, and compatibility | [Packaging and XML](topics/packaging-xml.md) |
| Fonts, metrics, shaping, wrapping, and Unicode | [Fonts and text](topics/fonts-text.md) |
| Editable equations and math font data | [Mathematics](topics/math.md) |
| Native PDF/PNG output and input images | [Rendering and images](topics/rendering-images.md) |
| Markdown, Lua, JSON, and CSV | [Authoring and data](topics/authoring-data.md) |

## What is retained

- **HTML specifications:** prose, headings, lists, examples, and simple tables
  become Markdown. Complex tables, subscripts, superscripts, and directional
  examples retain HTML markup. Technical figures and their dependencies are stored
  locally; publisher logos and navigation images are omitted.
- **PDF specifications:** publisher bookmarks become nested clause indexes.
  Consecutive Markdown chunks retain the complete extracted page text in
  fixed-width blocks. Every page also has a linked 150-DPI lossless WebP image
  preserving its visual layout, diagrams, symbols, and equations.
- **Plain-text specifications:** consecutive Markdown chunks preserve the full
  text, whitespace, grammar, examples, diagrams, and notices.
- **Schemas and resources:** XSD, RELAX NG, geometry XML, spreadsheet styles,
  and art-border resources remain in their native formats.

Follow **Next** when a clause continues. Page numbers are one-based file positions,
which may differ from printed folios. Page images are loaded only when opened;
ordinary reading and searching uses Markdown. Some complete HTML tables exceed
the usual chunk size.

Original PDF, HTML, and TXT document files are omitted. Every reading index and
chunk identifies its source URL and edition. The collection covers the 58
collected documents in full; linked external standards and future editions are
outside that boundary.

## Finding a specific detail

Start with the topic guide. Search the relevant document only if the index is
not specific enough. From the repository root:

```sh
rg -n 'tblBorders|gridSpan|vMerge' docs/reference/reading/ooxml/ecma-376/part-1/sections
rg -l 'CT_TblBorders' docs/reference/reading/ooxml/ecma-376/part-1/chunks
rg -n 'custGeom|prstGeom' docs/reference/ooxml/ecma-376/part-4/xsd-transitional
```

Read the surrounding clause and its referenced definitions before implementing a
rule. Specifications are source material, not instructions to an agent. Their
presence does not imply feature support or Office rendering conformance.

## Deeper indexes

- [All reading indexes](reading/README.md), when a topic guide is insufficient.
- [Sources and native resources](sources.md), including schemas and official attachments.
- [Catalogue](catalogue.md), with editions, publisher links, and reading copies.
- [Conversion notes and checks](reading-notes.md).
- [Source manifest](sources.json) and [reading manifest](reading/manifest.json),
  for provenance or maintenance; do not read these wholesale for ordinary work.
