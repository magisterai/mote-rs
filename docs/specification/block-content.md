# Literal blocks, dividers and definitions

[Specification index](README.md) · [Semantic text formatting](semantic-formatting.md)

These forms extend the existing paragraph, inline-code and list rules.

## Literal block content

`raw` creates a block of literal text. It accepts an optional positional string,
number or boolean and at most one inline body. A supplied positional value takes
precedence, but the body is still evaluated and validated. Without either, the
block is empty. A body follows ordinary inline evaluation; a quoted argument
keeps directives literal.

`style` and `language` are strings. `preserve_space` is a boolean or a recognized
boolean string. When enabled, spaces and blank lines are retained, tabs use fixed
spacing, and CRLF/CR line endings normalize to LF. The shared `properties` rules
apply. The language label does not enable syntax highlighting. Direct `font`, `size`,
and `color` values are retained for native and presentation text; Word output
requires styles for those controls.

A fence of at least three backticks or tildes opens literal block content at a
block boundary. A matching fence of exactly the same length closes it; the
closing line may contain indentation and trailing horizontal whitespace. The
first word after the opening fence is the language label. The body is literal,
including its final line ending, and requests preserved whitespace. An unmatched
fence falls back to ordinary markup parsing.

## Dividers

`divider` has no positional argument or body. It accepts `style` as an output style
hint, a normalized `color`, a non-negative `width` length, a `spacing` object of
lengths and the shared `properties` object. `spacing.before` and `spacing.after`
control space around the rule. Native zero-width dividers clamp to one point and
warn; editable Word has its own minimum border width.

A standalone run of at least three hyphens, asterisks or underscores is the
Markdown form. The run is contiguous; trailing horizontal whitespace is allowed.
Underlined headings take precedence when a preceding eligible text line exists.
At the top level of a presentation, dividers separate slides. Inside an explicit
slide's text content, they produce the supported text-rule fallback. Document
layout paints a horizontal rule without changing layout mode.

## Definition lists

`definition_list` requires one body containing only `term` and `definition`
elements and separating whitespace. Child order is preserved; alternating pairs
are not required. Empty lists are allowed and report a warning. The container
accepts a string `style` hint and the shared `properties` object.

`term` requires one inline body. `definition` requires one block-content body,
with ordinary text normalized into paragraphs. Neither accepts positional
arguments. A definition accepts a string `style`; a term retains additional
attributes without dedicated outer type checks. Both retain `properties`.
Their `style` selects a paragraph style in the supported list output. Default
term text is bold; default definitions are indented in document layout.

Independent blocks nested inside definitions retain their model structure, but
native and presentation output maps each term or definition to one paragraph.
Word also preserves separate definition paragraphs inside table cells. Output
properties without an implemented effect must report diagnostics. These forms do
not establish full Markdown definition-list syntax.

## Examples

```mote
~~~text
  #paragraph[This stays literal.]
~~~

#raw("  Keep these spaces", preserve_space = true)
```

```mote
#definition_list[
  #term[Template]
  #definition[A reusable source document.]
  #term[Directive]
  #definition[A named instruction with arguments and optional content.]
]

---

#paragraph[Content after the divider.]
```
