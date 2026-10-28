# `luaL_argerror`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-lua-hook.md) · [Next](018-lual-error.md)

<a id="luaL_argerror"></a>

### `luaL_argerror`

[-0, +0, *v*]



```
int luaL_argerror (lua_State *L, int narg, const char *extramsg);
```




Raises an error with the following message,
where `func` is retrieved from the call stack:



```

     bad argument #<narg> to <func> (<extramsg>)
```




This function never returns,
but it is an idiom to use it in C functions
as `return luaL_argerror(args)`.

---

<a id="luaL_Buffer"></a>

### `luaL_Buffer`



```
typedef struct luaL_Buffer luaL_Buffer;
```




Type for a *string buffer*.

A string buffer allows C code to build Lua strings piecemeal.
Its pattern of use is as follows:

- First you declare a variable `b` of type [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer).
- Then you initialize it with a call `luaL_buffinit(L, &b)`.
- Then you add string pieces to the buffer calling any of
  the `luaL_add*` functions.
- You finish by calling `luaL_pushresult(&b)`.
  This call leaves the final string on the top of the stack.

During its normal operation,
a string buffer uses a variable number of stack slots.
So, while using a buffer, you cannot assume that you know where
the top of the stack is.
You can use the stack between successive calls to buffer operations
as long as that use is balanced;
that is,
when you call a buffer operation,
the stack is at the same level
it was immediately after the previous buffer operation.
(The only exception to this rule is [`luaL_addvalue`](016-lua-hook.md#luaL_addvalue).)
After calling [`luaL_pushresult`](019-lual-pushresult.md#luaL_pushresult) the stack is back to its
level when the buffer was initialized,
plus the final string on its top.

---

<a id="luaL_buffinit"></a>

### `luaL_buffinit`

[-0, +0, *-*]



```
void luaL_buffinit (lua_State *L, luaL_Buffer *B);
```




Initializes a buffer `B`.
This function does not allocate any space;
the buffer must be declared as a variable
(see [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer)).

---

<a id="luaL_callmeta"></a>

### `luaL_callmeta`

[-0, +(0|1), *e*]



```
int luaL_callmeta (lua_State *L, int obj, const char *e);
```




Calls a metamethod.

If the object at index `obj` has a metatable and this
metatable has a field `e`,
this function calls this field and passes the object as its only argument.
In this case this function returns 1 and pushes onto the
stack the value returned by the call.
If there is no metatable or no metamethod,
this function returns 0 (without pushing any value on the stack).

---

<a id="luaL_checkany"></a>

### `luaL_checkany`

[-0, +0, *v*]



```
void luaL_checkany (lua_State *L, int narg);
```




Checks whether the function has an argument
of any type (including **nil**) at position `narg`.

---

<a id="luaL_checkint"></a>

### `luaL_checkint`

[-0, +0, *v*]



```
int luaL_checkint (lua_State *L, int narg);
```




Checks whether the function argument `narg` is a number
and returns this number cast to an `int`.

---

<a id="luaL_checkinteger"></a>

### `luaL_checkinteger`

[-0, +0, *v*]



```
lua_Integer luaL_checkinteger (lua_State *L, int narg);
```




Checks whether the function argument `narg` is a number
and returns this number cast to a [`lua_Integer`](010-lua-equal.md#lua_Integer).

---

<a id="luaL_checklong"></a>

### `luaL_checklong`

[-0, +0, *v*]



```
long luaL_checklong (lua_State *L, int narg);
```




Checks whether the function argument `narg` is a number
and returns this number cast to a `long`.

---

<a id="luaL_checklstring"></a>

### `luaL_checklstring`

[-0, +0, *v*]



```
const char *luaL_checklstring (lua_State *L, int narg, size_t *l);
```




Checks whether the function argument `narg` is a string
and returns this string;
if `l` is not `NULL` fills `*l`
with the string's length.

This function uses [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) to get its result,
so all conversions and caveats of that function apply here.

---

<a id="luaL_checknumber"></a>

### `luaL_checknumber`

[-0, +0, *v*]



```
lua_Number luaL_checknumber (lua_State *L, int narg);
```




Checks whether the function argument `narg` is a number
and returns this number.

---

<a id="luaL_checkoption"></a>

### `luaL_checkoption`

[-0, +0, *v*]



```
int luaL_checkoption (lua_State *L,
                      int narg,
                      const char *def,
                      const char *const lst[]);
```




Checks whether the function argument `narg` is a string and
searches for this string in the array `lst`
(which must be NULL-terminated).
Returns the index in the array where the string was found.
Raises an error if the argument is not a string or
if the string cannot be found.

If `def` is not `NULL`,
the function uses `def` as a default value when
there is no argument `narg` or if this argument is **nil**.

This is a useful function for mapping strings to C enums.
(The usual convention in Lua libraries is
to use strings instead of numbers to select options.)

---

<a id="luaL_checkstack"></a>

### `luaL_checkstack`

[-0, +0, *v*]



```
void luaL_checkstack (lua_State *L, int sz, const char *msg);
```




Grows the stack size to `top + sz` elements,
raising an error if the stack cannot grow to that size.
`msg` is an additional text to go into the error message.

---

<a id="luaL_checkstring"></a>

### `luaL_checkstring`

[-0, +0, *v*]



```
const char *luaL_checkstring (lua_State *L, int narg);
```




Checks whether the function argument `narg` is a string
and returns this string.

This function uses [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) to get its result,
so all conversions and caveats of that function apply here.

---

<a id="luaL_checktype"></a>

### `luaL_checktype`

[-0, +0, *v*]



```
void luaL_checktype (lua_State *L, int narg, int t);
```




Checks whether the function argument `narg` has type `t`.
See [`lua_type`](014-lua-tocfunction.md#lua_type) for the encoding of types for `t`.

---

<a id="luaL_checkudata"></a>

### `luaL_checkudata`

[-0, +0, *v*]



```
void *luaL_checkudata (lua_State *L, int narg, const char *tname);
```




Checks whether the function argument `narg` is a userdata
of the type `tname` (see [`luaL_newmetatable`](018-lual-error.md#luaL_newmetatable)).

---

<a id="luaL_dofile"></a>

### `luaL_dofile`

[-0, +?, *m*]



```
int luaL_dofile (lua_State *L, const char *filename);
```




Loads and runs the given file.
It is defined as the following macro:



```

     (luaL_loadfile(L, filename) || lua_pcall(L, 0, LUA_MULTRET, 0))
```




It returns 0 if there are no errors
or 1 in case of errors.

---

<a id="luaL_dostring"></a>

### `luaL_dostring`

[-0, +?, *m*]



```
int luaL_dostring (lua_State *L, const char *str);
```




Loads and runs the given string.
It is defined as the following macro:



```

     (luaL_loadstring(L, str) || lua_pcall(L, 0, LUA_MULTRET, 0))
```




It returns 0 if there are no errors
or 1 in case of errors.

---

