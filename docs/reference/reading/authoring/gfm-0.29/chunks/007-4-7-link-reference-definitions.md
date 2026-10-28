# 4.7 Link reference definitions

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-4-6-html-blocks.md) · [Next](008-4-8-paragraphs.md)

<a id="example-143"></a>

[Example 143](007-4-7-link-reference-definitions.md#example-143)



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




<a id="example-144"></a>

[Example 144](007-4-7-link-reference-definitions.md#example-144)



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

<a id="example-145"></a>

[Example 145](007-4-7-link-reference-definitions.md#example-145)



```
<style>p{color:red;}</style>
*foo*
```






```
<style>p{color:red;}</style>
<p><em>foo</em></p>
```




<a id="example-146"></a>

[Example 146](007-4-7-link-reference-definitions.md#example-146)



```
<!-- foo -->*bar*
*baz*
```






```
<!-- foo -->*bar*
<p><em>baz</em></p>
```




Note that anything on the last line after the
end tag will be included in the [HTML block](006-4-6-html-blocks.md#html-block):

<a id="example-147"></a>

[Example 147](007-4-7-link-reference-definitions.md#example-147)



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

<a id="example-148"></a>

[Example 148](007-4-7-link-reference-definitions.md#example-148)



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

<a id="example-149"></a>

[Example 149](007-4-7-link-reference-definitions.md#example-149)



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

<a id="example-150"></a>

[Example 150](007-4-7-link-reference-definitions.md#example-150)



```
<!DOCTYPE html>
```






```
<!DOCTYPE html>
```




CDATA (type 5):

<a id="example-151"></a>

[Example 151](007-4-7-link-reference-definitions.md#example-151)



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




The opening tag can be indented 1-3 spaces, but not 4:

<a id="example-152"></a>

[Example 152](007-4-7-link-reference-definitions.md#example-152)



```
  <!-- foo -->

    <!-- foo -->
```






```
  <!-- foo -->
<pre><code>&lt;!-- foo --&gt;
</code></pre>
```




<a id="example-153"></a>

[Example 153](007-4-7-link-reference-definitions.md#example-153)



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

<a id="example-154"></a>

[Example 154](007-4-7-link-reference-definitions.md#example-154)



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
a document, and except for blocks of types 1–5, [above](006-4-6-html-blocks.md#html-block):

<a id="example-155"></a>

[Example 155](007-4-7-link-reference-definitions.md#example-155)



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

<a id="example-156"></a>

[Example 156](007-4-7-link-reference-definitions.md#example-156)



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
> block should not be indented with tabs or spaces.

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

<a id="example-157"></a>

[Example 157](007-4-7-link-reference-definitions.md#example-157)



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




<a id="example-158"></a>

[Example 158](007-4-7-link-reference-definitions.md#example-158)



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

<a id="example-159"></a>

[Example 159](007-4-7-link-reference-definitions.md#example-159)



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

<a id="example-160"></a>

[Example 160](007-4-7-link-reference-definitions.md#example-160)



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
[above](006-4-6-html-blocks.md#html-blocks), raw HTML blocks starting with `<pre>`
*can* contain blank lines.

<a id="link-reference-definitions"></a>

## 4.7 Link reference definitions

<a id="link-reference-definition"></a>

A [link reference definition](007-4-7-link-reference-definitions.md#link-reference-definition)
consists of a [link label](018-continuation-18.md#link-label), indented up to three spaces, followed
by a colon (`:`), optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) (including up to one
[line ending](002-1-3-why-is-a-spec-needed.md#line-ending)), a [link destination](017-6-5-strikethrough-extension.md#link-destination),
optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) (including up to one
[line ending](002-1-3-why-is-a-spec-needed.md#line-ending)), and an optional [link
title](017-6-5-strikethrough-extension.md#link-title), which if it is present must be separated
from the [link destination](017-6-5-strikethrough-extension.md#link-destination) by [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace).
No further [non-whitespace characters](002-1-3-why-is-a-spec-needed.md#non-whitespace-character) may occur on the line.

A [link reference definition](007-4-7-link-reference-definitions.md#link-reference-definition)
does not correspond to a structural element of a document. Instead, it
defines a label which can be used in [reference links](018-continuation-18.md#reference-link)
and reference-style [images](019-6-7-images.md#images) elsewhere in the document. [Link
reference definitions](007-4-7-link-reference-definitions.md#link-reference-definitions) can come either before or after the links that use
them.

<a id="example-161"></a>

[Example 161](007-4-7-link-reference-definitions.md#example-161)



```
[foo]: /url "title"

[foo]
```






```
<p><a href="/url" title="title">foo</a></p>
```




<a id="example-162"></a>

[Example 162](007-4-7-link-reference-definitions.md#example-162)



```
   [foo]: 
      /url  
           'the title'  

[foo]
```






```
<p><a href="/url" title="the title">foo</a></p>
```




<a id="example-163"></a>

[Example 163](007-4-7-link-reference-definitions.md#example-163)



```
[Foo*bar\]]:my_(url) 'title (with parens)'

[Foo*bar\]]
```






```
<p><a href="my_(url)" title="title (with parens)">Foo*bar]</a></p>
```




<a id="example-164"></a>

[Example 164](007-4-7-link-reference-definitions.md#example-164)



```
[Foo bar]:
<my url>
'title'

[Foo bar]
```






```
<p><a href="my%20url" title="title">Foo bar</a></p>
```




The title may extend over multiple lines:

<a id="example-165"></a>

[Example 165](007-4-7-link-reference-definitions.md#example-165)



```
[foo]: /url '
title
line1
line2
'

[foo]
```






```
<p><a href="/url" title="
title
line1
line2
">foo</a></p>
```




However, it may not contain a [blank line](002-1-3-why-is-a-spec-needed.md#blank-line):

<a id="example-166"></a>

[Example 166](007-4-7-link-reference-definitions.md#example-166)



```
[foo]: /url 'title

with blank line'

[foo]
```






```
<p>[foo]: /url 'title</p>
<p>with blank line'</p>
<p>[foo]</p>
```




The title may be omitted:

<a id="example-167"></a>

[Example 167](007-4-7-link-reference-definitions.md#example-167)



```
[foo]:
/url

[foo]
```






```
<p><a href="/url">foo</a></p>
```




The link destination may not be omitted:

<a id="example-168"></a>

[Example 168](007-4-7-link-reference-definitions.md#example-168)



```
[foo]:

[foo]
```






```
<p>[foo]:</p>
<p>[foo]</p>
```




However, an empty link destination may be specified using
angle brackets:

<a id="example-169"></a>

[Example 169](007-4-7-link-reference-definitions.md#example-169)



```
[foo]: <>

[foo]
```






```
<p><a href="">foo</a></p>
```




The title must be separated from the link destination by
whitespace:

<a id="example-170"></a>

[Example 170](007-4-7-link-reference-definitions.md#example-170)



```
[foo]: <bar>(baz)

[foo]
```






```
<p>[foo]: <bar>(baz)</p>
<p>[foo]</p>
```




Both title and destination can contain backslash escapes
and literal backslashes:

<a id="example-171"></a>

[Example 171](007-4-7-link-reference-definitions.md#example-171)



```
[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]
```






```
<p><a href="/url%5Cbar*baz" title="foo&quot;bar\baz">foo</a></p>
```




A link can come before its corresponding definition:

<a id="example-172"></a>

[Example 172](007-4-7-link-reference-definitions.md#example-172)



```
[foo]

[foo]: url
```






```
<p><a href="url">foo</a></p>
```




If there are several matching definitions, the first one takes
precedence:

<a id="example-173"></a>

[Example 173](007-4-7-link-reference-definitions.md#example-173)



```
[foo]

[foo]: first
[foo]: second
```






```
<p><a href="first">foo</a></p>
```




As noted in the section on [Links](017-6-5-strikethrough-extension.md#links), matching of labels is
case-insensitive (see [matches](018-continuation-18.md#matches)).

<a id="example-174"></a>

[Example 174](007-4-7-link-reference-definitions.md#example-174)



```
[FOO]: /url

[Foo]
```






```
<p><a href="/url">Foo</a></p>
```




<a id="example-175"></a>

[Example 175](007-4-7-link-reference-definitions.md#example-175)



```
[ΑΓΩ]: /φου

[αγω]
```






```
<p><a href="/%CF%86%CE%BF%CF%85">αγω</a></p>
```




Here is a link reference definition with no corresponding link.
It contributes nothing to the document.

<a id="example-176"></a>

[Example 176](007-4-7-link-reference-definitions.md#example-176)



```
[foo]: /url
```






```

```




Here is another one:

<a id="example-177"></a>

[Example 177](007-4-7-link-reference-definitions.md#example-177)



```
[
foo
]: /url
bar
```






```
<p>bar</p>
```




This is not a link reference definition, because there are
[non-whitespace characters](002-1-3-why-is-a-spec-needed.md#non-whitespace-character) after the title:

<a id="example-178"></a>

[Example 178](007-4-7-link-reference-definitions.md#example-178)



```
[foo]: /url "title" ok
```






```
<p>[foo]: /url &quot;title&quot; ok</p>
```




This is a link reference definition, but it has no title:

<a id="example-179"></a>

[Example 179](007-4-7-link-reference-definitions.md#example-179)



```
[foo]: /url
"title" ok
```






```
<p>&quot;title&quot; ok</p>
```




This is not a link reference definition, because it is indented
four spaces:

<a id="example-180"></a>

[Example 180](007-4-7-link-reference-definitions.md#example-180)



```
    [foo]: /url "title"

[foo]
```






```
<pre><code>[foo]: /url &quot;title&quot;
</code></pre>
<p>[foo]</p>
```




This is not a link reference definition, because it occurs inside
a code block:

<a id="example-181"></a>

[Example 181](007-4-7-link-reference-definitions.md#example-181)



````
```
[foo]: /url
```

[foo]
````






```
<pre><code>[foo]: /url
</code></pre>
<p>[foo]</p>
```




A [link reference definition](007-4-7-link-reference-definitions.md#link-reference-definition) cannot interrupt a paragraph.

<a id="example-182"></a>

[Example 182](007-4-7-link-reference-definitions.md#example-182)



```
Foo
[bar]: /baz

[bar]
```






```
<p>Foo
[bar]: /baz</p>
<p>[bar]</p>
```




However, it can directly follow other block elements, such as headings
and thematic breaks, and it need not be followed by a blank line.

<a id="example-183"></a>

[Example 183](007-4-7-link-reference-definitions.md#example-183)



