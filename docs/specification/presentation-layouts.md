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
PresentationML placeholder or slide-master inheritance support. The following sections define custom declarations.

## Custom declarations and inheritance

`layout` declares a reusable layout and emits no visible content. `name` is
required text or a number converted to text; whitespace is trimmed and the result
must be nonempty. Built-in names are reserved. An optional `based_on` text name
inherits a built-in or custom layout. Forward references are allowed; unknown
parents and cycles are errors. A later declaration with the same name replaces
it in its original declaration position.

The optional body contains only empty `box` slot declarations. Each needs a
nonempty `slot` name, unique within that declaration. A new slot needs a complete
`position`. An inherited slot may omit its position and other properties.

`kind` declares the accepted content: `text`, `image`, `table`, `chart`, or
`content` (also `any`). Omission inherits the parent's kind; a new slot defaults
to content. `required=true` requires the slide to supply that slot. Omission
inherits its required state; a new slot is optional. Explicit `required=false`
makes an inherited required slot optional. These fields belong only to layout
slot declarations, not slide assignments or ordinary boxes.

```mote
#presentation(size="standard")
#layout(name="report", based_on="title_and_content")[
  #box(slot="title", kind="text", required=true,
       fill="#eff6ff", text={color="#2563eb"})
  #box(slot="content", text={size=pt(22)})
]
#slide(layout="report")[
  #box(slot="title")[#heading(level=1)[Quarterly report]]
  #box(slot="content")[Revenue increased by 12%.]
]
```

Inheritance merges property tables recursively. Replacing an existing slot
retains its order; new slots append in declaration order. `remove=true` removes
an inherited slot. A removal may contain only `slot` and `remove`: no kind,
required flag, geometry, styling or content. Removing an unknown slot is an error.

```mote
#layout(name="opening", based_on="title_slide")[
  #box(slot="subtitle", remove=true)
  #box(slot="title", text={alignment="center"})
]
#slide(layout="opening")[#box(slot="title")[#heading(level=1)[Annual review]]]
```

A layout may declare `background`, `footer`, `date` and `slide_number` defaults
using the slide value rules. Layout-to-layout inheritance merges nested background
fields. A slide's explicit background replaces the whole inherited value, so a
color-only slide override suppresses an inherited image. Slide field overrides,
including false, take precedence over layout defaults, which take precedence
over deck defaults. Image and font resources use the caller's asset resolver and
the authoring source, including values supplied through JSON.

`properties` overrides the layout's ordinary property arguments after outer type
checks. Its `name` and `based_on` entries do not replace the declaration identity
or parent; those come from their direct arguments.

## Lua and inspection boundaries

Typed JSON values can supply layout names, slot positions and formatting through
the same directive arguments. `document.layout` creates a declaration and returns
nil; it can derive a layout and set metadata. The following opt-in Lua example
inherits all slots from a built-in layout:

```mote
#let(configured=document.layout({name="summary",based_on="title_and_content",footer="Internal"}))
#slide(layout="summary")[
  #box(slot="title")[#heading(level=1)[Summary]]
  #box(slot="content")[A reusable layout declared through Lua.]
]
```

The existing empty-content Lua round-trip converts an empty box's children into
text. Consequently, passing an empty `document.box` into `document.layout` cannot
currently declare a slot. Declare slots with `layout`/`box` directives; Lua may
inherit them and supply slide content. This limitation also applies when an empty
string body is supplied.

The library model retains ordered layout declarations, slots and optional
required flags. JSON inspection currently retains slide assignments but omits
layout declarations. It is therefore not a complete layout-definition export.

Mote's `based_on` is resolved before output. It does not represent a PresentationML
layout-to-layout property: PPTX retains the established slide/layout/master package
relationships and materializes the resolved objects.

The following examples are intentionally invalid. A required slot must be supplied:

```mote
#layout(name="report",based_on="title_slide")[#box(slot="title",required=true)]
#slide(layout="report")[]
```

Inheritance cannot contain a cycle:

```mote
#layout(name="first",based_on="second")
#layout(name="second",based_on="first")
```
