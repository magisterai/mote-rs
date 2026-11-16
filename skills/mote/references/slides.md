# Slides

A **deck** is a sequence of slides, declared by `#presentation`. A **slide** holds
either loose content, which is laid out automatically, or positioned objects. A
**box** is a positioned rectangle holding text, images, tables and charts. A
**layout** is a named arrangement of **slots**, boxes whose position is
given once and filled on each slide.

## The deck

```mote
#presentation(
  size = "wide",
  background = "#ffffff",
  theme = { colors = { accent1 = "#2563eb", text = "#1f2633" }, fonts = { heading = "Helvetica", body = "Helvetica" } },
  footer = "Meridian Transit Survey",
  slide_number = true
)

#slide(notes = "Open with the headline number.")[
  #heading(level = 1)[Quarterly review]

  - Revenue rose by 12%.
  - Two new markets.
]
```

- The `size` argument is `wide` (16:9), `standard` (4:3), or a table of `width`
  and `height`.
- The `background` argument is a color or a table with an image `src` and a `fit`
  of `stretch`, `cover` or `contain`.
- The `theme` table gives `colors` (`accent1` to `accent6`, `text`, `background`,
  `hyperlink`) and `fonts` (`heading`, `body`), which charts and text inherit.
  The `text` color is the color of all text in boxes, shapes and charts that
  sets none of its own, so a dark `background` requires a light `text`. The
  `heading` font is used by headings and the `body` font by all other text.
- The `footer`, `date` and `slide_number` arguments add those fields to every
  slide; a slide may override or disable each one.
- The `notes` argument of a slide gives the speaker notes, as plain text.
- The `duration` argument of a slide and the `animate` argument of its objects
  are described in [animation](animation.md).

A deck may also be written as Markdown: in a source with `#presentation`, each
level-one heading begins a slide, and a line of `---` between blank lines
separates slides.

## Positioned slides

```mote
#presentation(size = "wide")
#slide[
  #box(position = { x = inch(0.6), y = inch(0.4), width = inch(12), height = inch(1) })[
    #heading(level = 1)[Results]
  ]
  #box(position = { x = inch(0.6), y = inch(1.6), width = inch(5.8), height = inch(4.5) },
       fill = "#f8fafc", margin = { left = pt(12), right = pt(12) }, text = { size = pt(20), fit = "shrink" })[
    - Revenue rose by 12%.
    - Retention improved.
  ]
  #box(position = { x = inch(6.8), y = inch(1.6), width = inch(5.8), height = inch(4.5) })[
    #chart(kind = "column", data = { categories = { "Q1", "Q2" }, series = { { name = "Revenue", values = { 10, 12 } } } })
  ]
]
```

- Coordinates are measured from the top-left corner of the slide; a wide slide
  is 13.333 by 7.5 inches, and a standard slide 10 by 7.5 inches.
- A box accepts the appearance and `text` arguments of a shape, described in
  [shapes](shapes.md), including `fit = "shrink"` for text that must fit.
- An image alone in a box takes its size from the box, and a `width` or `height`
  on the image has no effect and gives a warning. The picture is stretched to
  the box unless the box has `fit = "contain"`, which shows the whole picture
  centered in the box, or `fit = "cover"`, which fills the box and crops the
  excess. A photograph therefore requires `fit` on its box:
  `#box(position = { ... }, fit = "contain")[#image("photo.jpg", alt = "...")]`.
- An image in a box is faded by the `opacity` of the image, as in
  `#image("logo.png", opacity = 0.4)`; the `opacity` of the box does not fade
  its picture.
- A title is a box of text with a large `text.size`, or a box that holds a
  heading; both are ordinary editable text in PowerPoint.
- A box may mix text with images, tables and charts. The pieces are stacked
  down the box in their order: text takes the height of its lines, an image the
  height of its proportion, and tables and charts share what remains. PowerPoint
  receives the pieces as one group named after the box. A warning says when
  the pieces exceed the box.
- Shapes and lines may stand beside boxes on a positioned slide.
- A `#group(name = "Card")[ ... ]` gathers boxes, shapes, lines and other groups
  into one PowerPoint group, which is selected and moved as a unit there. The
  group without arguments moves nothing: each object keeps its own position on
  the slide, and PDF and PNG draw the objects exactly as they draw them
  ungrouped. An `offset = { x = inch(4), y = inch(1) }` displaces every object
  of the group, and a `rotation = 15` turns the group clockwise about its
  centre, so that one arrangement may be placed many times.
  Text, headings and lists go in a box or a shape of the group, never loose in
  the group itself.

