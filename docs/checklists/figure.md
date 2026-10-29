# Figures

[Directive index](README.md) · [Standard inventory](attributes/anchors-and-fields.md)

- [x] One required block body; named arguments only; at most one direct caption.
- [x] `kind`: metadata and default caption-numbering label.
- [x] `alignment`: default alignment for compatible children that do not override it.
- [ ] `style`: retained metadata; no figure-level style effect.
- [ ] `placement`: normalized and retained; DOCX uses `align` as an alignment fallback, while native layout ignores this object. Floating figure geometry is unsupported.
- [x] `numbering`: shared caption-numbering fields, inherited by associated captions.
- [x] `properties`: retained overrides; associate labels with caption title and number.

The placement object inventories `width`, `height`, `x`, `y`, `margin`, `align`,
`type`, `float`, `wrap`, `anchor`, `allow_overlap`, `behind_text`, `layout_in_cell`,
`locked`, `relative_height`, and horizontal/vertical `relative_to`, `alignment`,
and `offset`. Normalization and retention do not establish a layout effect.

See [language semantics](../specification/figures-and-captions.md),
[model/package tests](../../tests/captions.rs), and
[native tests](../../tests/caption_rendering.rs). Office visual review is unreviewed.

Native figures honor caption position; DOCX preserves authored child order.
Place the caption on the intended side and use direct `alignment` for consistent
output ordering and alignment. See the caption checklist for numbering limits.
