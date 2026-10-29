# Block-quote checklist

[Checklist index](README.md) · [Specification](../specification/block-quotes.md)

## Content and arguments

- [x] One required block body, including an empty body; no positional arguments.
- [x] Ordinary text becomes paragraphs; explicit blocks and lists retain structure.
- [x] Each Markdown quote line creates a separate quote block.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

Only outer `style`, `border`, and `fill` types are checked. After property
merging, `fill` normalizes; other geometry remains unvalidated metadata.

- [x] `style`: paragraph-style reference, preserving omission without inserting a default.
- [x] `border`: `top`, `right`, `bottom`, `left`, and `all`; side values as on
  [paragraphs](paragraph.md).
- [x] `fill`: shading color.
- [x] `alignment`: `left`, `center`, `right`, or `justify` for native and
  presentation text.
- [x] `spacing.before`, `spacing.after`, and `spacing.line`: native and
  presentation paragraph spacing, including the line-spacing rule/value object.
- [x] `indent`: `left`, `right`, `first_line`, and `hanging` for native and
  presentation text.
- [x] `spacing_before`, `spacing_after`, and `line_spacing`: presentation spacing
  aliases and their precedence.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

Style references, colors, borders, and layout requests are defined in the
formatting chapter; presentation containers remain future work. DOCX emits
`style`, `border`, `fill`, alignment, spacing and indentation. The capture confirms
those geometry values despite unsupported-property warnings from its validator.
Presentation border/fill behaviour must not be inferred from paged-document support.

Nested `text` and `paragraph` defaults belong to future presentation container
checklists; they are not extra direct quote-formatting controls in this scope.

## Output limits

DOCX emits one paragraph per quote, with a fallback or named style and direct
paragraph properties. Nested paragraph, list and quote content flattens to inline
runs without separators or numbered markers. Separate child-block formatting is
unsupported and fails explicitly. Empty quotes remain paragraphs; an empty style
reference suppresses the fallback reference. Formatting does not escape the quote.
See [quote captures](../../tests/fixtures/docx/block-quotes/README.md) and
[tests](../../tests/docx_quotes.rs). Word review remains unreviewed.

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word paragraph properties](attributes/word-paragraphs.md).
- [Word run properties](attributes/word-runs.md).
- [DrawingML text properties](attributes/drawing-text.md).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [quote tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.

## Native output evidence

PDF and PNG use the shared block plan, including list and heading numbering,
quote text, and paragraph fills. Native paragraph borders currently use the
resolved `all.color` or `bottom.color` for a 0.35-point bottom rule on each line;
other sides, widths, and border styles are outside that painting behavior.
List item child blocks and quote child blocks flatten to inline text; nested lists
inside list items retain their own markers. See the
[native block captures](../../tests/fixtures/native-blocks/README.md) and
[block tests](../../tests/native_blocks.rs). Office visual review remains unreviewed.
