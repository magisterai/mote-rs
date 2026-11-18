# Group

[Directive index](README.md) · [Presentation properties](attributes/presentations.md) ·
[Shapes and connectors](attributes/shapes-and-lines.md)

A group gathers positioned objects into one object, on a slide or in a document. The
[language definition](../specification/presentations.md#groups) is covered by
[model tests](../../tests/presentations.rs), and its output by
[slide rendering tests](../../tests/presentation_rendering.rs).

- [x] Named arguments only; block body required.
- [x] `name`: string or number, normalized to a string.
- [x] Body: boxes, shapes, lines and groups only; other content is rejected.
- [x] A box assigned to a layout `slot` is rejected within a group.
- [x] A group counts as a positioned object of its slide.
- [x] `properties` overrides and retained metadata; Lua `document.group`.
- [x] `position`: optional `x` and `y`, signed lengths that state the corner
  from which the objects are measured, zero where one alone is stated; optional
  `width` and `height`, nonnegative lengths that state the frame, which is
  otherwise the reach of the objects from the corner.
- [x] In a document, a group whose position states a place is fixed on its
  page, and a group whose position states none is set in the text and takes the
  room of its frame.
- [x] `alignment`: left/center/right, of a group set in the text.
- [x] `rotation`: number/string of clockwise degrees, with an optional `deg`
  suffix, about the centre of the frame of the group.
- [x] `animate`: effects on the group as one object; see
  [animation](slide.md#animation).
- [ ] The frame of a group has no effect on a slide, and neither scales nor
  cuts off an object anywhere. Group flips have no Mote argument.
- [ ] Group fill, opacity and effects have no Mote argument. PowerPoint gives a
  group no opacity of its own; a group is made faint by `fade_in` or `fade_out`.

PPTX writes a group shape, nested as authored, whose frame is the smallest
rectangle about the frames of its objects and whose child frame is the same, so
that each object keeps its slide coordinates. The corner of the `position`
displaces the frame and not the child frame, and a `rotation` is written upon the frame, so that
PowerPoint moves and turns the objects together. A group without a name is named
`Group N`. A group that holds no object at any depth is omitted with a warning. Native PDF and PNG draw the
objects of a group in their order, exactly as they draw them ungrouped, and
draw a displaced group in its place and a turned group as one turned layer.

A [box](box.md) of several pieces is written as a group in the same way.

DOCX writes a group of a document as one drawing that holds a group shape:
inline in a paragraph of its own where the group is set in the text, and
anchored to the page where it is fixed. The frame of the group shape begins at
the corner and has the size of the frame, and its child frame begins at zero
with the same size. Shapes, lines and groups are written within it; a box is
omitted with a warning, since a document sets no box. The namespace of the
group shape is declared on the drawing and marked ignorable, as the namespace
of a shape is on the document, and no fallback is written. Native document
output paints the objects about the corner, and gives a group set in the text
the height of its frame and eight points beneath it, the space of the paragraph
that holds the drawing in Word. A rotation turns a group of a document about
the centre of its frame.

Word opens the
[inline-drawings probe](../../tests/office/probes/inline-drawings.mote), and
its export is recorded among the
[Office references](../../tests/office/README.md): the lines about a shape and
a group set in the text lie within 0.2 point of the native lines.

PowerPoint opens the [groups case](../../tests/cases/powerpoint-32-groups.mote), and
its export is recorded among the
[Office references](../../tests/office/README.md).
