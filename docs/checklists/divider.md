# Horizontal dividers checklist

[Checklist index](README.md)

## Language and attributes

- [x] No content body or positional arguments.
- [x] style: output style hint; color: normalized hexadecimal color.
- [x] width: normalized length; spacing.before and spacing.after: lengths.
- [x] properties: shared attribute overrides.
- [x] Markdown hyphen, asterisk and underscore rules; setext-heading precedence.
- [x] Document rule painting and presentation boundary semantics.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/block-content.md),
[implementation tests](../../tests/block_content.rs), and
[frozen captures](../../tests/fixtures/block-content/README.md).

See the full [shared attribute inventories](attributes/README.md) for paragraph,
border, spacing, style and DrawingML text properties, including unsupported values.
