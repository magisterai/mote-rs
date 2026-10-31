# Combined rendering captures

These sources exercise interactions between already defined attributes. They add
coverage to the focused directive fixtures, rather than replacing their contracts.
See the [coverage map](../../../docs/rendering-coverage.md) and
[regression tests](../../rendering_audit.rs).

| Source | Combined behavior |
| --- | --- |
| `document-typography` | Declared fonts, mixed sizes, justified wrapping, exact/minimum spacing, indentation, semantic formatting, tabs, raw text and definitions. |
| `document-pagination` | Keep chains, widow control, a kerned footnote, page references and current/total running fields across four pages. |
| `document-sections` | Continuous transitions into and out of two columns, balancing, inherited stories and changed margins. |
| `document-table-flow` | Natural row splitting, repeated headers, rowspan continuity, lists, nested tables and following text across two pages. |
| `document-table-edges` | Conditional table styling, fractional widths, adjacent conflicts, none/visible edges, merged cells and a borderless control. |
| `document-objects` | A transformed linked picture, caption and signed chart in surrounding document flow. |
| `document-drawing-metrics` | Document shapes with calibrated font aliases, bottom/center alignment, code runs and explicit line spacing. |
| `document-formatted-notes` | Footnotes inside all six semantic wrappers, note-band reservation, declared-font kerning, a term footnote and a definition endnote. |
| `slide-text` | Mixed sizes/scripts, exact line spacing, bottom alignment, monospace line metrics, numbered markers, shrink/clip/resize and rotated shape text. |
| `slide-table` | Row heights, merged cells, vertical text, paragraph content, no-wrap behavior and border conflicts. |
| `slide-media-edges` | Background image, translucent foreground, crop, flips, rotation, effects, dashed strokes, linked media, shadow and canvas corners. |
| `slide-positioned-charts` | Four independently positioned chart kinds, signed and zero values, titles, legends and data-label positions. |
| `slide-layouts` | Derived layouts, slot order, typed pictures, local styling, margins and inherited geometry. |
| `slide-automatic` | Automatic text, nested lists, tables, images and footer fields on a standard-ratio slide. |
| `slide-charts` | Four loose charts receive independent automatic frames. The deliberately crowded result retains the equal-height layout limit. |
| `slide-chart-automatic` | Two loose charts with sufficient vertical space; both editable objects and native titles remain present. |
| `slide-chart-percentages` | Negative-only, mixed-sign and zero categories in percentage columns, bars and areas, including percentage labels. |
| `slide-chart-legends` | Long series names at all four legend positions, wrapped rows and separate axis-title space. |
| `document-chart-labels` | Complete long labels above bar fills, wrapped legends and signed percentage charts in document flow. |

`cases.json` identifies each output format. Nineteen cases contain twenty-nine
native pages. Each capture records source and
asset hashes, complete model JSON, all editable package parts, and native PNGs.
Repeated package parts are stored once by content hash. Package comparison
canonicalizes XML syntax and volatile metadata; it does not remove formatting,
relationships, chart data or workbooks. The input model must remain unchanged.

Native PNG limits are fixed per page: maximum channel difference, mean difference
over the union of nonwhite pixels, and the count of pixels above 64/255. Ordinary
text pages have no pixels above 64. Rotated text, patterned edges and transformed
images have explicitly bounded edge exceptions. Limits must not be increased to
accommodate changed wrapping, positioning, pagination or missing objects.

All twenty-nine native PDF pages and corresponding PNGs were inspected. Twenty-two
PDF pages are pixel-identical at 144 DPI; the remaining differences are confined
to text rasterization and existing edge rounding. Signed PDF coordinates retain
rotated axis titles whose unrotated text begins outside the page.

Percentage stacks normalize by total magnitude and preserve signs. Zero categories
paint no bar area. Complete labels fit their allocated frames, bar labels paint
above every series, and legends reserve measured rows. These rules do not provide
collision avoidance for every label or automatic pagination of crowded charts.
The four-chart automatic case remains visibly cramped. Long decimal tick text,
radial-label collisions, document picture border limits and output-specific table
properties remain visible. `category_axis`/`value_axis` and `show_*` label fields in
the earlier cases remain extra fields without their own output mapping; the new
cases use the documented `axis` and `data_labels` vocabulary.
Existing sources were not changed to hide these behaviors.

These are native-rendering and package comparisons. Microsoft Word and PowerPoint
visual review remains unreviewed.
