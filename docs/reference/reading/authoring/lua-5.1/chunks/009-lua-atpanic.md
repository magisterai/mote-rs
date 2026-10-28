# `lua_atpanic`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-3-2-stack-size.md) · [Next](010-lua-equal.md)

<a id="lua_atpanic"></a>

### `lua_atpanic`

[-0, +0, *-*]



```
lua_CFunction lua_atpanic (lua_State *L, lua_CFunction panicf);
```




Sets a new panic function and returns the old one.

If an error happens outside any protected environment,
Lua calls a *panic function*
and then calls `exit(EXIT_FAILURE)`,
thus exiting the host application.
Your panic function can avoid this exit by
never returning (e.g., doing a long jump).

The panic function can access the error message at the top of the stack.

---

<a id="lua_call"></a>

### `lua_call`

[-(nargs + 1), +nresults, *e*]



```
void lua_call (lua_State *L, int nargs, int nresults);
```




Calls a function.

<a id="pdf-LUA_MULTRET"></a>

To call a function you must use the following protocol:
first, the function to be called is pushed onto the stack;
then, the arguments to the function are pushed
in direct order;
that is, the first argument is pushed first.
Finally you call [`lua_call`](009-lua-atpanic.md#lua_call);
`nargs` is the number of arguments that you pushed onto the stack.
All arguments and the function value are popped from the stack
when the function is called.
The function results are pushed onto the stack when the function returns.
The number of results is adjusted to `nresults`,
unless `nresults` is `LUA_MULTRET`.
In this case, *all* results from the function are pushed.
Lua takes care that the returned values fit into the stack space.
The function results are pushed onto the stack in direct order
(the first result is pushed first),
so that after the call the last result is on the top of the stack.

Any error inside the called function is propagated upwards
(with a `longjmp`).

The following example shows how the host program can do the
equivalent to this Lua code:



```

     a = f("how", t.x, 14)
```




Here it is in C:



```

     lua_getfield(L, LUA_GLOBALSINDEX, "f"); /* function to be called */
     lua_pushstring(L, "how");                        /* 1st argument */
     lua_getfield(L, LUA_GLOBALSINDEX, "t");   /* table to be indexed */
     lua_getfield(L, -1, "x");        /* push result of t.x (2nd arg) */
     lua_remove(L, -2);                  /* remove 't' from the stack */
     lua_pushinteger(L, 14);                          /* 3rd argument */
     lua_call(L, 3, 1);     /* call 'f' with 3 arguments and 1 result */
     lua_setfield(L, LUA_GLOBALSINDEX, "a");        /* set global 'a' */
```




Note that the code above is "balanced":
at its end, the stack is back to its original configuration.
This is considered good programming practice.

---

<a id="lua_CFunction"></a>

### `lua_CFunction`



```
typedef int (*lua_CFunction) (lua_State *L);
```




Type for C functions.

In order to communicate properly with Lua,
a C function must use the following protocol,
which defines the way parameters and results are passed:
a C function receives its arguments from Lua in its stack
in direct order (the first argument is pushed first).
So, when the function starts,
`lua_gettop(L)` returns the number of arguments received by the function.
The first argument (if any) is at index 1
and its last argument is at index `lua_gettop(L)`.
To return values to Lua, a C function just pushes them onto the stack,
in direct order (the first result is pushed first),
and returns the number of results.
Any other value in the stack below the results will be properly
discarded by Lua.
Like a Lua function, a C function called by Lua can also return
many results.

As an example, the following function receives a variable number
of numerical arguments and returns their average and sum:



```

     static int foo (lua_State *L) {
       int n = lua_gettop(L);    /* number of arguments */
       lua_Number sum = 0;
       int i;
       for (i = 1; i <= n; i++) {
         if (!lua_isnumber(L, i)) {
           lua_pushstring(L, "incorrect argument");
           lua_error(L);
         }
         sum += lua_tonumber(L, i);
       }
       lua_pushnumber(L, sum/n);        /* first result */
       lua_pushnumber(L, sum);         /* second result */
       return 2;                   /* number of results */
     }
```




---

<a id="lua_checkstack"></a>

### `lua_checkstack`

[-0, +0, *m*]



```
int lua_checkstack (lua_State *L, int extra);
```




Ensures that there are at least `extra` free stack slots in the stack.
It returns false if it cannot grow the stack to that size.
This function never shrinks the stack;
if the stack is already larger than the new size,
it is left unchanged.

---

<a id="lua_close"></a>

### `lua_close`

[-0, +0, *-*]



```
void lua_close (lua_State *L);
```




Destroys all objects in the given Lua state
(calling the corresponding garbage-collection metamethods, if any)
and frees all dynamic memory used by this state.
On several platforms, you may not need to call this function,
because all resources are naturally released when the host program ends.
On the other hand, long-running programs,
such as a daemon or a web server,
might need to release states as soon as they are not needed,
to avoid growing too large.

---

<a id="lua_concat"></a>

### `lua_concat`

[-n, +1, *e*]



```
void lua_concat (lua_State *L, int n);
```




Concatenates the `n` values at the top of the stack,
pops them, and leaves the result at the top.
If `n` is 1, the result is the single value on the stack
(that is, the function does nothing);
if `n` is 0, the result is the empty string.
Concatenation is performed following the usual semantics of Lua
(see [§2.5.4](004-2-5-1-arithmetic-operators.md#2.5.4)).

---

<a id="lua_cpcall"></a>

### `lua_cpcall`

[-0, +(0|1), *-*]



```
int lua_cpcall (lua_State *L, lua_CFunction func, void *ud);
```




Calls the C function `func` in protected mode.
`func` starts with only one element in its stack,
a light userdata containing `ud`.
In case of errors,
[`lua_cpcall`](009-lua-atpanic.md#lua_cpcall) returns the same error codes as [`lua_pcall`](011-lua-istable.md#lua_pcall),
plus the error object on the top of the stack;
otherwise, it returns zero, and does not change the stack.
All values returned by `func` are discarded.

---

<a id="lua_createtable"></a>

### `lua_createtable`

[-0, +1, *m*]



```
void lua_createtable (lua_State *L, int narr, int nrec);
```




Creates a new empty table and pushes it onto the stack.
The new table has space pre-allocated
for `narr` array elements and `nrec` non-array elements.
This pre-allocation is useful when you know exactly how many elements
the table will have.
Otherwise you can use the function [`lua_newtable`](011-lua-istable.md#lua_newtable).

---

<a id="lua_dump"></a>

### `lua_dump`

[-0, +0, *m*]



```
int lua_dump (lua_State *L, lua_Writer writer, void *data);
```




Dumps a function as a binary chunk.
Receives a Lua function on the top of the stack
and produces a binary chunk that,
if loaded again,
results in a function equivalent to the one dumped.
As it produces parts of the chunk,
[`lua_dump`](009-lua-atpanic.md#lua_dump) calls function `writer` (see [`lua_Writer`](014-lua-tocfunction.md#lua_Writer))
with the given `data`
to write them.

The value returned is the error code returned by the last
call to the writer;
0 means no errors.

This function does not pop the Lua function from the stack.

---

