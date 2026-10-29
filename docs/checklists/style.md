# Style declaration checklist

[Checklist index](README.md)

## Declaration and attributes

- [x] Retain extra flat and nested properties; distinguish signature validation
  from the selected color, length, boolean, spacing, and integer normalizers.
- [x] Accept `table` as a declaration type and normalize its property object:
  width, indent, cell spacing, alignment, layout, cell margins, text direction,
  wrapping, paragraph/text groups, edge regions, and row/column banding. This
  records declarations only; table application and output remain pending.
- [x] Warn on numeric name conversion; retain whitespace-only style identities
  and apply the final self-inheritance check to empty identities as well.
- [x] Named arguments only, no body, required `name`; produce no content node.
- [x] String or numeric name; paragraph type by default, explicit `paragraph`
  and `character` types, plus the table declaration type above; keep authored
  properties alongside the effective type.
- [x] `based_on`: parent style; `next` and `link`: retained relationships, not
  inheritance edges; `font`: font name or alias.
- [x] `paragraph`: existing paragraph geometry, spacing, borders, tabs, shading,
  pagination, and outline fields, plus `background` shading metadata.
- [x] `run`: font, size, color, highlight, fill/background, bold, italic,
  underline, small caps, strike, and double strike.
- [x] Flat run/paragraph properties and `first_line_indent`/`hanging_indent`
  metadata; nested paragraph/run fields override flat counterparts.
- [x] Replacement by name and document-wide availability, including forward
  references; retain declarations in the model and JSON inspection.

## Resolution

- [x] Reject direct self-inheritance after final declaration replacement.
- [x] Exact style lookup first, then deterministic matching after lowercasing
  and removing all characters except ASCII letters and digits.
- [x] Resolve `based_on` from base to derived, merging nested paragraph fields
  and preserving explicit false/zero; handle flat/nested spacing aliases.
- [x] Missing parents and cycles stop traversal while retaining discovered
  properties; expose these conditions in resolution results.
- [x] Apply named styles and direct overrides to inherited formatting without
  modifying authored attributes or leaking changes into siblings.
- [x] Resolve font aliases from the resulting run font reference.

## Support limits

- [ ] `table`: apply table styles and conditional regions to table content.
  Declaration storage is implemented; application requires the table model.

Output defaults, implicit style selection, measured fonts, and rendered style
application remain unsupported.

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Style and conditional table properties](attributes/styles.md).
- [Word paragraph properties](attributes/word-paragraphs.md).
- [Word run properties](attributes/word-runs.md).
- [Font properties](attributes/fonts.md).
- [DrawingML text defaults](attributes/drawing-text.md).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Specification](../specification/styles-and-fonts.md),
[implementation](../../src/evaluation/declarations.rs),
[language tests](../../tests/declarations.rs), [resolver](../../src/formatting.rs),
[resolution tests](../../tests/formatting.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.
