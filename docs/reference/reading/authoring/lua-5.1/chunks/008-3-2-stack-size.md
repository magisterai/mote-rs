# 3.2 – Stack Size

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-2-10-2-weak-tables.md) · [Next](009-lua-atpanic.md)

<a id="3.2"></a>

## 3.2 –  Stack Size

When you interact with Lua API,
you are responsible for ensuring consistency.
In particular,
*you are responsible for controlling stack overflow*.
You can use the function [`lua_checkstack`](009-lua-atpanic.md#lua_checkstack)
to grow the stack size.

<a id="pdf-LUA_MINSTACK"></a>

Whenever Lua calls C,
it ensures that at least `LUA_MINSTACK` stack positions are available.
`LUA_MINSTACK` is defined as 20,
so that usually you do not have to worry about stack space
unless your code has loops pushing elements onto the stack.

Most query functions accept as indices any value inside the
available stack space, that is, indices up to the maximum stack size
you have set through [`lua_checkstack`](009-lua-atpanic.md#lua_checkstack).
Such indices are called *acceptable indices*.
More formally, we define an *acceptable index*
as follows:



```

     (index < 0 && abs(index) <= top) ||
     (index > 0 && index <= stackspace)
```




Note that 0 is never an acceptable index.

<a id="3.3"></a>

## 3.3 –  Pseudo-Indices

Unless otherwise noted,
any function that accepts valid indices can also be called with
*pseudo-indices*,
which represent some Lua values that are accessible to C code
but which are not in the stack.
Pseudo-indices are used to access the thread environment,
the function environment,
the registry,
and the upvalues of a C function (see [§3.4](008-3-2-stack-size.md#3.4)).

<a id="pdf-LUA_GLOBALSINDEX"></a>

<a id="pdf-LUA_ENVIRONINDEX"></a>

The thread environment (where global variables live) is
always at pseudo-index `LUA_GLOBALSINDEX`.
The environment of the running C function is always
at pseudo-index `LUA_ENVIRONINDEX`.

To access and change the value of global variables,
you can use regular table operations over an environment table.
For instance, to access the value of a global variable, do



```

     lua_getfield(L, LUA_GLOBALSINDEX, varname);
```




<a id="3.4"></a>

## 3.4 –  C Closures

When a C function is created,
it is possible to associate some values with it,
thus creating a *C closure*;
these values are called *upvalues* and are
accessible to the function whenever it is called
(see [`lua_pushcclosure`](012-lua-pop.md#lua_pushcclosure)).

<a id="lua_upvalueindex"></a>

Whenever a C function is called,
its upvalues are located at specific pseudo-indices.
These pseudo-indices are produced by the macro
`lua_upvalueindex`.
The first value associated with a function is at position
`lua_upvalueindex(1)`, and so on.
Any access to `lua_upvalueindex(n)`,
where *n* is greater than the number of upvalues of the
current function (but not greater than 256),
produces an acceptable (but invalid) index.

<a id="3.5"></a>

## 3.5 –  Registry

<a id="pdf-LUA_REGISTRYINDEX"></a>

Lua provides a *registry*,
a pre-defined table that can be used by any C code to
store whatever Lua value it needs to store.
This table is always located at pseudo-index
`LUA_REGISTRYINDEX`.
Any C library can store data into this table,
but it should take care to choose keys different from those used
by other libraries, to avoid collisions.
Typically, you should use as key a string containing your library name
or a light userdata with the address of a C object in your code.

The integer keys in the registry are used by the reference mechanism,
implemented by the auxiliary library,
and therefore should not be used for other purposes.

<a id="3.6"></a>

## 3.6 –  Error Handling in C

Internally, Lua uses the C `longjmp` facility to handle errors.
(You can also choose to use exceptions if you use C++;
see file `luaconf.h`.)
When Lua faces any error
(such as memory allocation errors, type errors, syntax errors,
and runtime errors)
it *raises* an error;
that is, it does a long jump.
A *protected environment* uses `setjmp`
to set a recover point;
any error jumps to the most recent active recover point.

Most functions in the API can throw an error,
for instance due to a memory allocation error.
The documentation for each function indicates whether
it can throw errors.

Inside a C function you can throw an error by calling [`lua_error`](010-lua-equal.md#lua_error).

<a id="3.7"></a>

## 3.7 –  Functions and Types

Here we list all functions and types from the C API in
alphabetical order.
Each function has an indicator like this:
[-o, +p, *x*]

The first field, `o`,
is how many elements the function pops from the stack.
The second field, `p`,
is how many elements the function pushes onto the stack.
(Any function always pushes its results after popping its arguments.)
A field in the form `x|y` means the function can push (or pop)
`x` or `y` elements,
depending on the situation;
an interrogation mark '`?`' means that
we cannot know how many elements the function pops/pushes
by looking only at its arguments
(e.g., they may depend on what is on the stack).
The third field, `x`,
tells whether the function may throw errors:
'`-`' means the function never throws any error;
'`m`' means the function may throw an error
only due to not enough memory;
'`e`' means the function may throw other kinds of errors;
'`v`' means the function may throw an error on purpose.

---

<a id="lua_Alloc"></a>

### `lua_Alloc`



```
typedef void * (*lua_Alloc) (void *ud,
                             void *ptr,
                             size_t osize,
                             size_t nsize);
```




The type of the memory-allocation function used by Lua states.
The allocator function must provide a
functionality similar to `realloc`,
but not exactly the same.
Its arguments are
`ud`, an opaque pointer passed to [`lua_newstate`](011-lua-istable.md#lua_newstate);
`ptr`, a pointer to the block being allocated/reallocated/freed;
`osize`, the original size of the block;
`nsize`, the new size of the block.
`ptr` is `NULL` if and only if `osize` is zero.
When `nsize` is zero, the allocator must return `NULL`;
if `osize` is not zero,
it should free the block pointed to by `ptr`.
When `nsize` is not zero, the allocator returns `NULL`
if and only if it cannot fill the request.
When `nsize` is not zero and `osize` is zero,
the allocator should behave like `malloc`.
When `nsize` and `osize` are not zero,
the allocator behaves like `realloc`.
Lua assumes that the allocator never fails when
`osize >= nsize`.

Here is a simple implementation for the allocator function.
It is used in the auxiliary library by [`luaL_newstate`](018-lual-error.md#luaL_newstate).



```

     static void *l_alloc (void *ud, void *ptr, size_t osize,
                                                size_t nsize) {
       (void)ud;  (void)osize;  /* not used */
       if (nsize == 0) {
         free(ptr);
         return NULL;
       }
       else
         return realloc(ptr, nsize);
     }
```




This code assumes
that `free(NULL)` has no effect and that
`realloc(NULL, size)` is equivalent to `malloc(size)`.
ANSI C ensures both behaviors.

---

