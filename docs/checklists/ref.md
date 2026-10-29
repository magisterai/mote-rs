# Cross-references

[Directive index](README.md) · [Standard inventory](attributes/anchors-and-fields.md)

- [x] No body; positional string target or named string/object target.
- [x] `target`: string or object containing label/name.
- [x] `kind`: title, number, page, text; `display`: auto, title, number, page, text.
- [x] `fallback`: string overriding the inferred visible text; `properties`: retained overrides.
- [x] Markdown internal links lower to references; forward destinations resolve after collecting anchors.
- [x] Page references participate in pagination until values and layout stabilize.

See [language semantics](../specification/anchors-and-references.md),
[model/package tests](../../tests/references.rs), and
[native tests](../../tests/reference_rendering.rs). Office visual review is unreviewed.

Native page destinations require placed body content. Labels in running stories,
mid-paragraph labels, trailing labels, and labels before lists or breaks retain
visible reference text but warn and omit unresolved links. Editable DOCX fields
retain their target identity; cached page values are updated by the Office reader.
