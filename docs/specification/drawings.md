# Shapes and lines

[Specification index](README.md) · Previous: [Document constructors](document-constructors.md)

Shapes and lines describe positioned drawing objects. Their attributes and
children are retained as typed model values. Output support is tracked separately
in the [shape](../checklists/shape.md) and [line](../checklists/line.md) checklists.

## Shapes

`shape` takes named arguments and an optional block body. Its required `position`
object contains `width` and `height`, and may contain `x` and `y`. These four
fields use the existing nonnegative length rules. Plain text in a source body
becomes a paragraph.

The **place** of an object is the pair `x` and `y` of its position. A position
that states one of them takes zero inches for the other; a position that states
neither keeps neither. On a slide, and within a group, an object without a place
stands at the origin of its container. In a document, the place decides how the
object stands:

- an object with a place is **fixed**: its coordinates are measured from the
  origin of the page on which it falls, and it takes no room in the text;
- an object without a place is **set in the text**: it stands between the blocks
  before and after it and takes the room of its frame.

```mote
#shape(position = {x = inch(1), y = inch(2), width = inch(3), height = inch(1)},
       kind = "rounded", fill = "#e6f2ed",
       border = {width = pt(1), color = "#28734f"})[
  A positioned drawing with editable text.
]
```

`kind` names one of the preset shapes of DrawingML, each of which the table
gives beside the preset that defines it. The aliases `rect`, `rounded`, and
`oval` normalize to `rectangle`, `round_rect`, and `ellipse`. An omitted kind
remains omitted in the model and uses rectangle geometry in output. Optional
`name` accepts a string or number and becomes text.

| Family | Kinds, each with its preset where the names differ |
| --- | --- |
| Basic | `rectangle` (rect), `round_rect` (roundRect), `ellipse`, `triangle`, `right_triangle` (rtTriangle), `diamond`, `parallelogram`, `trapezoid`, `pentagon`, `hexagon`, `octagon`, `plus`, `cloud` |
| Flowchart | `terminator`, `document`, `predefined_process`, `manual_input`, `preparation`, `delay`, `off_page_connector` and `stored_data` (flowChartOnlineStorage); the others are the presets `flowChart` with the name of the kind |
| Solids | `cylinder` (can), `cube` |
| Arrows | `right_arrow`, `left_arrow`, `up_arrow`, `down_arrow`, `left_right_arrow`, `notched_right_arrow`, `chevron`, `home_plate`, `circular_arrow` |
| Callouts | `rect_callout` (wedgeRectCallout), `round_rect_callout` (wedgeRoundRectCallout), `ellipse_callout` (wedgeEllipseCallout) |
| Rectangle variants | `snip_rect` (snip1Rect), `round_top_rect` (round2SameRect), `folded_corner`, `frame` |
| Arcs and rings | `pie`, `donut`, `block_arc` |
| Braces | `left_brace`, `right_brace` |
| Symbols | `star4`, `star5`, `heart`, `lightning_bolt` |

Four arguments adjust the preset of a kind. On a shape of another kind each is
retained without effect, and output reports a warning.

`corner_radius` is a nonnegative length: the radius of each corner of a
`round_rect` shape. The radius is at most half the shorter side of the shape,
and a greater length is drawn as that half. If `corner_radius` is omitted, the
radius is one sixth of the shorter side, so that the corner grows with the
shape.

```mote
#shape(position = {width = inch(3), height = inch(2)}, kind = "round_rect",
       corner_radius = pt(6), fill = "#dcfce7")[A card]
#shape(position = {y = inch(2.5), width = inch(1), height = inch(0.3)},
       kind = "round_rect", corner_radius = pt(6), fill = "#dcfce7")[A tag]
```

`tail` is a table of the lengths `x` and `y`: the point at which the tail of a
`rect_callout`, a `round_rect_callout` or an `ellipse_callout` ends, measured
from the origin from which the `x` and `y` of the position of the shape are
measured. A coordinate that is omitted keeps the default of the preset, whose
tail ends below the left half of the frame. The tail turns and flips with the
shape.

`angles` is a table of the numbers `start` and `end`, in degrees clockwise from
the right, each of which may be text with a `deg` suffix. A `pie` is the sector
from `start` clockwise to `end`, a `block_arc` the arc so bounded, and a
`circular_arrow` runs from `start` clockwise to its head at `end`. An angle that
is omitted keeps the default of the preset.

