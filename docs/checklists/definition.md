# Definitions checklist

[Checklist index](README.md)

## Language and attributes

- [x] Quoted named-style shorthand, preserving the existing attribute and output
  rules; see [style application syntax](style.md#application-syntax).

- [x] One required content body; no positional arguments.
- [x] style: string paragraph-style reference; properties: shared attribute overrides.

## Scope and evidence

These entries inventory the approved language boundary. Output support is separate
from attribute retention; unsupported output properties must report diagnostics.
Office visual review remains pending.

Evidence: [language definition](../specification/block-content.md),
[implementation tests](../../tests/block_content.rs), and
[frozen captures](../../tests/fixtures/block-content/README.md).

See the full [shared attribute inventories](attributes/README.md) for paragraph,
border, spacing, style and DrawingML text properties, including unsupported values.

Within table cells, supported definition paragraph boundaries remain
separate in editable and native output. See the [authoring content
captures](../../tests/fixtures/authoring-content/README.md).
