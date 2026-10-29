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
Native layout and real Word visual review remain unreviewed.

Native PDF/PNG applies page, column, and block line breaks to the shared flow
cursor. A column break advances to the next column, then to the next page when
no column remains. See the [column captures](../../tests/fixtures/native-columns/README.md).
