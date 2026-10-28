# 6 Classifications

Source: [UAX #11: East Asian Width](https://www.unicode.org/reports/tr11/tr11-44.html) · Unicode 17.0.0, revision 44.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md)

<a id="Classifications"></a>

## 6  [Classifications](004-6-classifications.md#Classifications)

The classifications presented here are based on the most widely used mixed-width legacy character sets in use in East Asia as of this writing. In particular, the assignments of the Neutral or Ambiguous categories depend on the contents of these character sets. For example, an implementation that knows *a priori* that it needs to interchange data *only* with the Japanese Shift-JIS character set, but not with other East Asian character sets, could reduce the number of characters in the Ambiguous classification to those actually encoded in Shift-JIS. Alternatively, such a reduction could be done implicitly at runtime in the context of interoperating with Shift-JIS fonts or data sources. Conversely, if additional character sets are created and widely adopted for legacy purposes, more characters would need to be classified as ambiguous.

<a id="Unassigned"></a>

### 6.1  [Unassigned and Private-Use Characters](004-6-classifications.md#Unassigned)

All private-use characters are by default classified as Ambiguous, because their definition depends on context.

Unassigned code points in ranges intended for CJK ideographs are classified as Wide. Those ranges are:

- the CJK Unified Ideographs block, 4E00..9FFF
- the CJK Unified Ideographs Extension A block, 3400..4DBF
- the CJK Compatibility Ideographs block, F900..FAFF
- the Supplementary Ideographic Plane, 20000..2FFFF
- the Tertiary Ideographic Plane, 30000..3FFFF

All other unassigned code points are by default classified as Neutral.

For additional recommendations for handling the default property value for unassigned characters, see *Section 5.3*, *Unknown and Missing Characters,* in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

<a id="Combining"></a>

### 6.2  [Combining Marks](004-6-classifications.md#Combining)

Combining marks have been classified and are given a property assignment based on their typical applicability. For example, combining marks typically applied to characters of class **N**, **Na**, or **W** are classified as **A**. Combining marks for purely non–East Asian scripts are marked as **N**, and nonspacing marks used only with wide characters are given a **W**. Even more so than for other characters, the East\_Asian\_Width property for combining marks is not the same as their display width.

In particular, nonspacing marks do not possess actual advance width. Therefore, even when displaying combining marks, the East\_Asian\_Width property cannot be related to the advance width of these characters. However, it can be useful in determining the encoding length in a legacy encoding, or the choice of font for the range of characters including that nonspacing mark. The width of the glyph image of a nonspacing mark should always be chosen as the appropriate one for the width of the base character.

<a id="DataFile"></a>

### 6.3  [Data File](004-6-classifications.md#DataFile)

The East\_Asian\_Width classification of all Unicode characters is listed in the file EastAsianWidth.txt [[Data11](https://www.unicode.org/reports/tr41/tr41-36.html#Data11)] in the [Unicode Character Database](https://www.unicode.org/ucd/) [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]. This is a tab-delimited, two-column, plain text file, with code position and East\_Asian\_Width designator. A comment at the end of each line indicates the character name. Ideographic, Hangul, Surrogate, and Private Use ranges are collapsed by giving a range in the first column.

<a id="Adding"></a>

### 6.4  [Adding Characters](004-6-classifications.md#Adding)

As more characters are added to the Unicode Standard, or if additional character sets are created and widely adopted for legacy purposes, the assignment of East\_Asian\_Width may be changed for some characters. Implementations should not make any assumptions to the contrary. The sets of Narrow, Fullwidth, and Halfwidth characters are fixed for all practical purposes. New characters for most scripts will be Neutral characters; however, characters for East Asian scripts using wide characters will be classified as Wide. Symbol characters that are, or are expected to be, used both as wide characters in East Asian usage and as narrow characters in non–East Asian usage will be classified Ambiguous.

<a id="References"></a>

## [References](004-6-classifications.md#References)

For references for this annex, see Unicode Standard Annex #41, “[Common References for Unicode Standard Annexes](https://www.unicode.org/reports/tr41/tr41-36.html).”

<a id="Acknowledgments"></a>

## [Acknowledgments](004-6-classifications.md#Acknowledgments)

Asmus Freytag was the author of the initial version of this annex, and served as the editor up through and including the version for Unicode 6.1.0.

Michel Suignard provided extensive input into the analysis and source material for the detail assignments of these properties. Mark Davis and Ken Whistler performed consistency checks on the data files at various times. Tomohiro Kubota reviewed the East\_Asian\_Width assignments against some common legacy encodings.

<a id="Modifications"></a>

## [Modifications](004-6-classifications.md#Modifications)

The following summarizes modifications from the previous published version of this annex.

### Revision 44

- **Reissued** for Unicode 17.0.0.

Previous revisions can be accessed with the “Previous Version” link in the header.

---

© 1998–2025 Unicode, Inc. This publication is protected by copyright, and permission must be obtained from Unicode, Inc. prior to any reproduction, modification, or other use not permitted by the [Terms of Use](https://www.unicode.org/copyright.html). Specifically, you may make copies of this publication and may annotate and translate it solely for personal or internal business purposes and not for public distribution, provided that any such permitted copies and modifications fully reproduce all copyright and other legal notices contained in the original. You may not make copies of or modifications to this publication for public distribution, or incorporate it in whole or in part into any product or publication without the express written permission of Unicode.

Use of all Unicode Products, including this publication, is governed by the Unicode [Terms of Use](https://www.unicode.org/copyright.html). The authors, contributors, and publishers have taken care in the preparation of this publication, but make no express or implied representation or warranty of any kind and assume no responsibility or liability for errors or omissions or for consequential or incidental damages that may arise therefrom. This publication is provided “AS-IS” without charge as a convenience to users.

Unicode and the Unicode Logo are registered trademarks of Unicode, Inc., in the United States and other countries.
