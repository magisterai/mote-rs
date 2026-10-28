# `lua_istable`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-lua-equal.md) · [Next](012-lua-pop.md)

<a id="lua_istable"></a>

### `lua_istable`

[-0, +0, *-*]



```
int lua_istable (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a table,
and 0 otherwise.

---

<a id="lua_isthread"></a>

### `lua_isthread`

[-0, +0, *-*]



```
int lua_isthread (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a thread,
and 0 otherwise.

---

<a id="lua_isuserdata"></a>

### `lua_isuserdata`

[-0, +0, *-*]



```
int lua_isuserdata (lua_State *L, int index);
```




Returns 1 if the value at the given acceptable index is a userdata
(either full or light), and 0 otherwise.

---

<a id="lua_lessthan"></a>

### `lua_lessthan`

[-0, +0, *e*]



```
int lua_lessthan (lua_State *L, int index1, int index2);
```




Returns 1 if the value at acceptable index `index1` is smaller
than the value at acceptable index `index2`,
following the semantics of the Lua `<` operator
(that is, may call metamethods).
Otherwise returns 0.
Also returns 0 if any of the indices is non valid.

---

<a id="lua_load"></a>

### `lua_load`

[-0, +1, *-*]



```
int lua_load (lua_State *L,
              lua_Reader reader,
              void *data,
              const char *chunkname);
```




Loads a Lua chunk.
If there are no errors,
[`lua_load`](011-lua-istable.md#lua_load) pushes the compiled chunk as a Lua
function on top of the stack.
Otherwise, it pushes an error message.
The return values of [`lua_load`](011-lua-istable.md#lua_load) are:

- **0:** no errors;

<a id="pdf-LUA_ERRSYNTAX"></a>

- **`LUA_ERRSYNTAX`:**
  syntax error during pre-compilation;
- **[`LUA_ERRMEM`](011-lua-istable.md#pdf-LUA_ERRMEM):**
  memory allocation error.

This function only loads a chunk;
it does not run it.

[`lua_load`](011-lua-istable.md#lua_load) automatically detects whether the chunk is text or binary,
and loads it accordingly (see program `luac`).

The [`lua_load`](011-lua-istable.md#lua_load) function uses a user-supplied `reader` function
to read the chunk (see [`lua_Reader`](013-lua-rawseti.md#lua_Reader)).
The `data` argument is an opaque value passed to the reader function.

The `chunkname` argument gives a name to the chunk,
which is used for error messages and in debug information (see [§3.8](014-lua-tocfunction.md#3.8)).

---

<a id="lua_newstate"></a>

### `lua_newstate`

[-0, +0, *-*]



```
lua_State *lua_newstate (lua_Alloc f, void *ud);
```




Creates a new, independent state.
Returns `NULL` if cannot create the state
(due to lack of memory).
The argument `f` is the allocator function;
Lua does all memory allocation for this state through this function.
The second argument, `ud`, is an opaque pointer that Lua
simply passes to the allocator in every call.

---

<a id="lua_newtable"></a>

### `lua_newtable`

[-0, +1, *m*]



```
void lua_newtable (lua_State *L);
```




Creates a new empty table and pushes it onto the stack.
It is equivalent to `lua_createtable(L, 0, 0)`.

---

<a id="lua_newthread"></a>

### `lua_newthread`

[-0, +1, *m*]



```
lua_State *lua_newthread (lua_State *L);
```




Creates a new thread, pushes it on the stack,
and returns a pointer to a [`lua_State`](013-lua-rawseti.md#lua_State) that represents this new thread.
The new state returned by this function shares with the original state
all global objects (such as tables),
but has an independent execution stack.

There is no explicit function to close or to destroy a thread.
Threads are subject to garbage collection,
like any Lua object.

---

<a id="lua_newuserdata"></a>

### `lua_newuserdata`

[-0, +1, *m*]



```
void *lua_newuserdata (lua_State *L, size_t size);
```




This function allocates a new block of memory with the given size,
pushes onto the stack a new full userdata with the block address,
and returns this address.

Userdata represent C values in Lua.
A *full userdata* represents a block of memory.
It is an object (like a table):
you must create it, it can have its own metatable,
and you can detect when it is being collected.
A full userdata is only equal to itself (under raw equality).

When Lua collects a full userdata with a `gc` metamethod,
Lua calls the metamethod and marks the userdata as finalized.
When this userdata is collected again then
Lua frees its corresponding memory.

---

<a id="lua_next"></a>

### `lua_next`

[-1, +(2|0), *e*]



```
int lua_next (lua_State *L, int index);
```




Pops a key from the stack,
and pushes a key-value pair from the table at the given index
(the "next" pair after the given key).
If there are no more elements in the table,
then [`lua_next`](011-lua-istable.md#lua_next) returns 0 (and pushes nothing).

A typical traversal looks like this:



```

     /* table is in the stack at index 't' */
     lua_pushnil(L);  /* first key */
     while (lua_next(L, t) != 0) {
       /* uses 'key' (at index -2) and 'value' (at index -1) */
       printf("%s - %s\n",
              lua_typename(L, lua_type(L, -2)),
              lua_typename(L, lua_type(L, -1)));
       /* removes 'value'; keeps 'key' for next iteration */
       lua_pop(L, 1);
     }
```




While traversing a table,
do not call [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) directly on a key,
unless you know that the key is actually a string.
Recall that [`lua_tolstring`](014-lua-tocfunction.md#lua_tolstring) *changes*
the value at the given index;
this confuses the next call to [`lua_next`](011-lua-istable.md#lua_next).

---

<a id="lua_Number"></a>

### `lua_Number`



```
typedef double lua_Number;
```




The type of numbers in Lua.
By default, it is double, but that can be changed in `luaconf.h`.

Through the configuration file you can change
Lua to operate with another type for numbers (e.g., float or long).

---

<a id="lua_objlen"></a>

### `lua_objlen`

[-0, +0, *-*]



```
size_t lua_objlen (lua_State *L, int index);
```




Returns the "length" of the value at the given acceptable index:
for strings, this is the string length;
for tables, this is the result of the length operator ('`#`');
for userdata, this is the size of the block of memory allocated
for the userdata;
for other values, it is 0.

---

<a id="lua_pcall"></a>

### `lua_pcall`

[-(nargs + 1), +(nresults|1), *-*]



```
int lua_pcall (lua_State *L, int nargs, int nresults, int errfunc);
```




Calls a function in protected mode.

Both `nargs` and `nresults` have the same meaning as
in [`lua_call`](009-lua-atpanic.md#lua_call).
If there are no errors during the call,
[`lua_pcall`](011-lua-istable.md#lua_pcall) behaves exactly like [`lua_call`](009-lua-atpanic.md#lua_call).
However, if there is any error,
[`lua_pcall`](011-lua-istable.md#lua_pcall) catches it,
pushes a single value on the stack (the error message),
and returns an error code.
Like [`lua_call`](009-lua-atpanic.md#lua_call),
[`lua_pcall`](011-lua-istable.md#lua_pcall) always removes the function
and its arguments from the stack.

If `errfunc` is 0,
then the error message returned on the stack
is exactly the original error message.
Otherwise, `errfunc` is the stack index of an
*error handler function*.
(In the current implementation, this index cannot be a pseudo-index.)
In case of runtime errors,
this function will be called with the error message
and its return value will be the message returned on the stack by [`lua_pcall`](011-lua-istable.md#lua_pcall).

Typically, the error handler function is used to add more debug
information to the error message, such as a stack traceback.
Such information cannot be gathered after the return of [`lua_pcall`](011-lua-istable.md#lua_pcall),
since by then the stack has unwound.

The [`lua_pcall`](011-lua-istable.md#lua_pcall) function returns 0 in case of success
or one of the following error codes
(defined in `lua.h`):

<a id="pdf-LUA_ERRRUN"></a>

- **`LUA_ERRRUN`:**
  a runtime error.

<a id="pdf-LUA_ERRMEM"></a>

- **`LUA_ERRMEM`:**
  memory allocation error.
  For such errors, Lua does not call the error handler function.

<a id="pdf-LUA_ERRERR"></a>

- **`LUA_ERRERR`:**
  error while running the error handler function.

---

