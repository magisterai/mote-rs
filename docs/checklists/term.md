# Definition terms checklist

[Checklist index](README.md)

## Language and attributes

- [x] One required inline body; no positional arguments.
- [x] style: retained paragraph-style reference, with no dedicated outer type check; properties: shared attribute overrides.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/block-content.md),
[implementation tests](../../tests/block_content.rs), and
[frozen captures](../../tests/fixtures/block-content/README.md).

See the full [shared attribute inventories](attributes/README.md) for paragraph,
border, spacing, style and DrawingML text properties, including unsupported values.
