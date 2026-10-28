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

- [x] `style`: named character style.
- [x] `font`: font name or declared font alias.
- [x] `size`: text size.
- [x] `color`: text color.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

These direct attributes target native and presentation text formatting. DOCX
italic content uses inherited run formatting; direct attributes on `emphasis`
are not part of its mapping in this scope. Composition through styled spans
belongs to the future span directive.

Style and font references are recorded without resolving their declarations.

## Output

- [ ] Define effective italic formatting, nested overrides, and output-specific
  handling of the attributes above.
- [ ] Verify editable Office text and independently painted native PDF/PNG text.

## Evidence

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [inline tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
