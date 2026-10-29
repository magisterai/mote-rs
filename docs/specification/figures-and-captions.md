# Figures and captions

[Specification](README.md) · [Figure checklist](../checklists/figure.md) ·
[Caption checklist](../checklists/caption.md)

`#figure[block content]` groups content with an optional caption and invisible
labels. `#caption[inline content]` supplies the caption text. Both require one
body and accept named arguments only. A figure may contain at most one direct
caption. Captions can also stand alone.

A figure's `kind` is descriptive metadata; `table` selects the default numbering
label “Table”, while other kinds use “Figure”. An explicit `alignment` supplies
default alignment for compatible children that do not declare their own. A caption
accepts `alignment` and a paragraph `style`, defaulting to `Caption`.

Caption `position` is `above` or `below`. Native layout orders the caption around
the other figure content; editable document output currently retains authored
child order. Author the caption on the intended side when both outputs matter.

Numbering is opt-in through a `numbering` object on the figure or caption:

- `enabled` controls numbering; an object without this field enables it.
- `id` names a shared sequence. The default sequence name comes from its label.
  Sequence identifiers use bookmark-safe names, so punctuation can normalize to
  the same identity.
- `label` supplies the visible prefix; `prefix` is its fallback alias.
- `start` is a positive integer for the first number. `restart` resets the sequence
  to `start`, or one if no start was supplied.
- `format` is `decimal`, `lower_alpha`, `upper_alpha`, `lower_roman`, or `upper_roman`.
- `separator` follows the number; its default is `. `.

Caption fields override figure fields individually. Enablement comes from the
first numbering object present, caption before figure. Numbering runs across
figures and standalone captions sharing the same sequence. The generated prefix
is bold; authored caption text keeps its own formatting.

Labels inside a figure, or immediately preceding it, associate with its caption.
Missing title and number metadata are inferred from the caption. A number reference
therefore displays the label and formatted number, such as “Figure 1”. Explicit
label metadata takes precedence. A label preceding a standalone caption infers its
title but does not acquire a figure number.

```mote
#paragraph[See #ref("results", kind = "number").]
#figure(kind = "table", numbering = { enabled = true })[
  #label("results")
  #table[
    #row[#cell[Region]#cell[Revenue]]
    #row[#cell[North]#cell[120]]
  ]
  #caption(position = "below")[Revenue by region]
]
```

The model retains the reference, figure, label, table, and caption. Numbering is
resolved during output and does not replace authored caption children in the model.

Native caption numbering inside cells and running stories is currently omitted.
Their metadata can still advance reference numbering, so visible caption numbers
and references can differ in that context.

Both directives accept flattened `properties` overrides and retain unknown nested
metadata. A figure also accepts `style` and the image-style `placement` object
(see [images](images.md)); these remain metadata except for the documented child
alignment behavior. This does not implement a floating figure layout. Output limits
are recorded in the checklists.
