# Cell

[Language](../specification/tables.md) · [Model/DOCX tests](../../tests/tables.rs) · [Native tests](../../tests/table_rendering.rs)

[Directive index](README.md) · [Standard properties](attributes/tables.md)

- [x] One required body with block content; ordinary text becomes paragraphs.
- [x] `style`: named style reference retained for native formatting and applicable table-style regions.
- [x] `width`: nonnegative length; `colspan`, `rowspan`: positive integers.
- [x] `fill`: hexadecimal color.
- [x] `alignment`: left, center, right, justify; `vertical_alignment`: top, center/middle, bottom.
- [x] `margin`: nonnegative top/right/bottom/left lengths.
- [x] `borders`: per-edge style, size/width and color, emitted as editable cell borders.
- [x] `text_direction`: ltr, rtl, tb_rl, bt_lr; `no_wrap`: boolean.
- [x] `properties`: flattened overrides and retained metadata.
- [x] Paragraphs, links, images, lists and nested tables retain supported cell content.

The linked inventory includes omitted-cell grids, merge records, logical edges,
shading patterns, revision metadata and unsupported text-flow controls. Listing
those fields does not make them part of the Mote attribute vocabulary.

Language/model and editable DOCX support are verified by captured packages.

Native PDF/PNG supports column/row spans, width, margins, fill, horizontal and
vertical alignment, shared borders, and structured text, links, images, lists and
nested tables. Cell content is clipped to its physical fragment. Paragraph bands
and paragraph borders inside cells are omitted; inline highlights remain visible.
`text_direction` and `no_wrap` affect editable DOCX only and produce native support warnings.

Microsoft Word visual review remains unreviewed.
