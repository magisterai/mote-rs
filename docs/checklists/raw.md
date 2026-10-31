# Literal block content checklist

[Checklist index](README.md)

## Language and attributes

- [x] Optional scalar text argument or inline body; text argument takes precedence.
- [x] language and style: string metadata and style reference.
- [x] preserve_space: normalized boolean; properties: shared attribute overrides.
- [x] `font`, `size`, and `color`: retained direct controls for native and presentation text; Word requires styles for these controls.
- [x] Fenced Markdown code retains literal contents and language labels.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/block-content.md),
[implementation tests](../../tests/block_content.rs), and
[frozen captures](../../tests/fixtures/block-content/README.md).

The full property inventories include unsupported capabilities:

- [Word run properties](attributes/word-runs.md).
- [DrawingML text properties](attributes/drawing-text.md).
- [Word extensions](attributes/word-extensions.md).

- [Word paragraph properties](attributes/word-paragraphs.md).
