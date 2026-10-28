# Paragraph checklist

[Checklist index](README.md) · [Specification](../specification/basic-content.md)

## Content and arguments

- [x] One required inline body, including an empty body; no positional arguments.
- [x] Implicit paragraphs, whitespace handling, block boundaries, and rejection of
  nested blocks in an explicit paragraph.
- [x] Omitted attributes remain absent; explicit zero spacing remains present.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

- [x] `alignment`: `left`, `center`, `right`, or `justify`, with exact spelling.
- [x] `spacing.before`: a non-negative unit-bearing length.
- [x] `spacing.after`: a non-negative unit-bearing length.
- [x] `spacing.line`: line spacing as a length or an object with `rule` and
  `value`; rules are `multiple`, `exact`, and `at_least`.
- [x] `spacing_before`, `spacing_after`, `line_spacing`: flat spacing aliases,
  including their precedence relative to `spacing` fields.
- [x] `indent`: `left`, `right`, `first_line`, and `hanging` lengths.
- [x] `style`: paragraph-style reference; resolution of effective properties remains pending.
- [x] `border`: `top`, `right`, `bottom`, `left`, and `all`; each side accepts a
  style string or `style`/`val`, `size`/`width`, `space`, and `color` fields.
- [x] `fill`: paragraph shading color.
- [x] `tabs`: a tab-stop object or array of objects, with `position`, `alignment`,
  and optional `leader`; include the single-entry alignment/position shorthand.
- [x] `keep_with_next`: boolean pagination request recorded in the model.
- [x] `keep_together`: boolean pagination request recorded in the model.
- [x] `page_break_before`: boolean pagination request recorded in the model.
- [x] `widow_control`: boolean pagination request recorded in the model.
- [x] `outline_level`: non-negative outline level; document its output limits.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

Style references, colors, borders, tab-stop arrays, and pagination requests are
defined in the formatting chapter. Style declarations and presentation containers
remain future work. Tab stops and outline levels require explicit
output coverage; they are not established by the current model. Text direction
is not a general paragraph attribute in this checklist. Presentation container
`text` and `list` defaults belong to their future container checklists; they are
not additional direct paragraph controls here.

## Output

- [ ] Document attribute mappings and unsupported-property diagnostics for each
  applicable output and layout mode.
- [ ] Verify DOCX in Word and presentation output in PowerPoint where applicable;
  verify native PDF and PNG independently, including spacing and pagination.

## Evidence

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [evaluation tests](../../tests/evaluation.rs),
and [specification examples](../../tests/conformance.rs) cover the checked items.
