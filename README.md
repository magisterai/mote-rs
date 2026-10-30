# Mote

**Microsoft Office Typesetting Engine**

The goal is to build a Rust typesetting engine that turns Markdown into polished,
editable Word documents and PowerPoint presentations, with native PDF and PNG
output from the same source.

Mote is inspired by Typst's approach to programmable typesetting.

- Start with plain Markdown; add directives for richer structure and layout.
- Make source straightforward for people and language models to write.
- Support reusable styles and templates, Lua scripting, and JSON data sources.
- Preserve editable text, tables, charts, and shapes in Office output.
- Render PDF and PNG directly, with layout fidelity checked against Microsoft
  Word and PowerPoint.

The current implementation produces JSON, editable DOCX, and native PDF/PNG for
the documented document subset, including text, tables, images, sections, headers
and footers, references, notes, shapes, and charts. PPTX and native slide
output support explicit slides with positioned text, images, tables, shapes,
connectors and charts, plus background images, footer fields, dates and slide
numbers. PPTX also retains speaker notes.
Typed bindings, JSON data, conditions, iteration, opt-in Lua, and reusable
functions generate content through the same directives. Output is checked against
captured packages, geometry, and rasters; new Office cases remain unreviewed.

Directives use `#name(arguments)[content]` for structure and formatting:

```mote
#heading(level = 1)[Quarterly report]

#paragraph[
  Revenue increased by #span(bold = true, color = "#28734f")[12%].
]

#list[
  #item[Expanded into two new markets.]
  #item[Improved customer retention.]
]
```

See the [language specification](docs/specification/README.md),
[implementation overview](docs/implementation.md), and
[directive checklists](docs/checklists/README.md) for definitions, current coverage,
and progress.
