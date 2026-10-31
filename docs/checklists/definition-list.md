# Definition lists checklist

[Checklist index](README.md)

## Language and attributes

- [x] definition_list requires a body of term and definition elements.
- [x] style: string style hint; properties: shared attribute overrides.
- [x] Preserve child order and validate structural children.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/block-content.md),
[implementation tests](../../tests/block_content.rs), and
[frozen captures](../../tests/fixtures/block-content/README.md).

See the full [shared attribute inventories](attributes/README.md) for paragraph,
border, spacing, style and DrawingML text properties, including unsupported values.
