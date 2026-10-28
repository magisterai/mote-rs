# `lua_Debug`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-lua-tocfunction.md) · [Next](016-lua-hook.md)

<a id="lua_Debug"></a>

### `lua_Debug`



```
typedef struct lua_Debug {
  int event;
  const char *name;           /* (n) */
  const char *namewhat;       /* (n) */
  const char *what;           /* (S) */
  const char *source;         /* (S) */
  int currentline;            /* (l) */
  int nups;                   /* (u) number of upvalues */
  int linedefined;            /* (S) */
  int lastlinedefined;        /* (S) */
  char short_src[LUA_IDSIZE]; /* (S) */
  /* private part */
  other fields
} lua_Debug;
```




A structure used to carry different pieces of
information about an active function.
[`lua_getstack`](015-lua-debug.md#lua_getstack) fills only the private part
of this structure, for later use.
To fill the other fields of [`lua_Debug`](015-lua-debug.md#lua_Debug) with useful information,
call [`lua_getinfo`](015-lua-debug.md#lua_getinfo).

The fields of [`lua_Debug`](015-lua-debug.md#lua_Debug) have the following meaning:

- **`source`:**
  If the function was defined in a string,
  then `source` is that string.
  If the function was defined in a file,
  then `source` starts with a '`@`' followed by the file name.
- **`short_src`:**
  a "printable" version of `source`, to be used in error messages.
- **`linedefined`:**
  the line number where the definition of the function starts.
- **`lastlinedefined`:**
  the line number where the definition of the function ends.
- **`what`:**
  the string `"Lua"` if the function is a Lua function,
  `"C"` if it is a C function,
  `"main"` if it is the main part of a chunk,
  and `"tail"` if it was a function that did a tail call.
  In the latter case,
  Lua has no other information about the function.
- **`currentline`:**
  the current line where the given function is executing.
  When no line information is available,
  `currentline` is set to -1.
- **`name`:**
  a reasonable name for the given function.
  Because functions in Lua are first-class values,
  they do not have a fixed name:
  some functions can be the value of multiple global variables,
  while others can be stored only in a table field.
  The `lua_getinfo` function checks how the function was
  called to find a suitable name.
  If it cannot find a name,
  then `name` is set to `NULL`.
- **`namewhat`:**
  explains the `name` field.
  The value of `namewhat` can be
  `"global"`, `"local"`, `"method"`,
  `"field"`, `"upvalue"`, or `""` (the empty string),
  according to how the function was called.
  (Lua uses the empty string when no other option seems to apply.)
- **`nups`:**
  the number of upvalues of the function.

---

<a id="lua_gethook"></a>

### `lua_gethook`

[-0, +0, *-*]



```
lua_Hook lua_gethook (lua_State *L);
```




Returns the current hook function.

---

<a id="lua_gethookcount"></a>

### `lua_gethookcount`

[-0, +0, *-*]



```
int lua_gethookcount (lua_State *L);
```




Returns the current hook count.

---

<a id="lua_gethookmask"></a>

### `lua_gethookmask`

[-0, +0, *-*]



```
int lua_gethookmask (lua_State *L);
```




Returns the current hook mask.

---

<a id="lua_getinfo"></a>

### `lua_getinfo`

[-(0|1), +(0|1|2), *m*]



```
int lua_getinfo (lua_State *L, const char *what, lua_Debug *ar);
```




Returns information about a specific function or function invocation.

To get information about a function invocation,
the parameter `ar` must be a valid activation record that was
filled by a previous call to [`lua_getstack`](015-lua-debug.md#lua_getstack) or
given as argument to a hook (see [`lua_Hook`](016-lua-hook.md#lua_Hook)).

To get information about a function you push it onto the stack
and start the `what` string with the character '`>`'.
(In that case,
`lua_getinfo` pops the function in the top of the stack.)
For instance, to know in which line a function `f` was defined,
you can write the following code:



```

     lua_Debug ar;
     lua_getfield(L, LUA_GLOBALSINDEX, "f");  /* get global 'f' */
     lua_getinfo(L, ">S", &ar);
     printf("%d\n", ar.linedefined);
```




Each character in the string `what`
selects some fields of the structure `ar` to be filled or
a value to be pushed on the stack:

- **'`n`':** fills in the field `name` and `namewhat`;
- **'`S`':**
  fills in the fields `source`, `short_src`,
  `linedefined`, `lastlinedefined`, and `what`;
- **'`l`':** fills in the field `currentline`;
- **'`u`':** fills in the field `nups`;
- **'`f`':**
  pushes onto the stack the function that is
  running at the given level;
- **'`L`':**
  pushes onto the stack a table whose indices are the
  numbers of the lines that are valid on the function.
  (A *valid line* is a line with some associated code,
  that is, a line where you can put a break point.
  Non-valid lines include empty lines and comments.)

This function returns 0 on error
(for instance, an invalid option in `what`).

---

<a id="lua_getlocal"></a>

### `lua_getlocal`

[-0, +(0|1), *-*]



```
const char *lua_getlocal (lua_State *L, lua_Debug *ar, int n);
```




Gets information about a local variable of a given activation record.
The parameter `ar` must be a valid activation record that was
filled by a previous call to [`lua_getstack`](015-lua-debug.md#lua_getstack) or
given as argument to a hook (see [`lua_Hook`](016-lua-hook.md#lua_Hook)).
The index `n` selects which local variable to inspect
(1 is the first parameter or active local variable, and so on,
until the last active local variable).
[`lua_getlocal`](015-lua-debug.md#lua_getlocal) pushes the variable's value onto the stack
and returns its name.

Variable names starting with '`(`' (open parentheses)
represent internal variables
(loop control variables, temporaries, and C function locals).

Returns `NULL` (and pushes nothing)
when the index is greater than
the number of active local variables.

---

<a id="lua_getstack"></a>

### `lua_getstack`

[-0, +0, *-*]



```
int lua_getstack (lua_State *L, int level, lua_Debug *ar);
```




Get information about the interpreter runtime stack.

This function fills parts of a [`lua_Debug`](015-lua-debug.md#lua_Debug) structure with
an identification of the *activation record*
of the function executing at a given level.
Level 0 is the current running function,
whereas level *n+1* is the function that has called level *n*.
When there are no errors, [`lua_getstack`](015-lua-debug.md#lua_getstack) returns 1;
when called with a level greater than the stack depth,
it returns 0.

---

<a id="lua_getupvalue"></a>

### `lua_getupvalue`

[-0, +(0|1), *-*]



```
const char *lua_getupvalue (lua_State *L, int funcindex, int n);
```




Gets information about a closure's upvalue.
(For Lua functions,
upvalues are the external local variables that the function uses,
and that are consequently included in its closure.)
[`lua_getupvalue`](015-lua-debug.md#lua_getupvalue) gets the index `n` of an upvalue,
pushes the upvalue's value onto the stack,
and returns its name.
`funcindex` points to the closure in the stack.
(Upvalues have no particular order,
as they are active through the whole function.
So, they are numbered in an arbitrary order.)

Returns `NULL` (and pushes nothing)
when the index is greater than the number of upvalues.
For C functions, this function uses the empty string `""`
as a name for all upvalues.

---

