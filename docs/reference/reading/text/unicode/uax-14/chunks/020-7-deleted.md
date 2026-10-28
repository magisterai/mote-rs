# 7 Deleted

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](019-6-2-tailorable-line-breaking-rules.md) · [Next](021-9-implementation-notes.md)

<a id="PairBasedImplementation"></a>

## 7  [Deleted](020-7-deleted.md#PairBasedImplementation)

Formerly was: Pair Table-Based Implementation.

<a id="Customization"></a>

## 8  [Customization](020-7-deleted.md#Customization)

A real-world line breaking algorithm has to be tailorable to some degree to meet
user or document requirements.

In Korean, for example, two distinct line breaking modes occur,
which can be summarized as breaking after each character or breaking after spaces
(as in Latin text). The former tends to occur when text is set justified; the latter, when
ragged margins are used. In that case, even ideographs are broken only at space
characters. In Japanese, for example, tighter and looser specifications of prohibited line breaks
may be used.

Specialized text or specialized text constructs may need specific line
breaking behavior that differs from the default line breaking rules given in
this annex. This may require additional tailorings beyond those considered
in this section. For example, the rules given here are insufficient for
mathematical equations, whether inline or in display format. Likewise, text
that commonly contains lengthy URLs might benefit from special tailoring that suppresses
[SY](015-sy-symbols-allowing-break-after.md#SY) × [NU](013-ns-nonstarters.md#NU)
from rule [LB25](019-6-2-tailorable-line-breaking-rules.md#LB25) within the scope of a
URL to allow breaks after a “/” separated segment in the URL regardless of
whether the next segment starts with a digit.

> Notes:
>
> - Locale-sensitive line break specifications can be expressed in LDML [[UTS35](https://www.unicode.org/reports/tr41/tr41-36.html#UTS35)].
>   Tailorings are available in the Common Locale Data Repository [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)].
> - Some changes to rules and data are needed for the best segmentation
>   behavior of emoji zwj sequences [[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)].
>   Implementations are strongly encouraged to use the line break rules in
>   the latest version of CLDR (Version 35 or later)
>   [[CLDR](https://www.unicode.org/reports/tr41/tr41-36.html#CLDR)]
>   and the latest emoji properties (Version 12.0 or later)
>   [[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)].

The remainder of this section gives an overview of common types of tailorings.

<a id="Tailoring"></a>

### 8.1  [Types of Tailoring](020-7-deleted.md#Tailoring)

There are two principal ways of tailoring
the line breaking algorithm:

1. **Changing the line breaking class assignment for some characters**  
   This is useful in cases where the line breaking properties of one class of
   characters are occasionally lumped together with the properties of another
   class to achieve a less restrictive line breaking behavior.
2. **Changing the line breaking rules**  
   Adding new rules, or altering or removing existing rules, provides more flexibility in
   changing the line breaking behavior. This can also include introducing new character classes
   for use by the new or altered rules.

For example, specialized
rules could be added to recognize and break common constructs, such as URLs, numeric
expressions, and so on. Such open-ended customizations place no limits on possible changes, other than the
requirement that non-tailorable line breaking rules be
correctly implemented. This means that whatever changes are made must
be equivalent to changes to the line breaking assignments of tailorable line breaking rules, and to alteration,
removal, or addition of rules applied after rule LB12.

<a id="Examples"></a>

### 8.2  [Examples of Customization](020-7-deleted.md#Examples)

***Example 1.***
The exact method of resolving the line break class for
characters with class [SA](014-qu-quotation.md#SA) is not
specified in the default algorithm. One method of implementing line breaks for complex
scripts is to invoke context-based classification for all runs of characters
with class [SA](014-qu-quotation.md#SA). For example, a dictionary-based algorithm could return
different classes for Thai letters depending on their context: letters at the
start of Thai words would become [BB](008-bb-break-before.md#BB) and other Thai letters would become
[AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL). Alternatively, for text consisting of,
or predominantly containing characters with line breaking class [SA](014-qu-quotation.md#SA),
it may be useful to instead defer the determination of line breaks to a
different algorithm.

***Example 2.*** To implement terminal style line breaks, it would be necessary to allow
breaks at fixed positions. These could occur
inside a run of spaces or in the middle of words without regard to
hyphenation. Such a modification essentially disregards the output of
the line breaking algorithm, and is therefore not a conformant tailoring. For
a system that supports both regular line breaking and terminal style line
breaks, only some of its line break modes would be conformant.

***Example 3.***
Depending on the nature of the document, Korean either uses implicit breaking around characters
(type 2 as defined in *Section 3, [Introduction](004-3-introduction.md#Introduction)*) or uses spaces (type 1).
Space-based layout is common in magazines and other informal documents with ragged margins,
while books, with both margins justified, use the other type, as
it affords more line break opportunities and therefore leads to better
justification.

***Example 4.***In a Far Eastern context it is sometimes necessary to allow alphabetic characters and digit strings to break anywhere.
According to reference [[Suign98](https://www.unicode.org/reports/tr41/tr41-36.html#Suign98)],
this can again be done in the same way as Korean.
This can be implemented by adjusting rules [LB23](019-6-2-tailorable-line-breaking-rules.md#LB23),
[LB25](019-6-2-tailorable-line-breaking-rules.md#LB25) and [LB28](019-6-2-tailorable-line-breaking-rules.md#LB28)
to allow breaks between all permutations of the character classes
[AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) and [NU](013-ns-nonstarters.md#NU).

***Example 5.***Some users prefer to relax the requirement that Kana syllables be kept
together. For example, the syllable *kyu,* spelled with the two kanas *KI*
and “small *yu*”, would no longer be kept together as if *KI* and *yu*
were atomic. This customization can be handled by
mapping class [CJ](009-cb-contingent-break-opportunity.md#CJ) to be handled as class [ID](011-h2-hangul-lv-syllable.md#ID)
in rule [LB1](018-6-1-non-tailorable-line-breaking-rules.md#LB1).

***Example 6.*** Tailor to prevent line breaks from falling within default grapheme
clusters, as defined by Unicode Standard Annex #29, “Unicode Text Segmentation”
[[UAX29](https://www.unicode.org/reports/tr41/tr41-36.html#UAX29)]. The tailoring can be accomplished by
first segmenting the text into grapheme clusters according to the rules defined
in UAX #29, and then finding line breaks according to the default line break rules,
as follows: After applying the mandatory line break rules,
give each grapheme cluster the line breaking class of its first code point.

An example of a grapheme cluster that would be split by the default line break
rules is U+0020 SPACE followed by a combining mark.

<a id="Example7"></a>

***Example 7 (deleted).****Versions 4.1.0 through 15.1.0 of The Unicode Standard
defined a tailoring of the line breaking of numeric expressions as Example 7.
This tailoring was used in the test files provided with Unicode 5.1.0 and later.
Since Unicode version 16.0, that behavior has been incorporated into the default;
it no longer constitutes a tailoring.*

***Example 8.*** Some scripts that traditionally follow the Brahmic style of context analysis are nowadays occasionally written with spaces, and word-based line breaking might be desired in that case.
This can be accomplished by remapping the line break classes [AK](006-5-1-description-of-line-breaking-properties.md#AK), [AP](007-al-ordinary-alphabetic-and-symbol-characters.md#AP), and [AS](007-al-ordinary-alphabetic-and-symbol-characters.md#AS) to [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL); and [VI](015-sy-symbols-allowing-break-after.md#VI) or [VF](015-sy-symbols-allowing-break-after.md#VF) to [CM](009-cb-contingent-break-opportunity.md#CM).
In some cases other word-forming characters, such as U+A9CF JAVANESE PANGRANGKEP, also need to be remapped to [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL).
Digits, which may have line break class [AS](007-al-ordinary-alphabetic-and-symbol-characters.md#AS) or [ID](011-h2-hangul-lv-syllable.md#ID) in such scripts, need to be remapped to [NU](013-ns-nonstarters.md#NU).
Punctuation, which may have line break class [ID](011-h2-hangul-lv-syllable.md#ID) in such scripts, need to be remapped to [AL](007-al-ordinary-alphabetic-and-symbol-characters.md#AL) or [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA).

