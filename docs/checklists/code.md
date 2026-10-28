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

## Output

- [ ] Define per-output style, whitespace, and unsupported-metadata handling.
- [ ] Verify Office text and independent native PDF/PNG, including spaces, tabs,
  blank lines, mixed inline formatting, and line endings.

## Evidence

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [inline tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
