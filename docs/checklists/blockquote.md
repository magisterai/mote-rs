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
formatting chapter; presentation containers remain future work. DOCX's scoped quote
attributes are `style`, `border`, and `fill`; native and presentation controls
listed above do not imply additional DOCX support. Presentation border/fill
behaviour must not be inferred from paged-document support.

Nested `text` and `paragraph` defaults belong to future presentation container
checklists; they are not extra direct quote-formatting controls in this scope.

## Output

- [ ] Define how formatting applies to quote children for each supported output,
  including unsupported-property diagnostics.
- [ ] Verify Office output and independent native PDF/PNG with multiple paragraphs,
  lists, nesting, and quote formatting.

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [quote tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
