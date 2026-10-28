# `string.len (s)`

Source: [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/manual.html) · 5.1.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.lua.org/manual/5.1/manual.html).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](023-package-path.md) · [Next](025-table-insert-table-pos-value.md)

<a id="pdf-string.len"></a>

### `string.len (s)`

Receives a string and returns its length.
The empty string `""` has length 0.
Embedded zeros are counted,
so `"a\000bc\000"` has length 5.

---

<a id="pdf-string.lower"></a>

### `string.lower (s)`

Receives a string and returns a copy of this string with all
uppercase letters changed to lowercase.
All other characters are left unchanged.
The definition of what an uppercase letter is depends on the current locale.

---

<a id="pdf-string.match"></a>

### `string.match (s, pattern [, init])`

Looks for the first *match* of
`pattern` in the string `s`.
If it finds one, then `match` returns
the captures from the pattern;
otherwise it returns **nil**.
If `pattern` specifies no captures,
then the whole match is returned.
A third, optional numerical argument `init` specifies
where to start the search;
its default value is 1 and can be negative.

---

<a id="pdf-string.rep"></a>

### `string.rep (s, n)`

Returns a string that is the concatenation of `n` copies of
the string `s`.

---

<a id="pdf-string.reverse"></a>

### `string.reverse (s)`

Returns a string that is the string `s` reversed.

---

<a id="pdf-string.sub"></a>

### `string.sub (s, i [, j])`

Returns the substring of `s` that
starts at `i` and continues until `j`;
`i` and `j` can be negative.
If `j` is absent, then it is assumed to be equal to -1
(which is the same as the string length).
In particular,
the call `string.sub(s,1,j)` returns a prefix of `s`
with length `j`,
and `string.sub(s, -i)` returns a suffix of `s`
with length `i`.

---

<a id="pdf-string.upper"></a>

### `string.upper (s)`

Receives a string and returns a copy of this string with all
lowercase letters changed to uppercase.
All other characters are left unchanged.
The definition of what a lowercase letter is depends on the current locale.

<a id="5.4.1"></a>

### 5.4.1 –  Patterns

#### Character Class:

A *character class* is used to represent a set of characters.
The following combinations are allowed in describing a character class:

- ***x*:**
  (where *x* is not one of the *magic characters*
  `^$()%.[]*+-?`)
  represents the character *x* itself.
- **`.`:** (a dot) represents all characters.
- **`%a`:** represents all letters.
- **`%c`:** represents all control characters.
- **`%d`:** represents all digits.
- **`%l`:** represents all lowercase letters.
- **`%p`:** represents all punctuation characters.
- **`%s`:** represents all space characters.
- **`%u`:** represents all uppercase letters.
- **`%w`:** represents all alphanumeric characters.
- **`%x`:** represents all hexadecimal digits.
- **`%z`:** represents the character with representation 0.
- **`%x`:** (where *x* is any non-alphanumeric character)
  represents the character *x*.
  This is the standard way to escape the magic characters.
  Any punctuation character (even the non magic)
  can be preceded by a '`%`'
  when used to represent itself in a pattern.
- **`[set]`:**
  represents the class which is the union of all
  characters in *set*.
  A range of characters can be specified by
  separating the end characters of the range with a '`-`'.
  All classes `%`*x* described above can also be used as
  components in *set*.
  All other characters in *set* represent themselves.
  For example, `[%w_]` (or `[_%w]`)
  represents all alphanumeric characters plus the underscore,
  `[0-7]` represents the octal digits,
  and `[0-7%l%-]` represents the octal digits plus
  the lowercase letters plus the '`-`' character.

  The interaction between ranges and classes is not defined.
  Therefore, patterns like `[%a-z]` or `[a-%%]`
  have no meaning.
- **`[^set]`:**
  represents the complement of *set*,
  where *set* is interpreted as above.

For all classes represented by single letters (`%a`, `%c`, etc.),
the corresponding uppercase letter represents the complement of the class.
For instance, `%S` represents all non-space characters.

The definitions of letter, space, and other character groups
depend on the current locale.
In particular, the class `[a-z]` may not be equivalent to `%l`.

#### Pattern Item:

A *pattern item* can be

- a single character class,
  which matches any single character in the class;
- a single character class followed by '`*`',
  which matches 0 or more repetitions of characters in the class.
  These repetition items will always match the longest possible sequence;
- a single character class followed by '`+`',
  which matches 1 or more repetitions of characters in the class.
  These repetition items will always match the longest possible sequence;
- a single character class followed by '`-`',
  which also matches 0 or more repetitions of characters in the class.
  Unlike '`*`',
  these repetition items will always match the *shortest* possible sequence;
- a single character class followed by '`?`',
  which matches 0 or 1 occurrence of a character in the class;
- `%n`, for *n* between 1 and 9;
  such item matches a substring equal to the *n*-th captured string
  (see below);
- `%bxy`, where *x* and *y* are two distinct characters;
  such item matches strings that start with *x*, end with *y*,
  and where the *x* and *y* are *balanced*.
  This means that, if one reads the string from left to right,
  counting *+1* for an *x* and *-1* for a *y*,
  the ending *y* is the first *y* where the count reaches 0.
  For instance, the item `%b()` matches expressions with
  balanced parentheses.

#### Pattern:

A *pattern* is a sequence of pattern items.
A '`^`' at the beginning of a pattern anchors the match at the
beginning of the subject string.
A '`$`' at the end of a pattern anchors the match at the
end of the subject string.
At other positions,
'`^`' and '`$`' have no special meaning and represent themselves.

#### Captures:

A pattern can contain sub-patterns enclosed in parentheses;
they describe *captures*.
When a match succeeds, the substrings of the subject string
that match captures are stored (*captured*) for future use.
Captures are numbered according to their left parentheses.
For instance, in the pattern `"(a*(.)%w(%s*))"`,
the part of the string matching `"a*(.)%w(%s*)"` is
stored as the first capture (and therefore has number 1);
the character matching "`.`" is captured with number 2,
and the part matching "`%s*`" has number 3.

As a special case, the empty capture `()` captures
the current string position (a number).
For instance, if we apply the pattern `"()aa()"` on the
string `"flaaap"`, there will be two captures: 3 and 5.

A pattern cannot contain embedded zeros. Use `%z` instead.

<a id="5.5"></a>

## 5.5 –  Table Manipulation

<a id="pdf-table"></a>

This library provides generic functions for table manipulation.
It provides all its functions inside the table `table`.

Most functions in the table library assume that the table
represents an array or a list.
For these functions, when we talk about the "length" of a table
we mean the result of the length operator.

---

<a id="pdf-table.concat"></a>

### `table.concat (table [, sep [, i [, j]]])`

Given an array where all elements are strings or numbers,
returns `table[i]..sep..table[i+1] ··· sep..table[j]`.
The default value for `sep` is the empty string,
the default for `i` is 1,
and the default for `j` is the length of the table.
If `i` is greater than `j`, returns the empty string.

---

