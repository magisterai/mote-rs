# 6.3 Code spans

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-6-inlines.md) · [Next](015-continuation-15.md)

<a id="code-spans"></a>

## 6.3 Code spans

<a id="backtick-string"></a>

A [backtick string](014-6-3-code-spans.md#backtick-string)
is a string of one or more backtick characters (`` ` ``) that is neither
preceded nor followed by a backtick.

<a id="code-span"></a>

A [code span](014-6-3-code-spans.md#code-span) begins with a backtick string and ends with
a backtick string of equal length. The contents of the code span are
the characters between the two backtick strings, normalized in the
following ways:

- First, [line endings](002-1-3-why-is-a-spec-needed.md#line-ending) are converted to [spaces](002-1-3-why-is-a-spec-needed.md#space).
- If the resulting string both begins *and* ends with a [space](002-1-3-why-is-a-spec-needed.md#space)
  character, but does not consist entirely of [space](002-1-3-why-is-a-spec-needed.md#space)
  characters, a single [space](002-1-3-why-is-a-spec-needed.md#space) character is removed from the
  front and back. This allows you to include code that begins
  or ends with backtick characters, which must be separated by
  whitespace from the opening or closing backtick strings.

This is a simple code span:

<a id="example-338"></a>

[Example 338](014-6-3-code-spans.md#example-338)



```
`foo`
```






```
<p><code>foo</code></p>
```




Here two backticks are used, because the code contains a backtick.
This example also illustrates stripping of a single leading and
trailing space:

<a id="example-339"></a>

[Example 339](014-6-3-code-spans.md#example-339)



```
`` foo ` bar ``
```






```
<p><code>foo ` bar</code></p>
```




This example shows the motivation for stripping leading and trailing
spaces:

<a id="example-340"></a>

[Example 340](014-6-3-code-spans.md#example-340)



```
` `` `
```






```
<p><code>``</code></p>
```




Note that only *one* space is stripped:

<a id="example-341"></a>

[Example 341](014-6-3-code-spans.md#example-341)



```
`  ``  `
```






```
<p><code> `` </code></p>
```




The stripping only happens if the space is on both
sides of the string:

<a id="example-342"></a>

[Example 342](014-6-3-code-spans.md#example-342)



```
` a`
```






```
<p><code> a</code></p>
```




Only [spaces](002-1-3-why-is-a-spec-needed.md#space), and not [unicode whitespace](002-1-3-why-is-a-spec-needed.md#unicode-whitespace) in general, are
stripped in this way:

<a id="example-343"></a>

[Example 343](014-6-3-code-spans.md#example-343)



```
` b `
```






```
<p><code> b </code></p>
```




No stripping occurs if the code span contains only spaces:

<a id="example-344"></a>

[Example 344](014-6-3-code-spans.md#example-344)



```
` `
`  `
```






```
<p><code> </code>
<code>  </code></p>
```




[Line endings](002-1-3-why-is-a-spec-needed.md#line-ending) are treated like spaces:

<a id="example-345"></a>

[Example 345](014-6-3-code-spans.md#example-345)



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




<a id="example-346"></a>

[Example 346](014-6-3-code-spans.md#example-346)



```
``
foo 
``
```






```
<p><code>foo </code></p>
```




Interior spaces are not collapsed:

<a id="example-347"></a>

[Example 347](014-6-3-code-spans.md#example-347)



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

<a id="example-348"></a>

[Example 348](014-6-3-code-spans.md#example-348)



```
`foo\`bar`
```






```
<p><code>foo\</code>bar`</p>
```




Backslash escapes are never needed, because one can always choose a
string of *n* backtick characters as delimiters, where the code does
not contain any strings of exactly *n* backtick characters.

<a id="example-349"></a>

[Example 349](014-6-3-code-spans.md#example-349)



```
``foo`bar``
```






```
<p><code>foo`bar</code></p>
```




<a id="example-350"></a>

[Example 350](014-6-3-code-spans.md#example-350)



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

<a id="example-351"></a>

[Example 351](014-6-3-code-spans.md#example-351)



```
*foo`*`
```






```
<p>*foo<code>*</code></p>
```




And this is not parsed as a link:

<a id="example-352"></a>

[Example 352](014-6-3-code-spans.md#example-352)



```
[not a `link](/foo`)
```






```
<p>[not a <code>link](/foo</code>)</p>
```




Code spans, HTML tags, and autolinks have the same precedence.
Thus, this is code:

<a id="example-353"></a>

[Example 353](014-6-3-code-spans.md#example-353)



```
`<a href="`">`
```






```
<p><code>&lt;a href=&quot;</code>&quot;&gt;`</p>
```




But this is an HTML tag:

<a id="example-354"></a>

[Example 354](014-6-3-code-spans.md#example-354)



```
<a href="`">`
```






```
<p><a href="`">`</p>
```




And this is code:

<a id="example-355"></a>

[Example 355](014-6-3-code-spans.md#example-355)



```
`<http://foo.bar.`baz>`
```






```
<p><code>&lt;http://foo.bar.</code>baz&gt;`</p>
```




But this is an autolink:

<a id="example-356"></a>

[Example 356](014-6-3-code-spans.md#example-356)



```
<http://foo.bar.`baz>`
```






```
<p><a href="http://foo.bar.%60baz">http://foo.bar.`baz</a>`</p>
```




When a backtick string is not closed by a matching backtick string,
we just have literal backticks:

<a id="example-357"></a>

[Example 357](014-6-3-code-spans.md#example-357)



````
```foo``
````






````
<p>```foo``</p>
````




<a id="example-358"></a>

[Example 358](014-6-3-code-spans.md#example-358)



```
`foo
```






```
<p>`foo</p>
```




The following case also illustrates the need for opening and
closing backtick strings to be equal in length:

<a id="example-359"></a>

[Example 359](014-6-3-code-spans.md#example-359)



```
`foo``bar``
```






```
<p>`foo<code>bar</code></p>
```




<a id="emphasis-and-strong-emphasis"></a>

## 6.4 Emphasis and strong emphasis

John Gruber’s original [Markdown syntax
description](http://daringfireball.net/projects/markdown/syntax#em) says:

> Markdown treats asterisks (`*`) and underscores (`_`) as indicators of
> emphasis. Text wrapped with one `*` or `_` will be wrapped with an HTML
> `<em>` tag; double `*`’s or `_`’s will be wrapped with an HTML `<strong>`
> tag.

This is enough for most users, but these rules leave much undecided,
especially when it comes to nested emphasis. The original
`Markdown.pl` test suite makes it clear that triple `***` and
`___` delimiters can be used for strong emphasis, and most
implementations have also allowed the following patterns:



```
***strong emph***
***strong** in emph*
***emph* in strong**
**in strong *emph***
*in emph **strong***
```




The following patterns are less widely supported, but the intent
is clear and they are useful (especially in contexts like bibliography
entries):



```
*emph *with emph* in it*
**strong **with strong** in it**
```




Many implementations have also restricted intraword emphasis to
the `*` forms, to avoid unwanted emphasis in words containing
internal underscores. (It is best practice to put these in code
spans, but users often do not.)



```
internal emphasis: foo*bar*baz
no emphasis: foo_bar_baz
```




The rules given below capture all of these patterns, while allowing
for efficient parsing strategies that do not backtrack.

<a id="delimiter-run"></a>

First, some definitions. A [delimiter run](014-6-3-code-spans.md#delimiter-run) is either
a sequence of one or more `*` characters that is not preceded or
followed by a non-backslash-escaped `*` character, or a sequence
of one or more `_` characters that is not preceded or followed by
a non-backslash-escaped `_` character.

<a id="left-flanking-delimiter-run"></a>

A [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run) is
a [delimiter run](014-6-3-code-spans.md#delimiter-run) that is (1) not followed by [Unicode whitespace](002-1-3-why-is-a-spec-needed.md#unicode-whitespace),
and either (2a) not followed by a [punctuation character](002-1-3-why-is-a-spec-needed.md#punctuation-character), or
(2b) followed by a [punctuation character](002-1-3-why-is-a-spec-needed.md#punctuation-character) and
preceded by [Unicode whitespace](002-1-3-why-is-a-spec-needed.md#unicode-whitespace) or a [punctuation character](002-1-3-why-is-a-spec-needed.md#punctuation-character).
For purposes of this definition, the beginning and the end of
the line count as Unicode whitespace.

<a id="right-flanking-delimiter-run"></a>

A [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run) is
a [delimiter run](014-6-3-code-spans.md#delimiter-run) that is (1) not preceded by [Unicode whitespace](002-1-3-why-is-a-spec-needed.md#unicode-whitespace),
and either (2a) not preceded by a [punctuation character](002-1-3-why-is-a-spec-needed.md#punctuation-character), or
(2b) preceded by a [punctuation character](002-1-3-why-is-a-spec-needed.md#punctuation-character) and
followed by [Unicode whitespace](002-1-3-why-is-a-spec-needed.md#unicode-whitespace) or a [punctuation character](002-1-3-why-is-a-spec-needed.md#punctuation-character).
For purposes of this definition, the beginning and the end of
the line count as Unicode whitespace.

Here are some examples of delimiter runs.

- left-flanking but not right-flanking:



```
***abc
  _abc
**"abc"
 _"abc"
```



- right-flanking but not left-flanking:



```
 abc***
 abc_
"abc"**
"abc"_
```



- Both left and right-flanking:



```
 abc***def
"abc"_"def"
```



- Neither left nor right-flanking:



```
abc *** def
a _ b
```




(The idea of distinguishing left-flanking and right-flanking
delimiter runs based on the character before and the character
after comes from Roopesh Chander’s
[vfmd](http://www.vfmd.org/vfmd-spec/specification/#procedure-for-identifying-emphasis-tags).
vfmd uses the terminology “emphasis indicator string” instead of “delimiter
run,” and its rules for distinguishing left- and right-flanking runs
are a bit more complex than the ones given here.)

The following rules define emphasis and strong emphasis:

1. <a id="can-open-emphasis"></a>

   A single `*` character [can open emphasis](014-6-3-code-spans.md#can-open-emphasis)
   iff (if and only if) it is part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run).
2. A single `_` character [can open emphasis](014-6-3-code-spans.md#can-open-emphasis) iff
   it is part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run)
   and either (a) not part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run)
   or (b) part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run)
   preceded by punctuation.
3. <a id="can-close-emphasis"></a>

   A single `*` character [can close emphasis](014-6-3-code-spans.md#can-close-emphasis)
   iff it is part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run).
4. A single `_` character [can close emphasis](014-6-3-code-spans.md#can-close-emphasis) iff
   it is part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run)
   and either (a) not part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run)
   or (b) part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run)
   followed by punctuation.
5. <a id="can-open-strong-emphasis"></a>

   A double `**` [can open strong emphasis](014-6-3-code-spans.md#can-open-strong-emphasis)
   iff it is part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run).
6. A double `__` [can open strong emphasis](014-6-3-code-spans.md#can-open-strong-emphasis) iff
   it is part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run)
   and either (a) not part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run)
   or (b) part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run)
   preceded by punctuation.
7. <a id="can-close-strong-emphasis"></a>

