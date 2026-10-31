# General fields

[Directive index](README.md) · [Shared property inventory](attributes/anchors-and-fields.md)

- [x] `name`: one required positional or named string; supplies cached text.
- [x] `code`: optional string; the uppercase name supplies the instruction when
  code is absent or empty.
- [x] `properties`: shared overrides followed by normalization.
- [x] No content body.
- [x] Lua `document.field` constructor and JSON retention.

DOCX emits a field instruction and cached name text. Native document and
presentation output omit dynamic field content with a warning; field evaluation
is not implied.

Unsupported standard properties remain in the shared inventories; direct authoring
mappings are limited to the attributes listed here.

Evidence: [language definition](../specification/annotations.md),
[contract and output tests](../../tests/annotations.rs), and
[frozen captures](../../tests/fixtures/annotations/README.md).
