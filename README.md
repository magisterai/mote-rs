# Mote

Documents are how work reaches other people. A student hands in a thesis. A job
seeker sends a résumé. A team presents a plan; a company reports to its board.
Each travels as a file, usually a .docx, a .pptx or a PDF, and each is opened,
edited, commented on, shared and kept long after it was written. As language
models take on more of that writing, more of what they produce has to arrive the
same way: as a file someone else can open in Word or PowerPoint, change, and own.

The tools have not kept pace. Asked for such a file, a model writes a program
instead. That program runs to hundreds of lines and builds the document one
paragraph, table and shape at a time. The program must then be run, in an
environment with the right libraries installed, and it fails or misbehaves at
run time in ways the model must diagnose before any document exists.

To see the result, the model needs a full office suite. Without one it has no
way to check its work. Even with a tool such as LibreOffice, the page it sees is
only that suite's version of the document: line breaks, spacing, fonts and chart
layout can all differ from what the recipient will see in Word or PowerPoint.
Every check costs time, too: the suite must start, convert the file and export
the pages. Every revision costs tokens, as the model rewrites or patches a long
program to change a single heading.

"This will begin to make things right."

Mote is a document engine built for the way models already write. Any Markdown
file is already a Mote document, and it produces a finished file as it stands. A
separate set of defaults can then give it a typeface, a page size, spacing and a
table style without touching the original text. Where one passage needs more,
such as a styled introduction, a chart built from data or a positioned slide, a
short directive is added at that point and the rest of the document is left as
it was. A few dozen lines take the place of a program, and changing a heading
means changing one line. Mote is a single executable with nothing else to
install, and an invalid source produces no output, only an error that names its
file, line and column.

From that one source, Mote produces an editable Word document, an editable
PowerPoint presentation, and PDF and PNG pages. The Office files contain real
structures rather than pictures of them: named styles, numbered lists, slide
masters, speaker notes, and charts with their workbooks. The recipient can carry
on the work in Word or PowerPoint. Mote lays out and draws the PDF and PNG pages
itself, without starting a separate application, so a model can render what it
has just written, look at it, and correct the source in a single loop.

Mote is Word, PowerPoint and Acrobat for language models.

## Report

A research note for Word, written in
[135 lines of source](readme/examples/report.mote) that read their figures from a
[JSON file](readme/examples/transit.json). The note has a title block, an
abstract, a full-width figure, a two-column body, a numbered table and a second
chart, with running headers and footers. Both charts are native Word charts
with their data in an embedded workbook.

![Native preview of a one-page research note titled "A temperate sub-Neptune transiting the nearby M dwarf MTS-0412", with a transit light curve, a two-column body, a candidates table and a radius histogram.](readme/images/report-1.png)

[Source](readme/examples/report.mote) ·
[Word download](readme/downloads/report.docx) ·
[PDF download](readme/downloads/report.pdf)

## Deck

A briefing on the same discovery for PowerPoint, in
[125 lines](readme/examples/deck.mote). The deck has a theme, a starfield drawn
by a loop, positioned text, editable shapes and connectors, a line chart and a
doughnut chart, and speaker notes on every slide. The deck is written for its
format, not converted from the note.

<p>
<img src="readme/images/deck-1.png" width="49%" alt="Title slide: a star with a transiting planet beside the title A temperate sub-Neptune around a nearby red dwarf.">
<img src="readme/images/deck-2.png" width="49%" alt="Slide with three cards: 2.2 times the radius of the Earth, 11.8 days to complete one orbit, 1.3 times the sunlight that reaches the Earth.">
<img src="readme/images/deck-3.png" width="49%" alt="Slide pairing a short explanation with the transit light curve.">
<img src="readme/images/deck-4.png" width="49%" alt="Slide with a doughnut chart of 48 candidates and a three-step plan.">
</p>

[Source](readme/examples/deck.mote) ·
[PowerPoint download](readme/downloads/deck.pptx) ·
[PDF download](readme/downloads/deck.pdf)

The sections that follow introduce the language step by step, from plain
Markdown to these finished pieces.

## Markdown

Headings, emphasis, lists and tables require no Mote directives:

