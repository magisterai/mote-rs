# `rawequal (v1, v2)`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](020-collectgarbage-opt-arg.md) · [Next](022-module-name.md)

<a id="pdf-rawequal"></a>

### `rawequal (v1, v2)`

Checks whether `v1` is equal to `v2`,
without invoking any metamethod.
Returns a boolean.

---

<a id="pdf-rawget"></a>

### `rawget (table, index)`

Gets the real value of `table[index]`,
without invoking any metamethod.
`table` must be a table;
`index` may be any value.

---

<a id="pdf-rawset"></a>

### `rawset (table, index, value)`

Sets the real value of `table[index]` to `value`,
without invoking any metamethod.
`table` must be a table,
`index` any value different from **nil**,
and `value` any Lua value.

This function returns `table`.

---

<a id="pdf-select"></a>

### `select (index, ···)`

If `index` is a number,
returns all arguments after argument number `index`.
Otherwise, `index` must be the string `"#"`,
and `select` returns the total number of extra arguments it received.

---

<a id="pdf-setfenv"></a>

### `setfenv (f, table)`

Sets the environment to be used by the given function.
`f` can be a Lua function or a number
that specifies the function at that stack level:
Level 1 is the function calling `setfenv`.
`setfenv` returns the given function.

As a special case, when `f` is 0 `setfenv` changes
the environment of the running thread.
In this case, `setfenv` returns no values.

---

<a id="pdf-setmetatable"></a>

### `setmetatable (table, metatable)`

Sets the metatable for the given table.
(You cannot change the metatable of other types from Lua, only from C.)
If `metatable` is **nil**,
removes the metatable of the given table.
If the original metatable has a `"__metatable"` field,
raises an error.

This function returns `table`.

---

<a id="pdf-tonumber"></a>

### `tonumber (e [, base])`

Tries to convert its argument to a number.
If the argument is already a number or a string convertible
to a number, then `tonumber` returns this number;
otherwise, it returns **nil**.

An optional argument specifies the base to interpret the numeral.
The base may be any integer between 2 and 36, inclusive.
In bases above 10, the letter '`A`' (in either upper or lower case)
represents 10, '`B`' represents 11, and so forth,
with '`Z`' representing 35.
In base 10 (the default), the number can have a decimal part,
as well as an optional exponent part (see [§2.1](001-lua-lua-5-1-reference-manual.md#2.1)).
In other bases, only unsigned integers are accepted.

---

<a id="pdf-tostring"></a>

### `tostring (e)`

Receives an argument of any type and
converts it to a string in a reasonable format.
For complete control of how numbers are converted,
use [`string.format`](023-package-path.md#pdf-string.format).

If the metatable of `e` has a `"__tostring"` field,
then `tostring` calls the corresponding value
with `e` as argument,
and uses the result of the call as its result.

---

<a id="pdf-type"></a>

### `type (v)`

Returns the type of its only argument, coded as a string.
The possible results of this function are
"`nil`" (a string, not the value **nil**),
"`number`",
"`string`",
"`boolean`",
"`table`",
"`function`",
"`thread`",
and "`userdata`".

---

<a id="pdf-unpack"></a>

### `unpack (list [, i [, j]])`

Returns the elements from the given table.
This function is equivalent to



```

     return list[i], list[i+1], ···, list[j]
```




except that the above code can be written only for a fixed number
of elements.
By default, `i` is 1 and `j` is the length of the list,
as defined by the length operator (see [§2.5.5](004-2-5-1-arithmetic-operators.md#2.5.5)).

---

<a id="pdf-_VERSION"></a>

### `_VERSION`

A global variable (not a function) that
holds a string containing the current interpreter version.
The current contents of this variable is "`Lua 5.1`".

---

<a id="pdf-xpcall"></a>

### `xpcall (f, err)`

This function is similar to [`pcall`](020-collectgarbage-opt-arg.md#pdf-pcall),
except that you can set a new error handler.

`xpcall` calls function `f` in protected mode,
using `err` as the error handler.
Any error inside `f` is not propagated;
instead, `xpcall` catches the error,
calls the `err` function with the original error object,
and returns a status code.
Its first result is the status code (a boolean),
which is true if the call succeeds without errors.
In this case, `xpcall` also returns all results from the call,
after this first result.
In case of any error,
`xpcall` returns **false** plus the result from `err`.

<a id="5.2"></a>

## 5.2 –  Coroutine Manipulation

<a id="pdf-coroutine"></a>

The operations related to coroutines comprise a sub-library of
the basic library and come inside the table `coroutine`.
See [§2.11](007-2-10-2-weak-tables.md#2.11) for a general description of coroutines.

---

<a id="pdf-coroutine.create"></a>

### `coroutine.create (f)`

Creates a new coroutine, with body `f`.
`f` must be a Lua function.
Returns this new coroutine,
an object with type `"thread"`.

---

<a id="pdf-coroutine.resume"></a>

### `coroutine.resume (co [, val1, ···])`

Starts or continues the execution of coroutine `co`.
The first time you resume a coroutine,
it starts running its body.
The values `val1`, ··· are passed
as the arguments to the body function.
If the coroutine has yielded,
`resume` restarts it;
the values `val1`, ··· are passed
as the results from the yield.

If the coroutine runs without any errors,
`resume` returns **true** plus any values passed to `yield`
(if the coroutine yields) or any values returned by the body function
(if the coroutine terminates).
If there is any error,
`resume` returns **false** plus the error message.

---

<a id="pdf-coroutine.running"></a>

### `coroutine.running ()`

Returns the running coroutine,
or **nil** when called by the main thread.

---

<a id="pdf-coroutine.status"></a>

### `coroutine.status (co)`

Returns the status of coroutine `co`, as a string:
`"running"`,
if the coroutine is running (that is, it called `status`);
`"suspended"`, if the coroutine is suspended in a call to `yield`,
or if it has not started running yet;
`"normal"` if the coroutine is active but not running
(that is, it has resumed another coroutine);
and `"dead"` if the coroutine has finished its body function,
or if it has stopped with an error.

---

<a id="pdf-coroutine.wrap"></a>

### `coroutine.wrap (f)`

Creates a new coroutine, with body `f`.
`f` must be a Lua function.
Returns a function that resumes the coroutine each time it is called.
Any arguments passed to the function behave as the
extra arguments to `resume`.
Returns the same values returned by `resume`,
except the first boolean.
In case of error, propagates the error.

---

<a id="pdf-coroutine.yield"></a>

### `coroutine.yield (···)`

Suspends the execution of the calling coroutine.
The coroutine cannot be running a C function,
a metamethod, or an iterator.
Any arguments to `yield` are passed as extra results to `resume`.

<a id="5.3"></a>

## 5.3 –  Modules

<a id="pdf-package"></a>

The package library provides basic
facilities for loading and building modules in Lua.
It exports two of its functions directly in the global environment:
[`require`](022-module-name.md#pdf-require) and [`module`](022-module-name.md#pdf-module).
Everything else is exported in a table `package`.

---

