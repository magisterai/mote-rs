# 6.1 Non-tailorable Line Breaking Rules

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](017-5-5-use-of-double-hyphen.md) · [Next](019-6-2-tailorable-line-breaking-rules.md)

<a id="BreakingRules"></a>

### 6.1  [Non-tailorable Line Breaking Rules](018-6-1-non-tailorable-line-breaking-rules.md#BreakingRules)

The rules in this subsection and the membership
in the classes [BK](008-bb-break-before.md#BK), [CM](009-cb-contingent-break-opportunity.md#CM),
[CR](010-cp-closing-parenthesis.md#CR), [GL](010-cp-closing-parenthesis.md#GL),
[LF](012-hl-hebrew-letter.md#LF), [NL](012-hl-hebrew-letter.md#NL),
[SP](014-qu-quotation.md#SP),
[WJ](015-sy-symbols-allowing-break-after.md#WJ), [ZW](016-zw-zero-width-space-non-tailorable.md#ZW)
and [ZWJ](016-zw-zero-width-space-non-tailorable.md#ZWJ)
define behavior that is required of all line break
implementations; see *Section 4, [Conformance](004-3-introduction.md#Conformance)*.

***Resolve line breaking classes:***

<a id="LB1"></a>

[**LB1**](018-6-1-non-tailorable-line-breaking-rules.md#LB1) Assign a line breaking class to each code point of the input.
Resolve [AI](006-5-1-description-of-line-breaking-properties.md#AI), [CB](009-cb-contingent-break-opportunity.md#CB),
[CJ](009-cb-contingent-break-opportunity.md#CJ),
[SA](014-qu-quotation.md#SA), [SG](014-qu-quotation.md#SG),
and [XX](015-sy-symbols-allowing-break-after.md#XX) into other line
breaking classes depending on criteria outside the scope of this algorithm.

In the absence of such criteria all characters with a specific
combination of original class and
General\_Category property value are resolved as follows:

| Resolved | Original | General\_Category |
| --- | --- | --- |
| [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) | [AI](006-5-1-description-of-line-breaking-properties.md#AI), [SG](014-qu-quotation.md#SG), [XX](015-sy-symbols-allowing-break-after.md#XX) | Any |
| [CM](009-cb-contingent-break-opportunity.md#CM) | [SA](014-qu-quotation.md#SA) | Only Mn or Mc |
| [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) | [SA](014-qu-quotation.md#SA) | Any except Mn and Mc |
| [NS](013-ns-nonstarters.md#NS) | [CJ](009-cb-contingent-break-opportunity.md#CJ) | Any |

***Start and end of text:***

There are two special logical positions: **sot**, which occurs before the first character in the text, and
**eot,** which occurs after the last character in the text. Thus an
empty string would consist of **sot** followed immediately by **eot**. With these
two definitions, the line break rules for start and end of text can be
specified as follows:

<a id="LB2"></a>

[**LB2**](018-6-1-non-tailorable-line-breaking-rules.md#LB2) Never break at the start of text.

sot ×

<a id="LB3"></a>

[**LB3**](018-6-1-non-tailorable-line-breaking-rules.md#LB3) Always break at the end of text.

! eot

These two rules are designed to deal with degenerate cases, so that there
is at least one character on each line, and at least one line break for
the whole text. Emergency line breaking behavior usually also allows line
breaks anywhere on the line if a legal line break cannot be found. This has
the effect of preventing text from running into the margins.

***Mandatory breaks:***

A hard line break can consist of [BK](008-bb-break-before.md#BK) or a Newline
Function (NLF) as described in *Section 5.8, Newline Guidelines*,
of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]. These three rules are
designed to handle the line ending and line separating characters as
described there.

<a id="LB4"></a>

[**LB4**](018-6-1-non-tailorable-line-breaking-rules.md#LB4) Always break after hard line
breaks.

BK !

<a id="LB5"></a>

[**LB5**](018-6-1-non-tailorable-line-breaking-rules.md#LB5) Treat [CR](010-cp-closing-parenthesis.md#CR)
followed by [LF](012-hl-hebrew-letter.md#LF), as well as [CR](010-cp-closing-parenthesis.md#CR),
[LF](012-hl-hebrew-letter.md#LF), and [NL](012-hl-hebrew-letter.md#NL)
as hard line breaks.

CR × LF

CR !

LF !

NL !

> **Note:** When displaying source code, failing to support all forms of the new line function
> can have security implications; for instance, executable code can appear commented out.
> It is therefore strongly recommended that source code editors support the VT character
> within the BK class, and support the NEL character within the NL class, even though that support is
> not required for conformance.
> See *Unicode Technical Standard #55,
> Unicode Source Code Handling* [[UTS55](https://www.unicode.org/reports/tr41/tr41-36.html#UTS55)].

<a id="LB6"></a>

[**LB6**](018-6-1-non-tailorable-line-breaking-rules.md#LB6) Do not break before hard line breaks.

× ( BK | CR | LF | NL )

***Explicit breaks and non-breaks:***

<a id="LB7"></a>

[**LB7**](018-6-1-non-tailorable-line-breaking-rules.md#LB7) Do not break before spaces or zero
width space.

× SP

× ZW

<a id="LB8"></a>

[**LB8**](018-6-1-non-tailorable-line-breaking-rules.md#LB8) Break before any character following a zero-width space,
even if one or more spaces intervene.

ZW SP\* ÷

<a id="LB8a"></a>

[**LB8a**](018-6-1-non-tailorable-line-breaking-rules.md#LB8a)
Do not break after a
[zero width joiner](016-zw-zero-width-space-non-tailorable.md#ZWJ).

ZWJ ×

A [ZWJ](016-zw-zero-width-space-non-tailorable.md#ZWJ) will prevent breaks between most pairs of characters.
This behavior is used to prevent breaks within emoji zwj sequences.

***Combining marks:***

See also *Section 9.2, [Legacy Support for Space Character as Base for Combining Marks](021-9-implementation-notes.md#LegacySpace).*

<a id="LB9"></a>

[**LB9**](018-6-1-non-tailorable-line-breaking-rules.md#LB9) Do not break a combining character sequence; treat it as
if it has the line breaking class of the base character in all of the following rules.
Treat [ZWJ](016-zw-zero-width-space-non-tailorable.md#ZWJ) as if it were [CM](009-cb-contingent-break-opportunity.md#CM).

Treat X (CM | ZWJ)\* as if it were X.

where X is any line break class except [BK](008-bb-break-before.md#BK),
[CR](010-cp-closing-parenthesis.md#CR), [LF](012-hl-hebrew-letter.md#LF),
[NL](012-hl-hebrew-letter.md#NL), [SP](014-qu-quotation.md#SP), or
[ZW](016-zw-zero-width-space-non-tailorable.md#ZW).

In subsequent rules, any [CM](009-cb-contingent-break-opportunity.md#CM) or [ZWJ](016-zw-zero-width-space-non-tailorable.md#ZWJ) characters affected by this rule are ignored.
Note that despite the summary title, this rule is not limited to
standard combining character sequences. For the purposes
of line breaking, sequences containing most of the control codes or layout
control characters are treated like combining sequences.

<a id="LB10"></a>

[**LB10**](018-6-1-non-tailorable-line-breaking-rules.md#LB10) Treat any remaining
[combining mark](009-cb-contingent-break-opportunity.md#CM) or [ZWJ](016-zw-zero-width-space-non-tailorable.md#ZWJ) as
[AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).

Treat any remaining CM or ZWJ as if it had the properties of U+0041 A LATIN CAPITAL LETTER A, that is, Line\_Break=AL, General\_Category=Lu, East\_Asian\_Width=Na, Extended\_Pictographic=N.

This catches the case where a [CM](009-cb-contingent-break-opportunity.md#CM) is the first character on the line or
follows [SP](014-qu-quotation.md#SP), [BK](008-bb-break-before.md#BK),
[CR](010-cp-closing-parenthesis.md#CR), [LF](012-hl-hebrew-letter.md#LF),
[NL](012-hl-hebrew-letter.md#NL), or [ZW](016-zw-zero-width-space-non-tailorable.md#ZW).

***Word joiner:***

<a id="LB11"></a>

[**LB11**](018-6-1-non-tailorable-line-breaking-rules.md#LB11) Do not break before or after
Word joiner and related
characters.

× WJ

WJ ×

***Non-breaking characters:***

<a id="LB12"></a>

[**LB12**](018-6-1-non-tailorable-line-breaking-rules.md#LB12)
Do not break after NBSP and related characters.

GL ×

