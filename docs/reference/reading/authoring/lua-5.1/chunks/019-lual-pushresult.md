# `luaL_pushresult`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-lual-error.md) · [Next](020-collectgarbage-opt-arg.md)

<a id="luaL_pushresult"></a>

### `luaL_pushresult`

[-?, +1, *m*]



```
void luaL_pushresult (luaL_Buffer *B);
```




Finishes the use of buffer `B` leaving the final string on
the top of the stack.

---

<a id="luaL_ref"></a>

### `luaL_ref`

[-1, +0, *m*]



```
int luaL_ref (lua_State *L, int t);
```




Creates and returns a *reference*,
in the table at index `t`,
for the object at the top of the stack (and pops the object).

A reference is a unique integer key.
As long as you do not manually add integer keys into table `t`,
[`luaL_ref`](019-lual-pushresult.md#luaL_ref) ensures the uniqueness of the key it returns.
You can retrieve an object referred by reference `r`
by calling `lua_rawgeti(L, t, r)`.
Function [`luaL_unref`](019-lual-pushresult.md#luaL_unref) frees a reference and its associated object.

<a id="pdf-LUA_REFNIL"></a>

<a id="pdf-LUA_NOREF"></a>

If the object at the top of the stack is **nil**,
[`luaL_ref`](019-lual-pushresult.md#luaL_ref) returns the constant `LUA_REFNIL`.
The constant `LUA_NOREF` is guaranteed to be different
from any reference returned by [`luaL_ref`](019-lual-pushresult.md#luaL_ref).

---

<a id="luaL_Reg"></a>

### `luaL_Reg`



```
typedef struct luaL_Reg {
  const char *name;
  lua_CFunction func;
} luaL_Reg;
```




Type for arrays of functions to be registered by
[`luaL_register`](019-lual-pushresult.md#luaL_register).
`name` is the function name and `func` is a pointer to
the function.
Any array of [`luaL_Reg`](019-lual-pushresult.md#luaL_Reg) must end with an sentinel entry
in which both `name` and `func` are `NULL`.

---

<a id="luaL_register"></a>

### `luaL_register`

[-(0|1), +1, *m*]



```
void luaL_register (lua_State *L,
                    const char *libname,
                    const luaL_Reg *l);
```




Opens a library.

When called with `libname` equal to `NULL`,
it simply registers all functions in the list `l`
(see [`luaL_Reg`](019-lual-pushresult.md#luaL_Reg)) into the table on the top of the stack.

When called with a non-null `libname`,
`luaL_register` creates a new table `t`,
sets it as the value of the global variable `libname`,
sets it as the value of `package.loaded[libname]`,
and registers on it all functions in the list `l`.
If there is a table in `package.loaded[libname]` or in
variable `libname`,
reuses this table instead of creating a new one.

In any case the function leaves the table
on the top of the stack.

---

<a id="luaL_typename"></a>

### `luaL_typename`

[-0, +0, *-*]



```
const char *luaL_typename (lua_State *L, int index);
```




Returns the name of the type of the value at the given index.

---

<a id="luaL_typerror"></a>

### `luaL_typerror`

[-0, +0, *v*]



```
int luaL_typerror (lua_State *L, int narg, const char *tname);
```




Generates an error with a message like the following:



```

     location: bad argument narg to 'func' (tname expected, got rt)
```




where `location` is produced by [`luaL_where`](019-lual-pushresult.md#luaL_where),
`func` is the name of the current function,
and `rt` is the type name of the actual argument.

---

<a id="luaL_unref"></a>

### `luaL_unref`

[-0, +0, *-*]



```
void luaL_unref (lua_State *L, int t, int ref);
```




Releases reference `ref` from the table at index `t`
(see [`luaL_ref`](019-lual-pushresult.md#luaL_ref)).
The entry is removed from the table,
so that the referred object can be collected.
The reference `ref` is also freed to be used again.

If `ref` is [`LUA_NOREF`](019-lual-pushresult.md#pdf-LUA_NOREF) or [`LUA_REFNIL`](019-lual-pushresult.md#pdf-LUA_REFNIL),
[`luaL_unref`](019-lual-pushresult.md#luaL_unref) does nothing.

---

<a id="luaL_where"></a>

### `luaL_where`

[-0, +1, *m*]



```
void luaL_where (lua_State *L, int lvl);
```




Pushes onto the stack a string identifying the current position
of the control at level `lvl` in the call stack.
Typically this string has the following format:



```

     chunkname:currentline:
```




Level 0 is the running function,
level 1 is the function that called the running function,
etc.

This function is used to build a prefix for error messages.

<a id="5"></a>

# 5 –  Standard Libraries

The standard Lua libraries provide useful functions
that are implemented directly through the C API.
Some of these functions provide essential services to the language
(e.g., [`type`](021-rawequal-v1-v2.md#pdf-type) and [`getmetatable`](020-collectgarbage-opt-arg.md#pdf-getmetatable));
others provide access to "outside" services (e.g., I/O);
and others could be implemented in Lua itself,
but are quite useful or have critical performance requirements that
deserve an implementation in C (e.g., [`table.sort`](025-table-insert-table-pos-value.md#pdf-table.sort)).

All libraries are implemented through the official C API
and are provided as separate C modules.
Currently, Lua has the following standard libraries:

- basic library, which includes the coroutine sub-library;
- package library;
- string manipulation;
- table manipulation;
- mathematical functions (sin, log, etc.);
- input and output;
- operating system facilities;
- debug facilities.

Except for the basic and package libraries,
each library provides all its functions as fields of a global table
or as methods of its objects.

<a id="pdf-luaopen_base"></a>

<a id="pdf-luaopen_package"></a>

<a id="pdf-luaopen_string"></a>

<a id="pdf-luaopen_table"></a>

<a id="pdf-luaopen_math"></a>

<a id="pdf-luaopen_io"></a>

<a id="pdf-luaopen_os"></a>

<a id="pdf-luaopen_debug"></a>

<a id="pdf-lualib.h"></a>

To have access to these libraries,
the C host program should call the [`luaL_openlibs`](018-lual-error.md#luaL_openlibs) function,
which opens all standard libraries.
Alternatively,
it can open them individually by calling
`luaopen_base` (for the basic library),
`luaopen_package` (for the package library),
`luaopen_string` (for the string library),
`luaopen_table` (for the table library),
`luaopen_math` (for the mathematical library),
`luaopen_io` (for the I/O library),
`luaopen_os` (for the Operating System library),
and `luaopen_debug` (for the debug library).
These functions are declared in `lualib.h`
and should not be called directly:
you must call them like any other Lua C function,
e.g., by using [`lua_call`](009-lua-atpanic.md#lua_call).

<a id="5.1"></a>

## 5.1 –  Basic Functions

The basic library provides some core functions to Lua.
If you do not include this library in your application,
you should check carefully whether you need to provide
implementations for some of its facilities.

---

<a id="pdf-assert"></a>

### `assert (v [, message])`

Issues an error when
the value of its argument `v` is false (i.e., **nil** or **false**);
otherwise, returns all its arguments.
`message` is an error message;
when absent, it defaults to "assertion failed!"

---

