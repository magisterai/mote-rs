# Shapes

A **shape** is positioned geometry with optional text, available in documents and
on slides, and written `#shape(...)[text]`. A **line**, written `#line(...)`, is a
straight connector with optional arrowheads, available on slides and within a
group. A **group**, written `#group[...]`, gathers shapes, lines and groups into
one drawing.

## A shape

```mote
#shape(
  position = { x = inch(5.5), y = inch(1), width = inch(2.5), height = inch(1.25) },
  kind = "round_rect",
  fill = "#dcfce7",
  border = { color = "#15803d", width = pt(1) },
  text = { size = pt(10), color = "#14532d", alignment = "center", vertical_alignment = "center" }
)[Key finding: retention rose for the third quarter running.]
```

- The `position` argument is required. It gives `width` and `height`, and
  optionally `x` and `y`.
- The `kind` argument is `rectangle`, the default, or one of `round_rect`,
  `ellipse`, `triangle`, `diamond`, `hexagon`, `chevron`, `cloud`,
  `parallelogram` and `trapezoid`.
- The `corner_radius` argument is a length, such as `pt(6)`, giving the corner
  of a `round_rect`. If the argument is omitted, the corner is a sixth of the
  shorter side, so a small tag and a large card receive different corners; a
  stated radius gives them the same corner.
- The `fill` argument is a color. The `border` argument gives `width`, `color`,
  `style` (`solid`, `dashed`, `dotted` or `none`) and `opacity`.
- The remaining appearance arguments are `opacity`, a percentage such as `35`;
  `rotation`, in degrees; `flip_horizontal` and `flip_vertical`; and
  `shadow = { color, blur, distance, angle, opacity }`.
- The optional body holds the shape's text: paragraphs, inline formatting and
  lists.

## Placement

In a document, a shape or a group **without** `x` and `y` stands in the text: it
takes the room of its frame between the paragraphs about it, and moves with
them.

```mote
The plan has two stages.

#group(alignment = "center")[
  #shape(position = { x = inch(0), y = inch(0), width = inch(2), height = pt(20) },
    fill = "#2563eb", text = { size = pt(9), color = "#ffffff" })[Design]
  #shape(position = { x = inch(1.5), y = inch(0.4), width = inch(3), height = pt(20) },
    fill = "#059669", text = { size = pt(9), color = "#ffffff" })[Build]
  #line(from = { x = inch(0), y = inch(0.9) }, to = { x = inch(4.5), y = inch(0.9) },
    border = { width = pt(1), color = "#6b7280" }, arrow = "triangle")
]

#shape(position = { width = inch(3), height = inch(0.6) }, kind = "round_rect",
  fill = "#dcfce7", text = { size = pt(10), vertical_alignment = "center" })[Both stages are funded.]
```

- The objects of a group are measured from the corner of the group. A group in
  the text is as wide and as tall as its objects reach from that corner;
  `position = { width = inch(5), height = inch(2) }` states the frame instead.
- The `alignment` of a group in the text is `left`, `center` or `right`.
- A shape or a group in the text is not divided between pages or columns, and
  takes its room in a table cell as it does in the body.

In a document, a shape or a group **with** `x` and `y` is fixed: the coordinates
are measured from the top-left corner of the page on which the directive falls,
the object takes no room, and the text does not wrap around it. Such an object
belongs in a margin or behind text with a low `opacity`.

```mote
#paragraph[This line sits inside a highlighted band.]

#shape(
  position = { x = inch(1), y = inch(1), width = inch(6.5), height = pt(18) },
  fill = "#fde68a",
  opacity = 35
)
```

On a slide every object is placed: `x` and `y` are measured from the top-left
corner of the slide, and are zero where omitted.

A `#box` belongs to a slide alone, and a box in a document fails to compile. A
framed passage in a document is a shape with text, and a table, an image or a
chart stands in the text itself.

## Shape text

The `text` table sets defaults for the body: `font`, `size`, `color`, `bold`,
`italic`, `underline`, `alignment` (`left`, `center`, `right` or `justify`),
`vertical_alignment` (`top`, `center` or `bottom`), `margin` (lengths for `left`,
`right`, `top` and `bottom`) and `fit`. The `fit` value determines the treatment
of text that exceeds the shape:

- `clip`, the default, cuts the text at the shape's edge, with a warning;
- `shrink` reduces the text until it fits;
- `resize_shape` extends the text area downwards.

A table, image or page break inside a shape is omitted, with a warning.

## Lines and arrows

```mote
#presentation(size = "wide")
#slide[
  #line(
    from = { x = inch(1), y = inch(1) },
    to = { x = inch(4), y = inch(2) },
    border = { width = pt(2), color = "#28734f" },
    arrow = "triangle",
    cap = "round"
  )
]
```

The `arrow` argument is the marker at the end, one of `triangle`, `stealth`,
`diamond`, `oval` and `none`, or a table `{ start = "oval", end = "triangle" }`
giving both ends. The `cap` argument is `flat`, `round` or `square`. In a document
a line is drawn within a group; a line that stands alone appears as the text
`[line]`, with a warning.
