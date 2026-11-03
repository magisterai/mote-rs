# Strikethrough checklist

[Checklist index](README.md)

## Language and attributes

- [x] Quoted named-style shorthand, preserving the existing attribute and output
  rules; see [style application syntax](style.md#application-syntax).

- [x] One required inline body; no positional arguments.
- [x] kind: single or double; omission requests single.
- [x] style, font, size, color, and properties.
- [x] Markdown `~~text~~` lowers to a single-strike inline body; escaped and
  unmatched delimiters remain literal text.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/semantic-formatting.md),
[implementation tests](../../tests/semantic_formatting.rs), and
[frozen captures](../../tests/fixtures/semantic-formatting/README.md).

The full property inventories include unsupported capabilities:

- [Word run properties](attributes/word-runs.md).
- [DrawingML text properties](attributes/drawing-text.md).
- [Word extensions](attributes/word-extensions.md).

Output notes: Word maps underline attributes, highlight `fill` and strike `kind`.
Other direct wrapper style/font/size/color attributes have no Word mapping; use
surrounding spans. Native wave underlines use a shared sampled path; none suppresses the underline.

Markdown lowering is covered by [authoring content tests](../../tests/authoring_content.rs).
