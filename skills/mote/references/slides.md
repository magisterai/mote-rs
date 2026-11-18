# Slides

A **deck** is a sequence of slides, declared by `#presentation`. A **slide** holds
either loose content, which is laid out automatically, or positioned objects. A
**box** is a positioned rectangle holding text, images, tables and charts, on a
slide alone; a document accepts no box, and uses a [shape](shapes.md) instead. A
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
- An image in a box is faded by the `opacity` of the image, as in
  `#image("logo.png", opacity = 0.4)`; the `opacity` of the box does not fade
  its picture.
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
  ungrouped. A `position = { x = inch(4), y = inch(1) }` displaces every object
  of the group, and a `rotation = 15` turns the group clockwise about its
  centre, so that one arrangement may be placed many times.
  Text, headings and lists go in a box or a shape of the group, never loose in
  the group itself.

## Common arrangements

Each slide below is a complete arrangement for a wide slide. The positions are
the part to reuse; colors and fonts are left at their defaults.

```mote
#presentation(size = "wide", footer = "Survey 2026", slide_number = true,
  field_text = { size = pt(11), color = "#475569" })

#slide(notes = "A title slide: a large title above a subtitle.")[
  #box(position = { x = inch(1), y = inch(2.4), width = inch(11.3), height = inch(1.5) },
       text = { size = pt(54), bold = true, vertical_alignment = "bottom" })[The first flight]
  #box(position = { x = inch(1), y = inch(4), width = inch(11.3), height = inch(0.9) },
       text = { size = pt(26) })[An evening talk]
]

#slide(notes = "A picture beside text, with a credit beneath the picture.")[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(1) },
       text = { size = pt(40), bold = true })[A picture beside text]
  #box(position = { x = inch(0.7), y = inch(1.7), width = inch(5.8), height = inch(4.6) })[
    #image("photo.png", alt = "The rocket on its stand")
  ]
  #box(position = { x = inch(0.7), y = inch(6.35), width = inch(5.8), height = inch(0.4) },
       text = { size = pt(12) })[Credit: the archive]
  #box(position = { x = inch(6.9), y = inch(1.7), width = inch(5.7), height = inch(4.6) },
       text = { size = pt(26), vertical_alignment = "middle" })[
    - The flight lasted 2.5 seconds.
    - The rocket reached 12.5 metres.
  ]
]

#slide(notes = "Two pictures side by side, each with a caption.")[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(1) },
       text = { size = pt(40), bold = true })[Two pictures]
  #box(position = { x = inch(0.7), y = inch(1.7), width = inch(5.8), height = inch(3.8) }, fit = "cover")[
    #image("photo.png", alt = "The first vehicle")
  ]
  #box(position = { x = inch(6.8), y = inch(1.7), width = inch(5.8), height = inch(3.8) }, fit = "cover")[
    #image("photo.png", alt = "The second vehicle")
  ]
  #box(position = { x = inch(0.7), y = inch(5.6), width = inch(5.8), height = inch(1) },
       text = { size = pt(20) })[The first vehicle, 1981.]
  #box(position = { x = inch(6.8), y = inch(5.6), width = inch(5.8), height = inch(1) },
       text = { size = pt(20) })[The second vehicle, 2018.]
]

#slide(notes = "A chart across the whole slide.")[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(1) },
       text = { size = pt(40), bold = true })[A chart]
  #box(position = { x = inch(0.7), y = inch(1.6), width = inch(12), height = inch(5.1) })[
    #chart(kind = "column", data_labels = true,
           data = { categories = { "1957", "1967", "2018" }, series = { { name = "Tonnes", values = { 0.5, 140, 63.8 } } } })
  ]
]

#slide(notes = "A table, its text sized by the box.")[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(1) },
       text = { size = pt(40), bold = true })[A table]
  #box(position = { x = inch(1.7), y = inch(1.8), width = inch(10), height = inch(3) },
       text = { size = pt(22) })[
    | Rocket | Year | Payload |
    | :--- | :---: | ---: |
    | Saturn V | 1967 | 140 |
    | Falcon Heavy | 2018 | 63.8 |
  ]
]

#slide(notes = "A timeline: a line, a marker for each date, and labels.")[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(1) },
       text = { size = pt(40), bold = true })[A timeline]
  #line(from = { x = inch(1), y = inch(3.8) }, to = { x = inch(12.3), y = inch(3.8) },
        border = { color = "#475569", width = pt(2) })
  #for entry in { { x = 2, left = 0.65, year = "1926", event = "First liquid-fuelled flight" },
                  { x = 6.2, left = 4.85, year = "1957", event = "First satellite" },
                  { x = 10.4, left = 9.05, year = "1969", event = "First Moon landing" } } {
    #shape(position = { x = inch(entry.x), y = inch(3.65), width = inch(0.3), height = inch(0.3) },
           kind = "ellipse", fill = "#2563eb")
    #box(position = { x = inch(entry.left), y = inch(2.7), width = inch(3), height = inch(0.8) },
         text = { size = pt(26), bold = true, alignment = "center" })[#entry.year]
    #box(position = { x = inch(entry.left), y = inch(4.2), width = inch(3), height = inch(1.2) },
         text = { size = pt(18), alignment = "center" })[#entry.event]
  }
]
```

- A picture keeps its proportion in its box and is centered there; `fit =
  "cover"` fills the box and crops the excess, and `fit = "stretch"` distorts
  the picture to the box. A `width` or `height` on the image itself has no
  effect in a box.
- The `text` argument of a box sets the size and color of its paragraphs and of
  a table within it. A box of text without `text.fit` neither shrinks nor grows:
  text that does not fit is cut off at the edge of the box, and a warning names
  the box. `text = { fit = "shrink" }` reduces the text of a box until it fits.
  The rows of a table share the height of its box; a row whose text needs more
  takes it from the others, and a warning is given only where the table cannot
  fit its box.
- A table on a slide has a pale fill
  and white borders by default. The declaration
  `#set table(cell = { fill = "#16213a" }, borders = { all = { color = "#334155", width = pt(0.5) } })`
  before the slides gives every table that fill and those borders, which a
  dark deck requires.
- The `field_text` argument of the deck, or of one slide, sets the `size`,
  `color`, `font`, `bold` and `italic` of the footer, date and slide number.
  The footer stands at the bottom left and the slide number at the bottom
  right, so the lowest half inch of a slide is best left free.
- The declarations of a deck, `#presentation`, `#style`, `#layout` and
  `#function`, stand before the first slide. A `#style "Heading1" (...)`
  declaration formats every level-one heading of the deck.
- The previews are the check: each slide is rendered to PNG and looked at for
  overflow, overlap and empty areas.

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
