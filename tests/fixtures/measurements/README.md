# Font measurements

This frozen collection defines reproducible measurements for the first native
text-layout work. It includes the font bytes, requests, expected results and
fingerprints needed for offline comparisons. The [native measurement tests](../../measurement.rs)
check every case through the Rust measurer. The collection introduces no language syntax.

## Coverage

The 960 cases combine:

- Helvetica, Times and Courier core metrics, plus Source Sans 3 in TrueType and
  OpenType/CFF form: 20 font configurations across regular, bold, italic and
  bold-italic variants.
- Sizes of 9.5, 12, 12.5078125 and 24 points. The third size lies halfway between
  two 1/64-point values and exercises size quantization.
- Empty text, kerning pairs (`AV To Wa`), ligature letters (`office ffi`), accents
  and punctuation (`café € —…`), missing glyphs (`A☃字☃B`), and preserved spaces
  (`  A  B `).
- Layout kerning enabled and disabled. Painting widths retain kerning in both
  cases; core metrics do not apply kerning.

These cases characterize character advances and pair kerning. The ligature
letters do not establish ligature substitution or shaping support. Font fallback,
collections, variable fonts, script shaping, line breaking, paragraph layout,
pagination and painted output remain outside this collection.

## Files and format

| File | Contents |
| --- | --- |
| [requests.json](requests.json) | Font identities and one request per case. File fonts include a relative path, face index, SHA-256, byte length and source URL. |
| [expected.json](expected.json) | Frozen measurements, keyed by case identifier, with the request file's SHA-256. |
| [capture.json](capture.json) | Schema version, file fingerprints, case count, units, tolerance and review status. |
| [fonts/](fonts/) | Eight unmodified font files and their [license](fonts/LICENSE.md). |

All JSON documents use schema version 1. Each request supplies `id`, `font_id`,
`size_points`, `layout_kerning` and `text`. A candidate result uses the same
structure as `expected.json`: `schema_version`, `requests_sha256` and a
`measurements` array. Case order is immaterial; identifiers must be unique and
cover exactly the requested cases.

Each measurement contains:

| Field | Meaning |
| --- | --- |
| `id`, `font_id`, `variant` | Exact case and selected font identity. Variants are `regular`, `bold`, `italic` and `bold_italic`. |
| `metric_source` | `core` for standard PDF font metrics, or `declared` for the specified local font file. |
| `prepared_text` | Text after replacing each unavailable character with `?`; whitespace is preserved. |
| `missing_codepoints` | Unique missing Unicode scalar values, as integers in first-occurrence order. |
| `layout_width` | Advance used for layout, honoring the request's kerning flag. |
| `paint_width` | Advance with pair kerning. |
| `ascent`, `descent`, `height` | Font extents in points; height includes the font's line gap and is not an authored paragraph line height. |

All five metrics are measured in points, with 72 points per inch. Empty text has
zero widths but retains the font extents. Results must contain finite JSON
numbers, not numeric strings. Boolean values, duplicate fields, unknown fields,
missing cases and duplicate identifiers are errors or differences.

## Captured metric behavior

File-backed glyph advances and extents use 26.6 fixed-point values, expressed as
multiples of 1/64 point. Font size is rounded to that scale before glyph advances
are scaled and rounded individually. Extents use the `hhea` ascent, descent and
line gap; a library's higher-level preferred metrics may select different tables.
Core metrics instead scale integer widths in thousandths of an em without that
26.6 quantization and use their standard WinAnsi character repertoire.

The capture preserves a kerning-scale difference that must stay visible during
implementation: TrueType pair adjustments use `units_per_em / 64` points as the
scale, independent of the requested size, while CFF pair adjustments use the
quantized requested size. At 12 points, regular Source Sans 3 measures
`AV To Wa` as 44.453125 points in TrueType and 44.796875 points in CFF. Both
have an unkerned layout width of 45.953125 points. Do not silently replace these
expectations with a library's defaults or describe the difference as equivalent
geometry. Any intentional behavior change needs separate review.

## Comparison

Verify fixture integrity without a renderer or installed fonts:

```sh
python3 tools/compare_measurements.py --fixture tests/fixtures/measurements
```

Produce and compare candidate measurements:

```sh
cargo run --example measure_fonts -- \
  tests/fixtures/measurements target/measurements-candidate.json
python3 tools/compare_measurements.py \
  --fixture tests/fixtures/measurements \
  --actual target/measurements-candidate.json \
  --output target/measurements/review-001
```

The output directory must be new. `fixture_verified` means that fixture structure,
coverage and fingerprints passed; it says nothing about a candidate measurer.
`matched` requires all candidate fields to agree, with an absolute tolerance of
0.000000001 point for metrics only. That tolerance permits numeric serialization
noise, not a different rounding policy. All other fields compare exactly.
Differences and invalid inputs return nonzero and appear in `report.json` when
an output directory is supplied. Inputs are checked again after comparison.
The candidate example reads requests, fonts and the capture's request identity;
run fixture verification first. It never reads expected measurements.

The command never regenerates expectations or downloads fonts. Review intentional
baseline changes with their measurements and fingerprints together. These captures
are **unreviewed in Microsoft Word**; matching them does not establish visual,
wrapping or pagination conformance.

## Font sources

The files are from Adobe's [Source Sans 3 release 3.052R](https://github.com/adobe-fonts/source-sans/releases/tag/3.052R).
Each descriptor records the exact versioned download URL. The original
[SIL Open Font License](https://raw.githubusercontent.com/adobe-fonts/source-sans/3.052R/LICENSE.md)
text is retained beside the font files, with line endings and trailing whitespace
normalized. The eight font files total 2,655,116 bytes. No installed or proprietary
font is needed to run these comparisons.