```markdown
# Quarterly brief

Revenue grew by **18%**, with steady progress across both teams.

- Launch the customer research programme.
- Expand the self-service onboarding guide.

| Team | Focus | Status |
| :--- | :--- | :--- |
| Product | Customer research | In progress |
| Operations | Self-service onboarding | Ready |
```

[Markdown source](readme/examples/plain.md) ·
[Word download](readme/downloads/plain.docx) ·
[PowerPoint download](readme/downloads/plain.pptx) ·
[Document preview](readme/images/plain-1.png) ·
[Slide preview](readme/images/plain-slides-1.png)

The same source produces both editable formats. The PowerPoint output uses
automatic slide layout; the native PDF and PNG output may use either document
or presentation layout.

## Defaults

The Markdown above stays intact; a separate defaults file precedes it. A style
sets the base typography, and `#set` supplies properties to the elements that
follow, including those created by Markdown. The helpers `pt(...)` and
`inch(...)` express lengths. `Normal` is the default paragraph style, `Heading1`
styles level-one headings, and `extends` inherits from another style.

```mote
#style "Normal" (run = {font = "Source Sans 3", size = pt(12), color = "#334155"})
#style "Heading1" extends "Normal" (
  run = {size = pt(28), bold = true, color = "#102a43"},
  paragraph = {spacing = {before = pt(0), after = pt(12)}}
)
#set paragraph(spacing = {after = pt(10)})
#set table(
  width = inch(6.7),
  cell = {margin = {vertical = pt(7), horizontal = pt(10)}, fill = "#f0f7f7"},
  borders = {all = {style = "none"}, inside_h = {width = pt(0.5), color = "#c4dcdb"}}
)
```

The excerpt sets the typography, paragraph spacing and table appearance. The
full [defaults file](readme/examples/defaults.mote) also declares the font files
and a compact page size. The defaults are applied by concatenating them with the
original Markdown at render time; the original file is left unchanged, and no
special command-line option is required.

![Native document preview of the unchanged Markdown with typography and table defaults.](readme/images/defaults-1.png)

[Defaults source](readme/examples/defaults.mote) ·
[Unchanged Markdown](readme/examples/plain.md) ·
[Word download](readme/downloads/defaults.docx)

## Directives

A directive takes the form `#name(arguments)[content]`. The named arguments set
properties, and the bracketed body supplies content. A quoted name applies a
named style. In the example below only the introduction needs further
formatting; the heading, list and table remain Markdown. The source imports the
defaults from the previous example.

```mote
#style "Lead" extends "Normal" (run = {size = pt(17), color = "#0f766e"})

# Quarterly brief

#paragraph "Lead"[A strong quarter. A focused next step.]
#paragraph[
  Revenue grew by #span(bold = true, color = "#0f766e")[18%],
  with steady progress across both teams.
]

- Launch the customer research programme.
- Expand the self-service onboarding guide.

| Team | Focus | Status |
| :--- | :--- | :--- |
| Product | Customer research | In progress |
| Operations | Self-service onboarding | Ready |
```

![Native document preview with a teal introduction and emphasized revenue figure.](readme/images/directives-1.png)

[Complete source](readme/examples/directives.mote) ·
[Word download](readme/downloads/directives.docx)

A span repeated with the same attributes is better served by a character style,
declared with `type = "character"` and applied by name. The research note
above sets its running header this way:

```mote
#style "Running" (type = "character", run = {font = "Source Sans 3", size = pt(8), color = "#6b7891"})
#style "Brand" extends "Running" (type = "character", run = {bold = true, color = "#14213d"})

#header[#paragraph(tabs = {right = inch(7)})[#span "Brand"[MERIDIAN TRANSIT SURVEY]#tab()#span "Running"[Research Note 7 · November 2026]]]
```

The Word package receives both as native character styles, so a later edit to
the style in Word changes every span that uses it. The right-aligned tab stop
places the note number against the right margin.

## Slides

The `presentation` directive sets the canvas and the theme. A slide contains
either loose content, which Mote arranges in automatic frames, or positioned
objects: a `box` holds text, a `shape` draws an editable shape that may hold text
of its own, and a `line` draws a connector. Each object takes a `position` on the
slide, and the `notes` argument supplies the speaker notes. The second slide of
the deck above reads as follows, with `flush` bound earlier to a table of zero
margins and a `#set shape(...)` default centring the text in each shape:

