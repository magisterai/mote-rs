# Shapes and lines

[Specification index](README.md) · Previous: [Document constructors](document-constructors.md)

Shapes and lines describe positioned drawing objects. Their attributes and
children are retained as typed model values. Output support is tracked separately
in the [shape](../checklists/shape.md) and [line](../checklists/line.md) checklists.

## Shapes

`shape` takes named arguments and an optional block body. Its required `position`
object contains `width` and `height`; omitted `x` and `y` become zero inches.
These four fields use the existing nonnegative length rules. Coordinates are
measured from the page or slide origin, rather than the current text position.
Plain text in a source body becomes a paragraph.

```mote
#shape(position = {x = inch(1), y = inch(2), width = inch(3), height = inch(1)},
       kind = "rounded", fill = "#e6f2ed",
       border = {width = pt(1), color = "#28734f"})[
  A positioned drawing with editable text.
]
```

`kind` accepts `rectangle`, `round_rect`, `ellipse`, `triangle`, `diamond`,
`hexagon`, `chevron`, `cloud`, `parallelogram`, and `trapezoid`. The aliases
`rect`, `rounded`, and `oval` normalize to `rectangle`, `round_rect`, and
`ellipse`. An omitted kind remains omitted in the model and uses rectangle
geometry in output. Optional `name` accepts a string or number and becomes text.

`fill` is a color. `border` uses the image-border fields `width`, `color`, `style`,
and `opacity`. Styles are `single`, `solid`, `dashed`, `dotted`, or `none`.
`rotation` accepts a number or string with an optional `deg` suffix, removed in
the model. `flip_horizontal` and `flip_vertical` use the boolean rules.
`opacity` is retained as text after removing an optional percent suffix.

`shadow` contains `color`, `blur`, `distance`, `opacity`, and `angle`. Blur and
distance are lengths; angle follows the rotation rule. Unknown nested properties
are retained as metadata without implied output behavior.

## Shape text

The optional `text` object supplies text defaults. It supports `font`, `size`,
`color`, `highlight`, `bold`, `italic`, `underline`, `small_caps`, `strike`, and
`double_strike`, using the existing span rules. A numeric nested font name becomes
a string. `alignment` accepts left, center, right, or justify;
`vertical_alignment` accepts top, center, or bottom (`middle` means center).
`margin` is a map of lengths, normally left, right, top, and bottom.

`fit` is `clip`, `shrink`, or `resize_shape`. The aliases `none`, `autofit`, and
`resize` normalize respectively to those values. `text.paragraph` uses the
paragraph-style property family. Text and `text.list` also accept the established
`spacing`, `spacing_before`, `spacing_after`, `line_spacing`, and `indent` fields.

```mote
#shape(position = {width = inch(2), height = inch(1)},
       text = {size = pt(16), color = "#123456", alignment = "center",
               vertical_alignment = "middle", fit = "autofit",
               margin = {left = pt(4), right = pt(4)}})[Status]
```

Direct shape attributes also retain text defaults: font, size, color, highlight,
the boolean formatting fields, alignment, vertical alignment, and margin.
Nested `text` values take precedence when output resolves them. Direct highlight
and font metadata remain as authored; use the nested text fields for their
normalized forms. The usual `properties` override rules apply before normalization.

## Lines and connector ends

`line` takes named arguments and no body. Required `from` and `to` objects each
contain required `x` and `y` lengths. `name`, `border`, and `opacity` follow the
shape rules. `cap` accepts `flat`, `round`, or `square`.

```mote
#line(from = {x = inch(1), y = inch(1)},
      to = {x = inch(3), y = inch(2)},
      border = {width = pt(2), color = "#28734f"},
      arrow = "triangle", cap = "round")
```

`arrow` may be a string selecting the end marker, or an object containing
`start` and `end`. Marker kinds are `none`, `triangle`, `stealth`, `diamond`, and
`oval`; `arrow` is an alias for `triangle`. Object fields `from` and `to` are
aliases for `start` and `end`; an explicit canonical field wins over its alias.

Shared `size` and `length` fields apply to markers. `start_size`, `start_length`,
`end_size`, and `end_length` supply individual values. Their `from_` and `to_`
aliases normalize to `start_` and `end_`. Values `small`, `medium`, and `large`
normalize to `sm`, `med`, and `lg`; the short forms are also accepted.

Lines belong to presentation output; document output support must not be inferred
from a valid line model. Lua provides `document.shape` and `document.line` with
the same argument and normalization rules. As with other Lua block constructors,
explicitly construct paragraphs when the shape body requires paragraph structure.
