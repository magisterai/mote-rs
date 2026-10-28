# `lua_rawseti`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-lua-pop.md) · [Next](014-lua-tocfunction.md)

<a id="lua_rawseti"></a>

### `lua_rawseti`

[-1, +0, *m*]



```
void lua_rawseti (lua_State *L, int index, int n);
```




Does the equivalent of `t[n] = v`,
where `t` is the value at the given valid index
and `v` is the value at the top of the stack.

This function pops the value from the stack.
The assignment is raw;
that is, it does not invoke metamethods.

---

<a id="lua_Reader"></a>

### `lua_Reader`



```
typedef const char * (*lua_Reader) (lua_State *L,
                                    void *data,
                                    size_t *size);
```




The reader function used by [`lua_load`](011-lua-istable.md#lua_load).
Every time it needs another piece of the chunk,
[`lua_load`](011-lua-istable.md#lua_load) calls the reader,
passing along its `data` parameter.
The reader must return a pointer to a block of memory
with a new piece of the chunk
and set `size` to the block size.
The block must exist until the reader function is called again.
To signal the end of the chunk,
the reader must return `NULL` or set `size` to zero.
The reader function may return pieces of any size greater than zero.

---

<a id="lua_register"></a>

### `lua_register`

[-0, +0, *e*]



```
void lua_register (lua_State *L,
                   const char *name,
                   lua_CFunction f);
```




Sets the C function `f` as the new value of global `name`.
It is defined as a macro:



```

     #define lua_register(L,n,f) \
            (lua_pushcfunction(L, f), lua_setglobal(L, n))
```




---

<a id="lua_remove"></a>

### `lua_remove`

[-1, +0, *-*]



```
void lua_remove (lua_State *L, int index);
```




Removes the element at the given valid index,
shifting down the elements above this index to fill the gap.
Cannot be called with a pseudo-index,
because a pseudo-index is not an actual stack position.

---

<a id="lua_replace"></a>

### `lua_replace`

[-1, +0, *-*]



```
void lua_replace (lua_State *L, int index);
```




Moves the top element into the given position (and pops it),
without shifting any element
(therefore replacing the value at the given position).

---

<a id="lua_resume"></a>

### `lua_resume`

[-?, +?, *-*]



```
int lua_resume (lua_State *L, int narg);
```




Starts and resumes a coroutine in a given thread.

To start a coroutine, you first create a new thread
(see [`lua_newthread`](011-lua-istable.md#lua_newthread));
then you push onto its stack the main function plus any arguments;
then you call [`lua_resume`](013-lua-rawseti.md#lua_resume),
with `narg` being the number of arguments.
This call returns when the coroutine suspends or finishes its execution.
When it returns, the stack contains all values passed to [`lua_yield`](014-lua-tocfunction.md#lua_yield),
or all values returned by the body function.
[`lua_resume`](013-lua-rawseti.md#lua_resume) returns
[`LUA_YIELD`](013-lua-rawseti.md#pdf-LUA_YIELD) if the coroutine yields,
0 if the coroutine finishes its execution
without errors,
or an error code in case of errors (see [`lua_pcall`](011-lua-istable.md#lua_pcall)).
In case of errors,
the stack is not unwound,
so you can use the debug API over it.
The error message is on the top of the stack.
To restart a coroutine, you put on its stack only the values to
be passed as results from `yield`,
and then call [`lua_resume`](013-lua-rawseti.md#lua_resume).

---

<a id="lua_setallocf"></a>

### `lua_setallocf`

[-0, +0, *-*]



```
void lua_setallocf (lua_State *L, lua_Alloc f, void *ud);
```




Changes the allocator function of a given state to `f`
with user data `ud`.

---

<a id="lua_setfenv"></a>

### `lua_setfenv`

[-1, +0, *-*]



```
int lua_setfenv (lua_State *L, int index);
```




Pops a table from the stack and sets it as
the new environment for the value at the given index.
If the value at the given index is
neither a function nor a thread nor a userdata,
[`lua_setfenv`](013-lua-rawseti.md#lua_setfenv) returns 0.
Otherwise it returns 1.

---

<a id="lua_setfield"></a>

### `lua_setfield`

[-1, +0, *e*]



```
void lua_setfield (lua_State *L, int index, const char *k);
```




Does the equivalent to `t[k] = v`,
where `t` is the value at the given valid index
and `v` is the value at the top of the stack.

This function pops the value from the stack.
As in Lua, this function may trigger a metamethod
for the "newindex" event (see [§2.8](005-2-5-9-function-definitions.md#2.8)).

---

<a id="lua_setglobal"></a>

### `lua_setglobal`

[-1, +0, *e*]



```
void lua_setglobal (lua_State *L, const char *name);
```




Pops a value from the stack and
sets it as the new value of global `name`.
It is defined as a macro:



```

     #define lua_setglobal(L,s)   lua_setfield(L, LUA_GLOBALSINDEX, s)
```




---

<a id="lua_setmetatable"></a>

### `lua_setmetatable`

[-1, +0, *-*]



```
int lua_setmetatable (lua_State *L, int index);
```




Pops a table from the stack and
sets it as the new metatable for the value at the given
acceptable index.

---

<a id="lua_settable"></a>

### `lua_settable`

[-2, +0, *e*]



```
void lua_settable (lua_State *L, int index);
```




Does the equivalent to `t[k] = v`,
where `t` is the value at the given valid index,
`v` is the value at the top of the stack,
and `k` is the value just below the top.

This function pops both the key and the value from the stack.
As in Lua, this function may trigger a metamethod
for the "newindex" event (see [§2.8](005-2-5-9-function-definitions.md#2.8)).

---

<a id="lua_settop"></a>

### `lua_settop`

[-?, +?, *-*]



```
void lua_settop (lua_State *L, int index);
```




Accepts any acceptable index, or 0,
and sets the stack top to this index.
If the new top is larger than the old one,
then the new elements are filled with **nil**.
If `index` is 0, then all stack elements are removed.

---

<a id="lua_State"></a>

### `lua_State`



```
typedef struct lua_State lua_State;
```




Opaque structure that keeps the whole state of a Lua interpreter.
The Lua library is fully reentrant:
it has no global variables.
All information about a state is kept in this structure.

A pointer to this state must be passed as the first argument to
every function in the library, except to [`lua_newstate`](011-lua-istable.md#lua_newstate),
which creates a Lua state from scratch.

---

<a id="lua_status"></a>

### `lua_status`

[-0, +0, *-*]



```
int lua_status (lua_State *L);
```




Returns the status of the thread `L`.

<a id="pdf-LUA_YIELD"></a>

The status can be 0 for a normal thread,
an error code if the thread finished its execution with an error,
or `LUA_YIELD` if the thread is suspended.

---

<a id="lua_toboolean"></a>

### `lua_toboolean`

[-0, +0, *-*]



```
int lua_toboolean (lua_State *L, int index);
```




Converts the Lua value at the given acceptable index to a C boolean
value (0 or 1).
Like all tests in Lua,
[`lua_toboolean`](013-lua-rawseti.md#lua_toboolean) returns 1 for any Lua value
different from **false** and **nil**;
otherwise it returns 0.
It also returns 0 when called with a non-valid index.
(If you want to accept only actual boolean values,
use [`lua_isboolean`](010-lua-equal.md#lua_isboolean) to test the value's type.)

---