```mote
#presentation(
  size = "wide", background = "#0b1226",
  theme = {
    fonts = {heading = "Source Serif 4", body = "Source Sans 3"},
    colors = {text = "#e8ecf6", accent1 = "#f08a3c", accent2 = "#7f9cff"}
  }
)

#slide(notes = "Three numbers. The third is why the planet is called temperate.")[
  #box(position = {x = inch(0.9), y = inch(0.7), width = inch(11.5), height = inch(1.1)}, margin = flush,
       text = {font = "Source Serif 4", size = pt(36), bold = true, color = "#ffffff"})[
    MTS-0412 b in three numbers
  ]
  #shape(kind = "round_rect", fill = "#16204a",
         position = {x = inch(0.9), y = inch(2.2), width = inch(3.65), height = inch(4.2)})[
    #paragraph[#span(font = "Source Serif 4", size = pt(80), bold = true, color = "#f08a3c")[2.2]]
    #paragraph[#span(size = pt(19), color = "#d5dcef")[times the radius of the Earth]]
  ]
  #shape(kind = "round_rect", fill = "#16204a",
         position = {x = inch(4.84), y = inch(2.2), width = inch(3.65), height = inch(4.2)})[
    #paragraph[#span(font = "Source Serif 4", size = pt(80), bold = true, color = "#7f9cff")[11.8]]
    #paragraph[#span(size = pt(19), color = "#d5dcef")[days to complete one orbit]]
  ]
  #shape(kind = "round_rect", fill = "#f08a3c",
         position = {x = inch(8.78), y = inch(2.2), width = inch(3.65), height = inch(4.2)})[
    #paragraph[#span(font = "Source Serif 4", size = pt(80), bold = true, color = "#0b1226")[1.3×]]
    #paragraph[#span(size = pt(19), color = "#0b1226")[the sunlight that reaches the Earth]]
  ]
]

```

![Native slide preview with three cards: 2.2 times the radius of the Earth, 11.8 days to complete one orbit, and 1.3 times the sunlight that reaches the Earth.](readme/images/deck-2.png)

A slide may also use a named layout, such as the built-in `two_column`, in which
each `box(slot = ...)` fills one named region; a custom layout may inherit from
an existing one.

[Complete deck source](readme/examples/deck.mote) ·
[PowerPoint download](readme/downloads/deck.pptx)

## Charts

The `json(...)` helper loads data relative to the source file, and `let` gives
the data a name. Static evaluation suffices; no scripting is required. The file
[revenue.json](readme/examples/revenue.json) contains monthly values in
thousands:

```json
{
  "categories": ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
  "series": [
    {"name": "Online", "values": [24, 30, 32, 36, 44, 50]},
    {"name": "Retail", "values": [18, 20, 22, 24, 28, 32]}
  ]
}
```

The chart receives the data together with its size, colors and labels:

```mote
#let(revenue = json("revenue.json"))

# Half-year revenue

Monthly revenue, in thousands.

#chart(
  kind = "column", data = revenue,
  width = inch(6.7), height = inch(3.1),
  colors = {"#16877b", "#6b8ce3"}, legend = "bottom",
  data_labels = {value = true, position = "outside_end"},
  gridlines = {major = true, color = "#dfe7ed"}
)
```

![Native document chart preview showing monthly online and retail revenue.](readme/images/chart-1.png)

[Complete chart source](readme/examples/chart.mote) ·
[Word download](readme/downloads/chart.docx)

The Word download contains an editable chart and its embedded workbook. The
supported kinds are column, bar, line, area, pie and doughnut charts, with
stacked variants, axes, legends and data labels.

## Lua

Lua is optional and must be enabled with `--lua`. A `#function ... end`
definition contains Lua statements and returns a value or document content. In
the example, a function sums the monthly values of each channel and converts
thousands to dollars. The Mote `#for` loop repeats a paragraph for each channel,
and `format_money(...)` formats the result.

```mote
#let(revenue = json("revenue.json"))
#function channel_total(values)
  local total = 0
  for index, amount in ipairs(values) do
    total = total + amount
  end
  return total * 1000
end

# Revenue summary

Six months of revenue, calculated from the monthly data.

#for channel in revenue.series {
  #paragraph[#strong[#channel.name:] #format_money(channel_total(channel.values))]
}
```

