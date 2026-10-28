# 4.7 Link reference definitions

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-continuation-8.md) · [Next](010-4-8-paragraphs.md)

<a id="link-reference-definitions"></a>

## 4.7 Link reference definitions

<a id="link-reference-definition"></a>

A [link reference definition](009-4-7-link-reference-definitions.md#link-reference-definition)
consists of a [link label](019-continuation-19.md#link-label), optionally preceded by up to three spaces of
indentation, followed
by a colon (`:`), optional spaces or tabs (including up to one
[line ending](002-1-3-about-this-document.md#line-ending)), a [link destination](018-6-3-links.md#link-destination),
optional spaces or tabs (including up to one
[line ending](002-1-3-about-this-document.md#line-ending)), and an optional [link
title](018-6-3-links.md#link-title), which if it is present must be separated
from the [link destination](018-6-3-links.md#link-destination) by spaces or tabs.
No further character may occur.

A [link reference definition](009-4-7-link-reference-definitions.md#link-reference-definition)
does not correspond to a structural element of a document. Instead, it
defines a label which can be used in [reference links](019-continuation-19.md#reference-link)
and reference-style [images](020-6-4-images.md#images) elsewhere in the document. [Link
reference definitions](009-4-7-link-reference-definitions.md#link-reference-definition) can come either before or after the links that use
them.

<a id="example-192"></a>

[Example 192](009-4-7-link-reference-definitions.md#example-192)Try It



```
[foo]: /url "title"

[foo]
```






```
<p><a href="/url" title="title">foo</a></p>
```




<a id="example-193"></a>

[Example 193](009-4-7-link-reference-definitions.md#example-193)Try It



```
   [foo]: 
      /url  
           'the title'  

[foo]
```






```
<p><a href="/url" title="the title">foo</a></p>
```




<a id="example-194"></a>

[Example 194](009-4-7-link-reference-definitions.md#example-194)Try It



```
[Foo*bar\]]:my_(url) 'title (with parens)'

[Foo*bar\]]
```






```
<p><a href="my_(url)" title="title (with parens)">Foo*bar]</a></p>
```




<a id="example-195"></a>

[Example 195](009-4-7-link-reference-definitions.md#example-195)Try It



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

<a id="example-196"></a>

[Example 196](009-4-7-link-reference-definitions.md#example-196)Try It



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




However, it may not contain a [blank line](002-1-3-about-this-document.md#blank-line):

<a id="example-197"></a>

[Example 197](009-4-7-link-reference-definitions.md#example-197)Try It



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

<a id="example-198"></a>

[Example 198](009-4-7-link-reference-definitions.md#example-198)Try It



```
[foo]:
/url

[foo]
```






```
<p><a href="/url">foo</a></p>
```




The link destination may not be omitted:

<a id="example-199"></a>

[Example 199](009-4-7-link-reference-definitions.md#example-199)Try It



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

<a id="example-200"></a>

[Example 200](009-4-7-link-reference-definitions.md#example-200)Try It



```
[foo]: <>

[foo]
```






```
<p><a href="">foo</a></p>
```




The title must be separated from the link destination by
spaces or tabs:

<a id="example-201"></a>

[Example 201](009-4-7-link-reference-definitions.md#example-201)Try It



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

<a id="example-202"></a>

[Example 202](009-4-7-link-reference-definitions.md#example-202)Try It



```
[foo]: /url\bar\*baz "foo\"bar\baz"

[foo]
```






```
<p><a href="/url%5Cbar*baz" title="foo&quot;bar\baz">foo</a></p>
```




A link can come before its corresponding definition:

<a id="example-203"></a>

[Example 203](009-4-7-link-reference-definitions.md#example-203)Try It



```
[foo]

[foo]: url
```






```
<p><a href="url">foo</a></p>
```




If there are several matching definitions, the first one takes
precedence:

<a id="example-204"></a>

[Example 204](009-4-7-link-reference-definitions.md#example-204)Try It



```
[foo]

[foo]: first
[foo]: second
```






```
<p><a href="first">foo</a></p>
```




As noted in the section on [Links](018-6-3-links.md#links), matching of labels is
case-insensitive (see [matches](019-continuation-19.md#matches)).

<a id="example-205"></a>

[Example 205](009-4-7-link-reference-definitions.md#example-205)Try It



```
[FOO]: /url

[Foo]
```






```
<p><a href="/url">Foo</a></p>
```




<a id="example-206"></a>

[Example 206](009-4-7-link-reference-definitions.md#example-206)Try It



```
[ΑΓΩ]: /φου

[αγω]
```






```
<p><a href="/%CF%86%CE%BF%CF%85">αγω</a></p>
```




Whether something is a [link reference definition](009-4-7-link-reference-definitions.md#link-reference-definition) is
independent of whether the link reference it defines is
used in the document. Thus, for example, the following
document contains just a link reference definition, and
no visible content:

<a id="example-207"></a>

[Example 207](009-4-7-link-reference-definitions.md#example-207)Try It



```
[foo]: /url
```






```

```




Here is another one:

<a id="example-208"></a>

[Example 208](009-4-7-link-reference-definitions.md#example-208)Try It



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
characters other than spaces or tabs after the title:

<a id="example-209"></a>

[Example 209](009-4-7-link-reference-definitions.md#example-209)Try It



```
[foo]: /url "title" ok
```






```
<p>[foo]: /url &quot;title&quot; ok</p>
```




This is a link reference definition, but it has no title:

<a id="example-210"></a>

[Example 210](009-4-7-link-reference-definitions.md#example-210)Try It



```
[foo]: /url
"title" ok
```






```
<p>&quot;title&quot; ok</p>
```




This is not a link reference definition, because it is indented
four spaces:

<a id="example-211"></a>

[Example 211](009-4-7-link-reference-definitions.md#example-211)Try It



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

<a id="example-212"></a>

[Example 212](009-4-7-link-reference-definitions.md#example-212)Try It



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




A [link reference definition](009-4-7-link-reference-definitions.md#link-reference-definition) cannot interrupt a paragraph.

<a id="example-213"></a>

[Example 213](009-4-7-link-reference-definitions.md#example-213)Try It



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

<a id="example-214"></a>

[Example 214](009-4-7-link-reference-definitions.md#example-214)Try It



