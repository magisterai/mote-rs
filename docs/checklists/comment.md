# Comments

[Directive index](README.md) · [Shared property inventory](attributes/comments.md)

- [x] Required inline or block content body; no positional arguments.
- [x] `id`: optional string/number metadata, independent of package identities.
- [x] `author`: optional string; absent or empty authors use `Mote` in DOCX.
- [x] `initials`: optional string.
- [x] `date`: valid ISO calendar date string or typed date.
- [x] `properties`: shared overrides followed by normalization.
- [x] Lua `document.comment` constructor and JSON retention.

DOCX emits comment ranges and a comments story; package identifiers are allocated
independently of authored `id`. Nested or running-story annotations flatten with
warnings. Native document and presentation output retain body text without
annotation geometry.

Unsupported standard properties remain in the shared inventories; direct authoring
mappings are limited to the attributes listed here.

Evidence: [language definition](../specification/annotations.md),
[contract and output tests](../../tests/annotations.rs), and
[frozen captures](../../tests/fixtures/annotations/README.md).
