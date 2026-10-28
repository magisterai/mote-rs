# 4.2 ATX headings

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-2-2-tabs.md) · [Next](005-4-4-indented-code-blocks.md)

<a id="atx-headings"></a>

## 4.2 ATX headings

<a id="atx-heading"></a>

An [ATX heading](004-4-2-atx-headings.md#atx-heading)
consists of a string of characters, parsed as inline content, between an
opening sequence of 1–6 unescaped `#` characters and an optional
closing sequence of any number of unescaped `#` characters.
The opening sequence of `#` characters must be followed by a
[space](002-1-3-why-is-a-spec-needed.md#space) or by the end of line. The optional closing sequence of `#`s must be
preceded by a [space](002-1-3-why-is-a-spec-needed.md#space) and may be followed by spaces only. The opening
`#` character may be indented 0-3 spaces. The raw contents of the
heading are stripped of leading and trailing spaces before being parsed
as inline content. The heading level is equal to the number of `#`
characters in the opening sequence.

Simple headings:

<a id="example-32"></a>

[Example 32](004-4-2-atx-headings.md#example-32)



```
# foo
## foo
### foo
#### foo
##### foo
###### foo
```






```
<h1>foo</h1>
<h2>foo</h2>
<h3>foo</h3>
<h4>foo</h4>
<h5>foo</h5>
<h6>foo</h6>
```




More than six `#` characters is not a heading:

<a id="example-33"></a>

[Example 33](004-4-2-atx-headings.md#example-33)



```
####### foo
```






```
<p>####### foo</p>
```




At least one space is required between the `#` characters and the
heading’s contents, unless the heading is empty. Note that many
implementations currently do not require the space. However, the
space was required by the
[original ATX implementation](http://www.aaronsw.com/2002/atx/atx.py),
and it helps prevent things like the following from being parsed as
headings:

<a id="example-34"></a>

[Example 34](004-4-2-atx-headings.md#example-34)



```
#5 bolt

#hashtag
```






```
<p>#5 bolt</p>
<p>#hashtag</p>
```




This is not a heading, because the first `#` is escaped:

<a id="example-35"></a>

[Example 35](004-4-2-atx-headings.md#example-35)



```
\## foo
```






```
<p>## foo</p>
```




Contents are parsed as inlines:

<a id="example-36"></a>

[Example 36](004-4-2-atx-headings.md#example-36)



```
# foo *bar* \*baz\*
```






```
<h1>foo <em>bar</em> *baz*</h1>
```




Leading and trailing [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is ignored in parsing inline content:

<a id="example-37"></a>

[Example 37](004-4-2-atx-headings.md#example-37)



```
#                  foo                     
```






```
<h1>foo</h1>
```




One to three spaces indentation are allowed:

<a id="example-38"></a>

[Example 38](004-4-2-atx-headings.md#example-38)



```
 ### foo
  ## foo
   # foo
```






```
<h3>foo</h3>
<h2>foo</h2>
<h1>foo</h1>
```




Four spaces are too much:

<a id="example-39"></a>

[Example 39](004-4-2-atx-headings.md#example-39)



```
    # foo
```






```
<pre><code># foo
</code></pre>
```




<a id="example-40"></a>

[Example 40](004-4-2-atx-headings.md#example-40)



```
foo
    # bar
```






```
<p>foo
# bar</p>
```




A closing sequence of `#` characters is optional:

<a id="example-41"></a>

[Example 41](004-4-2-atx-headings.md#example-41)



```
## foo ##
  ###   bar    ###
```






```
<h2>foo</h2>
<h3>bar</h3>
```




It need not be the same length as the opening sequence:

<a id="example-42"></a>

[Example 42](004-4-2-atx-headings.md#example-42)



```
# foo ##################################
##### foo ##
```






```
<h1>foo</h1>
<h5>foo</h5>
```




Spaces are allowed after the closing sequence:

<a id="example-43"></a>

[Example 43](004-4-2-atx-headings.md#example-43)



```
### foo ###     
```






```
<h3>foo</h3>
```




A sequence of `#` characters with anything but [spaces](002-1-3-why-is-a-spec-needed.md#space) following it
is not a closing sequence, but counts as part of the contents of the
heading:

<a id="example-44"></a>

[Example 44](004-4-2-atx-headings.md#example-44)



```
### foo ### b
```






```
<h3>foo ### b</h3>
```




The closing sequence must be preceded by a space:

<a id="example-45"></a>

[Example 45](004-4-2-atx-headings.md#example-45)



```
# foo#
```






```
<h1>foo#</h1>
```




Backslash-escaped `#` characters do not count as part
of the closing sequence:

<a id="example-46"></a>

[Example 46](004-4-2-atx-headings.md#example-46)



```
### foo \###
## foo #\##
# foo \#
```






```
<h3>foo ###</h3>
<h2>foo ###</h2>
<h1>foo #</h1>
```




ATX headings need not be separated from surrounding content by blank
lines, and they can interrupt paragraphs:

<a id="example-47"></a>

[Example 47](004-4-2-atx-headings.md#example-47)



```
****
## foo
****
```






```
<hr />
<h2>foo</h2>
<hr />
```




<a id="example-48"></a>

[Example 48](004-4-2-atx-headings.md#example-48)



```
Foo bar
# baz
Bar foo
```






```
<p>Foo bar</p>
<h1>baz</h1>
<p>Bar foo</p>
```




ATX headings can be empty:

<a id="example-49"></a>

[Example 49](004-4-2-atx-headings.md#example-49)



```
## 
#
### ###
```






```
<h2></h2>
<h1></h1>
<h3></h3>
```




<a id="setext-headings"></a>

## 4.3 Setext headings

<a id="setext-heading"></a>

A [setext heading](004-4-2-atx-headings.md#setext-heading) consists of one or more
lines of text, each containing at least one [non-whitespace
character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character), with no more than 3 spaces indentation, followed by
a [setext heading underline](004-4-2-atx-headings.md#setext-heading-underline). The lines of text must be such
that, were they not followed by the setext heading underline,
they would be interpreted as a paragraph: they cannot be
interpretable as a [code fence](005-4-4-indented-code-blocks.md#code-fence), [ATX heading](004-4-2-atx-headings.md#atx-headings),
[block quote](009-5-container-blocks.md#block-quotes), [thematic break](003-2-2-tabs.md#thematic-break),
[list item](010-5-2-list-items.md#list-items), or [HTML block](006-4-6-html-blocks.md#html-blocks).

<a id="setext-heading-underline"></a>

A [setext heading underline](004-4-2-atx-headings.md#setext-heading-underline) is a sequence of
`=` characters or a sequence of `-` characters, with no more than 3
spaces indentation and any number of trailing spaces. If a line
containing a single `-` can be interpreted as an
empty [list items](010-5-2-list-items.md#list-items), it should be interpreted this way
and not as a [setext heading underline](004-4-2-atx-headings.md#setext-heading-underline).

The heading is a level 1 heading if `=` characters are used in
the [setext heading underline](004-4-2-atx-headings.md#setext-heading-underline), and a level 2 heading if `-`
characters are used. The contents of the heading are the result
of parsing the preceding lines of text as CommonMark inline
content.

In general, a setext heading need not be preceded or followed by a
blank line. However, it cannot interrupt a paragraph, so when a
setext heading comes after a paragraph, a blank line is needed between
them.

Simple examples:

<a id="example-50"></a>

[Example 50](004-4-2-atx-headings.md#example-50)



```
Foo *bar*
=========

Foo *bar*
---------
```






```
<h1>Foo <em>bar</em></h1>
<h2>Foo <em>bar</em></h2>
```




The content of the header may span more than one line:

<a id="example-51"></a>

[Example 51](004-4-2-atx-headings.md#example-51)



```
Foo *bar
baz*
====
```






```
<h1>Foo <em>bar
baz</em></h1>
```




The contents are the result of parsing the headings’s raw
content as inlines. The heading’s raw content is formed by
concatenating the lines and removing initial and final
[whitespace](002-1-3-why-is-a-spec-needed.md#whitespace).

<a id="example-52"></a>

[Example 52](004-4-2-atx-headings.md#example-52)



```
  Foo *bar
baz*→
====
```






```
<h1>Foo <em>bar
baz</em></h1>
```




The underlining can be any length:

<a id="example-53"></a>

[Example 53](004-4-2-atx-headings.md#example-53)



```
Foo
-------------------------

Foo
=
```






```
<h2>Foo</h2>
<h1>Foo</h1>
```




The heading content can be indented up to three spaces, and need
not line up with the underlining:

<a id="example-54"></a>

[Example 54](004-4-2-atx-headings.md#example-54)



```
   Foo
---

  Foo
-----

  Foo
  ===
```






```
<h2>Foo</h2>
<h2>Foo</h2>
<h1>Foo</h1>
```




Four spaces indent is too much:

<a id="example-55"></a>

[Example 55](004-4-2-atx-headings.md#example-55)



```
    Foo
    ---

    Foo
---
```






```
<pre><code>Foo
---

Foo
</code></pre>
<hr />
```




The setext heading underline can be indented up to three spaces, and
may have trailing spaces:

<a id="example-56"></a>

[Example 56](004-4-2-atx-headings.md#example-56)



```
Foo
   ----      
```






```
<h2>Foo</h2>
```




Four spaces is too much:

<a id="example-57"></a>

[Example 57](004-4-2-atx-headings.md#example-57)



```
Foo
    ---
```






```
<p>Foo
---</p>
```




The setext heading underline cannot contain internal spaces:

<a id="example-58"></a>

[Example 58](004-4-2-atx-headings.md#example-58)



```
Foo
= =

Foo
--- -
```






```
<p>Foo
= =</p>
<p>Foo</p>
<hr />
```




Trailing spaces in the content line do not cause a line break:

<a id="example-59"></a>

[Example 59](004-4-2-atx-headings.md#example-59)



```
Foo  
-----
```






```
<h2>Foo</h2>
```




Nor does a backslash at the end:

<a id="example-60"></a>

[Example 60](004-4-2-atx-headings.md#example-60)



```
Foo\
----
```






```
<h2>Foo\</h2>
```




Since indicators of block structure take precedence over
indicators of inline structure, the following are setext headings:

<a id="example-61"></a>

[Example 61](004-4-2-atx-headings.md#example-61)



```
`Foo
----
`

<a title="a lot
---
of dashes"/>
```






```
<h2>`Foo</h2>
<p>`</p>
<h2>&lt;a title=&quot;a lot</h2>
<p>of dashes&quot;/&gt;</p>
```




The setext heading underline cannot be a [lazy continuation
line](011-5-2-1-motivation.md#lazy-continuation-line) in a list item or block quote:

<a id="example-62"></a>

[Example 62](004-4-2-atx-headings.md#example-62)



```
> Foo
---
```






```
<blockquote>
<p>Foo</p>
</blockquote>
<hr />
```




<a id="example-63"></a>

[Example 63](004-4-2-atx-headings.md#example-63)



```
> foo
bar
===
```






```
<blockquote>
<p>foo
bar
===</p>
</blockquote>
```




<a id="example-64"></a>

[Example 64](004-4-2-atx-headings.md#example-64)



```
- Foo
---
```






```
<ul>
<li>Foo</li>
</ul>
<hr />
```




A blank line is needed between a paragraph and a following
setext heading, since otherwise the paragraph becomes part
of the heading’s content:

<a id="example-65"></a>

[Example 65](004-4-2-atx-headings.md#example-65)



```
Foo
Bar
---
```






```
<h2>Foo
Bar</h2>
```




But in general a blank line is not required before or after
setext headings:

<a id="example-66"></a>

[Example 66](004-4-2-atx-headings.md#example-66)



```
---
Foo
---
Bar
---
Baz
```






```
<hr />
<h2>Foo</h2>
<h2>Bar</h2>
<p>Baz</p>
```




Setext headings cannot be empty:

<a id="example-67"></a>

[Example 67](004-4-2-atx-headings.md#example-67)



```

====
```






```
<p>====</p>
```




Setext heading text lines must not be interpretable as block
constructs other than paragraphs. So, the line of dashes
in these examples gets interpreted as a thematic break:

<a id="example-68"></a>

[Example 68](004-4-2-atx-headings.md#example-68)



```
---
---
```






```
<hr />
<hr />
```




<a id="example-69"></a>

[Example 69](004-4-2-atx-headings.md#example-69)



```
- foo
-----
```






```
<ul>
<li>foo</li>
</ul>
<hr />
```




<a id="example-70"></a>

[Example 70](004-4-2-atx-headings.md#example-70)



```
    foo
---
```






```
<pre><code>foo
</code></pre>
<hr />
```




<a id="example-71"></a>

[Example 71](004-4-2-atx-headings.md#example-71)



```
> foo
-----
```






```
<blockquote>
<p>foo</p>
</blockquote>
<hr />
```




If you want a heading with `> foo` as its literal text, you can
use backslash escapes:

<a id="example-72"></a>

[Example 72](004-4-2-atx-headings.md#example-72)



```
\> foo
------
```






```
<h2>&gt; foo</h2>
```




**Compatibility note:** Most existing Markdown implementations
do not allow the text of setext headings to span multiple lines.
But there is no consensus about how to interpret



```
Foo
bar
---
baz
```




One can find four different interpretations:

1. paragraph “Foo”, heading “bar”, paragraph “baz”
2. paragraph “Foo bar”, thematic break, paragraph “baz”
3. paragraph “Foo bar — baz”
4. heading “Foo bar”, paragraph “baz”

We find interpretation 4 most natural, and interpretation 4
increases the expressive power of CommonMark, by allowing
multiline headings. Authors who want interpretation 1 can
put a blank line after the first paragraph:

<a id="example-73"></a>

[Example 73](004-4-2-atx-headings.md#example-73)



```
Foo

bar
---
baz
```






```
<p>Foo</p>
<h2>bar</h2>
<p>baz</p>
```




Authors who want interpretation 2 can put blank lines around
the thematic break,

<a id="example-74"></a>

[Example 74](004-4-2-atx-headings.md#example-74)



```
Foo
bar

---

baz
```






```
<p>Foo
bar</p>
<hr />
<p>baz</p>
```




or use a thematic break that cannot count as a [setext heading
underline](004-4-2-atx-headings.md#setext-heading-underline), such as

<a id="example-75"></a>

[Example 75](004-4-2-atx-headings.md#example-75)



```
Foo
bar
* * *
baz
```






```
<p>Foo
bar</p>
<hr />
<p>baz</p>
```




Authors who want interpretation 3 can use backslash escapes:

<a id="example-76"></a>

[Example 76](004-4-2-atx-headings.md#example-76)



```
Foo
bar
\---
baz
```






```
<p>Foo
bar
---
baz</p>
```




