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
Those belong to a separate future directive and checklist.

## Output

- [ ] Map the explicit element and Markdown hard break to equivalent visible line
  breaks in each applicable output.
- [ ] Verify Office and independent native PDF/PNG line placement within the
  existing paragraph.

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [inline tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
