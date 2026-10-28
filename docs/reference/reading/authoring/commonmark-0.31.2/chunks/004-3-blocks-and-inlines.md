# 3 Blocks and inlines

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-2-3-insecure-characters.md) · [Next](005-4-3-setext-headings.md)

<a id="blocks-and-inlines"></a>

# 3 Blocks and inlines

<a id="blocks"></a>

<a id="inline"></a>

We can think of a document as a sequence of
[blocks](004-3-blocks-and-inlines.md#blocks)—structural elements like paragraphs, block
quotations, lists, headings, rules, and code blocks. Some blocks (like
block quotes and list items) contain other blocks; others (like
headings and paragraphs) contain [inline](004-3-blocks-and-inlines.md#inline) content—text,
links, emphasized text, images, code spans, and so on.

<a id="precedence"></a>

## 3.1 Precedence

Indicators of block structure always take precedence over indicators
of inline structure. So, for example, the following is a list with
two items, not a list with one item containing a code span:

<a id="example-42"></a>

[Example 42](004-3-blocks-and-inlines.md#example-42)Try It



```
- `one
- two`
```






```
<ul>
<li>`one</li>
<li>two`</li>
</ul>
```




This means that parsing can proceed in two steps: first, the block
structure of the document can be discerned; second, text lines inside
paragraphs, headings, and other block constructs can be parsed for inline
structure. The second step requires information about link reference
definitions that will be available only at the end of the first
step. Note that the first step requires processing lines in sequence,
but the second can be parallelized, since the inline parsing of
one block element does not affect the inline parsing of any other.

<a id="container-blocks-and-leaf-blocks"></a>

## 3.2 Container blocks and leaf blocks

We can divide blocks into two types:
[container blocks](010-4-8-paragraphs.md#container-blocks),
which can contain other blocks, and [leaf blocks](004-3-blocks-and-inlines.md#leaf-blocks),
which cannot.

<a id="leaf-blocks"></a>

# 4 Leaf blocks

This section describes the different kinds of leaf block that make up a
Markdown document.

<a id="thematic-breaks"></a>

## 4.1 Thematic breaks

<a id="thematic-break"></a>

A line consisting of optionally up to three spaces of indentation, followed by a
sequence of three or more matching `-`, `_`, or `*` characters, each followed
optionally by any number of spaces or tabs, forms a
[thematic break](004-3-blocks-and-inlines.md#thematic-break).

<a id="example-43"></a>

[Example 43](004-3-blocks-and-inlines.md#example-43)Try It



```
***
---
___
```






```
<hr />
<hr />
<hr />
```




Wrong characters:

<a id="example-44"></a>

[Example 44](004-3-blocks-and-inlines.md#example-44)Try It



```
+++
```






```
<p>+++</p>
```




<a id="example-45"></a>

[Example 45](004-3-blocks-and-inlines.md#example-45)Try It



```
===
```






```
<p>===</p>
```




Not enough characters:

<a id="example-46"></a>

[Example 46](004-3-blocks-and-inlines.md#example-46)Try It



```
--
**
__
```






```
<p>--
**
__</p>
```




Up to three spaces of indentation are allowed:

<a id="example-47"></a>

[Example 47](004-3-blocks-and-inlines.md#example-47)Try It



```
 ***
  ***
   ***
```






```
<hr />
<hr />
<hr />
```




Four spaces of indentation is too many:

<a id="example-48"></a>

[Example 48](004-3-blocks-and-inlines.md#example-48)Try It



```
    ***
```






```
<pre><code>***
</code></pre>
```




<a id="example-49"></a>

[Example 49](004-3-blocks-and-inlines.md#example-49)Try It



```
Foo
    ***
```






```
<p>Foo
***</p>
```




More than three characters may be used:

<a id="example-50"></a>

[Example 50](004-3-blocks-and-inlines.md#example-50)Try It



```
_____________________________________
```






```
<hr />
```




Spaces and tabs are allowed between the characters:

<a id="example-51"></a>

[Example 51](004-3-blocks-and-inlines.md#example-51)Try It



```
 - - -
```






```
<hr />
```




<a id="example-52"></a>

[Example 52](004-3-blocks-and-inlines.md#example-52)Try It



```
 **  * ** * ** * **
```






```
<hr />
```




<a id="example-53"></a>

[Example 53](004-3-blocks-and-inlines.md#example-53)Try It



```
-     -      -      -
```






```
<hr />
```




Spaces and tabs are allowed at the end:

<a id="example-54"></a>

[Example 54](004-3-blocks-and-inlines.md#example-54)Try It



```
- - - -    
```






```
<hr />
```




However, no other characters may occur in the line:

<a id="example-55"></a>

[Example 55](004-3-blocks-and-inlines.md#example-55)Try It



```
_ _ _ _ a

a------

---a---
```






```
<p>_ _ _ _ a</p>
<p>a------</p>
<p>---a---</p>
```




It is required that all of the characters other than spaces or tabs be the same.
So, this is not a thematic break:

<a id="example-56"></a>

[Example 56](004-3-blocks-and-inlines.md#example-56)Try It



```
 *-*
```






```
<p><em>-</em></p>
```




Thematic breaks do not need blank lines before or after:

<a id="example-57"></a>

[Example 57](004-3-blocks-and-inlines.md#example-57)Try It



```
- foo
***
- bar
```






```
<ul>
<li>foo</li>
</ul>
<hr />
<ul>
<li>bar</li>
</ul>
```




Thematic breaks can interrupt a paragraph:

<a id="example-58"></a>

[Example 58](004-3-blocks-and-inlines.md#example-58)Try It



```
Foo
***
bar
```






```
<p>Foo</p>
<hr />
<p>bar</p>
```




If a line of dashes that meets the above conditions for being a
thematic break could also be interpreted as the underline of a [setext
heading](005-4-3-setext-headings.md#setext-heading), the interpretation as a
[setext heading](005-4-3-setext-headings.md#setext-heading) takes precedence. Thus, for example,
this is a setext heading, not a paragraph followed by a thematic break:

<a id="example-59"></a>

[Example 59](004-3-blocks-and-inlines.md#example-59)Try It



```
Foo
---
bar
```






```
<h2>Foo</h2>
<p>bar</p>
```




When both a thematic break and a list item are possible
interpretations of a line, the thematic break takes precedence:

<a id="example-60"></a>

[Example 60](004-3-blocks-and-inlines.md#example-60)Try It



```
* Foo
* * *
* Bar
```






```
<ul>
<li>Foo</li>
</ul>
<hr />
<ul>
<li>Bar</li>
</ul>
```




If you want a thematic break in a list item, use a different bullet:

<a id="example-61"></a>

[Example 61](004-3-blocks-and-inlines.md#example-61)Try It



```
- Foo
- * * *
```






```
<ul>
<li>Foo</li>
<li>
<hr />
</li>
</ul>
```




<a id="atx-headings"></a>

## 4.2 ATX headings

<a id="atx-heading"></a>

An [ATX heading](004-3-blocks-and-inlines.md#atx-heading)
consists of a string of characters, parsed as inline content, between an
opening sequence of 1–6 unescaped `#` characters and an optional
closing sequence of any number of unescaped `#` characters.
The opening sequence of `#` characters must be followed by spaces or tabs, or
by the end of line. The optional closing sequence of `#`s must be preceded by
spaces or tabs and may be followed by spaces or tabs only. The opening
`#` character may be preceded by up to three spaces of indentation. The raw
contents of the heading are stripped of leading and trailing space or tabs
before being parsed as inline content. The heading level is equal to the number
of `#` characters in the opening sequence.

Simple headings:

<a id="example-62"></a>

[Example 62](004-3-blocks-and-inlines.md#example-62)Try It



