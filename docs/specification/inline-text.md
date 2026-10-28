# Inline text

[Specification index](README.md) · [Basic content](basic-content.md)

Status: initial draft. This chapter adds emphasis, inline code, and explicit line
breaks to the paragraphs, headings, and strong spans already defined. All are
inline content. At document level, they become part of an implicit paragraph.

## 1. Emphasis

**Emphasis** requests italic text. The `emphasis` directive requires exactly one
inline body, which may be empty. It accepts no positional arguments. This chapter
defines no named attributes for it. Paragraph and heading blocks are not allowed
inside its body.

```mote
#paragraph[Read #emphasis[this] carefully.]
```

Only “this” is emphasized. Enclosing text in single underscores is the Markdown
form and produces the same content:

```mote
Read _this_ carefully.
```

Use `\_` for a literal underscore. An opening emphasis delimiter requires a
closing delimiter. Mote uses underscores for emphasis; a single asterisk pair
requests strong text. Double asterisks and double underscores also request strong
text. The examples use double asterisks for strong text consistently.

Emphasis and strong text can be nested. The inner span retains the formatting
request of the outer span, so the following word requests both bold and italic:

```mote
#paragraph[#strong[#emphasis[Both]]]
```

The body may itself contain ordinary text and other inline elements. Nesting
does not add spaces or create another paragraph. Surrounding spaces inside an
explicit body remain content, as defined in the basic-content chapter.

## 2. Literal inline code

**Inline code** marks a short piece of code or other literal text within a block.
Its evaluated content records that intent; font selection belongs to later
formatting rules.

Enclose literal text in backticks. The enclosed text is not interpreted as
Markdown or directives, and backslashes remain literal:

```mote
#paragraph[Write `#strong[text]` literally.]
```

The code span contains the characters `#strong[text]`. It contains no strong
element. The `code` directive can also take one positional string argument. That
string is evaluated as a string, including the string escapes already defined,
and then becomes the code text:

```mote
#paragraph[Write #code("#strong[text]") literally.]
```

These two examples have the same evaluated content.

A pair of double backticks can enclose a single backtick. Longer opening runs
use a closing run of the same length. If no matching closing run exists, the
unmatched backticks remain ordinary text.

```mote
#paragraph[Code: ``a ` b``.]
```

Here the code text is “a ` b”. Backtick spans preserve line endings and internal
spaces. If the text starts and ends with an ordinary space and contains a
non-whitespace character, exactly one space is removed from each end. A span
containing only whitespace retains all of it. This padding rule allows a
backtick at the edge of the literal text to be separated from its delimiter.

```mote
#paragraph[Code: `  padded  `.]
```

The code text has one space before and one after “padded”.

## 3. Code directive bodies

The `code` directive may instead take one bracketed inline body. A body follows
the normal content rules: directives and Markdown inside it are evaluated.
Surrounding spaces are preserved, and a normal source line ending supplies a
space. Use the string argument or backtick form when the text must be literal.

```mote
#paragraph[Code: #code[#strong[name]].]
```

This code element contains a strong span containing “name”.

`code` accepts at most one positional argument and at most one body. The
positional value may be a string, a number, or a boolean; numbers and booleans
become their textual values. If both an argument and a body are supplied, the
argument supplies the code text; the body is still evaluated and validated.
No named attributes are defined in this chapter.

Neither an argument nor a body is required. An empty call creates an empty code
element:

```mote
#paragraph[Before #code() after.]
```

## 4. Explicit line breaks

A **line break** starts another line within the same paragraph or heading. It
does not create a new paragraph. The `line_break` directive takes no positional arguments
and no body; its empty parentheses make it a call:

```mote
#paragraph[First line#line_break()Second line]
```

The paragraph contains text, a line-break element, and more text. The break
does not contribute a space on either side.

Markdown can request a hard line break with a backslash immediately followed by
a source line ending. Two or more ordinary spaces immediately before a line
ending also request a hard break; those spaces are consumed as syntax. A lone
ordinary line ending remains a space, and a blank line separates paragraphs at
document level.

```mote
First line\
Second line
```

This is one paragraph containing “First line”, a line feed, and “Second line”.
The Markdown form records the break in text, while the directive retains an
explicit `line_break` element. Both request a line break within the block.
The quoted-string escape `\n` remains a string escape, not content syntax.

## 5. Invalid uses

These examples intentionally fail evaluation.

Emphasis requires a body:

```mote
#emphasis()
```

Inline code bodies cannot contain paragraph blocks:

```mote
#code[#paragraph[Block]]
```

The absence value `nil` is not a supported positional code value:

```mote
#code(nil)
```

A line-break directive cannot take a body:

```mote
#line_break()[Text]
```

Continue with [Lists](lists.md), which introduces blocks that contain items.
