# `lua_tocfunction`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-lua-rawseti.md) · [Next](015-lua-debug.md)

<a id="lua_tocfunction"></a>

### `lua_tocfunction`

[-0, +0, *-*]



```
lua_CFunction lua_tocfunction (lua_State *L, int index);
```




Converts a value at the given acceptable index to a C function.
That value must be a C function;
otherwise, returns `NULL`.

---

<a id="lua_tointeger"></a>

### `lua_tointeger`

[-0, +0, *-*]



```
lua_Integer lua_tointeger (lua_State *L, int index);
```




Converts the Lua value at the given acceptable index
to the signed integral type [`lua_Integer`](010-lua-equal.md#lua_Integer).
The Lua value must be a number or a string convertible to a number
(see [§2.2.1](002-2-2-1-coercion.md#2.2.1));
otherwise, [`lua_tointeger`](014-lua-tocfunction.md#lua_tointeger) returns 0.

If the number is not an integer,
it is truncated in some non-specified way.

---

<a id="lua_tolstring"></a>

### `lua_tolstring`

[-0, +0, *m*]



```
const char *lua_tolstring (lua_State *L, int index, size_t *len);
```




Converts the Lua value at the given acceptable index to a C string.
If `len` is not `NULL`,
it also sets `*len` with the string length.
The Lua value must be a string or a number;
otherwise, the function returns `NULL`.
If the value is a number,
then [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) also
*changes the actual value in the stack to a string*.
(This change confuses [`lua_next`](011-lua-istable.md#lua_next)
when [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) is applied to keys during a table traversal.)

[`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) returns a fully aligned pointer
to a string inside the Lua state.
This string always has a zero ('`\0`')
after its last character (as in C),
but can contain other zeros in its body.
Because Lua has garbage collection,
there is no guarantee that the pointer returned by [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring)
will be valid after the corresponding value is removed from the stack.

---

<a id="lua_tonumber"></a>

### `lua_tonumber`

[-0, +0, *-*]



```
lua_Number lua_tonumber (lua_State *L, int index);
```




Converts the Lua value at the given acceptable index
to the C type [`lua_Number`](011-lua-istable.md#lua_Number) (see [`lua_Number`](011-lua-istable.md#lua_Number)).
The Lua value must be a number or a string convertible to a number
(see [§2.2.1](002-2-2-1-coercion.md#2.2.1));
otherwise, [`lua_tonumber`](014-lua-tocfunction.md#lua_tonumber) returns 0.

---

<a id="lua_topointer"></a>

### `lua_topointer`

[-0, +0, *-*]



```
const void *lua_topointer (lua_State *L, int index);
```




Converts the value at the given acceptable index to a generic
C pointer (`void*`).
The value can be a userdata, a table, a thread, or a function;
otherwise, [`lua_topointer`](014-lua-tocfunction.md#lua_topointer) returns `NULL`.
Different objects will give different pointers.
There is no way to convert the pointer back to its original value.

Typically this function is used only for debug information.

---

<a id="lua_tostring"></a>

### `lua_tostring`

[-0, +0, *m*]



```
const char *lua_tostring (lua_State *L, int index);
```




Equivalent to [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) with `len` equal to `NULL`.

---

<a id="lua_tothread"></a>

### `lua_tothread`

[-0, +0, *-*]



```
lua_State *lua_tothread (lua_State *L, int index);
```




Converts the value at the given acceptable index to a Lua thread
(represented as `lua_State*`).
This value must be a thread;
otherwise, the function returns `NULL`.

---

<a id="lua_touserdata"></a>

### `lua_touserdata`

[-0, +0, *-*]



```
void *lua_touserdata (lua_State *L, int index);
```




If the value at the given acceptable index is a full userdata,
returns its block address.
If the value is a light userdata,
returns its pointer.
Otherwise, returns `NULL`.

---

<a id="lua_type"></a>

### `lua_type`

[-0, +0, *-*]



```
int lua_type (lua_State *L, int index);
```




Returns the type of the value in the given acceptable index,
or `LUA_TNONE` for a non-valid index
(that is, an index to an "empty" stack position).
The types returned by [`lua_type`](014-lua-tocfunction.md#lua_type) are coded by the following constants
defined in `lua.h`:
`LUA_TNIL`,
`LUA_TNUMBER`,
`LUA_TBOOLEAN`,
`LUA_TSTRING`,
`LUA_TTABLE`,
`LUA_TFUNCTION`,
`LUA_TUSERDATA`,
`LUA_TTHREAD`,
and
`LUA_TLIGHTUSERDATA`.

---

<a id="lua_typename"></a>

### `lua_typename`

[-0, +0, *-*]



```
const char *lua_typename  (lua_State *L, int tp);
```




Returns the name of the type encoded by the value `tp`,
which must be one the values returned by [`lua_type`](014-lua-tocfunction.md#lua_type).

---

<a id="lua_Writer"></a>

### `lua_Writer`



```
typedef int (*lua_Writer) (lua_State *L,
                           const void* p,
                           size_t sz,
                           void* ud);
```




The type of the writer function used by [`lua_dump`](009-lua-atpanic.md#lua_dump).
Every time it produces another piece of chunk,
[`lua_dump`](009-lua-atpanic.md#lua_dump) calls the writer,
passing along the buffer to be written (`p`),
its size (`sz`),
and the `data` parameter supplied to [`lua_dump`](009-lua-atpanic.md#lua_dump).

The writer returns an error code:
0 means no errors;
any other value means an error and stops [`lua_dump`](009-lua-atpanic.md#lua_dump) from
calling the writer again.

---

<a id="lua_xmove"></a>

### `lua_xmove`

[-?, +?, *-*]



```
void lua_xmove (lua_State *from, lua_State *to, int n);
```




Exchange values between different threads of the *same* global state.

This function pops `n` values from the stack `from`,
and pushes them onto the stack `to`.

---

<a id="lua_yield"></a>

### `lua_yield`

[-?, +?, *-*]



```
int lua_yield  (lua_State *L, int nresults);
```




Yields a coroutine.

This function should only be called as the
return expression of a C function, as follows:



```

     return lua_yield (L, nresults);
```




When a C function calls [`lua_yield`](014-lua-tocfunction.md#lua_yield) in that way,
the running coroutine suspends its execution,
and the call to [`lua_resume`](013-lua-rawseti.md#lua_resume) that started this coroutine returns.
The parameter `nresults` is the number of values from the stack
that are passed as results to [`lua_resume`](013-lua-rawseti.md#lua_resume).

<a id="3.8"></a>

## 3.8 –  The Debug Interface

Lua has no built-in debugging facilities.
Instead, it offers a special interface
by means of functions and *hooks*.
This interface allows the construction of different
kinds of debuggers, profilers, and other tools
that need "inside information" from the interpreter.

---

