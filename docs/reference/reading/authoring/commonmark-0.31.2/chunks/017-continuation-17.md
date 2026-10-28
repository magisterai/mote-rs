# Continuation 17

Source: [CommonMark Specification](https://spec.commonmark.org/0.31.2/) · 0.31.2, 2024-01-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://spec.commonmark.org/0.31.2/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-continuation-16.md) · [Next](018-6-3-links.md)

```
<p>**** is not an empty strong emphasis</p>
```




Rule 10:

Any nonempty sequence of inline elements can be the contents of an
strongly emphasized span.

<a id="example-422"></a>

[Example 422](017-continuation-17.md#example-422)Try It



```
**foo [bar](/url)**
```






```
<p><strong>foo <a href="/url">bar</a></strong></p>
```




<a id="example-423"></a>

[Example 423](017-continuation-17.md#example-423)Try It



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

<a id="example-424"></a>

[Example 424](017-continuation-17.md#example-424)Try It



```
__foo _bar_ baz__
```






```
<p><strong>foo <em>bar</em> baz</strong></p>
```




<a id="example-425"></a>

[Example 425](017-continuation-17.md#example-425)Try It



```
__foo __bar__ baz__
```






```
<p><strong>foo <strong>bar</strong> baz</strong></p>
```




<a id="example-426"></a>

[Example 426](017-continuation-17.md#example-426)Try It



```
____foo__ bar__
```






```
<p><strong><strong>foo</strong> bar</strong></p>
```




<a id="example-427"></a>

[Example 427](017-continuation-17.md#example-427)Try It



```
**foo **bar****
```






```
<p><strong>foo <strong>bar</strong></strong></p>
```




<a id="example-428"></a>

[Example 428](017-continuation-17.md#example-428)Try It



```
**foo *bar* baz**
```






```
<p><strong>foo <em>bar</em> baz</strong></p>
```




<a id="example-429"></a>

[Example 429](017-continuation-17.md#example-429)Try It



```
**foo*bar*baz**
```






```
<p><strong>foo<em>bar</em>baz</strong></p>
```




<a id="example-430"></a>

[Example 430](017-continuation-17.md#example-430)Try It



```
***foo* bar**
```






```
<p><strong><em>foo</em> bar</strong></p>
```




<a id="example-431"></a>

[Example 431](017-continuation-17.md#example-431)Try It



```
**foo *bar***
```






```
<p><strong>foo <em>bar</em></strong></p>
```




Indefinite levels of nesting are possible:

<a id="example-432"></a>

[Example 432](017-continuation-17.md#example-432)Try It



```
**foo *bar **baz**
bim* bop**
```






```
<p><strong>foo <em>bar <strong>baz</strong>
bim</em> bop</strong></p>
```




<a id="example-433"></a>

[Example 433](017-continuation-17.md#example-433)Try It



```
**foo [*bar*](/url)**
```






```
<p><strong>foo <a href="/url"><em>bar</em></a></strong></p>
```




There can be no empty emphasis or strong emphasis:

<a id="example-434"></a>

[Example 434](017-continuation-17.md#example-434)Try It



```
__ is not an empty emphasis
```






```
<p>__ is not an empty emphasis</p>
```




<a id="example-435"></a>

[Example 435](017-continuation-17.md#example-435)Try It



```
____ is not an empty strong emphasis
```






```
<p>____ is not an empty strong emphasis</p>
```




Rule 11:

<a id="example-436"></a>

[Example 436](017-continuation-17.md#example-436)Try It



```
foo ***
```






```
<p>foo ***</p>
```




<a id="example-437"></a>

[Example 437](017-continuation-17.md#example-437)Try It



```
foo *\**
```






```
<p>foo <em>*</em></p>
```




<a id="example-438"></a>

[Example 438](017-continuation-17.md#example-438)Try It



```
foo *_*
```






```
<p>foo <em>_</em></p>
```




<a id="example-439"></a>

[Example 439](017-continuation-17.md#example-439)Try It



```
foo *****
```






```
<p>foo *****</p>
```




<a id="example-440"></a>

[Example 440](017-continuation-17.md#example-440)Try It



```
foo **\***
```






```
<p>foo <strong>*</strong></p>
```




<a id="example-441"></a>

[Example 441](017-continuation-17.md#example-441)Try It



```
foo **_**
```






```
<p>foo <strong>_</strong></p>
```




Note that when delimiters do not match evenly, Rule 11 determines
that the excess literal `*` characters will appear outside of the
emphasis, rather than inside it:

<a id="example-442"></a>

[Example 442](017-continuation-17.md#example-442)Try It



```
**foo*
```






```
<p>*<em>foo</em></p>
```




<a id="example-443"></a>

[Example 443](017-continuation-17.md#example-443)Try It



```
*foo**
```






```
<p><em>foo</em>*</p>
```




<a id="example-444"></a>

[Example 444](017-continuation-17.md#example-444)Try It



```
***foo**
```






```
<p>*<strong>foo</strong></p>
```




<a id="example-445"></a>

[Example 445](017-continuation-17.md#example-445)Try It



```
****foo*
```






```
<p>***<em>foo</em></p>
```




<a id="example-446"></a>

[Example 446](017-continuation-17.md#example-446)Try It



```
**foo***
```






```
<p><strong>foo</strong>*</p>
```




<a id="example-447"></a>

[Example 447](017-continuation-17.md#example-447)Try It



```
*foo****
```






```
<p><em>foo</em>***</p>
```




Rule 12:

<a id="example-448"></a>

[Example 448](017-continuation-17.md#example-448)Try It



```
foo ___
```






```
<p>foo ___</p>
```




<a id="example-449"></a>

[Example 449](017-continuation-17.md#example-449)Try It



```
foo _\__
```






```
<p>foo <em>_</em></p>
```




<a id="example-450"></a>

[Example 450](017-continuation-17.md#example-450)Try It



```
foo _*_
```






```
<p>foo <em>*</em></p>
```




<a id="example-451"></a>

[Example 451](017-continuation-17.md#example-451)Try It



```
foo _____
```






```
<p>foo _____</p>
```




<a id="example-452"></a>

[Example 452](017-continuation-17.md#example-452)Try It



```
foo __\___
```






```
<p>foo <strong>_</strong></p>
```




<a id="example-453"></a>

[Example 453](017-continuation-17.md#example-453)Try It



```
foo __*__
```






```
<p>foo <strong>*</strong></p>
```




<a id="example-454"></a>

[Example 454](017-continuation-17.md#example-454)Try It



```
__foo_
```






```
<p>_<em>foo</em></p>
```




Note that when delimiters do not match evenly, Rule 12 determines
that the excess literal `_` characters will appear outside of the
emphasis, rather than inside it:

<a id="example-455"></a>

[Example 455](017-continuation-17.md#example-455)Try It



```
_foo__
```






```
<p><em>foo</em>_</p>
```




<a id="example-456"></a>

[Example 456](017-continuation-17.md#example-456)Try It



```
___foo__
```






```
<p>_<strong>foo</strong></p>
```




<a id="example-457"></a>

[Example 457](017-continuation-17.md#example-457)Try It



```
____foo_
```






```
<p>___<em>foo</em></p>
```




<a id="example-458"></a>

[Example 458](017-continuation-17.md#example-458)Try It



```
__foo___
```






```
<p><strong>foo</strong>_</p>
```




<a id="example-459"></a>

[Example 459](017-continuation-17.md#example-459)Try It



```
_foo____
```






```
<p><em>foo</em>___</p>
```




Rule 13 implies that if you want emphasis nested directly inside
emphasis, you must use different delimiters:

<a id="example-460"></a>

[Example 460](017-continuation-17.md#example-460)Try It



```
**foo**
```






```
<p><strong>foo</strong></p>
```




<a id="example-461"></a>

[Example 461](017-continuation-17.md#example-461)Try It



```
*_foo_*
```






```
<p><em><em>foo</em></em></p>
```




<a id="example-462"></a>

[Example 462](017-continuation-17.md#example-462)Try It



```
__foo__
```






```
<p><strong>foo</strong></p>
```




<a id="example-463"></a>

[Example 463](017-continuation-17.md#example-463)Try It



```
_*foo*_
```






```
<p><em><em>foo</em></em></p>
```




However, strong emphasis within strong emphasis is possible without
switching delimiters:

<a id="example-464"></a>

[Example 464](017-continuation-17.md#example-464)Try It



```
****foo****
```






```
<p><strong><strong>foo</strong></strong></p>
```




<a id="example-465"></a>

[Example 465](017-continuation-17.md#example-465)Try It



```
____foo____
```






```
<p><strong><strong>foo</strong></strong></p>
```




Rule 13 can be applied to arbitrarily long sequences of
delimiters:

<a id="example-466"></a>

[Example 466](017-continuation-17.md#example-466)Try It



```
******foo******
```






```
<p><strong><strong><strong>foo</strong></strong></strong></p>
```




Rule 14:

<a id="example-467"></a>

[Example 467](017-continuation-17.md#example-467)Try It



```
***foo***
```






```
<p><em><strong>foo</strong></em></p>
```




<a id="example-468"></a>

[Example 468](017-continuation-17.md#example-468)Try It



```
_____foo_____
```






```
<p><em><strong><strong>foo</strong></strong></em></p>
```




Rule 15:

<a id="example-469"></a>

[Example 469](017-continuation-17.md#example-469)Try It



```
*foo _bar* baz_
```






```
<p><em>foo _bar</em> baz_</p>
```




<a id="example-470"></a>

[Example 470](017-continuation-17.md#example-470)Try It



```
*foo __bar *baz bim__ bam*
```






```
<p><em>foo <strong>bar *baz bim</strong> bam</em></p>
```




Rule 16:

<a id="example-471"></a>

[Example 471](017-continuation-17.md#example-471)Try It



```
**foo **bar baz**
```






```
<p>**foo <strong>bar baz</strong></p>
```




<a id="example-472"></a>

[Example 472](017-continuation-17.md#example-472)Try It



```
*foo *bar baz*
```






```
<p>*foo <em>bar baz</em></p>
```




Rule 17:

<a id="example-473"></a>

[Example 473](017-continuation-17.md#example-473)Try It



```
*[bar*](/url)
```






```
<p>*<a href="/url">bar*</a></p>
```




<a id="example-474"></a>

[Example 474](017-continuation-17.md#example-474)Try It



```
_foo [bar_](/url)
```






```
<p>_foo <a href="/url">bar_</a></p>
```




<a id="example-475"></a>

[Example 475](017-continuation-17.md#example-475)Try It



```
*<img src="foo" title="*"/>
```






```
<p>*<img src="foo" title="*"/></p>
```




<a id="example-476"></a>

[Example 476](017-continuation-17.md#example-476)Try It



```
**<a href="**">
```






```
<p>**<a href="**"></p>
```




<a id="example-477"></a>

[Example 477](017-continuation-17.md#example-477)Try It



```
__<a href="__">
```






```
<p>__<a href="__"></p>
```




<a id="example-478"></a>

[Example 478](017-continuation-17.md#example-478)Try It



```
*a `*`*
```






```
<p><em>a <code>*</code></em></p>
```




<a id="example-479"></a>

[Example 479](017-continuation-17.md#example-479)Try It



```
_a `_`_
```






```
<p><em>a <code>_</code></em></p>
```




<a id="example-480"></a>

[Example 480](017-continuation-17.md#example-480)Try It



```
**a<https://foo.bar/?q=**>
```






```
<p>**a<a href="https://foo.bar/?q=**">https://foo.bar/?q=**</a></p>
```




<a id="example-481"></a>

[Example 481](017-continuation-17.md#example-481)Try It



```
__a<https://foo.bar/?q=__>
```






```
<p>__a<a href="https://foo.bar/?q=__">https://foo.bar/?q=__</a></p>
```




