# Break checklist

[Checklist index](README.md)

## Content and attributes

- [x] Block `break` with no positional arguments or body.
- [x] `type`: string, default `page`; page, column and line breaks.
- [x] Shared `properties` merging and extra metadata retention; validate the outer
  type before applying an override.
- [ ] Section-related types: `section`, `next_page`, `continuous`, `even_page`,
  `odd_page`, `next_column`. These need the explicit section model and are deferred.

## Specification inventory

[Word break properties](attributes/word-runs.md#w-ct_br) inventory `type` and
`clear`; [break values](attributes/values-word.md#w-st_brtype) retain the standard
values. A Mote block line break is distinct from an inline `line_break` directive.
Section starts belong to the complete
[section inventory](attributes/word-paragraphs.md#w-ct_sectpr).
DrawingML break/run properties remain in
[DrawingML text](attributes/drawing-text.md#a-ct_textlinebreak).

## Output and evidence

[Specification](../specification/flow-breaks.md) and
[language/output tests](../../tests/breaks.rs) cover top-level page, column and
block line breaks, column prefixes, consecutive/trailing breaks, heading spacing,
metadata and invalid calls. Nested breaks remain model-only and fail DOCX output.
Explicit breaks are not pagination estimates.

DOCX writes a page break that follows a paragraph or a heading at the end of
that paragraph, and otherwise in a paragraph of its own. A paragraph that holds
the break alone needs a line of room, and Word leaves a page empty where the
page before the break has none; Word's export of the
[page-break probe](../../tests/office/probes/page-break.mote) had three such
pages before the break was written so, and has none now. A page break that
follows another is written in a paragraph of its own, so that the page between
them stays empty. DOCX writes a block line break as an empty paragraph, since a
paragraph that holds a line break alone occupies two lines in Word. A column
break begins the block after it where that block is written as a paragraph: a
paragraph, a heading, a quotation, a literal block, a divider, a definition
list, or a shape or group set in the text. Before another block the break is
written in a paragraph of its own, whose mark Word sets as an empty line at the
head of the next column.

Native PDF/PNG applies page and column breaks to the shared flow cursor, and
lays out a block line break as an empty paragraph. The block that a page break
sets at the head of a page lies there without the space before it, as in Word,
whether the break is a `break` or the `page_break_before` of a paragraph. Two
adjacent tables are parted by an empty paragraph, which DOCX must write between
them. Word's exports of the page-break probe and of the
[block-gaps probe](../../tests/office/probes/block-gaps.mote) show these rules,
and the [break tests](../../tests/breaks.rs) hold them. A column break advances to the next column, then to the next page when
no column remains. See the [column captures](../../tests/fixtures/native-columns/README.md).
