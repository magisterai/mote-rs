# Mote

**Microsoft Office Typesetting Engine**

Mote is a Rust typesetting engine for creating editable Word documents and
PowerPoint presentations from Markdown, with native PDF and PNG output from the
same source. Inspired by Typst, its goal is to make programmable document
authoring straightforward for people and language models.

Start with ordinary Markdown. Add directives for typography, page layout, charts
and slides; use reusable styles, Lua and external data as the document grows.

## Capabilities

| Area | What you can author |
| --- | --- |
| Text and typography | Headings, paragraphs, lists and tasks, links, quotations, code, inline formatting, named styles with inheritance, scoped defaults, and declared font files. |
| Document layout | Page sizes and margins, sections, equal-width columns, pagination, page and column breaks, headers and footers, page numbers, footnotes and endnotes. |
| Tables and images | Merged cells, repeated table headers, borders, fills and alignment; images with sizing, cropping, fitting and transforms; figures and numbered captions. |
| Drawings and charts | Shapes, connectors and arrows; column, bar, line, area, pie and doughnut charts, including stacked variants, legends, axes and data labels. |
| Presentations | Automatic Markdown layout, positioned objects, themes, backgrounds, built-in and custom named layouts, layout inheritance, slide fields and speaker notes. |
| Programming and data | Typed bindings, conditions, loops, reusable functions, imports, opt-in Lua expressions, JSON and CSV data, and source-relative asset resolution. |
| Document structure | Bookmarks and cross-references, editable equations, comments, checkboxes, fields and basic metadata, with output-specific limits. |

DOCX and PPTX preserve editable text, tables, shapes and charts; charts include
embedded workbooks. PDF and PNG are rendered directly by Mote. JSON output
exposes evaluated content for inspection. Detailed support varies by output;
see the [rendering coverage](docs/rendering-coverage.md).

## Documents: Markdown with reusable styles

Directives use `#name(arguments)[content]`. Named arguments set properties, and
the bracketed body supplies content. `pt(...)` and `inch(...)` express lengths.
A quoted name applies a style; `extends` inherits one. `Normal` is the built-in
default paragraph style.

The excerpts below focus on content. Their linked sources include font declarations,
page setup and formatting defaults. Every preview is native PNG output from
the Rust implementation.

```mote
#style "Body" extends "Normal" (paragraph = {spacing = {after = pt(10)}})
#style "Lead" extends "Body" (run = {size = pt(17), color = "#0f766e"})

#heading(level = 1)[Quarterly brief]
#paragraph "Lead"[A strong quarter. A focused next step.]
#paragraph "Body"[
  Revenue grew by #span(bold = true)[18%], with steady progress across both teams.
]

- Launch the customer research programme.
- Expand the self-service onboarding guide.

| Team | Focus | Status |
| :--- | :--- | :--- |
| Product | Customer research | In progress |
| Operations | Self-service onboarding | Ready |
```

![Native document preview with a styled quarterly brief, a list and a table.](docs/images/readme/document-1.png)

[Complete document source](examples/readme/document.mote) ·
[Styles and fonts](docs/specification/styles-and-fonts.md)

## Data: JSON, Lua and editable charts

`let` binds a value, `json(...)` loads data relative to the source file, and
`format_money(...)` formats an amount. This example enables Lua for the addition;
loading JSON alone does not require Lua. The same data supplies the chart.

```mote
#let(revenue = json("revenue.json"))
#let(total = revenue.online + revenue.retail)

#heading(level = 1)[Half-year revenue]
#paragraph[
  Total revenue: #span(bold = true)[#format_money(total)]. Monthly values in thousands.
]
#chart(
  kind = "column", data = revenue.monthly,
  width = inch(6.7), height = inch(3.1),
  colors = {"#16877b", "#6b8ce3"}, legend = "bottom",
  data_labels = {value = true, position = "outside_end"},
  gridlines = {major = true, color = "#dfe7ed"}
)
```

![Native chart preview showing half-year revenue and monthly online and retail sales.](docs/images/readme/chart-1.png)

[Complete chart source](examples/readme/chart.mote) ·
[Sample JSON](examples/readme/revenue.json) ·
[Charts](docs/specification/charts.md) · [Lua](docs/specification/lua-expressions.md)

## Presentations: reusable layouts and editable objects

`presentation` sets the canvas and theme. A `slide` can use a named layout;
`box(slot = ...)` supplies content to one of its named regions. Here,
`two_column` provides the title, left and right regions. Boxes can also have
explicit positions, or slides can use automatic layout from plain Markdown.

```mote
#presentation(
  size = "wide", background = "#102a43",
  theme = {
    fonts = {heading = "Source Sans 3", body = "Source Sans 3"},
    colors = {text = "#ffffff"}
  }
)
#slide(
  layout = "two_column",
  notes = "Review the research and delivery plans with each team."
)[
  #box(slot = "title")[#heading(level = 1)[Quarterly priorities]]
  #box(slot = "left", fill = "#183f55",
       margin = {top = pt(24), right = pt(24), bottom = pt(24), left = pt(24)},
       text = {size = pt(24), color = "#ffffff"})[
    #paragraph[#span(size = pt(36), bold = true, color = "#5eead4")[Learn.]]
    Turn customer feedback into a clearer product roadmap.

    - Twelve customer interviews
    - Three research themes
    - One shared set of priorities
  ]
  #box(slot = "right", fill = "#e3f5ef",
       margin = {top = pt(24), right = pt(24), bottom = pt(24), left = pt(24)},
       text = {size = pt(24), color = "#102a43"})[
    #paragraph[#span(size = pt(36), bold = true)[Deliver.]]
    Make the first week simpler for every new customer.

    - Self-service onboarding
    - Two guided templates
    - A practical getting-started guide
  ]
]
```

![Native slide preview with a dark background and two columns titled Learn and Deliver.](docs/images/readme/presentation-1.png)

[Complete slide source](examples/readme/presentation.mote) ·
[Presentation layouts](docs/specification/presentation-layouts.md)

## Try it

With Rust and Cargo installed, build and run the examples from the repository root:

```sh
cargo build --release
mkdir -p target/readme
target/release/mote --format docx -o target/readme/brief.docx examples/readme/document.mote
target/release/mote --lua --format pdf -o target/readme/revenue.pdf examples/readme/chart.mote
target/release/mote --format pptx -o target/readme/slides.pptx examples/readme/presentation.mote
```

Use `--format png --dpi 144 -o PREFIX` for numbered page images, or `--format json`
to inspect evaluated content. An explicit `#presentation(...)` selects slide
layout for PDF/PNG; use `--layout presentation` for a bare Markdown deck.
[Example notes](examples/readme/README.md) explain how to regenerate the previews.

## Status and documentation

Mote is under active development, with support tracked per attribute and output
format. Native mathematical typesetting and complex-script shaping are not yet
implemented; equations currently use a source-text fallback in PDF/PNG.
Microsoft Word and PowerPoint visual review remains pending; the previews above
demonstrate native output.

- [Language specification](docs/specification/README.md): syntax and semantics, introduced progressively.
- [Rendering coverage](docs/rendering-coverage.md): behavior and limitations by output format.
- [Directive checklists](docs/checklists/README.md): supported attributes and remaining inventories.
- [Implementation overview](docs/implementation.md): architecture and validation.
- [Format references](docs/reference/README.md): retained OOXML and related specifications.
