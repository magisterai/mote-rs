# ** BB**: Break Before

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-al-ordinary-alphabetic-and-symbol-characters.md) · [Next](009-cb-contingent-break-opportunity.md)

|  |  |
| --- | --- |
| A9CF | JAVANESE PANGRANGKEP |
| AA40..AA42 | CHAM LETTER FINAL K..CHAM LETTER FINAL NG |
| AA44..AA4B | CHAM LETTER FINAL CH..CHAM LETTER FINAL SS |
| 1133D | GRANTHA SIGN AVAGRAHA |
| 1135D | GRANTHA SIGN PLUTA |
| 11EF2 | MAKASAR ANGKA |

<a id="BB"></a>

### ** [BB](008-bb-break-before.md#BB)**: Break Before

*[LB21](019-6-2-tailorable-line-breaking-rules.md#LB21)*

Characters of this line break class move to the next line at a line break
and thus provide a line break opportunity before.

Examples of [BB](008-bb-break-before.md#BB) characters are described
in the following sections.

#### Dictionary Use

|  |  |
| --- | --- |
| 00B4 | ACUTE ACCENT |
| 1FFD | GREEK OXIA |

In some dictionaries, stressed syllables are indicated with a spacing acute
accent instead of the hyphenation point. In this case the accent moves to
the next line, and the preceding line ends with a hyphen.
The oxia is canonically equivalent to the acute accent.

|  |  |
| --- | --- |
| 02DF | MODIFIER LETTER CROSS ACCENT |

A cross accent also appears in some dictionaries to mark the stress of the following
syllable, and should be handled in the same way as the other stress marking
characters in this section. The accent should not be separated from the
syllable it marks by a break.

|  |  |
| --- | --- |
| 02C8 | MODIFIER LETTER VERTICAL LINE |
| 02CC | MODIFIER LETTER LOW VERTICAL LINE |

These characters are used in dictionaries to indicate stress and secondary
stress when IPA is used. Both are prefixes to the stressed syllable in IPA.
Breaking before them keeps them with the
syllable.

> Note: It is hard to find actual examples in most dictionaries
> because the pronunciation fields usually occur right after the headword, and
> the columns are wide enough to prevent line breaks in most pronunciations.

#### Tibetan and Phags-Pa Head Letters

|  |  |
| --- | --- |
| 0F01 | TIBETAN MARK GTER YIG MGO TRUNCATED A |
| 0F02 | TIBETAN MARK GTER YIG MGO -UM RNAM BCAD MA |
| 0F03 | TIBETAN MARK GTER YIG MGO -UM GTER TSHEG MA |
| 0F04 | TIBETAN MARK INITIAL YIG MGO MDUN MA |
| 0F06 | TIBETAN MARK CARET YIG MGO PHUR SHAD MA |
| 0F07 | TIBETAN MARK YIG MGO TSHEG SHAD MA |
| 0F09 | TIBETAN MARK BSKUR YIG MGO |
| 0F0A | TIBETAN MARK BKA- SHOG YIG MGO |
| 0FD0 | TIBETAN MARK BSKA- SHOG GI MGO RGYAN |
| 0FD1 | TIBETAN MARK MNYAM YIG GI MGO RGYAN |
| 0FD3 | TIBETAN MARK INITIAL BRDA RNYING YIG MGO MDUN MA |
| A874 | PHAGS-PA SINGLE HEAD MARK |
| A875 | PHAGS-PA DOUBLE HEAD MARK |

Tibetan head letters allow a break before. For more information,
see *Section 5.6, [Tibetan Line Breaking](017-5-5-use-of-double-hyphen.md#TibetanLinebreaking)*.

#### Mongolian

|  |  |
| --- | --- |
| 1806 | MONGOLIAN TODO SOFT HYPHEN |

Despite its name, this Mongolian character is not an invisible control like
SOFT HYPHEN,
but rather a visible character like a regular hyphen. Unlike the hyphen, MONGOLIAN TODO SOFT HYPHEN stays with the following line. Whenever optional line breaks are to be marked invisibly,
SOFT HYPHEN should be used instead.

<a id="B2"></a>

### ** [B2](008-bb-break-before.md#B2)**: Break Opportunity Before and After

*[LB17](019-6-2-tailorable-line-breaking-rules.md#LB17)*

|  |  |
| --- | --- |
| 2014 | EM DASH |

The EM DASH is used to set off
parenthetical text. Normally, it is used without spaces. However, this is language dependent.
For example, in Swedish, spaces are used around
the EM DASH. Line breaks can occur
before and after an EM DASH. Because EM DASHes
are sometimes used in pairs instead of a single quotation dash, the default
behavior is not to break the line between even though not all
fonts use connecting glyphs for the
EM DASH.

Some languages, including Spanish, use EM DASH to set off
a parenthetical, and the surrounding dashes should not be broken from the contained text.
In this usage there is space on the side where it can be broken. This does not conflict with
symmetrical usages, either with spaces on both sides of the em-dash or with no spaces.

<a id="BK"></a>

### ** [BK](008-bb-break-before.md#BK)**: Mandatory Break (Non-tailorable)

*[LB4](018-6-1-non-tailorable-line-breaking-rules.md#LB4), [LB6](018-6-1-non-tailorable-line-breaking-rules.md#LB6), [LB9](018-6-1-non-tailorable-line-breaking-rules.md#LB9), [LB15a](019-6-2-tailorable-line-breaking-rules.md#LB15a), [LB15b](019-6-2-tailorable-line-breaking-rules.md#LB15b), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a)*

Explicit breaks act independently of the surrounding characters.
No characters can be added to the [BK](008-bb-break-before.md#BK) class as
part of tailoring, but implementations are not required to support the VT
character.

|  |  |
| --- | --- |
| 000B | LINE TABULATION (VT) |
| 000C | FORM FEED (FF) |

FORM FEED separates pages. The text on the new page starts at the beginning
of the line. In some layout modes there may be no
visible advance to a new “page”.

|  |  |
| --- | --- |
| 2028 | LINE SEPARATOR |

The text after the LINE SEPARATOR starts at the beginning of the line.
This is similar to HTML <BR>.

|  |  |
| --- | --- |
| 2029 | PARAGRAPH SEPARATOR |

The text of the new paragraph starts at the beginning of the line.
This character defines a paragraph break, causing suitable formatting to be
applied, for example, interparagraph spacing or first line indentation.
LINE SEPARATOR, FF, VT as well as [CR](010-cp-closing-parenthesis.md#CR),
[LF](012-hl-hebrew-letter.md#LF) and [NL](012-hl-hebrew-letter.md#NL) do not define a paragraph break.

#### Newline Function (NLF)

Newline Functions are defined in the Unicode
Standard as providing additional mandatory breaks. They are not
individual characters, but are encoded as sequences of the control characters
NEL, LF, and CR. If a character sequence for a
Newline Function contains more than one character, it is kept together.
The particular sequences that form an NLF
depend on the implementation and other circumstances as described in *Section 5.8,
Newline Guidelines*, of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

This specification defines the NLF implicitly. It
defines the three character
classes [CR](010-cp-closing-parenthesis.md#CR),
[LF](012-hl-hebrew-letter.md#LF), and [NL](012-hl-hebrew-letter.md#NL). Their line
break behavior, defined in rule [LB5](018-6-1-non-tailorable-line-breaking-rules.md#LB5) in *Section 6.1,
[Non-tailorable Line Breaking Rules](018-6-1-non-tailorable-line-breaking-rules.md#BreakingRules)*, is to
break after [NL](012-hl-hebrew-letter.md#NL),
[LF](012-hl-hebrew-letter.md#LF),
or [CR](010-cp-closing-parenthesis.md#CR), but not between
[CR](010-cp-closing-parenthesis.md#CR) and  [LF](012-hl-hebrew-letter.md#LF).

