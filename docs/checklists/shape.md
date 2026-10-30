# Shape

[Directive index](README.md) · [Drawing property inventory](attributes/shapes-and-lines.md)

The [language model](../specification/drawings.md) is covered by
[normalization and capture tests](../../tests/drawings.rs). Output remains pending.
 Shape properties inherit the complete
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

Output boundaries to verify: absolute page anchoring and editable Word shape text;
editable slide shape geometry; native path geometry, preset text rectangles,
transforms, clipping, fills, borders, opacity, shadows and text. These support notes
do not claim an Office render review or authorize unsupported standard geometry.
