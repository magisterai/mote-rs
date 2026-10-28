# `debug.getlocal (\[thread,\] level, local)`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](027-os-date-format-time.md) · [Next](029-7-incompatibilities-with-the-previous-version.md)

<a id="pdf-debug.getlocal"></a>

### `debug.getlocal ([thread,] level, local)`

This function returns the name and the value of the local variable
with index `local` of the function at level `level` of the stack.
(The first parameter or local variable has index 1, and so on,
until the last active local variable.)
The function returns **nil** if there is no local
variable with the given index,
and raises an error when called with a `level` out of range.
(You can call [`debug.getinfo`](027-os-date-format-time.md#pdf-debug.getinfo) to check whether the level is valid.)

Variable names starting with '`(`' (open parentheses)
represent internal variables
(loop control variables, temporaries, and C function locals).

---

<a id="pdf-debug.getmetatable"></a>

### `debug.getmetatable (object)`

Returns the metatable of the given `object`
or **nil** if it does not have a metatable.

---

<a id="pdf-debug.getregistry"></a>

### `debug.getregistry ()`

Returns the registry table (see [§3.5](008-3-2-stack-size.md#3.5)).

---

<a id="pdf-debug.getupvalue"></a>

### `debug.getupvalue (func, up)`

This function returns the name and the value of the upvalue
with index `up` of the function `func`.
The function returns **nil** if there is no upvalue with the given index.

---

<a id="pdf-debug.setfenv"></a>

### `debug.setfenv (object, table)`

Sets the environment of the given `object` to the given `table`.
Returns `object`.

---

<a id="pdf-debug.sethook"></a>

### `debug.sethook ([thread,] hook, mask [, count])`

Sets the given function as a hook.
The string `mask` and the number `count` describe
when the hook will be called.
The string mask may have the following characters,
with the given meaning:

- **`"c"`:** the hook is called every time Lua calls a function;
- **`"r"`:** the hook is called every time Lua returns from a function;
- **`"l"`:** the hook is called every time Lua enters a new line of code.

With a `count` different from zero,
the hook is called after every `count` instructions.

When called without arguments,
[`debug.sethook`](028-debug-getlocal-thread-level-local.md#pdf-debug.sethook) turns off the hook.

When the hook is called, its first parameter is a string
describing the event that has triggered its call:
`"call"`, `"return"` (or `"tail return"`,
when simulating a return from a tail call),
`"line"`, and `"count"`.
For line events,
the hook also gets the new line number as its second parameter.
Inside a hook,
you can call `getinfo` with level 2 to get more information about
the running function
(level 0 is the `getinfo` function,
and level 1 is the hook function),
unless the event is `"tail return"`.
In this case, Lua is only simulating the return,
and a call to `getinfo` will return invalid data.

---

<a id="pdf-debug.setlocal"></a>

### `debug.setlocal ([thread,] level, local, value)`

This function assigns the value `value` to the local variable
with index `local` of the function at level `level` of the stack.
The function returns **nil** if there is no local
variable with the given index,
and raises an error when called with a `level` out of range.
(You can call `getinfo` to check whether the level is valid.)
Otherwise, it returns the name of the local variable.

---

<a id="pdf-debug.setmetatable"></a>

### `debug.setmetatable (object, table)`

Sets the metatable for the given `object` to the given `table`
(which can be **nil**).

---

<a id="pdf-debug.setupvalue"></a>

### `debug.setupvalue (func, up, value)`

This function assigns the value `value` to the upvalue
with index `up` of the function `func`.
The function returns **nil** if there is no upvalue
with the given index.
Otherwise, it returns the name of the upvalue.

---

<a id="pdf-debug.traceback"></a>

### `debug.traceback ([thread,] [message [, level]])`

Returns a string with a traceback of the call stack.
An optional `message` string is appended
at the beginning of the traceback.
An optional `level` number tells at which level
to start the traceback
(default is 1, the function calling `traceback`).

<a id="6"></a>

# 6 –  Lua Stand-alone

Although Lua has been designed as an extension language,
to be embedded in a host C program,
it is also frequently used as a stand-alone language.
An interpreter for Lua as a stand-alone language,
called simply `lua`,
is provided with the standard distribution.
The stand-alone interpreter includes
all standard libraries, including the debug library.
Its usage is:



```

     lua [options] [script [args]]
```




The options are:

- **`-e stat`:** executes string *stat*;
- **`-l mod`:** "requires" *mod*;
- **`-i`:** enters interactive mode after running *script*;
- **`-v`:** prints version information;
- **`--`:** stops handling options;
- **`-`:** executes `stdin` as a file and stops handling options.

After handling its options, `lua` runs the given *script*,
passing to it the given *args* as string arguments.
When called without arguments,
`lua` behaves as `lua -v -i`
when the standard input (`stdin`) is a terminal,
and as `lua -` otherwise.

<a id="pdf-LUA_INIT"></a>

Before running any argument,
the interpreter checks for an environment variable `LUA_INIT`.
If its format is `@filename`,
then `lua` executes the file.
Otherwise, `lua` executes the string itself.

All options are handled in order, except `-i`.
For instance, an invocation like



```

     $ lua -e'a=1' -e 'print(a)' script.lua
```




will first set `a` to 1, then print the value of `a` (which is '`1`'),
and finally run the file `script.lua` with no arguments.
(Here `$` is the shell prompt. Your prompt may be different.)

Before starting to run the script,
`lua` collects all arguments in the command line
in a global table called `arg`.
The script name is stored at index 0,
the first argument after the script name goes to index 1,
and so on.
Any arguments before the script name
(that is, the interpreter name plus the options)
go to negative indices.
For instance, in the call



```

     $ lua -la b.lua t1 t2
```




the interpreter first runs the file `a.lua`,
then creates a table



```

     arg = { [-2] = "lua", [-1] = "-la",
             [0] = "b.lua",
             [1] = "t1", [2] = "t2" }
```




and finally runs the file `b.lua`.
The script is called with `arg[1]`, `arg[2]`, ···
as arguments;
it can also access these arguments with the vararg expression '`...`'.

In interactive mode,
if you write an incomplete statement,
the interpreter waits for its completion
by issuing a different prompt.

<a id="pdf-_PROMPT"></a>

<a id="pdf-_PROMPT2"></a>

If the global variable `_PROMPT` contains a string,
then its value is used as the prompt.
Similarly, if the global variable `_PROMPT2` contains a string,
its value is used as the secondary prompt
(issued during incomplete statements).
Therefore, both prompts can be changed directly on the command line
or in any Lua programs by assigning to `_PROMPT`.
See the next example:



```

     $ lua -e"_PROMPT='myprompt> '" -i
```




(The outer pair of quotes is for the shell,
the inner pair is for Lua.)
Note the use of `-i` to enter interactive mode;
otherwise,
the program would just end silently
right after the assignment to `_PROMPT`.

To allow the use of Lua as a
script interpreter in Unix systems,
the stand-alone interpreter skips
the first line of a chunk if it starts with `#`.
Therefore, Lua scripts can be made into executable programs
by using `chmod +x` and the `#!` form,
as in



```

     #!/usr/local/bin/lua
```




(Of course,
the location of the Lua interpreter may be different in your machine.
If `lua` is in your `PATH`,
then



```

     #!/usr/bin/env lua
```




is a more portable solution.)

