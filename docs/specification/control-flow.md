# Conditions and iteration

[Specification](README.md) · [Conditions](../checklists/if.md) · [Iteration](../checklists/for.md)

A **condition** selects content. `#if(value)[then content]` evaluates its value and
includes the body when that value is true in a conditional context. The optional
`else [alternative content]` supplies the other branch. A newline may precede
`else`; the bracketed body follows it on the same line. Only the selected branch
is evaluated, and it receives a child scope. No selected branch means no content.

The following values are false in this context: nil, false, zero, an empty string,
an empty array or object, and a zero length. All other currently supported values
are true. In particular, `"false"` is a nonempty string and therefore true.
These are Mote's conditional rules, independent of a scripting language's rules.

```mote
#let(paid=false)
#paragraph[Status: #if(paid)[Paid] else [Awaiting payment].]
```

Branches preserve the surrounding content mode. An inline branch may contain
inline directives; it cannot insert a paragraph or another block element.
A block branch can contain flowing text, paragraph breaks, and block directives.
Branch syntax itself does not force a new paragraph. Empty bodies are allowed.

An **iteration** repeats a body for values in a collection:
`#for name in expression {content}`. The expression must produce an array or an
object. Arrays visit values in order. Objects visit values in ascending key order;
the loop variable contains the value, not the key. Empty collections produce no
content. Scalars, including nil and strings, are not iterable.

```mote
#let(items={{name="Analysis"},{name="Review"}})
#list[#for item in items {#item[#item.name]}]
```

The braces contain Mote markup, not a script. Put whitespace between the iterable
expression and its opening brace. Each iteration gets a fresh child scope with
the named loop binding. It can see outer values, and nested loops can see the
outer loop's binding. Rebinding a name inside an iteration does not change its
outer value or the next iteration. Helper/directive names remain separate from
value names, as the `item` example demonstrates.

In block context each iteration's body is normalized into blocks separately. In
inline context iterations remain inline:

```mote
#paragraph[Values: #for value in {1,2,3} {#strong[#value] }done.]
```

Conditions and iterations can generate existing rows, cells, list items, and note
bodies. Their output must still satisfy the receiving directive's body rules.
Definitions in an unselected branch are not evaluated; syntax is parsed in all
branches, so an unselected branch must still have valid syntax.

Document declarations retain their document-wide effects. A current section
limitation is that sections generated within control flow also return their body
as ordinary surrounding content. That content can therefore appear both in the
explicit sections and in a later implicit section. Keep section declarations at
the document's top level when this duplicate content is unwanted.

These forms use the static expression subset introduced in earlier chapters.
Arithmetic, comparisons, and custom function definitions are separate scripting
capabilities; they are not implied by conditions or iteration.
