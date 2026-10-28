# 6 Inlines

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-5-2-1-motivation.md) · [Next](015-6-2-emphasis-and-strong-emphasis.md)

```
<ol>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
</ol>
<pre><code>3. c
</code></pre>
```




This is a loose list, because there is a blank line between
two of the list items:

<a id="example-314"></a>

[Example 314](014-6-inlines.md#example-314)Try It



```
- a
- b

- c
```






```
<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>c</p>
</li>
</ul>
```




So is this, with a empty second item:

<a id="example-315"></a>

[Example 315](014-6-inlines.md#example-315)Try It



```
* a
*

* c
```






```
<ul>
<li>
<p>a</p>
</li>
<li></li>
<li>
<p>c</p>
</li>
</ul>
```




These are loose lists, even though there are no blank lines between the items,
because one of the items directly contains two block-level elements
with a blank line between them:

<a id="example-316"></a>

[Example 316](014-6-inlines.md#example-316)Try It



```
- a
- b

  c
- d
```






```
<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
<p>c</p>
</li>
<li>
<p>d</p>
</li>
</ul>
```




<a id="example-317"></a>

[Example 317](014-6-inlines.md#example-317)Try It



```
- a
- b

  [ref]: /url
- d
```






```
<ul>
<li>
<p>a</p>
</li>
<li>
<p>b</p>
</li>
<li>
<p>d</p>
</li>
</ul>
```




This is a tight list, because the blank lines are in a code block:

<a id="example-318"></a>

[Example 318](014-6-inlines.md#example-318)Try It



````
- a
- ```
  b


  ```
- c
````






```
<ul>
<li>a</li>
<li>
<pre><code>b


</code></pre>
</li>
<li>c</li>
</ul>
```




This is a tight list, because the blank line is between two
paragraphs of a sublist. So the sublist is loose while
the outer list is tight:

<a id="example-319"></a>

[Example 319](014-6-inlines.md#example-319)Try It



```
- a
  - b

    c
- d
```






```
<ul>
<li>a
<ul>
<li>
<p>b</p>
<p>c</p>
</li>
</ul>
</li>
<li>d</li>
</ul>
```




This is a tight list, because the blank line is inside the
block quote:

<a id="example-320"></a>

[Example 320](014-6-inlines.md#example-320)Try It



```
* a
  > b
  >
* c
```






```
<ul>
<li>a
<blockquote>
<p>b</p>
</blockquote>
</li>
<li>c</li>
</ul>
```




This list is tight, because the consecutive block elements
are not separated by blank lines:

<a id="example-321"></a>

[Example 321](014-6-inlines.md#example-321)Try It



````
- a
  > b
  ```
  c
  ```
- d
````






```
<ul>
<li>a
<blockquote>
<p>b</p>
</blockquote>
<pre><code>c
</code></pre>
</li>
<li>d</li>
</ul>
```




A single-paragraph list is tight:

<a id="example-322"></a>

[Example 322](014-6-inlines.md#example-322)Try It



```
- a
```






```
<ul>
<li>a</li>
</ul>
```




<a id="example-323"></a>

[Example 323](014-6-inlines.md#example-323)Try It



```
- a
  - b
```






```
<ul>
<li>a
<ul>
<li>b</li>
</ul>
</li>
</ul>
```




This list is loose, because of the blank line between the
two block elements in the list item:

<a id="example-324"></a>

[Example 324](014-6-inlines.md#example-324)Try It



````
1. ```
   foo
   ```

   bar
````






```
<ol>
<li>
<pre><code>foo
</code></pre>
<p>bar</p>
</li>
</ol>
```




Here the outer list is loose, the inner list tight:

<a id="example-325"></a>

[Example 325](014-6-inlines.md#example-325)Try It



```
* foo
  * bar

  baz
```






```
<ul>
<li>
<p>foo</p>
<ul>
<li>bar</li>
</ul>
<p>baz</p>
</li>
</ul>
```




<a id="example-326"></a>

[Example 326](014-6-inlines.md#example-326)Try It



```
- a
  - b
  - c

- d
  - e
  - f
```






```
<ul>
<li>
<p>a</p>
<ul>
<li>b</li>
<li>c</li>
</ul>
</li>
<li>
<p>d</p>
<ul>
<li>e</li>
<li>f</li>
</ul>
</li>
</ul>
```




<a id="inlines"></a>

# 6 Inlines

Inlines are parsed sequentially from the beginning of the character
stream to the end (left to right, in left-to-right languages).
Thus, for example, in

<a id="example-327"></a>

[Example 327](014-6-inlines.md#example-327)Try It



```
`hi`lo`
```






```
<p><code>hi</code>lo`</p>
```




`hi` is parsed as code, leaving the backtick at the end as a literal
backtick.

<a id="code-spans"></a>

## 6.1 Code spans

<a id="backtick-string"></a>

A [backtick string](014-6-inlines.md#backtick-string)
is a string of one or more backtick characters (`` ` ``) that is neither
preceded nor followed by a backtick.

<a id="code-span"></a>

A [code span](014-6-inlines.md#code-span) begins with a backtick string and ends with
a backtick string of equal length. The contents of the code span are
the characters between these two backtick strings, normalized in the
following ways:

- First, [line endings](002-1-3-about-this-document.md#line-ending) are converted to [spaces](002-1-3-about-this-document.md#space).
- If the resulting string both begins *and* ends with a [space](002-1-3-about-this-document.md#space)
  character, but does not consist entirely of [space](002-1-3-about-this-document.md#space)
  characters, a single [space](002-1-3-about-this-document.md#space) character is removed from the
  front and back. This allows you to include code that begins
  or ends with backtick characters, which must be separated by
  whitespace from the opening or closing backtick strings.

This is a simple code span:

<a id="example-328"></a>

[Example 328](014-6-inlines.md#example-328)Try It



```
`foo`
```






```
<p><code>foo</code></p>
```




Here two backticks are used, because the code contains a backtick.
This example also illustrates stripping of a single leading and
trailing space:

<a id="example-329"></a>

[Example 329](014-6-inlines.md#example-329)Try It



```
`` foo ` bar ``
```






```
<p><code>foo ` bar</code></p>
```




This example shows the motivation for stripping leading and trailing
spaces:

<a id="example-330"></a>

[Example 330](014-6-inlines.md#example-330)Try It



```
` `` `
```






```
<p><code>``</code></p>
```




Note that only *one* space is stripped:

<a id="example-331"></a>

[Example 331](014-6-inlines.md#example-331)Try It



```
`  ``  `
```






```
<p><code> `` </code></p>
```




The stripping only happens if the space is on both
sides of the string:

<a id="example-332"></a>

[Example 332](014-6-inlines.md#example-332)Try It



```
` a`
```






```
<p><code> a</code></p>
```




Only [spaces](002-1-3-about-this-document.md#space), and not [unicode whitespace](002-1-3-about-this-document.md#unicode-whitespace) in general, are
stripped in this way:

<a id="example-333"></a>

[Example 333](014-6-inlines.md#example-333)Try It



```
` b `
```






```
<p><code> b </code></p>
```




No stripping occurs if the code span contains only spaces:

<a id="example-334"></a>

[Example 334](014-6-inlines.md#example-334)Try It



```
` `
`  `
```






```
<p><code> </code>
<code>  </code></p>
```




[Line endings](002-1-3-about-this-document.md#line-ending) are treated like spaces:

<a id="example-335"></a>

[Example 335](014-6-inlines.md#example-335)Try It



```
``
foo
bar  
baz
``
```






```
<p><code>foo bar   baz</code></p>
```




<a id="example-336"></a>

[Example 336](014-6-inlines.md#example-336)Try It



```
``
foo 
``
```






```
<p><code>foo </code></p>
```




Interior spaces are not collapsed:

<a id="example-337"></a>

[Example 337](014-6-inlines.md#example-337)Try It



```
`foo   bar 
baz`
```






```
<p><code>foo   bar  baz</code></p>
```




Note that browsers will typically collapse consecutive spaces
when rendering `<code>` elements, so it is recommended that
the following CSS be used:



```
code{white-space: pre-wrap;}
```




Note that backslash escapes do not work in code spans. All backslashes
are treated literally:

<a id="example-338"></a>

[Example 338](014-6-inlines.md#example-338)Try It



```
`foo\`bar`
```






```
<p><code>foo\</code>bar`</p>
```




Backslash escapes are never needed, because one can always choose a
string of *n* backtick characters as delimiters, where the code does
not contain any strings of exactly *n* backtick characters.

<a id="example-339"></a>

[Example 339](014-6-inlines.md#example-339)Try It



```
``foo`bar``
```






```
<p><code>foo`bar</code></p>
```




<a id="example-340"></a>

[Example 340](014-6-inlines.md#example-340)Try It



```
` foo `` bar `
```






```
<p><code>foo `` bar</code></p>
```




Code span backticks have higher precedence than any other inline
constructs except HTML tags and autolinks. Thus, for example, this is
not parsed as emphasized text, since the second `*` is part of a code
span:

<a id="example-341"></a>

[Example 341](014-6-inlines.md#example-341)Try It



```
*foo`*`
```






```
<p>*foo<code>*</code></p>
```




And this is not parsed as a link:

<a id="example-342"></a>

[Example 342](014-6-inlines.md#example-342)Try It



```
[not a `link](/foo`)
```






```
<p>[not a <code>link](/foo</code>)</p>
```




Code spans, HTML tags, and autolinks have the same precedence.
Thus, this is code:

<a id="example-343"></a>

[Example 343](014-6-inlines.md#example-343)Try It



```
`<a href="`">`
```






```
<p><code>&lt;a href=&quot;</code>&quot;&gt;`</p>
```




But this is an HTML tag:

<a id="example-344"></a>

[Example 344](014-6-inlines.md#example-344)Try It



```
<a href="`">`
```






```
<p><a href="`">`</p>
```




And this is code:

<a id="example-345"></a>

[Example 345](014-6-inlines.md#example-345)Try It



```
`<https://foo.bar.`baz>`
```






```
<p><code>&lt;https://foo.bar.</code>baz&gt;`</p>
```




But this is an autolink:

<a id="example-346"></a>

[Example 346](014-6-inlines.md#example-346)Try It



```
<https://foo.bar.`baz>`
```






```
<p><a href="https://foo.bar.%60baz">https://foo.bar.`baz</a>`</p>
```




When a backtick string is not closed by a matching backtick string,
we just have literal backticks:

<a id="example-347"></a>

[Example 347](014-6-inlines.md#example-347)Try It



````
```foo``
````






````
<p>```foo``</p>
````




<a id="example-348"></a>

[Example 348](014-6-inlines.md#example-348)Try It



```
`foo
```






```
<p>`foo</p>
```




The following case also illustrates the need for opening and
closing backtick strings to be equal in length:

<a id="example-349"></a>

[Example 349](014-6-inlines.md#example-349)Try It



```
`foo``bar``
```






```
<p>`foo<code>bar</code></p>
```