## A slide of a picture and text

```mote
#presentation(size = "wide", background = "#f7f3ea",
  theme = { colors = { text = "#1c2733", accent1 = "#b4532a" }, fonts = { heading = "Helvetica", body = "Helvetica" } })
#style "Heading1" (run = { size = pt(40), bold = true, color = "#b4532a" })

#slide[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(1.1) })[
    # The first flight
  ]
  #box(position = { x = inch(0.7), y = inch(1.8), width = inch(5.6), height = inch(5) }, fit = "contain")[
    #image("photo.png", alt = "The rocket on its stand")
  ]
  #box(position = { x = inch(6.8), y = inch(1.8), width = inch(5.8), height = inch(4.4) },
       text = { size = pt(26), vertical_alignment = "middle" })[
    - The flight lasted 2.5 seconds.
    - The rocket reached 12.5 metres.
  ]
  #box(position = { x = inch(6.8), y = inch(6.3), width = inch(5.8), height = inch(0.4) },
       text = { size = pt(12), color = "#6b7280" })[Credit: the archive]
]
```

- The colors and fonts above are one example; a deck takes a palette and
  typefaces that suit its subject and audience.
- The declarations of a deck, `#presentation`, `#style`, `#layout` and
  `#function`, stand before the first slide.
- The deck `footer` is drawn small and grey at the bottom left of each slide and
  the slide number at the bottom right, so the lowest half inch of a slide is
  best left free of other objects.
- The `text` argument of a box does not reach a table within it. The text of a
  table on a slide is sized and colored in its cells, as in
  `#cell[#span(size = pt(20))[Saturn V]]`.
- A `#style "Normal"` declaration is best left out of a deck: its size and
  color override the `text` argument of every box and shape.
- A table on a slide is written with `#table`, since a Markdown table marks its
  header to repeat across pages and that gives a warning on a slide. The rows
  of the table share the height of its box equally.
- A slide shown to a room wants a title of 36 to 44 points and body text of 22
  to 28 points. The boxes together should occupy most of the slide: a picture
  and its text each take about half the width and most of the height below
  the title.
- A `#style "Heading1" (...)` declaration sets the size, weight and color of
  every level-one heading of the deck, and `"Heading2"` of every level-two
  heading, so a title needs no formatting of its own.
- A box of text without `fit` neither shrinks nor grows, so text too long for
  its box does not fit within it; `text = { fit = "shrink" }` prevents this.
- The previews are the check: each slide is rendered to PNG and looked at for
  overflow, overlap, empty areas and text too small to read.

## Layouts

The built-in layouts are `title_slide` (`title`, `subtitle`), `title_and_content`
(`title`, `content`), `section_header` (`title`, `subtitle`), `two_column`
(`title`, `left`, `right`), `comparison` (`title`, `left_title`, `left`,
`right_title`, `right`), `image_and_caption` (`title`, `image`, `caption`),
`chart_and_caption` (`title`, `chart`, `caption`) and `blank`. A slide names its
layout and fills slots with `#box(slot = "name")`.

```mote
#presentation(size = "wide")
#layout(name = "report", based_on = "title_and_content")[
  #box(slot = "title", fill = "#eff6ff", text = { color = "#2563eb" })
]

#slide(layout = "two_column")[
  #box(slot = "title")[#heading(level = 1)[Two markets]]
  #box(slot = "left")[North grew by 12%.]
  #box(slot = "right")[South grew by 4%.]
]

#slide(layout = "report")[
  #box(slot = "title")[#heading(level = 1)[Summary]]
  #box(slot = "content")[Both markets grew.]
]
```

A `#layout` declaration defines a custom layout: its body holds empty slot boxes
with a `position`, a `kind` (`text`, `image`, `table`, `chart` or `content`) and
default appearance, and `based_on` inherits the slots of another layout. A slot
box on a slide may override any of the slot's properties.

## Limits

- A slide holds loose content or positioned objects, never both.
- Layouts become ordinary positioned objects in the PPTX, not PowerPoint
  placeholders or slide masters.
