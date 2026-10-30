# Shape

[Directive index](README.md) · [Drawing property inventory](attributes/shapes-and-lines.md)

The [language model](../specification/drawings.md) is covered by
[normalization and capture tests](../../tests/drawings.rs). Editable DOCX shape output is covered by
[complete package captures](../../tests/shape_captures.rs). Native preset geometry is covered by
[path and raster captures](../../tests/shape_geometry.rs), and native text by
[text geometry and raster captures](../../tests/shape_text.rs). Shape properties inherit the complete
[DrawingML text](attributes/drawing-text.md), [image/placement](attributes/images.md),
and [value-domain](attributes/values.md) inventories. An unchecked standard
property has no implied Mote argument or output support.

- [x] Named arguments only; optional block body; no positional arguments.
- [x] `position` required: `width`, `height`; `x`, `y` default to zero inches.
  All coordinates and dimensions use the existing length conversions.
- [x] `name`: string or number, normalized to a string.
- [x] `kind`: rectangle (`rect` alias), round_rect (`rounded`), ellipse (`oval`),
  triangle, diamond, hexagon, chevron, cloud, parallelogram, trapezoid.
- [x] `fill`: normalized color.
- [x] `border`: width, color, style (single/solid/dashed/dotted/none), opacity;
  unknown nested metadata retained without inferred output effects.
- [x] `rotation`: number/string, with an optional `deg` suffix.
- [x] `flip_horizontal`, `flip_vertical`: boolean values.
- [x] `alignment`: left/center/right/justify; `vertical_alignment`: top/center/bottom.
- [x] `margin`: named length fields.
- [x] `opacity`: number or percentage text; retained as normalized text.
- [x] `shadow`: color, blur, distance, opacity, angle; retained additional metadata.
- [x] `text`: fit (clip/none, shrink/autofit, resize/resize_shape), margin,
  alignment, vertical_alignment, font, span formatting, paragraph and list defaults.
- [x] `text.paragraph`: the established paragraph-style property family;
  text/list spacing_before, spacing_after, line_spacing, spacing and indent.
- [x] Shared direct font, size, color, highlight, bold, italic, underline,
  small_caps, strike and double_strike; `properties` override rules.
- [x] Optional body contains significant block content, including text paragraphs.
- [x] Lua `document.shape` uses the same validation and typed attributes.

DOCX emits absolute page anchors, editable Word shape text, preset geometry,
transforms, fills, borders, opacity, shadows, text margins and fit settings.
Native document output paints preset paths, transforms, fills, borders, opacity
and shadows with one-point anchor flow. Shape text supports preset-specific text
bounds, nested text defaults, paragraph and inline formatting, lists and task
markers, margins, alignment, clipping, shrink-to-fit, text-height growth, and
rotation. Text-height growth extends the content clip without enlarging the
painted shape outline. Text stays readable through shape flips.

Tables, images, nested drawings, figure/caption blocks and page breaks inside
shape text are omitted with warnings. Field, note and anchor semantics contribute
only their child text. Paragraph pagination properties have no drawing-text effect.
Rotated PDF hyperlinks retain unrotated hit areas and warn about that limit.
Table-cell shapes and presentation output remain pending. Microsoft Word
render review is unreviewed; matching package captures do not establish visual
Office conformance. Unknown metadata has no implied output effect.