![Native document preview showing calculated online revenue of 216000 dollars and retail revenue of 144000 dollars.](readme/images/lua-1.png)

[Complete source](readme/examples/lua.mote) ·
[Word download](readme/downloads/lua.docx)

## CLI

Build the executable with Rust and Cargo from the repository root:

```sh
cargo build --release
```

The executable is `target/release/mote`; `cargo install --path .` places it on
the `PATH`. A single invocation compiles one source:

```text
mote [--format json|docx|pptx|pdf|png] [-o OUTPUT] [OPTIONS] [SOURCE|-]
```

| Option | Effect |
| --- | --- |
| `--format FORMAT` | `json` (the default), `docx`, `pptx`, `pdf` or `png`. |
| `-o`, `--output PATH` | The output file. Required for every format except JSON, which is written to standard output when no path is given. |
| `--dpi N` | The PNG resolution, from 1 to 9600; the default is 72. PNG only. |
| `--layout document\|presentation` | The native layout for PDF and PNG. Without it, a source that declares `#presentation(...)` is laid out as slides and any other source as pages. |
| `--lua` | Enables Lua expressions and `#function` definitions, which are otherwise rejected. |
| `--lua-timeout DURATION` | The time budget shared by all Lua in one run, such as `500ms`, `10s`, `2m` or `1h`. The default is `10s`; a negative value removes the limit. |
| `--` | Treats every later argument as a file name. |
| `-h`, `--help` | Prints the usage summary. |

Mote reads the source from the named file, or from standard input when the name
is `-` or absent. Imports, JSON and CSV data, images and font files resolve
relative to the source file. PNG output writes one image per page: `-o
pages/note` produces `pages/note-1.png`, `pages/note-2.png` and so on, and a
directory path produces `page-1.png` within it.

```sh
mote report.mote                                          # evaluated model as JSON
mote --format docx -o report.docx report.mote             # editable Word document
mote --format pptx -o deck.pptx deck.mote                 # editable PowerPoint deck
mote --format pdf -o report.pdf report.mote               # native PDF
mote --format png --dpi 144 -o pages/deck deck.mote       # pages/deck-1.png, ...
mote --format png --layout presentation -o slides/ notes.md
cat defaults.mote notes.md | mote --format docx -o notes.docx -
mote --lua --lua-timeout 2s --format docx -o summary.docx lua.mote
```

A warning is written to standard error as `file:line:column: warning: message`
and does not stop the output. An error is written as `file:line:column: message`;
Mote then exits with status 1 and writes no output file. The
[example guide](readme/examples/README.md) explains how the downloads and
previews above are regenerated.

## Review

The command `cargo review` renders the whole rendering catalogue without opening
an Office application. Each case yields its source, its editable package, a
native PDF and a PNG for each page; `cargo review --case NAME` refreshes a
single case. The [rendering case guide](tests/cases/README.md) describes the
catalogue and its diagnostics.

The same facilities serve an agent reviewing its own work. Each warning and
error names its file, line and column, and an error leaves no partial output. The PNG pages
show what the agent has produced, and the JSON output shows how the source was
understood.

A package that renders natively may still be rejected in part by Office: Word
omits a chart whose markup violates its schema without reporting an error. The
command `cargo xtask validate PACKAGE...` therefore checks every
XML part of the editable output, including embedded workbooks, against the
ECMA-376 schemas retained in the repository. All 95 catalogue cases validate.

## Status

Mote is under active development, and its support is tracked for each attribute
and output format. The implementation comprises some 41,000 lines of Rust and
nearly 500 tests; the language is described in 36 specification chapters, and
the support for each of 64 directives is inventoried against the OOXML
standards. Native mathematical typesetting and complex-script shaping
are not yet implemented; equations fall back to their source text in PDF and
PNG. The previews above are native Mote output. A visual review in Microsoft
Word and PowerPoint remains outstanding.

- [Language specification](docs/specification/README.md): the syntax and semantics, introduced progressively.
- [Rendering coverage](docs/rendering-coverage.md): the behaviour and limitations of each output format.
- [Directive checklists](docs/checklists/README.md): the supported attributes and the remaining inventories.
- [Implementation overview](docs/implementation.md): the architecture and its validation.
- [Format references](docs/reference/README.md): the retained OOXML and related specifications.
