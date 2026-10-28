# 1.3 About this document

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-commonmark-spec.md) · [Next](003-2-3-insecure-characters.md)

<a id="about-this-document"></a>

## 1.3 About this document

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

Note that not every feature of the HTML samples is mandated by
the spec. For example, the spec says what counts as a link
destination, but it doesn’t mandate that non-ASCII characters in
the URL be percent-encoded. To use the automatic tests,
implementers will need to provide a renderer that conforms to
the expectations of the spec examples (percent-encoding
non-ASCII characters in URLs). But a conforming implementation
can use a different renderer and may choose not to
percent-encode non-ASCII characters in URLs.

This document is generated from a text file, `spec.txt`, written
in Markdown with a small extension for the side-by-side tests.
The script `tools/makespec.py` can be used to convert `spec.txt` into
HTML or CommonMark (which can then be converted into other formats).

In the examples, the `→` character is used to represent tabs.

<a id="preliminaries"></a>

# 2 Preliminaries

<a id="characters-and-lines"></a>

## 2.1 Characters and lines

Any sequence of [characters](002-1-3-about-this-document.md#character) is a valid CommonMark
document.

<a id="character"></a>

A [character](002-1-3-about-this-document.md#character) is a Unicode code point. Although some
code points (for example, combining accents) do not correspond to
characters in an intuitive sense, all code points count as characters
for purposes of this spec.

This spec does not specify an encoding; it thinks of lines as composed
of [characters](002-1-3-about-this-document.md#character) rather than bytes. A conforming parser may be limited
to a certain encoding.

<a id="line"></a>

A [line](002-1-3-about-this-document.md#line) is a sequence of zero or more [characters](002-1-3-about-this-document.md#character)
other than line feed (`U+000A`) or carriage return (`U+000D`),
followed by a [line ending](002-1-3-about-this-document.md#line-ending) or by the end of file.

<a id="line-ending"></a>

A [line ending](002-1-3-about-this-document.md#line-ending) is a line feed (`U+000A`), a carriage return
(`U+000D`) not followed by a line feed, or a carriage return and a
following line feed.

<a id="blank-line"></a>

A line containing no characters, or a line containing only spaces
(`U+0020`) or tabs (`U+0009`), is called a [blank line](002-1-3-about-this-document.md#blank-line).

The following definitions of character classes will be used in this spec:

<a id="unicode-whitespace-character"></a>

A [Unicode whitespace character](002-1-3-about-this-document.md#unicode-whitespace-character) is a character in the Unicode `Zs` general
category, or a tab (`U+0009`), line feed (`U+000A`), form feed (`U+000C`), or
carriage return (`U+000D`).

<a id="unicode-whitespace"></a>

[Unicode whitespace](002-1-3-about-this-document.md#unicode-whitespace) is a sequence of one or more
[Unicode whitespace characters](002-1-3-about-this-document.md#unicode-whitespace-character).

<a id="tab"></a>

A [tab](002-1-3-about-this-document.md#tab) is `U+0009`.

<a id="space"></a>

A [space](002-1-3-about-this-document.md#space) is `U+0020`.

<a id="ascii-control-character"></a>

An [ASCII control character](002-1-3-about-this-document.md#ascii-control-character) is a character between `U+0000–1F` (both
including) or `U+007F`.

<a id="ascii-punctuation-character"></a>

An [ASCII punctuation character](002-1-3-about-this-document.md#ascii-punctuation-character)
is `!`, `"`, `#`, `$`, `%`, `&`, `'`, `(`, `)`,
`*`, `+`, `,`, `-`, `.`, `/` (U+0021–2F),
`:`, `;`, `<`, `=`, `>`, `?`, `@` (U+003A–0040),
`[`, `\`, `]`, `^`, `_`, `` ` `` (U+005B–0060),
`{`, `|`, `}`, or `~` (U+007B–007E).

<a id="unicode-punctuation-character"></a>

A [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character) is a character in the Unicode `P`
(puncuation) or `S` (symbol) general categories.

<a id="tabs"></a>

## 2.2 Tabs

Tabs in lines are not expanded to [spaces](002-1-3-about-this-document.md#space). However,
in contexts where spaces help to define block structure,
tabs behave as if they were replaced by spaces with a tab stop
of 4 characters.

Thus, for example, a tab can be used instead of four spaces
in an indented code block. (Note, however, that internal
tabs are passed through as literal tabs, not expanded to
spaces.)

<a id="example-1"></a>

[Example 1](002-1-3-about-this-document.md#example-1)Try It



```
→foo→baz→→bim
```






```
<pre><code>foo→baz→→bim
</code></pre>
```




<a id="example-2"></a>

[Example 2](002-1-3-about-this-document.md#example-2)Try It



```
  →foo→baz→→bim
```






```
<pre><code>foo→baz→→bim
</code></pre>
```




<a id="example-3"></a>

[Example 3](002-1-3-about-this-document.md#example-3)Try It



```
    a→a
    ὐ→a
```






```
<pre><code>a→a
ὐ→a
</code></pre>
```




In the following example, a continuation paragraph of a list
item is indented with a tab; this has exactly the same effect
as indentation with four spaces would:

<a id="example-4"></a>

[Example 4](002-1-3-about-this-document.md#example-4)Try It



```
  - foo

→bar
```






```
<ul>
<li>
<p>foo</p>
<p>bar</p>
</li>
</ul>
```




<a id="example-5"></a>

[Example 5](002-1-3-about-this-document.md#example-5)Try It



```
- foo

→→bar
```






```
<ul>
<li>
<p>foo</p>
<pre><code>  bar
</code></pre>
</li>
</ul>
```




Normally the `>` that begins a block quote may be followed
optionally by a space, which is not considered part of the
content. In the following case `>` is followed by a tab,
which is treated as if it were expanded into three spaces.
Since one of these spaces is considered part of the
delimiter, `foo` is considered to be indented six spaces
inside the block quote context, so we get an indented
code block starting with two spaces.

<a id="example-6"></a>

[Example 6](002-1-3-about-this-document.md#example-6)Try It



```
>→→foo
```






```
<blockquote>
<pre><code>  foo
</code></pre>
</blockquote>
```




<a id="example-7"></a>

[Example 7](002-1-3-about-this-document.md#example-7)Try It



```
-→→foo
```






```
<ul>
<li>
<pre><code>  foo
</code></pre>
</li>
</ul>
```




<a id="example-8"></a>

[Example 8](002-1-3-about-this-document.md#example-8)Try It



```
    foo
→bar
```






```
<pre><code>foo
bar
</code></pre>
```




<a id="example-9"></a>

[Example 9](002-1-3-about-this-document.md#example-9)Try It



```
 - foo
   - bar
→ - baz
```






```
<ul>
<li>foo
<ul>
<li>bar
<ul>
<li>baz</li>
</ul>
</li>
</ul>
</li>
</ul>
```




<a id="example-10"></a>

[Example 10](002-1-3-about-this-document.md#example-10)Try It



```
#→Foo
```






```
<h1>Foo</h1>
```




<a id="example-11"></a>

[Example 11](002-1-3-about-this-document.md#example-11)Try It



```
*→*→*→
```






```
<hr />
```




