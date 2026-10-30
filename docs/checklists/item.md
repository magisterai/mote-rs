# Item checklist

[Checklist index](README.md) · [Specification](../specification/lists.md)

## Content and arguments

- [x] One required block body, including an empty body; no positional arguments.
- [x] Bare text becomes paragraphs in explicit items; Markdown items retain
  direct inline content and their source markers.
- [x] Markdown `[ ]`, `[x]` and `[X]` markers map to the existing task state.
- [x] Nested lists belong to item bodies; standalone adjacent items group by kind.

## Attributes

Checked items cover evaluation, validation, and model storage as defined in
[Formatting attributes](../specification/formatting-attributes.md). Effective
style resolution and output remain separate.

- [x] `marker`: string source/grouping metadata, including the ordered/unordered
  distinction. Custom marker appearance is not covered by this check.
- [x] `level`: positive item nesting level.
- [x] `tight`: request zero after-spacing.
- [x] `indent.left`, `indent.hanging`, and `indent.level`: item layout requests, retained independently
  of the containing list until layout resolves their precedence.
- [x] `spacing.after`: item after-spacing request, retained on the item.
- [x] `spacing.before` and `spacing.line`: shared spacing values for presentation
  layout, with output-specific limits.
- [x] `checked`: boolean task-item state; define absence separately from false.
- [x] `checkbox.checked`: retained nested state; the item-level `checked` value
  controls the task marker and overrides this nested value. Nested state alone
  must not create a task item.
- [x] `checkbox.kind`: `content_control` or `legacy_form_field` for DOCX controls.
- [x] `checkbox.name`: control/form-field name.
- [x] `checkbox.title`: content-control title; not a legacy-form-field property.
- [x] `style`: item paragraph style where native output applies it; specify the
  supported output scope explicitly.
- [x] `properties`: the shared attribute-object mechanism in the index.

## Dependencies and limits

Task-item state and checkbox metadata are defined before their examples in the
formatting chapter. Editable Word
controls and a native checkmark drawing are distinct output behaviours. A custom
bullet marker has presentation-specific scope; arbitrary ordered marker templates
and marker font/alignment controls are not goals here.

## Output limits

DOCX bullet and ordered items support levels, geometry overrides, inline formatting and nested
lists. Item left indentation is absolute; list indentation supplies the level base.
Paragraph children flatten into runs in one item paragraph, with nested lists
emitted afterward. Formatting on those child paragraphs remains unsupported.
Task items emit Word 2010 checkbox content controls by default or legacy form
fields. Item `checked` overrides nested state; nested metadata alone does not
create a task. Tasks use direct list geometry without consuming numbering.
Modern controls support name/title; legacy fields support names, with generated
names when absent. Legacy titles are unsupported. See the
[task capture](../../tests/fixtures/docx/task-items/README.md),
[control tests](../../tests/docx_controls.rs), [list output tests](../../tests/docx_lists.rs) and the
[bullet capture](../../tests/fixtures/docx/bullet-lists/README.md). Word rendering
remains unreviewed.

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word numbering and marker properties](attributes/numbering.md).
- [Word paragraph properties](attributes/word-paragraphs.md).
- [DrawingML paragraphs and bullets](attributes/drawing-text.md#a-ct_textparagraphproperties).
- [Task controls](attributes/controls.md).
- [Extended checkbox properties](attributes/word-extensions.md#w14-ct_sdtcheckbox).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Attribute implementation](../../src/evaluation/attributes.rs),
[attribute tests](../../tests/attributes.rs), [evaluator](../../src/evaluation.rs), [list tests](../../tests/content.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.

## Native output evidence

PDF and PNG use the shared block plan, including list and heading numbering,
quote text, and paragraph fills. Native paragraph borders currently use the
resolved `all.color` or `bottom.color` for a 0.35-point bottom rule on each line;
other sides, widths, and border styles are outside that painting behavior.
List item child blocks and quote child blocks flatten to inline text; nested lists
inside list items retain their own markers. See the
[native block captures](../../tests/fixtures/native-blocks/README.md) and
[block tests](../../tests/native_blocks.rs). Office visual review remains unreviewed.
