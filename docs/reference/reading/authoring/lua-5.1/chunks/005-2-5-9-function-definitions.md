# 2.5.9 – Function Definitions

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-2-5-1-arithmetic-operators.md) · [Next](006-2-9-environments.md)

<a id="2.5.9"></a>

### 2.5.9 –  Function Definitions

The syntax for function definition is



```

	function ::= function funcbody
	funcbody ::= `(´ [parlist] `)´ block end
```




The following syntactic sugar simplifies function definitions:



```

	stat ::= function funcname funcbody
	stat ::= local function Name funcbody
	funcname ::= Name {`.´ Name} [`:´ Name]
```




The statement



```

     function f () body end
```




translates to



```

     f = function () body end
```




The statement



```

     function t.a.b.c.f () body end
```




translates to



```

     t.a.b.c.f = function () body end
```




The statement



```

     local function f () body end
```




translates to



```

     local f; f = function () body end
```




*not* to



```

     local f = function () body end
```




(This only makes a difference when the body of the function
contains references to `f`.)

A function definition is an executable expression,
whose value has type *function*.
When Lua pre-compiles a chunk,
all its function bodies are pre-compiled too.
Then, whenever Lua executes the function definition,
the function is *instantiated* (or *closed*).
This function instance (or *closure*)
is the final value of the expression.
Different instances of the same function
can refer to different external local variables
and can have different environment tables.

Parameters act as local variables that are
initialized with the argument values:



```

	parlist ::= namelist [`,´ `...´] | `...´
```




When a function is called,
the list of arguments is adjusted to
the length of the list of parameters,
unless the function is a variadic or *vararg function*,
which is
indicated by three dots ('`...`') at the end of its parameter list.
A vararg function does not adjust its argument list;
instead, it collects all extra arguments and supplies them
to the function through a *vararg expression*,
which is also written as three dots.
The value of this expression is a list of all actual extra arguments,
similar to a function with multiple results.
If a vararg expression is used inside another expression
or in the middle of a list of expressions,
then its return list is adjusted to one element.
If the expression is used as the last element of a list of expressions,
then no adjustment is made
(unless that last expression is enclosed in parentheses).

As an example, consider the following definitions:



```

     function f(a, b) end
     function g(a, b, ...) end
     function r() return 1,2,3 end
```




Then, we have the following mapping from arguments to parameters and
to the vararg expression:



```

     CALL            PARAMETERS
     
     f(3)             a=3, b=nil
     f(3, 4)          a=3, b=4
     f(3, 4, 5)       a=3, b=4
     f(r(), 10)       a=1, b=10
     f(r())           a=1, b=2
     
     g(3)             a=3, b=nil, ... -->  (nothing)
     g(3, 4)          a=3, b=4,   ... -->  (nothing)
     g(3, 4, 5, 8)    a=3, b=4,   ... -->  5  8
     g(5, r())        a=5, b=1,   ... -->  2  3
```




Results are returned using the **return** statement (see [§2.4.4](003-2-4-4-control-structures.md#2.4.4)).
If control reaches the end of a function
without encountering a **return** statement,
then the function returns with no results.

The *colon* syntax
is used for defining *methods*,
that is, functions that have an implicit extra parameter `self`.
Thus, the statement



```

     function t.a.b.c:f (params) body end
```




is syntactic sugar for



```

     t.a.b.c.f = function (self, params) body end
```




<a id="2.6"></a>

## 2.6 –  Visibility Rules

Lua is a lexically scoped language.
The scope of variables begins at the first statement *after*
their declaration and lasts until the end of the innermost block that
includes the declaration.
Consider the following example:



```

     x = 10                -- global variable
     do                    -- new block
       local x = x         -- new 'x', with value 10
       print(x)            --> 10
       x = x+1
       do                  -- another block
         local x = x+1     -- another 'x'
         print(x)          --> 12
       end
       print(x)            --> 11
     end
     print(x)              --> 10  (the global one)
```




Notice that, in a declaration like `local x = x`,
the new `x` being declared is not in scope yet,
and so the second `x` refers to the outside variable.

Because of the lexical scoping rules,
local variables can be freely accessed by functions
defined inside their scope.
A local variable used by an inner function is called
an *upvalue*, or *external local variable*,
inside the inner function.

Notice that each execution of a **local** statement
defines new local variables.
Consider the following example:



```

     a = {}
     local x = 20
     for i=1,10 do
       local y = 0
       a[i] = function () y=y+1; return x+y end
     end
```




The loop creates ten closures
(that is, ten instances of the anonymous function).
Each of these closures uses a different `y` variable,
while all of them share the same `x`.

<a id="2.7"></a>

## 2.7 –  Error Handling

Because Lua is an embedded extension language,
all Lua actions start from C code in the host program
calling a function from the Lua library (see [`lua_pcall`](011-lua-istable.md#lua_pcall)).
Whenever an error occurs during Lua compilation or execution,
control returns to C,
which can take appropriate measures
(such as printing an error message).

Lua code can explicitly generate an error by calling the
[`error`](020-collectgarbage-opt-arg.md#pdf-error) function.
If you need to catch errors in Lua,
you can use the [`pcall`](020-collectgarbage-opt-arg.md#pdf-pcall) function.

<a id="2.8"></a>

## 2.8 –  Metatables

Every value in Lua can have a *metatable*.
This *metatable* is an ordinary Lua table
that defines the behavior of the original value
under certain special operations.
You can change several aspects of the behavior
of operations over a value by setting specific fields in its metatable.
For instance, when a non-numeric value is the operand of an addition,
Lua checks for a function in the field `"__add"` in its metatable.
If it finds one,
Lua calls this function to perform the addition.

We call the keys in a metatable *events*
and the values *metamethods*.
In the previous example, the event is `"add"`
and the metamethod is the function that performs the addition.

You can query the metatable of any value
through the [`getmetatable`](020-collectgarbage-opt-arg.md#pdf-getmetatable) function.

You can replace the metatable of tables
through the [`setmetatable`](021-rawequal-v1-v2.md#pdf-setmetatable)
function.
You cannot change the metatable of other types from Lua
(except by using the debug library);
you must use the C API for that.

Tables and full userdata have individual metatables
(although multiple tables and userdata can share their metatables).
Values of all other types share one single metatable per type;
that is, there is one single metatable for all numbers,
one for all strings, etc.

A metatable controls how an object behaves in arithmetic operations,
order comparisons, concatenation, length operation, and indexing.
A metatable also can define a function to be called when a userdata
is garbage collected.
For each of these operations Lua associates a specific key
called an *event*.
When Lua performs one of these operations over a value,
it checks whether this value has a metatable with the corresponding event.
If so, the value associated with that key (the metamethod)
controls how Lua will perform the operation.

Metatables control the operations listed next.
Each operation is identified by its corresponding name.
The key for each operation is a string with its name prefixed by
two underscores, '`__`';
for instance, the key for operation "add" is the
string `"__add"`.
The semantics of these operations is better explained by a Lua function
describing how the interpreter executes the operation.

The code shown here in Lua is only illustrative;
the real behavior is hard coded in the interpreter
and it is much more efficient than this simulation.
All functions used in these descriptions
([`rawget`](021-rawequal-v1-v2.md#pdf-rawget), [`tonumber`](021-rawequal-v1-v2.md#pdf-tonumber), etc.)
are described in [§5.1](019-lual-pushresult.md#5.1).
In particular, to retrieve the metamethod of a given object,
we use the expression



```

     metatable(obj)[event]
```




This should be read as



```

     rawget(getmetatable(obj) or {}, event)
```




That is, the access to a metamethod does not invoke other metamethods,
and the access to objects with no metatables does not fail
(it simply results in **nil**).

- **"add":**
  the `+` operation.

  The function `getbinhandler` below defines how Lua chooses a handler
  for a binary operation.
  First, Lua tries the first operand.
  If its type does not define a handler for the operation,
  then Lua tries the second operand.



```

     function getbinhandler (op1, op2, event)
       return metatable(op1)[event] or metatable(op2)[event]
     end
```




  By using this function,
  the behavior of the `op1 + op2` is



```

     function add_event (op1, op2)
       local o1, o2 = tonumber(op1), tonumber(op2)
       if o1 and o2 then  -- both operands are numeric?
         return o1 + o2   -- '+' here is the primitive 'add'
       else  -- at least one of the operands is not numeric
         local h = getbinhandler(op1, op2, "__add")
         if h then
           -- call the handler with both operands
           return (h(op1, op2))
         else  -- no handler available: default behavior
           error(···)
         end
       end
     end
```



- **"sub":**
  the `-` operation.
  Behavior similar to the "add" operation.
- **"mul":**
  the `*` operation.
  Behavior similar to the "add" operation.
- **"div":**
  the `/` operation.
  Behavior similar to the "add" operation.
- **"mod":**
  the `%` operation.
  Behavior similar to the "add" operation,
  with the operation
  `o1 - floor(o1/o2)*o2` as the primitive operation.
- **"pow":**
  the `^` (exponentiation) operation.
  Behavior similar to the "add" operation,
  with the function `pow` (from the C math library)
  as the primitive operation.
- **"unm":**
  the unary `-` operation.



```

     function unm_event (op)
       local o = tonumber(op)
       if o then  -- operand is numeric?
         return -o  -- '-' here is the primitive 'unm'
       else  -- the operand is not numeric.
         -- Try to get a handler from the operand
         local h = metatable(op).__unm
         if h then
           -- call the handler with the operand
           return (h(op))
         else  -- no handler available: default behavior
           error(···)
         end
       end
     end
```



- **"concat":**
  the `..` (concatenation) operation.



```

     function concat_event (op1, op2)
       if (type(op1) == "string" or type(op1) == "number") and
          (type(op2) == "string" or type(op2) == "number") then
         return op1 .. op2  -- primitive string concatenation
       else
         local h = getbinhandler(op1, op2, "__concat")
         if h then
           return (h(op1, op2))
         else
           error(···)
         end
       end
     end
```



- **"len":**
  the `#` operation.



```

     function len_event (op)
       if type(op) == "string" then
         return strlen(op)         -- primitive string length
       elseif type(op) == "table" then
         return #op                -- primitive table length
       else
         local h = metatable(op).__len
         if h then
           -- call the handler with the operand
           return (h(op))
         else  -- no handler available: default behavior
           error(···)
         end
       end
     end
```




  See [§2.5.5](004-2-5-1-arithmetic-operators.md#2.5.5) for a description of the length of a table.
- **"eq":**
  the `==` operation.
  The function `getcomphandler` defines how Lua chooses a metamethod
  for comparison operators.
  A metamethod only is selected when both objects
  being compared have the same type
  and the same metamethod for the selected operation.



```

     function getcomphandler (op1, op2, event)
       if type(op1) ~= type(op2) then return nil end
       local mm1 = metatable(op1)[event]
       local mm2 = metatable(op2)[event]
       if mm1 == mm2 then return mm1 else return nil end
     end
```




  The "eq" event is defined as follows:



```

     function eq_event (op1, op2)
       if type(op1) ~= type(op2) then  -- different types?
         return false   -- different objects
       end
       if op1 == op2 then   -- primitive equal?
         return true   -- objects are equal
       end
       -- try metamethod
       local h = getcomphandler(op1, op2, "__eq")
       if h then
         return (h(op1, op2))
       else
         return false
       end
     end
```




  `a ~= b` is equivalent to `not (a == b)`.
- **"lt":**
  the `<` operation.



```

     function lt_event (op1, op2)
       if type(op1) == "number" and type(op2) == "number" then
         return op1 < op2   -- numeric comparison
       elseif type(op1) == "string" and type(op2) == "string" then
         return op1 < op2   -- lexicographic comparison
       else
         local h = getcomphandler(op1, op2, "__lt")
         if h then
           return (h(op1, op2))
         else
           error(···)
         end
       end
     end
```




  `a > b` is equivalent to `b < a`.
- **"le":**
  the `<=` operation.



