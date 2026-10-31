# Scoped defaults

`#set` supplies properties for future elements in the current lexical scope:
`#set paragraph(alignment = "justify")`. It produces no content.

The target must be a supported element name, such as `paragraph`, `heading`,
`table`, `cell`, `span`, or `image`. `item` and `list_item` target list items.
Declarations and helpers such as `document`, `style`, and `json` are not targets.
Use named properties in parentheses; positional arguments and a content body
are not accepted. Required content or geometry need not be supplied by a default. Required named
arguments, such as chart `data`, must still be supplied on the constructor call.

The target's normalization rules apply. `properties` has the same precedence as
on a directive. Unknown properties remain available in the model and are subject
to the receiving output's support limits.

Successive defaults merge recursively. An explicit element property overrides its
default; nested objects merge by field, while arrays and scalar values replace
one another. Defaults affect directives, Markdown elements and Lua document
constructors. Markdown's own explicit properties, such as a heading level or a
table column's alignment, take precedence.

Defaults apply progressively and do not change already constructed content.
A content body, conditional branch, loop iteration or section has a local scope;
its defaults end with that scope. Functions capture their lexical defaults when
defined. Evaluated content values retain the properties they already received.

See the [support checklist](../checklists/set.md).

## Examples

```mote
#paragraph[Before any default.]
#set paragraph(alignment = "right", spacing = { after = pt(6) })
After the default.

#if(true)[
  #set paragraph(alignment = "center")
  A local default.
]

#paragraph(alignment = "left")[An explicit override.]
```
