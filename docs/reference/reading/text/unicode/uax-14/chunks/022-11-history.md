# 11 History

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](021-9-implementation-notes.md)

<a id="RuleNumbering"></a>
<a id="History"></a>

## 11   [History](022-11-history.md#History)

<a id="Table5"></a>

Since its publication in 1999 as part of Unicode
Version 3.0.0, the line breaking algorithm has undergone many changes. It
started as a set of 29 line breaking classes involved in 23 rules which were
representable as a pair table with some special handling for combining marks
and spaces.
It now encompasses 48 line breaking classes involved in more than 40 rules,
many of which rely on extended context which may be several characters removed
from the position they govern.

As the algorithm grew, rules were split, reordered, added, and removed.
In Unicode Version 5.0, the rules were renumbered to reduce the number of
alphabetic suffixes on the rule numbers.

Please refer to Unicode Technical Note #54, “Annotated Line Breaking
Algorithm” [[UTN54](https://www.unicode.org/reports/tr41/tr41-36.html#UTN54)], for a
complete history of the changes to the text of this document since
Unicode Version 3.0.0, and for additonal background on these changes.

Of particular note is the history of the line breaking
assignment of U+034F COMBINING GRAPHEME JOINER.
This character was originally meant to merge adjoining characters into a
graphemic unit, and the character was accordingly originally documented in
Version 3.2 of this annex as having line breaking class
[GL](010-cp-closing-parenthesis.md#GL).
However, this behavior of the combining grapheme joiner was made obsolete in
Unicode Version 4.0, and the character was repurposed for uses where the line
breaking algorithm should ignore it.
From that point on, retaining the line breaking assignment [GL](010-cp-closing-parenthesis.md#GL) was
a mistake, and changing it to [CM](009-cb-contingent-break-opportunity.md#CM) would have
been appropriate. This was only corrected in Unicode Version 17.0, more than
twenty years later. For more on the history of U+034F COMBINING GRAPHEME JOINER,
including a mistake in the data files in the other direction between
Unicode Version 3.2 and Unicode Version 4.1, see Section 6.3 of UTC document
[L2/24-224](https://www.unicode.org/cgi-bin/GetMatchingDocs.pl?L2/24-224).

<a id="References"></a>

## [References](022-11-history.md#References)

For references for this annex, see Unicode Standard Annex #41,
“Common References for Unicode Standard Annexes”
[[UAX41](https://www.unicode.org/reports/tr41/tr41-36.html)].

<a id="Acknowledgments"></a>

## [Acknowledgments](022-11-history.md#Acknowledgments)

Asmus Freytag created the initial version of this
annex and maintained the text for many years. Andy Heninger
maintained the
text from 2008 through 2019. Christopher Chapman maintained the text from 2020 through 2022.
Robin Leroy has maintained the text since September 2022.

The initial assignments of properties are based on input by Michel
Suignard. Mark Davis provided algorithmic verification and formulation of the
rules, and detailed suggestions on the algorithm and text. Ken Whistler, Rick McGowan,
Deborah Anderson, Lorna Evans, and other members of the editorial committee
provided valuable feedback. Tim Partridge enlarged the information on
dictionary usage. Sun Gi Hong reviewed the information on Korean and provided
copious printed samples. Eric Muller reanalyzed the behavior of the soft hyphen
and collected the samples. Adam Twardoch provided the Polish example.
António Martins-Tuválkin supplied information about Portuguese. Tomoyuki
Sadahiro provided information on use of U+30A0. Christopher Fynn provided the background
information on Tibetan line breaking. Andrew West, Kamal Mansour, Andrew Glass,
Daniel Yacob, and Peter Kirk suggested improvements for Mongolian, Arabic,
Kharoshthi, Ethiopic, and Hebrew punctuation characters, respectively.
Kent Karlsson reviewed the line break properties for consistency.
Jerry Hall reviewed the sample code. Elika J. Etemad (fantasai)
reviewed the entire document in an effort to make it easier to reference from
external standards. Norbert Lindenberg added the Brahmic style of line breaking and provided clarifications
on the South East Asian style of line breaking.
Charlotte Buff and David Corbett provided ample feedback on property
assignments and ramifications of the rules. Many
others provided additional review of the rules and property assignments.

<a id="Modifications"></a>

## [Modifications](022-11-history.md#Modifications)

The following summarizes modifications from the previous revision of this
annex.

**Revision 55:**

- **Reissued** for Unicode 17.0.
- Updated the test files to more closely match the rules.
- Split part of class [BA](007-al-ordinary-alphabetic-and-symbol-characters.md#BA) into a new class Unambiguous\_Hyphen
  ([HH](011-h2-hangul-lv-syllable.md#HH)) and updated rules [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a) and
  [LB21a](019-6-2-tailorable-line-breaking-rules.md#LB21a) to use it.
  Rules [LB12a](019-6-2-tailorable-line-breaking-rules.md#LB12a) and [LB21](019-6-2-tailorable-line-breaking-rules.md#LB21) treat HH like
  BA, preserving their old behavior.
  [[181-C53](https://www.unicode.org/cgi-bin/GetL2Ref.pl?181-C53)]
- Updated rule [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a) to treat Hebrew letters (HL) like
  other alphabetic characters (AL).
  [[181-C53](https://www.unicode.org/cgi-bin/GetL2Ref.pl?181-C53)]
- Updated the descriptions of classes [CM](009-cb-contingent-break-opportunity.md#CM) and [GL](010-cp-closing-parenthesis.md#GL)
  to reflect the change to the Line\_Break property of U+034F COMBINING GRAPHEME JOINER.
  Added a discussion of this long-standing mistaken assignment in Section 11, [History](022-11-history.md#History).
  [[181-C54](https://www.unicode.org/cgi-bin/GetL2Ref.pl?181-C53)]
- Section 5.3, *[Use of Hyphen](016-zw-zero-width-space-non-tailorable.md#Hyphen)*: updated for [LB20a](019-6-2-tailorable-line-breaking-rules.md#LB20a) added in Unicode 16.0. [[179-C32](https://www.unicode.org/cgi-bin/GetL2Ref.pl?179-C32)]
- Section 5.5, *[Use of Double Hyphen](017-5-5-use-of-double-hyphen.md#DoubleHyphen)*:
  Add a discussion of the actual DOUBLE HYPHEN, based on [L2/11-038](https://www.unicode.org/L2/L2011/11038-double-hyphen.pdf).
- Section 5.5, *[Use of Double Hyphen](017-5-5-use-of-double-hyphen.md#DoubleHyphen)*:
  Corrected confusing statements about U+2E17 DOUBLE OBLIQUE HYPHEN.

Modifications for previous versions are listed in those respective versions.

---

© 1999–2025 Unicode, Inc. This publication is protected by copyright, and permission must be obtained from Unicode, Inc. prior to any reproduction, modification, or other use not permitted by the [Terms of Use](https://www.unicode.org/copyright.html). Specifically, you may make copies of this publication and may annotate and translate it solely for personal or internal business purposes and not for public distribution, provided that any such permitted copies and modifications fully reproduce all copyright and other legal notices contained in the original. You may not make copies of or modifications to this publication for public distribution, or incorporate it in whole or in part into any product or publication without the express written permission of Unicode.

Use of all Unicode Products, including this publication, is governed by the Unicode [Terms of Use](https://www.unicode.org/copyright.html). The authors, contributors, and publishers have taken care in the preparation of this publication, but make no express or implied representation or warranty of any kind and assume no responsibility or liability for errors or omissions or for consequential or incidental damages that may arise therefrom. This publication is provided “AS-IS” without charge as a convenience to users.

Unicode and the Unicode Logo are registered trademarks of Unicode, Inc., in the United States and other countries.
