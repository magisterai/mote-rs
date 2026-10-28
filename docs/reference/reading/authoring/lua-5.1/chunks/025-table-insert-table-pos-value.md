# `table.insert (table, \[pos,\] value)`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](024-string-len-s.md) · [Next](026-io-flush.md)

<a id="pdf-table.insert"></a>

### `table.insert (table, [pos,] value)`

Inserts element `value` at position `pos` in `table`,
shifting up other elements to open space, if necessary.
The default value for `pos` is `n+1`,
where `n` is the length of the table (see [§2.5.5](004-2-5-1-arithmetic-operators.md#2.5.5)),
so that a call `table.insert(t,x)` inserts `x` at the end
of table `t`.

---

<a id="pdf-table.maxn"></a>

### `table.maxn (table)`

Returns the largest positive numerical index of the given table,
or zero if the table has no positive numerical indices.
(To do its job this function does a linear traversal of
the whole table.)

---

<a id="pdf-table.remove"></a>

### `table.remove (table [, pos])`

Removes from `table` the element at position `pos`,
shifting down other elements to close the space, if necessary.
Returns the value of the removed element.
The default value for `pos` is `n`,
where `n` is the length of the table,
so that a call `table.remove(t)` removes the last element
of table `t`.

---

<a id="pdf-table.sort"></a>

### `table.sort (table [, comp])`

Sorts table elements in a given order, *in-place*,
from `table[1]` to `table[n]`,
where `n` is the length of the table.
If `comp` is given,
then it must be a function that receives two table elements,
and returns true
when the first is less than the second
(so that `not comp(a[i+1],a[i])` will be true after the sort).
If `comp` is not given,
then the standard Lua operator `<` is used instead.

The sort algorithm is not stable;
that is, elements considered equal by the given order
may have their relative positions changed by the sort.

<a id="5.6"></a>

## 5.6 –  Mathematical Functions

<a id="pdf-math"></a>

This library is an interface to the standard C math library.
It provides all its functions inside the table `math`.

---

<a id="pdf-math.abs"></a>

### `math.abs (x)`

Returns the absolute value of `x`.

---

<a id="pdf-math.acos"></a>

### `math.acos (x)`

Returns the arc cosine of `x` (in radians).

---

<a id="pdf-math.asin"></a>

### `math.asin (x)`

Returns the arc sine of `x` (in radians).

---

<a id="pdf-math.atan"></a>

### `math.atan (x)`

Returns the arc tangent of `x` (in radians).

---

<a id="pdf-math.atan2"></a>

### `math.atan2 (y, x)`

Returns the arc tangent of `y/x` (in radians),
but uses the signs of both parameters to find the
quadrant of the result.
(It also handles correctly the case of `x` being zero.)

---

<a id="pdf-math.ceil"></a>

### `math.ceil (x)`

Returns the smallest integer larger than or equal to `x`.

---

<a id="pdf-math.cos"></a>

### `math.cos (x)`

Returns the cosine of `x` (assumed to be in radians).

---

<a id="pdf-math.cosh"></a>

### `math.cosh (x)`

Returns the hyperbolic cosine of `x`.

---

<a id="pdf-math.deg"></a>

### `math.deg (x)`

Returns the angle `x` (given in radians) in degrees.

---

<a id="pdf-math.exp"></a>

### `math.exp (x)`

Returns the value *e<sup>x</sup>*.

---

<a id="pdf-math.floor"></a>

### `math.floor (x)`

Returns the largest integer smaller than or equal to `x`.

---

<a id="pdf-math.fmod"></a>

### `math.fmod (x, y)`

Returns the remainder of the division of `x` by `y`
that rounds the quotient towards zero.

---

<a id="pdf-math.frexp"></a>

### `math.frexp (x)`

Returns `m` and `e` such that *x = m2<sup>e</sup>*,
`e` is an integer and the absolute value of `m` is
in the range *[0.5, 1)*
(or zero when `x` is zero).

---

<a id="pdf-math.huge"></a>

### `math.huge`

The value `HUGE_VAL`,
a value larger than or equal to any other numerical value.

---

<a id="pdf-math.ldexp"></a>

### `math.ldexp (m, e)`

Returns *m2<sup>e</sup>* (`e` should be an integer).

---

<a id="pdf-math.log"></a>

### `math.log (x)`

Returns the natural logarithm of `x`.

---

<a id="pdf-math.log10"></a>

### `math.log10 (x)`

Returns the base-10 logarithm of `x`.

---

<a id="pdf-math.max"></a>

### `math.max (x, ···)`

Returns the maximum value among its arguments.

---

<a id="pdf-math.min"></a>

### `math.min (x, ···)`

Returns the minimum value among its arguments.

---

<a id="pdf-math.modf"></a>

### `math.modf (x)`

Returns two numbers,
the integral part of `x` and the fractional part of `x`.

---

<a id="pdf-math.pi"></a>

### `math.pi`

The value of *pi*.

---

<a id="pdf-math.pow"></a>

### `math.pow (x, y)`

Returns *x<sup>y</sup>*.
(You can also use the expression `x^y` to compute this value.)

---

<a id="pdf-math.rad"></a>

### `math.rad (x)`

Returns the angle `x` (given in degrees) in radians.

---

<a id="pdf-math.random"></a>

### `math.random ([m [, n]])`

This function is an interface to the simple
pseudo-random generator function `rand` provided by ANSI C.
(No guarantees can be given for its statistical properties.)

When called without arguments,
returns a uniform pseudo-random real number
in the range *[0,1)*.
When called with an integer number `m`,
`math.random` returns
a uniform pseudo-random integer in the range *[1, m]*.
When called with two integer numbers `m` and `n`,
`math.random` returns a uniform pseudo-random
integer in the range *[m, n]*.

---

<a id="pdf-math.randomseed"></a>

### `math.randomseed (x)`

Sets `x` as the "seed"
for the pseudo-random generator:
equal seeds produce equal sequences of numbers.

---

<a id="pdf-math.sin"></a>

### `math.sin (x)`

Returns the sine of `x` (assumed to be in radians).

---

<a id="pdf-math.sinh"></a>

### `math.sinh (x)`

Returns the hyperbolic sine of `x`.

---

<a id="pdf-math.sqrt"></a>

### `math.sqrt (x)`

Returns the square root of `x`.
(You can also use the expression `x^0.5` to compute this value.)

---

<a id="pdf-math.tan"></a>

### `math.tan (x)`

Returns the tangent of `x` (assumed to be in radians).

---

<a id="pdf-math.tanh"></a>

### `math.tanh (x)`

Returns the hyperbolic tangent of `x`.

<a id="5.7"></a>

## 5.7 –  Input and Output Facilities

The I/O library provides two different styles for file manipulation.
The first one uses implicit file descriptors;
that is, there are operations to set a default input file and a
default output file,
and all input/output operations are over these default files.
The second style uses explicit file descriptors.

<a id="pdf-io"></a>

When using implicit file descriptors,
all operations are supplied by table `io`.
When using explicit file descriptors,
the operation [`io.open`](026-io-flush.md#pdf-io.open) returns a file descriptor
and then all operations are supplied as methods of the file descriptor.

<a id="pdf-io.stdin"></a>

<a id="pdf-io.stdout"></a>

<a id="pdf-io.stderr"></a>

The table `io` also provides
three predefined file descriptors with their usual meanings from C:
`io.stdin`, `io.stdout`, and `io.stderr`.
The I/O library never closes these files.

Unless otherwise stated,
all I/O functions return **nil** on failure
(plus an error message as a second result and
a system-dependent error code as a third result)
and some value different from **nil** on success.

---

<a id="pdf-io.close"></a>

### `io.close ([file])`

Equivalent to `file:close()`.
Without a `file`, closes the default output file.

---

