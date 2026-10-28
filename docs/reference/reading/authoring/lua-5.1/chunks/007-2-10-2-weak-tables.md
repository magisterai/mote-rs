# 2.10.2 – Weak Tables

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-2-9-environments.md) · [Next](008-3-2-stack-size.md)

<a id="2.10.2"></a>

### 2.10.2 –  Weak Tables

A *weak table* is a table whose elements are
*weak references*.
A weak reference is ignored by the garbage collector.
In other words,
if the only references to an object are weak references,
then the garbage collector will collect this object.

A weak table can have weak keys, weak values, or both.
A table with weak keys allows the collection of its keys,
but prevents the collection of its values.
A table with both weak keys and weak values allows the collection of
both keys and values.
In any case, if either the key or the value is collected,
the whole pair is removed from the table.
The weakness of a table is controlled by the
`__mode` field of its metatable.
If the `__mode` field is a string containing the character '`k`',
the keys in the table are weak.
If `__mode` contains '`v`',
the values in the table are weak.

After you use a table as a metatable,
you should not change the value of its `__mode` field.
Otherwise, the weak behavior of the tables controlled by this
metatable is undefined.

<a id="2.11"></a>

## 2.11 –  Coroutines

Lua supports coroutines,
also called *collaborative multithreading*.
A coroutine in Lua represents an independent thread of execution.
Unlike threads in multithread systems, however,
a coroutine only suspends its execution by explicitly calling
a yield function.

You create a coroutine with a call to [`coroutine.create`](021-rawequal-v1-v2.md#pdf-coroutine.create).
Its sole argument is a function
that is the main function of the coroutine.
The `create` function only creates a new coroutine and
returns a handle to it (an object of type *thread*);
it does not start the coroutine execution.

When you first call [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume),
passing as its first argument
a thread returned by [`coroutine.create`](021-rawequal-v1-v2.md#pdf-coroutine.create),
the coroutine starts its execution,
at the first line of its main function.
Extra arguments passed to [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume) are passed on
to the coroutine main function.
After the coroutine starts running,
it runs until it terminates or *yields*.

A coroutine can terminate its execution in two ways:
normally, when its main function returns
(explicitly or implicitly, after the last instruction);
and abnormally, if there is an unprotected error.
In the first case, [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume) returns **true**,
plus any values returned by the coroutine main function.
In case of errors, [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume) returns **false**
plus an error message.

A coroutine yields by calling [`coroutine.yield`](021-rawequal-v1-v2.md#pdf-coroutine.yield).
When a coroutine yields,
the corresponding [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume) returns immediately,
even if the yield happens inside nested function calls
(that is, not in the main function,
but in a function directly or indirectly called by the main function).
In the case of a yield, [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume) also returns **true**,
plus any values passed to [`coroutine.yield`](021-rawequal-v1-v2.md#pdf-coroutine.yield).
The next time you resume the same coroutine,
it continues its execution from the point where it yielded,
with the call to [`coroutine.yield`](021-rawequal-v1-v2.md#pdf-coroutine.yield) returning any extra
arguments passed to [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume).

Like [`coroutine.create`](021-rawequal-v1-v2.md#pdf-coroutine.create),
the [`coroutine.wrap`](021-rawequal-v1-v2.md#pdf-coroutine.wrap) function also creates a coroutine,
but instead of returning the coroutine itself,
it returns a function that, when called, resumes the coroutine.
Any arguments passed to this function
go as extra arguments to [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume).
[`coroutine.wrap`](021-rawequal-v1-v2.md#pdf-coroutine.wrap) returns all the values returned by [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume),
except the first one (the boolean error code).
Unlike [`coroutine.resume`](021-rawequal-v1-v2.md#pdf-coroutine.resume),
[`coroutine.wrap`](021-rawequal-v1-v2.md#pdf-coroutine.wrap) does not catch errors;
any error is propagated to the caller.

As an example,
consider the following code:



```

     function foo (a)
       print("foo", a)
       return coroutine.yield(2*a)
     end
     
     co = coroutine.create(function (a,b)
           print("co-body", a, b)
           local r = foo(a+1)
           print("co-body", r)
           local r, s = coroutine.yield(a+b, a-b)
           print("co-body", r, s)
           return b, "end"
     end)
            
     print("main", coroutine.resume(co, 1, 10))
     print("main", coroutine.resume(co, "r"))
     print("main", coroutine.resume(co, "x", "y"))
     print("main", coroutine.resume(co, "x", "y"))
```




When you run it, it produces the following output:



```

     co-body 1       10
     foo     2
     
     main    true    4
     co-body r
     main    true    11      -9
     co-body x       y
     main    true    10      end
     main    false   cannot resume dead coroutine
```




<a id="3"></a>

# 3 –  The Application Program Interface

<a id="pdf-lua.h"></a>

This section describes the C API for Lua, that is,
the set of C functions available to the host program to communicate
with Lua.
All API functions and related types and constants
are declared in the header file `lua.h`.

Even when we use the term "function",
any facility in the API may be provided as a macro instead.
All such macros use each of their arguments exactly once
(except for the first argument, which is always a Lua state),
and so do not generate any hidden side-effects.

<a id="pdf-luai_apicheck"></a>

As in most C libraries,
the Lua API functions do not check their arguments for validity or consistency.
However, you can change this behavior by compiling Lua
with a proper definition for the macro `luai_apicheck`,
in file `luaconf.h`.

<a id="3.1"></a>

## 3.1 –  The Stack

Lua uses a *virtual stack* to pass values to and from C.
Each element in this stack represents a Lua value
(**nil**, number, string, etc.).

Whenever Lua calls C, the called function gets a new stack,
which is independent of previous stacks and of stacks of
C functions that are still active.
This stack initially contains any arguments to the C function
and it is where the C function pushes its results
to be returned to the caller (see [`lua_CFunction`](009-lua-atpanic.md#lua_CFunction)).

For convenience,
most query operations in the API do not follow a strict stack discipline.
Instead, they can refer to any element in the stack
by using an *index*:
A positive index represents an *absolute* stack position
(starting at 1);
a negative index represents an *offset* relative to the top of the stack.
More specifically, if the stack has *n* elements,
then index 1 represents the first element
(that is, the element that was pushed onto the stack first)
and
index *n* represents the last element;
index -1 also represents the last element
(that is, the element at the top)
and index *-n* represents the first element.
We say that an index is *valid*
if it lies between 1 and the stack top
(that is, if `1 ≤ abs(index) ≤ top`).

