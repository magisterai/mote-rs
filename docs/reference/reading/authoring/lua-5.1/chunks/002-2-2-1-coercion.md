# 2.2.1 – Coercion

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-lua-lua-5-1-reference-manual.md) · [Next](003-2-4-4-control-structures.md)

<a id="2.2.1"></a>

### 2.2.1 –  Coercion

Lua provides automatic conversion between
string and number values at run time.
Any arithmetic operation applied to a string tries to convert
this string to a number, following the usual conversion rules.
Conversely, whenever a number is used where a string is expected,
the number is converted to a string, in a reasonable format.
For complete control over how numbers are converted to strings,
use the `format` function from the string library
(see [`string.format`](023-package-path.md#pdf-string.format)).

<a id="2.3"></a>

## 2.3 –  Variables

Variables are places that store values.
There are three kinds of variables in Lua:
global variables, local variables, and table fields.

A single name can denote a global variable or a local variable
(or a function's formal parameter,
which is a particular kind of local variable):



```

	var ::= Name
```




Name denotes identifiers, as defined in [§2.1](001-lua-lua-5-1-reference-manual.md#2.1).

Any variable is assumed to be global unless explicitly declared
as a local (see [§2.4.7](003-2-4-4-control-structures.md#2.4.7)).
Local variables are *lexically scoped*:
local variables can be freely accessed by functions
defined inside their scope (see [§2.6](005-2-5-9-function-definitions.md#2.6)).

Before the first assignment to a variable, its value is **nil**.

Square brackets are used to index a table:



```

	var ::= prefixexp `[´ exp `]´
```




The meaning of accesses to global variables
and table fields can be changed via metatables.
An access to an indexed variable `t[i]` is equivalent to
a call `gettable_event(t,i)`.
(See [§2.8](005-2-5-9-function-definitions.md#2.8) for a complete description of the
`gettable_event` function.
This function is not defined or callable in Lua.
We use it here only for explanatory purposes.)

The syntax `var.Name` is just syntactic sugar for
`var["Name"]`:



```

	var ::= prefixexp `.´ Name
```




All global variables live as fields in ordinary Lua tables,
called *environment tables* or simply
*environments* (see [§2.9](006-2-9-environments.md#2.9)).
Each function has its own reference to an environment,
so that all global variables in this function
will refer to this environment table.
When a function is created,
it inherits the environment from the function that created it.
To get the environment table of a Lua function,
you call [`getfenv`](020-collectgarbage-opt-arg.md#pdf-getfenv).
To replace it,
you call [`setfenv`](021-rawequal-v1-v2.md#pdf-setfenv).
(You can only manipulate the environment of C functions
through the debug library; (see [§5.9](027-os-date-format-time.md#5.9)).)

An access to a global variable `x`
is equivalent to `_env.x`,
which in turn is equivalent to



```

     gettable_event(_env, "x")
```




where `_env` is the environment of the running function.
(See [§2.8](005-2-5-9-function-definitions.md#2.8) for a complete description of the
`gettable_event` function.
This function is not defined or callable in Lua.
Similarly, the `_env` variable is not defined in Lua.
We use them here only for explanatory purposes.)

<a id="2.4"></a>

## 2.4 –  Statements

Lua supports an almost conventional set of statements,
similar to those in Pascal or C.
This set includes
assignments, control structures, function calls,
and variable declarations.

<a id="2.4.1"></a>

### 2.4.1 –  Chunks

The unit of execution of Lua is called a *chunk*.
A chunk is simply a sequence of statements,
which are executed sequentially.
Each statement can be optionally followed by a semicolon:



```

	chunk ::= {stat [`;´]}
```




There are no empty statements and thus '`;;`' is not legal.

Lua handles a chunk as the body of an anonymous function
with a variable number of arguments
(see [§2.5.9](005-2-5-9-function-definitions.md#2.5.9)).
As such, chunks can define local variables,
receive arguments, and return values.

A chunk can be stored in a file or in a string inside the host program.
To execute a chunk,
Lua first pre-compiles the chunk into instructions for a virtual machine,
and then it executes the compiled code
with an interpreter for the virtual machine.

Chunks can also be pre-compiled into binary form;
see program `luac` for details.
Programs in source and compiled forms are interchangeable;
Lua automatically detects the file type and acts accordingly.

<a id="2.4.2"></a>

### 2.4.2 –  Blocks

A block is a list of statements;
syntactically, a block is the same as a chunk:



```

	block ::= chunk
```




A block can be explicitly delimited to produce a single statement:



```

	stat ::= do block end
```




Explicit blocks are useful
to control the scope of variable declarations.
Explicit blocks are also sometimes used to
add a **return** or **break** statement in the middle
of another block (see [§2.4.4](003-2-4-4-control-structures.md#2.4.4)).

<a id="2.4.3"></a>

### 2.4.3 –  Assignment

Lua allows multiple assignments.
Therefore, the syntax for assignment
defines a list of variables on the left side
and a list of expressions on the right side.
The elements in both lists are separated by commas:



```

	stat ::= varlist `=´ explist
	varlist ::= var {`,´ var}
	explist ::= exp {`,´ exp}
```




Expressions are discussed in [§2.5](003-2-4-4-control-structures.md#2.5).

Before the assignment,
the list of values is *adjusted* to the length of
the list of variables.
If there are more values than needed,
the excess values are thrown away.
If there are fewer values than needed,
the list is extended with as many **nil**'s as needed.
If the list of expressions ends with a function call,
then all values returned by that call enter the list of values,
before the adjustment
(except when the call is enclosed in parentheses; see [§2.5](003-2-4-4-control-structures.md#2.5)).

The assignment statement first evaluates all its expressions
and only then are the assignments performed.
Thus the code



```

     i = 3
     i, a[i] = i+1, 20
```




sets `a[3]` to 20, without affecting `a[4]`
because the `i` in `a[i]` is evaluated (to 3)
before it is assigned 4.
Similarly, the line



```

     x, y = y, x
```




exchanges the values of `x` and `y`,
and



```

     x, y, z = y, z, x
```




cyclically permutes the values of `x`, `y`, and `z`.

The meaning of assignments to global variables
and table fields can be changed via metatables.
An assignment to an indexed variable `t[i] = val` is equivalent to
`settable_event(t,i,val)`.
(See [§2.8](005-2-5-9-function-definitions.md#2.8) for a complete description of the
`settable_event` function.
This function is not defined or callable in Lua.
We use it here only for explanatory purposes.)

An assignment to a global variable `x = val`
is equivalent to the assignment
`_env.x = val`,
which in turn is equivalent to



```

     settable_event(_env, "x", val)
```




where `_env` is the environment of the running function.
(The `_env` variable is not defined in Lua.
We use it here only for explanatory purposes.)

