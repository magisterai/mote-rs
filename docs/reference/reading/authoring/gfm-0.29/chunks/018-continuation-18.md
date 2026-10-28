# Continuation 18

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](017-6-5-strikethrough-extension.md) · [Next](019-6-7-images.md)

<a id="example-518"></a>

[Example 518](018-continuation-18.md#example-518)



```
[link](/url 'title "and" title')
```






```
<p><a href="/url" title="title &quot;and&quot; title">link</a></p>
```




(Note: `Markdown.pl` did allow double quotes inside a double-quoted
title, and its test suite included a test demonstrating this.
But it is hard to see a good rationale for the extra complexity this
brings, since there are already many ways—backslash escaping,
entity and numeric character references, or using a different
quote type for the enclosing title—to write titles containing
double quotes. `Markdown.pl`’s handling of titles has a number
of other strange features. For example, it allows single-quoted
titles in inline links, but not reference links. And, in
reference links but not inline links, it allows a title to begin
with `"` and end with `)`. `Markdown.pl` 1.0.1 even allows
titles with no closing quotation mark, though 1.0.2b8 does not.
It seems preferable to adopt a simple, rational rule that works
the same way in inline links and link reference definitions.)

[Whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is allowed around the destination and title:

<a id="example-519"></a>

[Example 519](018-continuation-18.md#example-519)



```
[link](   /uri
  "title"  )
```






```
<p><a href="/uri" title="title">link</a></p>
```




But it is not allowed between the link text and the
following parenthesis:

<a id="example-520"></a>

[Example 520](018-continuation-18.md#example-520)



```
[link] (/uri)
```






```
<p>[link] (/uri)</p>
```




The link text may contain balanced brackets, but not unbalanced ones,
unless they are escaped:

<a id="example-521"></a>

[Example 521](018-continuation-18.md#example-521)



```
[link [foo [bar]]](/uri)
```






```
<p><a href="/uri">link [foo [bar]]</a></p>
```




<a id="example-522"></a>

[Example 522](018-continuation-18.md#example-522)



```
[link] bar](/uri)
```






```
<p>[link] bar](/uri)</p>
```




<a id="example-523"></a>

[Example 523](018-continuation-18.md#example-523)



```
[link [bar](/uri)
```






```
<p>[link <a href="/uri">bar</a></p>
```




<a id="example-524"></a>

[Example 524](018-continuation-18.md#example-524)



```
[link \[bar](/uri)
```






```
<p><a href="/uri">link [bar</a></p>
```




The link text may contain inline content:

<a id="example-525"></a>

[Example 525](018-continuation-18.md#example-525)



```
[link *foo **bar** `#`*](/uri)
```






```
<p><a href="/uri">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>
```




<a id="example-526"></a>

[Example 526](018-continuation-18.md#example-526)



```
[![moon](moon.jpg)](/uri)
```






```
<p><a href="/uri"><img src="moon.jpg" alt="moon" /></a></p>
```




However, links may not contain other links, at any level of nesting.

<a id="example-527"></a>

[Example 527](018-continuation-18.md#example-527)



```
[foo [bar](/uri)](/uri)
```






```
<p>[foo <a href="/uri">bar</a>](/uri)</p>
```




<a id="example-528"></a>

[Example 528](018-continuation-18.md#example-528)



```
[foo *[bar [baz](/uri)](/uri)*](/uri)
```






```
<p>[foo <em>[bar <a href="/uri">baz</a>](/uri)</em>](/uri)</p>
```




<a id="example-529"></a>

[Example 529](018-continuation-18.md#example-529)



```
![[[foo](uri1)](uri2)](uri3)
```






```
<p><img src="uri3" alt="[foo](uri2)" /></p>
```




These cases illustrate the precedence of link text grouping over
emphasis grouping:

<a id="example-530"></a>

[Example 530](018-continuation-18.md#example-530)



```
*[foo*](/uri)
```






```
<p>*<a href="/uri">foo*</a></p>
```




<a id="example-531"></a>

[Example 531](018-continuation-18.md#example-531)



```
[foo *bar](baz*)
```






```
<p><a href="baz*">foo *bar</a></p>
```




Note that brackets that *aren’t* part of links do not take
precedence:

<a id="example-532"></a>

[Example 532](018-continuation-18.md#example-532)



```
*foo [bar* baz]
```






```
<p><em>foo [bar</em> baz]</p>
```




These cases illustrate the precedence of HTML tags, code spans,
and autolinks over link grouping:

<a id="example-533"></a>

[Example 533](018-continuation-18.md#example-533)



```
[foo <bar attr="](baz)">
```






```
<p>[foo <bar attr="](baz)"></p>
```




<a id="example-534"></a>

[Example 534](018-continuation-18.md#example-534)



```
[foo`](/uri)`
```






```
<p>[foo<code>](/uri)</code></p>
```




<a id="example-535"></a>

[Example 535](018-continuation-18.md#example-535)



```
[foo<http://example.com/?search=](uri)>
```






```
<p>[foo<a href="http://example.com/?search=%5D(uri)">http://example.com/?search=](uri)</a></p>
```




<a id="reference-link"></a>

There are three kinds of [reference link](018-continuation-18.md#reference-link)s:
[full](018-continuation-18.md#full-reference-link), [collapsed](019-6-7-images.md#collapsed-reference-link),
and [shortcut](019-6-7-images.md#shortcut-reference-link).

<a id="full-reference-link"></a>

A [full reference link](018-continuation-18.md#full-reference-link)
consists of a [link text](017-6-5-strikethrough-extension.md#link-text) immediately followed by a [link label](018-continuation-18.md#link-label)
that [matches](018-continuation-18.md#matches) a [link reference definition](007-4-7-link-reference-definitions.md#link-reference-definition) elsewhere in the document.

<a id="link-label"></a>

A [link label](018-continuation-18.md#link-label) begins with a left bracket (`[`) and ends
with the first right bracket (`]`) that is not backslash-escaped.
Between these brackets there must be at least one [non-whitespace character](002-1-3-why-is-a-spec-needed.md#non-whitespace-character).
Unescaped square bracket characters are not allowed inside the
opening and closing square brackets of [link labels](018-continuation-18.md#link-label). A link
label can have at most 999 characters inside the square
brackets.

<a id="matches"></a>

One label [matches](018-continuation-18.md#matches)
another just in case their normalized forms are equal. To normalize a
label, strip off the opening and closing brackets,
perform the *Unicode case fold*, strip leading and trailing
[whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) and collapse consecutive internal
[whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) to a single space. If there are multiple
matching reference link definitions, the one that comes first in the
document is used. (It is desirable in such cases to emit a warning.)

The link’s URI and title are provided by the matching [link
reference definition](007-4-7-link-reference-definitions.md#link-reference-definition).

Here is a simple example:

<a id="example-536"></a>

[Example 536](018-continuation-18.md#example-536)



```
[foo][bar]

[bar]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




The rules for the [link text](017-6-5-strikethrough-extension.md#link-text) are the same as with
[inline links](017-6-5-strikethrough-extension.md#inline-link). Thus:

The link text may contain balanced brackets, but not unbalanced ones,
unless they are escaped:

<a id="example-537"></a>

[Example 537](018-continuation-18.md#example-537)



```
[link [foo [bar]]][ref]

[ref]: /uri
```






```
<p><a href="/uri">link [foo [bar]]</a></p>
```




<a id="example-538"></a>

[Example 538](018-continuation-18.md#example-538)



```
[link \[bar][ref]

[ref]: /uri
```






```
<p><a href="/uri">link [bar</a></p>
```




The link text may contain inline content:

<a id="example-539"></a>

[Example 539](018-continuation-18.md#example-539)



```
[link *foo **bar** `#`*][ref]

[ref]: /uri
```






```
<p><a href="/uri">link <em>foo <strong>bar</strong> <code>#</code></em></a></p>
```




<a id="example-540"></a>

[Example 540](018-continuation-18.md#example-540)



```
[![moon](moon.jpg)][ref]

[ref]: /uri
```






```
<p><a href="/uri"><img src="moon.jpg" alt="moon" /></a></p>
```




However, links may not contain other links, at any level of nesting.

<a id="example-541"></a>

[Example 541](018-continuation-18.md#example-541)



```
[foo [bar](/uri)][ref]

[ref]: /uri
```






```
<p>[foo <a href="/uri">bar</a>]<a href="/uri">ref</a></p>
```




<a id="example-542"></a>

[Example 542](018-continuation-18.md#example-542)



```
[foo *bar [baz][ref]*][ref]

[ref]: /uri
```






```
<p>[foo <em>bar <a href="/uri">baz</a></em>]<a href="/uri">ref</a></p>
```




(In the examples above, we have two [shortcut reference links](019-6-7-images.md#shortcut-reference-link)
instead of one [full reference link](018-continuation-18.md#full-reference-link).)

The following cases illustrate the precedence of link text grouping over
emphasis grouping:

<a id="example-543"></a>

[Example 543](018-continuation-18.md#example-543)



```
*[foo*][ref]

[ref]: /uri
```






```
<p>*<a href="/uri">foo*</a></p>
```




<a id="example-544"></a>

[Example 544](018-continuation-18.md#example-544)



```
[foo *bar][ref]*

[ref]: /uri
```






```
<p><a href="/uri">foo *bar</a>*</p>
```




These cases illustrate the precedence of HTML tags, code spans,
and autolinks over link grouping:

<a id="example-545"></a>

[Example 545](018-continuation-18.md#example-545)



```
[foo <bar attr="][ref]">

[ref]: /uri
```






```
<p>[foo <bar attr="][ref]"></p>
```




<a id="example-546"></a>

[Example 546](018-continuation-18.md#example-546)



```
[foo`][ref]`

[ref]: /uri
```






```
<p>[foo<code>][ref]</code></p>
```




<a id="example-547"></a>

[Example 547](018-continuation-18.md#example-547)



```
[foo<http://example.com/?search=][ref]>

[ref]: /uri
```






```
<p>[foo<a href="http://example.com/?search=%5D%5Bref%5D">http://example.com/?search=][ref]</a></p>
```




Matching is case-insensitive:

<a id="example-548"></a>

[Example 548](018-continuation-18.md#example-548)



```
[foo][BaR]

[bar]: /url "title"
```






```
<p><a href="/url" title="title">foo</a></p>
```




Unicode case fold is used:

<a id="example-549"></a>

[Example 549](018-continuation-18.md#example-549)



```
[ẞ]

[SS]: /url
```






```
<p><a href="/url">ẞ</a></p>
```




Consecutive internal [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is treated as one space for
purposes of determining matching:

<a id="example-550"></a>

[Example 550](018-continuation-18.md#example-550)



```
[Foo
  bar]: /url

[Baz][Foo bar]
```






```
<p><a href="/url">Baz</a></p>
```




No [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is allowed between the [link text](017-6-5-strikethrough-extension.md#link-text) and the
[link label](018-continuation-18.md#link-label):

<a id="example-551"></a>

[Example 551](018-continuation-18.md#example-551)



```
[foo] [bar]

[bar]: /url "title"
```






```
<p>[foo] <a href="/url" title="title">bar</a></p>
```




<a id="example-552"></a>

[Example 552](018-continuation-18.md#example-552)



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
[inline links](017-6-5-strikethrough-extension.md#inline-link), which (according to both original Markdown and
this spec) cannot have whitespace after the link text. More
importantly, it prevents inadvertent capture of consecutive
[shortcut reference links](019-6-7-images.md#shortcut-reference-link). If whitespace is allowed between the
link text and the link label, then in the following we will have
a single reference link, not two shortcut reference links, as
intended:



```
[foo]
[bar]

[foo]: /url1
[bar]: /url2
```




(Note that [shortcut reference links](019-6-7-images.md#shortcut-reference-link) were introduced by Gruber
himself in a beta version of `Markdown.pl`, but never included
in the official syntax description. Without shortcut reference
links, it is harmless to allow space between the link text and
link label; but once shortcut references are introduced, it is
too dangerous to allow this, as it frequently leads to
unintended results.)

When there are multiple matching [link reference definitions](007-4-7-link-reference-definitions.md#link-reference-definitions),
the first is used:

<a id="example-553"></a>

[Example 553](018-continuation-18.md#example-553)



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

<a id="example-554"></a>

[Example 554](018-continuation-18.md#example-554)



```
[bar][foo\!]

[foo!]: /url
```






```
<p>[bar][foo!]</p>
```




[Link labels](018-continuation-18.md#link-label) cannot contain brackets, unless they are
backslash-escaped:

<a id="example-555"></a>

[Example 555](018-continuation-18.md#example-555)



```
[foo][ref[]

[ref[]: /uri
```






```
<p>[foo][ref[]</p>
<p>[ref[]: /uri</p>
```




<a id="example-556"></a>

[Example 556](018-continuation-18.md#example-556)



```
[foo][ref[bar]]

[ref[bar]]: /uri
```






```
<p>[foo][ref[bar]]</p>
<p>[ref[bar]]: /uri</p>
```




<a id="example-557"></a>

[Example 557](018-continuation-18.md#example-557)



```
[[[foo]]]

[[[foo]]]: /url
```






```
<p>[[[foo]]]</p>
<p>[[[foo]]]: /url</p>
```




<a id="example-558"></a>

[Example 558](018-continuation-18.md#example-558)



