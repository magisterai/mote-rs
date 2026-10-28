# ** CP**: Closing Parenthesis

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-cb-contingent-break-opportunity.md) · [Next](011-h2-hangul-lv-syllable.md)

<a id="CP"></a>

### ** [CP](010-cp-closing-parenthesis.md#CP)**: Closing Parenthesis

*[LB13](019-6-2-tailorable-line-breaking-rules.md#LB13), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB16](019-6-2-tailorable-line-breaking-rules.md#LB16), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25), [LB30](019-6-2-tailorable-line-breaking-rules.md#LB30)*

This class contains two common characters, U+0029 RIGHT PARENTHESIS
and U+005D RIGHT SQUARE BRACKET. It also contains
closing brackets used in phonetic notations.
Characters of class [CP](010-cp-closing-parenthesis.md#CP) differ from those of the
[CL](009-cb-contingent-break-opportunity.md#CL) (Close Punctuation) class
in that they will not cause a break opportunity when appearing in contexts like “(s)he.”
In all other respects the breaking behavior of [CP](010-cp-closing-parenthesis.md#CP) and
[CL](009-cb-contingent-break-opportunity.md#CL) are the same.

|  |  |
| --- | --- |
| 0029 | RIGHT PARENTHESIS |
| 005D | RIGHT SQUARE BRACKET |
| 2E56 | RIGHT SQUARE BRACKET WITH STROKE |
| 2E58 | RIGHT SQUARE BRACKET WITH DOUBLE STROKE |
| 2E5A | TOP HALF RIGHT PARENTHESIS |
| 2E5C | BOTTOM HALF RIGHT PARENTHESIS |

<a id="CR"></a>

### ** [CR](010-cp-closing-parenthesis.md#CR)**: Carriage Return (Non-tailorable)

*[LB5](018-6-1-non-tailorable-line-breaking-rules.md#LB5), [LB6](018-6-1-non-tailorable-line-breaking-rules.md#LB6), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

|  |  |
| --- | --- |
| 000D | CARRIAGE RETURN (CR) |

A [CR](010-cp-closing-parenthesis.md#CR) indicates a mandatory break after, unless followed by
a [LF](012-hl-hebrew-letter.md#LF). See also the discussion
under [BK](008-bb-break-before.md#BK).

> Note: On some platforms the character sequence
> <CR, CR, LF> is used to indicate
> the location of actual line breaks, whereas <CR, LF> is treated like a hard line
> break. As soon as a user edits the text, the location of all the <CR, CR, LF>
> sequences may change as the new text breaks differently, while the relative
> position of any <CR, LF> to the surrounding text stays the same. This
> convention allows an editor to return a buffer and the client to tell which text is displayed on
> which line by counting the number of <CR, CR, LF> and <CR, LF> sequences.
> This convention is essentially equivalent to
> markup that captures the result of applying the line break algorithm, not a
> tailoring of the CR character. The <CR, CR, LF> sequences are thus not
> considered part of
> the plain text content.

<a id="EB"></a>

### ** [EB](010-cp-closing-parenthesis.md#EB)**: Emoji Base

*[LB23a](019-6-2-tailorable-line-breaking-rules.md#LB23a), [LB30b](019-6-2-tailorable-line-breaking-rules.md#LB30b)*

This class includes characters whose appearance can be modified by a subsequent emoji modifier
in an emoji modifier sequence. This class directly corresponds to the
Emoji\_Modifier\_Base property as defined in *Section 1.4.4 Emoji Modifiers* of
[[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)].

Examples include:

|  |  |
| --- | --- |
| 1F466 | BOY |
| 1F478 | PRINCESS |
| 1F6B4 | BICYCLIST |

Breaks within emoji modifier sequences are prevented by rule [LB30b](019-6-2-tailorable-line-breaking-rules.md#LB30b).
In other contexts, characters of class EB behave similarly to ideographs of class
[ID](011-h2-hangul-lv-syllable.md#ID), with break opportunities before and after.

<a id="EM"></a>

### ** [EM](010-cp-closing-parenthesis.md#EM)**: Emoji Modifier

*[LB23a](019-6-2-tailorable-line-breaking-rules.md#LB23a), [LB30b](019-6-2-tailorable-line-breaking-rules.md#LB30b)*

This class includes characters that can be used to modify
the appearance of a preceding emoji in an emoji modifier sequence.
This class directly corresponds to the Emoji\_Modifier property
as defined in *Section 1.4.4 Emoji Modifiers* of [[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)].

Breaks within emoji modifier sequences are prevented by rule [LB30b](019-6-2-tailorable-line-breaking-rules.md#LB30b).

Emoji modifiers include:

|  |  |
| --- | --- |
| 1F3FB..1F3FF | EMOJI MODIFIER FITZPATRICK TYPE-1-2..EMOJI MODIFIER FITZPATRICK TYPE-6 |

<a id="EX"></a>

### ** [EX](010-cp-closing-parenthesis.md#EX)**: Exclamation/Interrogation

*[LB13](019-6-2-tailorable-line-breaking-rules.md#LB13), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b)*

Characters in this line break class behave like closing characters, except in relation to postfix
([PO](013-ns-nonstarters.md#PO)) and
non-starter characters ([NS](013-ns-nonstarters.md#NS)).
Examples include:

|  |  |
| --- | --- |
| 0021 | EXCLAMATION MARK |
| 003F | QUESTION MARK |
| 05C6 | HEBREW PUNCTUATION NUN HAFUKHA |
| 061B | ARABIC SEMICOLON |
| 061E | ARABIC TRIPLE DOT PUNCTUATION MARK |
| 061F | ARABIC QUESTION MARK |
| 06D4 | ARABIC FULL STOP |
| 07F9 | NKO EXCLAMATION MARK |
| 0F0D | TIBETAN MARK SHAD |
| FF01 | FULLWIDTH EXCLAMATION MARK |
| FF1F | FULLWIDTH QUESTION MARK |

<a id="GL"></a>

### ** [GL](010-cp-closing-parenthesis.md#GL)**: Non-breaking (“Glue”) (Non-tailorable)

*[LB12](018-6-1-non-tailorable-line-breaking-rules.md#LB12), [LB12a](019-6-2-tailorable-line-breaking-rules.md#LB12a), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

Non-breaking characters prohibit breaks on either
side, but that prohibition can be overridden by  [SP](014-qu-quotation.md#SP) or
[ZW](016-zw-zero-width-space-non-tailorable.md#ZW).
In particular, when NO-BREAK SPACE
follows SPACE, there is a break opportunity after
the SPACE and the NO-BREAK SPACE
will go as visible space onto the next line.
See also [WJ](015-sy-symbols-allowing-break-after.md#WJ). The following are examples of characters of line break
class [GL](010-cp-closing-parenthesis.md#GL):

|  |  |
| --- | --- |
| 00A0 | NO-BREAK SPACE (NBSP) |
| 202F | NARROW NO-BREAK SPACE (NNBSP) |
| 180E | MONGOLIAN VOWEL SEPARATOR (MVS) |

NO-BREAK SPACE is the preferred character to use where two words
are to be visually separated but kept on the same line, as in the case of a title and a
name “Dr.<NBSP>Joseph Becker”. When SPACE follows NO-BREAK SPACE,
there is no break, because there never is a break in front of SPACE.

<a id="NNBSPdoc"></a>

NARROW NO-BREAK SPACE has exactly the same line breaking behavior
as NO-BREAK SPACE,
but with a narrow display width.
The MONGOLIAN VOWEL SEPARATOR
acts like a NARROW NO-BREAK SPACE
in its line breaking behavior. Both of these characters are regularly used in
Mongolian text, where they participate in special shaping behavior,
as described in *Section 13.5, Mongolian* of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

When NARROW NO-BREAK SPACE
occurs in French text, it should be interpreted as an “espace fine
insécable”.

|  |  |
| --- | --- |
| 1107F | BRAHMI NUMBER JOINER |
| 13430..13436 | EGYPTIAN HIEROGLYPH VERTICAL JOINER..EGYPTIAN HIEROGLYPH OVERLAY MIDDLE |
| 13439..1343B | EGYPTIAN HIEROGLYPH INSERT AT MIDDLE..EGYPTIAN HIEROGLYPH INSERT AT BOTTOM |
| 16FE4 | KHITAN SMALL SCRIPT FILLER |

These characters participate in shaping behavior.
Together with the characters on either side, they form a ligature, quadrat,
or cluster, within which there can be no line break.
See
*Section 14.1, Brahmi*,
*Section 11.4, Egyptian Hieroglyphs*, and
*Section 18.12, Khitan Small Script*, respectively, of
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

|  |  |
| --- | --- |
| 2007 | FIGURE SPACE |

This is the preferred space to use in numbers. It has the same width as a
digit and keeps the number together for the purpose of line breaking.

|  |  |
| --- | --- |
| 2011 | NON-BREAKING HYPHEN |

This is the preferred character to use where words need to be hyphenated but
may not be broken at the hyphen. Because of its use
as a substitute for ordinary hyphen, the appearance of this character should
match that of U+2010 HYPHEN.

|  |  |
| --- | --- |
| 0F08 | TIBETAN MARK SBRUL SHAD |
| 0F0C | TIBETAN MARK DELIMITER TSHEG BSTAR |
| 0F12 | TIBETAN MARK RGYA GRAM SHAD |

The TSHEG BSTAR looks exactly like a Tibetan *tsheg*, but can be used to prevent
a break like *no-break space*. It inhibits breaking on either side. For
more information, see *Section 5.6, [Tibetan Line Breaking](017-5-5-use-of-double-hyphen.md#TibetanLinebreaking)*.

|  |  |
| --- | --- |
| 035C..0362 | COMBINING DOUBLE BREVE BELOW..COMBINING DOUBLE RIGHTWARDS ARROW BELOW |

These diacritics span two characters, so no word or line breaks are
possible on either side.

|  |  |
| --- | --- |
| FE20 | COMBINING LIGATURE LEFT HALF |
| FE22 | COMBINING DOUBLE TILDE LEFT HALF |
| FE24 | COMBINING MACRON LEFT HALF |
| FE27 | COMBINING LIGATURE LEFT HALF BELOW |
| FE29 | COMBINING TILDE LEFT HALF BELOW |
| FE2B | COMBINING MACRON LEFT HALF BELOW |
| FE2E | COMBINING CYRILLIC TITLO LEFT HALF |
| FE26 | COMBINING CONJOINING MACRON |
| FE2D | COMBINING CONJOINING MACRON BELOW |

The left half diacritics are part of a legacy representation of the
double diacritics; they occur between the two characters spanned by the double
diacritic. Preventing breaks on either side therefore achieves the same
line breaking behavior as when using the preferred representation
U+035C..U+0362.

In addition, the conjoining macrons above and below, together with left and
right half marks, form marks spanning more than two characters; likewise no
line break occurs within such spans.

