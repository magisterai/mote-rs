# 5.2.1 Motivation

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-continuation-12.md) · [Next](014-6-inlines.md)

<a id="motivation"></a>

### 5.2.1 Motivation

John Gruber’s Markdown spec says the following about list items:

1. “List markers typically start at the left margin, but may be indented
   by up to three spaces. List markers must be followed by one or more
   spaces or a tab.”
2. “To make lists look nice, you can wrap items with hanging indents….
   But if you don’t want to, you don’t have to.”
3. “List items may consist of multiple paragraphs. Each subsequent
   paragraph in a list item must be indented by either 4 spaces or one
   tab.”
4. “It looks nice if you indent every line of the subsequent paragraphs,
   but here again, Markdown will allow you to be lazy.”
5. “To put a blockquote within a list item, the blockquote’s `>`
   delimiters need to be indented.”
6. “To put a code block within a list item, the code block needs to be
   indented twice — 8 spaces or two tabs.”

These rules specify that a paragraph under a list item must be indented
four spaces (presumably, from the left margin, rather than the start of
the list marker, but this is not said), and that code under a list item
must be indented eight spaces instead of the usual four. They also say
that a block quote must be indented, but not by how much; however, the
example given has four spaces indentation. Although nothing is said
about other kinds of block-level content, it is certainly reasonable to
infer that *all* block elements under a list item, including other
lists, must be indented four spaces. This principle has been called the
*four-space rule*.

The four-space rule is clear and principled, and if the reference
implementation `Markdown.pl` had followed it, it probably would have
become the standard. However, `Markdown.pl` allowed paragraphs and
sublists to start with only two spaces indentation, at least on the
outer level. Worse, its behavior was inconsistent: a sublist of an
outer-level list needed two spaces indentation, but a sublist of this
sublist needed three spaces. It is not surprising, then, that different
implementations of Markdown have developed very different rules for
determining what comes under a list item. (Pandoc and python-Markdown,
for example, stuck with Gruber’s syntax description and the four-space
rule, while discount, redcarpet, marked, PHP Markdown, and others
followed `Markdown.pl`’s behavior more closely.)

Unfortunately, given the divergences between implementations, there
is no way to give a spec for list items that will be guaranteed not
to break any existing documents. However, the spec given here should
correctly handle lists formatted with either the four-space rule or
the more forgiving `Markdown.pl` behavior, provided they are laid out
in a way that is natural for a human to read.

The strategy here is to let the width and indentation of the list marker
determine the indentation necessary for blocks to fall under the list
item, rather than having a fixed and arbitrary number. The writer can
think of the body of the list item as a unit which gets indented to the
right enough to fit the list marker (and any indentation on the list
marker). (The laziness rule, #5, then allows continuation lines to be
unindented if needed.)

This rule is superior, we claim, to any rule requiring a fixed level of
indentation from the margin. The four-space rule is clear but
unnatural. It is quite unintuitive that



```
- foo

  bar

  - baz
```




should be parsed as two lists with an intervening paragraph,



```
<ul>
<li>foo</li>
</ul>
<p>bar</p>
<ul>
<li>baz</li>
</ul>
```




as the four-space rule demands, rather than a single list,



```
<ul>
<li>
<p>foo</p>
<p>bar</p>
<ul>
<li>baz</li>
</ul>
</li>
</ul>
```




The choice of four spaces is arbitrary. It can be learned, but it is
not likely to be guessed, and it trips up beginners regularly.

Would it help to adopt a two-space rule? The problem is that such
a rule, together with the rule allowing up to three spaces of indentation for
the initial list marker, allows text that is indented *less than* the
original list marker to be included in the list item. For example,
`Markdown.pl` parses



```
   - one

  two
```




as a single list item, with `two` a continuation paragraph:



```
<ul>
<li>
<p>one</p>
<p>two</p>
</li>
</ul>
```




and similarly



```
>   - one
>
>  two
```




as



```
<blockquote>
<ul>
<li>
<p>one</p>
<p>two</p>
</li>
</ul>
</blockquote>
```




This is extremely unintuitive.

Rather than requiring a fixed indent from the margin, we could require
a fixed indent (say, two spaces, or even one space) from the list marker (which
may itself be indented). This proposal would remove the last anomaly
discussed. Unlike the spec presented above, it would count the following
as a list item with a subparagraph, even though the paragraph `bar`
is not indented as far as the first paragraph `foo`:



```
 10. foo

   bar  
```




