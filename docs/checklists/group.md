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
- [x] `offset`: a displacement of signed `x` and `y` lengths, zero where
  omitted, that moves every object of the group.
- [x] `rotation`: number/string of clockwise degrees, with an optional `deg`
  suffix, about the centre of the frame of the group.
- [x] `animate`: effects on the group as one object; see
  [animation](slide.md#animation).
- [ ] Group size and flips have no Mote argument.
- [ ] Group fill, opacity and effects have no Mote argument. PowerPoint gives a
  group no opacity of its own; a group is made faint by `fade_in` or `fade_out`.

PPTX writes a group shape, nested as authored, whose frame is the smallest
rectangle about the frames of its objects and whose child frame is the same, so
that each object keeps its slide coordinates. An `offset` displaces the frame
and not the child frame, and a `rotation` is written upon the frame, so that
PowerPoint moves and turns the objects together. A group without a name is named
`Group N`. A group that holds no object at any depth is omitted with a warning. Native PDF and PNG draw the
objects of a group in their order, exactly as they draw them ungrouped, and
draw a displaced group in its place and a turned group as one turned layer.

A [box](box.md) of several pieces is written as a group in the same way.

A group has an effect on a slide only. DOCX and native document output place
the objects of a group singly, with a warning.

PowerPoint opens the [groups case](../../tests/cases/powerpoint-32-groups.mote), and
its export is recorded among the
[Office references](../../tests/office/README.md).
