# Bookmarks

[Directive index](README.md) · [Standard inventory](attributes/anchors-and-fields.md)

- [x] Required string name, positional or named; optional inline body.
- [x] `kind`, `title`: string metadata.
- [x] Editable bookmark range, stable destination identity and preservation of enclosed text.

See [language semantics](../specification/anchors-and-references.md),
[model/package tests](../../tests/references.rs), and
[native tests](../../tests/reference_rendering.rs). Office visual review is unreviewed.

Native page destinations require placed body content. Labels in running stories,
mid-paragraph labels, trailing labels, and labels before lists or breaks retain
visible reference text but warn and omit unresolved links. Editable DOCX fields
retain their target identity; cached page values are updated by the Office reader.
