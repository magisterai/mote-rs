# Box

[Directive index](README.md) · [Drawing text](attributes/drawing-text.md) ·
[Shapes and connectors](attributes/shapes-and-lines.md) ·
[Presentation properties](attributes/presentations.md)

- [ ] Named arguments, block body and empty-box warnings.
- [ ] `position`: x/y default to zero inches; width/height required; all four
  coordinates normalize non-negative lengths; preserve extra fields.
- [ ] `name`: text or number; `style`: retained text style hint.
- [ ] Text flow, or exactly one image/table/chart child; ambiguous mixes rejected.
- [ ] `fill`, `border`: color, width, dash and enabled stroke fields.
- [ ] `alignment`; `vertical_alignment`: top, center/middle, bottom.
- [ ] `margin`: top/right/bottom/left; nested extra lengths remain normalized.
- [ ] `text.fit`: clip/none, shrink/autofit, resize/resize_shape.
- [ ] `text.margin`, alignment, vertical_alignment, font and run formatting.
- [ ] `text.paragraph`: paragraph formatting, spacing, indentation and line spacing.
- [ ] `text.list`: spacing, indentation, markers and numbering properties.
- [ ] Direct text color, size, bold, italic, underline, strike, double_strike,
  small_caps; extra metadata does not imply a rendering effect.
- [ ] `opacity`: fraction, percentage number or percentage string.
- [ ] `shadow`: enabled, color, blur, distance, angle and opacity.
- [ ] `fit`: contain, cover, stretch for an image child.
- [ ] `slot`, `kind`, `required`, `remove`: named layout declarations/assignments.
- [ ] `properties` overrides and Lua construction.

Initial output scope is positioned text. Image/table/chart boxes and named layout
slots remain separate steps. Text body properties, all paragraph/run/list fields,
geometry, fills, strokes, effects, hyperlinks, accessibility and nonvisual
metadata remain in the shared inventories, including unsupported values.
Native and editable output support and PowerPoint review remain pending.
