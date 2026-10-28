# Continuation 15

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-6-3-code-spans.md) · [Next](016-continuation-16.md)

   A double `**` [can close strong emphasis](014-6-3-code-spans.md#can-close-strong-emphasis)
   iff it is part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run).
8. A double `__` [can close strong emphasis](014-6-3-code-spans.md#can-close-strong-emphasis) iff
   it is part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run)
   and either (a) not part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run)
   or (b) part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run)
   followed by punctuation.
9. Emphasis begins with a delimiter that [can open emphasis](014-6-3-code-spans.md#can-open-emphasis) and ends
   with a delimiter that [can close emphasis](014-6-3-code-spans.md#can-close-emphasis), and that uses the same
   character (`_` or `*`) as the opening delimiter. The
   opening and closing delimiters must belong to separate
   [delimiter runs](014-6-3-code-spans.md#delimiter-run). If one of the delimiters can both
   open and close emphasis, then the sum of the lengths of the
   delimiter runs containing the opening and closing delimiters
   must not be a multiple of 3 unless both lengths are
   multiples of 3.
10. Strong emphasis begins with a delimiter that
    [can open strong emphasis](014-6-3-code-spans.md#can-open-strong-emphasis) and ends with a delimiter that
    [can close strong emphasis](014-6-3-code-spans.md#can-close-strong-emphasis), and that uses the same character
    (`_` or `*`) as the opening delimiter. The
    opening and closing delimiters must belong to separate
    [delimiter runs](014-6-3-code-spans.md#delimiter-run). If one of the delimiters can both open
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

<a id="example-360"></a>

[Example 360](015-continuation-15.md#example-360)



```
*foo bar*
```






```
<p><em>foo bar</em></p>
```




This is not emphasis, because the opening `*` is followed by
whitespace, and hence not part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run):

<a id="example-361"></a>

[Example 361](015-continuation-15.md#example-361)



```
a * foo bar*
```






```
<p>a * foo bar*</p>
```




This is not emphasis, because the opening `*` is preceded
by an alphanumeric and followed by punctuation, and hence
not part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run):

<a id="example-362"></a>

[Example 362](015-continuation-15.md#example-362)



```
a*"foo"*
```






```
<p>a*&quot;foo&quot;*</p>
```




Unicode nonbreaking spaces count as whitespace, too:

<a id="example-363"></a>

[Example 363](015-continuation-15.md#example-363)



```
* a *
```






```
<p>* a *</p>
```




Intraword emphasis with `*` is permitted:

<a id="example-364"></a>

[Example 364](015-continuation-15.md#example-364)



```
foo*bar*
```






```
<p>foo<em>bar</em></p>
```




<a id="example-365"></a>

[Example 365](015-continuation-15.md#example-365)



```
5*6*78
```






```
<p>5<em>6</em>78</p>
```




Rule 2:

<a id="example-366"></a>

[Example 366](015-continuation-15.md#example-366)



```
_foo bar_
```






```
<p><em>foo bar</em></p>
```




This is not emphasis, because the opening `_` is followed by
whitespace:

<a id="example-367"></a>

[Example 367](015-continuation-15.md#example-367)



```
_ foo bar_
```






```
<p>_ foo bar_</p>
```




This is not emphasis, because the opening `_` is preceded
by an alphanumeric and followed by punctuation:

<a id="example-368"></a>

[Example 368](015-continuation-15.md#example-368)



```
a_"foo"_
```






```
<p>a_&quot;foo&quot;_</p>
```




Emphasis with `_` is not allowed inside words:

<a id="example-369"></a>

[Example 369](015-continuation-15.md#example-369)



```
foo_bar_
```






```
<p>foo_bar_</p>
```




<a id="example-370"></a>

[Example 370](015-continuation-15.md#example-370)



```
5_6_78
```






```
<p>5_6_78</p>
```




<a id="example-371"></a>

[Example 371](015-continuation-15.md#example-371)



```
пристаням_стремятся_
```






```
<p>пристаням_стремятся_</p>
```




Here `_` does not generate emphasis, because the first delimiter run
is right-flanking and the second left-flanking:

<a id="example-372"></a>

[Example 372](015-continuation-15.md#example-372)



```
aa_"bb"_cc
```






```
<p>aa_&quot;bb&quot;_cc</p>
```




This is emphasis, even though the opening delimiter is
both left- and right-flanking, because it is preceded by
punctuation:

<a id="example-373"></a>

[Example 373](015-continuation-15.md#example-373)



```
foo-_(bar)_
```






```
<p>foo-<em>(bar)</em></p>
```




Rule 3:

This is not emphasis, because the closing delimiter does
not match the opening delimiter:

<a id="example-374"></a>

[Example 374](015-continuation-15.md#example-374)



```
_foo*
```






```
<p>_foo*</p>
```




This is not emphasis, because the closing `*` is preceded by
whitespace:

<a id="example-375"></a>

[Example 375](015-continuation-15.md#example-375)



```
*foo bar *
```






```
<p>*foo bar *</p>
```




A newline also counts as whitespace:

<a id="example-376"></a>

[Example 376](015-continuation-15.md#example-376)



```
*foo bar
*
```






```
<p>*foo bar
*</p>
```




This is not emphasis, because the second `*` is
preceded by punctuation and followed by an alphanumeric
(hence it is not part of a [right-flanking delimiter run](014-6-3-code-spans.md#right-flanking-delimiter-run):

<a id="example-377"></a>

[Example 377](015-continuation-15.md#example-377)



```
*(*foo)
```






```
<p>*(*foo)</p>
```




The point of this restriction is more easily appreciated
with this example:

<a id="example-378"></a>

[Example 378](015-continuation-15.md#example-378)



```
*(*foo*)*
```






```
<p><em>(<em>foo</em>)</em></p>
```




Intraword emphasis with `*` is allowed:

<a id="example-379"></a>

[Example 379](015-continuation-15.md#example-379)



```
*foo*bar
```






```
<p><em>foo</em>bar</p>
```




Rule 4:

This is not emphasis, because the closing `_` is preceded by
whitespace:

<a id="example-380"></a>

[Example 380](015-continuation-15.md#example-380)



```
_foo bar _
```






```
<p>_foo bar _</p>
```




This is not emphasis, because the second `_` is
preceded by punctuation and followed by an alphanumeric:

<a id="example-381"></a>

[Example 381](015-continuation-15.md#example-381)



```
_(_foo)
```






```
<p>_(_foo)</p>
```




This is emphasis within emphasis:

<a id="example-382"></a>

[Example 382](015-continuation-15.md#example-382)



```
_(_foo_)_
```






```
<p><em>(<em>foo</em>)</em></p>
```




Intraword emphasis is disallowed for `_`:

<a id="example-383"></a>

[Example 383](015-continuation-15.md#example-383)



```
_foo_bar
```






```
<p>_foo_bar</p>
```




<a id="example-384"></a>

[Example 384](015-continuation-15.md#example-384)



```
_пристаням_стремятся
```






```
<p>_пристаням_стремятся</p>
```




<a id="example-385"></a>

[Example 385](015-continuation-15.md#example-385)



```
_foo_bar_baz_
```






```
<p><em>foo_bar_baz</em></p>
```




This is emphasis, even though the closing delimiter is
both left- and right-flanking, because it is followed by
punctuation:

<a id="example-386"></a>

[Example 386](015-continuation-15.md#example-386)



```
_(bar)_.
```






```
<p><em>(bar)</em>.</p>
```




Rule 5:

<a id="example-387"></a>

[Example 387](015-continuation-15.md#example-387)



```
**foo bar**
```






```
<p><strong>foo bar</strong></p>
```




This is not strong emphasis, because the opening delimiter is
followed by whitespace:

<a id="example-388"></a>

[Example 388](015-continuation-15.md#example-388)



```
** foo bar**
```






```
<p>** foo bar**</p>
```




This is not strong emphasis, because the opening `**` is preceded
by an alphanumeric and followed by punctuation, and hence
not part of a [left-flanking delimiter run](014-6-3-code-spans.md#left-flanking-delimiter-run):

<a id="example-389"></a>

[Example 389](015-continuation-15.md#example-389)



```
a**"foo"**
```






```
<p>a**&quot;foo&quot;**</p>
```




Intraword strong emphasis with `**` is permitted:

<a id="example-390"></a>

[Example 390](015-continuation-15.md#example-390)



```
foo**bar**
```






```
<p>foo<strong>bar</strong></p>
```




Rule 6:

<a id="example-391"></a>

[Example 391](015-continuation-15.md#example-391)



```
__foo bar__
```






```
<p><strong>foo bar</strong></p>
```




This is not strong emphasis, because the opening delimiter is
followed by whitespace:

<a id="example-392"></a>

[Example 392](015-continuation-15.md#example-392)



```
__ foo bar__
```






```
<p>__ foo bar__</p>
```




A newline counts as whitespace:

<a id="example-393"></a>

[Example 393](015-continuation-15.md#example-393)



```
__
foo bar__
```






```
<p>__
foo bar__</p>
```




This is not strong emphasis, because the opening `__` is preceded
by an alphanumeric and followed by punctuation:

<a id="example-394"></a>

[Example 394](015-continuation-15.md#example-394)



```
a__"foo"__
```






```
<p>a__&quot;foo&quot;__</p>
```




Intraword strong emphasis is forbidden with `__`:

<a id="example-395"></a>

[Example 395](015-continuation-15.md#example-395)



```
foo__bar__
```






```
<p>foo__bar__</p>
```




<a id="example-396"></a>

[Example 396](015-continuation-15.md#example-396)



```
5__6__78
```






```
<p>5__6__78</p>
```




<a id="example-397"></a>

[Example 397](015-continuation-15.md#example-397)



```
пристаням__стремятся__
```






```
<p>пристаням__стремятся__</p>
```




<a id="example-398"></a>

[Example 398](015-continuation-15.md#example-398)



```
__foo, __bar__, baz__
```






```
<p><strong>foo, <strong>bar</strong>, baz</strong></p>
```




This is strong emphasis, even though the opening delimiter is
both left- and right-flanking, because it is preceded by
punctuation:

<a id="example-399"></a>

[Example 399](015-continuation-15.md#example-399)



```
foo-__(bar)__
```






```
<p>foo-<strong>(bar)</strong></p>
```




Rule 7:

This is not strong emphasis, because the closing delimiter is preceded
by whitespace:

<a id="example-400"></a>

[Example 400](015-continuation-15.md#example-400)



```
**foo bar **
```






```
<p>**foo bar **</p>
```




(Nor can it be interpreted as an emphasized `*foo bar *`, because of
Rule 11.)

This is not strong emphasis, because the second `**` is
preceded by punctuation and followed by an alphanumeric:

<a id="example-401"></a>

[Example 401](015-continuation-15.md#example-401)



```
**(**foo)
```






```
<p>**(**foo)</p>
```




The point of this restriction is more easily appreciated
with these examples:

<a id="example-402"></a>

[Example 402](015-continuation-15.md#example-402)



```
*(**foo**)*
```






```
<p><em>(<strong>foo</strong>)</em></p>
```




<a id="example-403"></a>

[Example 403](015-continuation-15.md#example-403)



```
**Gomphocarpus (*Gomphocarpus physocarpus*, syn.
*Asclepias physocarpa*)**
```






```
<p><strong>Gomphocarpus (<em>Gomphocarpus physocarpus</em>, syn.
<em>Asclepias physocarpa</em>)</strong></p>
```




<a id="example-404"></a>

[Example 404](015-continuation-15.md#example-404)



```
**foo "*bar*" foo**
```






```
<p><strong>foo &quot;<em>bar</em>&quot; foo</strong></p>
```




Intraword emphasis:

<a id="example-405"></a>

[Example 405](015-continuation-15.md#example-405)



```
**foo**bar
```






```
<p><strong>foo</strong>bar</p>
```




Rule 8:

