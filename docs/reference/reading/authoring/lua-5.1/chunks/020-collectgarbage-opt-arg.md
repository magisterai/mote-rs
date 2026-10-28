# `collectgarbage (\[opt \[, arg\]\])`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](019-lual-pushresult.md) · [Next](021-rawequal-v1-v2.md)

<a id="pdf-collectgarbage"></a>

### `collectgarbage ([opt [, arg]])`

This function is a generic interface to the garbage collector.
It performs different functions according to its first argument, `opt`:

- **"collect":**
  performs a full garbage-collection cycle.
  This is the default option.
- **"stop":**
  stops the garbage collector.
- **"restart":**
  restarts the garbage collector.
- **"count":**
  returns the total memory in use by Lua (in Kbytes).
- **"step":**
  performs a garbage-collection step.
  The step "size" is controlled by `arg`
  (larger values mean more steps) in a non-specified way.
  If you want to control the step size
  you must experimentally tune the value of `arg`.
  Returns **true** if the step finished a collection cycle.
- **"setpause":**
  sets `arg` as the new value for the *pause* of
  the collector (see [§2.10](006-2-9-environments.md#2.10)).
  Returns the previous value for *pause*.
- **"setstepmul":**
  sets `arg` as the new value for the *step multiplier* of
  the collector (see [§2.10](006-2-9-environments.md#2.10)).
  Returns the previous value for *step*.

---

<a id="pdf-dofile"></a>

### `dofile ([filename])`

Opens the named file and executes its contents as a Lua chunk.
When called without arguments,
`dofile` executes the contents of the standard input (`stdin`).
Returns all values returned by the chunk.
In case of errors, `dofile` propagates the error
to its caller (that is, `dofile` does not run in protected mode).

---

<a id="pdf-error"></a>

### `error (message [, level])`

Terminates the last protected function called
and returns `message` as the error message.
Function `error` never returns.

Usually, `error` adds some information about the error position
at the beginning of the message.
The `level` argument specifies how to get the error position.
With level 1 (the default), the error position is where the
`error` function was called.
Level 2 points the error to where the function
that called `error` was called; and so on.
Passing a level 0 avoids the addition of error position information
to the message.

---

<a id="pdf-_G"></a>

### `_G`

A global variable (not a function) that
holds the global environment (that is, `_G._G = _G`).
Lua itself does not use this variable;
changing its value does not affect any environment,
nor vice-versa.
(Use [`setfenv`](021-rawequal-v1-v2.md#pdf-setfenv) to change environments.)

---

<a id="pdf-getfenv"></a>

### `getfenv ([f])`

Returns the current environment in use by the function.
`f` can be a Lua function or a number
that specifies the function at that stack level:
Level 1 is the function calling `getfenv`.
If the given function is not a Lua function,
or if `f` is 0,
`getfenv` returns the global environment.
The default for `f` is 1.

---

<a id="pdf-getmetatable"></a>

### `getmetatable (object)`

If `object` does not have a metatable, returns **nil**.
Otherwise,
if the object's metatable has a `"__metatable"` field,
returns the associated value.
Otherwise, returns the metatable of the given object.

---

<a id="pdf-ipairs"></a>

### `ipairs (t)`

Returns three values: an iterator function, the table `t`, and 0,
so that the construction



```

     for i,v in ipairs(t) do body end
```




will iterate over the pairs (`1,t[1]`), (`2,t[2]`), ···,
up to the first integer key absent from the table.

---

<a id="pdf-load"></a>

### `load (func [, chunkname])`

Loads a chunk using function `func` to get its pieces.
Each call to `func` must return a string that concatenates
with previous results.
A return of an empty string, **nil**, or no value signals the end of the chunk.

If there are no errors,
returns the compiled chunk as a function;
otherwise, returns **nil** plus the error message.
The environment of the returned function is the global environment.

`chunkname` is used as the chunk name for error messages
and debug information.
When absent,
it defaults to "`=(load)`".

---

<a id="pdf-loadfile"></a>

### `loadfile ([filename])`

Similar to [`load`](020-collectgarbage-opt-arg.md#pdf-load),
but gets the chunk from file `filename`
or from the standard input,
if no file name is given.

---

<a id="pdf-loadstring"></a>

### `loadstring (string [, chunkname])`

Similar to [`load`](020-collectgarbage-opt-arg.md#pdf-load),
but gets the chunk from the given string.

To load and run a given string, use the idiom



```

     assert(loadstring(s))()
```




When absent,
`chunkname` defaults to the given string.

---

<a id="pdf-next"></a>

### `next (table [, index])`

Allows a program to traverse all fields of a table.
Its first argument is a table and its second argument
is an index in this table.
`next` returns the next index of the table
and its associated value.
When called with **nil** as its second argument,
`next` returns an initial index
and its associated value.
When called with the last index,
or with **nil** in an empty table,
`next` returns **nil**.
If the second argument is absent, then it is interpreted as **nil**.
In particular,
you can use `next(t)` to check whether a table is empty.

The order in which the indices are enumerated is not specified,
*even for numeric indices*.
(To traverse a table in numeric order,
use a numerical **for** or the [`ipairs`](020-collectgarbage-opt-arg.md#pdf-ipairs) function.)

The behavior of `next` is *undefined* if,
during the traversal,
you assign any value to a non-existent field in the table.
You may however modify existing fields.
In particular, you may clear existing fields.

---

<a id="pdf-pairs"></a>

### `pairs (t)`

Returns three values: the [`next`](020-collectgarbage-opt-arg.md#pdf-next) function, the table `t`, and **nil**,
so that the construction



```

     for k,v in pairs(t) do body end
```




will iterate over all key–value pairs of table `t`.

See function [`next`](020-collectgarbage-opt-arg.md#pdf-next) for the caveats of modifying
the table during its traversal.

---

<a id="pdf-pcall"></a>

### `pcall (f, arg1, ···)`

Calls function `f` with
the given arguments in *protected mode*.
This means that any error inside `f` is not propagated;
instead, `pcall` catches the error
and returns a status code.
Its first result is the status code (a boolean),
which is true if the call succeeds without errors.
In such case, `pcall` also returns all results from the call,
after this first result.
In case of any error, `pcall` returns **false** plus the error message.

---

<a id="pdf-print"></a>

### `print (···)`

Receives any number of arguments,
and prints their values to `stdout`,
using the [`tostring`](021-rawequal-v1-v2.md#pdf-tostring) function to convert them to strings.
`print` is not intended for formatted output,
but only as a quick way to show a value,
typically for debugging.
For formatted output, use [`string.format`](023-package-path.md#pdf-string.format).

---

