# Syntax and values

[Specification index](README.md) · [Fundamentals](fundamentals.md)

Status: initial draft. Read [Fundamentals](fundamentals.md) first. This chapter
specifies the spelling of the core forms introduced there. It is not a complete
Markdown or expression grammar. Every example uses those defined forms.

## 1. Source and names

Source is UTF-8 text. LF, CRLF, and CR are line endings. Within ordinary flowing
text, a single line ending supplies a space; a blank line separates paragraphs.
Leading and trailing whitespace around an implicit paragraph is discarded.
Whitespace inside an explicit content body is content: indentation is not a
formatting convention that automatically disappears.

The portable identifier form used in this chapter starts with an ASCII letter
or underscore, followed by ASCII letters, digits, or underscores. Names are
case-sensitive. Separate words in a name with underscores rather than hyphens.
Rules for other identifier characters are outside this chapter.

A directive starts with `#` immediately followed by its name. An opening `(` or
`[` immediately follows the name, and a content body immediately follows an
argument list when both are present:

```mote
#heading(level = 1)[Quarterly report]
```

At the start of a line, hashes followed by a space or tab introduce a Markdown
heading. Thus `# Quarterly report` is a level-one heading, while `#heading(...)`
is a directive call. `**text**` marks strong inline content. Later chapters define
[additional inline forms](inline-text.md), [lists](lists.md), and
[block quotes](block-quotes.md).

## 2. Calls and arguments

The common call forms are:

```text
#name(arguments)
#name[content]
#name(arguments)[content]
```

Arguments are comma-separated. A trailing comma is allowed. Spaces, tabs, and
line endings may surround arguments and `=`. Parentheses and braces inside
quoted strings do not close the surrounding call or object.

An **expression** is source that evaluates to a value. A named argument has the
form `identifier = expression`. A positional argument is an expression without
a name. Each callable defines the allowed arguments, their types, and whether a
body is required. For example, a heading accepts a
positional level:

```mote
#heading(2)[Results]
```

The expression forms specified here are strings, decimal numbers, booleans,
`nil`, property objects, and helper calls. A helper call inside an expression has
no `#`, as in
`spacing = { after = pt(12) }`.

## 3. Literal values

Strings use matching single or double quotes. The quotes are delimiters and are
not part of the value. The following escapes have the same meaning in either
quote style:

| Escape | Value |
| --- | --- |
| `\n` | Line feed |
| `\r` | Carriage return |
| `\t` | Tab |
| `\\` | Backslash |
| `\"` | Double quote |
| `\'` | Single quote |

A literal line ending inside a string remains part of that string. Other escape
forms are outside this chapter.

Numbers include integers and decimal fractions, with an optional leading minus:
`1`, `12.5`, and `-2`. A bare number has no physical unit. Exponent notation and
other numeric spellings are outside this chapter.

`true` and `false` are booleans. `nil` represents the absence of a value. They are
unquoted and distinct from the strings `"true"`, `"false"`, and `"nil"`.

## 4. Property objects and helpers

A property object groups named fields inside braces. Fields use the same
`name = expression` syntax, separated by commas, with an optional trailing comma.
Values can be nested objects or helper calls. As defined in the fundamentals,
`spacing.before` and `spacing.after` request distances outside a paragraph, and
`pt` constructs a length in points. The dot in this explanatory notation means
“the field within the object”; source writes the fields inside braces:

```mote
#paragraph(
  alignment = "center",
  spacing = { before = pt(6), after = pt(12), },
)[Quarterly report]
```

This chapter specifies objects with at least one named field. Empty collections
and arrays are outside its scope.

Helpers return values before the enclosing call uses them. `pt(12)` constructs a
length with magnitude 12 and unit `pt`; the result retains its unit. A negative
length can be constructed, but a property such as paragraph spacing requires a
non-negative value. Accepted values therefore depend on the receiving property.

Prefixing a helper call with `#` in content inserts its result there. A length
becomes its magnitude followed by its unit:

```mote
#paragraph[Length: #pt(12).]
```

The paragraph contains “Length: 12pt.” The length helpers and their accepted
arguments are defined in [Basic content](basic-content.md#5-length-helpers).

## 5. Content bodies and escapes

Square brackets enclose content, which may include text, Markdown, and nested
directives. Nested directive bodies close independently:

```mote
#paragraph[Revenue increased by #strong[12%].]
```

In content, use `\#` for a literal hash, `\*` for an asterisk, `\]` for a closing
bracket, and `\\` for a backslash:

```mote
#paragraph[Write \#strong[text\] to show the syntax.]
```

This produces “Write #strong[text] to show the syntax.” The string escape `\n`
belongs to quoted expressions; it is not a line-break escape in content.
Other backslash forms, including mathematical notation, will be specified
separately.

## 6. Errors

An unterminated string, missing closing delimiter, missing argument value, or
invalid argument name prevents successful evaluation. Diagnostics identify the
source location and explain the problem. Exact diagnostic wording is not part
of this chapter's contract.

Successful parsing does not imply a valid call. Evaluation checks that the name
is known, required arguments and bodies are present, and values satisfy the
callable's rules. For example, `#heading(level = 0)[Title]` has valid syntax but
an invalid level: heading levels must be positive integers.

An implementation must report unsupported directives or helper calls rather
than silently treating them as successful document content. The next chapter,
[Basic content](basic-content.md), defines the calls used here in more detail.
