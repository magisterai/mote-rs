# 6.2 Tailorable Line Breaking Rules

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-6-1-non-tailorable-line-breaking-rules.md) · [Next](020-7-deleted.md)

<a id="TailorableBreakingRules"></a>

### 6.2  [Tailorable Line Breaking Rules](019-6-2-tailorable-line-breaking-rules.md#TailorableBreakingRules)

The following rules and the classes referenced in them provide a reasonable
default set of line break opportunities. Implementations should implement them
unless alternate approaches produce better results for some classes of
text or applications. When using alternative rules or algorithms, implementations
must ensure that the mandatory breaks, break opportunities and non-break positions
determined by the algorithm and rules of *Section 6.1, [Non-tailorable Line Breaking Rules](018-6-1-non-tailorable-line-breaking-rules.md#BreakingRules)*,
are preserved. See
*Section 4, [Conformance](004-3-introduction.md#Conformance)*.

***Non-breaking characters:***

<a id="LB12a"></a>

[**LB12a**](019-6-2-tailorable-line-breaking-rules.md#LB12a) Do not break before NBSP and related characters,
except after spaces and hyphens.

[^SP BA HY HH] × GL

The expression [^SP
BA HY HH] designates any line break class other than
[SP](014-qu-quotation.md#SP), [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA), [HY](011-h2-hangul-lv-syllable.md#HY), or [HH](011-h2-hangul-lv-syllable.md#HH). The symbol ^ is used, instead of !,
to avoid confusion with the use of ! to indicate an explicit break. Unlike the case for
[WJ](015-sy-symbols-allowing-break-after.md#WJ), inserting a
[SP](014-qu-quotation.md#SP) overrides the non-breaking nature of
a [GL](010-cp-closing-parenthesis.md#GL). Allowing
a break after [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA) or
[HY](011-h2-hangul-lv-syllable.md#HY) matches widespread implementation practice
and supports a common way of handling special line breaking of
explicit hyphens, such as in Polish and Portuguese. See
*Section 5.3, [Use of Hyphen](016-zw-zero-width-space-non-tailorable.md#Hyphen)*.

***Opening and closing:***

These have special behavior with respect to spaces, and therefore come before rule
LB18.

<a id="LB13"></a>

[**LB13**](019-6-2-tailorable-line-breaking-rules.md#LB13)
Do not break before ‘]’ or ‘!’ or ‘/’, even after spaces.

× CL

× CP

× EX

× SY

<a id="LB14"></a>

[**LB14**](019-6-2-tailorable-line-breaking-rules.md#LB14) Do not break after ‘[’, even after spaces.

OP SP\* ×

<a id="LB15a"></a>

[**LB15a**](019-6-2-tailorable-line-breaking-rules.md#LB15a)
Do not break after an unresolved initial punctuation that lies at the start of
the line, after a space, after opening punctuation, or after an unresolved
quotation mark, even after spaces.

(sot | BK | CR | LF | NL | OP | QU | GL | SP | ZW) [\p{Pi}&QU] SP\* ×

<a id="LB15b"></a>

[**LB15b**](019-6-2-tailorable-line-breaking-rules.md#LB15b)
Do not break before an unresolved final punctuation that lies at the end
of the line, before a space, before a prohibited break, or before an unresolved
quotation mark, even after spaces.

× [\p{Pf}&QU] ( SP | GL | WJ | CL | QU | CP | EX | IS | SY | BK | CR | LF | NL | ZW | eot)

<a id="LB15c"></a>

[**LB15c**](019-6-2-tailorable-line-breaking-rules.md#LB15c)
Break before a decimal mark that follows a space, for instance, in ‘subtract .5’.

SP ÷ IS NU

<a id="LB15d"></a>

[**LB15d**](019-6-2-tailorable-line-breaking-rules.md#LB15d)
Otherwise, do not break before ‘;’, ‘,’, or ‘.’, even after spaces.

× IS

<a id="LB16"></a>

[**LB16**](019-6-2-tailorable-line-breaking-rules.md#LB16) Do not break
between closing punctuation and a nonstarter (lb=[NS](013-ns-nonstarters.md#NS)),
even with intervening spaces.

(CL | CP) SP\* × NS

<a id="LB17"></a>

[**LB17**](019-6-2-tailorable-line-breaking-rules.md#LB17) Do not break within ‘——’, even with intervening
spaces.

B2 SP\* × B2

***Spaces:***

<a id="LB18"></a>

[**LB18**](019-6-2-tailorable-line-breaking-rules.md#LB18) Break after spaces.

SP ÷

***Special case rules:***

<a id="LB19"></a>

[**LB19**](019-6-2-tailorable-line-breaking-rules.md#LB19) Do not break before non-initial unresolved quotation marks, such as ‘ ” ’ or ‘ " ’, nor after non-final unresolved quotation marks, such as ‘ “ ’ or ‘ " ’.

× [ QU - \p{Pi} ]

[ QU - \p{Pf} ] ×

<a id="LB19a"></a>

[**LB19a**](019-6-2-tailorable-line-breaking-rules.md#LB19a) Unless surrounded by East Asian characters, do not break either side of any unresolved quotation marks.

[^$EastAsian] × QU

× QU ( [^$EastAsian] | eot )

QU × [^$EastAsian]

( sot | [^$EastAsian] ) QU ×

<a id="LB20"></a>

[**LB20**](019-6-2-tailorable-line-breaking-rules.md#LB20) Break before and after unresolved
[CB](009-cb-contingent-break-opportunity.md#CB).

÷ CB

CB ÷

Conditional breaks should be resolved external to the line breaking rules.
However, the default action is to treat unresolved [CB](009-cb-contingent-break-opportunity.md#CB) as breaking before and
after.

<a id="LB20a"></a>

[**LB20a**](019-6-2-tailorable-line-breaking-rules.md#LB20a) Do not break after a word-initial hyphen.

( sot | BK | CR | LF | NL | SP | ZW | CB | GL ) ( HY | HH ) × ( AL | HL )

<a id="LB21"></a>

[**LB21**](019-6-2-tailorable-line-breaking-rules.md#LB21) Do not break before hyphen-minus, other hyphens,
fixed-width spaces, small kana, and other non-starters, or after acute
accents.

× BA

× HH

× HY

× NS

BB ×

<a id="LB21a"></a>

[**LB21a**](019-6-2-tailorable-line-breaking-rules.md#LB21a) Do not break after the hyphen in Hebrew + Hyphen + non-Hebrew.

HL (HY | HH) × [^HL]

<a id="LB21b"></a>

[**LB21b**](019-6-2-tailorable-line-breaking-rules.md#LB21b) Do not break between Solidus and Hebrew letters.

SY × HL

<a id="LB22"></a>

[**LB22**](019-6-2-tailorable-line-breaking-rules.md#LB22) Do not break before ellipses.

× IN

*Examples:* ‘9...’, ‘a...’, ‘H...’

***Numbers:***

Do not break alphanumerics.

<a id="LB23"></a>

[**LB23**](019-6-2-tailorable-line-breaking-rules.md#LB23)
Do not break between digits and letters.

(AL | HL) × NU

NU × (AL | HL)

<a id="LB23a"></a>

[**LB23a**](019-6-2-tailorable-line-breaking-rules.md#LB23a)
Do not break between numeric prefixes and ideographs, or between
ideographs and numeric postfixes.

PR × (ID | EB | EM)

(ID | EB | EM) × PO

<a id="LB24"></a>

[**LB24**](019-6-2-tailorable-line-breaking-rules.md#LB24) Do not break between
numeric prefix/postfix and letters, or between letters and prefix/postfix.

(PR | PO) × (AL | HL)

(AL | HL) × (PR | PO)

In general, it is recommended to not break lines inside numbers of the form described
by the following regular expression:

( [PR](013-ns-nonstarters.md#PR) | [PO](013-ns-nonstarters.md#PO)) ?
( [OP](013-ns-nonstarters.md#OP) | [HY](011-h2-hangul-lv-syllable.md#HY) ) ?
[IS](012-hl-hebrew-letter.md#IS) ?
[NU](013-ns-nonstarters.md#NU) ([NU](013-ns-nonstarters.md#NU) |
[SY](015-sy-symbols-allowing-break-after.md#SY) | [IS](012-hl-hebrew-letter.md#IS)) \*
([CL](009-cb-contingent-break-opportunity.md#CL) | [CP](010-cp-closing-parenthesis.md#CP)) ?
( [PR](013-ns-nonstarters.md#PR) | [PO](013-ns-nonstarters.md#PO)) ?

*Examples:* $(12.35)    2,1234   
(12)¢    12.54¢   
.50   
₹1,00,000.00   
-1/12

The default line breaking algorithm implements this with the following
rule. Note that some cases have already been handled, such as ‘9,’, ‘[9’.

<a id="LB25"></a>

[**LB25**](019-6-2-tailorable-line-breaking-rules.md#LB25) Do not break numbers:

NU ( SY | IS )\* CL × PO

NU ( SY | IS )\* CP × PO

NU ( SY | IS )\* CL × PR

NU ( SY | IS )\* CP × PR

NU ( SY | IS )\* × PO

NU ( SY | IS )\* × PR

PO × OP NU

PO × OP IS NU

PO × NU

PR × OP NU

PR × OP IS NU

PR × NU

HY × NU

IS × NU

NU ( SY | IS )\* × NU

***Korean syllable blocks***

Conjoining jamos, Hangul syllables, or combinations of both form Korean
Syllable Blocks. Such blocks are effectively treated as if they were Hangul
syllables; no breaks can occur in the middle of a syllable block. See
Unicode Standard Annex #29, “Unicode Text Segmentation”
[[UAX29](https://www.unicode.org/reports/tr41/tr41-36.html#UAX29)],
for more information on Korean Syllable Blocks.

<a id="LB26"></a>

[**LB26**](019-6-2-tailorable-line-breaking-rules.md#LB26) Do not break a Korean
syllable.

JL × (JL | JV | H2 | H3)

(JV | H2) × (JV | JT)

(JT | H3) × JT

where the notation (JT | H3) means JT or H3.
The effective line breaking class for the syllable block matches the
line breaking class for Hangul syllables, which is  [ID](011-h2-hangul-lv-syllable.md#ID)
by default. This is achieved by
the following rule:

<a id="LB27"></a>

[**LB27**](019-6-2-tailorable-line-breaking-rules.md#LB27) Treat a Korean Syllable Block the
same as [ID](011-h2-hangul-lv-syllable.md#ID).

(JL | JV | JT | H2 | H3) × PO

PR × (JL | JV | JT | H2 | H3)

When Korean uses SPACE for line breaking, the classes in rule
[LB26](019-6-2-tailorable-line-breaking-rules.md#LB26), as well as characters of
class [ID](011-h2-hangul-lv-syllable.md#ID),
are often tailored to [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL); see
*Section 8, [Customization](020-7-deleted.md#Customization)*.

***Finally, join alphabetic letters into words and break everything else.***

<a id="LB28"></a>

[**LB28**](019-6-2-tailorable-line-breaking-rules.md#LB28) Do not break between alphabetics (“at”).

(AL | HL) × (AL | HL)

<a id="LB28a"></a>

[**LB28a**](019-6-2-tailorable-line-breaking-rules.md#LB28a) Do not break inside the orthographic syllables of Brahmic scripts.

AP × (AK | [◌] | AS)

(AK | [◌] | AS) × (VF | VI)

(AK | [◌] | AS) VI × (AK | [◌])

(AK | [◌] | AS) × (AK | [◌] | AS) VF

> Note: In the above regular expressions,
> the class [◌] contains the single character U+25CC DOTTED CIRCLE.

<a id="LB29"></a>

[**LB29**](019-6-2-tailorable-line-breaking-rules.md#LB29) Do not break between numeric punctuation
and alphabetics (“e.g.”).

IS × (AL | HL)

<a id="LB30"></a>

[**LB30**](019-6-2-tailorable-line-breaking-rules.md#LB30) Do not break between
letters, numbers, or ordinary symbols and opening or closing parentheses.

(AL | HL | NU) × [OP-$EastAsian]

[CP-$EastAsian] × (AL | HL | NU)

The purpose of this rule is to prevent breaks in common cases where a part of a word
appears between delimiters—for example, in “person(s)”.

The excluded set ($EastAsian) refines the behavior of this rule, to enable
a break before an East Asian OP or after an East Asian CP. Those cases are identified by
excluding East\_Asian\_Width values of Fullwidth, Wide, or Halfwidth.
This is illustrated by the following
example, which shows East Asian corner brackets immediately following a Latin letter
in Japanese text. In such a case, the preferred line break is between the Latin letter and
the opening angle bracket.

| Preferred | Bad Break |
| --- | --- |
| 日中韓統合漢字拡張G 「ユニコード」 | 日中韓統合漢字拡張 G「ユニコード」 |

<a id="LB30a"></a>

[**LB30a**](019-6-2-tailorable-line-breaking-rules.md#LB30a)
Break between two regional indicator symbols if and only if there are an even number of
regional indicators preceding the position of the break.

sot (RI RI)\* RI × RI

[^RI] (RI RI)\* RI × RI

<a id="LB30b"></a>

[**LB30b**](019-6-2-tailorable-line-breaking-rules.md#LB30b)
Do not break between an [emoji base](010-cp-closing-parenthesis.md#EB) (or potential emoji) and an
[emoji modifier](010-cp-closing-parenthesis.md#EM).

EB × EM

[\p{Extended\_Pictographic}&\p{Cn}] × EM

<a id="LB31"></a>

[**LB31**](019-6-2-tailorable-line-breaking-rules.md#LB31) Break everywhere else.

ALL ÷

÷ ALL

