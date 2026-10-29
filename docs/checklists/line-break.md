# Line-break checklist

[Checklist index](README.md) · [Specification](../specification/inline-text.md)

## Content and arguments

- [x] An inline `line_break` element with no positional arguments or body.
- [x] A break remains inside its paragraph and adds no surrounding spaces.
- [x] Markdown hard breaks from two trailing spaces or a backslash before a line
  ending; their model representation is a line feed in text.

## Attributes

- [x] Accept named metadata and the shared `properties` object; retain values
  without formatting normalization. Positional arguments and bodies remain invalid.

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

Named attributes are retained metadata and do not create page or column breaks.
Those belong to the separate [break directive](break.md).

## Output limits

Explicit and Markdown hard breaks emit `w:br` in DOCX. Consecutive breaks and
placement among formatted runs have [structural coverage](../../tests/fixtures/docx/text-fidelity/README.md).
Named line-break attributes remain unsupported in output. Real Word review is
unreviewed; native PDF/PNG output remains unsupported.

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word break attributes](attributes/word-runs.md#w-ct_br).
- [DrawingML break properties](attributes/drawing-text.md#a-ct_textlinebreak).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [inline tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
