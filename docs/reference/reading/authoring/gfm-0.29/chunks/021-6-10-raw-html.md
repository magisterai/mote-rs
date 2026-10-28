# 6.10 Raw HTML

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](020-6-8-autolinks.md) · [Next](022-6-11-disallowed-raw-html-extension.md)

```
<p><a href="mailto:foo@bar.baz">mailto:foo@bar.baz</a></p>
<p><a href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a></p>
<p><a href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>.</p>
<p><a href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>/</p>
<p>mailto:a.b-c_d@a.b-</p>
<p>mailto:a.b-c_d@a.b_</p>
<p><a href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a></p>
<p><a href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a>.</p>
```




A described in the [specification](https://datatracker.ietf.org/doc/rfc7622/)
`xmpp` offers an optional `/` followed by a resource. The resource can contain
all alphanumeric characters, as well as `@` and `.`.

<a id="example-634"></a>

[Example 634](021-6-10-raw-html.md#example-634)



```
xmpp:foo@bar.baz/txt

xmpp:foo@bar.baz/txt@bin

xmpp:foo@bar.baz/txt@bin.com
```






```
<p><a href="xmpp:foo@bar.baz/txt">xmpp:foo@bar.baz/txt</a></p>
<p><a href="xmpp:foo@bar.baz/txt@bin">xmpp:foo@bar.baz/txt@bin</a></p>
<p><a href="xmpp:foo@bar.baz/txt@bin.com">xmpp:foo@bar.baz/txt@bin.com</a></p>
```




Further `/` characters are not considered part of the domain:

<a id="example-635"></a>

[Example 635](021-6-10-raw-html.md#example-635)



```
xmpp:foo@bar.baz/txt/bin
```






```
<p><a href="xmpp:foo@bar.baz/txt">xmpp:foo@bar.baz/txt</a>/bin</p>
```




<a id="raw-html"></a>

## 6.10 Raw HTML

Text between `<` and `>` that looks like an HTML tag is parsed as a
raw HTML tag and will be rendered in HTML without escaping.
Tag and attribute names are not limited to current HTML tags,
so custom tags (and even, say, DocBook tags) may be used.

Here is the grammar for tags:

<a id="tag-name"></a>

A [tag name](021-6-10-raw-html.md#tag-name) consists of an ASCII letter
followed by zero or more ASCII letters, digits, or
hyphens (`-`).

<a id="attribute"></a>

An [attribute](021-6-10-raw-html.md#attribute) consists of [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace),
an [attribute name](021-6-10-raw-html.md#attribute-name), and an optional
[attribute value specification](021-6-10-raw-html.md#attribute-value-specification).

<a id="attribute-name"></a>

An [attribute name](021-6-10-raw-html.md#attribute-name)
consists of an ASCII letter, `_`, or `:`, followed by zero or more ASCII
letters, digits, `_`, `.`, `:`, or `-`. (Note: This is the XML
specification restricted to ASCII. HTML5 is laxer.)

<a id="attribute-value-specification"></a>

An [attribute value specification](021-6-10-raw-html.md#attribute-value-specification)
consists of optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace),
a `=` character, optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), and an [attribute
value](021-6-10-raw-html.md#attribute-value).

<a id="attribute-value"></a>

An [attribute value](021-6-10-raw-html.md#attribute-value)
consists of an [unquoted attribute value](021-6-10-raw-html.md#unquoted-attribute-value),
a [single-quoted attribute value](021-6-10-raw-html.md#single-quoted-attribute-value), or a [double-quoted attribute value](021-6-10-raw-html.md#double-quoted-attribute-value).

<a id="unquoted-attribute-value"></a>

An [unquoted attribute value](021-6-10-raw-html.md#unquoted-attribute-value)
is a nonempty string of characters not
including [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), `"`, `'`, `=`, `<`, `>`, or `` ` ``.

<a id="single-quoted-attribute-value"></a>

A [single-quoted attribute value](021-6-10-raw-html.md#single-quoted-attribute-value)
consists of `'`, zero or more
characters not including `'`, and a final `'`.

<a id="double-quoted-attribute-value"></a>

A [double-quoted attribute value](021-6-10-raw-html.md#double-quoted-attribute-value)
consists of `"`, zero or more
characters not including `"`, and a final `"`.

<a id="open-tag"></a>

An [open tag](021-6-10-raw-html.md#open-tag) consists of a `<` character, a [tag name](021-6-10-raw-html.md#tag-name),
zero or more [attributes](021-6-10-raw-html.md#attribute), optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), an optional `/`
character, and a `>` character.

<a id="closing-tag"></a>

A [closing tag](021-6-10-raw-html.md#closing-tag) consists of the string `</`, a
[tag name](021-6-10-raw-html.md#tag-name), optional [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), and the character `>`.

<a id="html-comment"></a>

An [HTML comment](021-6-10-raw-html.md#html-comment) consists of `<!--` + *text* + `-->`,
where *text* does not start with `>` or `->`, does not end with `-`,
and does not contain `--`. (See the
[HTML5 spec](http://www.w3.org/TR/html5/syntax.html#comments).)

<a id="processing-instruction"></a>

A [processing instruction](021-6-10-raw-html.md#processing-instruction)
consists of the string `<?`, a string
of characters not including the string `?>`, and the string
`?>`.

<a id="declaration"></a>

A [declaration](021-6-10-raw-html.md#declaration) consists of the
string `<!`, a name consisting of one or more uppercase ASCII letters,
[whitespace](002-1-3-why-is-a-spec-needed.md#whitespace), a string of characters not including the
character `>`, and the character `>`.

<a id="cdata-section"></a>

A [CDATA section](021-6-10-raw-html.md#cdata-section) consists of
the string `<![CDATA[`, a string of characters not including the string
`]]>`, and the string `]]>`.

<a id="html-tag"></a>

An [HTML tag](021-6-10-raw-html.md#html-tag) consists of an [open tag](021-6-10-raw-html.md#open-tag), a [closing tag](021-6-10-raw-html.md#closing-tag),
an [HTML comment](021-6-10-raw-html.md#html-comment), a [processing instruction](021-6-10-raw-html.md#processing-instruction), a [declaration](021-6-10-raw-html.md#declaration),
or a [CDATA section](021-6-10-raw-html.md#cdata-section).

Here are some simple open tags:

<a id="example-636"></a>

[Example 636](021-6-10-raw-html.md#example-636)



```
<a><bab><c2c>
```






```
<p><a><bab><c2c></p>
```




Empty elements:

<a id="example-637"></a>

[Example 637](021-6-10-raw-html.md#example-637)



```
<a/><b2/>
```






```
<p><a/><b2/></p>
```




[Whitespace](002-1-3-why-is-a-spec-needed.md#whitespace) is allowed:

<a id="example-638"></a>

[Example 638](021-6-10-raw-html.md#example-638)



```
<a  /><b2
data="foo" >
```






```
<p><a  /><b2
data="foo" ></p>
```




With attributes:

<a id="example-639"></a>

[Example 639](021-6-10-raw-html.md#example-639)



```
<a foo="bar" bam = 'baz <em>"</em>'
_boolean zoop:33=zoop:33 />
```






```
<p><a foo="bar" bam = 'baz <em>"</em>'
_boolean zoop:33=zoop:33 /></p>
```




Custom tag names can be used:

<a id="example-640"></a>

[Example 640](021-6-10-raw-html.md#example-640)



```
Foo <responsive-image src="foo.jpg" />
```






```
<p>Foo <responsive-image src="foo.jpg" /></p>
```




Illegal tag names, not parsed as HTML:

<a id="example-641"></a>

[Example 641](021-6-10-raw-html.md#example-641)



```
<33> <__>
```






```
<p>&lt;33&gt; &lt;__&gt;</p>
```




Illegal attribute names:

<a id="example-642"></a>

[Example 642](021-6-10-raw-html.md#example-642)



```
<a h*#ref="hi">
```






```
<p>&lt;a h*#ref=&quot;hi&quot;&gt;</p>
```




Illegal attribute values:

<a id="example-643"></a>

[Example 643](021-6-10-raw-html.md#example-643)



```
<a href="hi'> <a href=hi'>
```






```
<p>&lt;a href=&quot;hi'&gt; &lt;a href=hi'&gt;</p>
```




Illegal [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace):

<a id="example-644"></a>

[Example 644](021-6-10-raw-html.md#example-644)



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




Missing [whitespace](002-1-3-why-is-a-spec-needed.md#whitespace):

<a id="example-645"></a>

[Example 645](021-6-10-raw-html.md#example-645)



```
<a href='bar'title=title>
```






```
<p>&lt;a href='bar'title=title&gt;</p>
```




Closing tags:

<a id="example-646"></a>

[Example 646](021-6-10-raw-html.md#example-646)



```
</a></foo >
```






```
<p></a></foo ></p>
```




Illegal attributes in closing tag:

<a id="example-647"></a>

[Example 647](021-6-10-raw-html.md#example-647)



```
</a href="foo">
```






```
<p>&lt;/a href=&quot;foo&quot;&gt;</p>
```




Comments:

<a id="example-648"></a>

[Example 648](021-6-10-raw-html.md#example-648)



```
foo <!-- this is a
comment - with hyphen -->
```






```
<p>foo <!-- this is a
comment - with hyphen --></p>
```




<a id="example-649"></a>

[Example 649](021-6-10-raw-html.md#example-649)



```
foo <!-- not a comment -- two hyphens -->
```






```
<p>foo &lt;!-- not a comment -- two hyphens --&gt;</p>
```




Not comments:

<a id="example-650"></a>

[Example 650](021-6-10-raw-html.md#example-650)



```
foo <!--> foo -->

foo <!-- foo--->
```






```
<p>foo &lt;!--&gt; foo --&gt;</p>
<p>foo &lt;!-- foo---&gt;</p>
```




Processing instructions:

<a id="example-651"></a>

[Example 651](021-6-10-raw-html.md#example-651)



```
foo <?php echo $a; ?>
```






```
<p>foo <?php echo $a; ?></p>
```




Declarations:

<a id="example-652"></a>

[Example 652](021-6-10-raw-html.md#example-652)



```
foo <!ELEMENT br EMPTY>
```






```
<p>foo <!ELEMENT br EMPTY></p>
```




CDATA sections:

<a id="example-653"></a>

[Example 653](021-6-10-raw-html.md#example-653)



```
foo <![CDATA[>&<]]>
```






```
<p>foo <![CDATA[>&<]]></p>
```




Entity and numeric character references are preserved in HTML
attributes:

<a id="example-654"></a>

[Example 654](021-6-10-raw-html.md#example-654)



```
foo <a href="&ouml;">
```






```
<p>foo <a href="&ouml;"></p>
```




Backslash escapes do not work in HTML attributes:

<a id="example-655"></a>

[Example 655](021-6-10-raw-html.md#example-655)



```
foo <a href="\*">
```






```
<p>foo <a href="\*"></p>
```




<a id="example-656"></a>

[Example 656](021-6-10-raw-html.md#example-656)



```
<a href="\"">
```






```
<p>&lt;a href=&quot;&quot;&quot;&gt;</p>
```




