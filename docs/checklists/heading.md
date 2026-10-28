# Heading checklist

[Checklist index](README.md) · [Specification](../specification/basic-content.md)

## Content and arguments

- [x] One required inline body, including an empty body.
- [x] Optional positive integer level, supplied positionally as a number or by
  name as a number/integer string; reject supplying it both ways.
- [x] Hash Markdown headings and omitted-level semantics.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

- [x] `level`: the documented hierarchy value; no output-format limit imposed by
  the current model.
- [x] `alignment`: `left`, `center`, `right`, or `justify`.
- [x] `spacing.before` and `spacing.after`: non-negative unit-bearing lengths.
- [x] `spacing.line`: length or `rule`/`value` object using `multiple`, `exact`,
  or `at_least`.
- [x] `spacing_before` and `spacing_after`: flat spacing aliases and precedence.
- [x] `line_spacing`: flat line-spacing control for presentation text.
- [x] `indent`: `left`, `right`, `first_line`, and `hanging` lengths.
- [x] `style`: heading paragraph-style reference, preserving omission without inserting a default.
- [x] `keep_with_next`: pagination boolean recorded in the model.
- [x] `page_break_before`: pagination boolean recorded in the model.
- [x] `widow_control`: pagination boolean recorded in the model.
- [x] `keep_together`: normalize and retain the existing pagination request;
  verify its output scope separately.
- [x] `numbering.enabled`: enable heading numbering.
- [x] `numbering.id`: numbering-sequence identity.
- [x] `numbering.format`: `decimal`, `lower_alpha`, `upper_alpha`, `lower_roman`,
  or `upper_roman`.
- [x] `numbering.separator`: separator between hierarchical numbering levels.
- [x] `numbering.start`: positive starting number.
- [x] `numbering.restart`: restart or continue the sequence.
- [x] `numbering.label` and `numbering.prefix`: text metadata; no heading-label
  generation is implied.
- [x] `fill` and `outline_level`: normalize as on paragraphs.
- [x] Markdown `marker = "#"`: retain source identity separately from level.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

Style references, sequence intent, and pagination requests are defined in the
formatting chapter. Effective styles and counters remain layout/output work.
Numbering retains `label` and `prefix` as text metadata, without generating
heading labels. An authored marker is not an arbitrary heading-number template.

Presentation container text defaults belong to their future container checklist;
an unrestricted nested `text` object is not a direct heading-formatting target.

The evaluator retains `keep_together` on headings; its output scope still needs
verification, independently of the completed attribute contract.

## Output

- [ ] Define heading styles, outline mapping, output level limits, and numbering
  behaviour for each applicable output.
- [ ] Verify editable Office output and independent native PDF/PNG, including
  heading pagination and numbered sequences.

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and property overrides.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [evaluation tests](../../tests/evaluation.rs),
and [specification examples](../../tests/conformance.rs) cover the checked items.
