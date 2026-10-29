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
- [ ] `family`, `pitch`: effective font-table metadata. Retained as metadata only.
- [x] `files`: explicit `regular`, `bold`, `italic`, `bold_italic` asset references;
  select the exact requested variant, resolve relative to the source, cache its
  bytes and report substitutions without synthesizing a different face.

DOCX emits declared script mappings in runs and styles, and physical typeface
names in its font table. Missing mappings remain inherited. `word_line_height`
has no DOCX size effect. [Structural comparisons](../../tests/fixtures/docx/styles-and-fonts/README.md)
cover aliases and literal names. Native measurement resolves declared files,
retains their bytes, reports substitutions and missing glyphs, and measures
static TrueType/CFF variants. DOCX ignores `files`. Installed-font discovery,
embedding, script shaping and painted output remain unsupported.
Real Word font/layout review remains unreviewed.

## Specification inventory

The full relevant property inventory includes supported, partial, and unsupported
standard capabilities. XML names identify specification properties, not new Mote
arguments. Direct-attribute limits above still apply; shared types do not grant
extra attributes to this directive. See the [inventory guide](attributes/README.md).

- [Word and DrawingML font properties](attributes/fonts.md).
- [Word font selection properties](attributes/fonts.md#w-ct_fonts).

## Evidence

[Property-contract tests](../../tests/property_contracts.rs) cover retention,
signature checks, normalization, and declaration boundaries.

[Specification](../specification/styles-and-fonts.md),
[implementation](../../src/evaluation/declarations.rs),
[language tests](../../tests/declarations.rs), [resolver](../../src/formatting.rs),
[resolution tests](../../tests/formatting.rs), and
[specification examples](../../tests/conformance.rs) cover the checked items.

[Font-resolution tests](../../tests/font_resolution.rs),
[measurement tests](../../tests/measurement.rs), and
[asset-cache tests](../../tests/asset_cache.rs) cover declared files and native metrics.

## Native installed fonts

Native output can discover exact Aptos, Aptos Display, Arial and Courier New
faces from the supported local Office/system locations. Word's bundled fonts take
precedence over PowerPoint and system copies. Family and variant names, static
outline validity and embedding permissions are checked before selection; a missing
bold or italic face is never synthesized. Declared `files` remain authoritative.
`TextMeasurer` keeps discovery opt-in; `render::plan` enables it and
`render::plan_with_fonts` accepts an explicit resolver or no discovery.
See [installed-font tests](../../tests/installed_fonts.rs).
