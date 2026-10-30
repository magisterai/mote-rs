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
- [x] `slot`: nonempty named assignment, with optional position and body.
- [x] `kind`, `required`, `remove`: layout-only declaration fields; see the
  [layout inventory](layout.md).
- [x] `properties` overrides and Lua construction.

Output covers positioned text, image, table and chart boxes. Built-in and custom named layout slots are supported. Text body properties, paragraph/run/list
fields, geometry, fills, strokes, effects, hyperlinks, accessibility and nonvisual
metadata remain in the shared inventories, including unsupported values.
Editable positioned text supports paragraph/run/list formatting, theme fonts and
colors, margins, alignment, clipping, fills, borders and links. Shrinking/resizing
uses the shared native measurement result.
Native PDF/PNG covers the same positioned-text subset, with source-relative font
assets and fixed geometry/raster captures. Text-box shadows fail explicitly. PowerPoint review remains pending. See
[package captures](../../tests/fixtures/presentation-text/README.md).

The [language definition](../specification/presentations.md) and
[model tests](../../tests/presentations.rs) cover checked contracts.

The image-box output boundary includes the existing `position`, `fit`, image
source, crop, flips, rotation, effects, link and metadata contracts. A box owns
the image frame; direct image width/height, alignment and spacing do not
reposition it. An image border overrides the box border. Image-box fills, opacity,
shadows and text properties have no picture effect. Background images have
[separate captures](../../tests/fixtures/presentation-backgrounds/README.md).
Editable PPTX and native PDF/PNG support this picture subset, including source-
relative assets, image effects, patterned translucent borders and rotated links.
Cropped-image fitting differs between editable and native output; see the fixed
[picture captures](../../tests/fixtures/presentation-images/README.md) and
[native tests](../../tests/presentation_rendering.rs). Missing images become PPTX
placeholder text and are omitted with a diagnostic natively.

Positioned [tables](../../tests/fixtures/presentation-tables/README.md) and
[charts](../../tests/fixtures/presentation-charts/README.md) use the box rectangle
for layout. Their capture guides record style, sizing and native output limits.
