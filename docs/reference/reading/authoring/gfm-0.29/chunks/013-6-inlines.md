# 6 Inlines

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-5-3-task-list-items-extension.md) · [Next](014-6-3-code-spans.md)

<a id="inlines"></a>

# 6 Inlines

Inlines are parsed sequentially from the beginning of the character
stream to the end (left to right, in left-to-right languages).
Thus, for example, in

<a id="example-307"></a>

[Example 307](013-6-inlines.md#example-307)



```
`hi`lo`
```






```
<p><code>hi</code>lo`</p>
```




`hi` is parsed as code, leaving the backtick at the end as a literal
backtick.

<a id="backslash-escapes"></a>

## 6.1 Backslash escapes

Any ASCII punctuation character may be backslash-escaped:

<a id="example-308"></a>

[Example 308](013-6-inlines.md#example-308)



```
\!\"\#\$\%\&\'\(\)\*\+\,\-\.\/\:\;\<\=\>\?\@\[\\\]\^\_\`\{\|\}\~
```






```
<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\]^_`{|}~</p>
```




Backslashes before other characters are treated as literal
backslashes:

<a id="example-309"></a>

[Example 309](013-6-inlines.md#example-309)



```
\→\A\a\ \3\φ\«
```






```
<p>\→\A\a\ \3\φ\«</p>
```




Escaped characters are treated as regular characters and do
not have their usual Markdown meanings:

<a id="example-310"></a>

[Example 310](013-6-inlines.md#example-310)



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

<a id="example-311"></a>

[Example 311](013-6-inlines.md#example-311)



```
\\*emphasis*
```






```
<p>\<em>emphasis</em></p>
```




A backslash at the end of the line is a [hard line break](022-6-11-disallowed-raw-html-extension.md#hard-line-break):

<a id="example-312"></a>

[Example 312](013-6-inlines.md#example-312)



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

<a id="example-313"></a>

[Example 313](013-6-inlines.md#example-313)



```
`` \[\` ``
```






```
<p><code>\[\`</code></p>
```




<a id="example-314"></a>

[Example 314](013-6-inlines.md#example-314)



```
    \[\]
```






```
<pre><code>\[\]
</code></pre>
```




<a id="example-315"></a>

[Example 315](013-6-inlines.md#example-315)



```
~~~
\[\]
~~~
```






```
<pre><code>\[\]
</code></pre>
```




<a id="example-316"></a>

[Example 316](013-6-inlines.md#example-316)



```
<http://example.com?find=\*>
```






```
<p><a href="http://example.com?find=%5C*">http://example.com?find=\*</a></p>
```




<a id="example-317"></a>

[Example 317](013-6-inlines.md#example-317)



```
<a href="/bar\/)">
```






```
<a href="/bar\/)">
```




But they work in all other contexts, including URLs and link titles,
link references, and [info strings](005-4-4-indented-code-blocks.md#info-string) in [fenced code blocks](005-4-4-indented-code-blocks.md#fenced-code-blocks):

<a id="example-318"></a>

[Example 318](013-6-inlines.md#example-318)



```
[foo](/bar\* "ti\*tle")
```






```
<p><a href="/bar*" title="ti*tle">foo</a></p>
```




<a id="example-319"></a>

[Example 319](013-6-inlines.md#example-319)



```
[foo]

[foo]: /bar\* "ti\*tle"
```






```
<p><a href="/bar*" title="ti*tle">foo</a></p>
```




<a id="example-320"></a>

[Example 320](013-6-inlines.md#example-320)



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

## 6.2 Entity and numeric character references

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

[Entity references](013-6-inlines.md#entity-references) consist of `&` + any of the valid
HTML5 entity names + `;`. The
document [https://html.spec.whatwg.org/multipage/entities.json](https://html.spec.whatwg.org/multipage/entities.json)
is used as an authoritative source for the valid entity
references and their corresponding code points.

<a id="example-321"></a>

[Example 321](013-6-inlines.md#example-321)



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
references](013-6-inlines.md#decimal-numeric-character-references)
consist of `&#` + a string of 1–7 arabic digits + `;`. A
numeric character reference is parsed as the corresponding
Unicode character. Invalid Unicode code points will be replaced by
the REPLACEMENT CHARACTER (`U+FFFD`). For security reasons,
the code point `U+0000` will also be replaced by `U+FFFD`.

<a id="example-322"></a>

[Example 322](013-6-inlines.md#example-322)



```
&#35; &#1234; &#992; &#0;
```






```
<p># Ӓ Ϡ �</p>
```




<a id="hexadecimal-numeric-character-references"></a>

[Hexadecimal numeric character
references](013-6-inlines.md#hexadecimal-numeric-character-references) consist of `&#` +
either `X` or `x` + a string of 1-6 hexadecimal digits + `;`.
They too are parsed as the corresponding Unicode character (this
time specified with a hexadecimal numeral instead of decimal).

<a id="example-323"></a>

[Example 323](013-6-inlines.md#example-323)



```
&#X22; &#XD06; &#xcab;
```






```
<p>&quot; ആ ಫ</p>
```




Here are some nonentities:

<a id="example-324"></a>

[Example 324](013-6-inlines.md#example-324)



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

<a id="example-325"></a>

[Example 325](013-6-inlines.md#example-325)



```
&copy
```






```
<p>&amp;copy</p>
```




Strings that are not on the list of HTML5 named entities are not
recognized as entity references either:

<a id="example-326"></a>

[Example 326](013-6-inlines.md#example-326)



```
&MadeUpEntity;
```






```
<p>&amp;MadeUpEntity;</p>
```




Entity and numeric character references are recognized in any
context besides code spans or code blocks, including
URLs, [link titles](017-6-5-strikethrough-extension.md#link-title), and [fenced code block](005-4-4-indented-code-blocks.md#fenced-code-block) [info strings](005-4-4-indented-code-blocks.md#info-string):

<a id="example-327"></a>

[Example 327](013-6-inlines.md#example-327)



```
<a href="&ouml;&ouml;.html">
```






```
<a href="&ouml;&ouml;.html">
```




<a id="example-328"></a>

[Example 328](013-6-inlines.md#example-328)



```
[foo](/f&ouml;&ouml; "f&ouml;&ouml;")
```






```
<p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>
```




<a id="example-329"></a>

[Example 329](013-6-inlines.md#example-329)



```
[foo]

[foo]: /f&ouml;&ouml; "f&ouml;&ouml;"
```






```
<p><a href="/f%C3%B6%C3%B6" title="föö">foo</a></p>
```




<a id="example-330"></a>

[Example 330](013-6-inlines.md#example-330)



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

<a id="example-331"></a>

[Example 331](013-6-inlines.md#example-331)



```
`f&ouml;&ouml;`
```






```
<p><code>f&amp;ouml;&amp;ouml;</code></p>
```




<a id="example-332"></a>

[Example 332](013-6-inlines.md#example-332)



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

<a id="example-333"></a>

[Example 333](013-6-inlines.md#example-333)



```
&#42;foo&#42;
*foo*
```






```
<p>*foo*
<em>foo</em></p>
```




<a id="example-334"></a>

[Example 334](013-6-inlines.md#example-334)



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




<a id="example-335"></a>

[Example 335](013-6-inlines.md#example-335)



```
foo&#10;&#10;bar
```






```
<p>foo

bar</p>
```




<a id="example-336"></a>

[Example 336](013-6-inlines.md#example-336)



```
&#9;foo
```






```
<p>→foo</p>
```




<a id="example-337"></a>

[Example 337](013-6-inlines.md#example-337)



```
[a](url &quot;tit&quot;)
```






```
<p>[a](url &quot;tit&quot;)</p>
```




