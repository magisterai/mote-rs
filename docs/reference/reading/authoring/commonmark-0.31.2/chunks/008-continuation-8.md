# Continuation 8

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-4-6-html-blocks.md) · [Next](009-4-7-link-reference-definitions.md)

<a id="example-172"></a>

[Example 172](008-continuation-8.md#example-172)Try It



```
<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
okay
```






```
<style
  type="text/css">
h1 {color:red;}

p {color:blue;}
</style>
<p>okay</p>
```




If there is no matching end tag, the block will end at the
end of the document (or the enclosing [block quote](010-4-8-paragraphs.md#block-quotes)
or [list item](011-5-2-list-items.md#list-items)):

<a id="example-173"></a>

[Example 173](008-continuation-8.md#example-173)Try It



```
<style
  type="text/css">

foo
```






```
<style
  type="text/css">

foo
```




<a id="example-174"></a>

[Example 174](008-continuation-8.md#example-174)Try It



```
> <div>
> foo

bar
```






```
<blockquote>
<div>
foo
</blockquote>
<p>bar</p>
```




<a id="example-175"></a>

[Example 175](008-continuation-8.md#example-175)Try It



```
- <div>
- foo
```






```
<ul>
<li>
<div>
</li>
<li>foo</li>
</ul>
```




The end tag can occur on the same line as the start tag:

<a id="example-176"></a>

[Example 176](008-continuation-8.md#example-176)Try It



```
<style>p{color:red;}</style>
*foo*
```






```
<style>p{color:red;}</style>
<p><em>foo</em></p>
```




<a id="example-177"></a>

[Example 177](008-continuation-8.md#example-177)Try It



```
<!-- foo -->*bar*
*baz*
```






```
<!-- foo -->*bar*
<p><em>baz</em></p>
```




Note that anything on the last line after the
end tag will be included in the [HTML block](007-4-6-html-blocks.md#html-block):

<a id="example-178"></a>

[Example 178](008-continuation-8.md#example-178)Try It



```
<script>
foo
</script>1. *bar*
```






```
<script>
foo
</script>1. *bar*
```




A comment (type 2):

<a id="example-179"></a>

[Example 179](008-continuation-8.md#example-179)Try It



```
<!-- Foo

bar
   baz -->
okay
```






```
<!-- Foo

bar
   baz -->
<p>okay</p>
```




A processing instruction (type 3):

<a id="example-180"></a>

[Example 180](008-continuation-8.md#example-180)Try It



```
<?php

  echo '>';

?>
okay
```






```
<?php

  echo '>';

?>
<p>okay</p>
```




A declaration (type 4):

<a id="example-181"></a>

[Example 181](008-continuation-8.md#example-181)Try It



```
<!DOCTYPE html>
```






```
<!DOCTYPE html>
```




CDATA (type 5):

<a id="example-182"></a>

[Example 182](008-continuation-8.md#example-182)Try It



```
<![CDATA[
function matchwo(a,b)
{
  if (a < b && a < 0) then {
    return 1;

  } else {

    return 0;
  }
}
]]>
okay
```






```
<![CDATA[
function matchwo(a,b)
{
  if (a < b && a < 0) then {
    return 1;

  } else {

    return 0;
  }
}
]]>
<p>okay</p>
```




The opening tag can be preceded by up to three spaces of indentation, but not
four:

<a id="example-183"></a>

[Example 183](008-continuation-8.md#example-183)Try It



```
  <!-- foo -->

    <!-- foo -->
```






```
  <!-- foo -->
<pre><code>&lt;!-- foo --&gt;
</code></pre>
```




<a id="example-184"></a>

[Example 184](008-continuation-8.md#example-184)Try It



```
  <div>

    <div>
```






```
  <div>
<pre><code>&lt;div&gt;
</code></pre>
```




An HTML block of types 1–6 can interrupt a paragraph, and need not be
preceded by a blank line.

<a id="example-185"></a>

[Example 185](008-continuation-8.md#example-185)Try It



```
Foo
<div>
bar
</div>
```






```
<p>Foo</p>
<div>
bar
</div>
```




However, a following blank line is needed, except at the end of
a document, and except for blocks of types 1–5, [above](007-4-6-html-blocks.md#html-block):

<a id="example-186"></a>

[Example 186](008-continuation-8.md#example-186)Try It



```
<div>
bar
</div>
*foo*
```






```
<div>
bar
</div>
*foo*
```




HTML blocks of type 7 cannot interrupt a paragraph:

<a id="example-187"></a>

[Example 187](008-continuation-8.md#example-187)Try It



```
Foo
<a href="bar">
baz
```






```
<p>Foo
<a href="bar">
baz</p>
```




This rule differs from John Gruber’s original Markdown syntax
specification, which says:

> The only restrictions are that block-level HTML elements —
> e.g. `<div>`, `<table>`, `<pre>`, `<p>`, etc. — must be separated from
> surrounding content by blank lines, and the start and end tags of the
> block should not be indented with spaces or tabs.

In some ways Gruber’s rule is more restrictive than the one given
here:

- It requires that an HTML block be preceded by a blank line.
- It does not allow the start tag to be indented.
- It requires a matching end tag, which it also does not allow to
  be indented.

Most Markdown implementations (including some of Gruber’s own) do not
respect all of these restrictions.

There is one respect, however, in which Gruber’s rule is more liberal
than the one given here, since it allows blank lines to occur inside
an HTML block. There are two reasons for disallowing them here.
First, it removes the need to parse balanced tags, which is
expensive and can require backtracking from the end of the document
if no matching end tag is found. Second, it provides a very simple
and flexible way of including Markdown content inside HTML tags:
simply separate the Markdown from the HTML using blank lines:

Compare:

<a id="example-188"></a>

[Example 188](008-continuation-8.md#example-188)Try It



```
<div>

*Emphasized* text.

</div>
```






```
<div>
<p><em>Emphasized</em> text.</p>
</div>
```




<a id="example-189"></a>

[Example 189](008-continuation-8.md#example-189)Try It



```
<div>
*Emphasized* text.
</div>
```






```
<div>
*Emphasized* text.
</div>
```




Some Markdown implementations have adopted a convention of
interpreting content inside tags as text if the open tag has
the attribute `markdown=1`. The rule given above seems a simpler and
more elegant way of achieving the same expressive power, which is also
much simpler to parse.

The main potential drawback is that one can no longer paste HTML
blocks into Markdown documents with 100% reliability. However,
*in most cases* this will work fine, because the blank lines in
HTML are usually followed by HTML block tags. For example:

<a id="example-190"></a>

[Example 190](008-continuation-8.md#example-190)Try It



```
<table>

<tr>

<td>
Hi
</td>

</tr>

</table>
```






```
<table>
<tr>
<td>
Hi
</td>
</tr>
</table>
```




There are problems, however, if the inner tags are indented
*and* separated by spaces, as then they will be interpreted as
an indented code block:

<a id="example-191"></a>

[Example 191](008-continuation-8.md#example-191)Try It



```
<table>

  <tr>

    <td>
      Hi
    </td>

  </tr>

</table>
```






```
<table>
  <tr>
<pre><code>&lt;td&gt;
  Hi
&lt;/td&gt;
</code></pre>
  </tr>
</table>
```




Fortunately, blank lines are usually not necessary and can be
deleted. The exception is inside `<pre>` tags, but as described
[above](007-4-6-html-blocks.md#html-blocks), raw HTML blocks starting with `<pre>`
*can* contain blank lines.

