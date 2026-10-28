# 2.9 – Environments

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-2-5-9-function-definitions.md) · [Next](007-2-10-2-weak-tables.md)

```

     function le_event (op1, op2)
       if type(op1) == "number" and type(op2) == "number" then
         return op1 <= op2   -- numeric comparison
       elseif type(op1) == "string" and type(op2) == "string" then
         return op1 <= op2   -- lexicographic comparison
       else
         local h = getcomphandler(op1, op2, "__le")
         if h then
           return (h(op1, op2))
         else
           h = getcomphandler(op1, op2, "__lt")
           if h then
             return not h(op2, op1)
           else
             error(···)
           end
         end
       end
     end
```




  `a >= b` is equivalent to `b <= a`.
  Note that, in the absence of a "le" metamethod,
  Lua tries the "lt", assuming that `a <= b` is
  equivalent to `not (b < a)`.
- **"index":**
  The indexing access `table[key]`.



```

     function gettable_event (table, key)
       local h
       if type(table) == "table" then
         local v = rawget(table, key)
         if v ~= nil then return v end
         h = metatable(table).__index
         if h == nil then return nil end
       else
         h = metatable(table).__index
         if h == nil then
           error(···)
         end
       end
       if type(h) == "function" then
         return (h(table, key))     -- call the handler
       else return h[key]           -- or repeat operation on it
       end
     end
```



- **"newindex":**
  The indexing assignment `table[key] = value`.



```

     function settable_event (table, key, value)
       local h
       if type(table) == "table" then
         local v = rawget(table, key)
         if v ~= nil then rawset(table, key, value); return end
         h = metatable(table).__newindex
         if h == nil then rawset(table, key, value); return end
       else
         h = metatable(table).__newindex
         if h == nil then
           error(···)
         end
       end
       if type(h) == "function" then
         h(table, key,value)           -- call the handler
       else h[key] = value             -- or repeat operation on it
       end
     end
```



- **"call":**
  called when Lua calls a value.



```

     function function_event (func, ...)
       if type(func) == "function" then
         return func(...)   -- primitive call
       else
         local h = metatable(func).__call
         if h then
           return h(func, ...)
         else
           error(···)
         end
       end
     end
```




<a id="2.9"></a>

## 2.9 –  Environments

Besides metatables,
objects of types thread, function, and userdata
have another table associated with them,
called their *environment*.
Like metatables, environments are regular tables and
multiple objects can share the same environment.

Threads are created sharing the environment of the creating thread.
Userdata and C functions are created sharing the environment
of the creating C function.
Non-nested Lua functions
(created by [`loadfile`](020-collectgarbage-opt-arg.md#pdf-loadfile), [`loadstring`](020-collectgarbage-opt-arg.md#pdf-loadstring) or [`load`](020-collectgarbage-opt-arg.md#pdf-load))
are created sharing the environment of the creating thread.
Nested Lua functions are created sharing the environment of
the creating Lua function.

Environments associated with userdata have no meaning for Lua.
It is only a convenience feature for programmers to associate a table to
a userdata.

Environments associated with threads are called
*global environments*.
They are used as the default environment for threads and
non-nested Lua functions created by the thread
and can be directly accessed by C code (see [§3.3](008-3-2-stack-size.md#3.3)).

The environment associated with a C function can be directly
accessed by C code (see [§3.3](008-3-2-stack-size.md#3.3)).
It is used as the default environment for other C functions
and userdata created by the function.

Environments associated with Lua functions are used to resolve
all accesses to global variables within the function (see [§2.3](002-2-2-1-coercion.md#2.3)).
They are used as the default environment for nested Lua functions
created by the function.

You can change the environment of a Lua function or the
running thread by calling [`setfenv`](021-rawequal-v1-v2.md#pdf-setfenv).
You can get the environment of a Lua function or the running thread
by calling [`getfenv`](020-collectgarbage-opt-arg.md#pdf-getfenv).
To manipulate the environment of other objects
(userdata, C functions, other threads) you must
use the C API.

<a id="2.10"></a>

## 2.10 –  Garbage Collection

Lua performs automatic memory management.
This means that
you have to worry neither about allocating memory for new objects
nor about freeing it when the objects are no longer needed.
Lua manages memory automatically by running
a *garbage collector* from time to time
to collect all *dead objects*
(that is, objects that are no longer accessible from Lua).
All memory used by Lua is subject to automatic management:
tables, userdata, functions, threads, strings, etc.

Lua implements an incremental mark-and-sweep collector.
It uses two numbers to control its garbage-collection cycles:
the *garbage-collector pause* and
the *garbage-collector step multiplier*.
Both use percentage points as units
(so that a value of 100 means an internal value of 1).

The garbage-collector pause
controls how long the collector waits before starting a new cycle.
Larger values make the collector less aggressive.
Values smaller than 100 mean the collector will not wait to
start a new cycle.
A value of 200 means that the collector waits for the total memory in use
to double before starting a new cycle.

The step multiplier
controls the relative speed of the collector relative to
memory allocation.
Larger values make the collector more aggressive but also increase
the size of each incremental step.
Values smaller than 100 make the collector too slow and
can result in the collector never finishing a cycle.
The default, 200, means that the collector runs at "twice"
the speed of memory allocation.

You can change these numbers by calling [`lua_gc`](010-lua-equal.md#lua_gc) in C
or [`collectgarbage`](020-collectgarbage-opt-arg.md#pdf-collectgarbage) in Lua.
With these functions you can also control
the collector directly (e.g., stop and restart it).

<a id="2.10.1"></a>

### 2.10.1 –  Garbage-Collection Metamethods

Using the C API,
you can set garbage-collector metamethods for userdata (see [§2.8](005-2-5-9-function-definitions.md#2.8)).
These metamethods are also called *finalizers*.
Finalizers allow you to coordinate Lua's garbage collection
with external resource management
(such as closing files, network or database connections,
or freeing your own memory).

Garbage userdata with a field `__gc` in their metatables are not
collected immediately by the garbage collector.
Instead, Lua puts them in a list.
After the collection,
Lua does the equivalent of the following function
for each userdata in that list:



```

     function gc_event (udata)
       local h = metatable(udata).__gc
       if h then
         h(udata)
       end
     end
```




At the end of each garbage-collection cycle,
the finalizers for userdata are called in *reverse*
order of their creation,
among those collected in that cycle.
That is, the first finalizer to be called is the one associated
with the userdata created last in the program.
The userdata itself is freed only in the next garbage-collection cycle.

