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
| `slide-charts` | Intentional unsupported composition: multiple loose charts must report an error instead of producing an empty slide. |

`cases.json` identifies each output format and the intentional failure. Fourteen
successful cases contain twenty-one native pages. Each capture records source and
asset hashes, complete model JSON, all editable package parts, and native PNGs.
Repeated package parts are stored once by content hash. Package comparison
canonicalizes XML syntax and volatile metadata; it does not remove formatting,
relationships, chart data or workbooks. The input model must remain unchanged.

Native PNG limits are fixed per page: maximum channel difference, mean difference
over the union of nonwhite pixels, and the count of pixels above 64/255. Ordinary
text pages have no pixels above 64. Rotated text, patterned edges and transformed
images have explicitly bounded edge exceptions. Limits must not be increased to
accommodate changed wrapping, positioning, pagination or missing objects.

All twenty-one native PDF pages and corresponding PNGs were inspected. Seventeen PDF
pages were pixel-identical to captured output at 144 DPI; three differed by at
most nineteen pixels with channel differences at most six. The remaining chart
page has an explicit PDF capture discrepancy at a rotated axis title whose
unrotated text starts outside the page: the current painter preserves the signed
coordinates, keeping the title visible. Native PNG and editable chart content
retain the title. This exception is not evidence of Office conformance.

Retained limitations stay visible. Long chart labels can clip, negative percentage
stacks use the existing denominator policy, document picture borders stay
unrotated and solid, and some table properties differ between document and slide
layout. `category_axis`/`value_axis` in the document chart and `show_*` label
fields in the positioned chart are retained extra fields with no output effect;
the supported `axis` fields and label positions are independently exercised.
Sources are not changed after capture to hide these behaviors.

These are native-rendering and package comparisons. Microsoft Word and PowerPoint
visual review remains unreviewed.
