# `lua_Hook`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](015-lua-debug.md) · [Next](017-lual-argerror.md)

<a id="lua_Hook"></a>

### `lua_Hook`



```
typedef void (*lua_Hook) (lua_State *L, lua_Debug *ar);
```




Type for debugging hook functions.

<a id="pdf-LUA_HOOKCALL"></a>

<a id="pdf-LUA_HOOKRET"></a>

<a id="pdf-LUA_HOOKTAILRET"></a>

<a id="pdf-LUA_HOOKLINE"></a>

<a id="pdf-LUA_HOOKCOUNT"></a>

Whenever a hook is called, its `ar` argument has its field
`event` set to the specific event that triggered the hook.
Lua identifies these events with the following constants:
`LUA_HOOKCALL`, `LUA_HOOKRET`,
`LUA_HOOKTAILRET`, `LUA_HOOKLINE`,
and `LUA_HOOKCOUNT`.
Moreover, for line events, the field `currentline` is also set.
To get the value of any other field in `ar`,
the hook must call [`lua_getinfo`](015-lua-debug.md#lua_getinfo).
For return events, `event` can be `LUA_HOOKRET`,
the normal value, or `LUA_HOOKTAILRET`.
In the latter case, Lua is simulating a return from
a function that did a tail call;
in this case, it is useless to call [`lua_getinfo`](015-lua-debug.md#lua_getinfo).

While Lua is running a hook, it disables other calls to hooks.
Therefore, if a hook calls back Lua to execute a function or a chunk,
this execution occurs without any calls to hooks.

---

<a id="lua_sethook"></a>

### `lua_sethook`

[-0, +0, *-*]



```
int lua_sethook (lua_State *L, lua_Hook f, int mask, int count);
```




Sets the debugging hook function.

<a id="pdf-LUA_MASKCALL"></a>

<a id="pdf-LUA_MASKRET"></a>

<a id="pdf-LUA_MASKLINE"></a>

<a id="pdf-LUA_MASKCOUNT"></a>

Argument `f` is the hook function.
`mask` specifies on which events the hook will be called:
it is formed by a bitwise or of the constants
`LUA_MASKCALL`,
`LUA_MASKRET`,
`LUA_MASKLINE`,
and `LUA_MASKCOUNT`.
The `count` argument is only meaningful when the mask
includes `LUA_MASKCOUNT`.
For each event, the hook is called as explained below:

- **The call hook:** is called when the interpreter calls a function.
  The hook is called just after Lua enters the new function,
  before the function gets its arguments.
- **The return hook:** is called when the interpreter returns from a function.
  The hook is called just before Lua leaves the function.
  You have no access to the values to be returned by the function.
- **The line hook:** is called when the interpreter is about to
  start the execution of a new line of code,
  or when it jumps back in the code (even to the same line).
  (This event only happens while Lua is executing a Lua function.)
- **The count hook:** is called after the interpreter executes every
  `count` instructions.
  (This event only happens while Lua is executing a Lua function.)

A hook is disabled by setting `mask` to zero.

---

<a id="lua_setlocal"></a>

### `lua_setlocal`

[-(0|1), +0, *-*]



```
const char *lua_setlocal (lua_State *L, lua_Debug *ar, int n);
```




Sets the value of a local variable of a given activation record.
Parameters `ar` and `n` are as in [`lua_getlocal`](015-lua-debug.md#lua_getlocal)
(see [`lua_getlocal`](015-lua-debug.md#lua_getlocal)).
[`lua_setlocal`](016-lua-hook.md#lua_setlocal) assigns the value at the top of the stack
to the variable and returns its name.
It also pops the value from the stack.

Returns `NULL` (and pops nothing)
when the index is greater than
the number of active local variables.

---

<a id="lua_setupvalue"></a>

### `lua_setupvalue`

[-(0|1), +0, *-*]



```
const char *lua_setupvalue (lua_State *L, int funcindex, int n);
```




Sets the value of a closure's upvalue.
It assigns the value at the top of the stack
to the upvalue and returns its name.
It also pops the value from the stack.
Parameters `funcindex` and `n` are as in the [`lua_getupvalue`](015-lua-debug.md#lua_getupvalue)
(see [`lua_getupvalue`](015-lua-debug.md#lua_getupvalue)).

Returns `NULL` (and pops nothing)
when the index is greater than the number of upvalues.

<a id="4"></a>

# 4 –  The Auxiliary Library

The *auxiliary library* provides several convenient functions
to interface C with Lua.
While the basic API provides the primitive functions for all
interactions between C and Lua,
the auxiliary library provides higher-level functions for some
common tasks.

All functions from the auxiliary library
are defined in header file `lauxlib.h` and
have a prefix `luaL_`.

All functions in the auxiliary library are built on
top of the basic API,
and so they provide nothing that cannot be done with this API.

Several functions in the auxiliary library are used to
check C function arguments.
Their names are always `luaL_check*` or `luaL_opt*`.
All of these functions throw an error if the check is not satisfied.
Because the error message is formatted for arguments
(e.g., "`bad argument #1`"),
you should not use these functions for other stack values.

<a id="4.1"></a>

## 4.1 –  Functions and Types

Here we list all functions and types from the auxiliary library
in alphabetical order.

---

<a id="luaL_addchar"></a>

### `luaL_addchar`

[-0, +0, *m*]



```
void luaL_addchar (luaL_Buffer *B, char c);
```




Adds the character `c` to the buffer `B`
(see [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer)).

---

<a id="luaL_addlstring"></a>

### `luaL_addlstring`

[-0, +0, *m*]



```
void luaL_addlstring (luaL_Buffer *B, const char *s, size_t l);
```




Adds the string pointed to by `s` with length `l` to
the buffer `B`
(see [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer)).
The string may contain embedded zeros.

---

<a id="luaL_addsize"></a>

### `luaL_addsize`

[-0, +0, *m*]



```
void luaL_addsize (luaL_Buffer *B, size_t n);
```




Adds to the buffer `B` (see [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer))
a string of length `n` previously copied to the
buffer area (see [`luaL_prepbuffer`](018-lual-error.md#luaL_prepbuffer)).

---

<a id="luaL_addstring"></a>

### `luaL_addstring`

[-0, +0, *m*]



```
void luaL_addstring (luaL_Buffer *B, const char *s);
```




Adds the zero-terminated string pointed to by `s`
to the buffer `B`
(see [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer)).
The string may not contain embedded zeros.

---

<a id="luaL_addvalue"></a>

### `luaL_addvalue`

[-1, +0, *m*]



```
void luaL_addvalue (luaL_Buffer *B);
```




Adds the value at the top of the stack
to the buffer `B`
(see [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer)).
Pops the value.

This is the only function on string buffers that can (and must)
be called with an extra element on the stack,
which is the value to be added to the buffer.

---

<a id="luaL_argcheck"></a>

### `luaL_argcheck`

[-0, +0, *v*]



```
void luaL_argcheck (lua_State *L,
                    int cond,
                    int narg,
                    const char *extramsg);
```




Checks whether `cond` is true.
If not, raises an error with the following message,
where `func` is retrieved from the call stack:



```

     bad argument #<narg> to <func> (<extramsg>)
```




---

