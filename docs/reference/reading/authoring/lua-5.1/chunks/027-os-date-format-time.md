# `os.date (\[format \[, time\]\])`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](026-io-flush.md) · [Next](028-debug-getlocal-thread-level-local.md)

<a id="pdf-os.date"></a>

### `os.date ([format [, time]])`

Returns a string or a table containing date and time,
formatted according to the given string `format`.

If the `time` argument is present,
this is the time to be formatted
(see the [`os.time`](027-os-date-format-time.md#pdf-os.time) function for a description of this value).
Otherwise, `date` formats the current time.

If `format` starts with '`!`',
then the date is formatted in Coordinated Universal Time.
After this optional character,
if `format` is the string "`*t`",
then `date` returns a table with the following fields:
`year` (four digits), `month` (1--12), `day` (1--31),
`hour` (0--23), `min` (0--59), `sec` (0--61),
`wday` (weekday, Sunday is 1),
`yday` (day of the year),
and `isdst` (daylight saving flag, a boolean).

If `format` is not "`*t`",
then `date` returns the date as a string,
formatted according to the same rules as the C function `strftime`.

When called without arguments,
`date` returns a reasonable date and time representation that depends on
the host system and on the current locale
(that is, `os.date()` is equivalent to `os.date("%c")`).

---

<a id="pdf-os.difftime"></a>

### `os.difftime (t2, t1)`

Returns the number of seconds from time `t1` to time `t2`.
In POSIX, Windows, and some other systems,
this value is exactly `t2`*-*`t1`.

---

<a id="pdf-os.execute"></a>

### `os.execute ([command])`

This function is equivalent to the C function `system`.
It passes `command` to be executed by an operating system shell.
It returns a status code, which is system-dependent.
If `command` is absent, then it returns nonzero if a shell is available
and zero otherwise.

---

<a id="pdf-os.exit"></a>

### `os.exit ([code])`

Calls the C function `exit`,
with an optional `code`,
to terminate the host program.
The default value for `code` is the success code.

---

<a id="pdf-os.getenv"></a>

### `os.getenv (varname)`

Returns the value of the process environment variable `varname`,
or **nil** if the variable is not defined.

---

<a id="pdf-os.remove"></a>

### `os.remove (filename)`

Deletes the file or directory with the given name.
Directories must be empty to be removed.
If this function fails, it returns **nil**,
plus a string describing the error.

---

<a id="pdf-os.rename"></a>

### `os.rename (oldname, newname)`

Renames file or directory named `oldname` to `newname`.
If this function fails, it returns **nil**,
plus a string describing the error.

---

<a id="pdf-os.setlocale"></a>

### `os.setlocale (locale [, category])`

Sets the current locale of the program.
`locale` is a string specifying a locale;
`category` is an optional string describing which category to change:
`"all"`, `"collate"`, `"ctype"`,
`"monetary"`, `"numeric"`, or `"time"`;
the default category is `"all"`.
The function returns the name of the new locale,
or **nil** if the request cannot be honored.

If `locale` is the empty string,
the current locale is set to an implementation-defined native locale.
If `locale` is the string "`C`",
the current locale is set to the standard C locale.

When called with **nil** as the first argument,
this function only returns the name of the current locale
for the given category.

---

<a id="pdf-os.time"></a>

### `os.time ([table])`

Returns the current time when called without arguments,
or a time representing the date and time specified by the given table.
This table must have fields `year`, `month`, and `day`,
and may have fields `hour`, `min`, `sec`, and `isdst`
(for a description of these fields, see the [`os.date`](027-os-date-format-time.md#pdf-os.date) function).

The returned value is a number, whose meaning depends on your system.
In POSIX, Windows, and some other systems, this number counts the number
of seconds since some given start time (the "epoch").
In other systems, the meaning is not specified,
and the number returned by `time` can be used only as an argument to
`date` and `difftime`.

---

<a id="pdf-os.tmpname"></a>

### `os.tmpname ()`

Returns a string with a file name that can
be used for a temporary file.
The file must be explicitly opened before its use
and explicitly removed when no longer needed.

On some systems (POSIX),
this function also creates a file with that name,
to avoid security risks.
(Someone else might create the file with wrong permissions
in the time between getting the name and creating the file.)
You still have to open the file to use it
and to remove it (even if you do not use it).

When possible,
you may prefer to use [`io.tmpfile`](026-io-flush.md#pdf-io.tmpfile),
which automatically removes the file when the program ends.

<a id="5.9"></a>

## 5.9 –  The Debug Library

This library provides
the functionality of the debug interface to Lua programs.
You should exert care when using this library.
The functions provided here should be used exclusively for debugging
and similar tasks, such as profiling.
Please resist the temptation to use them as a
usual programming tool:
they can be very slow.
Moreover, several of these functions
violate some assumptions about Lua code
(e.g., that variables local to a function
cannot be accessed from outside or
that userdata metatables cannot be changed by Lua code)
and therefore can compromise otherwise secure code.

<a id="pdf-debug"></a>

All functions in this library are provided
inside the `debug` table.
All functions that operate over a thread
have an optional first argument which is the
thread to operate over.
The default is always the current thread.

---

<a id="pdf-debug.debug"></a>

### `debug.debug ()`

Enters an interactive mode with the user,
running each string that the user enters.
Using simple commands and other debug facilities,
the user can inspect global and local variables,
change their values, evaluate expressions, and so on.
A line containing only the word `cont` finishes this function,
so that the caller continues its execution.

Note that commands for `debug.debug` are not lexically nested
within any function, and so have no direct access to local variables.

---

<a id="pdf-debug.getfenv"></a>

### `debug.getfenv (o)`

Returns the environment of object `o`.

---

<a id="pdf-debug.gethook"></a>

### `debug.gethook ([thread])`

Returns the current hook settings of the thread, as three values:
the current hook function, the current hook mask,
and the current hook count
(as set by the [`debug.sethook`](028-debug-getlocal-thread-level-local.md#pdf-debug.sethook) function).

---

<a id="pdf-debug.getinfo"></a>

### `debug.getinfo ([thread,] function [, what])`

Returns a table with information about a function.
You can give the function directly,
or you can give a number as the value of `function`,
which means the function running at level `function` of the call stack
of the given thread:
level 0 is the current function (`getinfo` itself);
level 1 is the function that called `getinfo`;
and so on.
If `function` is a number larger than the number of active functions,
then `getinfo` returns **nil**.

The returned table can contain all the fields returned by [`lua_getinfo`](015-lua-debug.md#lua_getinfo),
with the string `what` describing which fields to fill in.
The default for `what` is to get all information available,
except the table of valid lines.
If present,
the option '`f`'
adds a field named `func` with the function itself.
If present,
the option '`L`'
adds a field named `activelines` with the table of
valid lines.

For instance, the expression `debug.getinfo(1,"n").name` returns
a table with a name for the current function,
if a reasonable name can be found,
and the expression `debug.getinfo(print)`
returns a table with all available information
about the [`print`](020-collectgarbage-opt-arg.md#pdf-print) function.

---

