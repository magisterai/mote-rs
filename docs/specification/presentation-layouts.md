# Named presentation layouts

[Specification index](README.md) · Previous: [Presentations](presentations.md)

A named layout supplies reusable geometry for a slide. A **slot** is a named
rectangle with a content type and default box properties. The slide supplies
content using `box(slot="name")[...]`, without repeating its position.

## Built-in layouts

Built-ins adapt to the deck's slide dimensions. Their slots use shrinking text
fit by default. All built-in slots are optional: an omitted slot emits no object.

| Layout | Slots in output order |
| --- | --- |
| `blank` | None |
| `title_slide` | `title` (text), `subtitle` (text) |
| `title_and_content` | `title` (text), `content` (content) |
| `section_header` | `title` (text), `subtitle` (text) |
| `two_column` | `title` (text), `left` (content), `right` (content) |
| `comparison` | `title` (text), `left_title` (text), `left` (content), `right_title` (text), `right` (content) |
| `image_and_caption` | `title` (text), `image` (image), `caption` (text) |
| `chart_and_caption` | `title` (text), `chart` (chart), `caption` (text) |

A text slot accepts the text flow already defined for boxes. An image, table or
chart slot accepts exactly one element of that type. A content slot accepts any
valid box body: text flow, or exactly one image, table or chart. An empty text or
content assignment is valid; an empty typed-media assignment is not.

```mote
#presentation(size="wide")
#slide(layout="two_column")[
  #box(slot="title")[#heading(level=1)[Quarterly review]]
  #box(slot="left")[Revenue increased by 12%.]
  #box(slot="right")[Customer retention improved.]
]
```

## Assignment and overrides

`slide.layout` is a trimmed text name. A slot name is nonempty trimmed text or a
number converted to text. Slot boxes must be direct children of a slide using a
named layout. Unknown layouts or slots, duplicate assignments and content-type
mismatches are errors. Named-layout slides cannot contain loose content.

An assignment inherits the slot's box properties. Its authored properties replace
matching values; nested property tables merge recursively. Supplying a position
uses the usual position contract, including required width/height and zero x/y
defaults. Slot assignments may omit their body to supply an empty text/content
slot. They may also use `properties` or the existing Lua `document.box` and
`document.slide` constructors.

```mote
#slide(layout="title_and_content")[
  #box(slot="content", fill="#eff6ff", text={size=pt(22)})[
    A styled content frame.
  ]
  #box(slot="title")[#heading(level=1)[Summary]]
]
```

The title renders before content even though its assignment occurs later in the
source. Supplied slots follow layout order; ordinary positioned boxes, shapes and
lines then follow in their authored order. The evaluated JSON retains the authored
assignments and order. Resolution creates concrete geometry without mutating that
model.

Mote layouts are authoring constructs. Output materializes editable PPTX objects
and shares their resolved geometry with native PDF/PNG. This does not establish
PresentationML placeholder or slide-master inheritance support. Custom layout
declarations are introduced separately.
