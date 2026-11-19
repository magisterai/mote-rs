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
- [x] `position` required: `width`, `height`; optional `x`, `y`, of which one
  stated alone takes zero inches for the other. All coordinates and dimensions
  use the existing length conversions.
- [x] In a document, a shape whose position states `x` or `y` is fixed on its
  page, and a shape whose position states neither is set in the text and takes
  the room of its frame. A shape in a table cell is fixed in either case.
- [x] `name`: string or number, normalized to a string.
- [x] `kind`: rectangle (`rect` alias), round_rect (`rounded`), ellipse (`oval`),
  triangle, diamond, hexagon, chevron, cloud, parallelogram, trapezoid.
- [x] `corner_radius`: nonnegative length; the corner of a round_rect, at most
  half its shorter side; a warning on another kind. DOCX and PPTX write the
  `adj` guide of the preset; native output draws the same arc.
- [x] `fill`: normalized color.
- [x] `border`: width, color, style (single/solid/dashed/dotted/none), opacity;
  unknown nested metadata retained without inferred output effects. A dashed
  stroke is four widths on and three off, which is the `dash` preset of
  DrawingML; Word's export of the
  [editable shapes case](../../tests/cases/word-42-editable-drawingml-shapes.mote)
  and PowerPoint's of the
  [shapes and lines case](../../tests/cases/powerpoint-07-shapes-lines.mote)
  show it, and the dashes begin where the definition of the preset begins its
  outline.
- [x] `rotation`: number/string, with an optional `deg` suffix.
- [x] `flip_horizontal`, `flip_vertical`: boolean values.
- [x] `alignment`: left/center/right/justify; `vertical_alignment`: top/center/bottom.
- [x] `margin`: named length fields.
- [x] `opacity`: number or percentage text; retained as normalized text.
- [x] `shadow`: color, blur, distance, opacity, angle; retained additional metadata.
- [x] `text`: fit (clip/none, shrink/autofit, resize/resize_shape), margin,
  alignment, vertical_alignment, font, span formatting, paragraph and list defaults.
- [x] A warning where the frame cuts off the text: the lowest glyph outline lies
  below the text area, whether `text.fit` is clip or absent, or shrink at its
  least size. Native document output names the shape and its page; native slide
  output and PPTX name the shape by its path and its `name`. DOCX output
  measures no text and gives no such warning. See the
  [overflow tests](../../tests/text_overflow.rs).
- [x] `text.paragraph`: the established paragraph-style property family;
  text/list spacing_before, spacing_after, line_spacing, spacing and indent.
- [x] Shared direct font, size, color, highlight, bold, italic, underline,
  small_caps, strike and double_strike; `properties` override rules.
- [x] Optional body contains significant block content, including text paragraphs.
- [x] `animate`: effects on the clock of a slide; see [animation](slide.md#animation).
  A shape of a document is drawn as authored, with a native warning.
- [x] Lua `document.shape` uses the same validation and typed attributes.

Native output evaluates the outline, the text rectangle and the connection
sites of a shape from the definition of its preset in ECMA-376
presetShapeDefinitions.xml: the guide formulas, the paths with their arcs and
curves, and the paths that a definition fills lighter or darker or leaves
unfilled. The library holds a copy of the definitions of its kinds, which a
[test](../../tests/shape_presets.rs) compares with the standard.

DOCX emits an absolute page anchor for a fixed shape and an inline drawing in a
paragraph of its own for a shape set in the text, with editable Word shape text, preset geometry,
transforms, fills, borders, opacity, shadows, text margins and fit settings.
Native document output paints preset paths, transforms, fills, borders, opacity
and shadows, with one-point anchor flow for a fixed shape and the height of the
frame for a shape set in the text. The paragraph that holds a shape set in the
text is of the default paragraph style in DOCX, and native output gives the
shape the space before and after of that style, in the body and in a table
cell alike. Such a shape moves whole to the next column or page, stays at the
head of a page that cannot hold it, and begins with a column break that
precedes it, since a paragraph that holds a column break alone leaves an empty
line at the head of the next column in Word. Word's exports of the review cases
[drawings in text](../../tests/cases/word-70-drawings-in-text.mote),
[drawings under a style](../../tests/cases/word-71-drawings-under-style.mote) and
[drawings in columns](../../tests/cases/word-72-drawings-in-columns.mote) show
these rules, and the [drawing tests](../../tests/document_drawings.rs) hold
them. Shape text supports preset-specific text
bounds, nested text defaults, paragraph and inline formatting, lists and task
markers, margins, alignment, clipping, shrink-to-fit, text-height growth, and
rotation. Text-height growth extends the content clip without enlarging the
painted shape outline. Text stays readable through shape flips.

Tables, images, nested drawings, figure/caption blocks and page breaks inside
shape text are omitted with warnings. Field, note and anchor semantics contribute
only their child text. Paragraph pagination properties have no drawing-text effect.
Rotated PDF hyperlinks retain unrotated hit areas and warn about that limit.
A shape in a table cell is drawn at its own place on the page, as in the body,
and is not cut off at the cell ([table tests](../../tests/tables.rs)). A shape
in the body that moves to a new page leaves the space above it behind, as any
other block does and as Word's export of the
[shape-page-foot probe](../../tests/office/probes/shape-page-foot.mote) shows
([page foot tests](../../tests/page_foot.rs)). Positioned slide shapes have
[editable and native captures](../../tests/fixtures/presentation-shapes/README.md). Microsoft Word
render review is unreviewed; matching package captures do not establish visual
Office conformance. Unknown metadata has no implied output effect.

Positioned-slide output covers the existing drawing vocabulary: geometry,
transforms, fills, stroke patterns and opacity, names and text defaults. Shapes
also require preset text bounds, fitting and shadows; connectors require endpoint
geometry, caps and start/end marker kinds and sizes. Editable and native slide coverage is captured; PowerPoint visual review remains
unreviewed independently of model support.