`thickness` is a nonnegative length, at most half the shorter side of the
shape: the width of the ring of a `donut`, of the band of a `frame`, or of the
arc of a `block_arc`.

```mote
#shape(position = {x = inch(1), y = inch(1), width = inch(2), height = inch(1)},
       kind = "rect_callout", tail = {x = inch(3.5), y = inch(2.5)})[Note]
#shape(position = {x = inch(4), y = inch(1), width = inch(1), height = inch(1)},
       kind = "pie", angles = {start = 270, end = 180}, fill = "#28734f")
#shape(position = {x = inch(6), y = inch(1), width = inch(1), height = inch(1)},
       kind = "donut", thickness = pt(8), fill = "#28734f")
```

`fill` is a color. `border` uses the image-border fields `width`, `color`, `style`,
and `opacity`. Styles are `single`, `solid`, `dashed`, `dotted`, or `none`.
`rotation` accepts a number or string with an optional `deg` suffix, removed in
the model. `flip_horizontal` and `flip_vertical` use the boolean rules.
`opacity` is retained as text after removing an optional percent suffix.

`shadow` contains `color`, `blur`, `distance`, `opacity`, and `angle`. Blur and
distance are lengths; angle follows the rotation rule, and the editable formats
write the direction as the equivalent angle from 0 up to 360 degrees. Unknown nested properties
are retained as metadata without implied output behavior.

## Shape text

The optional `text` object supplies text defaults. It supports `font`, `size`,
`color`, `highlight`, `bold`, `italic`, `underline`, `small_caps`, `strike`, and
`double_strike`, using the existing span rules. A numeric nested font name becomes
a string. `alignment` accepts left, center, right, or justify;
`vertical_alignment` accepts top, center, or bottom (`middle` means center).
`margin` is a map of lengths, normally left, right, top, and bottom.

`fit` is `clip`, `shrink`, or `resize_shape`. The aliases `none`, `autofit`, and
`resize` normalize respectively to those values. A shape whose text falls
below its text area, because `fit` is `clip` or absent or because shrinking has
reached its least size, gives a warning that names the shape. `text.paragraph`
uses the paragraph-style property family. Text and `text.list` also accept the
established `spacing`, `spacing_before`, `spacing_after`, `line_spacing`, and
`indent` fields.
The text of a shape is set at single line spacing, with no space before or after
a paragraph, unless these fields or a paragraph style state otherwise.

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

An end may instead name a shape of the same slide or group with `shape`, and a
`side` of it, `top`, `bottom`, `left` or `right`, or a `site` number. The end
then lies at a connection site of the shape: one of the points that the
DrawingML definition of its kind gives, at which a connector meets the
outline. A `side` takes the site nearest the middle of that side of the frame
among those that face within an eighth of a turn of that way, or among all the
sites where none faces so; a `site` counts the sites from zero in the order of
the definition. The site turns and flips with the shape. Normalization replaces the
end with the `x` and `y` of the site and records the number of the site
beside `shape`, so that an editable connector stays attached to the shape. The
name must belong to one shape alone of the slide, or of the outermost group of
a document; a name that no shape of the slide or group of the line bears, an
end with both `shape` and `x` or `y`, and a site that does not exist are errors.

```mote
#slide[
  #shape(name = "receive", kind = "parallelogram",
         position = {x = inch(1), y = inch(2), width = inch(1.8), height = inch(0.8)})
  #shape(name = "validate", kind = "chevron",
         position = {x = inch(4), y = inch(2), width = inch(1.8), height = inch(0.8)})
  #line(from = {shape = "receive", side = "right"},
        to = {shape = "validate", side = "left"}, arrow = "triangle")
]
```

`arrow` may be a string selecting the end marker, or an object containing
`start` and `end`. Marker kinds are `none`, `triangle`, `stealth`, `diamond`, and
`oval`; `arrow` is an alias for `triangle`. Object fields `from` and `to` are
aliases for `start` and `end`; an explicit canonical field wins over its alias.

Shared `size` and `length` fields apply to markers. `start_size`, `start_length`,
`end_size`, and `end_length` supply individual values. Their `from_` and `to_`
aliases normalize to `start_` and `end_`. Values `small`, `medium`, and `large`
normalize to `sm`, `med`, and `lg`; the short forms are also accepted.

A line belongs to a slide, or to a [group](presentations.md#groups); a document
draws a line as an object of a group alone. Lua provides `document.shape` and `document.line` with
the same argument and normalization rules. As with other Lua block constructors,
explicitly construct paragraphs when the shape body requires paragraph structure.
