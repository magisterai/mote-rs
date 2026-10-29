# Bindings and property access

[Specification](README.md) · [Checklist](../checklists/let.md)

`#let(name=value, ...)` evaluates named values in source order. It produces no
content and accepts no body. An empty argument list is allowed. Each value is
available to later bindings, including later arguments in the same declaration.
Repeating a name replaces its value in the current scope. A forward reference to
an unknown name is an error.

Values keep their types: strings, numbers, booleans, nil, lengths, colors,
property objects, and arrays. They can supply directive attributes or helper
arguments. `#name` inserts a value as text. Nil inserts no content; at block level,
a declaration or nil value also consumes the next whitespace-only source node.
Objects and arrays use their value text representation; use a property path when
only a particular field should be displayed.

```mote
#let(title="Quarterly report", size=pt(12), shade=color("#123456"))
#heading[#title]
#paragraph[#span(size=size, color=shade)[Revenue]]
```

A **scope** is the set of bindings visible at a position. An explicit directive
body creates a child scope: it can read outer values, but declarations within it
remain local. After the body, the outer value is visible again. Attribute values
are evaluated in the surrounding scope before the body begins. Declarations
such as `document`, `font`, and `style` still affect document state; lexical value
scope does not turn them into local formatting defaults.

```mote
#let(name="Outer")
#paragraph[#let(name="Inner")#name]
#paragraph[#name]
```

Markdown headings, emphasis, strong text, lists, quotes, and table cells share
the surrounding scope. Their syntax does not create an explicit directive body.
A Markdown note definition has a child scope and resolves its values at its first
reference; subsequent references reuse that evaluated note content.

A dotted **property path** selects nested object fields. In an expression,
an array segment is a zero-based index: `rows.0.name` selects the first row's
`name`. Missing names, missing properties, out-of-range indices, and traversal
through a scalar value are errors with source locations.

```mote
#let(report={client={name="Acme"}, totals={10,20}}, second=report.totals.1)
#paragraph[#report.client.name: #second.]
```

Content references consume identifier segments only. The final period in
`#report.client.name.` is sentence punctuation. A numeric segment is not consumed
in content: `#rows.0` inserts `rows` followed by literal `.0`. Bind the indexed
value in an expression first, then insert its name as above. Bracket indexing,
operators, and arbitrary expression interpolation are not part of this static
expression subset.

Helper names and value names occupy separate namespaces. A binding named `upper`
does not replace the `upper(...)` helper. Prefer descriptive names over `true`,
`false`, and `nil`: a direct reference can resolve bindings with those names,
while those tokens remain literals within a compound static expression.

Bindings are evaluation state. They do not appear as declarations in the emitted
document model; only the resulting values and content do.
