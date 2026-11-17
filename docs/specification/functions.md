# Functions

[Specification index](README.md) · Previous: [Lua expressions](lua-expressions.md)

A function gives a name to reusable Lua code. Function definitions require Lua
execution to be enabled, just like [Lua expressions](lua-expressions.md).

Write `#function`, a name, a parenthesized parameter list, then Lua statements
ending with `end`. A `return` statement supplies the result. Parameters have no
inline defaults; an omitted argument is `nil`, which the body can handle.

```mote
#function greeting(name, suffix)
  if suffix == nil then suffix = "!" end
  return "Hello " .. name .. suffix
end
#greeting("Ada")

#greeting(suffix = ".", name = "Lin")
```

Calls accept positional or named arguments. A parameter cannot be supplied both
ways. Unknown names, extra positional arguments, and multiple content bodies are
errors. Repeating a named argument keeps its last value. A function may be called
from a Lua expression too; Lua calls pass positional values.

## Content parameters and returns

A final parameter named `body` receives an optional bracketed content body. It is
separate from positional and named arguments. A function without that final
parameter does not accept a body. The body is evaluated in a child lexical scope,
retaining inline and block content. In a preformatted context it preserves
whitespace.

A return value can be text, a number, a boolean, a typed length or color, an
object, an array, or document content. `nil` inserts nothing.

A function result is a value, and [data results](#data-results) describes an
array result that is used as one.

A function called in content inserts its result. An array result is there a
sequence of content: nested arrays flatten, strings and numbers become text, and
existing elements retain their structure and attributes.

```mote
#function surround(prefix, body)
  return {prefix, body, " / complete"}
end
#surround("Start: ")[**Important** text.]
```

A reference to a binding that holds an array result inserts the same sequence
if the array holds document content at any depth. An array of data alone is
inserted as every array value is, as the text of its entries separated by a
comma and a space.

The resulting content follows the surrounding directive's rules. Returning a
heading inside a paragraph is an error because paragraphs accept inline content.
Functions use the first returned Lua value; an ordinary expression with multiple
results uses the last. Returning an ordinary object inserts its textual value.

Returning an empty content body inserts no content. Tagged content and element
children preserve their empty sequence identity through Lua. An ordinary empty
Lua table remains an object and inserts `{}`; return `nil` when no value is wanted.

## Lexical definitions

A function captures the values and functions visible when it is defined. Later
bindings do not replace that snapshot. Function names and value names occupy
separate namespaces: `#name` reads a value; `#name(...)` calls a function. A local
function definition leaves scope with its enclosing body. A definition captures
its environment before its own name is installed, so self-recursion is not
provided by the declaration.

```mote
#let(label = "Draft")
#function status(name)
  return label .. ": " .. name
end
#let(label = "Final")
#status("Report")
```

This produces `Draft: Report`.

The Lua runtime is shared across a compilation. A function call synchronizes
Lua globals to the callee's captured values. The caller receives its own globals
again when the call returns or fails, whether the caller is an expression
outside any function body or the body of another function. Every value and
function visible at an expression therefore remains visible throughout it, and
every value and function captured by a function remains visible throughout its
body, whatever functions are called and in whatever order.

```mote
#let(label = "Draft")
#function status(name)
  return label .. ": " .. name
end
#let(label = "Final")
#function summary(name)
  return label .. " (" .. status(name) .. ") " .. label
end
#summary("Report")
```

This produces `Final (Draft: Report) Final`.

Function arguments are local Lua parameters and are unaffected by a call. A Lua
global that names no value or function visible to the caller or to the callee
is not synchronized, so an assignment to one persists after the function that
made it returns. Retained
function aliases continue to call the definition that created them.

## Data results

A function result is a value. A dense Lua array remains an array: its numbers
remain numbers, its strings remain strings, and its nested tables keep their
shape. An array result can therefore be bound with `let`, indexed, measured with
`#`, traversed by `for`, and given to a chart or to another function, exactly as
an array nested in a returned object can.

```mote
#function squares(count)
  local values = {}
  for index = 1, count do values[index] = index * index end
  return values
end
#let(values = squares(3), count = #values)
#for value in values {#paragraph[#value of #count]}
```

The example produces three paragraphs, `1 of 3`, `4 of 3` and `9 of 3`.

A result need not be bound before it is used: a function call within a larger
expression leaves the bindings of that expression visible, as
[lexical definitions](#lexical-definitions) states.

```mote
#function names()
  return { "A", "B" }
end
#let(values = { 1, 2 })
#let(data = { categories = names(), values = values })
#let(first = data.categories[1], last = data.values[2])
#paragraph[#first #last]
```

The example produces the paragraph `A 2`: the object `data` holds both
`categories` and `values`, although `values` was bound after `names` was
defined.

Functions share the Lua execution budget, restricted global environment, and
source-relative JSON resolver described in the preceding chapter. Their return
values feed the same model and output paths as ordinary directives. The next chapter defines the
[`document` constructor namespace](document-constructors.md).
