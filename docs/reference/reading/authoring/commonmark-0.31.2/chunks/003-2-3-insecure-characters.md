# 2.3 Insecure characters

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-1-3-about-this-document.md) · [Next](004-3-blocks-and-inlines.md)

<a id="insecure-characters"></a>

## 2.3 Insecure characters

For security reasons, the Unicode character `U+0000` must be replaced
with the REPLACEMENT CHARACTER (`U+FFFD`).

<a id="backslash-escapes"></a>

## 2.4 Backslash escapes

Any ASCII punctuation character may be backslash-escaped:

<a id="example-12"></a>

[Example 12](003-2-3-insecure-characters.md#example-12)Try It



```
\!\"\#\$\%\&\'\(\)\*\+\,\-\.\/\:\;\<\=\>\?\@\[\\\]\^\_\`\{\|\}\~
```






```
<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</p>
```




Backslashes before other characters are treated as literal
backslashes:

<a id="example-13"></a>

[Example 13](003-2-3-insecure-characters.md#example-13)Try It



```
\→\A\a\ \3\φ\«
```






```
<p>\→\A\a\ \3\φ\«</p>
```




Escaped characters are treated as regular characters and do
not have their usual Markdown meanings:

<a id="example-14"></a>

[Example 14](003-2-3-insecure-characters.md#example-14)Try It



```
\*not emphasized*
\<br/> not a tag
\[not a link](/foo)
\`not code`
1\. not a list
\* not a list
\# not a heading
\[foo]: /url "not a reference"
\&ouml; not a character entity
```






```
<p>*not emphasized*
&lt;br/&gt; not a tag
[not a link](/foo)
`not code`
1. not a list
* not a list
# not a heading
[foo]: /url &quot;not a reference&quot;
&amp;ouml; not a character entity</p>
```




If a backslash is itself escaped, the following character is not:

<a id="example-15"></a>

[Example 15](003-2-3-insecure-characters.md#example-15)Try It



```
\\*emphasis*
```






```
<p>\<em>emphasis</em></p>
```




A backslash at the end of the line is a [hard line break](022-6-7-hard-line-breaks.md#hard-line-break):

<a id="example-16"></a>

[Example 16](003-2-3-insecure-characters.md#example-16)Try It



```
foo\
bar
```






```
<p>foo<br />
bar</p>
```




Backslash escapes do not work in code blocks, code spans, autolinks, or
raw HTML:

<a id="example-17"></a>

[Example 17](003-2-3-insecure-characters.md#example-17)Try It



```
`` \[\` ``
```






```
<p><code>\[\`</code></p>
```




<a id="example-18"></a>

[Example 18](003-2-3-insecure-characters.md#example-18)Try It



```
    \[\]
```






```
<pre><code>\[\]
</code></pre>
```




<a id="example-19"></a>

[Example 19](003-2-3-insecure-characters.md#example-19)Try It



```
~~~
\[\]
~~~
```






```
<pre><code>\[\]
</code></pre>
```




<a id="example-20"></a>

[Example 20](003-2-3-insecure-characters.md#example-20)Try It



```
<https://example.com?find=\*>
```






```
<p><a href="https://example.com?find=%5C*">https://example.com?find=\*</a></p>
```




<a id="example-21"></a>

[Example 21](003-2-3-insecure-characters.md#example-21)Try It



```
<a href="/bar\/)">
```






```
<a href="/bar\/)">
```




But they work in all other contexts, including URLs and link titles,
link references, and [info strings](006-4-4-indented-code-blocks.md#info-string) in [fenced code blocks](006-4-4-indented-code-blocks.md#fenced-code-blocks):

<a id="example-22"></a>

[Example 22](003-2-3-insecure-characters.md#example-22)Try It



```
[foo](/bar\* "ti\*tle")
```






```
<p><a href="/bar*" title="ti*tle">foo</a></p>
```




<a id="example-23"></a>

[Example 23](003-2-3-insecure-characters.md#example-23)Try It



```
[foo]

[foo]: /bar\* "ti\*tle"
```






```
<p><a href="/bar*" title="ti*tle">foo</a></p>
```




<a id="example-24"></a>

[Example 24](003-2-3-insecure-characters.md#example-24)Try It



````
``` foo\+bar
foo
```
````






```
<pre><code class="language-foo+bar">foo
</code></pre>
```




<a id="entity-and-numeric-character-references"></a>

## 2.5 Entity and numeric character references

Valid HTML entity references and numeric character references
can be used in place of the corresponding Unicode character,
with the following exceptions:

- Entity and character references are not recognized in code
  blocks and code spans.
- Entity and character references cannot stand in place of
  special characters that define structural elements in
  CommonMark. For example, although `&#42;` can be used
  in place of a literal `*` character, `&#42;` cannot replace
  `*` in emphasis delimiters, bullet list markers, or thematic
  breaks.

Conforming CommonMark parsers need not store information about
whether a particular character was represented in the source
using a Unicode character or an entity reference.

<a id="entity-references"></a>

[Entity references](003-2-3-insecure-characters.md#entity-references) consist of `&` + any of the valid
HTML5 entity names + `;`. The
document [https://html.spec.whatwg.org/entities.json](https://html.spec.whatwg.org/entities.json)
is used as an authoritative source for the valid entity
references and their corresponding code points.

<a id="example-25"></a>

[Example 25](003-2-3-insecure-characters.md#example-25)Try It



```
&nbsp; &amp; &copy; &AElig; &Dcaron;
&frac34; &HilbertSpace; &DifferentialD;
&ClockwiseContourIntegral; &ngE;
```






```
<p>  &amp; © Æ Ď
¾ ℋ ⅆ
∲ ≧̸</p>
```




<a id="decimal-numeric-character-references"></a>

[Decimal numeric character
references](003-2-3-insecure-characters.md#decimal-numeric-character-references)
consist of `&#` + a string of 1–7 arabic digits + `;`. A
numeric character reference is parsed as the corresponding
Unicode character. Invalid Unicode code points will be replaced by
the REPLACEMENT CHARACTER (`U+FFFD`). For security reasons,
the code point `U+0000` will also be replaced by `U+FFFD`.

<a id="example-26"></a>

[Example 26](003-2-3-insecure-characters.md#example-26)Try It



```
&#35; &#1234; &#992; &#0;
```






```
<p># Ӓ Ϡ �</p>
```




<a id="hexadecimal-numeric-character-references"></a>

[Hexadecimal numeric character
references](003-2-3-insecure-characters.md#hexadecimal-numeric-character-references) consist of `&#` +
either `X` or `x` + a string of 1-6 hexadecimal digits + `;`.
They too are parsed as the corresponding Unicode character (this
time specified with a hexadecimal numeral instead of decimal).

<a id="example-27"></a>

[Example 27](003-2-3-insecure-characters.md#example-27)Try It



```
&#X22; &#XD06; &#xcab;
```






```
<p>&quot; ആ ಫ</p>
```




Here are some nonentities:

<a id="example-28"></a>

[Example 28](003-2-3-insecure-characters.md#example-28)Try It



```
&nbsp &x; &#; &#x;
&#87654321;
&#abcdef0;
&ThisIsNotDefined; &hi?;
```






```
<p>&amp;nbsp &amp;x; &amp;#; &amp;#x;
&amp;#87654321;
&amp;#abcdef0;
&amp;ThisIsNotDefined; &amp;hi?;</p>
```




Although HTML5 does accept some entity references
without a trailing semicolon (such as `&copy`), these are not
recognized here, because it makes the grammar too ambiguous:

<a id="example-29"></a>

[Example 29](003-2-3-insecure-characters.md#example-29)Try It



```
&copy
```






```
<p>&amp;copy</p>
```




Strings that are not on the list of HTML5 named entities are not
recognized as entity references either:

<a id="example-30"></a>

[Example 30](003-2-3-insecure-characters.md#example-30)Try It



```
&MadeUpEntity;
```






```
<p>&amp;MadeUpEntity;</p>
```




Entity and numeric character references are recognized in any
context besides code spans or code blocks, including
URLs, [link titles](018-6-3-links.md#link-title), and [fenced code block](006-4-4-indented-code-blocks.md#fenced-code-block) [info strings](006-4-4-indented-code-blocks.md#info-string):

<a id="example-31"></a>

[Example 31](003-2-3-insecure-characters.md#example-31)Try It



```
<a href="&ouml;&ouml;.html">
```






```
<a href="&ouml;&ouml;.html">
```




<a id="example-32"></a>

[Example 32](003-2-3-insecure-characters.md#example-32)Try It



```
[foo](/f&ouml;&ouml; "f&ouml;&ouml;")
```






```
<p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>
```




<a id="example-33"></a>

[Example 33](003-2-3-insecure-characters.md#example-33)Try It



```
[foo]

[foo]: /f&ouml;&ouml; "f&ouml;&ouml;"
```






```
<p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>
```




<a id="example-34"></a>

[Example 34](003-2-3-insecure-characters.md#example-34)Try It



````
``` f&ouml;&ouml;
foo
```
````






```
<pre><code class="language-föö">foo
</code></pre>
```




Entity and numeric character references are treated as literal
text in code spans and code blocks:

<a id="example-35"></a>

[Example 35](003-2-3-insecure-characters.md#example-35)Try It



```
`f&ouml;&ouml;`
```






```
<p><code>f&amp;ouml;&amp;ouml;</code></p>
```




<a id="example-36"></a>

[Example 36](003-2-3-insecure-characters.md#example-36)Try It



```
    f&ouml;f&ouml;
```






```
<pre><code>f&amp;ouml;f&amp;ouml;
</code></pre>
```




Entity and numeric character references cannot be used
in place of symbols indicating structure in CommonMark
documents.

<a id="example-37"></a>

[Example 37](003-2-3-insecure-characters.md#example-37)Try It



```
&#42;foo&#42;
*foo*
```






```
<p>*foo*
<em>foo</em></p>
```




<a id="example-38"></a>

[Example 38](003-2-3-insecure-characters.md#example-38)Try It



```
&#42; foo

* foo
```






```
<p>* foo</p>
<ul>
<li>foo</li>
</ul>
```




<a id="example-39"></a>

[Example 39](003-2-3-insecure-characters.md#example-39)Try It



```
foo&#10;&#10;bar
```






```
<p>foo

bar</p>
```




<a id="example-40"></a>

[Example 40](003-2-3-insecure-characters.md#example-40)Try It



```
&#9;foo
```






```
<p>→foo</p>
```




<a id="example-41"></a>

[Example 41](003-2-3-insecure-characters.md#example-41)Try It



```
[a](url &quot;tit&quot;)
```






```
<p>[a](url &quot;tit&quot;)</p>
```




