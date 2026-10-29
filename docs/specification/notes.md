# Footnotes and endnotes

[Specification index](README.md)

A **note** has an inline reference in the surrounding text and its own block
content. A footnote belongs at the bottom of the reference page; an endnote
belongs at the end of the document. Each kind has an independent numbering
sequence. Authored `id` metadata does not select, reuse, or restart that sequence.

## Directives

`#footnote[content]` and `#endnote[content]` require exactly one body, which may be
empty. Their bodies accept the block content already defined in earlier chapters.
Only named arguments are accepted. The optional `id` is a string or number.
`properties` applies the established attribute override rules; extra fields remain
metadata and do not establish note formatting or output controls.

```mote
#paragraph[
  The result is provisional#footnote(id = "method")[The sample is small.].
  See the archive#endnote[Archive catalogue, volume 2.].
]
```

The paragraph contains two inline note elements, each with a paragraph body.
The first is a footnote with `id = "method"`; the second is an endnote.
Notes in notes or running stories retain their model structure, but outputs may
need visible fallbacks because those placements do not have ordinary note semantics.

## Markdown footnotes

`[^name]` inserts a footnote. A definition at the beginning of a line has the form
`[^name]: content`; its body extends to the end of that line. Definitions may follow
references. They do not produce body text and are evaluated only when first used.
The last definition of a name wins. A later reference reuses the evaluated body,
while producing another inline note element. A missing definition produces the
name as visible note content. Direct or indirect recursive definitions are errors.

```mote
A measured result[^method].

[^method]: Three independent samples.
```

This produces one paragraph containing a footnote with `id = "method"`, whose body
is the paragraph “Three independent samples.” The definition itself is absent
from the document body.

Output support and limits are recorded in the [footnote](../checklists/footnote.md)
and [endnote](../checklists/endnote.md) checklists.
