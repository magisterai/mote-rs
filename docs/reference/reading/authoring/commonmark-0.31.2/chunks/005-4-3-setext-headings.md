# 4.3 Setext headings

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-3-blocks-and-inlines.md) · [Next](006-4-4-indented-code-blocks.md)

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

<a id="example-63"></a>

[Example 63](005-4-3-setext-headings.md#example-63)Try It



```
####### foo
```






```
<p>####### foo</p>
```




At least one space or tab is required between the `#` characters and the
heading’s contents, unless the heading is empty. Note that many
implementations currently do not require the space. However, the
space was required by the
[original ATX implementation](http://www.aaronsw.com/2002/atx/atx.py),
and it helps prevent things like the following from being parsed as
headings:

<a id="example-64"></a>

[Example 64](005-4-3-setext-headings.md#example-64)Try It



```
#5 bolt

#hashtag
```






```
<p>#5 bolt</p>
<p>#hashtag</p>
```




This is not a heading, because the first `#` is escaped:

<a id="example-65"></a>

[Example 65](005-4-3-setext-headings.md#example-65)Try It



```
\## foo
```






```
<p>## foo</p>
```




Contents are parsed as inlines:

<a id="example-66"></a>

[Example 66](005-4-3-setext-headings.md#example-66)Try It



```
# foo *bar* \*baz\*
```






```
<h1>foo <em>bar</em> *baz*</h1>
```




Leading and trailing spaces or tabs are ignored in parsing inline content:

<a id="example-67"></a>

[Example 67](005-4-3-setext-headings.md#example-67)Try It



```
#                  foo                     
```






```
<h1>foo</h1>
```




Up to three spaces of indentation are allowed:

<a id="example-68"></a>

[Example 68](005-4-3-setext-headings.md#example-68)Try It



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




Four spaces of indentation is too many:

<a id="example-69"></a>

[Example 69](005-4-3-setext-headings.md#example-69)Try It



```
    # foo
```






```
<pre><code># foo
</code></pre>
```




<a id="example-70"></a>

[Example 70](005-4-3-setext-headings.md#example-70)Try It



```
foo
    # bar
```






```
<p>foo
# bar</p>
```




A closing sequence of `#` characters is optional:

<a id="example-71"></a>

[Example 71](005-4-3-setext-headings.md#example-71)Try It



```
## foo ##
  ###   bar    ###
```






```
<h2>foo</h2>
<h3>bar</h3>
```




It need not be the same length as the opening sequence:

<a id="example-72"></a>

[Example 72](005-4-3-setext-headings.md#example-72)Try It



```
# foo ##################################
##### foo ##
```






```
<h1>foo</h1>
<h5>foo</h5>
```




Spaces or tabs are allowed after the closing sequence:

<a id="example-73"></a>

[Example 73](005-4-3-setext-headings.md#example-73)Try It



```
### foo ###     
```






```
<h3>foo</h3>
```




A sequence of `#` characters with anything but spaces or tabs following it
is not a closing sequence, but counts as part of the contents of the
heading:

<a id="example-74"></a>

[Example 74](005-4-3-setext-headings.md#example-74)Try It



```
### foo ### b
```






```
<h3>foo ### b</h3>
```




The closing sequence must be preceded by a space or tab:

<a id="example-75"></a>

[Example 75](005-4-3-setext-headings.md#example-75)Try It



```
# foo#
```






```
<h1>foo#</h1>
```




Backslash-escaped `#` characters do not count as part
of the closing sequence:

<a id="example-76"></a>

[Example 76](005-4-3-setext-headings.md#example-76)Try It



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

<a id="example-77"></a>

[Example 77](005-4-3-setext-headings.md#example-77)Try It



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




<a id="example-78"></a>

[Example 78](005-4-3-setext-headings.md#example-78)Try It



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

<a id="example-79"></a>

[Example 79](005-4-3-setext-headings.md#example-79)Try It



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

A [setext heading](005-4-3-setext-headings.md#setext-heading) consists of one or more
lines of text, not interrupted by a blank line, of which the first line does not
have more than 3 spaces of indentation, followed by
a [setext heading underline](005-4-3-setext-headings.md#setext-heading-underline). The lines of text must be such
that, were they not followed by the setext heading underline,
they would be interpreted as a paragraph: they cannot be
interpretable as a [code fence](006-4-4-indented-code-blocks.md#code-fence), [ATX heading](004-3-blocks-and-inlines.md#atx-headings),
[block quote](010-4-8-paragraphs.md#block-quotes), [thematic break](004-3-blocks-and-inlines.md#thematic-breaks),
[list item](011-5-2-list-items.md#list-items), or [HTML block](007-4-6-html-blocks.md#html-blocks).

<a id="setext-heading-underline"></a>

A [setext heading underline](005-4-3-setext-headings.md#setext-heading-underline) is a sequence of
`=` characters or a sequence of `-` characters, with no more than 3
spaces of indentation and any number of trailing spaces or tabs.

The heading is a level 1 heading if `=` characters are used in
the [setext heading underline](005-4-3-setext-headings.md#setext-heading-underline), and a level 2 heading if `-`
characters are used. The contents of the heading are the result
of parsing the preceding lines of text as CommonMark inline
content.

In general, a setext heading need not be preceded or followed by a
blank line. However, it cannot interrupt a paragraph, so when a
setext heading comes after a paragraph, a blank line is needed between
them.

Simple examples:

<a id="example-80"></a>

[Example 80](005-4-3-setext-headings.md#example-80)Try It



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

<a id="example-81"></a>

[Example 81](005-4-3-setext-headings.md#example-81)Try It



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
spaces or tabs.

<a id="example-82"></a>

[Example 82](005-4-3-setext-headings.md#example-82)Try It



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

<a id="example-83"></a>

[Example 83](005-4-3-setext-headings.md#example-83)Try It



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




The heading content can be preceded by up to three spaces of indentation, and
need not line up with the underlining:

<a id="example-84"></a>

[Example 84](005-4-3-setext-headings.md#example-84)Try It



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




Four spaces of indentation is too many:

<a id="example-85"></a>

[Example 85](005-4-3-setext-headings.md#example-85)Try It



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




The setext heading underline can be preceded by up to three spaces of
indentation, and may have trailing spaces or tabs:

<a id="example-86"></a>

[Example 86](005-4-3-setext-headings.md#example-86)Try It



```
Foo
   ----      
```






```
<h2>Foo</h2>
```




Four spaces of indentation is too many:

<a id="example-87"></a>

[Example 87](005-4-3-setext-headings.md#example-87)Try It



```
Foo
    ---
```






```
<p>Foo
---</p>
```




The setext heading underline cannot contain internal spaces or tabs:

<a id="example-88"></a>

[Example 88](005-4-3-setext-headings.md#example-88)Try It



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




Trailing spaces or tabs in the content line do not cause a hard line break:

<a id="example-89"></a>

[Example 89](005-4-3-setext-headings.md#example-89)Try It



```
Foo  
-----
```






```
<h2>Foo</h2>
```




Nor does a backslash at the end:

<a id="example-90"></a>

[Example 90](005-4-3-setext-headings.md#example-90)Try It



```
Foo\
----
```






```
<h2>Foo\</h2>
```




Since indicators of block structure take precedence over
indicators of inline structure, the following are setext headings:

<a id="example-91"></a>

[Example 91](005-4-3-setext-headings.md#example-91)Try It



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
line](012-continuation-12.md#lazy-continuation-line) in a list item or block quote:

<a id="example-92"></a>

[Example 92](005-4-3-setext-headings.md#example-92)Try It



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




<a id="example-93"></a>

[Example 93](005-4-3-setext-headings.md#example-93)Try It



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




<a id="example-94"></a>

[Example 94](005-4-3-setext-headings.md#example-94)Try It



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

<a id="example-95"></a>

[Example 95](005-4-3-setext-headings.md#example-95)Try It



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

<a id="example-96"></a>

[Example 96](005-4-3-setext-headings.md#example-96)Try It



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

<a id="example-97"></a>

[Example 97](005-4-3-setext-headings.md#example-97)Try It



```

====
```






```
<p>====</p>
```




Setext heading text lines must not be interpretable as block
constructs other than paragraphs. So, the line of dashes
in these examples gets interpreted as a thematic break:

<a id="example-98"></a>

[Example 98](005-4-3-setext-headings.md#example-98)Try It



```
---
---
```






```
<hr />
<hr />
```




<a id="example-99"></a>

[Example 99](005-4-3-setext-headings.md#example-99)Try It



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




<a id="example-100"></a>

[Example 100](005-4-3-setext-headings.md#example-100)Try It



```
    foo
---
```






```
<pre><code>foo
</code></pre>
<hr />
```




<a id="example-101"></a>

[Example 101](005-4-3-setext-headings.md#example-101)Try It



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

<a id="example-102"></a>

[Example 102](005-4-3-setext-headings.md#example-102)Try It



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

<a id="example-103"></a>

[Example 103](005-4-3-setext-headings.md#example-103)Try It



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

<a id="example-104"></a>

[Example 104](005-4-3-setext-headings.md#example-104)Try It



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
underline](005-4-3-setext-headings.md#setext-heading-underline), such as

<a id="example-105"></a>

[Example 105](005-4-3-setext-headings.md#example-105)Try It



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

<a id="example-106"></a>

[Example 106](005-4-3-setext-headings.md#example-106)Try It



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




