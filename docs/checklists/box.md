# Box

[Directive index](README.md) · [Drawing text](attributes/drawing-text.md) ·
[Shapes and connectors](attributes/shapes-and-lines.md) ·
[Presentation properties](attributes/presentations.md)

- [x] Named arguments, block body and empty-box warnings.
- [x] `position`: x/y default to zero inches; width/height required; all four
  coordinates normalize non-negative lengths; preserve extra fields.
- [x] `name`: text or number; `style`: retained text style hint.
- [x] Text flow, or exactly one image/table/chart child; ambiguous mixes rejected.
- [x] `fill`, `border`: color, width, dash and enabled stroke fields.
- [x] `alignment`; `vertical_alignment`: top, center/middle, bottom.
- [x] `margin`: top/right/bottom/left; nested extra lengths remain normalized.
- [x] `text.fit`: clip/none, shrink/autofit, resize/resize_shape.
- [x] `text.margin`, alignment, vertical_alignment, font and run formatting.
- [x] `text.paragraph`: paragraph formatting, spacing, indentation and line spacing.
- [x] `text.list`: spacing, indentation, markers and numbering properties.
- [x] Direct text color, size, bold, italic, underline, strike, double_strike,
  small_caps; extra metadata does not imply a rendering effect.
- [x] `opacity`: fraction, percentage number or percentage string.
- [x] `shadow`: enabled, color, blur, distance, angle and opacity.
- [x] `fit`: contain, cover, stretch for an image child.
- [ ] `slot`, `kind`, `required`, `remove`: named layout declarations/assignments.
- [x] `properties` overrides and Lua construction.

Initial output scope is positioned text. Image/table/chart boxes and named layout
slots remain separate steps. Text body properties, all paragraph/run/list fields,
geometry, fills, strokes, effects, hyperlinks, accessibility and nonvisual
metadata remain in the shared inventories, including unsupported values.
Editable positioned text supports paragraph/run/list formatting, theme fonts and
colors, margins, alignment, clipping, fills, borders and links. Shrinking/resizing uses the shared native measurement result.
Native PDF/PNG covers the same positioned-text subset, with source-relative font
assets and fixed geometry/raster captures. Shadows and non-text boxes fail
explicitly. PowerPoint review remains pending. See [package captures](../../tests/fixtures/presentation-text/README.md).

The [language definition](../specification/presentations.md) and
[model tests](../../tests/presentations.rs) cover checked contracts.

The image-box output boundary includes the existing `position`, `fit`, image
source, crop, flips, rotation, effects, link and metadata contracts. A box owns
the image frame; image block dimensions, alignment and spacing do not reposition
it. An image border overrides the box border. Image-box fills, opacity, shadows
and text properties have no picture effect. Background images remain separate.
These contracts are inventoried before implementation; image output is pending.
