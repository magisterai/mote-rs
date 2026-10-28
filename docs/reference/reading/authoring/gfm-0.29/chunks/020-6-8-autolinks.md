# 6.8 Autolinks

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](019-6-7-images.md) · [Next](021-6-10-raw-html.md)

<a id="autolinks"></a>

## 6.8 Autolinks

<a id="autolink"></a>

[Autolink](020-6-8-autolinks.md#autolink)s are absolute URIs and email addresses inside
`<` and `>`. They are parsed as links, with the URL or email address
as the link label.

<a id="uri-autolink"></a>

A [URI autolink](020-6-8-autolinks.md#uri-autolink) consists of `<`, followed by an
[absolute URI](020-6-8-autolinks.md#absolute-uri) followed by `>`. It is parsed as
a link to the URI, with the URI as the link’s label.

<a id="absolute-uri"></a>

An [absolute URI](020-6-8-autolinks.md#absolute-uri),
for these purposes, consists of a [scheme](020-6-8-autolinks.md#scheme) followed by a colon (`:`)
followed by zero or more characters other than ASCII
[whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) and control characters, `<`, and `>`. If
the URI includes these characters, they must be percent-encoded
(e.g. `%20` for a space).

<a id="scheme"></a>

For purposes of this spec, a [scheme](020-6-8-autolinks.md#scheme) is any sequence
of 2–32 characters beginning with an ASCII letter and followed
by any combination of ASCII letters, digits, or the symbols plus
(“+”), period (“.”), or hyphen (“-”).

Here are some valid autolinks:

<a id="example-603"></a>

[Example 603](020-6-8-autolinks.md#example-603)



```
<http://foo.bar.baz>
```






```
<p><a href="http://foo.bar.baz">http://foo.bar.baz</a></p>
```




<a id="example-604"></a>

[Example 604](020-6-8-autolinks.md#example-604)



```
<http://foo.bar.baz/test?q=hello&id=22&boolean>
```






```
<p><a href="http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean">http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>
```




<a id="example-605"></a>

[Example 605](020-6-8-autolinks.md#example-605)



```
<irc://foo.bar:2233/baz>
```






```
<p><a href="irc://foo.bar:2233/baz">irc://foo.bar:2233/baz</a></p>
```




Uppercase is also fine:

<a id="example-606"></a>

[Example 606](020-6-8-autolinks.md#example-606)



```
<MAILTO:FOO@BAR.BAZ>
```






```
<p><a href="MAILTO:FOO@BAR.BAZ">MAILTO:FOO@BAR.BAZ</a></p>
```




Note that many strings that count as [absolute URIs](020-6-8-autolinks.md#absolute-uri) for
purposes of this spec are not valid URIs, because their
schemes are not registered or because of other problems
with their syntax:

<a id="example-607"></a>

[Example 607](020-6-8-autolinks.md#example-607)



```
<a+b+c:d>
```






```
<p><a href="a+b+c:d">a+b+c:d</a></p>
```




<a id="example-608"></a>

[Example 608](020-6-8-autolinks.md#example-608)



```
<made-up-scheme://foo,bar>
```






```
<p><a href="made-up-scheme://foo,bar">made-up-scheme://foo,bar</a></p>
```




<a id="example-609"></a>

[Example 609](020-6-8-autolinks.md#example-609)



```
<http://../>
```






```
<p><a href="http://../">http://../</a></p>
```




<a id="example-610"></a>

[Example 610](020-6-8-autolinks.md#example-610)



```
<localhost:5001/foo>
```






```
<p><a href="localhost:5001/foo">localhost:5001/foo</a></p>
```




Spaces are not allowed in autolinks:

<a id="example-611"></a>

[Example 611](020-6-8-autolinks.md#example-611)



```
<http://foo.bar/baz bim>
```






```
<p>&lt;http://foo.bar/baz bim&gt;</p>
```




Backslash-escapes do not work inside autolinks:

<a id="example-612"></a>

[Example 612](020-6-8-autolinks.md#example-612)



```
<http://example.com/\[\>
```






```
<p><a href="http://example.com/%5C%5B%5C">http://example.com/\[\</a></p>
```




<a id="email-autolink"></a>

An [email autolink](020-6-8-autolinks.md#email-autolink)
consists of `<`, followed by an [email address](020-6-8-autolinks.md#email-address),
followed by `>`. The link’s label is the email address,
and the URL is `mailto:` followed by the email address.

<a id="email-address"></a>

An [email address](020-6-8-autolinks.md#email-address),
for these purposes, is anything that matches
the [non-normative regex from the HTML5
spec](https://html.spec.whatwg.org/multipage/forms.html#e-mail-state-(type=email)):



```
/^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?
(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/
```




Examples of email autolinks:

<a id="example-613"></a>

[Example 613](020-6-8-autolinks.md#example-613)



```
<foo@bar.example.com>
```






```
<p><a href="mailto:foo@bar.example.com">foo@bar.example.com</a></p>
```




<a id="example-614"></a>

[Example 614](020-6-8-autolinks.md#example-614)



```
<foo+special@Bar.baz-bar0.com>
```






```
<p><a href="mailto:foo+special@Bar.baz-bar0.com">foo+special@Bar.baz-bar0.com</a></p>
```




Backslash-escapes do not work inside email autolinks:

<a id="example-615"></a>

[Example 615](020-6-8-autolinks.md#example-615)



```
<foo\+@bar.example.com>
```






```
<p>&lt;foo+@bar.example.com&gt;</p>
```




These are not autolinks:

<a id="example-616"></a>

[Example 616](020-6-8-autolinks.md#example-616)



```
<>
```






```
<p>&lt;&gt;</p>
```




<a id="example-617"></a>

[Example 617](020-6-8-autolinks.md#example-617)



```
< http://foo.bar >
```






```
<p>&lt; http://foo.bar &gt;</p>
```




<a id="example-618"></a>

[Example 618](020-6-8-autolinks.md#example-618)



```
<m:abc>
```






```
<p>&lt;m:abc&gt;</p>
```




<a id="example-619"></a>

[Example 619](020-6-8-autolinks.md#example-619)



```
<foo.bar.baz>
```






```
<p>&lt;foo.bar.baz&gt;</p>
```




<a id="example-620"></a>

[Example 620](020-6-8-autolinks.md#example-620)



```
http://example.com
```






```
<p>http://example.com</p>
```




<a id="example-621"></a>

[Example 621](020-6-8-autolinks.md#example-621)



```
foo@bar.example.com
```






```
<p>foo@bar.example.com</p>
```




<a id="autolinks-extension-"></a>

## 6.9 Autolinks (extension)

GFM enables the `autolink` extension, where autolinks will be recognised in a
greater number of conditions.

[Autolink](020-6-8-autolinks.md#autolink)s can also be constructed without requiring the use of `<` and to `>`
to delimit them, although they will be recognized under a smaller set of
circumstances. All such recognized autolinks can only come at the beginning of
a line, after whitespace, or any of the delimiting characters `*`, `_`, `~`,
and `(`.

<a id="extended-www-autolink"></a>

<a id="valid-domain"></a>

An [extended www autolink](020-6-8-autolinks.md#extended-www-autolink) will be recognized
when the text `www.` is found followed by a [valid domain](020-6-8-autolinks.md#valid-domain).
A [valid domain](020-6-8-autolinks.md#valid-domain) consists of segments
of alphanumeric characters, underscores (`_`) and hyphens (`-`)
separated by periods (`.`).
There must be at least one period,
and no underscores may be present in the last two segments of the domain.

The scheme `http` will be inserted automatically:

<a id="example-622"></a>

[Example 622](020-6-8-autolinks.md#example-622)



```
www.commonmark.org
```






```
<p><a href="http://www.commonmark.org">www.commonmark.org</a></p>
```




After a [valid domain](020-6-8-autolinks.md#valid-domain), zero or more non-space non-`<` characters may follow:

<a id="example-623"></a>

[Example 623](020-6-8-autolinks.md#example-623)



```
Visit www.commonmark.org/help for more information.
```






```
<p>Visit <a href="http://www.commonmark.org/help">www.commonmark.org/help</a> for more information.</p>
```




<a id="extended-autolink-path-validation"></a>

We then apply [extended autolink path validation](020-6-8-autolinks.md#extended-autolink-path-validation) as follows:

Trailing punctuation (specifically, `?`, `!`, `.`, `,`, `:`, `*`, `_`, and `~`)
will not be considered part of the autolink, though they may be included in the
interior of the link:

<a id="example-624"></a>

[Example 624](020-6-8-autolinks.md#example-624)



```
Visit www.commonmark.org.

Visit www.commonmark.org/a.b.
```






```
<p>Visit <a href="http://www.commonmark.org">www.commonmark.org</a>.</p>
<p>Visit <a href="http://www.commonmark.org/a.b">www.commonmark.org/a.b</a>.</p>
```




When an autolink ends in `)`, we scan the entire autolink for the total number
of parentheses. If there is a greater number of closing parentheses than
opening ones, we don’t consider the unmatched trailing parentheses part of the
autolink, in order to facilitate including an autolink inside a parenthesis:

<a id="example-625"></a>

[Example 625](020-6-8-autolinks.md#example-625)



```
www.google.com/search?q=Markup+(business)

www.google.com/search?q=Markup+(business)))

(www.google.com/search?q=Markup+(business))

(www.google.com/search?q=Markup+(business)
```






```
<p><a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
<p><a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>))</p>
<p>(<a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>)</p>
<p>(<a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
```




This check is only done when the link ends in a closing parentheses `)`, so if
the only parentheses are in the interior of the autolink, no special rules are
applied:

<a id="example-626"></a>

[Example 626](020-6-8-autolinks.md#example-626)



```
www.google.com/search?q=(business))+ok
```






```
<p><a href="http://www.google.com/search?q=(business))+ok">www.google.com/search?q=(business))+ok</a></p>
```




If an autolink ends in a semicolon (`;`), we check to see if it appears to
resemble an [entity reference](013-6-inlines.md#entity-references); if the preceding text is `&`
followed by one or more alphanumeric characters. If so, it is excluded from
the autolink:

<a id="example-627"></a>

[Example 627](020-6-8-autolinks.md#example-627)



```
www.google.com/search?q=commonmark&hl=en

www.google.com/search?q=commonmark&hl;
```






```
<p><a href="http://www.google.com/search?q=commonmark&amp;hl=en">www.google.com/search?q=commonmark&amp;hl=en</a></p>
<p><a href="http://www.google.com/search?q=commonmark">www.google.com/search?q=commonmark</a>&amp;hl;</p>
```




`<` immediately ends an autolink.

<a id="example-628"></a>

[Example 628](020-6-8-autolinks.md#example-628)



```
www.commonmark.org/he<lp
```






```
<p><a href="http://www.commonmark.org/he">www.commonmark.org/he</a>&lt;lp</p>
```




<a id="extended-url-autolink"></a>

An [extended url autolink](020-6-8-autolinks.md#extended-url-autolink) will be recognised when one of the schemes
`http://`, or `https://`, followed by a [valid domain](020-6-8-autolinks.md#valid-domain), then zero or
more non-space non-`<` characters according to
[extended autolink path validation](020-6-8-autolinks.md#extended-autolink-path-validation):

<a id="example-629"></a>

[Example 629](020-6-8-autolinks.md#example-629)



```
http://commonmark.org

(Visit https://encrypted.google.com/search?q=Markup+(business))
```






```
<p><a href="http://commonmark.org">http://commonmark.org</a></p>
<p>(Visit <a href="https://encrypted.google.com/search?q=Markup+(business)">https://encrypted.google.com/search?q=Markup+(business)</a>)</p>
```




<a id="extended-email-autolink"></a>

An [extended email autolink](020-6-8-autolinks.md#extended-email-autolink) will be recognised when an email address is
recognised within any text node. Email addresses are recognised according to
the following rules:

- One ore more characters which are alphanumeric, or `.`, `-`, `_`, or `+`.
- An `@` symbol.
- One or more characters which are alphanumeric, or `-` or `_`,
  separated by periods (`.`).
  There must be at least one period.
  The last character must not be one of `-` or `_`.

The scheme `mailto:` will automatically be added to the generated link:

<a id="example-630"></a>

[Example 630](020-6-8-autolinks.md#example-630)



```
foo@bar.baz
```






```
<p><a href="mailto:foo@bar.baz">foo@bar.baz</a></p>
```




`+` can occur before the `@`, but not after.

<a id="example-631"></a>

[Example 631](020-6-8-autolinks.md#example-631)



```
hello@mail+xyz.example isn't valid, but hello+xyz@mail.example is.
```






```
<p>hello@mail+xyz.example isn't valid, but <a href="mailto:hello+xyz@mail.example">hello+xyz@mail.example</a> is.</p>
```




`.`, `-`, and `_` can occur on both sides of the `@`, but only `.` may occur at
the end of the email address, in which case it will not be considered part of
the address:

<a id="example-632"></a>

[Example 632](020-6-8-autolinks.md#example-632)



```
a.b-c_d@a.b

a.b-c_d@a.b.

a.b-c_d@a.b-

a.b-c_d@a.b_
```






```
<p><a href="mailto:a.b-c_d@a.b">a.b-c_d@a.b</a></p>
<p><a href="mailto:a.b-c_d@a.b">a.b-c_d@a.b</a>.</p>
<p>a.b-c_d@a.b-</p>
<p>a.b-c_d@a.b_</p>
```




<a id="extended-protocol-autolink"></a>

An [extended protocol autolink](020-6-8-autolinks.md#extended-protocol-autolink) will be recognised when a protocol is
recognised within any text node. Valid protocols are:

- `mailto:`
- `xmpp:`

The scheme of the protocol will automatically be added to the generated link.
All the rules of email address autolinking apply.

<a id="example-633"></a>

[Example 633](020-6-8-autolinks.md#example-633)



```
mailto:foo@bar.baz

mailto:a.b-c_d@a.b

mailto:a.b-c_d@a.b.

mailto:a.b-c_d@a.b/

mailto:a.b-c_d@a.b-

mailto:a.b-c_d@a.b_

xmpp:foo@bar.baz

xmpp:foo@bar.baz.
```






