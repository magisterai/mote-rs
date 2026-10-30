# Presentations and positioned boxes

[Specification index](README.md) · Previous: [Charts](charts.md)

A presentation is a sequence of slides. Each slide has its own canvas and content.
`presentation` declares deck defaults; `slide` creates an explicit slide; `box`
places content in a rectangle. These declarations and elements are independent
of document page margins and sections.

## Deck declarations

`presentation` accepts named arguments and no body. It emits no content. Repeated
calls replace matching properties while retaining other properties and previously
authored dimensions. `properties` overrides direct arguments after their declared
outer types have been checked.

`size` accepts `wide` (also `widescreen` or `16:9`), `standard` (also `4:3`), or a
property table of lengths. Named values are trimmed and normalized to `wide` or
`standard`. A size table normalizes every member as a non-negative length.
Its `width` and `height` update the retained dimensions. Direct `width` and
`height` take precedence within the same call. A later named size does not erase
previously authored dimensions. Omitted physical defaults belong to output and
are not inserted into the model.

`background` is a color or a table with `color`, `src` and `fit`. `src` names an
image resource; `fit` is `stretch`, `cover` or `contain`, using the image fitting
meanings defined earlier. Additional fields remain metadata. `footer` is text
or a boolean; `date` is fixed text, true for an automatic date field, or false to
suppress it. An automatic date uses the generation date with the full English
month name, day and year. `slide_number` normalizes a boolean and displays the
one-based slide index when enabled.

`theme` is a property table. Its `name` is retained metadata. `colors` normalizes
background, text, dark1, light1, dark2, light2, accent1–accent6, hyperlink and
followed_hyperlink. Background and text are aliases for the corresponding light
and dark theme colors during output. `fonts` normalizes heading, body, major and
minor font names, which may refer to the previously defined font declarations.
Unknown theme members remain metadata; a non-table colors/fonts value is retained
without becoming a resolved color or font mapping.

## Explicit slides

`slide` requires one block body and has no positional arguments. An empty body
creates an empty slide and a warning. `name` accepts text or a number and becomes
text. `notes` is speaker-note text. Background, footer, date and slide-number
properties override deck defaults; an explicit slide background replaces the
whole deck background, including an inherited image. False suppresses an inherited
footer, date or slide number. `footer=true` uses the declared deck footer text;
without deck text it produces no footer. A fixed footer or date string replaces
the inherited text. Speaker notes are trimmed plain text, independent of the
visible slide body; directive syntax inside a note string is not evaluated.

A slide contains either positioned boxes, shapes and lines, or loose content.
Mixing positioned objects and loose content in one slide is invalid.

Loose content receives automatic layout. A leading heading of any level occupies
the title frame; the remaining consecutive text blocks share a content frame.
Each image or table occupies a separate frame, with equal-height frames stacked
vertically. These frames adapt to the declared slide size and use text shrinking
to fit their available height. Explicitly positioned slides retain their authored
geometry. Automatic layout supports the same text, image and table properties as
positioned boxes.

```mote
#presentation(size="standard")
#slide[
  #heading(level=1)[Quarterly review]
  Revenue increased by #span(bold=true)[12%].

  - Expanded into two markets.
  - Improved customer retention.
]
```

## Plain Markdown decks

Presentation mode collects top-level content into slides. A level-one heading
starts a new slide when preceding content exists. A standalone line of at least
three hyphens (`---`) separates slides. Blank segments create no slides. Content
before the first heading forms a slide of its own. Lower-level headings stay on
the current slide. Explicit `slide` bodies form independent slides, and their
internal headings are not boundaries; implicit content may appear before or after
them.

Underlined Markdown headings use a nonempty text line followed immediately by a
line of at least two `=` characters (level one) or `-` characters (level two).
They retain `marker="setext"`. Underlined-heading recognition precedes divider
recognition, so adjacent `---` lines form a level-two heading containing `---`.
Put blank lines around separators to keep the boundary unambiguous.

```mote
# First slide

A short introduction.

---

## Second slide

The divider started this slide.
```

PPTX always selects presentation mode. Native PDF/PNG selects it through an
explicit presentation configuration or the caller's `presentation` layout mode.
Bare Markdown in automatic or document mode stays an ordinary document; headings
do not select a deck implicitly. A divider currently has no native document
painting behavior and reports unsupported content in document mode.

Named layouts and slot assignments are specified separately as their contracts
are introduced.

## Positioned boxes

`box` requires a block body and a `position` table. `width` and `height` are
required; omitted `x` and `y` become zero inches. All four values normalize
non-negative lengths. The origin is the slide's upper-left corner, horizontal
coordinates increase rightward and vertical coordinates increase downward.
Unknown position fields remain metadata.

A box contains text flow, or exactly one image, table or chart. Separate objects
need separate boxes. Empty boxes are allowed and warn. Text may include the
paragraphs, headings, spans, links, lists and block quotes defined earlier.

```mote
#presentation(size="wide", background="#ffffff")
#slide(name="Opening")[
  #box(position={x=inch(1), y=inch(1), width=inch(8), height=inch(1)})[
    #heading(level=1)[Quarterly review]
  ]
  #box(position={x=inch(1), y=inch(2.3), width=inch(8), height=inch(2)})[
    Revenue increased by #span(bold=true, color="#2563eb")[12%].
  ]
]
```

## Box appearance and text

Box appearance reuses the [shape formatting](drawings.md) definitions: `fill`,
`border`, `opacity`, `shadow`, alignment, vertical alignment, margins, direct run
formatting, and nested `text` properties. `name` accepts text or a number; `style`
is a retained text style hint. Box `fit` accepts contain, cover or stretch for an
image child. Rotation and flip metadata are not box geometry controls.

`text.fit` normalizes none to clip, autofit to shrink, and resize to resize_shape.
Nested text supports margins, alignment, vertical alignment, font and run
formatting, paragraph properties, and list spacing/indentation. Model support
records these properties; each output must separately implement their effects.

```mote
#slide[
  #box(position={width=inch(6), height=inch(2)}, fill="#f8fafc",
       margin={left=pt(12), right=pt(12)}, vertical_alignment="middle",
       text={size=pt(24), fit="shrink", alignment="center"})[
    A positioned message.
  ]
]
```

The corresponding Lua constructors are `document.presentation`, `document.slide`
and `document.box`. The declaration mutates deck configuration and returns nil;
slide and box constructors return typed block elements using the same validation.

```mote
#let(configured=document.presentation({size="standard"}))
#let(placed=document.box({position={width=inch(6),height=inch(2)}},"From Lua"))
#let(page=document.slide({name="Generated"},placed))
#page
```

The following is intentionally invalid: loose text cannot accompany a positioned
box in the same slide.

```mote
#slide[
  Loose text.
  #box(position={width=inch(4),height=inch(1)})[Positioned text.]
]
```

Current output support and review status are recorded in the
[presentation](../checklists/presentation.md), [slide](../checklists/slide.md) and
[box](../checklists/box.md) checklists. They are separate from the language model.
