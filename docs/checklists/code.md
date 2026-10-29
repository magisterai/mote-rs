# Code checklist

[Checklist index](README.md) · [Specification](../specification/inline-text.md)

## Content and arguments

- [x] Optional scalar positional text: string, number, or boolean.
- [x] Optional evaluated inline body; reject block children and multiple bodies.
- [x] Positional text takes precedence over an evaluated, validated body.
- [x] An empty call creates empty inline code.
- [x] Literal backtick spans, matching delimiters, padding-space rules, and
  preservation of literal content.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

Outer `style` and `language` arguments require strings; `preserve_space` is
normalized after merging. Other attributes retain their authored values.

- [x] `style`: character-style reference; omission remains unspecified in the model.
- [x] `preserve_space`: boolean control for literal spaces, blank lines, and line
  ending normalization through nested inline bodies. Tabs remain text until output.
- [x] `font`: font name or alias for native and presentation text.
- [x] `size`: text size for native and presentation text.
- [x] `color`: text color for native and presentation text.
- [x] `language`: optional language-label metadata. It does not imply syntax
  highlighting or an output formatting effect.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

The formatting chapter defines font/style references, colors, and preserved
whitespace before their examples. The direct
`font`, `size`, and `color` controls are not part of the DOCX code mapping in this
scope; DOCX code formatting uses styles and inherited formatting.

Fenced and block code belong to a future block-content checklist. They are not
completed by the inline backtick implementation. Preserve-space behaviour needs
separate presentation verification and truthful warnings for degraded output.

## Output limits

DOCX supports the default Code character style, inherited formatting, nested
inlines, preserved whitespace, and four-space expansion of preserved tabs.
Custom character-style references are emitted. Direct font/size/color attributes have
no DOCX mapping and produce an error. [Structural evidence](../../tests/fixtures/docx/inline-formatting/README.md)
is captured; real Word review is unreviewed and native PDF/PNG uses the shared text plan. See the
[native output fixtures](../../tests/fixtures/native-output/README.md).

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word run and whitespace properties](attributes/word-runs.md).
- [DrawingML run properties](attributes/drawing-text.md#a-ct_textcharacterproperties).
- [Word typography and effects](attributes/word-extensions.md).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [inline tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
