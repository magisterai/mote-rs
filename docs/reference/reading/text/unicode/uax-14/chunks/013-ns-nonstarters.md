# ** NS**: Nonstarters

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-hl-hebrew-letter.md) · [Next](014-qu-quotation.md)

<a id="NS"></a>

### ** [NS](013-ns-nonstarters.md#NS)**: Nonstarters

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1), [LB16](019-6-2-tailorable-line-breaking-rules.md#LB16), [LB21](019-6-2-tailorable-line-breaking-rules.md#LB21)*

Nonstarter characters cannot start a line, but unlike [CL](009-cb-contingent-break-opportunity.md#CL) they may allow a
break in some contexts when they follow one or more space characters.
Nonstarters include:

|  |  |
| --- | --- |
| 17D6 | KHMER SIGN CAMNUC PII KUUH |
| 203C | DOUBLE EXCLAMATION MARK |
| 203D | INTERROBANG |
| 2047 | DOUBLE QUESTION MARK |
| 2048 | QUESTION EXCLAMATION MARK |
| 2049 | EXCLAMATION QUESTION MARK |
| 3005 | IDEOGRAPHIC ITERATION MARK |
| 301C | WAVE DASH |
| 303C | MASU MARK |
| 303B | VERTICAL IDEOGRAPHIC ITERATION MARK |
| 309B.. 309E | KATAKANA-HIRAGANA VOICED SOUND MARK..HIRAGANA VOICED ITERATION MARK |
| 30A0 | KATAKANA-HIRAGANA DOUBLE HYPHEN |
| 30FB | KATAKANA MIDDLE DOT |
| 30FD..30FE | KATAKANA ITERATION MARK..KATAKANA VOICED ITERATION MARK |
| FE13 | PRESENTATION FORM FOR VERTICAL COLON |
| FE54..FE55 | SMALL SEMICOLON..SMALL COLON |
| FF1A..FF1B | FULLWIDTH COLON.. FULLWIDTH SEMICOLON |
| FF65 | HALFWIDTH KATAKANA MIDDLE DOT |
| FF9E..FF9F | HALFWIDTH KATAKANA VOICED SOUND MARK..HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK |

> Note: Optionally, the [NS](013-ns-nonstarters.md#NS)
> restriction may be relaxed by tailoring, with some or all
> characters treated like [ID](011-h2-hangul-lv-syllable.md#ID) to achieve a more permissive style of
> line breaking, especially in some East Asian document styles.
> Alternatively, line breaking can be tightened by moving characters that are
> [ID](011-h2-hangul-lv-syllable.md#ID) into [NS](013-ns-nonstarters.md#NS).

For additional information about U+30A0
KATAKANA-HIRAGANA DOUBLE HYPHEN, see *Section 5.5, [Use of Double Hyphen](017-5-5-use-of-double-hyphen.md#DoubleHyphen)*.

<a id="NU"></a>

### ** [NU](013-ns-nonstarters.md#NU):** Numeric

*[LB15c](019-6-2-tailorable-line-breaking-rules.md#LB15c), [LB23](019-6-2-tailorable-line-breaking-rules.md#LB23), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25), [LB30](019-6-2-tailorable-line-breaking-rules.md#LB30)*

These characters behave like ordinary characters ([AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL)) in the context of
most characters
but activate the prefix and postfix behavior of prefix and postfix characters.

Numeric characters consist of decimal digits (all characters of General\_Category Nd), except:

1. those with East\_Asian\_Width F (Fullwidth)
2. those from scripts that use the Brahmic style of context analysis

plus these characters:

|  |  |
| --- | --- |
| 066B | ARABIC DECIMAL SEPARATOR |
| 066C | ARABIC THOUSANDS SEPARATOR |

Unlike [IS](012-hl-hebrew-letter.md#IS) characters, the Arabic numeric
punctuation does not occur as sentence terminal punctuation outside numbers.

<a id="OP"></a>

### ** [OP](013-ns-nonstarters.md#OP)**: Open Punctuation

*[LB14](019-6-2-tailorable-line-breaking-rules.md#LB14), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25), [LB30](019-6-2-tailorable-line-breaking-rules.md#LB30)*

The opening character of any set of paired punctuation
should be kept with the character that follows. This is desirable,
even if there are intervening space characters, as it prevents the
appearance of a bare opening punctuation mark at the end of a line.
The [OP](013-ns-nonstarters.md#OP) line break
class consists of all characters of General\_Category Ps in the Unicode
Character Database, plus

|  |  |
| --- | --- |
| 00A1 | INVERTED EXCLAMATION MARK |
| 00BF | INVERTED QUESTION MARK |
| 2E18 | INVERTED INTERROBANG |

> *Note:* The first two of these characters used to be
> in the class [AI](006-5-1-description-of-line-breaking-properties.md#AI)
> based on their East\_Asian\_Width assignment of A. Such characters are
> normally resolved to either [ID](011-h2-hangul-lv-syllable.md#ID) or
> [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).
> However, the characters listed above are used as punctuation marks in
> Spanish, where they would behave more like a character of class
> [OP](013-ns-nonstarters.md#OP).

<a id="PO"></a>

### ** [PO](013-ns-nonstarters.md#PO):** Postfix Numeric

*[LB23a](019-6-2-tailorable-line-breaking-rules.md#LB23a), [LB24](019-6-2-tailorable-line-breaking-rules.md#LB24), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25), [LB27](019-6-2-tailorable-line-breaking-rules.md#LB27)*

Characters that usually follow a numerical expression may not be separated
from preceding numeric characters or preceding closing characters. For example, there is no break opportunity in “(12.00)%”.

Some of these characters—in
particular, *degree sign* and *percent sign*—can appear on both sides of a numeric
expression. Therefore the line breaking algorithm by default does not break
between [PO](013-ns-nonstarters.md#PO) and
numbers or letters on either side.

Examples of Postfix characters include

|  |  |
| --- | --- |
| 0025 | PERCENT SIGN |
| 00A2 | CENT SIGN |
| 00B0 | DEGREE SIGN |
| 060B | AFGHANI SIGN |
| 066A | ARABIC PERCENT SIGN |
| 2030 | PER MILLE SIGN |
| 2031 | PER TEN THOUSAND SIGN |
| 2032..2037 | PRIME..REVERSED TRIPLE PRIME |
| 20A7 | PESETA SIGN |
| 2103 | DEGREE CELSIUS |
| 2109 | DEGREE FAHRENHEIT |
| FDFC | RIAL SIGN |
| FE6A | SMALL PERCENT SIGN |
| FF05 | FULLWIDTH PERCENT SIGN |
| FFE0 | FULLWIDTH CENT SIGN |

Alphabetic characters are also widely used as unit designators in a postfix position. For purposes of
line breaking, their classification as
alphabetic is sufficient to keep them together with the preceding number.

<a id="PR"></a>

### ** [PR](013-ns-nonstarters.md#PR)**: Prefix Numeric

*[LB23a](019-6-2-tailorable-line-breaking-rules.md#LB23a), [LB24](019-6-2-tailorable-line-breaking-rules.md#LB24), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25), [LB27](019-6-2-tailorable-line-breaking-rules.md#LB27)*

Characters that usually precede a numerical expression may not be separated
from following numeric characters or following opening characters. For example, there is no break opportunity in “$(100.00)”.

Many currency signs can appear on
both sides, or even the middle, of a numeric expression. Therefore the
line breaking algorithm, by default, does not break between [PR](013-ns-nonstarters.md#PR) and
numbers or letters on either side.

All currency symbols
(General\_Category Sc) except those in class [PO](013-ns-nonstarters.md#PO)
have been assigned line breaking class [PR](013-ns-nonstarters.md#PR).
This class also contains all unassigned code points in the Currency Symbols block,
and additional characters, including:

|  |  |
| --- | --- |
| 002B | PLUS SIGN |
| 005C | REVERSE SOLIDUS |
| 00B1 | PLUS-MINUS SIGN |
| 2116 | NUMERO SIGN |
| 2212 | MINUS SIGN |
| 2213 | MINUS-OR-PLUS SIGN |

> Note: Many currency symbols may be used either as prefix or as
> postfix, depending on local convention. For details on the conventions used,
> see [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)].

