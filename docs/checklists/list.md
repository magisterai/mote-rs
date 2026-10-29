# List checklist

[Checklist index](README.md) · [Specification](../specification/lists.md)

## Content and arguments

- [x] One required body containing only items and separating whitespace; empty
  bodies are allowed and positional arguments are rejected.
- [x] Markdown unordered items, continuation, grouping, and nested lists inside
  item bodies.
- [x] Direct Markdown items in an explicit container follow the documented flat
  item restriction.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

- [x] `start` and `restart`: normalized numbering metadata; neither changes an
  unordered container into an ordered one.
- [x] `format`: validated list-format value; `bullet` requests unordered markers,
  and retaining a numeric format does not change the container kind.
- [x] `level`: positive base nesting level.
- [x] `tight`: request zero after-spacing.
- [x] `indent.left`, `indent.hanging`, and `indent.level`: lengths for the list
  text position, marker area, and additional indent per nesting level.
- [x] `spacing.after`: after-spacing length.
- [x] `spacing.before` and `spacing.line`: shared spacing values retained for
  presentation layout; paged unordered layout uses after-spacing.
- [x] `marker`: custom bullet text in presentation layout only; no arbitrary
  marker override is promised for paged documents.
- [x] `id`: list identity where numbering instances are emitted; it must not
  imply visible numeric bullet progression.
- [x] `style`: named paragraph style applied to presentation list items; this is
  not a paged-document list-style mapping.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

A positive `start` and boolean `restart` on `list` are specified and stored as
metadata. Their checkmarks do not claim bullet numbering. Effective numbered
sequences belong to the [ordered-list checklist](ordered-list.md).

Ordered formats on unordered lists and unrestricted marker styling are not
formatting goals here. Presentation container text/list defaults belong to their
future container checklists, rather than unrestricted direct list objects.

## Output limits

Rendered list defaults, item overrides, marker placement, nesting, and
continuation paragraphs remain unsupported.

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word numbering and marker properties](attributes/numbering.md).
- [Word paragraph properties](attributes/word-paragraphs.md).
- [DrawingML paragraphs and bullets](attributes/drawing-text.md#a-ct_textparagraphproperties).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [list tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
