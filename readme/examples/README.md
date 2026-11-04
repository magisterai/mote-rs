# README examples

The [main README](../../README.md) presents these examples. The editable and PDF
downloads are in [readme/downloads](../downloads/README.md). The
reef survey, the business content and all figures are illustrative.

| Example | Source | Composition |
| --- | --- | --- |
| Field report | [report.mote](report.mote) + [survey.json](survey.json) | A one-page Word report with a title block, a chart from JSON, a styled table, a callout table, a footnote, and running headers and footers. |
| Board deck | [deck.mote](deck.mote) + [survey.json](survey.json) | A four-slide PowerPoint deck with a theme, positioned text, shapes, connectors, a line chart and speaker notes. |
| Pure Markdown | [plain.md](plain.md) | The exact file is passed to both output formats, without any added declarations. |
| Defaults | [defaults.mote](defaults.mote) + [plain.md](plain.md) | The defaults are prepended in memory; the original Markdown remains unchanged. |
| Directives | [directives.mote](directives.mote) | The source imports the defaults and adds a styled introduction and an inline accent. |
| JSON chart | [chart.mote](chart.mote) + [revenue.json](revenue.json) | Static JSON loading creates an editable chart without scripting. |
| Lua calculation | [lua.mote](lua.mote) + [revenue.json](revenue.json) | A function sums monthly values and converts thousands to dollars. The example alone requires `--lua`. |

The report and the deck are written for their own formats rather than converted
from one another. The pure Markdown example alone is rendered in both formats,
since its purpose is to show one unchanged source serving both. The styled
tutorial examples use compact pages for the README.

## Keep Markdown unchanged

From the repository root, after building Mote:

```sh
mkdir -p target/readme
cd readme/examples
cat defaults.mote plain.md | ../../target/release/mote --format docx -o ../../target/readme/defaults.docx -
```

The compiler reads the combined stream. Neither input file is edited, and there
is no dedicated command-line option for defaults. Running in the source directory
keeps imports and resource paths relative to these examples.

## Regenerate the downloads and previews

From the repository root, with Rust, Cargo and `xmllint` installed:

```sh
cargo xtask readme
```

The generator calls the Mote library directly for every Office package, PDF and
native PNG preview. It checks that each package part is well-formed XML,
validates every part against the ECMA-376 schemas, checks the number of slides
and preview pages, and publishes the set only after generation succeeds. Every
native image is inspected before an updated set is committed.

Markdown heading markers and repeated table headers in the pure Markdown deck
report that they have no presentation effect. The generator accepts those
diagnostics, together with the fallback fonts of the pure Markdown example, and
rejects any other warning.

## Fonts

The full sources include setup that the README excerpts omit.
[fonts.mote](fonts.mote) declares Source Sans 3 from the existing
[font fixtures](../../tests/fixtures/measurements/fonts/), under their
[licence](../../tests/fixtures/measurements/fonts/LICENSE.md), and Source Serif 4
from [fonts](fonts/), under the SIL Open Font License in
[fonts/LICENSE.md](fonts/LICENSE.md). The `Aptos` alias supplies the native
chart's default font outlines. The native previews use these declared files; an
Office application needs both typefaces installed to show the same design. The
pure Markdown example retains the engine's default fonts.

The images in [readme/images](../images/) are native Mote
renders, not Office captures. A visual review in Word and PowerPoint remains
outstanding. Temporary artifacts belong under `target/readme/`.
