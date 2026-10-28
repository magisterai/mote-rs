# 6.5 Autolinks

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](020-6-4-images.md) · [Next](022-6-7-hard-line-breaks.md)

<a id="autolinks"></a>

## 6.5 Autolinks

<a id="autolink"></a>

[Autolink](021-6-5-autolinks.md#autolink)s are absolute URIs and email addresses inside
`<` and `>`. They are parsed as links, with the URL or email address
as the link label.

<a id="uri-autolink"></a>

A [URI autolink](021-6-5-autolinks.md#uri-autolink) consists of `<`, followed by an
[absolute URI](021-6-5-autolinks.md#absolute-uri) followed by `>`. It is parsed as
a link to the URI, with the URI as the link’s label.

<a id="absolute-uri"></a>

An [absolute URI](021-6-5-autolinks.md#absolute-uri),
for these purposes, consists of a [scheme](021-6-5-autolinks.md#scheme) followed by a colon (`:`)
followed by zero or more characters other than [ASCII control
characters](002-1-3-about-this-document.md#ascii-control-character), [space](002-1-3-about-this-document.md#space), `<`, and `>`.
If the URI includes these characters, they must be percent-encoded
(e.g. `%20` for a space).

<a id="scheme"></a>

For purposes of this spec, a [scheme](021-6-5-autolinks.md#scheme) is any sequence
of 2–32 characters beginning with an ASCII letter and followed
by any combination of ASCII letters, digits, or the symbols plus
(“+”), period (“.”), or hyphen (“-”).

Here are some valid autolinks:

<a id="example-594"></a>

[Example 594](021-6-5-autolinks.md#example-594)Try It



```
<http://foo.bar.baz>
```






```
<p><a href="http://foo.bar.baz">http://foo.bar.baz</a></p>
```




<a id="example-595"></a>

[Example 595](021-6-5-autolinks.md#example-595)Try It



```
<https://foo.bar.baz/test?q=hello&id=22&boolean>
```






```
<p><a href="https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean">https://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>
```




<a id="example-596"></a>

[Example 596](021-6-5-autolinks.md#example-596)Try It



```
<irc://foo.bar:2233/baz>
```






```
<p><a href="irc://foo.bar:2233/baz">irc://foo.bar:2233/baz</a></p>
```




Uppercase is also fine:

<a id="example-597"></a>

[Example 597](021-6-5-autolinks.md#example-597)Try It



```
<MAILTO:FOO@BAR.BAZ>
```






```
<p><a href="MAILTO:FOO@BAR.BAZ">MAILTO:FOO@BAR.BAZ</a></p>
```




Note that many strings that count as [absolute URIs](021-6-5-autolinks.md#absolute-uri) for
purposes of this spec are not valid URIs, because their
schemes are not registered or because of other problems
with their syntax:

<a id="example-598"></a>

[Example 598](021-6-5-autolinks.md#example-598)Try It



```
<a+b+c:d>
```






```
<p><a href="a+b+c:d">a+b+c:d</a></p>
```




<a id="example-599"></a>

[Example 599](021-6-5-autolinks.md#example-599)Try It



```
<made-up-scheme://foo,bar>
```






```
<p><a href="made-up-scheme://foo,bar">made-up-scheme://foo,bar</a></p>
```




<a id="example-600"></a>

[Example 600](021-6-5-autolinks.md#example-600)Try It



```
<https://../>
```






```
<p><a href="https://../">https://../</a></p>
```




<a id="example-601"></a>

[Example 601](021-6-5-autolinks.md#example-601)Try It



```
<localhost:5001/foo>
```






```
<p><a href="localhost:5001/foo">localhost:5001/foo</a></p>
```




Spaces are not allowed in autolinks:

<a id="example-602"></a>

[Example 602](021-6-5-autolinks.md#example-602)Try It



```
<https://foo.bar/baz bim>
```






```
<p>&lt;https://foo.bar/baz bim&gt;</p>
```




Backslash-escapes do not work inside autolinks:

<a id="example-603"></a>

[Example 603](021-6-5-autolinks.md#example-603)Try It



```
<https://example.com/\[\>
```






```
<p><a href="https://example.com/%5C%5B%5C">https://example.com/\[\</a></p>
```




<a id="email-autolink"></a>

An [email autolink](021-6-5-autolinks.md#email-autolink)
consists of `<`, followed by an [email address](021-6-5-autolinks.md#email-address),
followed by `>`. The link’s label is the email address,
and the URL is `mailto:` followed by the email address.

<a id="email-address"></a>

An [email address](021-6-5-autolinks.md#email-address),
for these purposes, is anything that matches
the [non-normative regex from the HTML5
spec](https://html.spec.whatwg.org/multipage/forms.html#e-mail-state-(type=email)):



```
/^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?
(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/
```




Examples of email autolinks:

<a id="example-604"></a>

[Example 604](021-6-5-autolinks.md#example-604)Try It



```
<foo@bar.example.com>
```






```
<p><a href="mailto:foo@bar.example.com">foo@bar.example.com</a></p>
```




<a id="example-605"></a>

[Example 605](021-6-5-autolinks.md#example-605)Try It



```
<foo+special@Bar.baz-bar0.com>
```






```
<p><a href="mailto:foo+special@Bar.baz-bar0.com">foo+special@Bar.baz-bar0.com</a></p>
```




Backslash-escapes do not work inside email autolinks:

<a id="example-606"></a>

[Example 606](021-6-5-autolinks.md#example-606)Try It



```
<foo\+@bar.example.com>
```






```
<p>&lt;foo+@bar.example.com&gt;</p>
```




These are not autolinks:

<a id="example-607"></a>

[Example 607](021-6-5-autolinks.md#example-607)Try It



```
<>
```






```
<p>&lt;&gt;</p>
```




<a id="example-608"></a>

[Example 608](021-6-5-autolinks.md#example-608)Try It



```
< https://foo.bar >
```






```
<p>&lt; https://foo.bar &gt;</p>
```




<a id="example-609"></a>

[Example 609](021-6-5-autolinks.md#example-609)Try It



```
<m:abc>
```






```
<p>&lt;m:abc&gt;</p>
```




<a id="example-610"></a>

[Example 610](021-6-5-autolinks.md#example-610)Try It



```
<foo.bar.baz>
```






```
<p>&lt;foo.bar.baz&gt;</p>
```




<a id="example-611"></a>

[Example 611](021-6-5-autolinks.md#example-611)Try It



```
https://example.com
```






```
<p>https://example.com</p>
```




<a id="example-612"></a>

[Example 612](021-6-5-autolinks.md#example-612)Try It



```
foo@bar.example.com
```






```
<p>foo@bar.example.com</p>
```




<a id="raw-html"></a>

## 6.6 Raw HTML

Text between `<` and `>` that looks like an HTML tag is parsed as a
raw HTML tag and will be rendered in HTML without escaping.
Tag and attribute names are not limited to current HTML tags,
so custom tags (and even, say, DocBook tags) may be used.

Here is the grammar for tags:

<a id="tag-name"></a>

A [tag name](021-6-5-autolinks.md#tag-name) consists of an ASCII letter
followed by zero or more ASCII letters, digits, or
hyphens (`-`).

<a id="attribute"></a>

An [attribute](021-6-5-autolinks.md#attribute) consists of spaces, tabs, and up to one line ending,
an [attribute name](021-6-5-autolinks.md#attribute-name), and an optional
[attribute value specification](021-6-5-autolinks.md#attribute-value-specification).

<a id="attribute-name"></a>

An [attribute name](021-6-5-autolinks.md#attribute-name)
consists of an ASCII letter, `_`, or `:`, followed by zero or more ASCII
letters, digits, `_`, `.`, `:`, or `-`. (Note: This is the XML
specification restricted to ASCII. HTML5 is laxer.)

<a id="attribute-value-specification"></a>

An [attribute value specification](021-6-5-autolinks.md#attribute-value-specification)
consists of optional spaces, tabs, and up to one line ending,
a `=` character, optional spaces, tabs, and up to one line ending,
and an [attribute value](021-6-5-autolinks.md#attribute-value).

<a id="attribute-value"></a>

An [attribute value](021-6-5-autolinks.md#attribute-value)
consists of an [unquoted attribute value](021-6-5-autolinks.md#unquoted-attribute-value),
a [single-quoted attribute value](021-6-5-autolinks.md#single-quoted-attribute-value), or a [double-quoted attribute value](021-6-5-autolinks.md#double-quoted-attribute-value).

<a id="unquoted-attribute-value"></a>

An [unquoted attribute value](021-6-5-autolinks.md#unquoted-attribute-value)
is a nonempty string of characters not
including spaces, tabs, line endings, `"`, `'`, `=`, `<`, `>`, or `` ` ``.

<a id="single-quoted-attribute-value"></a>

A [single-quoted attribute value](021-6-5-autolinks.md#single-quoted-attribute-value)
consists of `'`, zero or more
characters not including `'`, and a final `'`.

<a id="double-quoted-attribute-value"></a>

A [double-quoted attribute value](021-6-5-autolinks.md#double-quoted-attribute-value)
consists of `"`, zero or more
characters not including `"`, and a final `"`.

<a id="open-tag"></a>

An [open tag](021-6-5-autolinks.md#open-tag) consists of a `<` character, a [tag name](021-6-5-autolinks.md#tag-name),
zero or more [attributes](021-6-5-autolinks.md#attribute), optional spaces, tabs, and up to one line ending,
an optional `/` character, and a `>` character.

<a id="closing-tag"></a>

A [closing tag](021-6-5-autolinks.md#closing-tag) consists of the string `</`, a
[tag name](021-6-5-autolinks.md#tag-name), optional spaces, tabs, and up to one line ending, and the character
`>`.

<a id="html-comment"></a>

An [HTML comment](021-6-5-autolinks.md#html-comment) consists of `<!-->`, `<!--->`, or `<!--`, a string of
characters not including the string `-->`, and `-->` (see the
[HTML spec](https://html.spec.whatwg.org/multipage/parsing.html#markup-declaration-open-state)).

<a id="processing-instruction"></a>

A [processing instruction](021-6-5-autolinks.md#processing-instruction)
consists of the string `<?`, a string
of characters not including the string `?>`, and the string
`?>`.

<a id="declaration"></a>

A [declaration](021-6-5-autolinks.md#declaration) consists of the string `<!`, an ASCII letter, zero or more
characters not including the character `>`, and the character `>`.

<a id="cdata-section"></a>

A [CDATA section](021-6-5-autolinks.md#cdata-section) consists of
the string `<![CDATA[`, a string of characters not including the string
`]]>`, and the string `]]>`.

<a id="html-tag"></a>

An [HTML tag](021-6-5-autolinks.md#html-tag) consists of an [open tag](021-6-5-autolinks.md#open-tag), a [closing tag](021-6-5-autolinks.md#closing-tag),
an [HTML comment](021-6-5-autolinks.md#html-comment), a [processing instruction](021-6-5-autolinks.md#processing-instruction), a [declaration](021-6-5-autolinks.md#declaration),
or a [CDATA section](021-6-5-autolinks.md#cdata-section).

Here are some simple open tags:

<a id="example-613"></a>

[Example 613](021-6-5-autolinks.md#example-613)Try It



```
<a><bab><c2c>
```






```
<p><a><bab><c2c></p>
```




Empty elements:

<a id="example-614"></a>

[Example 614](021-6-5-autolinks.md#example-614)Try It



```
<a/><b2/>
```






```
<p><a/><b2/></p>
```




Whitespace is allowed:

<a id="example-615"></a>

[Example 615](021-6-5-autolinks.md#example-615)Try It



```
<a  /><b2
data="foo" >
```






```
<p><a  /><b2
data="foo" ></p>
```




With attributes:

<a id="example-616"></a>

[Example 616](021-6-5-autolinks.md#example-616)Try It



```
<a foo="bar" bam = 'baz <em>"</em>'
_boolean zoop:33=zoop:33 />
```






```
<p><a foo="bar" bam = 'baz <em>"</em>'
_boolean zoop:33=zoop:33 /></p>
```




Custom tag names can be used:

<a id="example-617"></a>

[Example 617](021-6-5-autolinks.md#example-617)Try It



```
Foo <responsive-image src="foo.jpg" />
```






```
<p>Foo <responsive-image src="foo.jpg" /></p>
```




Illegal tag names, not parsed as HTML:

<a id="example-618"></a>

[Example 618](021-6-5-autolinks.md#example-618)Try It



```
<33> <__>
```






```
<p>&lt;33&gt; &lt;__&gt;</p>
```




Illegal attribute names:

<a id="example-619"></a>

[Example 619](021-6-5-autolinks.md#example-619)Try It



```
<a h*#ref="hi">
```






```
<p>&lt;a h*#ref=&quot;hi&quot;&gt;</p>
```




Illegal attribute values:

<a id="example-620"></a>

[Example 620](021-6-5-autolinks.md#example-620)Try It



```
<a href="hi'> <a href=hi'>
```






```
<p>&lt;a href=&quot;hi'&gt; &lt;a href=hi'&gt;</p>
```




Illegal whitespace:

<a id="example-621"></a>

[Example 621](021-6-5-autolinks.md#example-621)Try It



```
< a><
foo><bar/ >
<foo bar=baz
bim!bop />
```






```
<p>&lt; a&gt;&lt;
foo&gt;&lt;bar/ &gt;
&lt;foo bar=baz
bim!bop /&gt;</p>
```




Missing whitespace:

<a id="example-622"></a>

[Example 622](021-6-5-autolinks.md#example-622)Try It



```
<a href='bar'title=title>
```






```
<p>&lt;a href='bar'title=title&gt;</p>
```




Closing tags:

<a id="example-623"></a>

[Example 623](021-6-5-autolinks.md#example-623)Try It



```
</a></foo >
```






```
<p></a></foo ></p>
```




Illegal attributes in closing tag:

<a id="example-624"></a>

[Example 624](021-6-5-autolinks.md#example-624)Try It



```
</a href="foo">
```






```
<p>&lt;/a href=&quot;foo&quot;&gt;</p>
```




Comments:

<a id="example-625"></a>

[Example 625](021-6-5-autolinks.md#example-625)Try It



```
foo <!-- this is a --
comment - with hyphens -->
```






```
<p>foo <!-- this is a --
comment - with hyphens --></p>
```




<a id="example-626"></a>

[Example 626](021-6-5-autolinks.md#example-626)Try It



```
foo <!--> foo -->

foo <!---> foo -->
```






```
<p>foo <!--> foo --&gt;</p>
<p>foo <!---> foo --&gt;</p>
```




