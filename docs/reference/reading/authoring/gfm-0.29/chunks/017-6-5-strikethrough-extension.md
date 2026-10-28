# 6.5 Strikethrough (extension)

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-continuation-16.md) · [Next](018-continuation-18.md)

```
_____foo_____
```






```
<p><em><strong><strong>foo</strong></strong></em></p>
```




Rule 15:

<a id="example-478"></a>

[Example 478](017-6-5-strikethrough-extension.md#example-478)



```
*foo _bar* baz_
```






```
<p><em>foo _bar</em> baz_</p>
```




<a id="example-479"></a>

[Example 479](017-6-5-strikethrough-extension.md#example-479)



```
*foo __bar *baz bim__ bam*
```






```
<p><em>foo <strong>bar *baz bim</strong> bam</em></p>
```




Rule 16:

<a id="example-480"></a>

[Example 480](017-6-5-strikethrough-extension.md#example-480)



```
**foo **bar baz**
```






```
<p>**foo <strong>bar baz</strong></p>
```




<a id="example-481"></a>

[Example 481](017-6-5-strikethrough-extension.md#example-481)



```
*foo *bar baz*
```






```
<p>*foo <em>bar baz</em></p>
```




Rule 17:

<a id="example-482"></a>

[Example 482](017-6-5-strikethrough-extension.md#example-482)



```
*[bar*](/url)
```






```
<p>*<a href="/url">bar*</a></p>
```




<a id="example-483"></a>

[Example 483](017-6-5-strikethrough-extension.md#example-483)



```
_foo [bar_](/url)
```






```
<p>_foo <a href="/url">bar_</a></p>
```




<a id="example-484"></a>

[Example 484](017-6-5-strikethrough-extension.md#example-484)



```
*<img src="foo" title="*"/>
```






```
<p>*<img src="foo" title="*"/></p>
```




<a id="example-485"></a>

[Example 485](017-6-5-strikethrough-extension.md#example-485)



```
**<a href="**">
```






```
<p>**<a href="**"></p>
```




<a id="example-486"></a>

[Example 486](017-6-5-strikethrough-extension.md#example-486)



```
__<a href="__">
```






```
<p>__<a href="__"></p>
```




<a id="example-487"></a>

[Example 487](017-6-5-strikethrough-extension.md#example-487)



```
*a `*`*
```






```
<p><em>a <code>*</code></em></p>
```




<a id="example-488"></a>

[Example 488](017-6-5-strikethrough-extension.md#example-488)



```
_a `_`_
```






```
<p><em>a <code>_</code></em></p>
```




<a id="example-489"></a>

[Example 489](017-6-5-strikethrough-extension.md#example-489)



```
**a<http://foo.bar/?q=**>
```






```
<p>**a<a href="http://foo.bar/?q=**">http://foo.bar/?q=**</a></p>
```




<a id="example-490"></a>

[Example 490](017-6-5-strikethrough-extension.md#example-490)



```
__a<http://foo.bar/?q=__>
```






```
<p>__a<a href="http://foo.bar/?q=__">http://foo.bar/?q=__</a></p>
```




<a id="strikethrough-extension-"></a>

## 6.5 Strikethrough (extension)

GFM enables the `strikethrough` extension, where an additional emphasis type is
available.

Strikethrough text is any text wrapped in a matching pair of one or two tildes
(`~`).

<a id="example-491"></a>

[Example 491](017-6-5-strikethrough-extension.md#example-491)



```
~~Hi~~ Hello, ~there~ world!
```






```
<p><del>Hi</del> Hello, <del>there</del> world!</p>
```




As with regular emphasis delimiters, a new paragraph will cause strikethrough
parsing to cease:

<a id="example-492"></a>

[Example 492](017-6-5-strikethrough-extension.md#example-492)



```
This ~~has a

new paragraph~~.
```






```
<p>This ~~has a</p>
<p>new paragraph~~.</p>
```




Three or more tildes do not create a strikethrough:

<a id="example-493"></a>

[Example 493](017-6-5-strikethrough-extension.md#example-493)



```
This will ~~~not~~~ strike.
```






```
<p>This will ~~~not~~~ strike.</p>
```




<a id="links"></a>

## 6.6 Links

A link contains [link text](017-6-5-strikethrough-extension.md#link-text) (the visible text), a [link destination](017-6-5-strikethrough-extension.md#link-destination)
(the URI that is the link destination), and optionally a [link title](017-6-5-strikethrough-extension.md#link-title).
There are two basic kinds of links in Markdown. In [inline links](017-6-5-strikethrough-extension.md#inline-link) the
destination and title are given immediately after the link text. In
[reference links](018-continuation-18.md#reference-link) the destination and title are defined elsewhere in
the document.

<a id="link-text"></a>

A [link text](017-6-5-strikethrough-extension.md#link-text) consists of a sequence of zero or more
inline elements enclosed by square brackets (`[` and `]`). The
following rules apply:

- Links may not contain other links, at any level of nesting. If
  multiple otherwise valid link definitions appear nested inside each
  other, the inner-most definition is used.
- Brackets are allowed in the [link text](017-6-5-strikethrough-extension.md#link-text) only if (a) they
  are backslash-escaped or (b) they appear as a matched pair of brackets,
  with an open bracket `[`, a sequence of zero or more inlines, and
  a close bracket `]`.
- Backtick [code spans](014-6-3-code-spans.md#code-spans), [autolinks](020-6-8-autolinks.md#autolinks), and raw [HTML tags](021-6-10-raw-html.md#html-tag) bind more tightly
  than the brackets in link text. Thus, for example,
  `` [foo`]` `` could not be a link text, since the second `]`
  is part of a code span.
- The brackets in link text bind more tightly than markers for
  [emphasis and strong emphasis](014-6-3-code-spans.md#emphasis-and-strong-emphasis). Thus, for example, `*[foo*](url)` is a link.

<a id="link-destination"></a>

A [link destination](017-6-5-strikethrough-extension.md#link-destination) consists of either

- a sequence of zero or more characters between an opening `<` and a
  closing `>` that contains no line breaks or unescaped
  `<` or `>` characters, or
- a nonempty sequence of characters that does not start with
  `<`, does not include ASCII space or control characters, and
  includes parentheses only if (a) they are backslash-escaped or
  (b) they are part of a balanced pair of unescaped parentheses.
  (Implementations may impose limits on parentheses nesting to
  avoid performance issues, but at least three levels of nesting
  should be supported.)

<a id="link-title"></a>

A [link title](017-6-5-strikethrough-extension.md#link-title) consists of either

- a sequence of zero or more characters between straight double-quote
  characters (`"`), including a `"` character only if it is
  backslash-escaped, or
- a sequence of zero or more characters between straight single-quote
  characters (`'`), including a `'` character only if it is
  backslash-escaped, or
- a sequence of zero or more characters between matching parentheses
  (`(...)`), including a `(` or `)` character only if it is
  backslash-escaped.

Although [link titles](017-6-5-strikethrough-extension.md#link-title) may span multiple lines, they may not contain
a [blank line](002-1-3-why-is-a-spec-needed.md#blank-line).

<a id="inline-link"></a>

An [inline link](017-6-5-strikethrough-extension.md#inline-link) consists of a [link text](017-6-5-strikethrough-extension.md#link-text) followed immediately
by a left parenthesis `(`, optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), an optional
[link destination](017-6-5-strikethrough-extension.md#link-destination), an optional [link title](017-6-5-strikethrough-extension.md#link-title) separated from the link
destination by [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), and a right
parenthesis `)`. The link’s text consists of the inlines contained
in the [link text](017-6-5-strikethrough-extension.md#link-text) (excluding the enclosing square brackets).
The link’s URI consists of the link destination, excluding enclosing
`<...>` if present, with backslash-escapes in effect as described
above. The link’s title consists of the link title, excluding its
enclosing delimiters, with backslash-escapes in effect as described
above.

Here is a simple inline link:

<a id="example-494"></a>

[Example 494](017-6-5-strikethrough-extension.md#example-494)



```
[link](/uri "title")
```






```
<p><a href="/uri" title="title">link</a></p>
```




The title may be omitted:

<a id="example-495"></a>

[Example 495](017-6-5-strikethrough-extension.md#example-495)



```
[link](/uri)
```






```
<p><a href="/uri">link</a></p>
```




Both the title and the destination may be omitted:

<a id="example-496"></a>

[Example 496](017-6-5-strikethrough-extension.md#example-496)



```
[link]()
```






```
<p><a href="">link</a></p>
```




<a id="example-497"></a>

[Example 497](017-6-5-strikethrough-extension.md#example-497)



```
[link](<>)
```






```
<p><a href="">link</a></p>
```




The destination can only contain spaces if it is
enclosed in pointy brackets:

<a id="example-498"></a>

[Example 498](017-6-5-strikethrough-extension.md#example-498)



```
[link](/my uri)
```






```
<p>[link](/my uri)</p>
```




<a id="example-499"></a>

[Example 499](017-6-5-strikethrough-extension.md#example-499)



```
[link](</my uri>)
```






```
<p><a href="/my%20uri">link</a></p>
```




The destination cannot contain line breaks,
even if enclosed in pointy brackets:

<a id="example-500"></a>

[Example 500](017-6-5-strikethrough-extension.md#example-500)



```
[link](foo
bar)
```






```
<p>[link](foo
bar)</p>
```




<a id="example-501"></a>

[Example 501](017-6-5-strikethrough-extension.md#example-501)



```
[link](<foo
bar>)
```






```
<p>[link](<foo
bar>)</p>
```




The destination can contain `)` if it is enclosed
in pointy brackets:

<a id="example-502"></a>

[Example 502](017-6-5-strikethrough-extension.md#example-502)



```
[a](<b)c>)
```






```
<p><a href="b)c">a</a></p>
```




Pointy brackets that enclose links must be unescaped:

<a id="example-503"></a>

[Example 503](017-6-5-strikethrough-extension.md#example-503)



```
[link](<foo\>)
```






```
<p>[link](&lt;foo&gt;)</p>
```




These are not links, because the opening pointy bracket
is not matched properly:

<a id="example-504"></a>

[Example 504](017-6-5-strikethrough-extension.md#example-504)



```
[a](<b)c
[a](<b)c>
[a](<b>c)
```






```
<p>[a](&lt;b)c
[a](&lt;b)c&gt;
[a](<b>c)</p>
```




Parentheses inside the link destination may be escaped:

<a id="example-505"></a>

[Example 505](017-6-5-strikethrough-extension.md#example-505)



```
[link](\(foo\))
```






```
<p><a href="(foo)">link</a></p>
```




Any number of parentheses are allowed without escaping, as long as they are
balanced:

<a id="example-506"></a>

[Example 506](017-6-5-strikethrough-extension.md#example-506)



```
[link](foo(and(bar)))
```






```
<p><a href="foo(and(bar))">link</a></p>
```




However, if you have unbalanced parentheses, you need to escape or use the
`<...>` form:

<a id="example-507"></a>

[Example 507](017-6-5-strikethrough-extension.md#example-507)



```
[link](foo\(and\(bar\))
```






```
<p><a href="foo(and(bar)">link</a></p>
```




<a id="example-508"></a>

[Example 508](017-6-5-strikethrough-extension.md#example-508)



```
[link](<foo(and(bar)>)
```






```
<p><a href="foo(and(bar)">link</a></p>
```




Parentheses and other symbols can also be escaped, as usual
in Markdown:

<a id="example-509"></a>

[Example 509](017-6-5-strikethrough-extension.md#example-509)



```
[link](foo\)\:)
```






```
<p><a href="foo):">link</a></p>
```




A link can contain fragment identifiers and queries:

<a id="example-510"></a>

[Example 510](017-6-5-strikethrough-extension.md#example-510)



```
[link](#fragment)

[link](http://example.com#fragment)

[link](http://example.com?foo=3#frag)
```






```
<p><a href="#fragment">link</a></p>
<p><a href="http://example.com#fragment">link</a></p>
<p><a href="http://example.com?foo=3#frag">link</a></p>
```




Note that a backslash before a non-escapable character is
just a backslash:

<a id="example-511"></a>

[Example 511](017-6-5-strikethrough-extension.md#example-511)



```
[link](foo\bar)
```






```
<p><a href="foo%5Cbar">link</a></p>
```




URL-escaping should be left alone inside the destination, as all
URL-escaped characters are also valid URL characters. Entity and
numerical character references in the destination will be parsed
into the corresponding Unicode code points, as usual. These may
be optionally URL-escaped when written as HTML, but this spec
does not enforce any particular policy for rendering URLs in
HTML or other formats. Renderers may make different decisions
about how to escape or normalize URLs in the output.

<a id="example-512"></a>

[Example 512](017-6-5-strikethrough-extension.md#example-512)



```
[link](foo%20b&auml;)
```






```
<p><a href="foo%20b%C3%A4">link</a></p>
```




Note that, because titles can often be parsed as destinations,
if you try to omit the destination and keep the title, you’ll
get unexpected results:

<a id="example-513"></a>

[Example 513](017-6-5-strikethrough-extension.md#example-513)



```
[link]("title")
```






```
<p><a href="%22title%22">link</a></p>
```




Titles may be in single quotes, double quotes, or parentheses:

<a id="example-514"></a>

[Example 514](017-6-5-strikethrough-extension.md#example-514)



```
[link](/url "title")
[link](/url 'title')
[link](/url (title))
```






```
<p><a href="/url" title="title">link</a>
<a href="/url" title="title">link</a>
<a href="/url" title="title">link</a></p>
```




Backslash escapes and entity and numeric character references
may be used in titles:

<a id="example-515"></a>

[Example 515](017-6-5-strikethrough-extension.md#example-515)



```
[link](/url "title \"&quot;")
```






```
<p><a href="/url" title="title &quot;&quot;">link</a></p>
```




Titles must be separated from the link using a [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace).
Other [Unicode whitespace](002-1-3-why-is-a-spec-needed.md#unicode-whitespace) like non-breaking space doesn’t work.

<a id="example-516"></a>

[Example 516](017-6-5-strikethrough-extension.md#example-516)



```
[link](/url "title")
```






```
<p><a href="/url%C2%A0%22title%22">link</a></p>
```




Nested balanced quotes are not allowed without escaping:

<a id="example-517"></a>

[Example 517](017-6-5-strikethrough-extension.md#example-517)



```
[link](/url "title "and" title")
```






```
<p>[link](/url &quot;title &quot;and&quot; title&quot;)</p>
```




But it is easy to work around this by using a different quote type:

