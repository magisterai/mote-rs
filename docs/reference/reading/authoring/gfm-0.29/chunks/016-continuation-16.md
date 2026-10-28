# Continuation 16

Source: [GitHub Flavored Markdown Specification](https://github.github.com/gfm/) · 0.29-gfm, 2019-04-06.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://github.github.com/gfm/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](015-continuation-15.md) · [Next](017-6-5-strikethrough-extension.md)

This is not strong emphasis, because the closing delimiter is
preceded by whitespace:

<a id="example-406"></a>

[Example 406](016-continuation-16.md#example-406)



```
__foo bar __
```






```
<p>__foo bar __</p>
```




This is not strong emphasis, because the second `__` is
preceded by punctuation and followed by an alphanumeric:

<a id="example-407"></a>

[Example 407](016-continuation-16.md#example-407)



```
__(__foo)
```






```
<p>__(__foo)</p>
```




The point of this restriction is more easily appreciated
with this example:

<a id="example-408"></a>

[Example 408](016-continuation-16.md#example-408)



```
_(__foo__)_
```






```
<p><em>(<strong>foo</strong>)</em></p>
```




Intraword strong emphasis is forbidden with `__`:

<a id="example-409"></a>

[Example 409](016-continuation-16.md#example-409)



```
__foo__bar
```






```
<p>__foo__bar</p>
```




<a id="example-410"></a>

[Example 410](016-continuation-16.md#example-410)



```
__пристаням__стремятся
```






```
<p>__пристаням__стремятся</p>
```




<a id="example-411"></a>

[Example 411](016-continuation-16.md#example-411)



```
__foo__bar__baz__
```






```
<p><strong>foo__bar__baz</strong></p>
```




This is strong emphasis, even though the closing delimiter is
both left- and right-flanking, because it is followed by
punctuation:

<a id="example-412"></a>

[Example 412](016-continuation-16.md#example-412)



```
__(bar)__.
```






```
<p><strong>(bar)</strong>.</p>
```




Rule 9:

Any nonempty sequence of inline elements can be the contents of an
emphasized span.

<a id="example-413"></a>

[Example 413](016-continuation-16.md#example-413)



```
*foo [bar](/url)*
```






```
<p><em>foo <a href="/url">bar</a></em></p>
```




<a id="example-414"></a>

[Example 414](016-continuation-16.md#example-414)



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

<a id="example-415"></a>

[Example 415](016-continuation-16.md#example-415)



```
_foo __bar__ baz_
```






```
<p><em>foo <strong>bar</strong> baz</em></p>
```




<a id="example-416"></a>

[Example 416](016-continuation-16.md#example-416)



```
_foo _bar_ baz_
```






```
<p><em>foo <em>bar</em> baz</em></p>
```




<a id="example-417"></a>

[Example 417](016-continuation-16.md#example-417)



```
__foo_ bar_
```






```
<p><em><em>foo</em> bar</em></p>
```




<a id="example-418"></a>

[Example 418](016-continuation-16.md#example-418)



```
*foo *bar**
```






```
<p><em>foo <em>bar</em></em></p>
```




<a id="example-419"></a>

[Example 419](016-continuation-16.md#example-419)



```
*foo **bar** baz*
```






```
<p><em>foo <strong>bar</strong> baz</em></p>
```




<a id="example-420"></a>

[Example 420](016-continuation-16.md#example-420)



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

<a id="example-421"></a>

[Example 421](016-continuation-16.md#example-421)



```
*foo**bar*
```






```
<p><em>foo**bar</em></p>
```




The same condition ensures that the following
cases are all strong emphasis nested inside
emphasis, even when the interior spaces are
omitted:

<a id="example-422"></a>

[Example 422](016-continuation-16.md#example-422)



```
***foo** bar*
```






```
<p><em><strong>foo</strong> bar</em></p>
```




<a id="example-423"></a>

[Example 423](016-continuation-16.md#example-423)



```
*foo **bar***
```






```
<p><em>foo <strong>bar</strong></em></p>
```




<a id="example-424"></a>

[Example 424](016-continuation-16.md#example-424)



```
*foo**bar***
```






```
<p><em>foo<strong>bar</strong></em></p>
```




When the lengths of the interior closing and opening
delimiter runs are *both* multiples of 3, though,
they can match to create emphasis:

<a id="example-425"></a>

[Example 425](016-continuation-16.md#example-425)



```
foo***bar***baz
```






```
<p>foo<em><strong>bar</strong></em>baz</p>
```




<a id="example-426"></a>

[Example 426](016-continuation-16.md#example-426)



```
foo******bar*********baz
```






```
<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>
```




Indefinite levels of nesting are possible:

<a id="example-427"></a>

[Example 427](016-continuation-16.md#example-427)



```
*foo **bar *baz* bim** bop*
```






```
<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>
```




<a id="example-428"></a>

[Example 428](016-continuation-16.md#example-428)



```
*foo [*bar*](/url)*
```






```
<p><em>foo <a href="/url"><em>bar</em></a></em></p>
```




There can be no empty emphasis or strong emphasis:

<a id="example-429"></a>

[Example 429](016-continuation-16.md#example-429)



```
** is not an empty emphasis
```






```
<p>** is not an empty emphasis</p>
```




<a id="example-430"></a>

[Example 430](016-continuation-16.md#example-430)



```
**** is not an empty strong emphasis
```






```
<p>**** is not an empty strong emphasis</p>
```




Rule 10:

Any nonempty sequence of inline elements can be the contents of an
strongly emphasized span.

<a id="example-431"></a>

[Example 431](016-continuation-16.md#example-431)



```
**foo [bar](/url)**
```






```
<p><strong>foo <a href="/url">bar</a></strong></p>
```




<a id="example-432"></a>

[Example 432](016-continuation-16.md#example-432)



```
**foo
bar**
```






```
<p><strong>foo
bar</strong></p>
```




In particular, emphasis and strong emphasis can be nested
inside strong emphasis:

<a id="example-433"></a>

[Example 433](016-continuation-16.md#example-433)



```
__foo _bar_ baz__
```






```
<p><strong>foo <em>bar</em> baz</strong></p>
```




<a id="example-434"></a>

[Example 434](016-continuation-16.md#example-434)



```
__foo __bar__ baz__
```






```
<p><strong>foo <strong>bar</strong> baz</strong></p>
```




<a id="example-435"></a>

[Example 435](016-continuation-16.md#example-435)



```
____foo__ bar__
```






```
<p><strong><strong>foo</strong> bar</strong></p>
```




<a id="example-436"></a>

[Example 436](016-continuation-16.md#example-436)



```
**foo **bar****
```






```
<p><strong>foo <strong>bar</strong></strong></p>
```




<a id="example-437"></a>

[Example 437](016-continuation-16.md#example-437)



```
**foo *bar* baz**
```






```
<p><strong>foo <em>bar</em> baz</strong></p>
```




<a id="example-438"></a>

[Example 438](016-continuation-16.md#example-438)



```
**foo*bar*baz**
```






```
<p><strong>foo<em>bar</em>baz</strong></p>
```




<a id="example-439"></a>

[Example 439](016-continuation-16.md#example-439)



```
***foo* bar**
```






```
<p><strong><em>foo</em> bar</strong></p>
```




<a id="example-440"></a>

[Example 440](016-continuation-16.md#example-440)



```
**foo *bar***
```






```
<p><strong>foo <em>bar</em></strong></p>
```




Indefinite levels of nesting are possible:

<a id="example-441"></a>

[Example 441](016-continuation-16.md#example-441)



```
**foo *bar **baz**
bim* bop**
```






```
<p><strong>foo <em>bar <strong>baz</strong>
bim</em> bop</strong></p>
```




<a id="example-442"></a>

[Example 442](016-continuation-16.md#example-442)



```
**foo [*bar*](/url)**
```






```
<p><strong>foo <a href="/url"><em>bar</em></a></strong></p>
```




There can be no empty emphasis or strong emphasis:

<a id="example-443"></a>

[Example 443](016-continuation-16.md#example-443)



```
__ is not an empty emphasis
```






```
<p>__ is not an empty emphasis</p>
```




<a id="example-444"></a>

[Example 444](016-continuation-16.md#example-444)



```
____ is not an empty strong emphasis
```






```
<p>____ is not an empty strong emphasis</p>
```




Rule 11:

<a id="example-445"></a>

[Example 445](016-continuation-16.md#example-445)



```
foo ***
```






```
<p>foo ***</p>
```




<a id="example-446"></a>

[Example 446](016-continuation-16.md#example-446)



```
foo *\**
```






```
<p>foo <em>*</em></p>
```




<a id="example-447"></a>

[Example 447](016-continuation-16.md#example-447)



```
foo *_*
```






```
<p>foo <em>_</em></p>
```




<a id="example-448"></a>

[Example 448](016-continuation-16.md#example-448)



```
foo *****
```






```
<p>foo *****</p>
```




<a id="example-449"></a>

[Example 449](016-continuation-16.md#example-449)



```
foo **\***
```






```
<p>foo <strong>*</strong></p>
```




<a id="example-450"></a>

[Example 450](016-continuation-16.md#example-450)



```
foo **_**
```






```
<p>foo <strong>_</strong></p>
```




Note that when delimiters do not match evenly, Rule 11 determines
that the excess literal `*` characters will appear outside of the
emphasis, rather than inside it:

<a id="example-451"></a>

[Example 451](016-continuation-16.md#example-451)



```
**foo*
```






```
<p>*<em>foo</em></p>
```




<a id="example-452"></a>

[Example 452](016-continuation-16.md#example-452)



```
*foo**
```






```
<p><em>foo</em>*</p>
```




<a id="example-453"></a>

[Example 453](016-continuation-16.md#example-453)



```
***foo**
```






```
<p>*<strong>foo</strong></p>
```




<a id="example-454"></a>

[Example 454](016-continuation-16.md#example-454)



```
****foo*
```






```
<p>***<em>foo</em></p>
```




<a id="example-455"></a>

[Example 455](016-continuation-16.md#example-455)



```
**foo***
```






```
<p><strong>foo</strong>*</p>
```




<a id="example-456"></a>

[Example 456](016-continuation-16.md#example-456)



```
*foo****
```






```
<p><em>foo</em>***</p>
```




Rule 12:

<a id="example-457"></a>

[Example 457](016-continuation-16.md#example-457)



```
foo ___
```






```
<p>foo ___</p>
```




<a id="example-458"></a>

[Example 458](016-continuation-16.md#example-458)



```
foo _\__
```






```
<p>foo <em>_</em></p>
```




<a id="example-459"></a>

[Example 459](016-continuation-16.md#example-459)



```
foo _*_
```






```
<p>foo <em>*</em></p>
```




<a id="example-460"></a>

[Example 460](016-continuation-16.md#example-460)



```
foo _____
```






```
<p>foo _____</p>
```




<a id="example-461"></a>

[Example 461](016-continuation-16.md#example-461)



```
foo __\___
```






```
<p>foo <strong>_</strong></p>
```




<a id="example-462"></a>

[Example 462](016-continuation-16.md#example-462)



```
foo __*__
```






```
<p>foo <strong>*</strong></p>
```




<a id="example-463"></a>

[Example 463](016-continuation-16.md#example-463)



```
__foo_
```






```
<p>_<em>foo</em></p>
```




Note that when delimiters do not match evenly, Rule 12 determines
that the excess literal `_` characters will appear outside of the
emphasis, rather than inside it:

<a id="example-464"></a>

[Example 464](016-continuation-16.md#example-464)



```
_foo__
```






```
<p><em>foo</em>_</p>
```




<a id="example-465"></a>

[Example 465](016-continuation-16.md#example-465)



```
___foo__
```






```
<p>_<strong>foo</strong></p>
```




<a id="example-466"></a>

[Example 466](016-continuation-16.md#example-466)



```
____foo_
```






```
<p>___<em>foo</em></p>
```




<a id="example-467"></a>

[Example 467](016-continuation-16.md#example-467)



```
__foo___
```






```
<p><strong>foo</strong>_</p>
```




<a id="example-468"></a>

[Example 468](016-continuation-16.md#example-468)



```
_foo____
```






```
<p><em>foo</em>___</p>
```




Rule 13 implies that if you want emphasis nested directly inside
emphasis, you must use different delimiters:

<a id="example-469"></a>

[Example 469](016-continuation-16.md#example-469)



```
**foo**
```






```
<p><strong>foo</strong></p>
```




<a id="example-470"></a>

[Example 470](016-continuation-16.md#example-470)



```
*_foo_*
```






```
<p><em><em>foo</em></em></p>
```




<a id="example-471"></a>

[Example 471](016-continuation-16.md#example-471)



```
__foo__
```






```
<p><strong>foo</strong></p>
```




<a id="example-472"></a>

[Example 472](016-continuation-16.md#example-472)



```
_*foo*_
```






```
<p><em><em>foo</em></em></p>
```




However, strong emphasis within strong emphasis is possible without
switching delimiters:

<a id="example-473"></a>

[Example 473](016-continuation-16.md#example-473)



```
****foo****
```






```
<p><strong><strong>foo</strong></strong></p>
```




<a id="example-474"></a>

[Example 474](016-continuation-16.md#example-474)



```
____foo____
```






```
<p><strong><strong>foo</strong></strong></p>
```




Rule 13 can be applied to arbitrarily long sequences of
delimiters:

<a id="example-475"></a>

[Example 475](016-continuation-16.md#example-475)



```
******foo******
```






```
<p><strong><strong><strong>foo</strong></strong></strong></p>
```




Rule 14:

<a id="example-476"></a>

[Example 476](016-continuation-16.md#example-476)



```
***foo***
```






```
<p><em><strong>foo</strong></em></p>
```




<a id="example-477"></a>

[Example 477](016-continuation-16.md#example-477)



