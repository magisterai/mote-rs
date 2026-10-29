# Strong checklist

[Checklist index](README.md) · [Specification](../specification/basic-content.md)
· [Inline nesting](../specification/inline-text.md)

## Content and arguments

- [x] Bold intent with one required inline body, including an empty body; no
  positional arguments.
- [x] Markdown strong forms and nesting with emphasis without extra whitespace.
- [x] Block content is invalid inside the body.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

All attributes retain their authored values without normalization or field-type
checks. The names below describe formatting conventions, not extra validation.

- [x] `style`: named character style.
- [x] `font`: font name or declared font alias.
- [x] `size`: text size.
- [x] `color`: text color.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

The attributes above target native and presentation text formatting. Direct
attributes on `strong` are not part of the DOCX mapping in this scope; DOCX bold
content uses inherited run formatting. Composition through styled spans is
defined in [Spans](../specification/spans.md).

The strong element retains bold intent independently of its formatting values.
Style and font references are recorded without resolving their declarations.

## Output limits

DOCX supports bold intent and nested inline overrides, with Latin and
complex-script properties. Direct attributes on strong remain outside the DOCX
mapping and produce an error. [Structural evidence](../../tests/fixtures/docx/inline-formatting/README.md)
is captured; real Word review is unreviewed and native PDF/PNG uses the shared text plan. See the
[native output fixtures](../../tests/fixtures/native-output/README.md).

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word run properties](attributes/word-runs.md).
- [DrawingML run properties](attributes/drawing-text.md#a-ct_textcharacterproperties).
- [Word typography and effects](attributes/word-extensions.md).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [evaluation tests](../../tests/evaluation.rs),
[inline tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
