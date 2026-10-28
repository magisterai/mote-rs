# `luaL_error`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](017-lual-argerror.md) · [Next](019-lual-pushresult.md)

<a id="luaL_error"></a>

### `luaL_error`

[-0, +0, *v*]



```
int luaL_error (lua_State *L, const char *fmt, ...);
```




Raises an error.
The error message format is given by `fmt`
plus any extra arguments,
following the same rules of [`lua_pushfstring`](012-lua-pop.md#lua_pushfstring).
It also adds at the beginning of the message the file name and
the line number where the error occurred,
if this information is available.

This function never returns,
but it is an idiom to use it in C functions
as `return luaL_error(args)`.

---

<a id="luaL_getmetafield"></a>

### `luaL_getmetafield`

[-0, +(0|1), *m*]



```
int luaL_getmetafield (lua_State *L, int obj, const char *e);
```




Pushes onto the stack the field `e` from the metatable
of the object at index `obj`.
If the object does not have a metatable,
or if the metatable does not have this field,
returns 0 and pushes nothing.

---

<a id="luaL_getmetatable"></a>

### `luaL_getmetatable`

[-0, +1, *-*]



```
void luaL_getmetatable (lua_State *L, const char *tname);
```




Pushes onto the stack the metatable associated with name `tname`
in the registry (see [`luaL_newmetatable`](018-lual-error.md#luaL_newmetatable)).

---

<a id="luaL_gsub"></a>

### `luaL_gsub`

[-0, +1, *m*]



```
const char *luaL_gsub (lua_State *L,
                       const char *s,
                       const char *p,
                       const char *r);
```




Creates a copy of string `s` by replacing
any occurrence of the string `p`
with the string `r`.
Pushes the resulting string on the stack and returns it.

---

<a id="luaL_loadbuffer"></a>

### `luaL_loadbuffer`

[-0, +1, *m*]



```
int luaL_loadbuffer (lua_State *L,
                     const char *buff,
                     size_t sz,
                     const char *name);
```




Loads a buffer as a Lua chunk.
This function uses [`lua_load`](011-lua-istable.md#lua_load) to load the chunk in the
buffer pointed to by `buff` with size `sz`.

This function returns the same results as [`lua_load`](011-lua-istable.md#lua_load).
`name` is the chunk name,
used for debug information and error messages.

---

<a id="luaL_loadfile"></a>

### `luaL_loadfile`

[-0, +1, *m*]



```
int luaL_loadfile (lua_State *L, const char *filename);
```




Loads a file as a Lua chunk.
This function uses [`lua_load`](011-lua-istable.md#lua_load) to load the chunk in the file
named `filename`.
If `filename` is `NULL`,
then it loads from the standard input.
The first line in the file is ignored if it starts with a `#`.

<a id="pdf-LUA_ERRFILE"></a>

This function returns the same results as [`lua_load`](011-lua-istable.md#lua_load),
but it has an extra error code `LUA_ERRFILE`
if it cannot open/read the file.

As [`lua_load`](011-lua-istable.md#lua_load), this function only loads the chunk;
it does not run it.

---

<a id="luaL_loadstring"></a>

### `luaL_loadstring`

[-0, +1, *m*]



```
int luaL_loadstring (lua_State *L, const char *s);
```




Loads a string as a Lua chunk.
This function uses [`lua_load`](011-lua-istable.md#lua_load) to load the chunk in
the zero-terminated string `s`.

This function returns the same results as [`lua_load`](011-lua-istable.md#lua_load).

Also as [`lua_load`](011-lua-istable.md#lua_load), this function only loads the chunk;
it does not run it.

---

<a id="luaL_newmetatable"></a>

### `luaL_newmetatable`

[-0, +1, *m*]



```
int luaL_newmetatable (lua_State *L, const char *tname);
```




If the registry already has the key `tname`,
returns 0.
Otherwise,
creates a new table to be used as a metatable for userdata,
adds it to the registry with key `tname`,
and returns 1.

In both cases pushes onto the stack the final value associated
with `tname` in the registry.

---

<a id="luaL_newstate"></a>

### `luaL_newstate`

[-0, +0, *-*]



```
lua_State *luaL_newstate (void);
```




Creates a new Lua state.
It calls [`lua_newstate`](011-lua-istable.md#lua_newstate) with an
allocator based on the standard C `realloc` function
and then sets a panic function (see [`lua_atpanic`](009-lua-atpanic.md#lua_atpanic)) that prints
an error message to the standard error output in case of fatal
errors.

Returns the new state,
or `NULL` if there is a memory allocation error.

---

<a id="luaL_openlibs"></a>

### `luaL_openlibs`

[-0, +0, *m*]



```
void luaL_openlibs (lua_State *L);
```




Opens all standard Lua libraries into the given state.

---

<a id="luaL_optint"></a>

### `luaL_optint`

[-0, +0, *v*]



```
int luaL_optint (lua_State *L, int narg, int d);
```




If the function argument `narg` is a number,
returns this number cast to an `int`.
If this argument is absent or is **nil**,
returns `d`.
Otherwise, raises an error.

---

<a id="luaL_optinteger"></a>

### `luaL_optinteger`

[-0, +0, *v*]



```
lua_Integer luaL_optinteger (lua_State *L,
                             int narg,
                             lua_Integer d);
```




If the function argument `narg` is a number,
returns this number cast to a [`lua_Integer`](010-lua-equal.md#lua_Integer).
If this argument is absent or is **nil**,
returns `d`.
Otherwise, raises an error.

---

<a id="luaL_optlong"></a>

### `luaL_optlong`

[-0, +0, *v*]



```
long luaL_optlong (lua_State *L, int narg, long d);
```




If the function argument `narg` is a number,
returns this number cast to a `long`.
If this argument is absent or is **nil**,
returns `d`.
Otherwise, raises an error.

---

<a id="luaL_optlstring"></a>

### `luaL_optlstring`

[-0, +0, *v*]



```
const char *luaL_optlstring (lua_State *L,
                             int narg,
                             const char *d,
                             size_t *l);
```




If the function argument `narg` is a string,
returns this string.
If this argument is absent or is **nil**,
returns `d`.
Otherwise, raises an error.

If `l` is not `NULL`,
fills the position `*l` with the results's length.

---

<a id="luaL_optnumber"></a>

### `luaL_optnumber`

[-0, +0, *v*]



```
lua_Number luaL_optnumber (lua_State *L, int narg, lua_Number d);
```




If the function argument `narg` is a number,
returns this number.
If this argument is absent or is **nil**,
returns `d`.
Otherwise, raises an error.

---

<a id="luaL_optstring"></a>

### `luaL_optstring`

[-0, +0, *v*]



```
const char *luaL_optstring (lua_State *L,
                            int narg,
                            const char *d);
```




If the function argument `narg` is a string,
returns this string.
If this argument is absent or is **nil**,
returns `d`.
Otherwise, raises an error.

---

<a id="luaL_prepbuffer"></a>

### `luaL_prepbuffer`

[-0, +0, *-*]



```
char *luaL_prepbuffer (luaL_Buffer *B);
```




<a id="pdf-LUAL_BUFFERSIZE"></a>

Returns an address to a space of size `LUAL_BUFFERSIZE`
where you can copy a string to be added to buffer `B`
(see [`luaL_Buffer`](017-lual-argerror.md#luaL_Buffer)).
After copying the string into this space you must call
[`luaL_addsize`](016-lua-hook.md#luaL_addsize) with the size of the string to actually add
it to the buffer.

---

