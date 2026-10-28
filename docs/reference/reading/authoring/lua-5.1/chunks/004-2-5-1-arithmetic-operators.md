# 2.5.1 – Arithmetic Operators

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-2-4-4-control-structures.md) · [Next](005-2-5-9-function-definitions.md)

<a id="2.5.1"></a>

### 2.5.1 –  Arithmetic Operators

Lua supports the usual arithmetic operators:
the binary `+` (addition),
`-` (subtraction), `*` (multiplication),
`/` (division), `%` (modulo), and `^` (exponentiation);
and unary `-` (negation).
If the operands are numbers, or strings that can be converted to
numbers (see [§2.2.1](002-2-2-1-coercion.md#2.2.1)),
then all operations have the usual meaning.
Exponentiation works for any exponent.
For instance, `x^(-0.5)` computes the inverse of the square root of `x`.
Modulo is defined as



```

     a % b == a - math.floor(a/b)*b
```




That is, it is the remainder of a division that rounds
the quotient towards minus infinity.

<a id="2.5.2"></a>

### 2.5.2 –  Relational Operators

The relational operators in Lua are



```

     ==    ~=    <     >     <=    >=
```




These operators always result in **false** or **true**.

Equality (`==`) first compares the type of its operands.
If the types are different, then the result is **false**.
Otherwise, the values of the operands are compared.
Numbers and strings are compared in the usual way.
Objects (tables, userdata, threads, and functions)
are compared by *reference*:
two objects are considered equal only if they are the *same* object.
Every time you create a new object
(a table, userdata, thread, or function),
this new object is different from any previously existing object.

You can change the way that Lua compares tables and userdata
by using the "eq" metamethod (see [§2.8](005-2-5-9-function-definitions.md#2.8)).

The conversion rules of [§2.2.1](002-2-2-1-coercion.md#2.2.1)
*do not* apply to equality comparisons.
Thus, `"0"==0` evaluates to **false**,
and `t[0]` and `t["0"]` denote different
entries in a table.

The operator `~=` is exactly the negation of equality (`==`).

The order operators work as follows.
If both arguments are numbers, then they are compared as such.
Otherwise, if both arguments are strings,
then their values are compared according to the current locale.
Otherwise, Lua tries to call the "lt" or the "le"
metamethod (see [§2.8](005-2-5-9-function-definitions.md#2.8)).
A comparison `a > b` is translated to `b < a`
and `a >= b` is translated to `b <= a`.

<a id="2.5.3"></a>

### 2.5.3 –  Logical Operators

The logical operators in Lua are
**and**, **or**, and **not**.
Like the control structures (see [§2.4.4](003-2-4-4-control-structures.md#2.4.4)),
all logical operators consider both **false** and **nil** as false
and anything else as true.

The negation operator **not** always returns **false** or **true**.
The conjunction operator **and** returns its first argument
if this value is **false** or **nil**;
otherwise, **and** returns its second argument.
The disjunction operator **or** returns its first argument
if this value is different from **nil** and **false**;
otherwise, **or** returns its second argument.
Both **and** and **or** use short-cut evaluation;
that is,
the second operand is evaluated only if necessary.
Here are some examples:



```

     10 or 20            --> 10
     10 or error()       --> 10
     nil or "a"          --> "a"
     nil and 10          --> nil
     false and error()   --> false
     false and nil       --> false
     false or nil        --> nil
     10 and 20           --> 20
```




(In this manual,
`-->` indicates the result of the preceding expression.)

<a id="2.5.4"></a>

### 2.5.4 –  Concatenation

The string concatenation operator in Lua is
denoted by two dots ('`..`').
If both operands are strings or numbers, then they are converted to
strings according to the rules mentioned in [§2.2.1](002-2-2-1-coercion.md#2.2.1).
Otherwise, the "concat" metamethod is called (see [§2.8](005-2-5-9-function-definitions.md#2.8)).

<a id="2.5.5"></a>

### 2.5.5 –  The Length Operator

The length operator is denoted by the unary operator `#`.
The length of a string is its number of bytes
(that is, the usual meaning of string length when each
character is one byte).

The length of a table `t` is defined to be any
integer index `n`
such that `t[n]` is not **nil** and `t[n+1]` is **nil**;
moreover, if `t[1]` is **nil**, `n` can be zero.
For a regular array, with non-nil values from 1 to a given `n`,
its length is exactly that `n`,
the index of its last value.
If the array has "holes"
(that is, **nil** values between other non-nil values),
then `#t` can be any of the indices that
directly precedes a **nil** value
(that is, it may consider any such **nil** value as the end of
the array).

<a id="2.5.6"></a>

### 2.5.6 –  Precedence

Operator precedence in Lua follows the table below,
from lower to higher priority:



```

     or
     and
     <     >     <=    >=    ~=    ==
     ..
     +     -
     *     /     %
     not   #     - (unary)
     ^
```




As usual,
you can use parentheses to change the precedences of an expression.
The concatenation ('`..`') and exponentiation ('`^`')
operators are right associative.
All other binary operators are left associative.

<a id="2.5.7"></a>

### 2.5.7 –  Table Constructors

Table constructors are expressions that create tables.
Every time a constructor is evaluated, a new table is created.
A constructor can be used to create an empty table
or to create a table and initialize some of its fields.
The general syntax for constructors is



```

	tableconstructor ::= `{´ [fieldlist] `}´
	fieldlist ::= field {fieldsep field} [fieldsep]
	field ::= `[´ exp `]´ `=´ exp | Name `=´ exp | exp
	fieldsep ::= `,´ | `;´
```




Each field of the form `[exp1] = exp2` adds to the new table an entry
with key `exp1` and value `exp2`.
A field of the form `name = exp` is equivalent to
`["name"] = exp`.
Finally, fields of the form `exp` are equivalent to
`[i] = exp`, where `i` are consecutive numerical integers,
starting with 1.
Fields in the other formats do not affect this counting.
For example,



```

     a = { [f(1)] = g; "x", "y"; x = 1, f(x), [30] = 23; 45 }
```




is equivalent to



```

     do
       local t = {}
       t[f(1)] = g
       t[1] = "x"         -- 1st exp
       t[2] = "y"         -- 2nd exp
       t.x = 1            -- t["x"] = 1
       t[3] = f(x)        -- 3rd exp
       t[30] = 23
       t[4] = 45          -- 4th exp
       a = t
     end
```




If the last field in the list has the form `exp`
and the expression is a function call or a vararg expression,
then all values returned by this expression enter the list consecutively
(see [§2.5.8](004-2-5-1-arithmetic-operators.md#2.5.8)).
To avoid this,
enclose the function call or the vararg expression
in parentheses (see [§2.5](003-2-4-4-control-structures.md#2.5)).

The field list can have an optional trailing separator,
as a convenience for machine-generated code.

<a id="2.5.8"></a>

### 2.5.8 –  Function Calls

A function call in Lua has the following syntax:



```

	functioncall ::= prefixexp args
```




In a function call,
first prefixexp and args are evaluated.
If the value of prefixexp has type *function*,
then this function is called
with the given arguments.
Otherwise, the prefixexp "call" metamethod is called,
having as first parameter the value of prefixexp,
followed by the original call arguments
(see [§2.8](005-2-5-9-function-definitions.md#2.8)).

The form



```

	functioncall ::= prefixexp `:´ Name args
```




can be used to call "methods".
A call `v:name(args)`
is syntactic sugar for `v.name(v,args)`,
except that `v` is evaluated only once.

Arguments have the following syntax:



```

	args ::= `(´ [explist] `)´
	args ::= tableconstructor
	args ::= String
```




All argument expressions are evaluated before the call.
A call of the form `f{fields}` is
syntactic sugar for `f({fields})`;
that is, the argument list is a single new table.
A call of the form `f'string'`
(or `f"string"` or `f[[string]]`)
is syntactic sugar for `f('string')`;
that is, the argument list is a single literal string.

As an exception to the free-format syntax of Lua,
you cannot put a line break before the '`(`' in a function call.
This restriction avoids some ambiguities in the language.
If you write



```

     a = f
     (g).x(a)
```




Lua would see that as a single statement, `a = f(g).x(a)`.
So, if you want two statements, you must add a semi-colon between them.
If you actually want to call `f`,
you must remove the line break before `(g)`.

A call of the form `return` *functioncall* is called
a *tail call*.
Lua implements *proper tail calls*
(or *proper tail recursion*):
in a tail call,
the called function reuses the stack entry of the calling function.
Therefore, there is no limit on the number of nested tail calls that
a program can execute.
However, a tail call erases any debug information about the
calling function.
Note that a tail call only happens with a particular syntax,
where the **return** has one single function call as argument;
this syntax makes the calling function return exactly
the returns of the called function.
So, none of the following examples are tail calls:



```

     return (f(x))        -- results adjusted to 1
     return 2 * f(x)
     return x, f(x)       -- additional results
     f(x); return         -- results discarded
     return x or f(x)     -- results adjusted to 1
```




