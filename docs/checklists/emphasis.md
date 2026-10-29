# Emphasis checklist

[Checklist index](README.md) · [Specification](../specification/inline-text.md)

## Content and arguments

- [x] Italic intent with one required inline body, including an empty body; no
  positional arguments.
- [x] Single-underscore Markdown form, escaping, and nesting with strong text.
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

These direct attributes target native and presentation text formatting. DOCX
italic content uses inherited run formatting; direct attributes on `emphasis`
are not part of its mapping in this scope. Composition through styled spans is
defined in [Spans](../specification/spans.md).

Style and font references are recorded without resolving their declarations.

## Output limits

DOCX supports italic intent and nested inline overrides, with Latin and
complex-script properties. Direct attributes on emphasis remain outside the DOCX
mapping and produce an error. [Structural evidence](../../tests/fixtures/docx/inline-formatting/README.md)
is captured; real Word review is unreviewed and native output remains unsupported.

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
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [inline tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
