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
object, or document content. `nil` inserts nothing. A dense Lua array returned by
a function becomes a sequence of content: nested arrays flatten, strings become
text, and existing elements retain their structure and attributes. This differs
from an ordinary array-valued expression, which retains its array value.

```mote
#function surround(prefix, body)
  return {prefix, body, " / complete"}
end
#surround("Start: ")[**Important** text.]
```

The resulting content follows the surrounding directive's rules. Returning a
heading inside a paragraph is an error because paragraphs accept inline content.
Functions use the first returned Lua value; an ordinary expression with multiple
results uses the last. Returning an ordinary object inserts its textual value.

Currently, returning an empty content body produces the text `{}` because its
empty node table follows the empty-table conversion rule. Avoid relying on an
empty body as an empty return; return `nil` explicitly when no content is wanted.

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

The Lua runtime is shared across a compilation. A nested function call currently
synchronizes Lua globals to the callee's captured values without restoring the
caller's globals afterward. Save any captured value needed after a nested call
in a Lua `local` before that call. Function arguments are local Lua parameters
and are unaffected. Retained function aliases continue to call the definition
that created them.

Functions share the Lua execution budget, restricted global environment, and
source-relative JSON resolver described in the preceding chapter. Their return
values feed the same model and output paths as ordinary directives. The next chapter defines the
[`document` constructor namespace](document-constructors.md).
