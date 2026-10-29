# Ordered-list checklist

[Checklist index](README.md) · [Specification](../specification/lists.md)

## Content and arguments

- [x] One required body containing only items and separating whitespace; no
  positional arguments.
- [x] Ordered Markdown items, source-marker retention, grouping, and nesting.
- [x] Explicit lists use the documented item-body and direct-Markdown rules.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

- [x] `start`: positive integer number or integer string; normalize to a number
  and preserve omission. This check covers model intent, not emitted numbering.
- [x] `id`: numbering-sequence identity for continuation.
- [x] `restart`: restart or continue the sequence, with defined omission rules.
- [x] `format`: `decimal`, `lower_alpha`, `upper_alpha`, `lower_roman`, or
  `upper_roman`; retain `bullet` without changing the ordered container kind.
- [x] `level`: positive base nesting level.
- [x] `tight`: request zero after-spacing.
- [x] `indent.left`, `indent.hanging`, and `indent.level`: lengths for list
  geometry and additional indent per nesting level.
- [x] `spacing.after`: after-spacing length.
- [x] `spacing.before` and `spacing.line`: shared spacing values for presentation
  layout, without expanding paged list layout.
- [x] `marker`: retained string metadata; it does not replace generated numbering
  with an arbitrary ordered marker template.
- [x] `style`: named paragraph style applied to presentation list items; this is
  not a paged-document list-style mapping.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

The formatting chapter defines sequence intent and layout values. A list-level
`start` differs from a source item marker; the current model retains
Markdown markers without synthesizing `start`.

Arbitrary ordered marker templates, custom marker font/alignment controls, and
additional numbering formats are outside this scope. Presentation container
text/list defaults belong to their future container checklists.
Continuation support must be stated separately for each output and layout mode.

## Output limits

DOCX emits numeric and bullet formats, starts, levels one through nine, restart,
continuation and list/item geometry. Named sequences continue by default; unnamed
sequences restart unless restart=false. Continued sequences ignore later starts;
a restart replaces the instance used for later continuation. Format and geometry
distinguish definitions. Source markers supply an omitted starting value from the
first item only. The [ordered-list captures](../../tests/fixtures/docx/ordered-lists/README.md)
and [continuation capture](../../tests/fixtures/docx/list-continuation/README.md)
verify all package parts. Word review remains unreviewed. Other item-content
limits match [bullet lists](list.md#output-limits).

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word numbering and marker properties](attributes/numbering.md).
- [Word paragraph properties](attributes/word-paragraphs.md).
- [DrawingML paragraphs and bullets](attributes/drawing-text.md#a-ct_textparagraphproperties).
- [Numbering extensions](attributes/word-extensions.md#w15-restartnumberingafterbreak).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [list tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
