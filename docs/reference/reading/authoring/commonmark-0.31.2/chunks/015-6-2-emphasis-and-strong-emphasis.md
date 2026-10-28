# 6.2 Emphasis and strong emphasis

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-6-inlines.md) · [Next](016-continuation-16.md)

<a id="emphasis-and-strong-emphasis"></a>

## 6.2 Emphasis and strong emphasis

John Gruber’s original [Markdown syntax
description](https://daringfireball.net/projects/markdown/syntax#em) says:

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

First, some definitions. A [delimiter run](015-6-2-emphasis-and-strong-emphasis.md#delimiter-run) is either
a sequence of one or more `*` characters that is not preceded or
followed by a non-backslash-escaped `*` character, or a sequence
of one or more `_` characters that is not preceded or followed by
a non-backslash-escaped `_` character.

<a id="left-flanking-delimiter-run"></a>

A [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run) is
a [delimiter run](015-6-2-emphasis-and-strong-emphasis.md#delimiter-run) that is (1) not followed by [Unicode whitespace](002-1-3-about-this-document.md#unicode-whitespace),
and either (2a) not followed by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character), or
(2b) followed by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character) and
preceded by [Unicode whitespace](002-1-3-about-this-document.md#unicode-whitespace) or a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character).
For purposes of this definition, the beginning and the end of
the line count as Unicode whitespace.

<a id="right-flanking-delimiter-run"></a>

A [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run) is
a [delimiter run](015-6-2-emphasis-and-strong-emphasis.md#delimiter-run) that is (1) not preceded by [Unicode whitespace](002-1-3-about-this-document.md#unicode-whitespace),
and either (2a) not preceded by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character), or
(2b) preceded by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character) and
followed by [Unicode whitespace](002-1-3-about-this-document.md#unicode-whitespace) or a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character).
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
[vfmd](https://web.archive.org/web/20220608143320/http://www.vfmd.org/vfmd-spec/specification/#procedure-for-identifying-emphasis-tags).
vfmd uses the terminology “emphasis indicator string” instead of “delimiter
run,” and its rules for distinguishing left- and right-flanking runs
are a bit more complex than the ones given here.)

The following rules define emphasis and strong emphasis:

1. <a id="can-open-emphasis"></a>

   A single `*` character [can open emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-open-emphasis)
   iff (if and only if) it is part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run).
2. A single `_` character [can open emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-open-emphasis) iff
   it is part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run)
   and either (a) not part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run)
   or (b) part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run)
   preceded by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character).
3. <a id="can-close-emphasis"></a>

   A single `*` character [can close emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-close-emphasis)
   iff it is part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run).
4. A single `_` character [can close emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-close-emphasis) iff
   it is part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run)
   and either (a) not part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run)
   or (b) part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run)
   followed by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character).
5. <a id="can-open-strong-emphasis"></a>

   A double `**` [can open strong emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-open-strong-emphasis)
   iff it is part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run).
6. A double `__` [can open strong emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-open-strong-emphasis) iff
   it is part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run)
   and either (a) not part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run)
   or (b) part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run)
   preceded by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character).
7. <a id="can-close-strong-emphasis"></a>

   A double `**` [can close strong emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-close-strong-emphasis)
   iff it is part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run).
