# 2.4.4 – Control Structures

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-2-2-1-coercion.md) · [Next](004-2-5-1-arithmetic-operators.md)

<a id="2.4.4"></a>

### 2.4.4 –  Control Structures

The control structures
**if**, **while**, and **repeat** have the usual meaning and
familiar syntax:



```

	stat ::= while exp do block end
	stat ::= repeat block until exp
	stat ::= if exp then block {elseif exp then block} [else block] end
```




Lua also has a **for** statement, in two flavors (see [§2.4.5](003-2-4-4-control-structures.md#2.4.5)).

The condition expression of a
control structure can return any value.
Both **false** and **nil** are considered false.
All values different from **nil** and **false** are considered true
(in particular, the number 0 and the empty string are also true).

In the **repeat**–**until** loop,
the inner block does not end at the **until** keyword,
but only after the condition.
So, the condition can refer to local variables
declared inside the loop block.

The **return** statement is used to return values
from a function or a chunk (which is just a function).
Functions and chunks can return more than one value,
and so the syntax for the **return** statement is



```

	stat ::= return [explist]
```




The **break** statement is used to terminate the execution of a
**while**, **repeat**, or **for** loop,
skipping to the next statement after the loop:



```

	stat ::= break
```




A **break** ends the innermost enclosing loop.

The **return** and **break**
statements can only be written as the *last* statement of a block.
If it is really necessary to **return** or **break** in the
middle of a block,
then an explicit inner block can be used,
as in the idioms
`do return end` and `do break end`,
because now **return** and **break** are the last statements in
their (inner) blocks.

<a id="2.4.5"></a>

### 2.4.5 –  For Statement

The **for** statement has two forms:
one numeric and one generic.

The numeric **for** loop repeats a block of code while a
control variable runs through an arithmetic progression.
It has the following syntax:



```

	stat ::= for Name `=´ exp `,´ exp [`,´ exp] do block end
```




The *block* is repeated for *name* starting at the value of
the first *exp*, until it passes the second *exp* by steps of the
third *exp*.
More precisely, a **for** statement like



```

     for v = e1, e2, e3 do block end
```




is equivalent to the code:



```

     do
       local var, limit, step = tonumber(e1), tonumber(e2), tonumber(e3)
       if not (var and limit and step) then error() end
       while (step > 0 and var <= limit) or (step <= 0 and var >= limit) do
         local v = var
         block
         var = var + step
       end
     end
```




Note the following:

- All three control expressions are evaluated only once,
  before the loop starts.
  They must all result in numbers.
- `var`, `limit`, and `step` are invisible variables.
  The names shown here are for explanatory purposes only.
- If the third expression (the step) is absent,
  then a step of 1 is used.
- You can use **break** to exit a **for** loop.
- The loop variable `v` is local to the loop;
  you cannot use its value after the **for** ends or is broken.
  If you need this value,
  assign it to another variable before breaking or exiting the loop.

The generic **for** statement works over functions,
called *iterators*.
On each iteration, the iterator function is called to produce a new value,
stopping when this new value is **nil**.
The generic **for** loop has the following syntax:



```

	stat ::= for namelist in explist do block end
	namelist ::= Name {`,´ Name}
```




A **for** statement like



```

     for var_1, ···, var_n in explist do block end
```




is equivalent to the code:



```

     do
       local f, s, var = explist
       while true do
         local var_1, ···, var_n = f(s, var)
         var = var_1
         if var == nil then break end
         block
       end
     end
```




Note the following:

- `explist` is evaluated only once.
  Its results are an *iterator* function,
  a *state*,
  and an initial value for the first *iterator variable*.
- `f`, `s`, and `var` are invisible variables.
  The names are here for explanatory purposes only.
- You can use **break** to exit a **for** loop.
- The loop variables `var_i` are local to the loop;
  you cannot use their values after the **for** ends.
  If you need these values,
  then assign them to other variables before breaking or exiting the loop.

<a id="2.4.6"></a>

### 2.4.6 –  Function Calls as Statements

To allow possible side-effects,
function calls can be executed as statements:



```

	stat ::= functioncall
```




In this case, all returned values are thrown away.
Function calls are explained in [§2.5.8](004-2-5-1-arithmetic-operators.md#2.5.8).

<a id="2.4.7"></a>

### 2.4.7 –  Local Declarations

Local variables can be declared anywhere inside a block.
The declaration can include an initial assignment:



```

	stat ::= local namelist [`=´ explist]
```




If present, an initial assignment has the same semantics
of a multiple assignment (see [§2.4.3](002-2-2-1-coercion.md#2.4.3)).
Otherwise, all variables are initialized with **nil**.

A chunk is also a block (see [§2.4.1](002-2-2-1-coercion.md#2.4.1)),
and so local variables can be declared in a chunk outside any explicit block.
The scope of such local variables extends until the end of the chunk.

The visibility rules for local variables are explained in [§2.6](005-2-5-9-function-definitions.md#2.6).

<a id="2.5"></a>

## 2.5 –  Expressions

The basic expressions in Lua are the following:



```

	exp ::= prefixexp
	exp ::= nil | false | true
	exp ::= Number
	exp ::= String
	exp ::= function
	exp ::= tableconstructor
	exp ::= `...´
	exp ::= exp binop exp
	exp ::= unop exp
	prefixexp ::= var | functioncall | `(´ exp `)´
```




Numbers and literal strings are explained in [§2.1](001-lua-lua-5-1-reference-manual.md#2.1);
variables are explained in [§2.3](002-2-2-1-coercion.md#2.3);
function definitions are explained in [§2.5.9](005-2-5-9-function-definitions.md#2.5.9);
function calls are explained in [§2.5.8](004-2-5-1-arithmetic-operators.md#2.5.8);
table constructors are explained in [§2.5.7](004-2-5-1-arithmetic-operators.md#2.5.7).
Vararg expressions,
denoted by three dots ('`...`'), can only be used when
directly inside a vararg function;
they are explained in [§2.5.9](005-2-5-9-function-definitions.md#2.5.9).

Binary operators comprise arithmetic operators (see [§2.5.1](004-2-5-1-arithmetic-operators.md#2.5.1)),
relational operators (see [§2.5.2](004-2-5-1-arithmetic-operators.md#2.5.2)), logical operators (see [§2.5.3](004-2-5-1-arithmetic-operators.md#2.5.3)),
and the concatenation operator (see [§2.5.4](004-2-5-1-arithmetic-operators.md#2.5.4)).
Unary operators comprise the unary minus (see [§2.5.1](004-2-5-1-arithmetic-operators.md#2.5.1)),
the unary **not** (see [§2.5.3](004-2-5-1-arithmetic-operators.md#2.5.3)),
and the unary *length operator* (see [§2.5.5](004-2-5-1-arithmetic-operators.md#2.5.5)).

Both function calls and vararg expressions can result in multiple values.
If an expression is used as a statement
(only possible for function calls (see [§2.4.6](003-2-4-4-control-structures.md#2.4.6))),
then its return list is adjusted to zero elements,
thus discarding all returned values.
If an expression is used as the last (or the only) element
of a list of expressions,
then no adjustment is made
(unless the call is enclosed in parentheses).
In all other contexts,
Lua adjusts the result list to one element,
discarding all values except the first one.

Here are some examples:



```

     f()                -- adjusted to 0 results
     g(f(), x)          -- f() is adjusted to 1 result
     g(x, f())          -- g gets x plus all results from f()
     a,b,c = f(), x     -- f() is adjusted to 1 result (c gets nil)
     a,b = ...          -- a gets the first vararg parameter, b gets
                        -- the second (both a and b can get nil if there
                        -- is no corresponding vararg parameter)
     
     a,b,c = x, f()     -- f() is adjusted to 2 results
     a,b,c = f()        -- f() is adjusted to 3 results
     return f()         -- returns all results from f()
     return ...         -- returns all received vararg parameters
     return x,y,f()     -- returns x, y, and all results from f()
     {f()}              -- creates a list with all results from f()
     {...}              -- creates a list with all vararg parameters
     {f(), nil}         -- f() is adjusted to 1 result
```




Any expression enclosed in parentheses always results in only one value.
Thus,
`(f(x,y,z))` is always a single value,
even if `f` returns several values.
(The value of `(f(x,y,z))` is the first value returned by `f`
or **nil** if `f` does not return any values.)

