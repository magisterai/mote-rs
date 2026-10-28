# Unicode® Standard Annex #11

Source: [UAX #11: East Asian Width](https://www.unicode.org/reports/tr11/tr11-44.html) · Unicode 17.0.0, revision 44.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-2-scope.md)

[Unicode](https://www.unicode.org/) · [Technical Reports](https://www.unicode.org/reports/)

## Unicode® Standard Annex #11

# East Asian Width

|  |  |
| --- | --- |
| Version | Unicode 17.0.0 |
| Editor | Ken Lunde 小林剣 |
| Date | 2025-07-24 |
| This Version | [https://www.unicode.org/reports/tr11/tr11-44.html](../README.md) |
| Previous Version | [https://www.unicode.org/reports/tr11/tr11-43.html](https://www.unicode.org/reports/tr11/tr11-43.html) |
| Latest Version | [https://www.unicode.org/reports/tr11/](https://www.unicode.org/reports/tr11/) |
| Latest Proposed Update | [https://www.unicode.org/reports/tr11/proposed.html](https://www.unicode.org/reports/tr11/proposed.html) |
| Revision | [44](004-6-classifications.md#Modifications) |

#### Summary

*This annex introduces the concept of inherent width, and presents the specification of a normative property for Unicode characters that is useful when processing text that needs to account for inherent width. This property is of particular use when interoperating with East Asian legacy character sets, but is applicable beyond that. The annex also discusses the interaction between inherent width and variation sequences.*

#### Status



*This document has been reviewed by Unicode members and other interested parties, and has been approved for publication by the Unicode Consortium. This is a stable document and may be used as reference material or cited as a normative reference by other specifications.*

> ***A Unicode Standard Annex (UAX)** forms an integral part of the Unicode Standard, but is published online as a separate document. The Unicode Standard may require conformance to normative content in a Unicode Standard Annex, if so specified in the Conformance chapter of that version of the Unicode Standard. The version number of a UAX document corresponds to the version of the Unicode Standard of which it forms a part.*

*Please submit corrigenda and other comments with the online reporting form [[Feedback](https://www.unicode.org/reporting.html)]. Related information that is useful in understanding this annex is found in Unicode Standard Annex #41, “[Common References for Unicode Standard Annexes](https://www.unicode.org/reports/tr41/tr41-36.html).” For the latest version of the Unicode Standard, see [[Unicode](https://www.unicode.org/versions/latest/)]. For a list of current Unicode Technical Reports, see [[Reports](https://www.unicode.org/reports/)]. For more information about versions of the Unicode Standard, see [[Versions](https://www.unicode.org/versions/)]. For any errata which may apply to this annex, see [[Errata](https://www.unicode.org/errata/)].*

#### Contents

- 1  [Overview](001-unicode-standard-annex-11.md#Overview)
- 2  [Scope](002-2-scope.md#Scope)
- 3  [Description](002-2-scope.md#Description)
- 4  [Definitions](002-2-scope.md#Definitions)
  - 4.1  [Relation to the Terms “Fullwidth” and “Halfwidth”](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#Relation)
  - 4.2  [Ambiguous Characters](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#Ambiguous)
  - 4.3  [Variation Sequences](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#VariationSequences)
- 5  [Recommendations](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#Recommendations)
- 6  [Classifications](004-6-classifications.md#Classifications)
  - 6.1  [Unassigned and Private-Use Characters](004-6-classifications.md#Unassigned)
  - 6.2  [Combining Marks](004-6-classifications.md#Combining)
  - 6.3  [Data File](004-6-classifications.md#DataFile)
  - 6.4  [Adding Characters](004-6-classifications.md#Adding)
- [References](004-6-classifications.md#References)
- [Acknowledgments](004-6-classifications.md#Acknowledgments)
- [Modifications](004-6-classifications.md#Modifications)

---

<a id="Overview"></a>

## 1  [Overview](001-unicode-standard-annex-11.md#Overview)

When processing text, there is the concept of an *inherent* width of a character, which is of particular importance in an East Asian context. This width takes on either of two values: *narrow* or *wide.* For traditional mixed-width East Asian legacy character sets, this classification into narrow and wide corresponds with few exceptions directly to the storage size for each character: a few narrow characters use a single byte per character and all other characters (usually wide) use two or more bytes.

Layout and line breaking (to cite only two examples) in East Asian context show systematic variations depending on the value of this East\_Asian\_Width property. *Wide* characters behave like ideographs; they tend to allow line breaks after each character and remain upright in vertical text layout. *Narrow* characters are kept together in words or runs that are rotated sideways in vertical text layout.

For a traditional East Asian *fixed pitch* font, this width translates to a display width of either one half or a whole unit width. A common name for this unit width is “Em.” While an Em is customarily the *height* of the letter “M,” it is the same as the unit *width* in East Asian fonts, because in these fonts the standard character cell is square. In contrast, the character width for a fixed-pitch Latin font like Courier is generally 3/5 of an Em.

In modern practice, most alphabetic characters are rendered by variable-width fonts using narrow characters, even if their encoding in common legacy sets uses multiple bytes. In contrast, emoji characters were first developed through the use of extensions of legacy East Asian encodings, such as Shift-JIS, and in such a context they were treated as wide characters. While these extensions have been added to Unicode or mapped to standardized variation sequences, their treatment as wide characters has been retained, and extended for consistency with emoji characters that lack a legacy encoding.

Except for a few characters, which are explicitly called out as *fullwidth* or *halfwidth* in the Unicode Standard, characters are not duplicated based on distinction in width. Some characters, such as the ideographs, are always wide; others are always narrow; and some can be narrow or wide, depending on the context. The Unicode character property *East\_Asian\_Width* provides a default classification of characters, which an implementation can use to decide at runtime whether to treat a character as narrow or wide.

The East\_Asian\_Width property does not preserve canonical equivalence, because the base characters of canonical decompositions almost always have a different East\_Asian\_Width property value than the precomposed characters. Decomposing a character, and applying the East\_Asian\_Width property to a base character and combining marks separately does not yield the expected values. For examples, see Section 4, [*Definitions*](002-2-scope.md#Definitions).

