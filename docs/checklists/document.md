# Document configuration checklist

[Checklist index](README.md)

## Declaration and attributes

- [x] Named arguments only, no body; produce no visible content node.
- [x] Document-wide configuration; repeated calls merge page and margin fields.
- [x] `page.size`: named paper size; `page.width` and `page.height`: non-negative lengths; `page.orientation`: portrait or landscape.
- [x] `margin.top`, `right`, `bottom`, `left`, `header`, `footer`, `gutter`: non-negative lengths.
- [x] Retain authored arguments separately from normalized geometry in JSON.
- [x] Reject flat page/margin aliases and invalid geometry objects or values.
- [x] `columns.count`: positive integer, including an integer string.
- [x] `columns.gap`: non-negative length; `separator`: normalized boolean.
- [x] `columns.equal_width`: normalized boolean retained in the model; equal-width
  output only. Unequal widths need individual column geometry and remain unsupported.
- [x] Repeated column declarations merge fields; omitted count creates no columns.
- [x] `default_font`: literal typeface or declared font alias, including script mappings
  and a Latin fallback for a script-only alias.
- [x] `default_size`: document text size, with unitless values in points.
- [x] `language`: default text language; empty values leave the output language unset.
- [x] Document defaults remain separate from explicit styles and direct attributes.
  Other `default_*` fields are not automatically supported formatting.

Extra document properties and page fields may be retained as metadata. This does
not give them an output effect. An unflattened `properties` argument is metadata.
Explicit section boundaries and header/footer content need separate directives.

## Specification inventory

The complete [section property inventory](attributes/word-paragraphs.md#w-ct_sectpr)
includes every retained Strict and Transitional field and its nested types:
[page size](attributes/word-paragraphs.md#w-ct_pagesz),
[margins](attributes/word-paragraphs.md#w-ct_pagemar),
[columns](attributes/word-paragraphs.md#w-ct_columns), page borders, paper sources,
page numbering, text direction, vertical alignment, line numbering, grids,
header/footer references, section starts, revision metadata, and note settings.
Unmapped entries remain unchecked; they are not invented Mote argument names.
[Style defaults](attributes/styles.md#w-ct_docdefaults),
[font selection](attributes/fonts.md), and
[run properties](attributes/word-runs.md) inventory inherited document defaults.
[Word extensions](attributes/word-extensions.md) includes section extensions.

## Output limits and evidence

DOCX writes text defaults for font, size and language without flattening them
into direct formatting. [Default tests](../../tests/docx_defaults.rs) cover
script mappings, precedence and size rounding. DOCX emits a single section with the configured paper, margins and equal-width columns. Missing values
use Letter, one-inch body margins, half-inch header/footer distances, and zero
gutter. Named-size dimensions, explicit overrides, orientation, unit conversion,
rounding, repeated declarations and overflow failures are covered by
[configuration tests](../../tests/configuration.rs) and
[DOCX configuration tests](../../tests/docx_configuration.rs), including frozen
package/model captures. Unsupported fields fail at output. Native layout and real
Word visual review remain unreviewed.

[Specification](../specification/document-configuration.md),
[evaluator](../../src/evaluation/configuration.rs), and
[writer](../../src/docx/configuration.rs) define and implement the checked scope.

Native PDF/PNG implements equal-width column flow, gap, and separator geometry.
Rules appear only between used columns and extend to their deepest content.
Explicit column breaks advance through the configured columns; page breaks start
new physical pages. The final implicit section is not balanced. Unequal widths
remain unsupported. See the [column captures](../../tests/fixtures/native-columns/README.md).
