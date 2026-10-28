# 5.2 List items

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-4-8-paragraphs.md) · [Next](012-continuation-12.md)

It is a consequence of the Laziness rule that any number
of initial `>`s may be omitted on a continuation line of a
nested block quote:

<a id="example-250"></a>

[Example 250](011-5-2-list-items.md#example-250)Try It



```
> > > foo
bar
```






```
<blockquote>
<blockquote>
<blockquote>
<p>foo
bar</p>
</blockquote>
</blockquote>
</blockquote>
```




<a id="example-251"></a>

[Example 251](011-5-2-list-items.md#example-251)Try It



```
>>> foo
> bar
>>baz
```






```
<blockquote>
<blockquote>
<blockquote>
<p>foo
bar
baz</p>
</blockquote>
</blockquote>
</blockquote>
```




When including an indented code block in a block quote,
remember that the [block quote marker](010-4-8-paragraphs.md#block-quote-marker) includes
both the `>` and a following space of indentation. So *five spaces* are needed
after the `>`:

<a id="example-252"></a>

[Example 252](011-5-2-list-items.md#example-252)Try It



```
>     code

>    not code
```






```
<blockquote>
<pre><code>code
</code></pre>
</blockquote>
<blockquote>
<p>not code</p>
</blockquote>
```




<a id="list-items"></a>

## 5.2 List items

<a id="list-marker"></a>

A [list marker](011-5-2-list-items.md#list-marker) is a
[bullet list marker](011-5-2-list-items.md#bullet-list-marker) or an [ordered list marker](011-5-2-list-items.md#ordered-list-marker).

<a id="bullet-list-marker"></a>

A [bullet list marker](011-5-2-list-items.md#bullet-list-marker)
is a `-`, `+`, or `*` character.

<a id="ordered-list-marker"></a>

An [ordered list marker](011-5-2-list-items.md#ordered-list-marker)
is a sequence of 1–9 arabic digits (`0-9`), followed by either a
`.` character or a `)` character. (The reason for the length
limit is that with 10 digits we start seeing integer overflows
in some browsers.)

The following rules define [list items](011-5-2-list-items.md#list-items):

1. **Basic case.** If a sequence of lines *Ls* constitute a sequence of
   blocks *Bs* starting with a character other than a space or tab, and *M* is
   a list marker of width *W* followed by 1 ≤ *N* ≤ 4 spaces of indentation,
   then the result of prepending *M* and the following spaces to the first line
   of *Ls*, and indenting subsequent lines of *Ls* by *W + N* spaces, is a
   list item with *Bs* as its contents. The type of the list item
   (bullet or ordered) is determined by the type of its list marker.
   If the list item is ordered, then it is also assigned a start
   number, based on the ordered list marker.

   Exceptions:

   1. When the first list item in a [list](013-5-2-1-motivation.md#list) interrupts
      a paragraph—that is, when it starts on a line that would
      otherwise count as [paragraph continuation text](010-4-8-paragraphs.md#paragraph-continuation-text)—then (a)
      the lines *Ls* must not begin with a blank line, and (b) if
      the list item is ordered, the start number must be 1.
   2. If any line is a [thematic break](004-3-blocks-and-inlines.md#thematic-breaks) then
      that line is not a list item.

For example, let *Ls* be the lines

<a id="example-253"></a>

[Example 253](011-5-2-list-items.md#example-253)Try It



```
A paragraph
with two lines.

    indented code

> A block quote.
```






```
<p>A paragraph
with two lines.</p>
<pre><code>indented code
</code></pre>
<blockquote>
<p>A block quote.</p>
</blockquote>
```




And let *M* be the marker `1.`, and *N* = 2. Then rule #1 says
that the following is an ordered list item with start number 1,
and the same contents as *Ls*:

<a id="example-254"></a>

[Example 254](011-5-2-list-items.md#example-254)Try It



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




The most important thing to notice is that the position of
the text after the list marker determines how much indentation
is needed in subsequent blocks in the list item. If the list
marker takes up two spaces of indentation, and there are three spaces between
the list marker and the next character other than a space or tab, then blocks
must be indented five spaces in order to fall under the list
item.

Here are some examples showing how far content must be indented to be
put under the list item:

<a id="example-255"></a>

[Example 255](011-5-2-list-items.md#example-255)Try It



```
- one

 two
```






```
<ul>
<li>one</li>
</ul>
<p>two</p>
```




<a id="example-256"></a>

[Example 256](011-5-2-list-items.md#example-256)Try It



```
- one

  two
```






```
<ul>
<li>
<p>one</p>
<p>two</p>
</li>
</ul>
```




<a id="example-257"></a>

[Example 257](011-5-2-list-items.md#example-257)Try It



```
 -    one

     two
```






```
<ul>
<li>one</li>
</ul>
<pre><code> two
</code></pre>
```




<a id="example-258"></a>

[Example 258](011-5-2-list-items.md#example-258)Try It



```
 -    one

      two
```






```
<ul>
<li>
<p>one</p>
<p>two</p>
</li>
</ul>
```




It is tempting to think of this in terms of columns: the continuation
blocks must be indented at least to the column of the first character other than
a space or tab after the list marker. However, that is not quite right.
The spaces of indentation after the list marker determine how much relative
indentation is needed. Which column this indentation reaches will depend on
how the list item is embedded in other constructions, as shown by
this example:

<a id="example-259"></a>

[Example 259](011-5-2-list-items.md#example-259)Try It



```
   > > 1.  one
>>
>>     two
```






```
<blockquote>
<blockquote>
<ol>
<li>
<p>one</p>
<p>two</p>
</li>
</ol>
</blockquote>
</blockquote>
```




Here `two` occurs in the same column as the list marker `1.`,
but is actually contained in the list item, because there is
sufficient indentation after the last containing blockquote marker.

The converse is also possible. In the following example, the word `two`
occurs far to the right of the initial text of the list item, `one`, but
it is not considered part of the list item, because it is not indented
far enough past the blockquote marker:

<a id="example-260"></a>

[Example 260](011-5-2-list-items.md#example-260)Try It



```
>>- one
>>
  >  > two
```






```
<blockquote>
<blockquote>
<ul>
<li>one</li>
</ul>
<p>two</p>
</blockquote>
</blockquote>
```




Note that at least one space or tab is needed between the list marker and
any following content, so these are not list items:

<a id="example-261"></a>

[Example 261](011-5-2-list-items.md#example-261)Try It



```
-one

2.two
```






```
<p>-one</p>
<p>2.two</p>
```




A list item may contain blocks that are separated by more than
one blank line.

<a id="example-262"></a>

[Example 262](011-5-2-list-items.md#example-262)Try It



```
- foo


  bar
```






```
<ul>
<li>
<p>foo</p>
<p>bar</p>
</li>
</ul>
```




A list item may contain any kind of block:

<a id="example-263"></a>

[Example 263](011-5-2-list-items.md#example-263)Try It



````
1.  foo

    ```
    bar
    ```

    baz

    > bam
````






```
<ol>
<li>
<p>foo</p>
<pre><code>bar
</code></pre>
<p>baz</p>
<blockquote>
<p>bam</p>
</blockquote>
</li>
</ol>
```




A list item that contains an indented code block will preserve
empty lines within the code block verbatim.

<a id="example-264"></a>

[Example 264](011-5-2-list-items.md#example-264)Try It



```
- Foo

      bar


      baz
```






```
<ul>
<li>
<p>Foo</p>
<pre><code>bar


baz
</code></pre>
</li>
</ul>
```




Note that ordered list start numbers must be nine digits or less:

<a id="example-265"></a>

[Example 265](011-5-2-list-items.md#example-265)Try It



```
123456789. ok
```






```
<ol start="123456789">
<li>ok</li>
</ol>
```




<a id="example-266"></a>

[Example 266](011-5-2-list-items.md#example-266)Try It



```
1234567890. not ok
```






```
<p>1234567890. not ok</p>
```




A start number may begin with 0s:

<a id="example-267"></a>

[Example 267](011-5-2-list-items.md#example-267)Try It



```
0. ok
```






```
<ol start="0">
<li>ok</li>
</ol>
```




<a id="example-268"></a>

[Example 268](011-5-2-list-items.md#example-268)Try It



```
003. ok
```






```
<ol start="3">
<li>ok</li>
</ol>
```




A start number may not be negative:

<a id="example-269"></a>

[Example 269](011-5-2-list-items.md#example-269)Try It



```
-1. not ok
```






```
<p>-1. not ok</p>
```




2. **Item starting with indented code.** If a sequence of lines *Ls*
   constitute a sequence of blocks *Bs* starting with an indented code
   block, and *M* is a list marker of width *W* followed by
   one space of indentation, then the result of prepending *M* and the
   following space to the first line of *Ls*, and indenting subsequent lines
   of *Ls* by *W + 1* spaces, is a list item with *Bs* as its contents.
   If a line is empty, then it need not be indented. The type of the
   list item (bullet or ordered) is determined by the type of its list
   marker. If the list item is ordered, then it is also assigned a
   start number, based on the ordered list marker.

An indented code block will have to be preceded by four spaces of indentation
beyond the edge of the region where text will be included in the list item.
In the following case that is 6 spaces:

<a id="example-270"></a>

[Example 270](011-5-2-list-items.md#example-270)Try It



```
- foo

      bar
```






```
<ul>
<li>
<p>foo</p>
<pre><code>bar
</code></pre>
</li>
</ul>
```




And in this case it is 11 spaces:

<a id="example-271"></a>

[Example 271](011-5-2-list-items.md#example-271)Try It



```
  10.  foo

           bar
```






```
<ol start="10">
<li>
<p>foo</p>
<pre><code>bar
</code></pre>
</li>
</ol>
```




If the *first* block in the list item is an indented code block,
then by rule #2, the contents must be preceded by *one* space of indentation
after the list marker:

<a id="example-272"></a>

[Example 272](011-5-2-list-items.md#example-272)Try It



```
    indented code

paragraph

    more code
```






```
<pre><code>indented code
</code></pre>
<p>paragraph</p>
<pre><code>more code
</code></pre>
```




<a id="example-273"></a>

[Example 273](011-5-2-list-items.md#example-273)Try It



```
1.     indented code

   paragraph

       more code
```






```
<ol>
<li>
<pre><code>indented code
</code></pre>
<p>paragraph</p>
<pre><code>more code
</code></pre>
</li>
</ol>
```




Note that an additional space of indentation is interpreted as space
inside the code block:

<a id="example-274"></a>

[Example 274](011-5-2-list-items.md#example-274)Try It



```
1.      indented code

   paragraph

       more code
```






```
<ol>
<li>
<pre><code> indented code
</code></pre>
<p>paragraph</p>
<pre><code>more code
</code></pre>
</li>
</ol>
```




Note that rules #1 and #2 only apply to two cases: (a) cases
in which the lines to be included in a list item begin with a
character other than a space or tab, and (b) cases in which
they begin with an indented code
block. In a case like the following, where the first block begins with
three spaces of indentation, the rules do not allow us to form a list item by
indenting the whole thing and prepending a list marker:

<a id="example-275"></a>

[Example 275](011-5-2-list-items.md#example-275)Try It



```
   foo

bar
```






```
<p>foo</p>
<p>bar</p>
```




<a id="example-276"></a>

[Example 276](011-5-2-list-items.md#example-276)Try It



```
-    foo

  bar
```






```
<ul>
<li>foo</li>
</ul>
<p>bar</p>
```




This is not a significant restriction, because when a block is preceded by up to
three spaces of indentation, the indentation can always be removed without
a change in interpretation, allowing rule #1 to be applied. So, in
the above case:

<a id="example-277"></a>

[Example 277](011-5-2-list-items.md#example-277)Try It



```
-  foo

   bar
```






```
<ul>
<li>
<p>foo</p>
<p>bar</p>
</li>
</ul>
```




3. **Item starting with a blank line.** If a sequence of lines *Ls*
   starting with a single [blank line](002-1-3-about-this-document.md#blank-line) constitute a (possibly empty)
   sequence of blocks *Bs*, and *M* is a list marker of width *W*,
   then the result of prepending *M* to the first line of *Ls*, and
   preceding subsequent lines of *Ls* by *W + 1* spaces of indentation, is a
   list item with *Bs* as its contents.
   If a line is empty, then it need not be indented. The type of the
   list item (bullet or ordered) is determined by the type of its list
   marker. If the list item is ordered, then it is also assigned a
   start number, based on the ordered list marker.

Here are some list items that start with a blank line but are not empty:

<a id="example-278"></a>

[Example 278](011-5-2-list-items.md#example-278)Try It



````
-
  foo
-
  ```
  bar
  ```
-
      baz
````






```
<ul>
<li>foo</li>
<li>
<pre><code>bar
</code></pre>
</li>
<li>
<pre><code>baz
</code></pre>
</li>
</ul>
```




When the list item starts with a blank line, the number of spaces
following the list marker doesn’t change the required indentation:

<a id="example-279"></a>

[Example 279](011-5-2-list-items.md#example-279)Try It



```
-   
  foo
```






```
<ul>
<li>foo</li>
</ul>
```




A list item can begin with at most one blank line.
In the following example, `foo` is not part of the list
item:

