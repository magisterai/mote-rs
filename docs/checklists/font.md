# Font declaration checklist

[Checklist index](README.md)

## Declaration and attributes

- [x] Warn on numeric name conversion and retain extra declaration metadata,
  including an unflattened `properties` value, without claiming output support.
- [x] Named arguments only, no body, required `name`; produce no content node.
- [x] `name`: string or numeric identity; names are case-sensitive.
- [x] `latin`, `east_asian`, `complex_script`: nonempty typeface strings.
- [x] `word_line_height`: positive finite numeric metadata.
- [x] Document-wide registration; a later declaration replaces the whole
  declaration with the same name, including when declared in an evaluated body.
- [x] Retain declarations in the document model and JSON inspection.

## Resolution and output

- [x] Resolve an exact font alias to its declared script-specific typefaces and
  line-height metadata; retain an undeclared name as a direct typeface reference.
- [ ] `family`, `pitch`, and `files`: inventory their output mappings and font
  variant selection. These fields are currently retained as metadata only.
- [ ] Font-file selection, font-table metadata, embedding, installation lookup,
  glyph coverage, and substitution diagnostics. These require the font/output
  subsystem and are outside this declaration increment.
- [ ] Verify actual font selection in editable Office and native PDF/PNG output.

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Specification](../specification/styles-and-fonts.md),
[implementation](../../src/evaluation/declarations.rs),
[language tests](../../tests/declarations.rs), [resolver](../../src/formatting.rs),
[resolution tests](../../tests/formatting.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