Arguably this text does read like a list item with `bar` as a subparagraph,
which may count in favor of the proposal. However, on this proposal indented
code would have to be indented six spaces after the list marker. And this
would break a lot of existing Markdown, which has the pattern:



```
1.  foo

        indented code
```




where the code is indented eight spaces. The spec above, by contrast, will
parse this text as expected, since the code block’s indentation is measured
from the beginning of `foo`.

The one case that needs special treatment is a list item that *starts*
with indented code. How much indentation is required in that case, since
we don’t have a “first paragraph” to measure from? Rule #2 simply stipulates
that in such cases, we require one space indentation from the list marker
(and then the normal four spaces for the indented code). This will match the
four-space rule in cases where the list marker plus its initial indentation
takes four spaces (a common case), but diverge in other cases.

<a id="lists"></a>

## 5.3 Lists

<a id="list"></a>

A [list](013-5-2-1-motivation.md#list) is a sequence of one or more
list items [of the same type](013-5-2-1-motivation.md#of-the-same-type). The list items
may be separated by any number of blank lines.

<a id="of-the-same-type"></a>

Two list items are [of the same type](013-5-2-1-motivation.md#of-the-same-type)
if they begin with a [list marker](011-5-2-list-items.md#list-marker) of the same type.
Two list markers are of the
same type if (a) they are bullet list markers using the same character
(`-`, `+`, or `*`) or (b) they are ordered list numbers with the same
delimiter (either `.` or `)`).

<a id="ordered-list"></a>

<a id="bullet-list"></a>

A list is an [ordered list](013-5-2-1-motivation.md#ordered-list)
if its constituent list items begin with
[ordered list markers](011-5-2-list-items.md#ordered-list-marker), and a
[bullet list](013-5-2-1-motivation.md#bullet-list) if its constituent list
items begin with [bullet list markers](011-5-2-list-items.md#bullet-list-marker).

<a id="start-number"></a>

The [start number](013-5-2-1-motivation.md#start-number)
of an [ordered list](013-5-2-1-motivation.md#ordered-list) is determined by the list number of
its initial list item. The numbers of subsequent list items are
disregarded.

<a id="loose"></a>

<a id="tight"></a>

A list is [loose](013-5-2-1-motivation.md#loose) if any of its constituent
list items are separated by blank lines, or if any of its constituent
list items directly contain two block-level elements with a blank line
between them. Otherwise a list is [tight](013-5-2-1-motivation.md#tight).
(The difference in HTML output is that paragraphs in a loose list are
wrapped in `<p>` tags, while paragraphs in a tight list are not.)

Changing the bullet or ordered list delimiter starts a new list:

<a id="example-301"></a>

[Example 301](013-5-2-1-motivation.md#example-301)Try It



```
- foo
- bar
+ baz
```






```
<ul>
<li>foo</li>
<li>bar</li>
</ul>
<ul>
<li>baz</li>
</ul>
```




<a id="example-302"></a>

[Example 302](013-5-2-1-motivation.md#example-302)Try It



```
1. foo
2. bar
3) baz
```






```
<ol>
<li>foo</li>
<li>bar</li>
</ol>
<ol start="3">
<li>baz</li>
</ol>
```




In CommonMark, a list can interrupt a paragraph. That is,
no blank line is needed to separate a paragraph from a following
list:

<a id="example-303"></a>

[Example 303](013-5-2-1-motivation.md#example-303)Try It



```
Foo
- bar
- baz
```






```
<p>Foo</p>
<ul>
<li>bar</li>
<li>baz</li>
</ul>
```




`Markdown.pl` does not allow this, through fear of triggering a list
via a numeral in a hard-wrapped line:



```
The number of windows in my house is
14.  The number of doors is 6.
```




Oddly, though, `Markdown.pl` *does* allow a blockquote to
interrupt a paragraph, even though the same considerations might
apply.

In CommonMark, we do allow lists to interrupt paragraphs, for
two reasons. First, it is natural and not uncommon for people
to start lists without blank lines:



```
I need to buy
- new shoes
- a coat
- a plane ticket
```




Second, we are attracted to a

> <a id="principle-of-uniformity"></a>
>
> [principle of uniformity](013-5-2-1-motivation.md#principle-of-uniformity):
> if a chunk of text has a certain
> meaning, it will continue to have the same meaning when put into a
> container block (such as a list item or blockquote).

(Indeed, the spec for [list items](011-5-2-list-items.md#list-items) and [block quotes](010-4-8-paragraphs.md#block-quotes) presupposes
this principle.) This principle implies that if



```
  * I need to buy
    - new shoes
    - a coat
    - a plane ticket
```




is a list item containing a paragraph followed by a nested sublist,
as all Markdown implementations agree it is (though the paragraph
may be rendered without `<p>` tags, since the list is “tight”),
then



```
I need to buy
- new shoes
- a coat
- a plane ticket
```




by itself should be a paragraph followed by a nested sublist.

Since it is well established Markdown practice to allow lists to
interrupt paragraphs inside list items, the [principle of
uniformity](013-5-2-1-motivation.md#principle-of-uniformity) requires us to allow this outside list items as
well. ([reStructuredText](https://docutils.sourceforge.net/rst.html)
takes a different approach, requiring blank lines before lists
even inside other list items.)

In order to solve the problem of unwanted lists in paragraphs with
hard-wrapped numerals, we allow only lists starting with `1` to
interrupt paragraphs. Thus,

<a id="example-304"></a>

[Example 304](013-5-2-1-motivation.md#example-304)Try It



```
The number of windows in my house is
14.  The number of doors is 6.
```






```
<p>The number of windows in my house is
14.  The number of doors is 6.</p>
```




We may still get an unintended result in cases like

<a id="example-305"></a>

[Example 305](013-5-2-1-motivation.md#example-305)Try It



```
The number of windows in my house is
1.  The number of doors is 6.
```






```
<p>The number of windows in my house is</p>
<ol>
<li>The number of doors is 6.</li>
</ol>
```




but this rule should prevent most spurious list captures.

There can be any number of blank lines between items:

<a id="example-306"></a>

[Example 306](013-5-2-1-motivation.md#example-306)Try It



```
- foo

- bar


- baz
```






```
<ul>
<li>
<p>foo</p>
</li>
<li>
<p>bar</p>
</li>
<li>
<p>baz</p>
</li>
</ul>
```




<a id="example-307"></a>

[Example 307](013-5-2-1-motivation.md#example-307)Try It



```
- foo
  - bar
    - baz


      bim
```






```
<ul>
<li>foo
<ul>
<li>bar
<ul>
<li>
<p>baz</p>
<p>bim</p>
</li>
</ul>
</li>
</ul>
</li>
</ul>
```




To separate consecutive lists of the same type, or to separate a
list from an indented code block that would otherwise be parsed
as a subparagraph of the final list item, you can insert a blank HTML
comment:

<a id="example-308"></a>

[Example 308](013-5-2-1-motivation.md#example-308)Try It



```
- foo
- bar

<!-- -->

- baz
- bim
```






```
<ul>
<li>foo</li>
<li>bar</li>
</ul>
<!-- -->
<ul>
<li>baz</li>
<li>bim</li>
</ul>
```




<a id="example-309"></a>

[Example 309](013-5-2-1-motivation.md#example-309)Try It



```
-   foo

    notcode

-   foo

<!-- -->

    code
```






```
<ul>
<li>
<p>foo</p>
<p>notcode</p>
</li>
<li>
<p>foo</p>
</li>
</ul>
<!-- -->
<pre><code>code
</code></pre>
```




List items need not be indented to the same level. The following
list items will be treated as items at the same list level,
since none is indented enough to belong to the previous list
item:

<a id="example-310"></a>

[Example 310](013-5-2-1-motivation.md#example-310)Try It



```
- a
 - b
  - c
   - d
  - e
 - f
- g
```






```
<ul>
<li>a</li>
<li>b</li>
<li>c</li>
<li>d</li>
<li>e</li>
<li>f</li>
<li>g</li>
</ul>
```




<a id="example-311"></a>

[Example 311](013-5-2-1-motivation.md#example-311)Try It



```
1. a

  2. b

   3. c
```






```
<ol>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>c</p>
</li>
</ol>
```




Note, however, that list items may not be preceded by more than
three spaces of indentation. Here `- e` is treated as a paragraph continuation
line, because it is indented more than three spaces:

<a id="example-312"></a>

[Example 312](013-5-2-1-motivation.md#example-312)Try It



```
- a
 - b
  - c
   - d
    - e
```






```
<ul>
<li>a</li>
<li>b</li>
<li>c</li>
<li>d
- e</li>
</ul>
```




And here, `3. c` is treated as in indented code block,
because it is indented four spaces and preceded by a
blank line.

<a id="example-313"></a>

[Example 313](013-5-2-1-motivation.md#example-313)Try It



```
1. a

  2. b

    3. c
```






