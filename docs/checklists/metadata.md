# Document metadata

[Directive index](README.md) · [Shared property inventory](attributes/metadata.md)

- [x] Named values only; no body or positional arguments.
- [x] Repeated declarations replace same-named values and preserve other values.
- [x] `title`: retained value, mapped to DOCX and PPTX core properties.
- [x] `author`: retained value, mapped to DOCX core properties.
- [x] `subject`, `keywords`, `description`, `category`, `company` and extra
  named values: model retention only, without an output mapping.
- [x] Lua `document.metadata` declaration and JSON retention.

DOCX maps title and author; PPTX maps title. Other fields are retained without
package or native rendering effects.

Unsupported standard properties remain in the shared inventories; direct authoring
mappings are limited to the attributes listed here.

Evidence: [language definition](../specification/annotations.md),
[contract and output tests](../../tests/annotations.rs), and
[frozen captures](../../tests/fixtures/annotations/README.md).
