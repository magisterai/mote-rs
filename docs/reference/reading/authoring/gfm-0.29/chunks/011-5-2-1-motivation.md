# 5.2.1 Motivation

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-5-2-list-items.md) · [Next](012-5-3-task-list-items-extension.md)

4. **Indentation.** If a sequence of lines *Ls* constitutes a list item
   according to rule #1, #2, or #3, then the result of indenting each line
   of *Ls* by 1-3 spaces (the same for each line) also constitutes a
   list item with the same contents and attributes. If a line is
   empty, then it need not be indented.

Indented one space:

<a id="example-264"></a>

[Example 264](011-5-2-1-motivation.md#example-264)



```
 1.  A paragraph
     with two lines.

         indented code

     > A block quote.
```






```
<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
```




Indented two spaces:

<a id="example-265"></a>

[Example 265](011-5-2-1-motivation.md#example-265)



```
  1.  A paragraph
      with two lines.

          indented code

      > A block quote.
```






```
<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
```




Indented three spaces:

<a id="example-266"></a>

[Example 266](011-5-2-1-motivation.md#example-266)



```
   1.  A paragraph
       with two lines.

           indented code

       > A block quote.
```






```
<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
```




Four spaces indent gives a code block:

<a id="example-267"></a>

[Example 267](011-5-2-1-motivation.md#example-267)



```
    1.  A paragraph
        with two lines.

            indented code

        > A block quote.
```






```
<pre><code>1.  A paragraph
    with two lines.

        indented code

    &gt; A block quote.
</code></pre>
```




<a id="lazy-continuation-line"></a>

5. **Laziness.** If a string of lines *Ls* constitute a [list
   item](010-5-2-list-items.md#list-items) with contents *Bs*, then the result of deleting
   some or all of the indentation from one or more lines in which the
   next [non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character) after the indentation is
   [paragraph continuation text](009-5-container-blocks.md#paragraph-continuation-text) is a
   list item with the same contents and attributes. The unindented
   lines are called
   [lazy continuation line](011-5-2-1-motivation.md#lazy-continuation-line)s.

Here is an example with [lazy continuation lines](011-5-2-1-motivation.md#lazy-continuation-line):

<a id="example-268"></a>

[Example 268](011-5-2-1-motivation.md#example-268)



```
  1.  A paragraph
with two lines.

          indented code

      > A block quote.
```






```
<ol>
<li>
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
</li>
</ol>
```




Indentation can be partially deleted:

<a id="example-269"></a>

[Example 269](011-5-2-1-motivation.md#example-269)



```
  1.  A paragraph
    with two lines.
```






```
<ol>
<li>A paragraph
with two lines.</li>
</ol>
```




These examples show how laziness can work in nested structures:

<a id="example-270"></a>

[Example 270](011-5-2-1-motivation.md#example-270)



```
> 1. > Blockquote
continued here.
```






```
<blockquote>
<ol>
<li>
<blockquote>
<p>Blockquote
continued here.</p>
</blockquote>
</li>
</ol>
</blockquote>
```




<a id="example-271"></a>

[Example 271](011-5-2-1-motivation.md#example-271)



```
> 1. > Blockquote
> continued here.
```






```
<blockquote>
<ol>
<li>
<blockquote>
<p>Blockquote
continued here.</p>
</blockquote>
</li>
</ol>
</blockquote>
```




6. **That’s all.** Nothing that is not counted as a list item by rules
   #1–5 counts as a [list item](010-5-2-list-items.md#list-items).

The rules for sublists follow from the general rules
[above](010-5-2-list-items.md#list-items). A sublist must be indented the same number
of spaces a paragraph would need to be in order to be included
in the list item.

So, in this case we need two spaces indent:

<a id="example-272"></a>

[Example 272](011-5-2-1-motivation.md#example-272)



```
- foo
  - bar
    - baz
      - boo
```






```
<ul>
<li>foo
<ul>
<li>bar
<ul>
<li>baz
<ul>
<li>boo</li>
</ul>
</li>
</ul>
</li>
</ul>
</li>
</ul>
```




One is not enough:

<a id="example-273"></a>

[Example 273](011-5-2-1-motivation.md#example-273)



```
- foo
 - bar
  - baz
   - boo
```






```
<ul>
<li>foo</li>
<li>bar</li>
<li>baz</li>
<li>boo</li>
</ul>
```




Here we need four, because the list marker is wider:

<a id="example-274"></a>

[Example 274](011-5-2-1-motivation.md#example-274)



```
10) foo
    - bar
```






```
<ol start="10">
<li>foo
<ul>
<li>bar</li>
</ul>
</li>
</ol>
```




Three is not enough:

<a id="example-275"></a>

[Example 275](011-5-2-1-motivation.md#example-275)



```
10) foo
   - bar
```






```
<ol start="10">
<li>foo</li>
</ol>
<ul>
<li>bar</li>
</ul>
```




A list may be the first block in a list item:

<a id="example-276"></a>

[Example 276](011-5-2-1-motivation.md#example-276)



```
- - foo
```






```
<ul>
<li>
<ul>
<li>foo</li>
</ul>
</li>
</ul>
```




<a id="example-277"></a>

[Example 277](011-5-2-1-motivation.md#example-277)



```
1. - 2. foo
```






```
<ol>
<li>
<ul>
<li>
<ol start="2">
<li>foo</li>
</ol>
</li>
</ul>
</li>
</ol>
```




A list item can contain a heading:

<a id="example-278"></a>

[Example 278](011-5-2-1-motivation.md#example-278)



```
- # Foo
- Bar
  ---
  baz
```






```
<ul>
<li>
<h1>Foo</h1>
</li>
<li>
<h2>Bar</h2>
baz</li>
</ul>
```




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
a rule, together with the rule allowing 1–3 spaces indentation of the
initial list marker, allows text that is indented *less than* the
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

