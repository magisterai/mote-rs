# 4.8 Paragraphs

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-4-7-link-reference-definitions.md) · [Next](009-5-container-blocks.md)

```
# [Foo]
[foo]: /url
> bar
```






```
<h1><a href="/url">Foo</a></h1>
<blockquote>
<p>bar</p>
</blockquote>
```




<a id="example-184"></a>

[Example 184](008-4-8-paragraphs.md#example-184)



```
[foo]: /url
bar
===
[foo]
```






```
<h1>bar</h1>
<p><a href="/url">foo</a></p>
```




<a id="example-185"></a>

[Example 185](008-4-8-paragraphs.md#example-185)



```
[foo]: /url
===
[foo]
```






```
<p>===
<a href="/url">foo</a></p>
```




Several [link reference definitions](007-4-7-link-reference-definitions.md#link-reference-definitions)
can occur one after another, without intervening blank lines.

<a id="example-186"></a>

[Example 186](008-4-8-paragraphs.md#example-186)



```
[foo]: /foo-url "foo"
[bar]: /bar-url
  "bar"
[baz]: /baz-url

[foo],
[bar],
[baz]
```






```
<p><a href="/foo-url" title="foo">foo</a>,
<a href="/bar-url" title="bar">bar</a>,
<a href="/baz-url">baz</a></p>
```




[Link reference definitions](007-4-7-link-reference-definitions.md#link-reference-definitions) can occur
inside block containers, like lists and block quotations. They
affect the entire document, not just the container in which they
are defined:

<a id="example-187"></a>

[Example 187](008-4-8-paragraphs.md#example-187)



```
[foo]

> [foo]: /url
```






```
<p><a href="/url">foo</a></p>
<blockquote>
</blockquote>
```




Whether something is a [link reference definition](007-4-7-link-reference-definitions.md#link-reference-definition) is
independent of whether the link reference it defines is
used in the document. Thus, for example, the following
document contains just a link reference definition, and
no visible content:

<a id="example-188"></a>

[Example 188](008-4-8-paragraphs.md#example-188)



```
[foo]: /url
```






```

```




<a id="paragraphs"></a>

## 4.8 Paragraphs

<a id="paragraph"></a>

A sequence of non-blank lines that cannot be interpreted as other
kinds of blocks forms a [paragraph](008-4-8-paragraphs.md#paragraph).
The contents of the paragraph are the result of parsing the
paragraph’s raw content as inlines. The paragraph’s raw content
is formed by concatenating the lines and removing initial and final
[whitespace](002-1-3-why-is-a-spec-needed.md#whitespace).

A simple example with two paragraphs:

<a id="example-189"></a>

[Example 189](008-4-8-paragraphs.md#example-189)



```
aaa

bbb
```






```
<p>aaa</p>
<p>bbb</p>
```




Paragraphs can contain multiple lines, but no blank lines:

<a id="example-190"></a>

[Example 190](008-4-8-paragraphs.md#example-190)



```
aaa
bbb

ccc
ddd
```






```
<p>aaa
bbb</p>
<p>ccc
ddd</p>
```




Multiple blank lines between paragraph have no effect:

<a id="example-191"></a>

[Example 191](008-4-8-paragraphs.md#example-191)



```
aaa


bbb
```






```
<p>aaa</p>
<p>bbb</p>
```




Leading spaces are skipped:

<a id="example-192"></a>

[Example 192](008-4-8-paragraphs.md#example-192)



```
  aaa
 bbb
```






```
<p>aaa
bbb</p>
```




Lines after the first may be indented any amount, since indented
code blocks cannot interrupt paragraphs.

<a id="example-193"></a>

[Example 193](008-4-8-paragraphs.md#example-193)



```
aaa
             bbb
                                       ccc
```






```
<p>aaa
bbb
ccc</p>
```




However, the first line may be indented at most three spaces,
or an indented code block will be triggered:

<a id="example-194"></a>

[Example 194](008-4-8-paragraphs.md#example-194)



```
   aaa
bbb
```






```
<p>aaa
bbb</p>
```




<a id="example-195"></a>

[Example 195](008-4-8-paragraphs.md#example-195)



```
    aaa
bbb
```






```
<pre><code>aaa
</code></pre>
<p>bbb</p>
```




Final spaces are stripped before inline parsing, so a paragraph
that ends with two or more spaces will not end with a [hard line
break](022-6-11-disallowed-raw-html-extension.md#hard-line-break):

<a id="example-196"></a>

[Example 196](008-4-8-paragraphs.md#example-196)



```
aaa     
bbb     
```






```
<p>aaa<br />
bbb</p>
```




<a id="blank-lines"></a>

## 4.9 Blank lines

[Blank lines](002-1-3-why-is-a-spec-needed.md#blank-line) between block-level elements are ignored,
except for the role they play in determining whether a [list](012-5-3-task-list-items-extension.md#list)
is [tight](012-5-3-task-list-items-extension.md#tight) or [loose](012-5-3-task-list-items-extension.md#loose).

Blank lines at the beginning and end of the document are also ignored.

<a id="example-197"></a>

[Example 197](008-4-8-paragraphs.md#example-197)



```
  

aaa
  

# aaa

  
```






```
<p>aaa</p>
<h1>aaa</h1>
```




<a id="tables-extension-"></a>

## 4.10 Tables (extension)

GFM enables the `table` extension, where an additional leaf block type is
available.

<a id="table"></a>

A [table](008-4-8-paragraphs.md#table) is an arrangement of data with rows and columns, consisting of a
single header row, a [delimiter row](008-4-8-paragraphs.md#delimiter-row) separating the header from the data, and
zero or more data rows.

Each row consists of cells containing arbitrary text, in which [inlines](013-6-inlines.md#inlines) are
parsed, separated by pipes (`|`). A leading and trailing pipe is also
recommended for clarity of reading, and if there’s otherwise parsing ambiguity.
Spaces between pipes and cell content are trimmed. Block-level elements cannot
be inserted in a table.

<a id="delimiter-row"></a>

The [delimiter row](008-4-8-paragraphs.md#delimiter-row) consists of cells whose only content are hyphens (`-`),
and optionally, a leading or trailing colon (`:`), or both, to indicate left,
right, or center alignment respectively.

<a id="example-198"></a>

[Example 198](008-4-8-paragraphs.md#example-198)



```
| foo | bar |
| --- | --- |
| baz | bim |
```






```
<table>
<thead>
<tr>
<th>foo</th>
<th>bar</th>
</tr>
</thead>
<tbody>
<tr>
<td>baz</td>
<td>bim</td>
</tr>
</tbody>
</table>
```




Cells in one column don’t need to match length, though it’s easier to read if
they are. Likewise, use of leading and trailing pipes may be inconsistent:

<a id="example-199"></a>

[Example 199](008-4-8-paragraphs.md#example-199)



```
| abc | defghi |
:-: | -----------:
bar | baz
```






```
<table>
<thead>
<tr>
<th align="center">abc</th>
<th align="right">defghi</th>
</tr>
</thead>
<tbody>
<tr>
<td align="center">bar</td>
<td align="right">baz</td>
</tr>
</tbody>
</table>
```




Include a pipe in a cell’s content by escaping it, including inside other
inline spans:

<a id="example-200"></a>

[Example 200](008-4-8-paragraphs.md#example-200)



```
| f\|oo  |
| ------ |
| b `\|` az |
| b **\|** im |
```






```
<table>
<thead>
<tr>
<th>f|oo</th>
</tr>
</thead>
<tbody>
<tr>
<td>b <code>|</code> az</td>
</tr>
<tr>
<td>b <strong>|</strong> im</td>
</tr>
</tbody>
</table>
```




The table is broken at the first empty line, or beginning of another
block-level structure:

<a id="example-201"></a>

[Example 201](008-4-8-paragraphs.md#example-201)



```
| abc | def |
| --- | --- |
| bar | baz |
> bar
```






```
<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
</tbody>
</table>
<blockquote>
<p>bar</p>
</blockquote>
```




<a id="example-202"></a>

[Example 202](008-4-8-paragraphs.md#example-202)



```
| abc | def |
| --- | --- |
| bar | baz |
bar

bar
```






```
<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
<tr>
<td>bar</td>
<td></td>
</tr>
</tbody>
</table>
<p>bar</p>
```




The header row must match the [delimiter row](008-4-8-paragraphs.md#delimiter-row) in the number of cells. If not,
a table will not be recognized:

<a id="example-203"></a>

[Example 203](008-4-8-paragraphs.md#example-203)



```
| abc | def |
| --- |
| bar |
```






```
<p>| abc | def |
| --- |
| bar |</p>
```




The remainder of the table’s rows may vary in the number of cells. If there
are a number of cells fewer than the number of cells in the header row, empty
cells are inserted. If there are greater, the excess is ignored:

<a id="example-204"></a>

[Example 204](008-4-8-paragraphs.md#example-204)



```
| abc | def |
| --- | --- |
| bar |
| bar | baz | boo |
```






```
<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
<tbody>
<tr>
<td>bar</td>
<td></td>
</tr>
<tr>
<td>bar</td>
<td>baz</td>
</tr>
</tbody>
</table>
```




If there are no rows in the body, no `<tbody>` is generated in HTML output:

<a id="example-205"></a>

[Example 205](008-4-8-paragraphs.md#example-205)



```
| abc | def |
| --- | --- |
```






```
<table>
<thead>
<tr>
<th>abc</th>
<th>def</th>
</tr>
</thead>
</table>
```




