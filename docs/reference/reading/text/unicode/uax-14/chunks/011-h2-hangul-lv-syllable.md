# ** H2**: Hangul LV Syllable

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-cp-closing-parenthesis.md) · [Next](012-hl-hebrew-letter.md)

<a id="H2"></a>

### ** [H2](011-h2-hangul-lv-syllable.md#H2)**: Hangul LV Syllable

*[LB26](019-6-2-tailorable-line-breaking-rules.md#LB26), [LB27](019-6-2-tailorable-line-breaking-rules.md#LB27)*

This class includes all characters of Hangul Syllable Type LV.

Together with conjoining jamos, Hangul syllables form Korean Syllable Blocks, which are kept together; see
Unicode Standard Annex #29, “Unicode Text Segmentation” [[UAX29](https://www.unicode.org/reports/tr41/tr41-36.html#UAX29)].
Korean uses space-based line breaking in many styles of documents. To
support these, Hangul syllables and conjoining jamos need to be tailored
to use class  [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL). The default in this specification is
class  [ID](011-h2-hangul-lv-syllable.md#ID), which supports the case of Korean documents not using
space-based line breaking. See *Section 8.1, [Types of Tailoring](020-7-deleted.md#Tailoring)*. See also
[JL](012-hl-hebrew-letter.md#JL),
[JT](012-hl-hebrew-letter.md#JT),
[JV](012-hl-hebrew-letter.md#JV), and
[H3](011-h2-hangul-lv-syllable.md#H3).

<a id="H3"></a>

### ** [H3](011-h2-hangul-lv-syllable.md#H3)**: Hangul LVT Syllable

*[LB26](019-6-2-tailorable-line-breaking-rules.md#LB26), [LB27](019-6-2-tailorable-line-breaking-rules.md#LB27)*

This class includes all characters of Hangul Syllable Type LVT. See also [JL](012-hl-hebrew-letter.md#JL),
[JT](012-hl-hebrew-letter.md#JT),
[JV](012-hl-hebrew-letter.md#JV), and
[H2](011-h2-hangul-lv-syllable.md#H2).

<a id="HH"></a>

### ** [HH](011-h2-hangul-lv-syllable.md#HH)**: Unambiguous Hyphen

*[LB12a](019-6-2-tailorable-line-breaking-rules.md#LB12a), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a), [LB21](019-6-2-tailorable-line-breaking-rules.md#LB21), [LB21a](019-6-2-tailorable-line-breaking-rules.md#LB21a)*

This class consists of breaking hyphens.
These characters
establish explicit break opportunities immediately after
each occurrence, unless they occur word-initally, as
when referring to a suffix such as *-ing*.
The hyphens become non-breaking between Hebrew and non-Hebrew.

|  |  |
| --- | --- |
| 058A | ARMENIAN HYPHEN |
| 05BE | HEBREW PUNCTUATION MAQAF |
| 1400 | CANADIAN SYLLABICS HYPHEN |
| 2010 | HYPHEN |
| 2012 | FIGURE DASH |
| 2013 | EN DASH |
| 2E17 | DOUBLE OBLIQUE HYPHEN |
| 2E40 | DOUBLE HYPHEN |
| 2E5D | OBLIQUE HYPHEN |
| 10D6E | GARAY HYPHEN |
| 10EAD | YEZIDI HYPHENATION MARK |

Hyphens are graphic characters with width. Because, unlike spaces, they
are visible, they are included in the measured part of the preceding line, except
where the layout style allows hyphens to hang into the margins.
For additional
information about how to format line breaks resulting from the presence of hyphens, see
*Section 5.3, [Use of Hyphen](016-zw-zero-width-space-non-tailorable.md#Hyphen)*.

<a id="HY"></a>

### ** [HY](011-h2-hangul-lv-syllable.md#HY)**: Hyphen

*[LB12a](019-6-2-tailorable-line-breaking-rules.md#LB12a), [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a), [LB21](019-6-2-tailorable-line-breaking-rules.md#LB21), [LB21a](019-6-2-tailorable-line-breaking-rules.md#LB21a), [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25)*

|  |  |
| --- | --- |
| 002D | HYPHEN-MINUS |

Some additional context analysis is required to distinguish usage of this
character as a hyphen from its usage as a minus sign (or indicator of numerical
range). If used as hyphen, it acts like U+2010 HYPHEN,
which has line break class [HH](011-h2-hangul-lv-syllable.md#HH).

> Note: Some typescript conventions use runs of
> HYPHEN-MINUS to stand in
> for longer dashes or horizontal rules. If actual character code conversion
> is not performed and it is desired to treat them like the characters or
> layout elements they stand for, line breaking needs to support these
> runs explicitly.

<a id="ID"></a>

### ** [ID](011-h2-hangul-lv-syllable.md#ID)**: Ideographic

*[LB23a](019-6-2-tailorable-line-breaking-rules.md#LB23a)*

Characters with this property do not require other characters to provide break opportunities;
lines can ordinarily break before and after and between pairs of ideographic characters.
Examples of characters with the [ID](011-h2-hangul-lv-syllable.md#ID)
line break class include most assigned characters in the ranges listed below.
Note that this class also includes characters other than Han ideographs.

|  |  |
| --- | --- |
| 2E80..2FFF | CJK, Kangxi Radicals, Ideographic Description Symbols |
| 3040..309F | Hiragana (except small characters) |
| 30A2..30FA | Katakana (except small characters) |
| 3400..4DBF | CJK Unified Ideographs Extension A |
| 4E00..9FFF | CJK Unified Ideographs |
| F900..FAFF | CJK Compatibility Ideographs |

See the data file LineBreak.txt [[Data14](https://www.unicode.org/reports/tr41/tr41-36.html#Data14)]
or the data file
DerivedLineBreak.txt [[Data14Derived](https://www.unicode.org/reports/tr41/tr41-36.html#Data14Derived)]
for the complete list of characters with the [ID](011-h2-hangul-lv-syllable.md#ID) line break class.

> Note: Use U+2060 WORD JOINER
> as a manual override to prevent
> break opportunities around characters of class [ID](011-h2-hangul-lv-syllable.md#ID).

Unassigned code points in blocks or regions of the Unicode codespace
that have been reserved for CJK scripts are also assigned this line break class.
These assignments anticipate that future characters assigned in these ranges will have
the class [ID](011-h2-hangul-lv-syllable.md#ID). Once a character is assigned to one
of these code points, the property value could change.

For example,
all of the undesignated code points in Planes 2 (20000..2FFFD) and 3 (30000..3FFFD)
default to [ID](011-h2-hangul-lv-syllable.md#ID).
See the data file DerivedLineBreak.txt for
the complete list of code point ranges which default to
the [ID](011-h2-hangul-lv-syllable.md#ID) line break class.

#### Korean

Korean is encoded with conjoining jamos, Hangul syllables, or both. See also [JL](012-hl-hebrew-letter.md#JL),
[JT](012-hl-hebrew-letter.md#JT),
[JV](012-hl-hebrew-letter.md#JV),
[H2](011-h2-hangul-lv-syllable.md#H2), and [H3](011-h2-hangul-lv-syllable.md#H3).
The following set of compatibility jamo is treated as [ID](011-h2-hangul-lv-syllable.md#ID)
by default.

|  |  |
| --- | --- |
| 3130..318F | HANGUL COMPATIBILITY JAMO |

#### Symbols

Certain pictographic symbols of General Category So
are also included in this line break class.

