# Table

[Language](../specification/tables.md) · [Model/DOCX tests](../../tests/tables.rs) · [Native tests](../../tests/table_rendering.rs)

[Directive index](README.md) · [Standard properties](attributes/tables.md)

- [x] Quoted named-style shorthand, preserving the existing attribute and output
  rules; see [style application syntax](style.md#application-syntax).

- [x] One required body containing rows and separating whitespace; empty-table warning.
- [x] `style`: named table style, including inherited and conditional-region defaults.
- [x] `width`, `indent`, `cell_spacing`: nonnegative lengths.
- [x] `layout`: auto, autofit, fixed; `alignment`: left, center, right, justify.
- [x] `cell`: default margin, fill, alignment, vertical_alignment, text_direction, no_wrap and borders.
- [x] `borders`: top/right/bottom/left/inside_h/inside_v with style, size/width and color.
- [x] `properties`: flattened overrides; retained unknown fields do not imply output support.
- [x] Markdown pipe tables, delimiter alignment, bold repeating header cells.

The standard inventory includes floating placement, revision metadata, logical
edges, conditional formatting, table grid and all nested property/value domains.
Only the documented Mote mappings are in implementation scope. Output support
and Office visual review must be recorded separately.

Language/model and editable DOCX support are verified by captured packages.

Native PDF/PNG supports width selection, alignment, cell defaults, row-based
conditional styling, nested tables, shared physical borders and page/column flow.
The fill and the borders of a paragraph in a cell are painted as the
[cell checklist](cell.md) describes.
A table lies the width of its top border below the content above it, in the
body and in a cell, and the row that begins a page lies the width of its top
border below the top of the text area; a repeated header row there takes the top
border of the table. Word draws the bottom border of the table beneath the last
row on a page, so a row fits the foot of a page that holds it and the excess of
that border over its own. The column that holds a nested table holds also the
halves of its side borders that lie outside its grid. A cell border that states
no colour takes the colour that DOCX writes for it. The
[page border tests](../../tests/table_page_borders.rs) hold these rules, which
Word's exports of the
[nested-table-cell-end](../../tests/office/probes/nested-table-cell-end.mote) and
[table-page-borders](../../tests/office/probes/table-page-borders.mote) probes
show.
A cell takes each property from the cell itself, else from the `cell` attribute
of its table, else from a region of the table style, else from what the style
gives every cell. Two properties depart from that order, as Word does: the
margin of a region outranks the margin of the table, and the vertical alignment
that the style gives every cell outranks that of a region. The last row outranks
the band on which it lies. A margin is resolved side by side, and a side that
nothing states is 4 points above and below and 6 points at the sides; DOCX
writes the resolved margins on the table, since Word gives a cell no margin on a
side that neither the table nor its style states and does not take the margin of
a style from the style on which it is based. A table takes its width and its
alignment from its style unless it states them, and DOCX writes both on the
table, since Word takes neither from a style; the alignment that a style gives
the table is not the alignment of the text of its cells. The
[precedence tests](../../tests/table_style_precedence.rs) hold these rules, which
Word's export of the
[table-style-precedence probe](../../tests/office/probes/table-style-precedence.mote)
shows.
Two adjacent tables are parted by an empty paragraph in DOCX, without which
Word would join them, and native output sets the same paragraph between them
([block-gaps probe](../../tests/office/probes/block-gaps.mote)).
`indent`, `cell_spacing` and `layout` affect editable DOCX only; native output warns
that these properties have no effect. First/last-column regions and column banding
also remain editable-output capabilities. See the [capture notes](../../tests/fixtures/tables/README.md)
for comparison coverage and font requirements.

Microsoft Word visual review remains unreviewed.

Positioned-slide table output is inventoried separately from document flow:
box geometry, table width/alignment, explicit column widths, row heights,
row/column merges, cell margins/fills/text direction/alignment, declared table
regions and text defaults, and physical border ownership. Editable PPTX and native PDF/PNG are covered by the
[positioned table captures](../../tests/fixtures/presentation-tables/README.md). Table styles may select `style_id` or
`powerpoint_style_id`; first/last row/column and banding flags are captured.
A row is never lower than the text of its cells. The rows that state no height
share a part of the box, a row that needs more than an equal share taking it
from the slack of the others; native output and PPTX state the same heights.
Both outputs warn where the rows cannot fit and pass the lower edge of the
box, and where rotated text is cut off in its cell.
See the [overflow tests](../../tests/text_overflow.rs).
PowerPoint visual review remains unreviewed.

The [focused table cases](../../tests/cases/README.md) and
[alignment tests](../../tests/table_alignment.rs) also cover Markdown column
alignment, cell and paragraph overrides, table-style paragraph spacing, and
margin shorthands in both editable and native slide output.
