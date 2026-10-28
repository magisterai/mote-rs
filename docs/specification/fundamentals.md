# Fundamentals

[Specification index](README.md)

Status: initial draft.

Mote is a document language for describing content, structure, and formatting.
**Evaluation** reads source and produces a **document model**: a structured
representation of that content and its properties. Layout and output use this
model later.

## 1. Content and directives

A **block** is a separate unit of document content. A paragraph is a block whose
text flows together. **Inline content** occurs within a block: ordinary text is
inline content, and formatting can apply to a smaller portion of that text.

A **directive** is an explicit Mote instruction. Write `#` immediately followed
by its name. A **content body** follows in square brackets. The `paragraph`
directive creates a paragraph from one inline content body:

```mote
#paragraph[Revenue increased.]
```

The paragraph contains “Revenue increased.” The brackets mark its body and are
not part of the text.

## 2. Arguments and attributes

An **argument** is a value supplied to a directive inside parentheses. An
**attribute** is a named property of the resulting element. Supply an attribute
as a named argument using `name = value`. Names are unquoted; separate multiple
arguments with commas. The general form is:

```text
#name(arguments)[content]
```

A **heading** is a block that identifies a part of a document. Its `level`
attribute is a positive integer: level 1 is the top level, and level 2 is a
subordinate level. An integer is written without quotes. The `heading` directive
creates a heading from one inline content body:

```mote
#heading(level = 1)[Quarterly report]
```

Here `level = 1` supplies the attribute and `[Quarterly report]` supplies the
heading's text. Parentheses carry arguments; square brackets carry content.

## 3. Nested content

The `strong` directive creates an inline span requesting bold text. It takes one
inline content body and needs no arguments. It can occur inside a paragraph:

```mote
#paragraph[Revenue increased by #strong[12%].]
```

The paragraph contains “Revenue increased by 12%.” Only “12%” belongs to the
strong span. The inner closing bracket ends that span; the outer closing bracket
ends the paragraph.

## 4. Markdown equivalents

Markdown provides shorter forms for these same elements:

- A line beginning with `#` and a space creates a level-one heading. Additional
  hashes increase the level.
- Ordinary text becomes a paragraph. A blank line separates paragraphs.
- Text enclosed in `**` becomes strong inline content.

With those rules, this source describes the heading and paragraph above:

```mote
# Quarterly report

Revenue increased by **12%**.
```

Markdown and directives share meaning and can be mixed. The space in
`# Quarterly report` distinguishes a Markdown heading from a directive name
immediately following `#`.

## 5. Property objects and helpers

Paragraphs have an `alignment` attribute specifying horizontal text alignment.
The value `"center"` requests centred text. Quotes delimit a **string**, which is
text used as a value; those quotes are not part of the value.

The `spacing` attribute groups distances outside the paragraph. Its `before`
field specifies space before the paragraph, and `after` specifies space after
it. A **property object** groups such named fields inside braces, using
comma-separated `name = value` entries.

A **helper** is a function that returns a value. The `pt` helper takes one
positional magnitude and constructs a length in points. A point is 1/72 of an
inch. In `pt(12)`, the argument `12` supplies the magnitude by its position rather
than by a name. Spacing requires non-negative lengths.

When supplying an argument value, call a helper directly. In content, prefix it
with `#` to insert its result as text. A length is written as its magnitude
followed by its unit, so inserting `pt(12)` produces `12pt`:

```mote
#paragraph(
  alignment = "center",
  spacing = { after = pt(12) }
)[Spacing value: #pt(12).]
```

This paragraph contains “Spacing value: 12pt.”, requests centre alignment, and
records 12 points of space after it. Line breaks inside the argument list do not
add text to its content body.

Continue with [Syntax and values](syntax-and-values.md) for the spelling of
values, calls, and escapes, then [Basic content](basic-content.md) for the
contracts of these directives, attributes, and length helpers.
