# Unicode® Standard Annex #14

Source: [UAX #14: Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/tr14-55.html) · Unicode 17.0.0, revision 55.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-2-definitions.md)

[Unicode](https://www.unicode.org/) · [Technical Reports](https://www.unicode.org/reports/)

## Unicode® Standard Annex #14

# Unicode Line Breaking Algorithm

|  |  |
| --- | --- |
| Version | Unicode 17.0.0 |
| Editors | Robin Leroy ([eggrobin@unicode.org](mailto:eggrobin@unicode.org)) |
| Date | 2025-09-05 |
| This Version | [https://www.unicode.org/reports/tr14/tr14-55.html](../README.md) |
| Previous Version | [https://www.unicode.org/reports/tr14/tr14-53.html](https://www.unicode.org/reports/tr14/tr14-53.html) |
| Latest Version | [https://www.unicode.org/reports/tr14/](https://www.unicode.org/reports/tr14/) |
| Latest Proposed Update | [https://www.unicode.org/reports/tr14/proposed.html](https://www.unicode.org/reports/tr14/proposed.html) |
| Revision | [55](022-11-history.md#Modifications) |

#### Summary

*This annex presents the Unicode line breaking algorithm along with detailed
descriptions of each of the character classes established by the Unicode line
breaking property. The line breaking algorithm produces a set of "break
opportunities", or positions that would be suitable for wrapping lines
when preparing text for display.*

#### Status



*This document has been reviewed by Unicode members and other
interested parties, and has been approved for publication by the Unicode
Consortium. This is a stable document and may be used as reference
material or cited as a normative reference by other specifications.*

> ***A Unicode Standard Annex (UAX)** forms an integral part of the
> Unicode Standard, but is published online as a separate document. The
> Unicode Standard may require conformance to normative content in a Unicode
> Standard Annex, if so specified in the Conformance chapter of that version
> of the Unicode Standard. The version number of a UAX document corresponds to
> the version of the Unicode Standard of which it forms a part.*

*Please submit corrigenda and other comments with the online reporting
form [[Feedback](https://www.unicode.org/reporting.html)].
Related information that is useful in understanding this annex is found in Unicode Standard Annex #41,
“[Common References for Unicode Standard Annexes](https://www.unicode.org/reports/tr41/tr41-36.html).”
For the latest version of the Unicode Standard, see [[Unicode](https://www.unicode.org/versions/latest/)].
For a list of current Unicode Technical Reports, see [[Reports](https://www.unicode.org/reports/)].
For more information about versions of the Unicode Standard, see [[Versions](https://www.unicode.org/versions/)].
For any errata which may apply to this annex, see [[Errata](https://www.unicode.org/errata/)].*

#### Contents

- 1  [Overview and Scope](001-unicode-standard-annex-14.md#Scope)
- 2  [Definitions](002-2-definitions.md#Definitions)
- 3  [Introduction](004-3-introduction.md#Introduction)
  - 3.1  [Determining Line Break Opportunities](004-3-introduction.md#BreakOpportunities)
- 4  [Conformance](004-3-introduction.md#Conformance)
  - 4.1  [Conformance Requirements](005-4-1-conformance-requirements.md#ConfRequirements)
- 5  [Line Breaking Properties](005-4-1-conformance-requirements.md#Properties)
  - 5.1  [Description of Line Breaking Properties](006-5-1-description-of-line-breaking-properties.md#DescriptionOfProperties)
  - 5.2  [Dictionary Usage](016-zw-zero-width-space-non-tailorable.md#Dictionary)
  - 5.3  [Use of Hyphen](016-zw-zero-width-space-non-tailorable.md#Hyphen)
  - 5.4  [Use of Soft Hyphen](016-zw-zero-width-space-non-tailorable.md#SoftHyphen)
  - 5.5  [Use of Double Hyphen](017-5-5-use-of-double-hyphen.md#DoubleHyphen)
  - 5.6  [Tibetan Line Breaking](017-5-5-use-of-double-hyphen.md#TibetanLinebreaking)
  - 5.7  [Word Separator Characters](017-5-5-use-of-double-hyphen.md#WordSeparators)
- 6  [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)
  - 6.1  [Non-tailorable Line Breaking Rules](018-6-1-non-tailorable-line-breaking-rules.md#BreakingRules)
  - 6.2  [Tailorable Line Breaking Rules](019-6-2-tailorable-line-breaking-rules.md#TailorableBreakingRules)
- 7  [Deleted.](020-7-deleted.md#PairBasedImplementation) (Formerly was: Pair Table-Based Implementation)
- 8  [Customization](020-7-deleted.md#Customization)
  - 8.1  [Types of Tailoring](020-7-deleted.md#Tailoring)
  - 8.2  [Examples of Customization](020-7-deleted.md#Examples)
- 9  [Implementation Notes](021-9-implementation-notes.md#ImplementationNotes)
  - 9.1  [Combining Marks in Regular Expression-Based Implementations](021-9-implementation-notes.md#RegExCombining)
  - 9.2  [Legacy Support for Space Character as Base for Combining Marks](021-9-implementation-notes.md#LegacySpace)
- 10  [Testing](021-9-implementation-notes.md#Testing)
- 11  [History](022-11-history.md#History)
- [References](022-11-history.md#References)
- [Acknowledgments](022-11-history.md#Acknowledgments)
- [Modifications](022-11-history.md#Modifications)

---



<a id="Scope"></a>

## 1  [Overview and Scope](001-unicode-standard-annex-14.md#Scope)

Line breaking, also known as word wrapping, is the process of breaking a section of
text into lines such that it will fit in the available width of a page, window or
other display area. The Unicode Line Breaking Algorithm performs part of this process.
Given an input text, it produces a set of positions called "break opportunities"
that are appropriate points to begin a new line. The selection of actual line
break positions from the set of break opportunities is not covered by the
Unicode Line Breaking Algorithm, but is in the domain of higher level software
with knowledge of the available width and the display size of the text.

The text of the Unicode Standard [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)] presents
a limited description of some of the characters with specific functions in
line breaking, but does not give a complete specification of line breaking behavior. This annex
provides more detailed information about default line breaking behavior, reflecting best
practices for the support of multilingual texts.

For most Unicode characters, considerable variation in line breaking
behavior can be expected, including variation based on local or stylistic
preferences. For that reason, the line breaking properties provided for
these characters are informative. Some characters are intended to explicitly
influence line breaking. Their line breaking behavior is therefore expected
to be identical across all implementations. As described in this annex,
the Unicode Standard assigns normative line breaking properties to those characters.
The Unicode Line Breaking Algorithm is a tailorable set of rules that
uses these line breaking properties in context to determine line break
opportunities.

This annex opens with formal definitions, a summary of the line breaking task
and the context in which it occurs in overall text
layout, followed by a brief section on conformance requirements.
Two main sections follow:

- *Section 5,
  [Line Breaking Properties](005-4-1-conformance-requirements.md#Properties)*, contains a narrative description of the
  line breaking behavior of the characters in the Unicode Standard, grouping them in alphabetical
  order by line breaking class.
- *Section 6,
  [Line Breaking Algorithm](017-5-5-use-of-double-hyphen.md#Algorithm)*,
  provides a set of rules listed in order of precedence that
  constitute a line breaking algorithm.

The next sections discuss issues of customization and implementation.

- *Section 8,
  [Customization](020-7-deleted.md#Customization)*, provides a discussion of how to tailor the algorithm.
- *Section 9,
  [Implementation Notes](021-9-implementation-notes.md#ImplementationNotes)*, provides additional information
  to implementers using regular expression-based techniques or requiring legacy support for
  combining marks.
- *Section 10,
  [Testing](021-9-implementation-notes.md#Testing)*, describes the test data file that is available
  for checking implementations of the line breaking algorithm.
- *Section 11,*
  *[History](022-11-history.md#History)*, provides references to additional
  documentation for investigating changes to the algorithm across Unicode
  versions.

