# Shape

[Directive index](README.md) · [Drawing property inventory](attributes/shapes-and-lines.md)

Inventory precedes implementation. Shape properties inherit the complete
[DrawingML text](attributes/drawing-text.md), [image/placement](attributes/images.md),
and [value-domain](attributes/values.md) inventories. An unchecked standard
property has no implied Mote argument or output support.

- [ ] Named arguments only; optional block body; no positional arguments.
- [ ] `position` required: `width`, `height`; `x`, `y` default to zero inches.
  All coordinates and dimensions use the existing length conversions.
- [ ] `name`: string or number, normalized to a string.
- [ ] `kind`: rectangle (`rect` alias), round_rect (`rounded`), ellipse (`oval`),
  triangle, diamond, hexagon, chevron, cloud, parallelogram, trapezoid.
- [ ] `fill`: normalized color.
- [ ] `border`: width, color, style (single/solid/dashed/dotted/none), opacity;
  unknown nested metadata retained without inferred output effects.
- [ ] `rotation`: number/string, with an optional `deg` suffix.
- [ ] `flip_horizontal`, `flip_vertical`: boolean values.
- [ ] `alignment`: left/center/right/justify; `vertical_alignment`: top/center/bottom.
- [ ] `margin`: named length fields.
- [ ] `opacity`: number or percentage text; retained as normalized text.
- [ ] `shadow`: color, blur, distance, opacity, angle; retained additional metadata.
- [ ] `text`: fit (clip/none, shrink/autofit, resize/resize_shape), margin,
  alignment, vertical_alignment, font, span formatting, paragraph and list defaults.
- [ ] `text.paragraph`: the established paragraph-style property family;
  text/list spacing_before, spacing_after, line_spacing, spacing and indent.
- [ ] Shared direct font, size, color, highlight, bold, italic, underline,
  small_caps, strike and double_strike; `properties` override rules.
- [ ] Optional body contains significant block content, including text paragraphs.
- [ ] Lua `document.shape` uses the same validation and typed attributes.

Output boundaries to verify: absolute page anchoring and editable Word shape text;
editable slide shape geometry; native path geometry, preset text rectangles,
transforms, clipping, fills, borders, opacity, shadows and text. These support notes
do not claim an Office render review or authorize unsupported standard geometry.
