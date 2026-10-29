# Captions

[Directive index](README.md) · [Standard inventory](attributes/anchors-and-fields.md)

- [x] One required inline body; named arguments only.
- [x] `position`: above or below; native figure order follows it. DOCX retains authored order.
- [x] `style` and `alignment`: paragraph formatting.
- [x] `numbering`: enabled, restart, start, id, label, prefix, format, separator; preserve unknown nested metadata.
- [x] Decimal, lower/upper alpha and lower/upper Roman numbering; shared sequence identity and restarts.
- [x] `properties`: retained overrides; standalone and figure-associated captions.

See [language semantics](../specification/figures-and-captions.md),
[model/package tests](../../tests/captions.rs), and
[native tests](../../tests/caption_rendering.rs). Office visual review is unreviewed.

DOCX emits editable SEQ fields and number bookmarks. Native body captions paint
numbered text. Captions in native cells and running stories omit numbering while
still contributing to reference metadata; visible numbers and references can
therefore differ. This limitation has a diagnostic and a frozen regression case.
