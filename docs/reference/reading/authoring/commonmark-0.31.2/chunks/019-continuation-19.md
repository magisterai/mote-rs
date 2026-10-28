# Continuation 19

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-6-3-links.md) · [Next](020-6-4-images.md)

<a id="example-515"></a>

[Example 515](019-continuation-19.md#example-515)Try It



```
[link \[bar](/uri)
```






```
<p><a href="/uri">link [bar</a></p>
```




The link text may contain inline content:

<a id="example-516"></a>

[Example 516](019-continuation-19.md#example-516)Try It



```
[link *foo **bar** `#`*](/uri)
```






```
<p><a href="/uri">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>
```




<a id="example-517"></a>

[Example 517](019-continuation-19.md#example-517)Try It



```
[![moon](moon.jpg)](/uri)
```






```
<p><a href="/uri"><img src="moon.jpg" alt="moon" /></a></p>
```




However, links may not contain other links, at any level of nesting.

<a id="example-518"></a>

[Example 518](019-continuation-19.md#example-518)Try It



```
[foo [bar](/uri)](/uri)
```






```
<p>[foo <a href="/uri">bar</a>](/uri)</p>
```




<a id="example-519"></a>

[Example 519](019-continuation-19.md#example-519)Try It



```
[foo *[bar [baz](/uri)](/uri)*](/uri)
```






```
<p>[foo <em>[bar <a href="/uri">baz</a>](/uri)</em>](/uri)</p>
```




<a id="example-520"></a>

[Example 520](019-continuation-19.md#example-520)Try It



```
![[[foo](uri1)](uri2)](uri3)
```






```
<p><img src="uri3" alt="[foo](uri2)" /></p>
```




These cases illustrate the precedence of link text grouping over
emphasis grouping:

<a id="example-521"></a>

[Example 521](019-continuation-19.md#example-521)Try It



```
*[foo*](/uri)
```






```
<p>*<a href="/uri">foo*</a></p>
```




<a id="example-522"></a>

[Example 522](019-continuation-19.md#example-522)Try It



```
[foo *bar](baz*)
```






```
<p><a href="baz*">foo *bar</a></p>
```




Note that brackets that *aren’t* part of links do not take
precedence:

<a id="example-523"></a>

[Example 523](019-continuation-19.md#example-523)Try It



```
*foo [bar* baz]
```






```
<p><em>foo [bar</em> baz]</p>
```




These cases illustrate the precedence of HTML tags, code spans,
and autolinks over link grouping:

<a id="example-524"></a>

[Example 524](019-continuation-19.md#example-524)Try It



```
[foo <bar attr="](baz)">
```






```
<p>[foo <bar attr="](baz)"></p>
```




<a id="example-525"></a>

[Example 525](019-continuation-19.md#example-525)Try It



```
[foo`](/uri)`
```






```
<p>[foo<code>](/uri)</code></p>
```




<a id="example-526"></a>

[Example 526](019-continuation-19.md#example-526)Try It



```
[foo<https://example.com/?search=](uri)>
```






```
<p>[foo<a href="https://example.com/?search=%5D(uri)">https://example.com/?search=](uri)</a></p>
```




<a id="reference-link"></a>

There are three kinds of [reference link](019-continuation-19.md#reference-link)s:
[full](019-continuation-19.md#full-reference-link), [collapsed](019-continuation-19.md#collapsed-reference-link),
and [shortcut](020-6-4-images.md#shortcut-reference-link).

<a id="full-reference-link"></a>

A [full reference link](019-continuation-19.md#full-reference-link)
consists of a [link text](018-6-3-links.md#link-text) immediately followed by a [link label](019-continuation-19.md#link-label)
that [matches](019-continuation-19.md#matches) a [link reference definition](009-4-7-link-reference-definitions.md#link-reference-definition) elsewhere in the document.

<a id="link-label"></a>

A [link label](019-continuation-19.md#link-label) begins with a left bracket (`[`) and ends
with the first right bracket (`]`) that is not backslash-escaped.
Between these brackets there must be at least one character that is not a space,
tab, or line ending.
Unescaped square bracket characters are not allowed inside the
opening and closing square brackets of [link labels](019-continuation-19.md#link-label). A link
label can have at most 999 characters inside the square
brackets.

<a id="matches"></a>

One label [matches](019-continuation-19.md#matches)
another just in case their normalized forms are equal. To normalize a
label, strip off the opening and closing brackets,
perform the *Unicode case fold*, strip leading and trailing
spaces, tabs, and line endings, and collapse consecutive internal
spaces, tabs, and line endings to a single space. If there are multiple
matching reference link definitions, the one that comes first in the
document is used. (It is desirable in such cases to emit a warning.)

The link’s URI and title are provided by the matching [link
reference definition](009-4-7-link-reference-definitions.md#link-reference-definition).

Here is a simple example:

<a id="example-527"></a>

[Example 527](019-continuation-19.md#example-527)Try It



```
[foo][bar]

[bar]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




The rules for the [link text](018-6-3-links.md#link-text) are the same as with
[inline links](018-6-3-links.md#inline-link). Thus:

The link text may contain balanced brackets, but not unbalanced ones,
unless they are escaped:

<a id="example-528"></a>

[Example 528](019-continuation-19.md#example-528)Try It



```
[link [foo [bar]]][ref]

[ref]: /uri
```






```
<p><a href="/uri">link [foo [bar]]</a></p>
```




<a id="example-529"></a>

[Example 529](019-continuation-19.md#example-529)Try It



```
[link \[bar][ref]

[ref]: /uri
```






```
<p><a href="/uri">link [bar</a></p>
```




The link text may contain inline content:

<a id="example-530"></a>

[Example 530](019-continuation-19.md#example-530)Try It



```
[link *foo **bar** `#`*][ref]

[ref]: /uri
```






```
<p><a href="/uri">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>
```




<a id="example-531"></a>

[Example 531](019-continuation-19.md#example-531)Try It



```
[![moon](moon.jpg)][ref]

[ref]: /uri
```






```
<p><a href="/uri"><img src="moon.jpg" alt="moon" /></a></p>
```




However, links may not contain other links, at any level of nesting.

<a id="example-532"></a>

[Example 532](019-continuation-19.md#example-532)Try It



```
[foo [bar](/uri)][ref]

[ref]: /uri
```






```
<p>[foo <a href="/uri">bar</a>]<a href="/uri">ref</a></p>
```




<a id="example-533"></a>

[Example 533](019-continuation-19.md#example-533)Try It



```
[foo *bar [baz][ref]*][ref]

[ref]: /uri
```






```
<p>[foo <em>bar <a href="/uri">baz</a></em>]<a href="/uri">ref</a></p>
```




(In the examples above, we have two [shortcut reference links](020-6-4-images.md#shortcut-reference-link)
instead of one [full reference link](019-continuation-19.md#full-reference-link).)

The following cases illustrate the precedence of link text grouping over
emphasis grouping:

<a id="example-534"></a>

[Example 534](019-continuation-19.md#example-534)Try It



```
*[foo*][ref]

[ref]: /uri
```






```
<p>*<a href="/uri">foo*</a></p>
```




<a id="example-535"></a>

[Example 535](019-continuation-19.md#example-535)Try It



```
[foo *bar][ref]*

[ref]: /uri
```






```
<p><a href="/uri">foo *bar</a>*</p>
```




These cases illustrate the precedence of HTML tags, code spans,
and autolinks over link grouping:

<a id="example-536"></a>

[Example 536](019-continuation-19.md#example-536)Try It



```
[foo <bar attr="][ref]">

[ref]: /uri
```






```
<p>[foo <bar attr="][ref]"></p>
```




<a id="example-537"></a>

[Example 537](019-continuation-19.md#example-537)Try It



```
[foo`][ref]`

[ref]: /uri
```






```
<p>[foo<code>][ref]</code></p>
```




<a id="example-538"></a>

[Example 538](019-continuation-19.md#example-538)Try It



```
[foo<https://example.com/?search=][ref]>

[ref]: /uri
```






```
<p>[foo<a href="https://example.com/?search=%5D%5Bref%5D">https://example.com/?search=][ref]</a></p>
```




Matching is case-insensitive:

<a id="example-539"></a>

[Example 539](019-continuation-19.md#example-539)Try It



```
[foo][BaR]

[bar]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




Unicode case fold is used:

<a id="example-540"></a>

[Example 540](019-continuation-19.md#example-540)Try It



```
[ẞ]

[SS]: /url
```






```
<p><a href="/url">ẞ</a></p>
```




Consecutive internal spaces, tabs, and line endings are treated as one space for
purposes of determining matching:

<a id="example-541"></a>

[Example 541](019-continuation-19.md#example-541)Try It



```
[Foo
  bar]: /url

[Baz][Foo bar]
```






```
<p><a href="/url">Baz</a></p>
```




No spaces, tabs, or line endings are allowed between the [link text](018-6-3-links.md#link-text) and the
[link label](019-continuation-19.md#link-label):

<a id="example-542"></a>

[Example 542](019-continuation-19.md#example-542)Try It



```
[foo] [bar]

[bar]: /url "title"
```






```
<p>[foo] <a href="/url" title="title">bar</a></p>
```




<a id="example-543"></a>

[Example 543](019-continuation-19.md#example-543)Try It



```
[foo]
[bar]

[bar]: /url "title"
```






```
<p>[foo]
<a href="/url" title="title">bar</a></p>
```




This is a departure from John Gruber’s original Markdown syntax
description, which explicitly allows whitespace between the link
text and the link label. It brings reference links in line with
[inline links](018-6-3-links.md#inline-link), which (according to both original Markdown and
this spec) cannot have whitespace after the link text. More
importantly, it prevents inadvertent capture of consecutive
[shortcut reference links](020-6-4-images.md#shortcut-reference-link). If whitespace is allowed between the
link text and the link label, then in the following we will have
a single reference link, not two shortcut reference links, as
intended:



```
[foo]
[bar]

[foo]: /url1
[bar]: /url2
```




(Note that [shortcut reference links](020-6-4-images.md#shortcut-reference-link) were introduced by Gruber
himself in a beta version of `Markdown.pl`, but never included
in the official syntax description. Without shortcut reference
links, it is harmless to allow space between the link text and
link label; but once shortcut references are introduced, it is
too dangerous to allow this, as it frequently leads to
unintended results.)

When there are multiple matching [link reference definitions](009-4-7-link-reference-definitions.md#link-reference-definition),
the first is used:

<a id="example-544"></a>

[Example 544](019-continuation-19.md#example-544)Try It



```
[foo]: /url1

[foo]: /url2

[bar][foo]
```






```
<p><a href="/url1">bar</a></p>
```




Note that matching is performed on normalized strings, not parsed
inline content. So the following does not match, even though the
labels define equivalent inline content:

<a id="example-545"></a>

[Example 545](019-continuation-19.md#example-545)Try It



```
[bar][foo\!]

[foo!]: /url
```






```
<p>[bar][foo!]</p>
```




[Link labels](019-continuation-19.md#link-label) cannot contain brackets, unless they are
backslash-escaped:

<a id="example-546"></a>

[Example 546](019-continuation-19.md#example-546)Try It



```
[foo][ref[]

[ref[]: /uri
```






```
<p>[foo][ref[]</p>
<p>[ref[]: /uri</p>
```




<a id="example-547"></a>

[Example 547](019-continuation-19.md#example-547)Try It



```
[foo][ref[bar]]

[ref[bar]]: /uri
```






```
<p>[foo][ref[bar]]</p>
<p>[ref[bar]]: /uri</p>
```




<a id="example-548"></a>

[Example 548](019-continuation-19.md#example-548)Try It



```
[[[foo]]]

[[[foo]]]: /url
```






```
<p>[[[foo]]]</p>
<p>[[[foo]]]: /url</p>
```




<a id="example-549"></a>

[Example 549](019-continuation-19.md#example-549)Try It



```
[foo][ref\[]

[ref\[]: /uri
```






```
<p><a href="/uri">foo</a></p>
```




Note that in this example `]` is not backslash-escaped:

<a id="example-550"></a>

[Example 550](019-continuation-19.md#example-550)Try It



```
[bar\\]: /uri

[bar\\]
```






```
<p><a href="/uri">bar\</a></p>
```




A [link label](019-continuation-19.md#link-label) must contain at least one character that is not a space, tab, or
line ending:

<a id="example-551"></a>

[Example 551](019-continuation-19.md#example-551)Try It



```
[]

[]: /uri
```






```
<p>[]</p>
<p>[]: /uri</p>
```




<a id="example-552"></a>

[Example 552](019-continuation-19.md#example-552)Try It



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

A [collapsed reference link](019-continuation-19.md#collapsed-reference-link)
consists of a [link label](019-continuation-19.md#link-label) that [matches](019-continuation-19.md#matches) a
[link reference definition](009-4-7-link-reference-definitions.md#link-reference-definition) elsewhere in the
document, followed by the string `[]`.
The contents of the link label are parsed as inlines,
which are used as the link’s text. The link’s URI and title are
provided by the matching reference link definition. Thus,
`[foo][]` is equivalent to `[foo][foo]`.

<a id="example-553"></a>

[Example 553](019-continuation-19.md#example-553)Try It



```
[foo][]

[foo]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




<a id="example-554"></a>

[Example 554](019-continuation-19.md#example-554)Try It



```
[*foo* bar][]

[*foo* bar]: /url "title"
```






```
<p><a href="/url" title="title"><em>foo</em> bar</a></p>
```




