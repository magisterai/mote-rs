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
one-based slide index when enabled. `field_text` is a table that formats the
footer, the date and the slide number: `size` is a length, `color` a color,
`font` a font name, and `bold` and `italic` are booleans; any other member is
invalid. Where it is omitted the three are set in grey at 10 points. `render_at` is a number of seconds, not
negative: the time at which the deck is rendered, as [animation](#animation)
defines it.

`theme` is a property table. Its `name` is retained metadata. `colors` normalizes
background, text, dark1, light1, dark2, light2, accent1–accent6, hyperlink and
followed_hyperlink. Background and text are aliases for the corresponding light
and dark theme colors during output. `fonts` normalizes heading, body, major and
minor font names, which may refer to the previously defined font declarations.
Unknown theme members remain metadata; a non-table colors/fonts value is retained
without becoming a resolved color or font mapping.

In presentation output, an empty source produces one blank slide at its declared
size, including inherited background and slide fields. An explicit empty
`#slide[]` has the same blank-slide behavior.

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
`duration` is a number of seconds, not negative: the length of the animation
of the slide, at which `--frames` ends. It does not move a presentation on.
`advance_after` is a number of seconds, not negative, after which a
presentation leaves the slide for the next without the act of a speaker.
`field_text` replaces, member by member, the `field_text` of the deck.

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

Continue with [Named presentation layouts](presentation-layouts.md) for slots and reusable geometry.

## Positioned boxes

`box` ordinarily requires a block body and a `position` table. Named slot
assignments use the exceptions defined in the next chapter. `width` and `height` are
required; omitted `x` and `y` become zero inches. All four values normalize
non-negative lengths. The origin is the slide's upper-left corner, horizontal
coordinates increase rightward and vertical coordinates increase downward.
Unknown position fields remain metadata.

A box contains text flow, images, tables and charts, in any order. Empty boxes
are allowed and warn. Text may include the paragraphs, headings, spans, links,
lists and block quotes defined earlier.

A **piece** of a box is a run of consecutive text flow, or a single image, table
or chart. A box of one piece is a single object. A box of several pieces is a
stack: the pieces follow one another down the frame of the box in their order,
each at the full width of the frame. A text piece takes the height of its lines.
An image takes the height its proportion gives at the width of the frame, and
no more than an equal share of the height the text leaves. The tables and charts
divide equally the height that remains. The `fill`, `border` and `shadow` of
the box apply to the frame, the text arguments to every text piece, and `fit`
to every image, where the default is `contain`. A stack ignores `text.fit`.

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
image child, and is contain where omitted: the whole image is shown in its
proportion, centred in the box. A `width` or `height` of the image itself has
no effect in a box. Rotation and flip metadata are not box geometry controls.

`text.fit` normalizes none to clip, autofit to shrink, and resize to resize_shape.
A box whose text falls below its frame, because `text.fit` is clip or absent or
because shrinking has reached its least size, gives a warning that names the
box. Text that fits gives none.
Nested text supports margins, alignment, vertical alignment, font and run
formatting, paragraph properties, and list spacing/indentation. Model support
records these properties; each output must separately implement their effects.

The run formatting of a box, its font, size, color and emphasis, is direct
formatting of each plain paragraph within: it overrides the `Normal` style,
which governs what the box leaves unstated. A named paragraph style and the
formatting of a heading are not overridden. A table in a box takes the same
run formatting, beneath the text formatting of its table style and of its
cells. A row that repeats as a header has nothing to repeat upon a slide and
is an ordinary row there. The height of a row upon a slide is a minimum: a row
is never lower than the text of its cells. The rows that state no height share
the box equally, save that a row whose text needs more than an equal share
takes what it needs and the others divide the remainder equally. A warning
names the box where the rows cannot fit and the table passes the lower edge of
its frame.

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

## Groups

A group gathers positioned objects of a slide into one object. `group` takes
named arguments and a required body, and produces a block `group` element. The
body holds boxes, shapes, lines and groups only; any other content is invalid,
as is a box assigned to a layout slot. Optional `name` accepts a string or
number and becomes text. A group is itself a positioned object, so that a slide
may hold groups beside boxes, shapes and lines, and a group may hold groups.

Text, a heading, a list and every other block belong in a box or a shape of
the group, where they are written exactly as in a box or a shape outside it. A
group has no frame of its own in which loose content could be placed.

A group without arguments moves nothing: each object within keeps the place
its own `position`, or its `from` and `to`, gives it on the slide. `offset` is
a table of the lengths `x` and `y`, of either sign and zero where omitted, by
which every object of the group is displaced. `rotation` is a number of
degrees, or text with an optional `deg` suffix, by which the group is turned
clockwise about the centre of the smallest rectangle that holds its objects.
A group may thus be written once and placed many times. The Lua constructor is
`document.group`.

```mote
#slide[
  #group(name="Card")[
    #shape(position={x=inch(1), y=inch(1), width=inch(4), height=inch(2)},
           kind="round_rect", fill="#dcfce7")
    #box(position={x=inch(1.2), y=inch(1.2), width=inch(3.6), height=inch(1)})[
      A caption on the card.
    ]
  ]
]
```

## Animation

The objects of a slide are animated on one clock. A time is a number of seconds
from the moment at which the slide is first shown.

`animate` is an argument of a box, a shape, a line and a group. Its value is
one effect or a list of effects, and an effect is a table. `effect` is required
and names the effect. `start` is the time at which the effect begins, and is
zero where omitted. `duration` is the time the effect takes, and is half a
second where omitted. Both are numbers of seconds, not negative. `easing` is
`linear`, `ease_in`, `ease_out` or `ease_in_out`, and is `linear` where
omitted: an effect that eases in gathers speed uniformly through the first
half of its duration, and an effect that eases out loses speed uniformly
through the second half.

| Effect | Arguments | Meaning |
| --- | --- | --- |
| `fade_in` | | The object comes into view by degrees. |
| `fade_out` | | The object passes out of view by degrees. |
| `wipe_in` | `from` | The object comes into view progressively, beginning at the edge `from`. |
| `wipe_out` | `from` | The object passes out of view progressively, beginning at the edge `from`. |
| `appear` | no `duration` | The object comes into view at `start`. |
| `disappear` | no `duration` | The object passes out of view at `start`. |
| `move` | `by` required | The object is displaced by the lengths `x` and `y` of `by`, of either sign. |
| `scale` | `to` required | The object is enlarged about its centre by the positive factor `to`. |
| `rotate` | `by` required | The object is turned clockwise about its centre by `by` degrees. |

The extent of an object is the smallest upright rectangle about its frame, and
that of a group the smallest about the extents of its members. `from` names an
edge of the extent, is `left`, `right`, `top` or `bottom`, and is `left` where
omitted. A `wipe_in` that is a fraction complete shows the part of the object
within that fraction of the extent, measured from the edge `from`; a `wipe_out`
that is a fraction complete hides that part and shows the rest.

An object whose first effect upon its visibility is `fade_in`, `wipe_in` or
`appear` is out of view until that effect begins. The effects of one object accumulate: a
second `move` begins where the first ended. An effect on a group acts upon the
group as one object. An argument that an effect does not take is invalid.

The binding `time` holds the time at which the source is rendered. Its value is
that of the `--time` option of the command line where one is given, otherwise
the `render_at` of the deck, otherwise zero. A value may therefore be computed
from `time`, in Mote or in Lua, where no effect expresses the change required:
a number that counts upward is an example. The option `--frames` renders the
source at successive times, and ends at the latest end of an effect or of a
slide `duration` unless `--to` is given.

A slide rendered without `--time` and without `render_at` is the slide as
authored: every effect is disregarded, and every object is in view at its
place. At a time, each object is drawn as its effects leave it at that time.

```mote
#presentation(render_at=1.5)
#slide(duration=3)[
  #shape(position={x=inch(1), y=inch(1), width=inch(2), height=inch(1)},
         fill="#2563eb",
         animate={
           {effect="fade_in", duration=1},
           {effect="move", by={x=inch(4)}, start=1, duration=2, easing="ease_in_out"},
         })[Seconds: #time]
]
```

Current output support and review status are recorded in the
[presentation](../checklists/presentation.md), [slide](../checklists/slide.md),
[box](../checklists/box.md) and [group](../checklists/group.md) checklists. They are separate from the language model.
