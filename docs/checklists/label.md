# Labels

[Directive index](README.md) · [Standard inventory](attributes/anchors-and-fields.md)

- [x] Required nonempty string name, positional or named; no body.
- [x] `kind`, `title`: strings; `number`: string or number.
- [x] `properties`: flattened overrides and retained metadata.
- [x] Attach standalone labels to the following block and infer its title without visible label text.

See [language semantics](../specification/anchors-and-references.md),
[model/package tests](../../tests/references.rs), and
[native tests](../../tests/reference_rendering.rs). Office visual review is unreviewed.

Native page destinations require placed body content. Labels in running stories,
mid-paragraph labels, trailing labels, and labels before lists or breaks retain
visible reference text but warn and omit unresolved links. Editable DOCX fields
retain their target identity; cached page values are updated by the Office reader.
