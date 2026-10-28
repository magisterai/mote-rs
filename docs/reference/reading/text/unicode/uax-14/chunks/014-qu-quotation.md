# ** QU:** Quotation

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-ns-nonstarters.md) · [Next](015-sy-symbols-allowing-break-after.md)

<a id="QU"></a>

### ** [QU](014-qu-quotation.md#QU):** Quotation

*[LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB19](019-6-2-tailorable-line-breaking-rules.md#LB19), [LB19a](019-6-2-tailorable-line-breaking-rules.md#LB19a)*

Some quotation characters can be opening or closing,
or even both, depending on usage.
The default is to use the General\_Category values Initial\_Punctation
and Final\_Punctation as a hint, together with context, but to err on the side
of treating them as both opening and closing,
thus preventing breaks on either side.
This will prevent some breaks that might have been
legal for a particular language or usage, such as
outside a Simplified Chinese quotation of Latin text, or before a German
quotation of text starting with a full stop.

> Note: If language information is available, it can be used to
> determine which character is used as the opening quote and which as the closing quote. See
> the information in *Section 6.2, General Punctuation*, in
> [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].
> In such a case, the quotation marks could be tailored to either
> [OP](013-ns-nonstarters.md#OP) or [CL](009-cb-contingent-break-opportunity.md#CL)
> depending on their actual usage.

The [QU](014-qu-quotation.md#QU) line break class consists of characters of
General\_Category Pf or Pi in the Unicode Character Database
and additional characters, including:

|  |  |
| --- | --- |
| 0022 | QUOTATION MARK |
| 0027 | APOSTROPHE |
| 275B | HEAVY SINGLE TURNED COMMA QUOTATION MARK ORNAMENT |
| 275C | HEAVY SINGLE COMMA QUOTATION MARK ORNAMENT |
| 275D | HEAVY DOUBLE TURNED COMMA QUOTATION MARK ORNAMENT |
| 275E | HEAVY DOUBLE COMMA QUOTATION MARK ORNAMENT |
| 2E00..2E01 | RIGHT ANGLE SUBSTITUTION MARKER..RIGHT ANGLE DOTTED SUBSTITUTION MARKER |
| 2E06..2E08 | RAISED INTERPOLATION MARKER..DOTTED TRANSPOSITION MARKER |
| 2E0B | RAISED SQUARE |

<a id="RI"></a>

### ** [RI](014-qu-quotation.md#RI)**: Regional Indicator

*[LB30a](019-6-2-tailorable-line-breaking-rules.md#LB30a)*

For line Breaking, the Regional Indicator characters are
all those with the Unicode character property of
Regional\_Indicator. This includes:

|  |  |
| --- | --- |
| 1F1E6..1F1FF | REGIONAL INDICATOR SYMBOL LETTER A .. REGIONAL INDICATOR SYMBOL LETTER Z |

Pairs of RI characters are used to represent a two-letter ISO 3166 region code.

Runs of adjacent RI characters are grouped into pairs, beginning at the start of the run.
No break opportunity occurs within a pair; breaks can occur between adjacent pairs.
When RI characters are adjacent to characters of other classes, breaks can occur before and after,
except where forbidden by other rules.

<a id="SA"></a>

### ** [SA](014-qu-quotation.md#SA)**: Complex-Context Dependent (South East Asian)

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1)*

Runs of these characters require morphological analysis to determine break
opportunities. This is similar to, for example, a hyphenation algorithm. For the
characters that have this property, **no** break opportunities will be
found otherwise. Therefore complex context analysis, often involving
dictionary lookup of some form, is required to determine non-emergency line
breaks. If such analysis is not available, it is recommended to treat them as
[AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).

> Note:
> These characters can be mapped into their equivalent line breaking classes
> by using dictionary lookup, thus permitting a logical
> separation of this algorithm from the morphological analysis.

The class [SA](014-qu-quotation.md#SA) consists of all characters of General\_Category Cf, Lo, Lm, Mn,
or Mc in the following blocks that are not members of another line break class.

|  |  |
| --- | --- |
| 0E00..0E7F | Thai |
| 0E80..0EFF | Lao |
| 1000..109F | Myanmar |
| 1780..17FF | Khmer |
| 1950..197F | Tai Le |
| 1980..19DF | New Tai Lue |
| 1A20..1AAF | Tai Tham |
| A9E0..A9FF | Myanmar Extended-B |
| AA60..AA7F | Myanmar Extended-A |
| AA80..AADF | Tai Viet |
| 11700..1173F | Ahom |

<a id="SG"></a>

### ** [SG](014-qu-quotation.md#SG):** Surrogate (Non-tailorable)

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1)*

Line break class [SG](014-qu-quotation.md#SG) comprises all code points with General\_Category Cs. The line breaking behavior of
isolated surrogates is undefined. In UTF-16,
paired surrogates represent non-BMP code points. Such code points must be
resolved before assigning line break properties. In UTF-8 and UTF-32
surrogate code points represent corrupted data and their line break behavior
is undefined.

> Note: The use of this line breaking class is deprecated. It was of
> limited usefulness for UTF-16 implementations that did not support characters beyond the BMP. The
> correct implementation is to resolve a *pair* of surrogates into a
> supplementary character before line breaking.

<a id="SP"></a>

### ** [SP](014-qu-quotation.md#SP)**: Space (Non-tailorable)

*[LB7](018-6-1-non-tailorable-line-breaking-rules.md#LB7), [LB8](018-6-1-non-tailorable-line-breaking-rules.md#LB8), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB12a](019-6-2-tailorable-line-breaking-rules.md#LB12a), [LB14](019-6-2-tailorable-line-breaking-rules.md#LB14), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB15c](019-6-2-tailorable-line-breaking-rules.md#LB15c), [LB16](019-6-2-tailorable-line-breaking-rules.md#LB16), [LB17](019-6-2-tailorable-line-breaking-rules.md#LB17), [LB18](019-6-2-tailorable-line-breaking-rules.md#LB18), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

The space characters are used as explicit break opportunities;
they allow line breaks before most other characters. However, spaces at the
end of a line are ordinarily not measured for fit. If there is a sequence of space
characters, and breaking after any of the space characters would result in the
same visible line, then the line breaking position after the last space character
in the sequence is the locally most optimal one. In other words, when the
last character measured for fit is *before* the space character, any number of
space characters are kept together invisibly on the previous line and the
first non-space character starts the next line.

|  |  |
| --- | --- |
| 0020 | SPACE (SP) |

> Note: By default, SPACE,
> but none of the other breaking spaces, is used in
> determining an indirect break. For other breaking space characters, see
> [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA).

