# 1.3 Why is a spec needed?

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-github-flavored-markdown-spec.md) · [Next](003-2-2-tabs.md)

<a id="why-is-a-spec-needed-"></a>

## 1.3 Why is a spec needed?

John Gruber’s [canonical description of Markdown’s
syntax](http://daringfireball.net/projects/markdown/syntax)
does not specify the syntax unambiguously. Here are some examples of
questions it does not answer:

1. How much indentation is needed for a sublist? The spec says that
   continuation paragraphs need to be indented four spaces, but is
   not fully explicit about sublists. It is natural to think that
   they, too, must be indented four spaces, but `Markdown.pl` does
   not require that. This is hardly a “corner case,” and divergences
   between implementations on this issue often lead to surprises for
   users in real documents. (See [this comment by John
   Gruber](http://article.gmane.org/gmane.text.markdown.general/1997).)
2. Is a blank line needed before a block quote or heading?
   Most implementations do not require the blank line. However,
   this can lead to unexpected results in hard-wrapped text, and
   also to ambiguities in parsing (note that some implementations
   put the heading inside the blockquote, while others do not).
   (John Gruber has also spoken [in favor of requiring the blank
   lines](http://article.gmane.org/gmane.text.markdown.general/2146).)
3. Is a blank line needed before an indented code block?
   (`Markdown.pl` requires it, but this is not mentioned in the
   documentation, and some implementations do not require it.)



```
paragraph
    code?
```



4. What is the exact rule for determining when list items get
   wrapped in `<p>` tags? Can a list be partially “loose” and partially
   “tight”? What should we do with a list like this?



```
1. one

2. two
3. three
```




   Or this?



```
1.  one
    - a

    - b
2.  two
```




   (There are some relevant comments by John Gruber
   [here](http://article.gmane.org/gmane.text.markdown.general/2554).)
5. Can list markers be indented? Can ordered list markers be right-aligned?



```
 8. item 1
 9. item 2
10. item 2a
```



6. Is this one list with a thematic break in its second item,
   or two lists separated by a thematic break?



```
* a
* * * * *
* b
```



7. When list markers change from numbers to bullets, do we have
   two lists or one? (The Markdown syntax description suggests two,
   but the perl scripts and many other implementations produce one.)



```
1. fee
2. fie
-  foe
-  fum
```



8. What are the precedence rules for the markers of inline structure?
   For example, is the following a valid link, or does the code span
   take precedence ?



```
[a backtick (`)](/url) and [another backtick (`)](/url).
```



9. What are the precedence rules for markers of emphasis and strong
   emphasis? For example, how should the following be parsed?



```
*foo *bar* baz*
```



10. What are the precedence rules between block-level and inline-level
    structure? For example, how should the following be parsed?



```
- `a long code span can contain a hyphen like this
  - and it can screw things up`
```



11. Can list items include section headings? (`Markdown.pl` does not
    allow this, but does allow blockquotes to include headings.)



```
- # Heading
```



12. Can list items be empty?



```
* a
*
* b
```



13. Can link references be defined inside block quotes or list items?



```
> Blockquote [foo].
>
> [foo]: /url
```



14. If there are multiple definitions for the same reference, which takes
    precedence?



```
[foo]: /url1
[foo]: /url2

[foo][]
```




In the absence of a spec, early implementers consulted `Markdown.pl`
to resolve these ambiguities. But `Markdown.pl` was quite buggy, and
gave manifestly bad results in many cases, so it was not a
satisfactory replacement for a spec.

Because there is no unambiguous spec, implementations have diverged
considerably. As a result, users are often surprised to find that
a document that renders one way on one system (say, a GitHub wiki)
renders differently on another (say, converting to docbook using
pandoc). To make matters worse, because nothing in Markdown counts
as a “syntax error,” the divergence often isn’t discovered right away.

<a id="about-this-document"></a>

## 1.4 About this document

This document attempts to specify Markdown syntax unambiguously.
It contains many examples with side-by-side Markdown and
HTML. These are intended to double as conformance tests. An
accompanying script `spec_tests.py` can be used to run the tests
against any Markdown program:



```
python test/spec_tests.py --spec spec.txt --program PROGRAM
```




Since this document describes how Markdown is to be parsed into
an abstract syntax tree, it would have made sense to use an abstract
representation of the syntax tree instead of HTML. But HTML is capable
of representing the structural distinctions we need to make, and the
choice of HTML for the tests makes it possible to run the tests against
an implementation without writing an abstract syntax tree renderer.

This document is generated from a text file, `spec.txt`, written
in Markdown with a small extension for the side-by-side tests.
The script `tools/makespec.py` can be used to convert `spec.txt` into
HTML or CommonMark (which can then be converted into other formats).

In the examples, the `→` character is used to represent tabs.

<a id="preliminaries"></a>

# 2 Preliminaries

<a id="characters-and-lines"></a>

## 2.1 Characters and lines

Any sequence of [characters](002-1-3-why-is-a-spec-needed.md#character) is a valid CommonMark
document.

<a id="character"></a>

A [character](002-1-3-why-is-a-spec-needed.md#character) is a Unicode code point. Although some
code points (for example, combining accents) do not correspond to
characters in an intuitive sense, all code points count as characters
for purposes of this spec.

This spec does not specify an encoding; it thinks of lines as composed
of [characters](002-1-3-why-is-a-spec-needed.md#character) rather than bytes. A conforming parser may be limited
to a certain encoding.

<a id="line"></a>

A [line](002-1-3-why-is-a-spec-needed.md#line) is a sequence of zero or more [characters](002-1-3-why-is-a-spec-needed.md#character)
other than newline (`U+000A`) or carriage return (`U+000D`),
followed by a [line ending](002-1-3-why-is-a-spec-needed.md#line-ending) or by the end of file.

<a id="line-ending"></a>

A [line ending](002-1-3-why-is-a-spec-needed.md#line-ending) is a newline (`U+000A`), a carriage return
(`U+000D`) not followed by a newline, or a carriage return and a
following newline.

<a id="blank-line"></a>

A line containing no characters, or a line containing only spaces
(`U+0020`) or tabs (`U+0009`), is called a [blank line](002-1-3-why-is-a-spec-needed.md#blank-line).

The following definitions of character classes will be used in this spec:

<a id="whitespace-character"></a>

A [whitespace character](002-1-3-why-is-a-spec-needed.md#whitespace-character) is a space
(`U+0020`), tab (`U+0009`), newline (`U+000A`), line tabulation (`U+000B`),
form feed (`U+000C`), or carriage return (`U+000D`).

<a id="whitespace"></a>

[Whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is a sequence of one or more [whitespace
characters](002-1-3-why-is-a-spec-needed.md#whitespace-character).

<a id="unicode-whitespace-character"></a>

A [Unicode whitespace character](002-1-3-why-is-a-spec-needed.md#unicode-whitespace-character) is
any code point in the Unicode `Zs` general category, or a tab (`U+0009`),
carriage return (`U+000D`), newline (`U+000A`), or form feed
(`U+000C`).

<a id="unicode-whitespace"></a>

[Unicode whitespace](002-1-3-why-is-a-spec-needed.md#unicode-whitespace) is a sequence of one
or more [Unicode whitespace characters](002-1-3-why-is-a-spec-needed.md#unicode-whitespace-character).

<a id="space"></a>

A [space](002-1-3-why-is-a-spec-needed.md#space) is `U+0020`.

<a id="non-whitespace-character"></a>

A [non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character) is any character
that is not a [whitespace character](002-1-3-why-is-a-spec-needed.md#whitespace-character).

<a id="ascii-punctuation-character"></a>

An [ASCII punctuation character](002-1-3-why-is-a-spec-needed.md#ascii-punctuation-character)
is `!`, `"`, `#`, `$`, `%`, `&`, `'`, `(`, `)`,
`*`, `+`, `,`, `-`, `.`, `/` (U+0021–2F),
`:`, `;`, `<`, `=`, `>`, `?`, `@` (U+003A–0040),
`[`, `\`, `]`, `^`, `_`, `` ` `` (U+005B–0060),
`{`, `|`, `}`, or `~` (U+007B–007E).

<a id="punctuation-character"></a>

A [punctuation character](002-1-3-why-is-a-spec-needed.md#punctuation-character) is an [ASCII
punctuation character](002-1-3-why-is-a-spec-needed.md#ascii-punctuation-character) or anything in
the general Unicode categories `Pc`, `Pd`, `Pe`, `Pf`, `Pi`, `Po`, or `Ps`.

