# Box

[Directive index](README.md) · [Drawing text](attributes/drawing-text.md) ·
[Shapes and connectors](attributes/shapes-and-lines.md) ·
[Presentation properties](attributes/presentations.md)

- [x] Named arguments, block body and empty-box warnings.
- [x] `position`: x/y default to zero inches; width/height required; all four
  coordinates normalize non-negative lengths; preserve extra fields.
- [x] `name`: text or number; `style`: retained text style hint.
- [x] Text flow, images, tables and charts in any order. A box of several pieces
  is a stack down its frame: text at the height of its lines, an image at the
  height of its proportion and at most an equal share, tables and charts in
  equal shares of the remainder. A stack ignores `text.fit` with a warning, and
  warns when its pieces exceed the frame. See the
  [stack test](../../tests/presentation_rendering.rs) and the
  [stacked boxes case](../../tests/cases/powerpoint-33-stacked-boxes.mote).
- [x] `fill`, `border`: color, width, dash and enabled stroke fields.
- [x] `alignment`; `vertical_alignment`: top, center/middle, bottom.
- [x] `margin`: top/right/bottom/left; nested extra lengths remain normalized.
- [x] `text.fit`: clip/none, shrink/autofit, resize/resize_shape.
- [x] A warning where the frame cuts off the text: the lowest glyph outline lies
  below the frame, whether `text.fit` is clip or absent, or shrink at its least
  size. The warning names the box by its path and its `name`, in native output
  and in PPTX. See the [overflow tests](../../tests/text_overflow.rs).
- [x] `text.margin`, alignment, vertical_alignment, font and run formatting.
- [x] `text.paragraph`: paragraph formatting, spacing, indentation and line spacing.
- [x] `text.list`: spacing, indentation, markers and numbering properties.
- [x] Direct text color, size, bold, italic, underline, strike, double_strike,
  small_caps; extra metadata does not imply a rendering effect.
- [x] The run formatting of a box overrides the `Normal` style in its plain
  paragraphs, and reaches a table within the box beneath the table style and
  the cells. See the [box test](../../tests/presentation_rendering.rs).
- [x] `opacity`: fraction, percentage number or percentage string.
- [x] `shadow`: color, blur, distance, angle and opacity normalization.
- [ ] `shadow.enabled`: retained metadata; does not enable or suppress the effect.
  A zero shadow opacity makes the effect invisible.
- [x] `fit`: contain, cover, stretch for an image child; contain where omitted.
  A `width` or `height` of the image has no effect and gives a warning that
  names the box as the source of the size.
- [x] `slot`: nonempty named assignment, with optional position and body.
- [x] `kind`, `required`, `remove`: layout-only declaration fields; see the
  [layout inventory](layout.md).
- [x] `animate`: effects on the clock of the slide; see [animation](slide.md#animation).
  A stack is animated as its group.
- [x] `properties` overrides and Lua construction.

Output covers positioned text, image, table and chart boxes. PPTX writes a
stack as a group shape named after the box, holding a rectangle for a filled or
bordered frame and one object for each piece; native PDF and PNG draw the same
objects. The places of the pieces are fixed when the output is written, so that
text edited afterwards in PowerPoint does not move the pieces beneath it. A
layout slot of kind `text` rejects a stack. Built-in and custom named layout slots are supported. Text body properties, paragraph/run/list
fields, geometry, fills, strokes, effects, hyperlinks, accessibility and nonvisual
metadata remain in the shared inventories, including unsupported values.
Editable positioned text supports paragraph/run/list formatting, theme fonts and
colors, margins, alignment, clipping, fills, borders, shadows and links. Shrinking/resizing
uses the shared native measurement result.
Native PDF/PNG covers the same positioned-text subset, with source-relative font
assets and fixed geometry/raster captures. Text-box shadows are omitted with a
diagnostic; box geometry and text fitting are preserved. Zero-opacity shadows
produce no omission warning. PowerPoint review remains pending. See
[package captures](../../tests/fixtures/presentation-text/README.md).

The [language definition](../specification/presentations.md) and
[model tests](../../tests/presentations.rs) cover checked contracts.

The image-box output boundary includes the existing `position`, `fit`, image
source, crop, flips, rotation, effects, link and metadata contracts. A box owns
the image frame; direct image width/height, alignment and spacing do not
reposition it. An image border overrides the box border. Image-box fills, box opacity,
shadows and text properties have no picture effect; the `opacity` of the image
itself fades the picture. Background images have
[separate captures](../../tests/fixtures/presentation-backgrounds/README.md).
Editable PPTX and native PDF/PNG support this picture subset, including source-
relative assets, image effects, patterned translucent borders and rotated links.
Contain/cover uses the cropped source aspect ratio in editable and native output.
Resolved native picture frames retain page-coordinate precision. See the fixed
[picture captures](../../tests/fixtures/presentation-images/README.md) and
[native tests](../../tests/presentation_rendering.rs). Missing images become PPTX
placeholder text and are omitted with a diagnostic natively.

Positioned [tables](../../tests/fixtures/presentation-tables/README.md) and
[charts](../../tests/fixtures/presentation-charts/README.md) use the box rectangle
for layout. Their capture guides record style, sizing and native output limits.
A row of a table is never lower than its text, and takes what it needs beyond
an equal share from the other rows; a warning names the box where the table
cannot fit and passes the lower edge of the frame.

Loose chart objects use measured automatic frames: one column when it fits,
then a row-major grid. Each plot reserves at least 72 points in both dimensions
(96 for radial charts) after decorations. An impossible arrangement reports an
error; it does not add slides. Explicit boxes retain their authored dimensions.
See the [automatic-frame tests](../../tests/presentation_rendering.rs).
