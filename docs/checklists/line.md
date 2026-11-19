# Line and connector

[Directive index](README.md) · [Drawing property inventory](attributes/shapes-and-lines.md)

The [language model](../specification/drawings.md) is covered by
[normalization and capture tests](../../tests/drawings.rs). Editable PPTX and native PDF/PNG have
[captured coverage](../../tests/fixtures/presentation-shapes/README.md).
All DrawingML line properties, marker ends,
transform records, connection metadata and value domains remain in the shared
inventory, including properties without an authoring mapping.

- [x] Named arguments only; no content body or positional arguments.
- [x] `name`: string or number, normalized to a string.
- [x] Required `from` and `to` point objects, each with required `x` and `y` lengths.
- [x] `border`: width, color, style (single/solid/dashed/dotted/none), opacity.
- [x] `cap`: flat, round or square.
- [x] `opacity`: numeric or percentage text, normalized to text.
- [x] `arrow` string selects the end marker; none, triangle/arrow, stealth,
  diamond and oval kinds.
- [x] `arrow` object start/end markers; from/to aliases normalize to start/end.
- [x] Shared size/length and start_size/start_length/end_size/end_length;
  from_size/from_length/to_size/to_length aliases normalize to start/end fields.
- [x] Marker sizes small/sm, medium/med and large/lg normalize to sm/med/lg.
- [x] Shared `properties` overrides and retained unknown nested metadata.
- [x] `animate`: effects on the clock of a slide; see [animation](slide.md#animation).
- [x] Lua `document.line` uses the same validation and typed attributes.

Output boundary: a slide draws a line, and a document draws a line that is an
object of a [group](group.md): DOCX writes the line as a connector shape of the
group shape, and native document output paints it. A line that stands alone in
a document is not drawn: DOCX omits it with a warning, while native document
output displays `[line]` and any retained child text with a warning. Shape borders are a separate
supported drawing property and must not be confused with line directives.

Positioned-slide output covers the existing drawing vocabulary: geometry,
transforms, fills, stroke patterns and opacity, names and text defaults. Shapes
also require preset text bounds, fitting and shadows; connectors require endpoint
geometry, caps and start/end marker kinds and sizes. Editable and native slide
coverage is captured. Native diamond markers use four vertices; marker width and
length resolve independently, including per-end overrides. Native output ends
the line at the base of a triangle marker and at the notch of a stealth marker,
so that the narrowing head does not lie over the full width of the line
([test](../../tests/output_edges.rs)); the arrows of the shapes-and-lines case
then leave no marks that differ from PowerPoint's export. An oval or a diamond marker is
centred on the end and covers it. The native PNG painter draws the cap that the
stroke states, as the PDF painter does
([test](../../tests/presentation_rendering.rs)).
PowerPoint visual review remains unreviewed independently of model support.
