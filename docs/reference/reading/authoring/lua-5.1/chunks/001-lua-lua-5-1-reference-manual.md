# Lua Lua 5.1 Reference Manual

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-2-2-1-coercion.md)

# [Lua](https://www.lua.org/home.html) Lua 5.1 Reference Manual

by Roberto Ierusalimschy, Luiz Henrique de Figueiredo, Waldemar Celes

Copyright © 2006–2012 Lua.org, PUC-Rio.
Freely available under the terms of the
[Lua license](https://www.lua.org/license.html).

[contents](https://www.lua.org/manual/5.1/index.html#contents)
·
[index](https://www.lua.org/manual/5.1/index.html#index)
·
[other versions](https://www.lua.org/manual/)
·
[english](https://www.lua.org/manual/5.1/)
·
[português](https://www.lua.org/manual/5.1/pt/)
·
[español](https://www.lua.org/manual/5.1/es/)
·
[deutsch](https://www.lua.org/manual/5.1/de/)



<a id="1"></a>

# 1 –  Introduction

Lua is an extension programming language designed to support
general procedural programming with data description
facilities.
It also offers good support for object-oriented programming,
functional programming, and data-driven programming.
Lua is intended to be used as a powerful, light-weight
scripting language for any program that needs one.
Lua is implemented as a library, written in *clean* C
(that is, in the common subset of ANSI C and C++).

Being an extension language, Lua has no notion of a "main" program:
it only works *embedded* in a host client,
called the *embedding program* or simply the *host*.
This host program can invoke functions to execute a piece of Lua code,
can write and read Lua variables,
and can register C functions to be called by Lua code.
Through the use of C functions, Lua can be augmented to cope with
a wide range of different domains,
thus creating customized programming languages sharing a syntactical framework.
The Lua distribution includes a sample host program called `lua`,
which uses the Lua library to offer a complete, stand-alone Lua interpreter.

Lua is free software,
and is provided as usual with no guarantees,
as stated in its license.
The implementation described in this manual is available
at Lua's official web site, `www.lua.org`.

Like any other reference manual,
this document is dry in places.
For a discussion of the decisions behind the design of Lua,
see the technical papers available at Lua's web site.
For a detailed introduction to programming in Lua,
see Roberto's book, *Programming in Lua (Second Edition)*.

<a id="2"></a>

# 2 –  The Language

This section describes the lexis, the syntax, and the semantics of Lua.
In other words,
this section describes
which tokens are valid,
how they can be combined,
and what their combinations mean.

The language constructs will be explained using the usual extended BNF notation,
in which
{*a*} means 0 or more *a*'s, and
[*a*] means an optional *a*.
Non-terminals are shown like non-terminal,
keywords are shown like **kword**,
and other terminal symbols are shown like `**=**´.
The complete syntax of Lua can be found in [§8](029-7-incompatibilities-with-the-previous-version.md#8)
at the end of this manual.

<a id="2.1"></a>

## 2.1 –  Lexical Conventions

*Names*
(also called *identifiers*)
in Lua can be any string of letters,
digits, and underscores,
not beginning with a digit.
This coincides with the definition of names in most languages.
(The definition of letter depends on the current locale:
any character considered alphabetic by the current locale
can be used in an identifier.)
Identifiers are used to name variables and table fields.

The following *keywords* are reserved
and cannot be used as names:



```

     and       break     do        else      elseif
     end       false     for       function  if
     in        local     nil       not       or
     repeat    return    then      true      until     while
```




Lua is a case-sensitive language:
`and` is a reserved word, but `And` and `AND`
are two different, valid names.
As a convention, names starting with an underscore followed by
uppercase letters (such as [`_VERSION`](021-rawequal-v1-v2.md#pdf-_VERSION))
are reserved for internal global variables used by Lua.

The following strings denote other tokens:



```

     +     -     *     /     %     ^     #
     ==    ~=    <=    >=    <     >     =
     (     )     {     }     [     ]
     ;     :     ,     .     ..    ...
```




*Literal strings*
can be delimited by matching single or double quotes,
and can contain the following C-like escape sequences:
'`\a`' (bell),
'`\b`' (backspace),
'`\f`' (form feed),
'`\n`' (newline),
'`\r`' (carriage return),
'`\t`' (horizontal tab),
'`\v`' (vertical tab),
'`\\`' (backslash),
'`\"`' (quotation mark [double quote]),
and '`\'`' (apostrophe [single quote]).
Moreover, a backslash followed by a real newline
results in a newline in the string.
A character in a string can also be specified by its numerical value
using the escape sequence `\ddd`,
where *ddd* is a sequence of up to three decimal digits.
(Note that if a numerical escape is to be followed by a digit,
it must be expressed using exactly three digits.)
Strings in Lua can contain any 8-bit value, including embedded zeros,
which can be specified as '`\0`'.

Literal strings can also be defined using a long format
enclosed by *long brackets*.
We define an *opening long bracket of level *n** as an opening
square bracket followed by *n* equal signs followed by another
opening square bracket.
So, an opening long bracket of level 0 is written as `[[`,
an opening long bracket of level 1 is written as `[=[`,
and so on.
A *closing long bracket* is defined similarly;
for instance, a closing long bracket of level 4 is written as `]====]`.
A long string starts with an opening long bracket of any level and
ends at the first closing long bracket of the same level.
Literals in this bracketed form can run for several lines,
do not interpret any escape sequences,
and ignore long brackets of any other level.
They can contain anything except a closing bracket of the proper level.

For convenience,
when the opening long bracket is immediately followed by a newline,
the newline is not included in the string.
As an example, in a system using ASCII
(in which '`a`' is coded as 97,
newline is coded as 10, and '`1`' is coded as 49),
the five literal strings below denote the same string:



```

     a = 'alo\n123"'
     a = "alo\n123\""
     a = '\97lo\10\04923"'
     a = [[alo
     123"]]
     a = [==[
     alo
     123"]==]
```




A *numerical constant* can be written with an optional decimal part
and an optional decimal exponent.
Lua also accepts integer hexadecimal constants,
by prefixing them with `0x`.
Examples of valid numerical constants are



```

     3   3.0   3.1416   314.16e-2   0.31416E1   0xff   0x56
```




A *comment* starts with a double hyphen (`--`)
anywhere outside a string.
If the text immediately after `--` is not an opening long bracket,
the comment is a *short comment*,
which runs until the end of the line.
Otherwise, it is a *long comment*,
which runs until the corresponding closing long bracket.
Long comments are frequently used to disable code temporarily.

<a id="2.2"></a>

## 2.2 –  Values and Types

Lua is a *dynamically typed language*.
This means that
variables do not have types; only values do.
There are no type definitions in the language.
All values carry their own type.

All values in Lua are *first-class values*.
This means that all values can be stored in variables,
passed as arguments to other functions, and returned as results.

There are eight basic types in Lua:
*nil*, *boolean*, *number*,
*string*, *function*, *userdata*,
*thread*, and *table*.
*Nil* is the type of the value **nil**,
whose main property is to be different from any other value;
it usually represents the absence of a useful value.
*Boolean* is the type of the values **false** and **true**.
Both **nil** and **false** make a condition false;
any other value makes it true.
*Number* represents real (double-precision floating-point) numbers.
(It is easy to build Lua interpreters that use other
internal representations for numbers,
such as single-precision float or long integers;
see file `luaconf.h`.)
*String* represents arrays of characters.
Lua is 8-bit clean:
strings can contain any 8-bit character,
including embedded zeros ('`\0`') (see [§2.1](001-lua-lua-5-1-reference-manual.md#2.1)).

Lua can call (and manipulate) functions written in Lua and
functions written in C
(see [§2.5.8](004-2-5-1-arithmetic-operators.md#2.5.8)).

The type *userdata* is provided to allow arbitrary C data to
be stored in Lua variables.
This type corresponds to a block of raw memory
and has no pre-defined operations in Lua,
except assignment and identity test.
However, by using *metatables*,
the programmer can define operations for userdata values
(see [§2.8](005-2-5-9-function-definitions.md#2.8)).
Userdata values cannot be created or modified in Lua,
only through the C API.
This guarantees the integrity of data owned by the host program.

The type *thread* represents independent threads of execution
and it is used to implement coroutines (see [§2.11](007-2-10-2-weak-tables.md#2.11)).
Do not confuse Lua threads with operating-system threads.
Lua supports coroutines on all systems,
even those that do not support threads.

The type *table* implements associative arrays,
that is, arrays that can be indexed not only with numbers,
but with any value (except **nil**).
Tables can be *heterogeneous*;
that is, they can contain values of all types (except **nil**).
Tables are the sole data structuring mechanism in Lua;
they can be used to represent ordinary arrays,
symbol tables, sets, records, graphs, trees, etc.
To represent records, Lua uses the field name as an index.
The language supports this representation by
providing `a.name` as syntactic sugar for `a["name"]`.
There are several convenient ways to create tables in Lua
(see [§2.5.7](004-2-5-1-arithmetic-operators.md#2.5.7)).

Like indices,
the value of a table field can be of any type (except **nil**).
In particular,
because functions are first-class values,
table fields can contain functions.
Thus tables can also carry *methods* (see [§2.5.9](005-2-5-9-function-definitions.md#2.5.9)).

Tables, functions, threads, and (full) userdata values are *objects*:
variables do not actually *contain* these values,
only *references* to them.
Assignment, parameter passing, and function returns
always manipulate references to such values;
these operations do not imply any kind of copy.

The library function [`type`](021-rawequal-v1-v2.md#pdf-type) returns a string describing the type
of a given value.

