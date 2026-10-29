# Table

[Language](../specification/tables.md) · [Tests](../../tests/tables.rs)

[Directive index](README.md) · [Standard properties](attributes/tables.md)

- [x] One required body containing rows and separating whitespace; empty-table warning.
- [x] `style`: named table style, including inherited and conditional-region defaults.
- [x] `width`, `indent`, `cell_spacing`: nonnegative lengths.
- [x] `layout`: auto, autofit, fixed; `alignment`: left, center, right, justify.
- [x] `cell`: default margin, fill, alignment, vertical_alignment, text_direction, no_wrap and borders.
- [x] `borders`: top/right/bottom/left/inside_h/inside_v with style, size/width and color.
- [x] `properties`: flattened overrides; retained unknown fields do not imply output support.
- [x] Markdown pipe tables, delimiter alignment, bold repeating header cells.

The standard inventory includes floating placement, revision metadata, logical
edges, conditional formatting, table grid and all nested property/value domains.
Only the documented Mote mappings are in implementation scope. Output support
and Office visual review must be recorded separately.

Language/model and editable DOCX support are verified by captured packages.
Native table layout is the next implementation step; Microsoft Word visual
review remains unreviewed.
