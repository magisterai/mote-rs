# ** AL**: Ordinary Alphabetic and Symbol Characters

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-5-1-description-of-line-breaking-properties.md) · [Next](008-bb-break-before.md)

<a id="AL"></a>

### ** [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL)**: Ordinary Alphabetic and Symbol Characters

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1), [LB10](018-6-1-non-tailorable-line-breaking-rules.md#LB10), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a), [LB23](019-6-2-tailorable-line-breaking-rules.md#LB23), [LB24](019-6-2-tailorable-line-breaking-rules.md#LB24), [LB28](019-6-2-tailorable-line-breaking-rules.md#LB28), [LB29](019-6-2-tailorable-line-breaking-rules.md#LB29), [LB30](019-6-2-tailorable-line-breaking-rules.md#LB30)*

Ordinary characters require other characters to provide break opportunities; otherwise, no
line breaks are allowed between pairs of them. However, this behavior is tailorable. In
some Far Eastern documents, it may be desirable to allow breaking between
pairs of ordinary characters—particularly Latin characters and symbols.

> Note: Use ZWSP as a manual override to provide break
> opportunities around alphabetic or symbol characters.

This class contains alphabetic or symbolic characters not explicitly assigned to another line breaking class. These are primarily characters of the following categories:

| Category | General\_Category Values |
| --- | --- |
| Alphabetic | Lu, Ll, Lt, Lm, and Lo |
| Symbols | Sm, Sk, and So |
| Non-decimal Numbers | Nl and No |
| Punctuation | Pc, Pd, and Po |

Line break class [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) also contains several format characters, including:

|  |  |
| --- | --- |
| 0600..0604 | ARABIC NUMBER SIGN..ARABIC SIGN SAMVAT |
| 06DD | ARABIC END OF AYAH |
| 070F | SYRIAC ABBREVIATION MARK |
| 2061..2064 | FUNCTION APPLICATION..INVISIBLE PLUS |
| 110BD | KAITHI NUMBER SIGN |

> These format characters occur in the middle or at the beginning of words or alphanumeric or symbol sequences. However, when alphabetic characters are tailored to allow breaks, these characters should not allow breaks after.

Major exceptions to the general pattern of alphabetic and symbolic characters having line break class [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) include:

> HL for Hebrew letters  
> AI or ID, based on the East Asian Width property of the character  
> ID for certain pictographic symbols  
> CJ for small hiragana and katakana  
> SA for complex context scripts  
> JL, JV, JT, H2 or H3 for Hangul characters

<a id="AP"></a>

### ** [AP](007-al-ordinary-alphabetic-and-symbol-characters.md#AP)**: Aksara Pre-Base

*[LB28a](019-6-2-tailorable-line-breaking-rules.md#LB28a)*

The [AP](007-al-ordinary-alphabetic-and-symbol-characters.md#AP) line break class is only used for scripts that use the Brahmic style of context analysis.
It contains the characters of such scripts that are part of an orthographic syllable but in logical order precede the base or any half-forms.
This includes characters with the Indic syllabic categories Consonant\_Preceding\_Repha, Consonant\_With\_Stacker, and Consonant\_Prefixed.

|  |  |
| --- | --- |
| 11003..11004 | BRAHMI SIGN JIHVAMULIYA..BRAHMI SIGN UPADHMANIYA |
| 11F02 | KAWI SIGN REPHA |

<a id="AS"></a>

### ** [AS](007-al-ordinary-alphabetic-and-symbol-characters.md#AS)**: Aksara Start

*[LB28a](019-6-2-tailorable-line-breaking-rules.md#LB28a)*

The [AS](007-al-ordinary-alphabetic-and-symbol-characters.md#AS) line break class is only used for scripts that use the Brahmic style of context analysis.
It contains characters that can occur as the bases of orthographic syllables, but cannot follow a virama of Indic syllabic category Virama or Invisible\_Stacker within the same orthographic syllable.
Depending on the script, this may include characters with the Indic syllabic categories Consonant, Vowel\_Independent,, and several others.
This class also contains all digits of scripts that use the Brahmic
style of line breaking; in some of these scripts, such as Brahmi or
Kawi, digits can occur as bases of orthographic syllables.

|  |  |
| --- | --- |
| 1B50..1B59 | BALINESE DIGIT ZERO..BALINESE DIGIT NINE |
| 1BC0..1BE5 | BATAK LETTER A..BATAK LETTER U |
| A9D0..A9D9 | JAVANESE DIGIT ZERO..JAVANESE DIGIT NINE |
| AA00..AA28 | CHAM LETTER A..CHAM LETTER HA |
| AA50..AA59 | CHAM DIGIT ZERO..CHAM DIGIT NINE |
| 11066..1106F | BRAHMI DIGIT ZERO..BRAHMI DIGIT NINE |
| 11350 | GRANTHA OM |
| 1135E..1135F | GRANTHA LETTER VEDIC ANUSVARA..GRANTHA LETTER VEDIC DOUBLE ANUSVARA |
| 11950..11959 | DIVES AKURU DIGIT ZERO..DIVES AKURU DIGIT NINE |
| 11EE0..11EF1 | MAKASAR LETTER KA..MAKASAR LETTER A |
| 11F50..11F59 | KAWI DIGIT ZERO..KAWI DIGIT NINE |

<a id="BA"></a>

### ** [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA)**: Break After

*[LB12a](019-6-2-tailorable-line-breaking-rules.md#LB12a), [LB21](019-6-2-tailorable-line-breaking-rules.md#LB21)*

Like SPACE, the characters in this class provide a break opportunity; unlike
SPACE, they do not take part in determining indirect breaks.
They can be subdivided into several categories.

#### ***Breaking Spaces***

Breaking spaces are a subset of characters with General\_Category Zs. Examples include:

|  |  |
| --- | --- |
| 1680 | OGHAM SPACE MARK |
| 2000 | EN QUAD |
| 2001 | EM QUAD |
| 2002 | EN SPACE |
| 2003 | EM SPACE |
| 2004 | THREE-PER-EM SPACE |
| 2005 | FOUR-PER-EM SPACE |
| 2006 | SIX-PER-EM SPACE |
| 2008 | PUNCTUATION SPACE |
| 2009 | THIN SPACE |
| 200A | HAIR SPACE |
| 205F | MEDIUM MATHEMATICAL SPACE |
| 3000 | IDEOGRAPHIC SPACE |

All of these space characters have a specific width, but
otherwise behave as breaking spaces. In setting a justified line, none of these spaces
normally changes in width, except for THIN SPACE
when used in mathematical notation. See also the [SP](014-qu-quotation.md#SP) property.

The OGHAM SPACE MARK may be rendered visibly between words but it is
recommended that it be elided at the end of a line. For more information,
see *Section 5.7, [Word Separator Characters](017-5-5-use-of-double-hyphen.md#WordSeparators)*.

For a list of all space characters in the Unicode Standard, see *Section 6.2, General Punctuation*,
in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

#### Tabs

|  |  |
| --- | --- |
| 0009 | TAB |

Except for the effect of the location of the tab stops, the tab character
acts similarly to a space for the purpose of line breaking.

#### Conditional Hyphens

|  |  |
| --- | --- |
| 00AD | SOFT HYPHEN (SHY) |

SHY is an invisible format character with no width. It marks
the place where an optional line break may occur inside a word.
It can be used with all scripts. If a line is broken at an optional
line break position marked by a SHY, the text at that line break
position often has a modified appearance as described in
*Section 5.4, [Use of Soft Hyphen](016-zw-zero-width-space-non-tailorable.md#SoftHyphen)*.

#### Visible Word Dividers

The following are examples of other forms of visible word dividers that
provide break opportunities:

|  |  |
| --- | --- |
| 0F0B | TIBETAN MARK INTERSYLLABIC TSHEG |
| 1361 | ETHIOPIC WORDSPACE |
| 17D8 | KHMER SIGN BEYYAL |
| 17DA | KHMER SIGN KOOMUUT |

The Tibetan *tsheg* is a visible mark, but it functions effectively
like a space to separate words (or other units) in Tibetan. It provides a
break opportunity after itself. For additional
information, see *Section 5.6, [Tibetan Line Breaking](017-5-5-use-of-double-hyphen.md#TibetanLinebreaking)*.

The ETHIOPIC WORDSPACE is a visible word delimiter and is kept on the
previous line. In contrast, U+1360 ETHIOPIC SECTION MARK is typically
used in a sequence of several such marks on a separate line, and separated by spaces. As such
lines are typically marked with separate hard line breaks ([BK](008-bb-break-before.md#BK)),
the section mark is treated like an ordinary symbol and given line break
class [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).

|  |  |
| --- | --- |
| 2027 | HYPHENATION POINT |

A hyphenation point is a raised dot, which is mainly used in dictionaries
and similar works to visibly indicate syllabification of words. Syllable
breaks frequently also are potential line break opportunities in the middle of words.
When an actual line break falls inside a word containing hyphenation
point characters, the hyphenation point is usually rendered as a regular hyphen at the
end of the line.

|  |  |
| --- | --- |
| 007C | VERTICAL LINE |

In some dictionaries, a vertical bar is used instead of a hyphenation
point. In this usage, U+0323 COMBINING DOT BELOW is
used to mark stressed syllables, so all breaks are marked by the vertical
bar. For an actual line break
the vertical bar is rendered as a hyphen at the end of the line.

#### Historic Word Separators

Historic texts, especially ancient ones, often do not use spaces, even
for scripts where modern use of spaces is standard. Special punctuation was
used to mark word boundaries in such texts. For modern text processing it is
recommended to treat these as line break opportunities by default. [WJ](015-sy-symbols-allowing-break-after.md#WJ) can
be used to override this default, where necessary.

Examples of Historic Word Separators include:

|  |  |
| --- | --- |
| 16EB | RUNIC SINGLE PUNCTUATION |
| 16EC | RUNIC MULTIPLE PUNCTUATION |
| 16ED | RUNIC CROSS PUNCTUATION |
| 2056 | THREE DOT PUNCTUATION |
| 2058 | FOUR DOT PUNCTUATION |
| 2059 | FIVE DOT PUNCTUATION |
| 205A | TWO DOT PUNCTUATION |
| 205B | FOUR DOT MARK |
| 205D | TRICOLON |
| 205E | VERTICAL FOUR DOTS |
| 2E19 | PALM BRANCH |
| 2E2A | TWO DOTS OVER ONE DOT PUNCTUATION |
| 2E2B | ONE DOT OVER TWO DOTS PUNCTUATION |
| 2E2C | SQUARED FOUR DOT PUNCTUATION |
| 2E2D | FIVE DOT MARK |
| 2E30 | RING POINT |
| 10100 | AEGEAN WORD SEPARATOR LINE |
| 10101 | AEGEAN WORD SEPARATOR DOT |
| 10102 | AEGEAN CHECK MARK |
| 1039F | UGARITIC WORD DIVIDER |
| 103D0 | OLD PERSIAN WORD DIVIDER |
| 1091F | PHOENICIAN WORD SEPARATOR |
| 12470 | CUNEIFORM PUNCTUATION SIGN OLD ASSYRIAN WORD DIVIDER |

#### Dandas

DEVANAGARI DANDA is similar to a
full stop. The *danda* or historically related symbols are used with several other
Indic scripts. Unlike a full stop, the *danda* is not used in number
formatting. DEVANAGARI DOUBLE DANDA marks the end of a verse. It also has
analogues in other scripts.

Examples of dandas include:

|  |  |
| --- | --- |
| 0964 | DEVANAGARI DANDA |
| 0965 | DEVANAGARI DOUBLE DANDA |
| 0E5A | THAI CHARACTER ANGKHANKHU |
| 0E5B | THAI CHARACTER KHOMUT |
| 104A | MYANMAR SIGN LITTLE SECTION |
| 104B | MYANMAR SIGN SECTION |
| 1735 | PHILIPPINE SINGLE PUNCTUATION |
| 1736 | PHILIPPINE DOUBLE PUNCTUATION |
| 17D4 | KHMER SIGN KHAN |
| 17D5 | KHMER SIGN BARIYOOSAN |
| 1B5E | BALINESE CARIK SIKI |
| 1B5F | BALINESE CARIK PAREREN |
| A8CE | SAURASHTRA DANDA |
| A8CF | SAURASHTRA DOUBLE DANDA |
| AA5D | CHAM PUNCTUATION DANDA |
| AA5E | CHAM PUNCTUATION DOUBLE DANDA |
| AA5F | CHAM PUNCTUATION TRIPLE DANDA |
| 10A56 | KHAROSHTHI PUNCTUATION DANDA |
| 10A57 | KHAROSHTHI PUNCTUATION DOUBLE DANDA |

#### Tibetan

|  |  |
| --- | --- |
| 0F34 | TIBETAN MARK BSDUS RTAGS |
| 0F7F | TIBETAN SIGN RNAM BCAD |
| 0F85 | TIBETAN MARK PALUTA |
| 0FBE | TIBETAN KU RU KHA |
| 0FBF | TIBETAN KU RU KHA BZHI MIG CAN |
| 0FD2 | TIBETAN MARK NYIS TSHEG |

For additional information, see *Section 5.6, [Tibetan Line Breaking](017-5-5-use-of-double-hyphen.md#TibetanLinebreaking)*.

#### Other Terminating Punctuation

Termination punctuation stays with the line, but otherwise allows a break after
it. This is similar to [EX](010-cp-closing-parenthesis.md#EX), except that
the latter may be separated by a space from the preceding word without
allowing a break, whereas these marks are used without spaces.
Terminating punctuation includes:

|  |  |
| --- | --- |
| 1804 | MONGOLIAN COLON |
| 1805 | MONGOLIAN FOUR DOTS |
| 1B5A | BALINESE PANTI |
| 1B5B | BALINESE PAMADA |
| 1B5D | BALINESE CARIK PAMUNGKAH |
| 1B60 | BALINESE PAMENENG |
| 1C3B | LEPCHA PUNCTUATION TA-ROL |
| 1C3C | LEPCHA PUNCTUATION NYET THYOOM TA-ROL |
| 1C3D | LEPCHA PUNCTUATION CER-WA |
| 1C3E | LEPCHA PUNCTUATION TSHOOK CER-WA |
| 1C3F | LEPCHA PUNCTUATION TSHOOK |
| 1C7E | OL CHIKI PUNCTUATION MUCAAD |
| 1C7F | OL CHIKI PUNCTUATION DOUBLE MUCAAD |
| 2CFA | COPTIC OLD NUBIAN DIRECT QUESTION MARK |
| 2CFB | COPTIC OLD NUBIAN INDIRECT QUESTION MARK |
| 2CFC | COPTIC OLD NUBIAN VERSE DIVIDER |
| 2CFF | COPTIC MORPHOLOGICAL DIVIDER |
| 2E0E..2E15 | EDITORIAL CORONIS..UPWARDS ANCORA |
| A60D | VAI COMMA |
| A60F | VAI QUESTION MARK |
| A92E | KAYAH LI SIGN CWI |
| A92F | KAYAH LI SIGN SHYA |
| 10A50 | KHAROSHTHI PUNCTUATION DOT |
| 10A51 | KHAROSHTHI PUNCTUATION SMALL CIRCLE |
| 10A52 | KHAROSHTHI PUNCTUATION CIRCLE |
| 10A53 | KHAROSHTHI PUNCTUATION CRESCENT BAR |
| 10A54 | KHAROSHTHI PUNCTUATION MANGALAM |
| 10A55 | KHAROSHTHI PUNCTUATION LOTUS |
| 11EF7..11EF8 | MAKASAR PASSIMBANG..MAKASAR END OF SECTION |

#### Letters Attached to Orthographic Syllables

In scripts that use the Brahmic style of line breaking, most characters that attach to the initial consonant cluster of an orthographic syllable and are part of that syllable are encoded as combining marks.
These have line break class [CM](009-cb-contingent-break-opportunity.md#CM).
Sometimes, however, additional characters with general category Lo or Lm, such as final consonants or vowel lengtheners, should remain attached to the preceding orthographic syllable.
They are then assigned line break class [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA).

