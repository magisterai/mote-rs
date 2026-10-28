# 5.1 Description of Line Breaking Properties

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-4-1-conformance-requirements.md) · [Next](007-al-ordinary-alphabetic-and-symbol-characters.md)

<a id="DescriptionOfProperties"></a>

### 5.1  [Description of Line Breaking Properties](006-5-1-description-of-line-breaking-properties.md#DescriptionOfProperties)

Line breaking classes are listed alphabetically.
For each line breaking class, the rules that explicitly reference that class
are listed in italics above the description of the class.
Note that characters in these classes may be involved in other rules;
for instance, rule [LB31](019-6-2-tailorable-line-breaking-rules.md#LB31) can apply to characters with
almost any line breaking class,
but it does not list any line breaking class explicitly.

<a id="AI"></a>

### ** [AI](006-5-1-description-of-line-breaking-properties.md#AI)**: Ambiguous (Alphabetic or Ideograph)

*[LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1)*

Some characters that ordinarily act like
alphabetic characters are treated like ideographs (line breaking class
 [ID](011-h2-hangul-lv-syllable.md#ID)) in certain East Asian legacy contexts.
Their line breaking behavior
therefore depends on the context. In the absence of appropriate context information,
they are treated as class [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL); see
the note at the end of this description.

As originally defined until Unicode Version
3.1.0, the line break class [AI](006-5-1-description-of-line-breaking-properties.md#AI) contained *all*
characters with East\_Asian\_Width value A (ambiguous width) that
would otherwise be [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) in this
classification. For more information on East\_Asian\_Width and how to
resolve it, see Unicode Standard Annex #11,
*East Asian Width* [[UAX11](https://www.unicode.org/reports/tr41/tr41-36.html#UAX11)].

The original definition included many Latin, Greek, and Cyrillic
characters. Since Unicode Version 4.0.1, these characters are classified by default as [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL)
because use of the [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL)
line breaking class better corresponds to modern practice. Where strict
compatibility with older legacy implementations is desired, some of these
characters need to be
treated as [ID](011-h2-hangul-lv-syllable.md#ID) in certain contexts. This can be done by always tailoring them
to [ID](011-h2-hangul-lv-syllable.md#ID) or by continuing to classify
them as [AI](006-5-1-description-of-line-breaking-properties.md#AI) and resolving them to [ID](011-h2-hangul-lv-syllable.md#ID)
where required.

As part of the same
revision, the set of ambiguous characters has been extended to completely encompass
the enclosed alphanumeric characters used for numbering of bullets.

In Unicode Version 4.0.1, the [AI](006-5-1-description-of-line-breaking-properties.md#AI) line breaking
class therefore included all characters with East Asian Width A that are outside the range U+0000..U+1FFF, plus the following
characters:

|  |  |
| --- | --- |
| 24EA | CIRCLED DIGIT ZERO |
| 2780..2793 | DINGBAT CIRCLED SANS-SERIF DIGIT ONE..DINGBAT NEGATIVE CIRCLED SANS-SERIF NUMBER TEN |

Since that time,
the East\_Asian\_Width and Line\_Break properties have been maintained
independently, with the latter being based on the need for language-specific
line-breaking behavior rather than compatibility with legacy encodings.
In particular, all vulgar fractions have Line\_Break=AI.

Characters with the line break class [AI](006-5-1-description-of-line-breaking-properties.md#AI)
with East\_Asian\_Width value A typically take the
[AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) line breaking class
when their resolved East\_Asian\_Width is N (narrow) and take the
line breaking class [ID](011-h2-hangul-lv-syllable.md#ID) when their
resolved width is W (wide). The remaining
characters are then resolved to [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL)
or [ID](011-h2-hangul-lv-syllable.md#ID) in a consistent fashion.
The details of this resolution are not specified in this annex. The line breaking rules in
*Section 6, [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)*
merely require that all ambiguous characters be resolved appropriately as part of
assigning line breaking classes to the input characters.

> Note: The canonical decompositions of characters of class
> [AI](006-5-1-description-of-line-breaking-properties.md#AI) are not necessarily of class
> [AI](006-5-1-description-of-line-breaking-properties.md#AI) themselves.
> The East\_Asian\_Width property A on which the definition of
> [AI](006-5-1-description-of-line-breaking-properties.md#AI) is largely based, does not preserve canonical equivalence.
> In the context of line breaking, the fact that a character has been assigned
> class [AI](006-5-1-description-of-line-breaking-properties.md#AI) means that the line break implementation must resolve it to either
> [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) or
> [ID](011-h2-hangul-lv-syllable.md#ID), in the
> absence of further tailoring. If
> preserving canonical equivalence is desired, an implementation is free to
> make sure that the *resolved* line break classes preserve canonical
> equivalence. Unless compatibility with particular legacy behavior is
> important, it may be sufficient to
> map all such characters to [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL). This
> achieves a canonically equivalent resolution of line breaking classes, and
> is compatible with emerging modern practice that treats these characters
> increasingly like regular alphabetic characters.

<a id="AK"></a>

### ** [AK](006-5-1-description-of-line-breaking-properties.md#AK)**: Aksara

*[LB28a](019-6-2-tailorable-line-breaking-rules.md#LB28a)*

The [AK](006-5-1-description-of-line-breaking-properties.md#AK) line break class is used for scripts that use the Brahmic style of context analysis and have a virama of Indic syllabic category Virama or Invisible\_Stacker.
It contains characters that can occur as the bases of orthographic syllables and can also follow a virama of Indic syllabic category Virama or Invisible\_Stacker within the same orthographic syllable.
Depending on the script, this may include characters with the Indic syllabic categories Consonant, Vowel\_Independent, or Number.

|  |  |
| --- | --- |
| 1B05..1B33 | BALINESE LETTER AKARA..BALINESE LETTER HA |
| 1B45..1B4C | BALINESE LETTER KAF SASAK..BALINESE LETTER ARCHAIC JNYA |
| A984..A9B2 | JAVANESE LETTER A..JAVANESE LETTER HA |
| 11005..11037 | BRAHMI LETTER A..BRAHMI LETTER OLD TAMIL NNNA |
| 11071..11072 | BRAHMI LETTER OLD TAMIL SHORT E..BRAHMI LETTER OLD TAMIL SHORT O |
| 11075 | BRAHMI LETTER OLD TAMIL LLA |
| 11305..1130C | GRANTHA LETTER A..GRANTHA LETTER VOCALIC L |
| 1130F..11310 | GRANTHA LETTER EE..GRANTHA LETTER AI |
| 11313..11328 | GRANTHA LETTER OO..GRANTHA LETTER NA |
| 1132A..11330 | GRANTHA LETTER PA..GRANTHA LETTER RA |
| 11332..11333 | GRANTHA LETTER LA..GRANTHA LETTER LLA |
| 11335..11339 | GRANTHA LETTER VA..GRANTHA LETTER HA |
| 11360..11361 | GRANTHA LETTER VOCALIC RR..GRANTHA LETTER VOCALIC LL |
| 11F04..11F10 | KAWI LETTER A..KAWI LETTER O |
| 11F12..11F33 | KAWI LETTER KA..KAWI LETTER JNYA |

