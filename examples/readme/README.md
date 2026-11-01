# README examples

These are the complete sources for the examples in the repository
[README](../../README.md). The README excerpts omit setup declarations; all
visible content appears in both the excerpts and the sources.

| Source | Demonstrates | Editable output |
| --- | --- | --- |
| [document.mote](document.mote) | Named styles, inheritance, inline formatting, Markdown lists and tables | DOCX |
| [chart.mote](chart.mote) | JSON data, Lua arithmetic, money formatting and a column chart | DOCX |
| [presentation.mote](presentation.mote) | A named slide layout, theme, styled boxes and speaker notes | PPTX |

The chart requires `--lua` for its arithmetic expression. Its illustrative
[revenue data](revenue.json) contains monthly values in thousands and channel
totals in dollars.

[fonts.mote](fonts.mote) declares Source Sans 3 using the existing
[font fixtures](../../tests/fixtures/measurements/fonts/) and their
[license](../../tests/fixtures/measurements/fonts/LICENSE.md). The `Aptos` alias
also supplies declared outlines for the native chart's default font. The samples
therefore render without depending on installed fonts. The document pages are
deliberately compact for README previews; their source declares the dimensions.

## Regenerate the previews

From the repository root:

```sh
cargo build --release
mkdir -p docs/images/readme
target/release/mote --format png --dpi 144 -o docs/images/readme/document examples/readme/document.mote
target/release/mote --lua --format png --dpi 144 -o docs/images/readme/chart examples/readme/chart.mote
target/release/mote --format png --dpi 144 -o docs/images/readme/presentation examples/readme/presentation.mote
```

Each source should produce one PNG. Inspect every generated image for overflow,
clipping and readable labels before updating the README. Keep its excerpts in
step with the corresponding source. Images live separately in
[docs/images/readme](../../docs/images/readme/), and the main README references
them directly.

The previews are produced by Mote's native renderer. They are not Word or
PowerPoint captures and do not establish Office visual conformance. Editable
packages and PDFs can be generated using the commands in the main README;
temporary outputs belong under `target/readme/`.
