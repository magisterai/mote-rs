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

- [ ] Resolve enclosing text formatting, named style, and direct attributes in
  order, without modifying the authored model.
- [ ] Verify editable Office output and independent native PDF/PNG.

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and property overrides.

[Specification](../specification/spans.md),
[implementation](../../src/evaluation/attributes.rs),
[language tests](../../tests/spans.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
