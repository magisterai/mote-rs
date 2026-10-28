# `lua_equal`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-lua-atpanic.md) · [Next](011-lua-istable.md)

<a id="lua_equal"></a>

### `lua_equal`

[-0, +0, *e*]



```
int lua_equal (lua_State *L, int index1, int index2);
```




Returns 1 if the two values in acceptable indices `index1` and
`index2` are equal,
following the semantics of the Lua `==` operator
(that is, may call metamethods).
Otherwise returns 0.
Also returns 0 if any of the indices is non valid.

---

<a id="lua_error"></a>

### `lua_error`

[-1, +0, *v*]



```
int lua_error (lua_State *L);
```




Generates a Lua error.
The error message (which can actually be a Lua value of any type)
must be on the stack top.
This function does a long jump,
and therefore never returns.
(see [`luaL_error`](018-lual-error.md#luaL_error)).

---

<a id="lua_gc"></a>

### `lua_gc`

[-0, +0, *e*]



```
int lua_gc (lua_State *L, int what, int data);
```




Controls the garbage collector.

This function performs several tasks,
according to the value of the parameter `what`:

- **`LUA_GCSTOP`:**
  stops the garbage collector.
- **`LUA_GCRESTART`:**
  restarts the garbage collector.
- **`LUA_GCCOLLECT`:**
  performs a full garbage-collection cycle.
- **`LUA_GCCOUNT`:**
  returns the current amount of memory (in Kbytes) in use by Lua.
- **`LUA_GCCOUNTB`:**
  returns the remainder of dividing the current amount of bytes of
  memory in use by Lua by 1024.
- **`LUA_GCSTEP`:**
  performs an incremental step of garbage collection.
  The step "size" is controlled by `data`
  (larger values mean more steps) in a non-specified way.
  If you want to control the step size
  you must experimentally tune the value of `data`.
  The function returns 1 if the step finished a
  garbage-collection cycle.
- **`LUA_GCSETPAUSE`:**
  sets `data` as the new value
  for the *pause* of the collector (see [§2.10](006-2-9-environments.md#2.10)).
  The function returns the previous value of the pause.
- **`LUA_GCSETSTEPMUL`:**
  sets `data` as the new value for the *step multiplier* of
  the collector (see [§2.10](006-2-9-environments.md#2.10)).
  The function returns the previous value of the step multiplier.

---

<a id="lua_getallocf"></a>

### `lua_getallocf`

[-0, +0, *-*]



```
lua_Alloc lua_getallocf (lua_State *L, void **ud);
```




Returns the memory-allocation function of a given state.
If `ud` is not `NULL`, Lua stores in `*ud` the
opaque pointer passed to [`lua_newstate`](011-lua-istable.md#lua_newstate).

---

<a id="lua_getfenv"></a>

### `lua_getfenv`

[-0, +1, *-*]



```
void lua_getfenv (lua_State *L, int index);
```




Pushes onto the stack the environment table of
the value at the given index.

---

<a id="lua_getfield"></a>

### `lua_getfield`

[-0, +1, *e*]



```
void lua_getfield (lua_State *L, int index, const char *k);
```




Pushes onto the stack the value `t[k]`,
where `t` is the value at the given valid index.
As in Lua, this function may trigger a metamethod
for the "index" event (see [§2.8](005-2-5-9-function-definitions.md#2.8)).

---

<a id="lua_getglobal"></a>

### `lua_getglobal`

[-0, +1, *e*]



```
void lua_getglobal (lua_State *L, const char *name);
```




Pushes onto the stack the value of the global `name`.
It is defined as a macro:



```

     #define lua_getglobal(L,s)  lua_getfield(L, LUA_GLOBALSINDEX, s)
```




---

<a id="lua_getmetatable"></a>

### `lua_getmetatable`

[-0, +(0|1), *-*]



```
int lua_getmetatable (lua_State *L, int index);
```




Pushes onto the stack the metatable of the value at the given
acceptable index.
If the index is not valid,
or if the value does not have a metatable,
the function returns 0 and pushes nothing on the stack.

---

<a id="lua_gettable"></a>

### `lua_gettable`

[-1, +1, *e*]



```
void lua_gettable (lua_State *L, int index);
```




Pushes onto the stack the value `t[k]`,
where `t` is the value at the given valid index
and `k` is the value at the top of the stack.

This function pops the key from the stack
(putting the resulting value in its place).
As in Lua, this function may trigger a metamethod
for the "index" event (see [§2.8](005-2-5-9-function-definitions.md#2.8)).

---

<a id="lua_gettop"></a>

### `lua_gettop`

[-0, +0, *-*]



```
int lua_gettop (lua_State *L);
```




Returns the index of the top element in the stack.
Because indices start at 1,
this result is equal to the number of elements in the stack
(and so 0 means an empty stack).

---

<a id="lua_insert"></a>

### `lua_insert`

[-1, +1, *-*]



```
void lua_insert (lua_State *L, int index);
```




Moves the top element into the given valid index,
shifting up the elements above this index to open space.
Cannot be called with a pseudo-index,
because a pseudo-index is not an actual stack position.

---

<a id="lua_Integer"></a>

### `lua_Integer`



```
typedef ptrdiff_t lua_Integer;
```




The type used by the Lua API to represent integral values.

By default it is a `ptrdiff_t`,
which is usually the largest signed integral type the machine handles
"comfortably".

---

<a id="lua_isboolean"></a>

### `lua_isboolean`

[-0, +0, *-*]



```
int lua_isboolean (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index has type boolean,
and 0 otherwise.

---

<a id="lua_iscfunction"></a>

### `lua_iscfunction`

[-0, +0, *-*]



```
int lua_iscfunction (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a C function,
and 0 otherwise.

---

<a id="lua_isfunction"></a>

### `lua_isfunction`

[-0, +0, *-*]



```
int lua_isfunction (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a function
(either C or Lua), and 0 otherwise.

---

<a id="lua_islightuserdata"></a>

### `lua_islightuserdata`

[-0, +0, *-*]



```
int lua_islightuserdata (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a light userdata,
and 0 otherwise.

---

<a id="lua_isnil"></a>

### `lua_isnil`

[-0, +0, *-*]



```
int lua_isnil (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is **nil**,
and 0 otherwise.

---

<a id="lua_isnone"></a>

### `lua_isnone`

[-0, +0, *-*]



```
int lua_isnone (lua_State *L, int index);
```




Returns 1 if the given acceptable index is not valid
(that is, it refers to an element outside the current stack),
and 0 otherwise.

---

<a id="lua_isnoneornil"></a>

### `lua_isnoneornil`

[-0, +0, *-*]



```
int lua_isnoneornil (lua_State *L, int index);
```




Returns 1 if the given acceptable index is not valid
(that is, it refers to an element outside the current stack)
or if the value at this index is **nil**,
and 0 otherwise.

---

<a id="lua_isnumber"></a>

### `lua_isnumber`

[-0, +0, *-*]



```
int lua_isnumber (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a number
or a string convertible to a number,
and 0 otherwise.

---

<a id="lua_isstring"></a>

### `lua_isstring`

[-0, +0, *-*]



```
int lua_isstring (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a string
or a number (which is always convertible to a string),
and 0 otherwise.

---

