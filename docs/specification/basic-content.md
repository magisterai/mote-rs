# Basic content

[Specification index](README.md) · [Syntax and values](syntax-and-values.md)

Status: initial draft. Read the fundamentals and syntax chapters first. This
chapter defines paragraphs, headings, strong text, and their basic alignment and
spacing attributes. Additional attributes and rendering rules will be specified
separately; the rules here describe evaluated content and formatting intent.

## 1. Shared content rules

Paragraphs and headings are blocks. Text and strong spans are inline content.
All three directives take exactly one square-bracketed body. The body may be
empty, but it must be present. Their bodies accept inline content, including
nested strong spans; they cannot contain paragraph or heading blocks.

A single source line ending inside flowing text supplies a space. At document
level, a blank line ends an implicit paragraph. A block directive also ends any
preceding implicit paragraph. Leading and trailing spaces, tabs, and line
endings at the edges of an implicit paragraph are discarded.

An explicit body preserves content spaces and indentation, except syntactic
indentation before directives and block markers as defined in the syntax chapter. A blank line
inside that body contributes two line-feed characters to its text; it does not
create a nested paragraph. Use separate paragraph directives for separate
paragraph blocks. [Inline text](inline-text.md) defines hard line-break syntax.

An **omitted attribute** is unspecified: no explicit value for that property is
added to the element. In particular, omitted spacing is distinct from an
explicit zero. The basic document model does not insert font choices, alignment,
or spacing defaults. Later formatting rules determine how unspecified
properties are rendered.

## 2. Paragraphs

`paragraph` produces one paragraph block. It accepts no positional arguments.
The named attributes covered here are `alignment` and `spacing`, defined in
section 6. Neither is required. An empty argument list may be omitted.

Ordinary Markdown text produces implicit paragraphs. For example, the single
line ending below joins two lines of text, while the blank line starts a second
paragraph:

```mote
First line
second line.

Another paragraph.
```

The result has two paragraphs containing “First line second line.” and “Another
paragraph.” These explicit directives produce the same content:

```mote
#paragraph[First line second line.]

#paragraph[Another paragraph.]
```

An explicit empty body still creates a paragraph block with no inline children:

```mote
#paragraph[]
```

## 3. Headings

`heading` produces one heading block. A level expresses position in a hierarchy:
1 is the top level, 2 is subordinate to 1, and so on. A supplied level must be a
positive integer. The document model does not impose an output format's
heading-level limit.

The optional `level` argument can be supplied by name or as the sole positional
argument. The positional form takes a number. The named form also accepts a
positive integer written as a string and converts it to a number. Supplying a
level both ways is an error. `alignment` and `spacing` have the same meaning as
on paragraphs.

This creates a level-two heading:

```mote
#heading(level = 2)[Results]
```

The positional form has the same result:

```mote
#heading(2)[Results]
```

For the Markdown form, the number of leading hashes determines the level. A
space or tab separates those hashes from the content, which extends to the line
ending. Thus the Markdown equivalent is:

```mote
## Results
```

The Markdown form also retains `marker = "#"` in the inspection model; the
numeric `level` records the number of hashes. The explicit directive does not
add that source marker. Adjacent text fragments remain separate in the model,
including line breaks and helper results; joining their text gives visible text.

If `level` is omitted, the evaluated heading leaves it unspecified. That is
distinct from authoring `level = 1`; examples that need a particular hierarchy
state their level. This example creates a heading without an explicit level:

```mote
#heading[Results]
```

Heading content can be empty. Neither its level nor its spelling determines a
font size at this stage.

## 4. Strong text

`strong` produces an inline span requesting bold text. It takes no positional
arguments. No named attributes are defined for it in this chapter. Its body
follows the shared inline-content rules and may be empty.

The `**content**` Markdown form has the same strong-text meaning. Within a
paragraph, surrounding text keeps its own formatting:

```mote
#paragraph[Read #strong[this] carefully.]
```

The paragraph contains “Read this carefully.”, with only “this” marked strong.
A strong span written directly at document level becomes part of an implicit
paragraph. It does not become a block itself.

## 5. Length helpers

A length carries a magnitude and a physical unit. Each helper below accepts
exactly one positional argument: a finite number or a string containing such a
number. There are no named arguments and no content body. A numeric string
supplies only the magnitude; the helper supplies the unit.

| Helper name | Result unit | Unit definition |
| --- | --- | --- |
| `pt` | `pt` | One point is 1/72 inch. |
| `in`, `inch` | `in` | One inch is 72 points. Both names construct the same unit. |
| `cm` | `cm` | One centimetre is 1/2.54 inch. |
| `mm` | `mm` | One millimetre is 1/25.4 inch. |
| `twip` | `twip` | One twip is 1/20 point. |

Zero and negative magnitudes can be constructed. The receiving attribute decides
whether a negative length is allowed; paragraph spacing does not allow it.
Evaluation retains the chosen unit rather than converting every length to a
common unit. These helpers require no scripting engine.

Inside content, a length becomes its magnitude followed immediately by its unit:

```mote
#paragraph[Lengths: #pt(12), #inch(1), #cm(2.54), #mm(25.4), #twip(20).]
```

The text is “Lengths: 12pt, 1in, 2.54cm, 25.4mm, 20twip.”

## 6. Alignment and spacing

The following optional attributes apply to both `paragraph` and `heading`.
Alignment uses one of these exact, case-sensitive string values:

| Value | Intended horizontal alignment |
| --- | --- |
| `"left"` | Lines align to the left edge. |
| `"center"` | Lines are centred between the edges. |
| `"right"` | Lines align to the right edge. |
| `"justify"` | Spaces between words adjust to align lines to both edges. |

Spaces inside the string are part of the value; they are not ignored.

`spacing` is a property object with `before`, `after`, or both. Each field
specifies a non-negative distance outside the block. Use a length helper or a
quoted magnitude followed by a supported unit, such as `"12pt"`. A quoted length
with a unit is normalized to a length value. This chapter specifies these
unit-bearing forms; the interpretation of unitless spacing is outside its scope.

An omitted field remains unspecified. Setting it to zero explicitly requests
zero spacing. The following paragraph requests centre alignment, six points
before it, and twelve points after it:

```mote
#paragraph(
  alignment = "center",
  spacing = { before = pt(6), after = pt(12) }
)[Quarterly report]
```

The equivalent quoted-length forms are:

```mote
#paragraph(
  alignment = "center",
  spacing = { before = "6pt", after = "12pt" }
)[Quarterly report]
```

Evaluation records the same attribute values for both. How neighbouring blocks'
spacing interacts is a layout rule and is not defined by adding these numbers
together here.

## 7. Invalid uses

These examples intentionally fail evaluation. They use the definitions above to
show where otherwise valid call syntax violates a content or value rule.

A paragraph requires a body, even when that body is empty:

```mote
#paragraph()
```

A heading cannot receive its level both positionally and by name:

```mote
#heading(2, level = 3)[Results]
```

A paragraph body cannot contain a heading block:

```mote
#paragraph[#heading(level = 1)[Title]]
```

Alignment values are case-sensitive, so this spelling is invalid:

```mote
#paragraph(alignment = "Center")[Text]
```

A length helper can construct a negative value, but paragraph spacing rejects it:

```mote
#paragraph(spacing = { after = pt(-1) })[Text]
```

The boolean `true` is not a numeric magnitude:

```mote
#pt(true)
```

Each failure must report a diagnostic. Exact wording and internal model
serialization are separate from these language rules.

Continue with [Inline text](inline-text.md) for emphasis, code, and line breaks.
