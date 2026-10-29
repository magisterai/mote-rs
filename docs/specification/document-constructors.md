# Document constructors in Lua

[Specification index](README.md) · Previous: [Functions](functions.md)

The `document` namespace lets Lua construct elements using the same names,
attributes, and value rules as source directives. A constructor returns an
element value, which a function can return or a binding can insert into content.

```mote
#function notice(body)
  return document.paragraph(
    {alignment = "center", spacing = {after = pt(8)}},
    document.strong("Notice: "), body)
end
#notice[The report is ready.]
```

## Arguments and bodies

An optional plain property table comes first and supplies named arguments. A
constructor's positional argument comes next, when defined: for example, a
heading level or link URL. If that argument is already present in the property
table, the remaining values are body content. The normal `properties` override
rules still apply inside the property table.

Remaining values form one content body. Strings and other scalar values become
text, element values retain their structure, and arrays recursively contribute
their members. A required body needs at least one value; an empty string supplies
an empty body. A constructor that does not accept a body treats its remaining
values as positional arguments and rejects extras.

```mote
#function heading_and_link()
  return {
    document.heading(2, "Resources"),
    document.paragraph(document.link(
      {url = "https://example.com", tooltip = "Project information"},
      "Project website"))
  }
end
#heading_and_link()
```

Lua bodies contain already evaluated nodes. Their strings are not parsed as
Markdown, and block bodies do not automatically wrap strings in paragraphs.
Construct the paragraph explicitly when its structure is needed, for example
`document.cell(document.paragraph("Value"))`. Inline constructors reject block
children. Lists accept items, tables accept rows, and rows accept cells, with
separating whitespace ignored.

## Available names

The namespace includes the currently documented constructors:

- Text and flow: `paragraph`, `heading`, `span`, `strong`, `emphasis`, `code`,
  `blockquote`, `line_break`, `tab`, and `break`.
- Lists and tables: `list`, `ordered_list`, `item`, `table`, `row`, and `cell`.
- Other content: `link`, `image`, `bookmark`, `label`, `ref`, `page_number`,
  `figure`, `caption`, `footnote`, and `endnote`.
- Declarations: `document`, `font`, `style`, `header`, and `footer`.
- Value helpers: `json`, `upper`, `color`, `pt`, `inch`, `in`, `cm`, `mm`, and `twip`.

Lua reserves the words `break` and `in`; use `document["break"](...)` and
`document["in"](...)`. The `section` directive requires raw source syntax and is
not in this namespace. Custom functions are called by their own names. A custom
function that shadows a native name removes that name from `document` in its
scope.

Declarations return nil and update the shared document state. A call such as
`document.document({page = {width = inch(8), height = inch(10)}})` sets document
geometry. Font/style properties and header/footer body rules retain their normal
meaning. Declaration-only functions can return nil without inserting content.

Constructor aliases retained in Lua continue to work in later expressions.
Element values use tagged Lua tables to preserve their kind, attributes, and
children; treating these as opaque values avoids changing their structure by
accident. The empty-table conversion limit described in [Functions](functions.md)
also affects empty element children. Leaf elements such as tabs and labels retain
their defined output behavior and do not render child data.

The same DOCX/native output support and diagnostics apply as for each source
directive. An attribute retained in the model is not necessarily painted or
represented in editable Office output.
