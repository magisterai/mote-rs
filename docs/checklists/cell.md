# Cell

[Language](../specification/tables.md) · [Model/DOCX tests](../../tests/tables.rs) · [Native tests](../../tests/table_rendering.rs)

[Directive index](README.md) · [Standard properties](attributes/tables.md)

- [x] Quoted named-style shorthand, preserving the existing attribute and output
  rules; see [style application syntax](style.md#application-syntax).

- [x] One required body with block content; ordinary text becomes paragraphs.
- [x] `style`: named style reference retained for native formatting and applicable table-style regions.
- [x] `width`: nonnegative length; `colspan`, `rowspan`: positive integers.
- [x] `fill`: hexadecimal color.
- [x] `alignment`: left, center, right, justify; `vertical_alignment`: top, center/middle, bottom.
- [x] `margin`: nonnegative top/right/bottom/left lengths; `all`, `horizontal`,
  and `vertical` shorthands, with explicit edges taking precedence, including zero.
- [x] `borders`: per-edge style, size/width and color, emitted as editable cell borders.
- [x] `text_direction`: ltr, rtl, tb_rl, bt_lr; `no_wrap`: boolean.
- [x] `properties`: flattened overrides and retained metadata.
- [x] Paragraphs, links, images, lists and nested tables retain supported cell content.

The linked inventory includes omitted-cell grids, merge records, logical edges,
shading patterns, revision metadata and unsupported text-flow controls. Listing
those fields does not make them part of the Mote attribute vocabulary.

Language/model and editable DOCX support are verified by captured packages.
WordprocessingML requires a cell to end with a paragraph. A cell whose content
ends with a nested table is therefore closed in DOCX by an empty paragraph with
an exact line of one point. Word gives that paragraph no height, as its export
of the
[nested-table-cell-end probe](../../tests/office/probes/nested-table-cell-end.mote)
shows, and native layout reserves none; the
[table tests](../../tests/tables.rs) cover the closing paragraph at two depths.

Native PDF/PNG supports column/row spans, width, margins, fill, horizontal and
vertical alignment, shared borders, and structured text, links, images, lists and
nested tables. Cell content is clipped to its physical fragment. The fill and
the borders of a paragraph in a cell are painted above the fill of the cell by
the rule of body text, measured from the text area of the cell, and are cut off
at the edge of the cell; the [paragraph checklist](paragraph.md) states the rule
and the [cell paragraph tests](../../tests/cell_paragraphs.rs) cover a repeated
header row, a merged cell, a vertically aligned cell, a nested table and a row
split between pages. Word's PDF export shows the same behaviour in each of these
cases in the
[cell-paragraph-shading probe](../../tests/office/probes/cell-paragraph-shading.mote):
the shading ends at the borders of the cell, no part of a rule is drawn beyond
the cell, and the lines lie within half a point of the native lines. The lines
of a nested table lie up to 0.75 point below the native lines, and those of
the rows on a page to which a table continues lie half a point below them,
whether or not a paragraph is shaded. The fill of a list item in a cell is not painted, and is reported.
`text_direction` and `no_wrap` affect editable DOCX only and produce native support warnings.

Microsoft Word visual review remains unreviewed.

Positioned-slide cells have [captured coverage](../../tests/fixtures/presentation-tables/README.md)
for merged slots, conditional style precedence, margins, vertical text, clipping
and shared physical edges. PowerPoint
wraps table-cell text; `no_wrap` must report that limitation in both slide outputs.

[Focused source cases](../../tests/cases/README.md) and
[alignment tests](../../tests/table_alignment.rs) check cell/paragraph alignment
precedence, vertical alignment with paragraph spacing, and margin shorthands.

Within table cells, supported quote and definition paragraph boundaries remain
separate in editable and native output. See the [authoring content
captures](../../tests/fixtures/authoring-content/README.md).
