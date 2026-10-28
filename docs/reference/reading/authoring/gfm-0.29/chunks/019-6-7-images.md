# 6.7 Images

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-continuation-18.md) · [Next](020-6-8-autolinks.md)

```
[foo][ref\[]

[ref\[]: /uri
```






```
<p><a href="/uri">foo</a></p>
```




Note that in this example `]` is not backslash-escaped:

<a id="example-559"></a>

[Example 559](019-6-7-images.md#example-559)



```
[bar\\]: /uri

[bar\\]
```






```
<p><a href="/uri">bar\</a></p>
```




A [link label](018-continuation-18.md#link-label) must contain at least one [non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character):

<a id="example-560"></a>

[Example 560](019-6-7-images.md#example-560)



```
[]

[]: /uri
```






```
<p>[]</p>
<p>[]: /uri</p>
```




<a id="example-561"></a>

[Example 561](019-6-7-images.md#example-561)



```
[
 ]

[
 ]: /uri
```






```
<p>[
]</p>
<p>[
]: /uri</p>
```




<a id="collapsed-reference-link"></a>

A [collapsed reference link](019-6-7-images.md#collapsed-reference-link)
consists of a [link label](018-continuation-18.md#link-label) that [matches](018-continuation-18.md#matches) a
[link reference definition](007-4-7-link-reference-definitions.md#link-reference-definition) elsewhere in the
document, followed by the string `[]`.
The contents of the first link label are parsed as inlines,
which are used as the link’s text. The link’s URI and title are
provided by the matching reference link definition. Thus,
`[foo][]` is equivalent to `[foo][foo]`.

<a id="example-562"></a>

[Example 562](019-6-7-images.md#example-562)



```
[foo][]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




<a id="example-563"></a>

[Example 563](019-6-7-images.md#example-563)



```
[*foo* bar][]

[*foo* bar]: /url "title"
```






```
<p><a href="/url" title="title"><em>foo</em> bar</a></p>
```




The link labels are case-insensitive:

<a id="example-564"></a>

[Example 564](019-6-7-images.md#example-564)



```
[Foo][]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">Foo</a></p>
```




As with full reference links, [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is not
allowed between the two sets of brackets:

<a id="example-565"></a>

[Example 565](019-6-7-images.md#example-565)



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

A [shortcut reference link](019-6-7-images.md#shortcut-reference-link)
consists of a [link label](018-continuation-18.md#link-label) that [matches](018-continuation-18.md#matches) a
[link reference definition](007-4-7-link-reference-definitions.md#link-reference-definition) elsewhere in the
document and is not followed by `[]` or a link label.
The contents of the first link label are parsed as inlines,
which are used as the link’s text. The link’s URI and title
are provided by the matching link reference definition.
Thus, `[foo]` is equivalent to `[foo][]`.

<a id="example-566"></a>

[Example 566](019-6-7-images.md#example-566)



```
[foo]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




<a id="example-567"></a>

[Example 567](019-6-7-images.md#example-567)



```
[*foo* bar]

[*foo* bar]: /url "title"
```






```
<p><a href="/url" title="title"><em>foo</em> bar</a></p>
```




<a id="example-568"></a>

[Example 568](019-6-7-images.md#example-568)



```
[[*foo* bar]]

[*foo* bar]: /url "title"
```






```
<p>[<a href="/url" title="title"><em>foo</em> bar</a>]</p>
```




<a id="example-569"></a>

[Example 569](019-6-7-images.md#example-569)



```
[[bar [foo]

[foo]: /url
```






```
<p>[[bar <a href="/url">foo</a></p>
```




The link labels are case-insensitive:

<a id="example-570"></a>

[Example 570](019-6-7-images.md#example-570)



```
[Foo]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">Foo</a></p>
```




A space after the link text should be preserved:

<a id="example-571"></a>

[Example 571](019-6-7-images.md#example-571)



```
[foo] bar

[foo]: /url
```






```
<p><a href="/url">foo</a> bar</p>
```




If you just want bracketed text, you can backslash-escape the
opening bracket to avoid links:

<a id="example-572"></a>

[Example 572](019-6-7-images.md#example-572)



```
\[foo]

[foo]: /url "title"
```






```
<p>[foo]</p>
```




Note that this is a link, because a link label ends with the first
following closing bracket:

<a id="example-573"></a>

[Example 573](019-6-7-images.md#example-573)



```
[foo*]: /url

*[foo*]
```






```
<p>*<a href="/url">foo*</a></p>
```




Full and compact references take precedence over shortcut
references:

<a id="example-574"></a>

[Example 574](019-6-7-images.md#example-574)



```
[foo][bar]

[foo]: /url1
[bar]: /url2
```






```
<p><a href="/url2">foo</a></p>
```




<a id="example-575"></a>

[Example 575](019-6-7-images.md#example-575)



```
[foo][]

[foo]: /url1
```






```
<p><a href="/url1">foo</a></p>
```




Inline links also take precedence:

<a id="example-576"></a>

[Example 576](019-6-7-images.md#example-576)



```
[foo]()

[foo]: /url1
```






```
<p><a href="">foo</a></p>
```




<a id="example-577"></a>

[Example 577](019-6-7-images.md#example-577)



```
[foo](not a link)

[foo]: /url1
```






```
<p><a href="/url1">foo</a>(not a link)</p>
```




In the following case `[bar][baz]` is parsed as a reference,
`[foo]` as normal text:

<a id="example-578"></a>

[Example 578](019-6-7-images.md#example-578)



```
[foo][bar][baz]

[baz]: /url
```






```
<p>[foo]<a href="/url">bar</a></p>
```




Here, though, `[foo][bar]` is parsed as a reference, since
`[bar]` is defined:

<a id="example-579"></a>

[Example 579](019-6-7-images.md#example-579)



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

<a id="example-580"></a>

[Example 580](019-6-7-images.md#example-580)



```
[foo][bar][baz]

[baz]: /url1
[foo]: /url2
```






```
<p>[foo]<a href="/url1">bar</a></p>
```




<a id="images"></a>

## 6.7 Images

<a id="image-description"></a>

Syntax for images is like the syntax for links, with one
difference. Instead of [link text](017-6-5-strikethrough-extension.md#link-text), we have an
[image description](019-6-7-images.md#image-description). The rules for this are the
same as for [link text](017-6-5-strikethrough-extension.md#link-text), except that (a) an
image description starts with `![` rather than `[`, and
(b) an image description may contain links.
An image description has inline elements
as its contents. When an image is rendered to HTML,
this is standardly used as the image’s `alt` attribute.

<a id="example-581"></a>

[Example 581](019-6-7-images.md#example-581)



```
![foo](/url "title")
```






```
<p><img src="/url" alt="foo" title="title" /></p>
```




<a id="example-582"></a>

[Example 582](019-6-7-images.md#example-582)



```
![foo *bar*]

[foo *bar*]: train.jpg "train & tracks"
```






```
<p><img src="train.jpg" alt="foo bar" title="train &amp; tracks" /></p>
```




<a id="example-583"></a>

[Example 583](019-6-7-images.md#example-583)



```
![foo ![bar](/url)](/url2)
```






```
<p><img src="/url2" alt="foo bar" /></p>
```




<a id="example-584"></a>

[Example 584](019-6-7-images.md#example-584)



```
![foo [bar](/url)](/url2)
```






```
<p><img src="/url2" alt="foo bar" /></p>
```




Though this spec is concerned with parsing, not rendering, it is
recommended that in rendering to HTML, only the plain string content
of the [image description](019-6-7-images.md#image-description) be used. Note that in
the above example, the alt attribute’s value is `foo bar`, not `foo [bar](/url)` or `foo <a href="/url">bar</a>`. Only the plain string
content is rendered, without formatting.

<a id="example-585"></a>

[Example 585](019-6-7-images.md#example-585)



```
![foo *bar*][]

[foo *bar*]: train.jpg "train & tracks"
```






```
<p><img src="train.jpg" alt="foo bar" title="train &amp; tracks" /></p>
```




<a id="example-586"></a>

[Example 586](019-6-7-images.md#example-586)



```
![foo *bar*][foobar]

[FOOBAR]: train.jpg "train & tracks"
```






```
<p><img src="train.jpg" alt="foo bar" title="train &amp; tracks" /></p>
```




<a id="example-587"></a>

[Example 587](019-6-7-images.md#example-587)



```
![foo](train.jpg)
```






```
<p><img src="train.jpg" alt="foo" /></p>
```




<a id="example-588"></a>

[Example 588](019-6-7-images.md#example-588)



```
My ![foo bar](/path/to/train.jpg  "title"   )
```






```
<p>My <img src="/path/to/train.jpg" alt="foo bar" title="title" /></p>
```




<a id="example-589"></a>

[Example 589](019-6-7-images.md#example-589)



```
![foo](<url>)
```






```
<p><img src="url" alt="foo" /></p>
```




<a id="example-590"></a>

[Example 590](019-6-7-images.md#example-590)



```
![](/url)
```






```
<p><img src="/url" alt="" /></p>
```




Reference-style:

<a id="example-591"></a>

[Example 591](019-6-7-images.md#example-591)



```
![foo][bar]

[bar]: /url
```






```
<p><img src="/url" alt="foo" /></p>
```




<a id="example-592"></a>

[Example 592](019-6-7-images.md#example-592)



```
![foo][bar]

[BAR]: /url
```






```
<p><img src="/url" alt="foo" /></p>
```




Collapsed:

<a id="example-593"></a>

[Example 593](019-6-7-images.md#example-593)



```
![foo][]

[foo]: /url "title"
```






```
<p><img src="/url" alt="foo" title="title" /></p>
```




<a id="example-594"></a>

[Example 594](019-6-7-images.md#example-594)



```
![*foo* bar][]

[*foo* bar]: /url "title"
```






```
<p><img src="/url" alt="foo bar" title="title" /></p>
```




The labels are case-insensitive:

<a id="example-595"></a>

[Example 595](019-6-7-images.md#example-595)



```
![Foo][]

[foo]: /url "title"
```






```
<p><img src="/url" alt="Foo" title="title" /></p>
```




As with reference links, [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is not allowed
between the two sets of brackets:

<a id="example-596"></a>

[Example 596](019-6-7-images.md#example-596)



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

<a id="example-597"></a>

[Example 597](019-6-7-images.md#example-597)



```
![foo]

[foo]: /url "title"
```






```
<p><img src="/url" alt="foo" title="title" /></p>
```




<a id="example-598"></a>

[Example 598](019-6-7-images.md#example-598)



```
![*foo* bar]

[*foo* bar]: /url "title"
```






```
<p><img src="/url" alt="foo bar" title="title" /></p>
```




Note that link labels cannot contain unescaped brackets:

<a id="example-599"></a>

[Example 599](019-6-7-images.md#example-599)



```
![[foo]]

[[foo]]: /url "title"
```






```
<p>![[foo]]</p>
<p>[[foo]]: /url &quot;title&quot;</p>
```




The link labels are case-insensitive:

<a id="example-600"></a>

[Example 600](019-6-7-images.md#example-600)



```
![Foo]

[foo]: /url "title"
```






```
<p><img src="/url" alt="Foo" title="title" /></p>
```




If you just want a literal `!` followed by bracketed text, you can
backslash-escape the opening `[`:

<a id="example-601"></a>

[Example 601](019-6-7-images.md#example-601)



```
!\[foo]

[foo]: /url "title"
```






```
<p>![foo]</p>
```




If you want a link after a literal `!`, backslash-escape the
`!`:

<a id="example-602"></a>

[Example 602](019-6-7-images.md#example-602)



```
\![foo]

[foo]: /url "title"
```






```
<p>!<a href="/url" title="title">foo</a></p>
```




