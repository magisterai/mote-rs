# Row

[Language](../specification/tables.md) · [Model/DOCX tests](../../tests/tables.rs) · [Native tests](../../tests/table_rendering.rs)

[Directive index](README.md) · [Standard properties](attributes/tables.md)

- [x] One required body containing cells and separating whitespace; empty-row warning.
- [x] `repeat_header`: boolean, mapped to a repeating DOCX header flag.
- [x] `height`: nonnegative length; `height_rule`: auto, at_least, exact.
- [x] `cant_split`: boolean, mapped to a DOCX row-splitting request.
- [x] `properties`: flattened overrides and retained metadata.

Row grid omissions, revisions, hidden state, table-property exceptions and other
standard properties remain in the linked inventory without invented Mote names.

Language/model and editable DOCX support are verified by captured packages.

Native PDF/PNG repeats contiguous leading header rows and honors `cant_split`.
The header rows lie on the page of the row that follows them: a table begins
on a page only if the page holds its header rows and the row below them, as
Word's export of the
[table-header-rows probe](../../tests/office/probes/table-header-rows.mote) shows
and the [page border tests](../../tests/table_page_borders.rs) hold. A header
row that begins a page is not preceded by the header rows. Of a row that can be
divided the page need hold the first part only, with or without header rows
above it. A row of exact height is not divided, and a row of a least height
begins only on a page that holds that height, as Word's export of the
[table-row-division probe](../../tests/office/probes/table-row-division.mote)
shows.
Repeated headers have distinct placements in successive columns on the same
page; the two-column fixture has captured native geometry and rasters.
Ordinary rows and merged groups can continue across pages and columns at safe
content boundaries. Oversized atomic rows warn about clipping; an oversized merged
group without a safe split warns and falls back to individual rows.
`height` and `height_rule` take effect in editable DOCX and in native output alike.
A native row whose exact height cuts off the content of a cell gives a warning
that names the row and the page on which its table begins; a nested table is
named as such. DOCX output measures no content and gives no such warning. See
the [overflow tests](../../tests/text_overflow.rs).

Microsoft Word visual review remains unreviewed.

Positioned-slide rows use their containing box and explicit heights rather than
page fragmentation. The height of a slide row is a minimum: the row grows to
hold the text of its cells, as PowerPoint grows it. The rows that state no
height share a part of the box: a row whose text needs more than an equal
share takes what it needs, and the remainder is divided equally among the
others, so that the table leaves its box only where the needed heights exceed
it. PPTX states the same heights as native output. A cell of rotated text does not grow its row, and warns where its text
is cut off. Slide sizing and merge continuity are covered by the
[positioned table captures](../../tests/fixtures/presentation-tables/README.md); document pagination flags do not imply slide pagination.
