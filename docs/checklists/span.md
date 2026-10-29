# Span checklist

[Checklist index](README.md)

## Content and arguments

- [x] One required inline body, including an empty body; no positional arguments.
- [x] Preserve nested inline structure without adding bold or italic intent.
- [x] Reject block children and multiple bodies; preserve ordinary flow and
  enclosing code whitespace rules.

## Attributes

- [x] Validate named argument types before property overrides; retain unnormalized
  font/style values supplied through `properties` and extra attributes.
- [x] `style`: character-style reference; `font`: font name or alias.
- [x] `size`: non-negative length, normalizing unit-bearing strings.
- [x] `color` and `highlight`: normalized hexadecimal colors.
- [x] `bold`, `italic`, `underline`, `small_caps`, `strike`, `double_strike`:
  normalized booleans, preserving omission and explicit false.
- [x] `properties`: check outer types, merge overrides, then normalize designated
  fields using the shared precedence rules.

## Resolution and output

- [x] Resolve enclosing text formatting, named style, and direct attributes in
  order, without modifying the authored model.

DOCX supports direct span fonts, sizes, colors, highlights, decorations and
explicit boolean overrides, including nested spans, named character styles and
script-specific font aliases. [Structural evidence](../../tests/fixtures/docx/inline-formatting/README.md)
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

[Specification](../specification/spans.md),
[implementation](../../src/evaluation/attributes.rs),
[language tests](../../tests/spans.rs), [resolver](../../src/formatting.rs),
[resolution tests](../../tests/formatting.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.

An empty direct font supplies no DOCX typeface override and preserves the enclosing
font mapping. [The captured regression](../../tests/fixtures/docx/empty-inline-font/README.md)
checks this through a nested character style and across following siblings.
