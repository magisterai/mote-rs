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

The language core supports basic content, lists, inline formatting, named styles,
and font declarations, with JSON inspection of the document model. DOCX output supports paragraphs, headings, bullet and ordered lists, block quotes, inline formatting, named styles and font mappings.
Native layout work includes font measurement, styled text, paragraph pagination,
and inline decoration geometry, checked against captured output. Complete native
rendering, broader Office output, PDF/PNG output, Lua scripting, and JSON data
loading remain planned.

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
