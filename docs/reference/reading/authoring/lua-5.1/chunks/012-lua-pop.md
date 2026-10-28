# `lua_pop`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-lua-istable.md) · [Next](013-lua-rawseti.md)

<a id="lua_pop"></a>

### `lua_pop`

[-n, +0, *-*]



```
void lua_pop (lua_State *L, int n);
```




Pops `n` elements from the stack.

---

<a id="lua_pushboolean"></a>

### `lua_pushboolean`

[-0, +1, *-*]



```
void lua_pushboolean (lua_State *L, int b);
```




Pushes a boolean value with value `b` onto the stack.

---

<a id="lua_pushcclosure"></a>

### `lua_pushcclosure`

[-n, +1, *m*]



```
void lua_pushcclosure (lua_State *L, lua_CFunction fn, int n);
```




Pushes a new C closure onto the stack.

When a C function is created,
it is possible to associate some values with it,
thus creating a C closure (see [§3.4](008-3-2-stack-size.md#3.4));
these values are then accessible to the function whenever it is called.
To associate values with a C function,
first these values should be pushed onto the stack
(when there are multiple values, the first value is pushed first).
Then [`lua_pushcclosure`](012-lua-pop.md#lua_pushcclosure)
is called to create and push the C function onto the stack,
with the argument `n` telling how many values should be
associated with the function.
[`lua_pushcclosure`](012-lua-pop.md#lua_pushcclosure) also pops these values from the stack.

The maximum value for `n` is 255.

---

<a id="lua_pushcfunction"></a>

### `lua_pushcfunction`

[-0, +1, *m*]



```
void lua_pushcfunction (lua_State *L, lua_CFunction f);
```




Pushes a C function onto the stack.
This function receives a pointer to a C function
and pushes onto the stack a Lua value of type `function` that,
when called, invokes the corresponding C function.

Any function to be registered in Lua must
follow the correct protocol to receive its parameters
and return its results (see [`lua_CFunction`](009-lua-atpanic.md#lua_CFunction)).

`lua_pushcfunction` is defined as a macro:



```

     #define lua_pushcfunction(L,f)  lua_pushcclosure(L,f,0)
```




---

<a id="lua_pushfstring"></a>

### `lua_pushfstring`

[-0, +1, *m*]



```
const char *lua_pushfstring (lua_State *L, const char *fmt, ...);
```




Pushes onto the stack a formatted string
and returns a pointer to this string.
It is similar to the C function `sprintf`,
but has some important differences:

- You do not have to allocate space for the result:
  the result is a Lua string and Lua takes care of memory allocation
  (and deallocation, through garbage collection).
- The conversion specifiers are quite restricted.
  There are no flags, widths, or precisions.
  The conversion specifiers can only be
  '`%%`' (inserts a '`%`' in the string),
  '`%s`' (inserts a zero-terminated string, with no size restrictions),
  '`%f`' (inserts a [`lua_Number`](011-lua-istable.md#lua_Number)),
  '`%p`' (inserts a pointer as a hexadecimal numeral),
  '`%d`' (inserts an `int`), and
  '`%c`' (inserts an `int` as a character).

---

<a id="lua_pushinteger"></a>

### `lua_pushinteger`

[-0, +1, *-*]



```
void lua_pushinteger (lua_State *L, lua_Integer n);
```




Pushes a number with value `n` onto the stack.

---

<a id="lua_pushlightuserdata"></a>

### `lua_pushlightuserdata`

[-0, +1, *-*]



```
void lua_pushlightuserdata (lua_State *L, void *p);
```




Pushes a light userdata onto the stack.

Userdata represent C values in Lua.
A *light userdata* represents a pointer.
It is a value (like a number):
you do not create it, it has no individual metatable,
and it is not collected (as it was never created).
A light userdata is equal to "any"
light userdata with the same C address.

---

<a id="lua_pushliteral"></a>

### `lua_pushliteral`

[-0, +1, *m*]



```
void lua_pushliteral (lua_State *L, const char *s);
```




This macro is equivalent to [`lua_pushlstring`](012-lua-pop.md#lua_pushlstring),
but can be used only when `s` is a literal string.
In these cases, it automatically provides the string length.

---

<a id="lua_pushlstring"></a>

### `lua_pushlstring`

[-0, +1, *m*]



```
void lua_pushlstring (lua_State *L, const char *s, size_t len);
```




Pushes the string pointed to by `s` with size `len`
onto the stack.
Lua makes (or reuses) an internal copy of the given string,
so the memory at `s` can be freed or reused immediately after
the function returns.
The string can contain embedded zeros.

---

<a id="lua_pushnil"></a>

### `lua_pushnil`

[-0, +1, *-*]



```
void lua_pushnil (lua_State *L);
```




Pushes a nil value onto the stack.

---

<a id="lua_pushnumber"></a>

### `lua_pushnumber`

[-0, +1, *-*]



```
void lua_pushnumber (lua_State *L, lua_Number n);
```




Pushes a number with value `n` onto the stack.

---

<a id="lua_pushstring"></a>

### `lua_pushstring`

[-0, +1, *m*]



```
void lua_pushstring (lua_State *L, const char *s);
```




Pushes the zero-terminated string pointed to by `s`
onto the stack.
Lua makes (or reuses) an internal copy of the given string,
so the memory at `s` can be freed or reused immediately after
the function returns.
The string cannot contain embedded zeros;
it is assumed to end at the first zero.

---

<a id="lua_pushthread"></a>

### `lua_pushthread`

[-0, +1, *-*]



```
int lua_pushthread (lua_State *L);
```




Pushes the thread represented by `L` onto the stack.
Returns 1 if this thread is the main thread of its state.

---

<a id="lua_pushvalue"></a>

### `lua_pushvalue`

[-0, +1, *-*]



```
void lua_pushvalue (lua_State *L, int index);
```




Pushes a copy of the element at the given valid index
onto the stack.

---

<a id="lua_pushvfstring"></a>

### `lua_pushvfstring`

[-0, +1, *m*]



```
const char *lua_pushvfstring (lua_State *L,
                              const char *fmt,
                              va_list argp);
```




Equivalent to [`lua_pushfstring`](012-lua-pop.md#lua_pushfstring), except that it receives a `va_list`
instead of a variable number of arguments.

---

<a id="lua_rawequal"></a>

### `lua_rawequal`

[-0, +0, *-*]



```
int lua_rawequal (lua_State *L, int index1, int index2);
```




Returns 1 if the two values in acceptable indices `index1` and
`index2` are primitively equal
(that is, without calling metamethods).
Otherwise returns 0.
Also returns 0 if any of the indices are non valid.

---

<a id="lua_rawget"></a>

### `lua_rawget`

[-1, +1, *-*]



```
void lua_rawget (lua_State *L, int index);
```




Similar to [`lua_gettable`](010-lua-equal.md#lua_gettable), but does a raw access
(i.e., without metamethods).

---

<a id="lua_rawgeti"></a>

### `lua_rawgeti`

[-0, +1, *-*]



```
void lua_rawgeti (lua_State *L, int index, int n);
```




Pushes onto the stack the value `t[n]`,
where `t` is the value at the given valid index.
The access is raw;
that is, it does not invoke metamethods.

---

<a id="lua_rawset"></a>

### `lua_rawset`

[-2, +0, *m*]



```
void lua_rawset (lua_State *L, int index);
```




Similar to [`lua_settable`](013-lua-rawseti.md#lua_settable), but does a raw assignment
(i.e., without metamethods).

---