8. A double `__` [can close strong emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-close-strong-emphasis) iff
   it is part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run)
   and either (a) not part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run)
   or (b) part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run)
   followed by a [Unicode punctuation character](002-1-3-about-this-document.md#unicode-punctuation-character).
9. Emphasis begins with a delimiter that [can open emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-open-emphasis) and ends
   with a delimiter that [can close emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-close-emphasis), and that uses the same
   character (`_` or `*`) as the opening delimiter. The
   opening and closing delimiters must belong to separate
   [delimiter runs](015-6-2-emphasis-and-strong-emphasis.md#delimiter-run). If one of the delimiters can both
   open and close emphasis, then the sum of the lengths of the
   delimiter runs containing the opening and closing delimiters
   must not be a multiple of 3 unless both lengths are
   multiples of 3.
10. Strong emphasis begins with a delimiter that
    [can open strong emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-open-strong-emphasis) and ends with a delimiter that
    [can close strong emphasis](015-6-2-emphasis-and-strong-emphasis.md#can-close-strong-emphasis), and that uses the same character
    (`_` or `*`) as the opening delimiter. The
    opening and closing delimiters must belong to separate
    [delimiter runs](015-6-2-emphasis-and-strong-emphasis.md#delimiter-run). If one of the delimiters can both open
    and close strong emphasis, then the sum of the lengths of
    the delimiter runs containing the opening and closing
    delimiters must not be a multiple of 3 unless both lengths
    are multiples of 3.
11. A literal `*` character cannot occur at the beginning or end of
    `*`-delimited emphasis or `**`-delimited strong emphasis, unless it
    is backslash-escaped.
12. A literal `_` character cannot occur at the beginning or end of
    `_`-delimited emphasis or `__`-delimited strong emphasis, unless it
    is backslash-escaped.

Where rules 1–12 above are compatible with multiple parsings,
the following principles resolve ambiguity:

13. The number of nestings should be minimized. Thus, for example,
    an interpretation `<strong>...</strong>` is always preferred to
    `<em><em>...</em></em>`.
14. An interpretation `<em><strong>...</strong></em>` is always
    preferred to `<strong><em>...</em></strong>`.
15. When two potential emphasis or strong emphasis spans overlap,
    so that the second begins before the first ends and ends after
    the first ends, the first takes precedence. Thus, for example,
    `*foo _bar* baz_` is parsed as `<em>foo _bar</em> baz_` rather
    than `*foo <em>bar* baz</em>`.
16. When there are two potential emphasis or strong emphasis spans
    with the same closing delimiter, the shorter one (the one that
    opens later) takes precedence. Thus, for example,
    `**foo **bar baz**` is parsed as `**foo <strong>bar baz</strong>`
    rather than `<strong>foo **bar baz</strong>`.
17. Inline code spans, links, images, and HTML tags group more tightly
    than emphasis. So, when there is a choice between an interpretation
    that contains one of these elements and one that does not, the
    former always wins. Thus, for example, `*[foo*](bar)` is
    parsed as `*<a href="bar">foo*</a>` rather than as
    `<em>[foo</em>](bar)`.

These rules can be illustrated through a series of examples.

Rule 1:

<a id="example-350"></a>

[Example 350](015-6-2-emphasis-and-strong-emphasis.md#example-350)Try It



```
*foo bar*
```






```
<p><em>foo bar</em></p>
```




This is not emphasis, because the opening `*` is followed by
whitespace, and hence not part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run):

<a id="example-351"></a>

[Example 351](015-6-2-emphasis-and-strong-emphasis.md#example-351)Try It



```
a * foo bar*
```






```
<p>a * foo bar*</p>
```




This is not emphasis, because the opening `*` is preceded
by an alphanumeric and followed by punctuation, and hence
not part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run):

<a id="example-352"></a>

[Example 352](015-6-2-emphasis-and-strong-emphasis.md#example-352)Try It



```
a*"foo"*
```






```
<p>a*&quot;foo&quot;*</p>
```




Unicode nonbreaking spaces count as whitespace, too:

<a id="example-353"></a>

[Example 353](015-6-2-emphasis-and-strong-emphasis.md#example-353)Try It



```
* a *
```






```
<p>* a *</p>
```




Unicode symbols count as punctuation, too:

<a id="example-354"></a>

[Example 354](015-6-2-emphasis-and-strong-emphasis.md#example-354)Try It



```
*$*alpha.

*£*bravo.

*€*charlie.
```






```
<p>*$*alpha.</p>
<p>*£*bravo.</p>
<p>*€*charlie.</p>
```




Intraword emphasis with `*` is permitted:

<a id="example-355"></a>

[Example 355](015-6-2-emphasis-and-strong-emphasis.md#example-355)Try It



```
foo*bar*
```






```
<p>foo<em>bar</em></p>
```




<a id="example-356"></a>

[Example 356](015-6-2-emphasis-and-strong-emphasis.md#example-356)Try It



```
5*6*78
```






```
<p>5<em>6</em>78</p>
```




Rule 2:

<a id="example-357"></a>

[Example 357](015-6-2-emphasis-and-strong-emphasis.md#example-357)Try It



```
_foo bar_
```






```
<p><em>foo bar</em></p>
```




This is not emphasis, because the opening `_` is followed by
whitespace:

<a id="example-358"></a>

[Example 358](015-6-2-emphasis-and-strong-emphasis.md#example-358)Try It



```
_ foo bar_
```






```
<p>_ foo bar_</p>
```




This is not emphasis, because the opening `_` is preceded
by an alphanumeric and followed by punctuation:

<a id="example-359"></a>

[Example 359](015-6-2-emphasis-and-strong-emphasis.md#example-359)Try It



```
a_"foo"_
```






```
<p>a_&quot;foo&quot;_</p>
```




Emphasis with `_` is not allowed inside words:

<a id="example-360"></a>

[Example 360](015-6-2-emphasis-and-strong-emphasis.md#example-360)Try It



```
foo_bar_
```






```
<p>foo_bar_</p>
```




<a id="example-361"></a>

[Example 361](015-6-2-emphasis-and-strong-emphasis.md#example-361)Try It



```
5_6_78
```






```
<p>5_6_78</p>
```




<a id="example-362"></a>

[Example 362](015-6-2-emphasis-and-strong-emphasis.md#example-362)Try It



```
пристаням_стремятся_
```






```
<p>пристаням_стремятся_</p>
```




