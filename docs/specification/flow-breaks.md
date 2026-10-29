# Flow breaks

[Specification index](README.md) · [Checklist](../checklists/break.md)

`break` is a block directive with named arguments and no body or positional
arguments. It interrupts block flow and creates a `break` element in the model.
Its `type` is `"page"`, `"column"`, or `"line"`; omission explicitly stores
`"page"`. A page break requests a new page. A column break requests the next
column in the configured column layout. A block line break creates a separate
break paragraph, while the earlier `line_break` directive stays inside its
paragraph. Neither kind estimates where content would otherwise paginate.

```mote
#paragraph[Before the page break.]
#break()
#paragraph[After the page break.]
```

The `columns` configuration is defined in Document configuration. A break does not
create columns or change document geometry:

```mote
#document(columns = { count = 2, gap = pt(18) })

#paragraph[First column.]
#break(type = "column")
#paragraph[Next column.]
```

A block line break introduces a separate break paragraph:

```mote
#paragraph[Before.]
#break(type = "line")
#paragraph[After.]
```

Named metadata and the shared `properties` object follow the usual merge rules.
The outer `type` must be a string even when properties override it. The merged
value must be one of the three defined types. DOCX rejects extra attributes.

A break is invalid inside inline content:

```mote
#paragraph[Before#break()After]
```

A body is invalid:

```mote
#break()[Text]
```

DOCX supports top-level block breaks. Nested breaks can be represented in a
quote or list-item body, but their output is not yet supported and fails
explicitly. Section-related break types will be introduced with explicit
sections; they are not aliases for these three types.
