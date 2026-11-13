# Group

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Shapes and connectors](attributes/shapes-and-lines.md)

A group gathers positioned slide objects into one object. The
[language definition](../specification/presentations.md#groups) is covered by
[model tests](../../tests/presentations.rs), and its output by
[slide rendering tests](../../tests/presentation_rendering.rs).

- [x] Named arguments only; block body required.
- [x] `name`: string or number, normalized to a string.
- [x] Body: boxes, shapes, lines and groups only; other content is rejected.
- [x] A box assigned to a layout `slot` is rejected within a group.
- [x] A group counts as a positioned object of its slide.
- [x] `properties` overrides and retained metadata; Lua `document.group`.
- [ ] Group position, size, rotation and flips: the DrawingML group transform
  is written only as the frame about the objects, and has no Mote argument.
- [ ] Group fill and effects have no Mote argument.

PPTX writes a group shape, nested as authored, whose frame is the smallest
rectangle about the frames of its objects and whose child frame is the same, so
that each object keeps its slide coordinates. A group without a name is named
`Group N`. An empty group is omitted with a warning. Native PDF and PNG draw the
objects of a group in their order, exactly as they draw them ungrouped.

A group has an effect on a slide only. DOCX and native document output place
the objects of a group singly, with a warning.

PowerPoint opens the [groups case](../../tests/cases/powerpoint-32-groups.mote), and
its export is recorded among the
[Office references](../../tests/office/README.md).
