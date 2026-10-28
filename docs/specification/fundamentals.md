# Fundamentals

[Specification index](README.md)

Status: initial draft.

Mote is a document language for describing content, structure, and formatting.
Source is evaluated into a structured document model from which output formats
can be produced.

## 1. Markdown and directives

Start with Markdown for ordinary document content. A **directive** is an explicit
Mote instruction embedded in the source. Directives can create content, configure
the document, or control evaluation. They are introduced by `#` followed by a
name, as in `#heading` or `#document`.

This source uses Markdown for a heading, a paragraph, and strong text:

```mote
# Quarterly report

Revenue increased by **12%**.
```

The same content can be expressed with directives:

```mote
#heading(level = 1)[Quarterly report]

#paragraph[Revenue increased by #strong[12%].]
```

Both examples describe a level-one heading followed by a paragraph containing
the text “Revenue increased by 12%.”, with “12%” marked as strong. Markdown and
directives can be mixed within one source.

The space after `#` in `# Quarterly report` identifies a Markdown heading. In
`#heading(...)`, the name immediately follows `#` and identifies a directive.

## 2. Arguments, attributes, and content bodies

Many directives use the following form:

```text
#name(arguments)[content]
```

- **Name:** identifies the operation, such as `heading`, `paragraph`, or `strong`.
- **Arguments:** values supplied inside parentheses. In `level = 1`, `level` is
  the argument name and `1` is its value. Multiple arguments are comma-separated.
- **Content body:** Mote content enclosed in square brackets. A body can contain
  text, Markdown, and nested directives, subject to the enclosing directive's
  content rules.

The `#strong[12%]` directive demonstrates a body without an argument list.
`#heading(level = 1)[Quarterly report]` has both arguments and a body. Each
directive defines which arguments and bodies it accepts; the common form is an
introduction to the syntax, not a complete grammar for every construct.

In the paragraph example, `#strong[12%]` is nested within the paragraph's body.
The paragraph contains inline content; the strong element applies to its own
content within that paragraph.

### Attribute syntax

**Attributes** are named properties of an element, such as a heading's level or a
paragraph's alignment. Supply them as named arguments inside the directive's
parentheses, using `name = value`:

```mote
#paragraph(
  alignment = "center",
  keep_with_next = true,
  spacing = { before = pt(6), after = pt(12) }
)[Quarterly report]
```

Attribute names are unquoted. Multiword names use underscores, as in
`keep_with_next`. Separate attributes with commas; spaces around `=` are optional,
and the argument list can span several lines.

Values have types:

| Syntax | Meaning |
| --- | --- |
| `level = 1` | A number, as in the heading example. |
| `alignment = "center"` | A quoted string selecting centre alignment. |
| `keep_with_next = true` | A boolean; use `true` or `false` without quotes. |
| `after = pt(12)` | A length of 12 points, constructed by the `pt` helper. |
| `spacing = { before = pt(6), after = pt(12) }` | A property object grouping related named values. |

A **property object** uses braces, with comma-separated `name = value` fields
inside. Here `before` and `after` belong to the `spacing` object. The paragraph
requests centre alignment, space before and after it, and that it stay with the
following paragraph when paginating. Its visible text is the content in
`[Quarterly report]`.

Each directive defines its accepted attributes and value types. Parentheses
carry arguments, which may include attributes; square brackets carry content.
Configuration directives use the same named-argument syntax for document
properties.

### Helper functions

A **helper** is a function that produces or transforms a value. Its result can
be used as an attribute value, assigned to a name, or inserted into content.
Call a helper with its name followed by arguments in parentheses:

| Call | Result |
| --- | --- |
| `pt(12)` | A length of 12 points. Other length helpers include `inch`, `cm`, `mm`, and `twip`. |
| `color("#fff")` | A colour value normalized to `#ffffff`. |
| `upper("paid")` | The string `"PAID"`. |
| `json("invoice.json")` | The data loaded from a JSON file; see section 5. |

In `pt(12)`, `12` is a **positional argument**: its position identifies its
meaning. Some helpers also accept named arguments, using the same `name = value`
syntax. For example, `format_money(12, currency = "USD")` returns `"$12.00"`.
Arguments are comma-separated; each helper defines which arguments it accepts.

Inside an argument expression, write a helper call directly. Within text, prefix
the call with `#` to evaluate it and insert its result:

```mote
#paragraph(spacing = { after = pt(12) })[Status: #upper("paid")]
```

Here `pt(12)` supplies the paragraph's spacing after it, while `#upper("paid")`
supplies text. The visible paragraph reads “Status: PAID”. These built-in helpers
work without enabling Lua. Later chapters will define the full helper catalogue
and its argument and result types.

## 3. Content and configuration

Content directives contribute document elements. Configuration directives set
properties used to interpret or format the document. For example:

```mote
#document(
  page = { size = "letter", orientation = "portrait" }
)

# Quarterly report

Revenue increased by **12%**.
```

Here `#document` sets the document's page configuration to portrait US Letter.
It has arguments and no content body, and contributes no visible text. The
heading and paragraph supply the visible content.

The `page` property object groups the page's `size` and `orientation` settings.

## 4. The document model

Evaluation converts source constructs into document values. Markdown headings
and explicit `#heading` directives both produce heading elements. Strong Markdown
and `#strong` both produce strong inline content.

The model distinguishes:

- **Block content**, such as headings, paragraphs, lists, and tables.
- **Inline content**, such as text and strong spans within a paragraph.
- **Configuration and declarations**, such as page properties, named styles,
  and fonts.

This model gives Markdown and directives shared meaning. Output generation uses
the evaluated document values. Content produced by Lua document functions also
enters this model.

## 5. Data and optional programming

JSON supplies structured data that source can use. Given an `invoice.json` file:

```json
{"client": {"name": "Acme"}}
```

This source loads the data, binds it to a name, and inserts a value into text:

```mote
#let(invoice = json("invoice.json"))

Client: #invoice.client.name
```

The result is a paragraph containing “Client: Acme”. `#let` declares the binding;
`json(...)` loads the data. The expression `#invoice.client.name` inserts the
selected value into the surrounding content. Relative data paths resolve from
the source file containing the reference.

Ordinary Markdown, native directives, and the JSON helper work without Lua.
Lua is optional and disabled by default. Enabling it permits Lua-backed document
functions and richer expressions. Those functions can compute values and produce
document content.

## 6. Evaluation order, layout, and output

Source is evaluated in order. When multiple source files are supplied, they are
evaluated in the supplied order within a shared root scope. Earlier bindings and
element defaults are available to later content. Later changes to element
defaults do not restyle content already evaluated. This permits a defaults file
to precede a document's body source.

The document model describes content and its properties. Layout determines where
that content is placed. Document layout flows content across pages; presentation
layout arranges content into slides.

Output format is a separate choice. DOCX represents editable Word documents and
PPTX represents editable PowerPoint presentations. Native PDF and PNG output can
use either document or presentation layout. Choosing PDF or PNG alone therefore
does not identify the intended layout model.

Subsequent chapters will define the detailed rules for syntax, scope, defaults,
layout selection, and each output format.
