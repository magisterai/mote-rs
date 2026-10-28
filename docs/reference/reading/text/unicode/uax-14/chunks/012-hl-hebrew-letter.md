# ** HL**: Hebrew Letter

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-h2-hangul-lv-syllable.md) · [Next](013-ns-nonstarters.md)

<a id="HL"></a>

### ** [HL](012-hl-hebrew-letter.md#HL)**: Hebrew Letter

*[LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a), [LB21a](019-6-2-tailorable-line-breaking-rules.md#LB21a), [LB21b](019-6-2-tailorable-line-breaking-rules.md#LB21b), [LB23](019-6-2-tailorable-line-breaking-rules.md#LB23), [LB24](019-6-2-tailorable-line-breaking-rules.md#LB24), [LB28](019-6-2-tailorable-line-breaking-rules.md#LB28), [LB29](019-6-2-tailorable-line-breaking-rules.md#LB29), [LB30](019-6-2-tailorable-line-breaking-rules.md#LB30)*

This class includes all Hebrew letters.

When a Hebrew letter is separated from following non-Hebrew text by a hyphen, there is no break on either side of the hyphen.
In this context a hyphen is any character of class [HY](011-h2-hangul-lv-syllable.md#HY)
or class [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA).
There is also no break between a solidus and a Hebrew letter.
In other respects, Hebrew letters behave the same as characters of class
[AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).

Included in this class are all characters of General Category Letter that have Script=Hebrew.

<a id="IN"></a>

### ** [IN](012-hl-hebrew-letter.md#IN)**: Inseparable Characters

*[LB22](019-6-2-tailorable-line-breaking-rules.md#LB22)*

#### Leaders

These characters are intended to be used consecutively.
There is never a line break between two characters of this class.

Examples include:

|  |  |
| --- | --- |
| 2024 | ONE DOT LEADER |
| 2025 | TWO DOT LEADER |
| 2026 | HORIZONTAL ELLIPSIS |
| FE19 | PRESENTATION FORM FOR VERTICAL HORIZONTAL ELLIPSIS |

HORIZONTAL ELLIPSIS can be used as a three-dot leader.

<a id="IS"></a>

### ** [IS](012-hl-hebrew-letter.md#IS)**: Infix Numeric Separator

*[LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB15c](019-6-2-tailorable-line-breaking-rules.md#LB15c), [LB15d](019-6-2-tailorable-line-breaking-rules.md#LB15d), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25), [LB29](019-6-2-tailorable-line-breaking-rules.md#LB29)*

Characters that usually occur inside a numerical expression may not be
separated from the numeric characters that follow, unless a space character
intervenes. For example, there is no break in “100.00” or “10,000”, nor in “12:59”.

Examples include:

|  |  |
| --- | --- |
| 002C | COMMA |
| 002E | FULL STOP |
| 003A | COLON |
| 003B | SEMICOLON |
| 037E | GREEK QUESTION MARK (canonically equivalent to 003B) |
| 0589 | ARMENIAN FULL STOP |
| 060C | ARABIC COMMA |
| 060D | ARABIC DATE SEPARATOR |
| 07F8 | NKO COMMA |
| 2044 | FRACTION SLASH |

When not used in a numeric context, infix separators are sentence-ending punctuation.
Therefore they always prevent breaks before.

> Note:
> FIGURE SPACE, not being a punctuation mark, has
> been given the line break class [GL](010-cp-closing-parenthesis.md#GL).

<a id="JL"></a>

### [JL](012-hl-hebrew-letter.md#JL): Hangul L Jamo

*[LB26](019-6-2-tailorable-line-breaking-rules.md#LB26), [LB27](019-6-2-tailorable-line-breaking-rules.md#LB27)*

The [JL](012-hl-hebrew-letter.md#JL) line break class consists of all characters of Hangul Syllable Type L.

Conjoining jamos form Korean Syllable Blocks, which are kept together; see
Unicode Standard Annex #29, “Unicode Text Segmentation” [[UAX29](https://www.unicode.org/reports/tr41/tr41-36.html#UAX29)].
Korean uses space-based line breaking in many styles of documents. To support
these, Hangul syllables and conjoining jamos need to be tailored
to use class  [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL). The default in this specification is
class  [ID](011-h2-hangul-lv-syllable.md#ID), which supports the case of Korean documents not using space-based
line breaking. See *Section 8.1, [Types of Tailoring](020-7-deleted.md#Tailoring)*. See also
 [JT](012-hl-hebrew-letter.md#JT), [JV](012-hl-hebrew-letter.md#JV),
[H2](011-h2-hangul-lv-syllable.md#H2), and
[H3](011-h2-hangul-lv-syllable.md#H3).

<a id="JT"></a>

### [JT](012-hl-hebrew-letter.md#JT): Hangul T Jamo

*[LB26](019-6-2-tailorable-line-breaking-rules.md#LB26), [LB27](019-6-2-tailorable-line-breaking-rules.md#LB27)*

The [JT](012-hl-hebrew-letter.md#JT) line break class consists of all characters of Hangul Syllable Type T. See also
[JL](012-hl-hebrew-letter.md#JL),
[JV](012-hl-hebrew-letter.md#JV),
[H2](011-h2-hangul-lv-syllable.md#H2), and
[H3](011-h2-hangul-lv-syllable.md#H3).

<a id="JV"></a>

### ** [JV](012-hl-hebrew-letter.md#JV)**: Hangul V Jamo

*[LB26](019-6-2-tailorable-line-breaking-rules.md#LB26), [LB27](019-6-2-tailorable-line-breaking-rules.md#LB27)*

The [JV](012-hl-hebrew-letter.md#JV) line break class consists of all characters of Hangul Syllable Type V. See also
[JL](012-hl-hebrew-letter.md#JL),
[JT](012-hl-hebrew-letter.md#JT),
[H2](011-h2-hangul-lv-syllable.md#H2), and
[H3](011-h2-hangul-lv-syllable.md#H3).

<a id="LF"></a>

### ** [LF](012-hl-hebrew-letter.md#LF)**: Line Feed (Non-tailorable)

*[LB5](018-6-1-non-tailorable-line-breaking-rules.md#LB5), [LB6](018-6-1-non-tailorable-line-breaking-rules.md#LB6), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

|  |  |
| --- | --- |
| 000A | LINE FEED (LF) |

There is a mandatory break after any LF character, but see the discussion
under [BK](008-bb-break-before.md#BK).

<a id="NL"></a>

### [NL](012-hl-hebrew-letter.md#NL): Next Line (Non-tailorable)

*[LB5](018-6-1-non-tailorable-line-breaking-rules.md#LB5), [LB6](018-6-1-non-tailorable-line-breaking-rules.md#LB6), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

|  |  |
| --- | --- |
| 0085 | NEXT LINE (NEL) |

The [NL](012-hl-hebrew-letter.md#NL) class acts like [BK](008-bb-break-before.md#BK)
in all respects (there is a mandatory break after any NEL character).
It cannot be tailored, but implementations are not required to support the
NEL character; see the discussion
under [BK](008-bb-break-before.md#BK).

