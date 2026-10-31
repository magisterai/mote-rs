# Standalone checkboxes

[Directive index](README.md) · [Shared property inventory](attributes/controls.md)

- [x] No body or positional arguments.
- [x] `checked`: normalized boolean, default false.
- [x] `kind`: `content_control` (default) or `legacy_form_field`.
- [x] `name`: optional string/number, normalized to a string.
- [x] `title`: optional string/number, normalized to a string.
- [x] `properties`: shared overrides followed by normalization.
- [x] Lua `document.checkbox` constructor and JSON retention.

DOCX emits editable controls. Native document output uses `[x]`/`[ ]`;
presentation text omits standalone controls with a warning. A legacy title is
retained without an output effect.

Unsupported standard properties remain in the shared inventories; direct authoring
mappings are limited to the attributes listed here.

Evidence: [language definition](../specification/annotations.md),
[contract and output tests](../../tests/annotations.rs), and
[frozen captures](../../tests/fixtures/annotations/README.md).
