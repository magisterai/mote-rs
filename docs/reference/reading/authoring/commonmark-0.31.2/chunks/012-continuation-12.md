# Continuation 12

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-5-2-list-items.md) · [Next](013-5-2-1-motivation.md)

<a id="example-280"></a>

[Example 280](012-continuation-12.md#example-280)Try It



```
-

  foo
```






```
<ul>
<li></li>
</ul>
<p>foo</p>
```




Here is an empty bullet list item:

<a id="example-281"></a>

[Example 281](012-continuation-12.md#example-281)Try It



```
- foo
-
- bar
```






```
<ul>
<li>foo</li>
<li></li>
<li>bar</li>
</ul>
```




It does not matter whether there are spaces or tabs following the [list marker](011-5-2-list-items.md#list-marker):

<a id="example-282"></a>

[Example 282](012-continuation-12.md#example-282)Try It



```
- foo
-   
- bar
```






```
<ul>
<li>foo</li>
<li></li>
<li>bar</li>
</ul>
```




Here is an empty ordered list item:

<a id="example-283"></a>

[Example 283](012-continuation-12.md#example-283)Try It



```
1. foo
2.
3. bar
```






```
<ol>
<li>foo</li>
<li></li>
<li>bar</li>
</ol>
```




A list may start or end with an empty list item:

<a id="example-284"></a>

[Example 284](012-continuation-12.md#example-284)Try It



```
*
```






```
<ul>
<li></li>
</ul>
```




However, an empty list item cannot interrupt a paragraph:

<a id="example-285"></a>

[Example 285](012-continuation-12.md#example-285)Try It



```
foo
*

foo
1.
```






```
<p>foo
*</p>
<p>foo
1.</p>
```




4. **Indentation.** If a sequence of lines *Ls* constitutes a list item
   according to rule #1, #2, or #3, then the result of preceding each line
   of *Ls* by up to three spaces of indentation (the same for each line) also
   constitutes a list item with the same contents and attributes. If a line is
   empty, then it need not be indented.

Indented one space:

<a id="example-286"></a>

[Example 286](012-continuation-12.md#example-286)Try It



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

<a id="example-287"></a>

[Example 287](012-continuation-12.md#example-287)Try It



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

<a id="example-288"></a>

[Example 288](012-continuation-12.md#example-288)Try It



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

<a id="example-289"></a>

[Example 289](012-continuation-12.md#example-289)Try It



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
   item](011-5-2-list-items.md#list-items) with contents *Bs*, then the result of deleting
   some or all of the indentation from one or more lines in which the
   next character other than a space or tab after the indentation is
   [paragraph continuation text](010-4-8-paragraphs.md#paragraph-continuation-text) is a
   list item with the same contents and attributes. The unindented
   lines are called
   [lazy continuation line](012-continuation-12.md#lazy-continuation-line)s.

Here is an example with [lazy continuation lines](012-continuation-12.md#lazy-continuation-line):

<a id="example-290"></a>

[Example 290](012-continuation-12.md#example-290)Try It



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

<a id="example-291"></a>

[Example 291](012-continuation-12.md#example-291)Try It



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

<a id="example-292"></a>

[Example 292](012-continuation-12.md#example-292)Try It



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




<a id="example-293"></a>

[Example 293](012-continuation-12.md#example-293)Try It



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
   #1–5 counts as a [list item](011-5-2-list-items.md#list-items).

The rules for sublists follow from the general rules
[above](011-5-2-list-items.md#list-items). A sublist must be indented the same number
of spaces of indentation a paragraph would need to be in order to be included
in the list item.

So, in this case we need two spaces indent:

<a id="example-294"></a>

[Example 294](012-continuation-12.md#example-294)Try It



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

<a id="example-295"></a>

[Example 295](012-continuation-12.md#example-295)Try It



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

<a id="example-296"></a>

[Example 296](012-continuation-12.md#example-296)Try It



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

<a id="example-297"></a>

[Example 297](012-continuation-12.md#example-297)Try It



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

<a id="example-298"></a>

[Example 298](012-continuation-12.md#example-298)Try It



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




<a id="example-299"></a>

[Example 299](012-continuation-12.md#example-299)Try It



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

<a id="example-300"></a>

[Example 300](012-continuation-12.md#example-300)Try It



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




