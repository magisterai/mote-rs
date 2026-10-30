# Positioned presentation text

These fixed sources cover explicit slide boundaries, wide/standard/custom page
sizes, solid backgrounds, theme colors and fonts, editable paragraphs, headings,
styles, lists, task marks, links, code, margins, alignment, clipping, fills and
borders. Empty, zero-size and off-canvas boxes remain visible as boundary cases.
Lua constructs the same model as direct directives. Shared fitting covers shrink
and resize modes, including explicit source-relative font files.

Each capture includes the evaluated JSON model, a source SHA-256, and a manifest
of every XML package part. Identical parts share content-addressed files. Tests
compare complete canonical XML, including relationships, themes, masters and
layouts; only volatile core timestamps are ignored. Package equality does not
establish PowerPoint layout conformance. These cases are unreviewed in PowerPoint.

Native captures cover eight sources and eleven slide pages. Every ordered paint
operation and text fragment is compared numerically, with fixed source and font
hashes. PNG comparisons use per-case bounds on channel differences and visible
pixel averages; no pixel may differ by more than 64 channel levels. Nine PDF
pages matched the captured raster exactly at 144 DPI. Two top-edge headings were
absent from the captured PDF but present in its PNG; the native painters retain
signed glyph coordinates and clip them at the page edge.

The formatting case deliberately includes a numeric line spacing of 1.2 twips.
Its overlapping lines exercise a subpixel highlight band; the geometry is fixed.
The `edges` case intentionally extends a box beyond the canvas.

Native output requires at least one explicit slide. `#slide[]` creates a blank
slide. The declaration-only `empty` source retains package coverage; it does not
have a native capture. Text-box shadows and automatic slide layout remain
unsupported. Positioned [pictures](../presentation-images/README.md),
[tables](../presentation-tables/README.md), [drawings](../presentation-shapes/README.md),
[charts](../presentation-charts/README.md), [backgrounds](../presentation-backgrounds/README.md)
and [fields/notes](../presentation-fields/README.md) have separate captures.
