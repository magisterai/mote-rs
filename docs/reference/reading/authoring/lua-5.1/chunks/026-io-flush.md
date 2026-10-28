# `io.flush ()`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](025-table-insert-table-pos-value.md) · [Next](027-os-date-format-time.md)

<a id="pdf-io.flush"></a>

### `io.flush ()`

Equivalent to `file:flush` over the default output file.

---

<a id="pdf-io.input"></a>

### `io.input ([file])`

When called with a file name, it opens the named file (in text mode),
and sets its handle as the default input file.
When called with a file handle,
it simply sets this file handle as the default input file.
When called without parameters,
it returns the current default input file.

In case of errors this function raises the error,
instead of returning an error code.

---

<a id="pdf-io.lines"></a>

### `io.lines ([filename])`

Opens the given file name in read mode
and returns an iterator function that,
each time it is called,
returns a new line from the file.
Therefore, the construction



```

     for line in io.lines(filename) do body end
```




will iterate over all lines of the file.
When the iterator function detects the end of file,
it returns **nil** (to finish the loop) and automatically closes the file.

The call `io.lines()` (with no file name) is equivalent
to `io.input():lines()`;
that is, it iterates over the lines of the default input file.
In this case it does not close the file when the loop ends.

---

<a id="pdf-io.open"></a>

### `io.open (filename [, mode])`

This function opens a file,
in the mode specified in the string `mode`.
It returns a new file handle,
or, in case of errors, **nil** plus an error message.

The `mode` string can be any of the following:

- **"r":** read mode (the default);
- **"w":** write mode;
- **"a":** append mode;
- **"r+":** update mode, all previous data is preserved;
- **"w+":** update mode, all previous data is erased;
- **"a+":** append update mode, previous data is preserved,
  writing is only allowed at the end of file.

The `mode` string can also have a '`b`' at the end,
which is needed in some systems to open the file in binary mode.
This string is exactly what is used in the
standard C function `fopen`.

---

<a id="pdf-io.output"></a>

### `io.output ([file])`

Similar to [`io.input`](026-io-flush.md#pdf-io.input), but operates over the default output file.

---

<a id="pdf-io.popen"></a>

### `io.popen (prog [, mode])`

Starts program `prog` in a separated process and returns
a file handle that you can use to read data from this program
(if `mode` is `"r"`, the default)
or to write data to this program
(if `mode` is `"w"`).

This function is system dependent and is not available
on all platforms.

---

<a id="pdf-io.read"></a>

### `io.read (···)`

Equivalent to `io.input():read`.

---

<a id="pdf-io.tmpfile"></a>

### `io.tmpfile ()`

Returns a handle for a temporary file.
This file is opened in update mode
and it is automatically removed when the program ends.

---

<a id="pdf-io.type"></a>

### `io.type (obj)`

Checks whether `obj` is a valid file handle.
Returns the string `"file"` if `obj` is an open file handle,
`"closed file"` if `obj` is a closed file handle,
or **nil** if `obj` is not a file handle.

---

<a id="pdf-io.write"></a>

### `io.write (···)`

Equivalent to `io.output():write`.

---

<a id="pdf-file:close"></a>

### `file:close ()`

Closes `file`.
Note that files are automatically closed when
their handles are garbage collected,
but that takes an unpredictable amount of time to happen.

---

<a id="pdf-file:flush"></a>

### `file:flush ()`

Saves any written data to `file`.

---

<a id="pdf-file:lines"></a>

### `file:lines ()`

Returns an iterator function that,
each time it is called,
returns a new line from the file.
Therefore, the construction



```

     for line in file:lines() do body end
```




will iterate over all lines of the file.
(Unlike [`io.lines`](026-io-flush.md#pdf-io.lines), this function does not close the file
when the loop ends.)

---

<a id="pdf-file:read"></a>

### `file:read (···)`

Reads the file `file`,
according to the given formats, which specify what to read.
For each format,
the function returns a string (or a number) with the characters read,
or **nil** if it cannot read data with the specified format.
When called without formats,
it uses a default format that reads the entire next line
(see below).

The available formats are

- **"\*n":**
  reads a number;
  this is the only format that returns a number instead of a string.
- **"\*a":**
  reads the whole file, starting at the current position.
  On end of file, it returns the empty string.
- **"\*l":**
  reads the next line (skipping the end of line),
  returning **nil** on end of file.
  This is the default format.
- ***number*:**
  reads a string with up to this number of characters,
  returning **nil** on end of file.
  If number is zero,
  it reads nothing and returns an empty string,
  or **nil** on end of file.

---

<a id="pdf-file:seek"></a>

### `file:seek ([whence] [, offset])`

Sets and gets the file position,
measured from the beginning of the file,
to the position given by `offset` plus a base
specified by the string `whence`, as follows:

- **"set":** base is position 0 (beginning of the file);
- **"cur":** base is current position;
- **"end":** base is end of file;

In case of success, function `seek` returns the final file position,
measured in bytes from the beginning of the file.
If this function fails, it returns **nil**,
plus a string describing the error.

The default value for `whence` is `"cur"`,
and for `offset` is 0.
Therefore, the call `file:seek()` returns the current
file position, without changing it;
the call `file:seek("set")` sets the position to the
beginning of the file (and returns 0);
and the call `file:seek("end")` sets the position to the
end of the file, and returns its size.

---

<a id="pdf-file:setvbuf"></a>

### `file:setvbuf (mode [, size])`

Sets the buffering mode for an output file.
There are three available modes:

- **"no":**
  no buffering; the result of any output operation appears immediately.
- **"full":**
  full buffering; output operation is performed only
  when the buffer is full (or when you explicitly `flush` the file
  (see [`io.flush`](026-io-flush.md#pdf-io.flush))).
- **"line":**
  line buffering; output is buffered until a newline is output
  or there is any input from some special files
  (such as a terminal device).

For the last two cases, `size`
specifies the size of the buffer, in bytes.
The default is an appropriate size.

---

<a id="pdf-file:write"></a>

### `file:write (···)`

Writes the value of each of its arguments to
the `file`.
The arguments must be strings or numbers.
To write other values,
use [`tostring`](021-rawequal-v1-v2.md#pdf-tostring) or [`string.format`](023-package-path.md#pdf-string.format) before `write`.

<a id="5.8"></a>

## 5.8 –  Operating System Facilities

<a id="pdf-os"></a>

This library is implemented through table `os`.

---

<a id="pdf-os.clock"></a>

### `os.clock ()`

Returns an approximation of the amount in seconds of CPU time
used by the program.

---

