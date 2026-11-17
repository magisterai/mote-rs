# Paragraph checklist

[Checklist index](README.md) · [Specification](../specification/basic-content.md)

## Content and arguments

- [x] Quoted named-style shorthand, preserving the existing attribute and output
  rules; see [style application syntax](style.md#application-syntax).

- [x] One required inline body, including an empty body; no positional arguments.
- [x] Implicit paragraphs, whitespace handling, block boundaries, and rejection of
  nested blocks in an explicit paragraph.
- [x] Indented body lines: a line ending and its surrounding indentation supply
  one space, and nothing at a body edge, unless space is preserved; see the
  [syntax chapter](../specification/syntax-and-values.md) and the
  [content tests](../../tests/content.rs).
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
- [x] `style`: paragraph-style reference; explicit resolution is covered by the
  [style checklist](style.md).
- [x] `border`: `top`, `right`, `bottom`, `left`, and `all`; each side accepts a
  style string or `style`/`val`, `size`/`width`, `space`, and `color` fields.
  The outer argument's object type and the `space` of each side are checked; other
  side contents remain metadata.
- [x] `border` side `space`: a length or a bare number of points from 0 to 31,
  kept as whole points; the native shading of a paragraph with a rule on either
  side reaches those rules across it, as Word's does. See the
  [border tests](../../tests/paragraph_borders.rs).
- [x] Consecutive paragraphs with identical borders and identical left and
  right indents share one box, as in Word: no rule and no border space lies
  between them, the side rules cross the space between them, and the shading of
  the upper paragraph crosses the space after it. Word joins the paragraphs of
  the DOCX itself; native layout joins those of one flow, and leaves the box open
  where a page divides it. See the
  [border tests](../../tests/paragraph_borders.rs) and the
  [paragraph-borders probe](../../tests/office/probes/paragraph-borders.mote).
- [x] `fill`: paragraph shading color.
- [x] `tabs`: a tab-stop object or array of objects, with `position`, `alignment`,
  and optional `leader`; include the single-entry alignment/position shorthand.
  Only the outer collection type is checked; stop contents remain metadata.
- [x] `keep_with_next`: boolean pagination request recorded in the model. DOCX
  writes `w:keepNext`. Native layout keeps the paragraph on the page of the
  start of the block after it: the first lines of a paragraph, the first row of
  a table, or the whole of a chart or a picture. See the
  [pagination tests](../../tests/figure_pagination.rs) and the
  [keep-with-next-blocks probe](../../tests/office/probes/keep-with-next-blocks.mote).
- [x] `keep_together`: boolean pagination request recorded in the model.
- [x] `page_break_before`: boolean pagination request recorded in the model.
- [x] `widow_control`: boolean pagination request recorded in the model.
- [x] `outline_level`: non-negative outline level; document its output limits.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

Style references, colors, borders, tab-stop arrays, and pagination requests are
defined in the formatting chapter. Style declarations have their own checklist.
Tab stops and outline levels require output coverage beyond model retention.
Text direction is not a general paragraph attribute here. Presentation container
`text` and `list` defaults belong to the [box checklist](box.md); they are not
additional direct paragraph controls.

## Output limits

DOCX maps alignment, spacing and its aliases, indentation, pagination flags,
outline levels 0–9, shading, borders, tab stops and named paragraph styles.
[Captured structural comparisons](../../tests/fixtures/docx/paragraph-formatting/README.md)
cover these mappings; page layout and real Word review remain unreviewed.
Native PDF/PNG implements paragraph measurement and pagination; its painting
limits and captures are described below. Custom tab stops, leaders and Word's
half-inch default stops control native placement; see the
[tab tests](../../tests/tabs.rs).

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word paragraph properties](attributes/word-paragraphs.md).
- [DrawingML paragraph properties](attributes/drawing-text.md#a-ct_textparagraphproperties).
- [Word paragraph extensions](attributes/word-extensions.md).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [evaluation tests](../../tests/evaluation.rs),
and [specification examples](../../tests/conformance.rs) cover the checked items.

[Empty-property captures](../../tests/fixtures/docx/empty-paragraph-properties/README.md)
cover border/tab alias fallback, omitted empty leaders and explicit empty sides
without rewriting authored metadata.

## Native output evidence

PDF and PNG use the shared block plan, including list and heading numbering,
quote text, and paragraph fills. Native paragraph borders paint each resolved
side as a solid rule outside the text, with the width and colour defaults of the
DOCX writer (0.5 points, `BFBFBF`). A side rule lies its `space` and a further
1.5 points from the text, as Word's does at every width and space measured. A
top or bottom border and its spacing add to the paragraph's height, as in Word.
The shading of a paragraph crosses the space after it only to a paragraph that
shares its borders, or from one unbordered paragraph to another. Word sets the
shading of a paragraph without a rule on either side, and its top and bottom
rules, 1.5 points beyond the text on each side, and begins the shading of a
first line at the left indent; native output sets all three at the text edge of
the line. Styles other than
single and thick are drawn as a single line and reported; borders inside table
cells are not painted. See the [native block tests](../../tests/native_blocks.rs).
List item child blocks and quote child blocks flatten to inline text; nested lists
inside list items retain their own markers. See the
[native block captures](../../tests/fixtures/native-blocks/README.md) and
[block tests](../../tests/native_blocks.rs). Office visual review remains unreviewed.
