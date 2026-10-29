# Tables

[Specification](README.md) · [Table checklist](../checklists/table.md) ·
[Row checklist](../checklists/row.md) · [Cell checklist](../checklists/cell.md)

A table contains rows; each row contains cells. These directives each require
exactly one body and accept named arguments. A table body contains only rows and
separating whitespace; a row body contains only cells and separating whitespace.
Empty tables and rows are accepted with a warning. A cell body accepts block
content, including ordinary text, which becomes a paragraph.

```mote
#table[
  #row(repeat_header = true)[#cell[Name] #cell[Amount]]
  #row[#cell[Subscriptions] #cell[120]]
]
```

This produces a block `table` with two `row` children, each containing two
`cell` children. Rows and cells have the `table_part` category in JSON. Each cell
contains one paragraph. Only the first row has `repeat_header = true`.

`width`, `indent`, and `cell_spacing` on a table are nonnegative lengths.
`layout` accepts auto, autofit or fixed. `alignment` accepts left, center, right,
or justify; justify uses left table placement. `style` names a table style as
defined in [Styles and fonts](styles-and-fonts.md). A table's `cell` object
supplies defaults for margin, fill, alignment, vertical_alignment,
text_direction, no_wrap and borders. Individual cells override those defaults.
Use `cell.margin`; `cell_padding` is rejected.

Rows accept boolean `repeat_header` and `cant_split`, a nonnegative `height`,
and `height_rule` of auto, at_least or exact. Repeated headers are the consecutive
leading header rows. Exact heights may clip content; at_least permits expansion.
Ordinary rows may split across pages unless `cant_split` applies.

Cells accept nonnegative `width`, positive integer `colspan` and `rowspan`, and
hexadecimal `fill`. Spans occupy logical grid columns and rows; following cells
use the next available column. `alignment` selects left, center, right or justify
text. `vertical_alignment` accepts top, center, middle or bottom; middle
normalizes to center. `margin` has top, right, bottom and left lengths; the older
name `padding` is rejected. Margin shorthands apply in order: `all`, then
`vertical`/`horizontal`, then individual edges. `no_wrap` is boolean. `text_direction` accepts ltr,
rtl, tb_rl or bt_lr. `style` supplies a named style for native cell formatting and applicable table-style
regions. DOCX applies table text defaults without adding a paragraph-style
reference solely from this cell attribute.

Table and cell `borders` objects specify physical edges: top, right, bottom and
left, plus table inside_h and inside_v. Each edge records style, width or size,
and hexadecimal color. `all` supplies every edge; table `outside` and `inside`
override their edge groups, followed by individual sides. `insideH`/`insideV`
are retained aliases that take precedence over `inside_h`/`inside_v`. Border
`width` is in points when numeric, otherwise a length; `size` is in eighths of a
point. `space` records a point distance in DOCX, and `val` is a fallback for
`style`. `none` and `nil` remove the edge. Standard border values beyond the implemented subset
remain listed in the attribute inventory; recording them does not imply painting
support. Border conflicts are resolved once per shared physical edge.

`properties` follows the usual flattened-override rules for all three directives.
Nested cell content uses the definitions from earlier chapters: paragraphs,
headings, spans, links, lists, quotes and images. Tables can also be nested in
cells. Retained properties without a native effect produce diagnostics.

A Markdown table requires a header line and a matching delimiter line with at
least two pipe-separated cells. Delimiters contain at least three hyphens;
leading/trailing colons select left/right alignment, and both select center.
Outer pipes are optional. Escaped pipes and pipes inside brackets or parentheses
do not divide cells. The header row repeats and its nonempty cells are bold.

```mote
| Name | Amount |
| :--- | ---: |
| Subscriptions | 120 |
```

This produces the same two-row table structure as above, with left alignment on
the first column, right alignment on the second, and `strong` content in header
cells. Markdown cells retain inline children directly; directive cell bodies
normalize text into paragraphs. A malformed delimiter remains ordinary text.
