# Continuation 16

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](015-6-2-emphasis-and-strong-emphasis.md) · [Next](017-continuation-17.md)

Here `_` does not generate emphasis, because the first delimiter run
is right-flanking and the second left-flanking:

<a id="example-363"></a>

[Example 363](016-continuation-16.md#example-363)Try It



```
aa_"bb"_cc
```






```
<p>aa_&quot;bb&quot;_cc</p>
```




This is emphasis, even though the opening delimiter is
both left- and right-flanking, because it is preceded by
punctuation:

<a id="example-364"></a>

[Example 364](016-continuation-16.md#example-364)Try It



```
foo-_(bar)_
```






```
<p>foo-<em>(bar)</em></p>
```




Rule 3:

This is not emphasis, because the closing delimiter does
not match the opening delimiter:

<a id="example-365"></a>

[Example 365](016-continuation-16.md#example-365)Try It



```
_foo*
```






```
<p>_foo*</p>
```




This is not emphasis, because the closing `*` is preceded by
whitespace:

<a id="example-366"></a>

[Example 366](016-continuation-16.md#example-366)Try It



```
*foo bar *
```






```
<p>*foo bar *</p>
```




A line ending also counts as whitespace:

<a id="example-367"></a>

[Example 367](016-continuation-16.md#example-367)Try It



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
(hence it is not part of a [right-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#right-flanking-delimiter-run):

<a id="example-368"></a>

[Example 368](016-continuation-16.md#example-368)Try It



```
*(*foo)
```






```
<p>*(*foo)</p>
```




The point of this restriction is more easily appreciated
with this example:

<a id="example-369"></a>

[Example 369](016-continuation-16.md#example-369)Try It



```
*(*foo*)*
```






```
<p><em>(<em>foo</em>)</em></p>
```




Intraword emphasis with `*` is allowed:

<a id="example-370"></a>

[Example 370](016-continuation-16.md#example-370)Try It



```
*foo*bar
```






```
<p><em>foo</em>bar</p>
```




Rule 4:

This is not emphasis, because the closing `_` is preceded by
whitespace:

<a id="example-371"></a>

[Example 371](016-continuation-16.md#example-371)Try It



```
_foo bar _
```






```
<p>_foo bar _</p>
```




This is not emphasis, because the second `_` is
preceded by punctuation and followed by an alphanumeric:

<a id="example-372"></a>

[Example 372](016-continuation-16.md#example-372)Try It



```
_(_foo)
```






```
<p>_(_foo)</p>
```




This is emphasis within emphasis:

<a id="example-373"></a>

[Example 373](016-continuation-16.md#example-373)Try It



```
_(_foo_)_
```






```
<p><em>(<em>foo</em>)</em></p>
```




Intraword emphasis is disallowed for `_`:

<a id="example-374"></a>

[Example 374](016-continuation-16.md#example-374)Try It



```
_foo_bar
```






```
<p>_foo_bar</p>
```




<a id="example-375"></a>

[Example 375](016-continuation-16.md#example-375)Try It



```
_пристаням_стремятся
```






```
<p>_пристаням_стремятся</p>
```




<a id="example-376"></a>

[Example 376](016-continuation-16.md#example-376)Try It



```
_foo_bar_baz_
```






```
<p><em>foo_bar_baz</em></p>
```




This is emphasis, even though the closing delimiter is
both left- and right-flanking, because it is followed by
punctuation:

<a id="example-377"></a>

[Example 377](016-continuation-16.md#example-377)Try It



```
_(bar)_.
```






```
<p><em>(bar)</em>.</p>
```




Rule 5:

<a id="example-378"></a>

[Example 378](016-continuation-16.md#example-378)Try It



```
**foo bar**
```






```
<p><strong>foo bar</strong></p>
```




This is not strong emphasis, because the opening delimiter is
followed by whitespace:

<a id="example-379"></a>

[Example 379](016-continuation-16.md#example-379)Try It



```
** foo bar**
```






```
<p>** foo bar**</p>
```




This is not strong emphasis, because the opening `**` is preceded
by an alphanumeric and followed by punctuation, and hence
not part of a [left-flanking delimiter run](015-6-2-emphasis-and-strong-emphasis.md#left-flanking-delimiter-run):

<a id="example-380"></a>

[Example 380](016-continuation-16.md#example-380)Try It



```
a**"foo"**
```






```
<p>a**&quot;foo&quot;**</p>
```




Intraword strong emphasis with `**` is permitted:

<a id="example-381"></a>

[Example 381](016-continuation-16.md#example-381)Try It



```
foo**bar**
```






```
<p>foo<strong>bar</strong></p>
```




Rule 6:

<a id="example-382"></a>

[Example 382](016-continuation-16.md#example-382)Try It



```
__foo bar__
```






```
<p><strong>foo bar</strong></p>
```




This is not strong emphasis, because the opening delimiter is
followed by whitespace:

<a id="example-383"></a>

[Example 383](016-continuation-16.md#example-383)Try It



```
__ foo bar__
```






```
<p>__ foo bar__</p>
```




A line ending counts as whitespace:

<a id="example-384"></a>

[Example 384](016-continuation-16.md#example-384)Try It



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

<a id="example-385"></a>

[Example 385](016-continuation-16.md#example-385)Try It



```
a__"foo"__
```






```
<p>a__&quot;foo&quot;__</p>
```




Intraword strong emphasis is forbidden with `__`:

<a id="example-386"></a>

[Example 386](016-continuation-16.md#example-386)Try It



```
foo__bar__
```






```
<p>foo__bar__</p>
```




<a id="example-387"></a>

[Example 387](016-continuation-16.md#example-387)Try It



```
5__6__78
```






```
<p>5__6__78</p>
```




<a id="example-388"></a>

[Example 388](016-continuation-16.md#example-388)Try It



```
пристаням__стремятся__
```






```
<p>пристаням__стремятся__</p>
```




<a id="example-389"></a>

[Example 389](016-continuation-16.md#example-389)Try It



```
__foo, __bar__, baz__
```






```
<p><strong>foo, <strong>bar</strong>, baz</strong></p>
```




This is strong emphasis, even though the opening delimiter is
both left- and right-flanking, because it is preceded by
punctuation:

<a id="example-390"></a>

[Example 390](016-continuation-16.md#example-390)Try It



```
foo-__(bar)__
```






```
<p>foo-<strong>(bar)</strong></p>
```




Rule 7:

This is not strong emphasis, because the closing delimiter is preceded
by whitespace:

<a id="example-391"></a>

[Example 391](016-continuation-16.md#example-391)Try It



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

<a id="example-392"></a>

[Example 392](016-continuation-16.md#example-392)Try It



```
**(**foo)
```






```
<p>**(**foo)</p>
```




The point of this restriction is more easily appreciated
with these examples:

<a id="example-393"></a>

[Example 393](016-continuation-16.md#example-393)Try It



```
*(**foo**)*
```






```
<p><em>(<strong>foo</strong>)</em></p>
```




<a id="example-394"></a>

[Example 394](016-continuation-16.md#example-394)Try It



```
**Gomphocarpus (*Gomphocarpus physocarpus*, syn.
*Asclepias physocarpa*)**
```






```
<p><strong>Gomphocarpus (<em>Gomphocarpus physocarpus</em>, syn.
<em>Asclepias physocarpa</em>)</strong></p>
```




<a id="example-395"></a>

[Example 395](016-continuation-16.md#example-395)Try It



```
**foo "*bar*" foo**
```






```
<p><strong>foo &quot;<em>bar</em>&quot; foo</strong></p>
```




Intraword emphasis:

<a id="example-396"></a>

[Example 396](016-continuation-16.md#example-396)Try It



```
**foo**bar
```






```
<p><strong>foo</strong>bar</p>
```




Rule 8:

This is not strong emphasis, because the closing delimiter is
preceded by whitespace:

<a id="example-397"></a>

[Example 397](016-continuation-16.md#example-397)Try It



```
__foo bar __
```






```
<p>__foo bar __</p>
```




This is not strong emphasis, because the second `__` is
preceded by punctuation and followed by an alphanumeric:

<a id="example-398"></a>

[Example 398](016-continuation-16.md#example-398)Try It



```
__(__foo)
```






```
<p>__(__foo)</p>
```




The point of this restriction is more easily appreciated
with this example:

<a id="example-399"></a>

[Example 399](016-continuation-16.md#example-399)Try It



```
_(__foo__)_
```






```
<p><em>(<strong>foo</strong>)</em></p>
```




Intraword strong emphasis is forbidden with `__`:

<a id="example-400"></a>

[Example 400](016-continuation-16.md#example-400)Try It



```
__foo__bar
```






```
<p>__foo__bar</p>
```




<a id="example-401"></a>

[Example 401](016-continuation-16.md#example-401)Try It



```
__пристаням__стремятся
```






```
<p>__пристаням__стремятся</p>
```




<a id="example-402"></a>

[Example 402](016-continuation-16.md#example-402)Try It



```
__foo__bar__baz__
```






```
<p><strong>foo__bar__baz</strong></p>
```




This is strong emphasis, even though the closing delimiter is
both left- and right-flanking, because it is followed by
punctuation:

<a id="example-403"></a>

[Example 403](016-continuation-16.md#example-403)Try It



```
__(bar)__.
```






```
<p><strong>(bar)</strong>.</p>
```




Rule 9:

Any nonempty sequence of inline elements can be the contents of an
emphasized span.

<a id="example-404"></a>

[Example 404](016-continuation-16.md#example-404)Try It



```
*foo [bar](/url)*
```






```
<p><em>foo <a href="/url">bar</a></em></p>
```




<a id="example-405"></a>

[Example 405](016-continuation-16.md#example-405)Try It



```
*foo
bar*
```






```
<p><em>foo
bar</em></p>
```




In particular, emphasis and strong emphasis can be nested
inside emphasis:

<a id="example-406"></a>

[Example 406](016-continuation-16.md#example-406)Try It



```
_foo __bar__ baz_
```






```
<p><em>foo <strong>bar</strong> baz</em></p>
```




<a id="example-407"></a>

[Example 407](016-continuation-16.md#example-407)Try It



```
_foo _bar_ baz_
```






```
<p><em>foo <em>bar</em> baz</em></p>
```




<a id="example-408"></a>

[Example 408](016-continuation-16.md#example-408)Try It



```
__foo_ bar_
```






```
<p><em><em>foo</em> bar</em></p>
```




<a id="example-409"></a>

[Example 409](016-continuation-16.md#example-409)Try It



```
*foo *bar**
```






```
<p><em>foo <em>bar</em></em></p>
```




<a id="example-410"></a>

[Example 410](016-continuation-16.md#example-410)Try It



```
*foo **bar** baz*
```






```
<p><em>foo <strong>bar</strong> baz</em></p>
```




<a id="example-411"></a>

[Example 411](016-continuation-16.md#example-411)Try It



```
*foo**bar**baz*
```






```
<p><em>foo<strong>bar</strong>baz</em></p>
```




Note that in the preceding case, the interpretation



```
<p><em>foo</em><em>bar<em></em>baz</em></p>
```




is precluded by the condition that a delimiter that
can both open and close (like the `*` after `foo`)
cannot form emphasis if the sum of the lengths of
the delimiter runs containing the opening and
closing delimiters is a multiple of 3 unless
both lengths are multiples of 3.

For the same reason, we don’t get two consecutive
emphasis sections in this example:

<a id="example-412"></a>

[Example 412](016-continuation-16.md#example-412)Try It



```
*foo**bar*
```






```
<p><em>foo**bar</em></p>
```




The same condition ensures that the following
cases are all strong emphasis nested inside
emphasis, even when the interior whitespace is
omitted:

<a id="example-413"></a>

[Example 413](016-continuation-16.md#example-413)Try It



```
***foo** bar*
```






```
<p><em><strong>foo</strong> bar</em></p>
```




<a id="example-414"></a>

[Example 414](016-continuation-16.md#example-414)Try It



```
*foo **bar***
```






```
<p><em>foo <strong>bar</strong></em></p>
```




<a id="example-415"></a>

[Example 415](016-continuation-16.md#example-415)Try It



```
*foo**bar***
```






```
<p><em>foo<strong>bar</strong></em></p>
```




When the lengths of the interior closing and opening
delimiter runs are *both* multiples of 3, though,
they can match to create emphasis:

<a id="example-416"></a>

[Example 416](016-continuation-16.md#example-416)Try It



```
foo***bar***baz
```






```
<p>foo<em><strong>bar</strong></em>baz</p>
```




<a id="example-417"></a>

[Example 417](016-continuation-16.md#example-417)Try It



```
foo******bar*********baz
```






```
<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>
```




Indefinite levels of nesting are possible:

<a id="example-418"></a>

[Example 418](016-continuation-16.md#example-418)Try It



```
*foo **bar *baz* bim** bop*
```






```
<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>
```




<a id="example-419"></a>

[Example 419](016-continuation-16.md#example-419)Try It



```
*foo [*bar*](/url)*
```






```
<p><em>foo <a href="/url"><em>bar</em></a></em></p>
```




There can be no empty emphasis or strong emphasis:

<a id="example-420"></a>

[Example 420](016-continuation-16.md#example-420)Try It



```
** is not an empty emphasis
```






```
<p>** is not an empty emphasis</p>
```




<a id="example-421"></a>

[Example 421](016-continuation-16.md#example-421)Try It



```
**** is not an empty strong emphasis
```






