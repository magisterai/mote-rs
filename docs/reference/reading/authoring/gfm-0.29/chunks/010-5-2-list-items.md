# 5.2 List items

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-5-container-blocks.md) · [Next](011-5-2-1-motivation.md)

<a id="list-items"></a>

## 5.2 List items

<a id="list-marker"></a>

A [list marker](010-5-2-list-items.md#list-marker) is a
[bullet list marker](010-5-2-list-items.md#bullet-list-marker) or an [ordered list marker](010-5-2-list-items.md#ordered-list-marker).

<a id="bullet-list-marker"></a>

A [bullet list marker](010-5-2-list-items.md#bullet-list-marker)
is a `-`, `+`, or `*` character.

<a id="ordered-list-marker"></a>

An [ordered list marker](010-5-2-list-items.md#ordered-list-marker)
is a sequence of 1–9 arabic digits (`0-9`), followed by either a
`.` character or a `)` character. (The reason for the length
limit is that with 10 digits we start seeing integer overflows
in some browsers.)

The following rules define [list items](010-5-2-list-items.md#list-items):

1. **Basic case.** If a sequence of lines *Ls* constitute a sequence of
   blocks *Bs* starting with a [non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character), and *M* is a
   list marker of width *W* followed by 1 ≤ *N* ≤ 4 spaces, then the result
   of prepending *M* and the following spaces to the first line of
   *Ls*, and indenting subsequent lines of *Ls* by *W + N* spaces, is a
   list item with *Bs* as its contents. The type of the list item
   (bullet or ordered) is determined by the type of its list marker.
   If the list item is ordered, then it is also assigned a start
   number, based on the ordered list marker.

   Exceptions:

   1. When the first list item in a [list](012-5-3-task-list-items-extension.md#list) interrupts
      a paragraph—that is, when it starts on a line that would
      otherwise count as [paragraph continuation text](009-5-container-blocks.md#paragraph-continuation-text)—then (a)
      the lines *Ls* must not begin with a blank line, and (b) if
      the list item is ordered, the start number must be 1.
   2. If any line is a [thematic break](003-2-2-tabs.md#thematic-break) then
      that line is not a list item.

For example, let *Ls* be the lines

<a id="example-231"></a>

[Example 231](010-5-2-list-items.md#example-231)



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

<a id="example-232"></a>

[Example 232](010-5-2-list-items.md#example-232)



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
marker takes up two spaces, and there are three spaces between
the list marker and the next [non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character), then blocks
must be indented five spaces in order to fall under the list
item.

Here are some examples showing how far content must be indented to be
put under the list item:

<a id="example-233"></a>

[Example 233](010-5-2-list-items.md#example-233)



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




<a id="example-234"></a>

[Example 234](010-5-2-list-items.md#example-234)



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




<a id="example-235"></a>

[Example 235](010-5-2-list-items.md#example-235)



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




<a id="example-236"></a>

[Example 236](010-5-2-list-items.md#example-236)



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
blocks must be indented at least to the column of the first
[non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character) after the list marker. However, that is not quite right.
The spaces after the list marker determine how much relative indentation
is needed. Which column this indentation reaches will depend on
how the list item is embedded in other constructions, as shown by
this example:

<a id="example-237"></a>

[Example 237](010-5-2-list-items.md#example-237)



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

<a id="example-238"></a>

[Example 238](010-5-2-list-items.md#example-238)



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




Note that at least one space is needed between the list marker and
any following content, so these are not list items:

<a id="example-239"></a>

[Example 239](010-5-2-list-items.md#example-239)



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

<a id="example-240"></a>

[Example 240](010-5-2-list-items.md#example-240)



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

<a id="example-241"></a>

[Example 241](010-5-2-list-items.md#example-241)



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

<a id="example-242"></a>

[Example 242](010-5-2-list-items.md#example-242)



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

<a id="example-243"></a>

[Example 243](010-5-2-list-items.md#example-243)



```
123456789. ok
```






```
<ol start="123456789">
<li>ok</li>
</ol>
```




<a id="example-244"></a>

[Example 244](010-5-2-list-items.md#example-244)



```
1234567890. not ok
```






```
<p>1234567890. not ok</p>
```




A start number may begin with 0s:

<a id="example-245"></a>

[Example 245](010-5-2-list-items.md#example-245)



```
0. ok
```






```
<ol start="0">
<li>ok</li>
</ol>
```




<a id="example-246"></a>

[Example 246](010-5-2-list-items.md#example-246)



```
003. ok
```






```
<ol start="3">
<li>ok</li>
</ol>
```




A start number may not be negative:

<a id="example-247"></a>

[Example 247](010-5-2-list-items.md#example-247)



```
-1. not ok
```






```
<p>-1. not ok</p>
```




2. **Item starting with indented code.** If a sequence of lines *Ls*
   constitute a sequence of blocks *Bs* starting with an indented code
   block, and *M* is a list marker of width *W* followed by
   one space, then the result of prepending *M* and the following
   space to the first line of *Ls*, and indenting subsequent lines of
   *Ls* by *W + 1* spaces, is a list item with *Bs* as its contents.
   If a line is empty, then it need not be indented. The type of the
   list item (bullet or ordered) is determined by the type of its list
   marker. If the list item is ordered, then it is also assigned a
   start number, based on the ordered list marker.

An indented code block will have to be indented four spaces beyond
the edge of the region where text will be included in the list item.
In the following case that is 6 spaces:

<a id="example-248"></a>

[Example 248](010-5-2-list-items.md#example-248)



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

<a id="example-249"></a>

[Example 249](010-5-2-list-items.md#example-249)



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
then by rule #2, the contents must be indented *one* space after the
list marker:

<a id="example-250"></a>

[Example 250](010-5-2-list-items.md#example-250)



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




<a id="example-251"></a>

[Example 251](010-5-2-list-items.md#example-251)



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




Note that an additional space indent is interpreted as space
inside the code block:

<a id="example-252"></a>

[Example 252](010-5-2-list-items.md#example-252)



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
[non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character), and (b) cases in which
they begin with an indented code
block. In a case like the following, where the first block begins with
a three-space indent, the rules do not allow us to form a list item by
indenting the whole thing and prepending a list marker:

<a id="example-253"></a>

[Example 253](010-5-2-list-items.md#example-253)



```
   foo

bar
```






```
<p>foo</p>
<p>bar</p>
```




<a id="example-254"></a>

[Example 254](010-5-2-list-items.md#example-254)



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




This is not a significant restriction, because when a block begins
with 1-3 spaces indent, the indentation can always be removed without
a change in interpretation, allowing rule #1 to be applied. So, in
the above case:

<a id="example-255"></a>

[Example 255](010-5-2-list-items.md#example-255)



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
   starting with a single [blank line](002-1-3-why-is-a-spec-needed.md#blank-line) constitute a (possibly empty)
   sequence of blocks *Bs*, not separated from each other by more than
   one blank line, and *M* is a list marker of width *W*,
   then the result of prepending *M* to the first line of *Ls*, and
   indenting subsequent lines of *Ls* by *W + 1* spaces, is a list
   item with *Bs* as its contents.
   If a line is empty, then it need not be indented. The type of the
   list item (bullet or ordered) is determined by the type of its list
   marker. If the list item is ordered, then it is also assigned a
   start number, based on the ordered list marker.

Here are some list items that start with a blank line but are not empty:

<a id="example-256"></a>

[Example 256](010-5-2-list-items.md#example-256)



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

<a id="example-257"></a>

[Example 257](010-5-2-list-items.md#example-257)



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

<a id="example-258"></a>

[Example 258](010-5-2-list-items.md#example-258)



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

<a id="example-259"></a>

[Example 259](010-5-2-list-items.md#example-259)



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




It does not matter whether there are spaces following the [list marker](010-5-2-list-items.md#list-marker):

<a id="example-260"></a>

[Example 260](010-5-2-list-items.md#example-260)



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

<a id="example-261"></a>

[Example 261](010-5-2-list-items.md#example-261)



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

<a id="example-262"></a>

[Example 262](010-5-2-list-items.md#example-262)



```
*
```






```
<ul>
<li></li>
</ul>
```




However, an empty list item cannot interrupt a paragraph:

<a id="example-263"></a>

[Example 263](010-5-2-list-items.md#example-263)



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




