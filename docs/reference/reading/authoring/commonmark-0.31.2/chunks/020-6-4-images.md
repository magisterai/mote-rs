# 6.4 Images

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](019-continuation-19.md) · [Next](021-6-5-autolinks.md)

The link labels are case-insensitive:

<a id="example-555"></a>

[Example 555](020-6-4-images.md#example-555)Try It



```
[Foo][]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">Foo</a></p>
```




As with full reference links, spaces, tabs, or line endings are not
allowed between the two sets of brackets:

<a id="example-556"></a>

[Example 556](020-6-4-images.md#example-556)Try It



```
[foo] 
[]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">foo</a>
[]</p>
```




<a id="shortcut-reference-link"></a>

A [shortcut reference link](020-6-4-images.md#shortcut-reference-link)
consists of a [link label](019-continuation-19.md#link-label) that [matches](019-continuation-19.md#matches) a
[link reference definition](009-4-7-link-reference-definitions.md#link-reference-definition) elsewhere in the
document and is not followed by `[]` or a link label.
The contents of the link label are parsed as inlines,
which are used as the link’s text. The link’s URI and title
are provided by the matching link reference definition.
Thus, `[foo]` is equivalent to `[foo][]`.

<a id="example-557"></a>

[Example 557](020-6-4-images.md#example-557)Try It



```
[foo]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




<a id="example-558"></a>

[Example 558](020-6-4-images.md#example-558)Try It



```
[*foo* bar]

[*foo* bar]: /url "title"
```






```
<p><a href="/url" title="title"><em>foo</em> bar</a></p>
```




<a id="example-559"></a>

[Example 559](020-6-4-images.md#example-559)Try It



```
[[*foo* bar]]

[*foo* bar]: /url "title"
```






```
<p>[<a href="/url" title="title"><em>foo</em> bar</a>]</p>
```




<a id="example-560"></a>

[Example 560](020-6-4-images.md#example-560)Try It



```
[[bar [foo]

[foo]: /url
```






```
<p>[[bar <a href="/url">foo</a></p>
```




The link labels are case-insensitive:

<a id="example-561"></a>

[Example 561](020-6-4-images.md#example-561)Try It



```
[Foo]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">Foo</a></p>
```




A space after the link text should be preserved:

<a id="example-562"></a>

[Example 562](020-6-4-images.md#example-562)Try It



```
[foo] bar

[foo]: /url
```






```
<p><a href="/url">foo</a> bar</p>
```




If you just want bracketed text, you can backslash-escape the
opening bracket to avoid links:

<a id="example-563"></a>

[Example 563](020-6-4-images.md#example-563)Try It



```
\[foo]

[foo]: /url "title"
```






```
<p>[foo]</p>
```




Note that this is a link, because a link label ends with the first
following closing bracket:

<a id="example-564"></a>

[Example 564](020-6-4-images.md#example-564)Try It



```
[foo*]: /url

*[foo*]
```






```
<p>*<a href="/url">foo*</a></p>
```




Full and collapsed references take precedence over shortcut
references:

<a id="example-565"></a>

[Example 565](020-6-4-images.md#example-565)Try It



```
[foo][bar]

[foo]: /url1
[bar]: /url2
```






```
<p><a href="/url2">foo</a></p>
```




<a id="example-566"></a>

[Example 566](020-6-4-images.md#example-566)Try It



```
[foo][]

[foo]: /url1
```






```
<p><a href="/url1">foo</a></p>
```




Inline links also take precedence:

<a id="example-567"></a>

[Example 567](020-6-4-images.md#example-567)Try It



```
[foo]()

[foo]: /url1
```






```
<p><a href="">foo</a></p>
```




<a id="example-568"></a>

[Example 568](020-6-4-images.md#example-568)Try It



```
[foo](not a link)

[foo]: /url1
```






```
<p><a href="/url1">foo</a>(not a link)</p>
```




In the following case `[bar][baz]` is parsed as a reference,
`[foo]` as normal text:

<a id="example-569"></a>

[Example 569](020-6-4-images.md#example-569)Try It



```
[foo][bar][baz]

[baz]: /url
```






```
<p>[foo]<a href="/url">bar</a></p>
```




Here, though, `[foo][bar]` is parsed as a reference, since
`[bar]` is defined:

<a id="example-570"></a>

[Example 570](020-6-4-images.md#example-570)Try It



```
[foo][bar][baz]

[baz]: /url1
[bar]: /url2
```






```
<p><a href="/url2">foo</a><a href="/url1">baz</a></p>
```




Here `[foo]` is not parsed as a shortcut reference, because it
is followed by a link label (even though `[bar]` is not defined):

<a id="example-571"></a>

[Example 571](020-6-4-images.md#example-571)Try It



```
[foo][bar][baz]

[baz]: /url1
[foo]: /url2
```






```
<p>[foo]<a href="/url1">bar</a></p>
```




<a id="images"></a>

## 6.4 Images

<a id="image-description"></a>

Syntax for images is like the syntax for links, with one
difference. Instead of [link text](018-6-3-links.md#link-text), we have an
[image description](020-6-4-images.md#image-description). The rules for this are the
same as for [link text](018-6-3-links.md#link-text), except that (a) an
image description starts with `![` rather than `[`, and
(b) an image description may contain links.
An image description has inline elements
as its contents. When an image is rendered to HTML,
this is standardly used as the image’s `alt` attribute.

<a id="example-572"></a>

[Example 572](020-6-4-images.md#example-572)Try It



```
![foo](/url "title")
```






```
<p><img src="/url" alt="foo" title="title" /></p>
```




<a id="example-573"></a>

[Example 573](020-6-4-images.md#example-573)Try It



```
![foo *bar*]

[foo *bar*]: train.jpg "train & tracks"
```






```
<p><img src="train.jpg" alt="foo bar" title="train &amp; tracks" /></p>
```




<a id="example-574"></a>

[Example 574](020-6-4-images.md#example-574)Try It



```
![foo ![bar](/url)](/url2)
```






```
<p><img src="/url2" alt="foo bar" /></p>
```




<a id="example-575"></a>

[Example 575](020-6-4-images.md#example-575)Try It



```
![foo [bar](/url)](/url2)
```






```
<p><img src="/url2" alt="foo bar" /></p>
```




Though this spec is concerned with parsing, not rendering, it is
recommended that in rendering to HTML, only the plain string content
of the [image description](020-6-4-images.md#image-description) be used. Note that in
the above example, the alt attribute’s value is `foo bar`, not `foo [bar](/url)` or `foo <a href="/url">bar</a>`. Only the plain string
content is rendered, without formatting.

<a id="example-576"></a>

[Example 576](020-6-4-images.md#example-576)Try It



```
![foo *bar*][]

[foo *bar*]: train.jpg "train & tracks"
```






```
<p><img src="train.jpg" alt="foo bar" title="train &amp; tracks" /></p>
```




<a id="example-577"></a>

[Example 577](020-6-4-images.md#example-577)Try It



```
![foo *bar*][foobar]

[FOOBAR]: train.jpg "train & tracks"
```






```
<p><img src="train.jpg" alt="foo bar" title="train &amp; tracks" /></p>
```




<a id="example-578"></a>

[Example 578](020-6-4-images.md#example-578)Try It



```
![foo](train.jpg)
```






```
<p><img src="train.jpg" alt="foo" /></p>
```




<a id="example-579"></a>

[Example 579](020-6-4-images.md#example-579)Try It



```
My ![foo bar](/path/to/train.jpg  "title"   )
```






```
<p>My <img src="/path/to/train.jpg" alt="foo bar" title="title" /></p>
```




<a id="example-580"></a>

[Example 580](020-6-4-images.md#example-580)Try It



```
![foo](<url>)
```






```
<p><img src="url" alt="foo" /></p>
```




<a id="example-581"></a>

[Example 581](020-6-4-images.md#example-581)Try It



```
![](/url)
```






```
<p><img src="/url" alt="" /></p>
```




Reference-style:

<a id="example-582"></a>

[Example 582](020-6-4-images.md#example-582)Try It



```
![foo][bar]

[bar]: /url
```






```
<p><img src="/url" alt="foo" /></p>
```




<a id="example-583"></a>

[Example 583](020-6-4-images.md#example-583)Try It



```
![foo][bar]

[BAR]: /url
```






```
<p><img src="/url" alt="foo" /></p>
```




Collapsed:

<a id="example-584"></a>

[Example 584](020-6-4-images.md#example-584)Try It



```
![foo][]

[foo]: /url "title"
```






```
<p><img src="/url" alt="foo" title="title" /></p>
```




<a id="example-585"></a>

[Example 585](020-6-4-images.md#example-585)Try It



```
![*foo* bar][]

[*foo* bar]: /url "title"
```






```
<p><img src="/url" alt="foo bar" title="title" /></p>
```




The labels are case-insensitive:

<a id="example-586"></a>

[Example 586](020-6-4-images.md#example-586)Try It



```
![Foo][]

[foo]: /url "title"
```






```
<p><img src="/url" alt="Foo" title="title" /></p>
```




As with reference links, spaces, tabs, and line endings, are not allowed
between the two sets of brackets:

<a id="example-587"></a>

[Example 587](020-6-4-images.md#example-587)Try It



```
![foo] 
[]

[foo]: /url "title"
```






```
<p><img src="/url" alt="foo" title="title" />
[]</p>
```




Shortcut:

<a id="example-588"></a>

[Example 588](020-6-4-images.md#example-588)Try It



```
![foo]

[foo]: /url "title"
```






```
<p><img src="/url" alt="foo" title="title" /></p>
```




<a id="example-589"></a>

[Example 589](020-6-4-images.md#example-589)Try It



```
![*foo* bar]

[*foo* bar]: /url "title"
```






```
<p><img src="/url" alt="foo bar" title="title" /></p>
```




Note that link labels cannot contain unescaped brackets:

<a id="example-590"></a>

[Example 590](020-6-4-images.md#example-590)Try It



```
![[foo]]

[[foo]]: /url "title"
```






```
<p>![[foo]]</p>
<p>[[foo]]: /url &quot;title&quot;</p>
```




The link labels are case-insensitive:

<a id="example-591"></a>

[Example 591](020-6-4-images.md#example-591)Try It



```
![Foo]

[foo]: /url "title"
```






```
<p><img src="/url" alt="Foo" title="title" /></p>
```




If you just want a literal `!` followed by bracketed text, you can
backslash-escape the opening `[`:

<a id="example-592"></a>

[Example 592](020-6-4-images.md#example-592)Try It



```
!\[foo]

[foo]: /url "title"
```






```
<p>![foo]</p>
```




If you want a link after a literal `!`, backslash-escape the
`!`:

<a id="example-593"></a>

[Example 593](020-6-4-images.md#example-593)Try It



```
\![foo]

[foo]: /url "title"
```






```
<p>!<a href="/url" title="title">foo</a></p>
```




