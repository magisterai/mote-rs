# Unicode® Standard Annex #9

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-1-introduction.md)

[Unicode](https://www.unicode.org/) · [Technical Reports](https://www.unicode.org/reports/)

## Unicode® Standard Annex #9

# Unicode Bidirectional Algorithm

|  |  |
| --- | --- |
| Version | Unicode 17.0.0 |
| Editors | Manish Goregaokar मनीष गोरेगांवकर ([manish@unicode.org](mailto:manish@unicode.org)), Robin Leroy ([eggrobin@unicode.org](mailto:eggrobin@unicode.org)) |
| Date | 2025-08-13 |
| This Version | [https://www.unicode.org/reports/tr9/tr9-51.html](../README.md) |
| Previous Version | [https://www.unicode.org/reports/tr9/tr9-50.html](https://www.unicode.org/reports/tr9/tr9-50.html) |
| Latest Version | [https://www.unicode.org/reports/tr9/](https://www.unicode.org/reports/tr9/) |
| Latest Proposed Update | [https://www.unicode.org/reports/tr9/proposed.html](https://www.unicode.org/reports/tr9/proposed.html) |
| Revision | [51](019-section-reorganization.md#Modifications) |

#### Summary

*This annex describes specifications for the positioning of characters in text containing characters flowing from right
to left, such as Arabic or Hebrew.*

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

#### *Contents*

- 1  [Introduction](002-1-introduction.md#Introduction)
- 2  [Directional Formatting Characters](002-1-introduction.md#Directional_Formatting_Characters)
  - 2.1  [Explicit Directional Embeddings](002-1-introduction.md#Explicit_Directional_Embeddings)
  - 2.2  [Explicit Directional Overrides](003-2-2-explicit-directional-overrides.md#Explicit_Directional_Overrides)
  - 2.3  [Terminating Explicit Directional
    Embeddings and Overrides](003-2-2-explicit-directional-overrides.md#Terminating_Explicit_Directional_Embeddings_and_Overrides)
  - 2.4  [Explicit Directional Isolates](003-2-2-explicit-directional-overrides.md#Explicit_Directional_Isolates)
  - 2.5  [Terminating Explicit Directional Isolates](003-2-2-explicit-directional-overrides.md#Terminating_Explicit_Directional_Isolates)
  - 2.6  [Implicit Directional Marks](003-2-2-explicit-directional-overrides.md#Implicit_Directional_Marks)
  - 2.7  [Markup and Formatting Characters](003-2-2-explicit-directional-overrides.md#Markup_And_Formatting)
- 3  [Basic Display Algorithm](004-3-basic-display-algorithm.md#Basic_Display_Algorithm)
  - 3.1  [Definitions](004-3-basic-display-algorithm.md#Definitions)
    - 3.1.1  [Basics](004-3-basic-display-algorithm.md#Basics):
      [BD1](004-3-basic-display-algorithm.md#BD1), [BD2](004-3-basic-display-algorithm.md#BD2), [BD3](004-3-basic-display-algorithm.md#BD3), [BD4](004-3-basic-display-algorithm.md#BD4),
      [BD5](004-3-basic-display-algorithm.md#BD5), [BD6](004-3-basic-display-algorithm.md#BD6), [BD7](004-3-basic-display-algorithm.md#BD7)
    - 3.1.2  [Matching Explicit Directional Formatting Characters](005-3-1-2-matching-explicit-directional-formatting-characters.md#Matching_Explicit_Directional_Formatting_Characters):
      [BD8](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD8), [BD9](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD9),
      [BD10](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD10), [BD11](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD11),
      [BD12](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD12), [BD13](005-3-1-2-matching-explicit-directional-formatting-characters.md#BD13)
    - 3.1.3  [Paired Brackets](006-3-1-3-paired-brackets.md#Paired_Brackets):
      [BD14](006-3-1-3-paired-brackets.md#BD14), [BD15](006-3-1-3-paired-brackets.md#BD15), [BD16](006-3-1-3-paired-brackets.md#BD16)
    - 3.1.4  [Additional Abbreviations](006-3-1-3-paired-brackets.md#Additional_Abbreviations)
  - 3.2  [Bidirectional Character Types](006-3-1-3-paired-brackets.md#Bidirectional_Character_Types)
  - 3.3  [Resolving Embedding Levels](007-3-3-resolving-embedding-levels.md#Resolving_Embedding_Levels)
    - 3.3.1  [The Paragraph Level](008-3-3-1-the-paragraph-level.md#The_Paragraph_Level):
      [P1](008-3-3-1-the-paragraph-level.md#P1), [P2](008-3-3-1-the-paragraph-level.md#P2), [P3](008-3-3-1-the-paragraph-level.md#P3)
    - 3.3.2  [Explicit Levels and Directions](008-3-3-1-the-paragraph-level.md#Explicit_Levels_and_Directions):
      [X1](008-3-3-1-the-paragraph-level.md#X1), [X2](008-3-3-1-the-paragraph-level.md#X2), [X3](008-3-3-1-the-paragraph-level.md#X3), [X4](008-3-3-1-the-paragraph-level.md#X4),
      [X5](008-3-3-1-the-paragraph-level.md#X5), [X5a](009-non-formatting-characters.md#X5a), [X5b](009-non-formatting-characters.md#X5b), [X5c](009-non-formatting-characters.md#X5c),
      [X6](009-non-formatting-characters.md#X6), [X6a](009-non-formatting-characters.md#X6a),
      [X7](009-non-formatting-characters.md#X7), [X8](009-non-formatting-characters.md#X8)
    - 3.3.3  [Preparations for Implicit Processing](010-3-3-3-preparations-for-implicit-processing.md#Preparations_for_Implicit_Processing):
      [X9](010-3-3-3-preparations-for-implicit-processing.md#X9), [X10](010-3-3-3-preparations-for-implicit-processing.md#X10)
    - 3.3.4  [Resolving Weak Types](011-3-3-4-resolving-weak-types.md#Resolving_Weak_Types):
      [W1](011-3-3-4-resolving-weak-types.md#W1), [W2](011-3-3-4-resolving-weak-types.md#W2), [W3](011-3-3-4-resolving-weak-types.md#W3), [W4](011-3-3-4-resolving-weak-types.md#W4),
      [W5](011-3-3-4-resolving-weak-types.md#W5), [W6](011-3-3-4-resolving-weak-types.md#W6), [W7](011-3-3-4-resolving-weak-types.md#W7)
    - 3.3.5  [Resolving Neutral and Isolate Formatting Types](011-3-3-4-resolving-weak-types.md#Resolving_Neutral_Types):
      [N0](011-3-3-4-resolving-weak-types.md#N0), [N1](011-3-3-4-resolving-weak-types.md#N1), [N2](011-3-3-4-resolving-weak-types.md#N2)
    - 3.3.6  [Resolving Implicit Levels](012-3-3-6-resolving-implicit-levels.md#Resolving_Implicit_Levels):
      [I1](012-3-3-6-resolving-implicit-levels.md#I1), [I2](012-3-3-6-resolving-implicit-levels.md#I2)
  - 3.4  [Reordering Resolved Levels](012-3-3-6-resolving-implicit-levels.md#Reordering_Resolved_Levels):
    [L1](012-3-3-6-resolving-implicit-levels.md#L1), [L2](012-3-3-6-resolving-implicit-levels.md#L2), [L3](012-3-3-6-resolving-implicit-levels.md#L3), [L4](012-3-3-6-resolving-implicit-levels.md#L4)
  - 3.5  [Shaping](013-3-5-shaping.md#Shaping)
- 4  [Bidirectional Conformance](013-3-5-shaping.md#Bidirectional_Conformance)
  - 4.1  [Boundary Neutrals](014-4-1-boundary-neutrals.md#Boundary_Neutrals)
  - 4.2  [Explicit Formatting Characters](014-4-1-boundary-neutrals.md#Explicit_Formatting_Characters)
  - 4.3  [Higher-Level Protocols](014-4-1-boundary-neutrals.md#Higher-Level_Protocols): [HL1](014-4-1-boundary-neutrals.md#HL1),
    [HL2](014-4-1-boundary-neutrals.md#HL2), [HL3](014-4-1-boundary-neutrals.md#HL3), [HL4](014-4-1-boundary-neutrals.md#HL4), [HL5](014-4-1-boundary-neutrals.md#HL5),
    [HL6](014-4-1-boundary-neutrals.md#HL6)
    - 4.3.1 [HL4 Example 1 for XML](014-4-1-boundary-neutrals.md#HL4Example1)
    - 4.3.2 [HL4 Example 2 for Program Text](015-4-3-2-hl4-example-2-for-program-text.md#HL4Example2)
    - 4.3.3 [HL4 Example 3 for URLs](015-4-3-2-hl4-example-2-for-program-text.md#HL4Example3)
  - 4.4 [Bidirectional Conformance Testing](016-4-4-bidirectional-conformance-testing.md#Bidi_Conformance_Testing)
- 5  [Implementation Notes](016-4-4-bidirectional-conformance-testing.md#Implementation_Notes)
  - 5.1  [Reference Code](016-4-4-bidirectional-conformance-testing.md#Reference_Code)
  - 5.2  [Retaining BNs and
    Explicit Formatting Characters](016-4-4-bidirectional-conformance-testing.md#Retaining_Explicit_Formatting_Characters)
- 6  [Usage](016-4-4-bidirectional-conformance-testing.md#Usage)
  - 6.1  [Joiners](016-4-4-bidirectional-conformance-testing.md#Joiners)
  - 6.2  [Vertical Text](017-6-2-vertical-text.md#Vertical_Text)
  - 6.3  [Formatting](017-6-2-vertical-text.md#Formatting)
  - 6.4  [Separating Punctuation Marks](018-6-4-separating-punctuation-marks.md#Separators)
  - 6.5  [Conversion to Plain Text](018-6-4-separating-punctuation-marks.md#Conversion_to_Plain_Text)
- 7  [Mirroring](018-6-4-separating-punctuation-marks.md#Mirroring)
- [Migration Issues](018-6-4-separating-punctuation-marks.md#Migration)
  - [Section Reorganization](019-section-reorganization.md#section_reorg)
- [Acknowledgments](019-section-reorganization.md#Acknowledgements)
- [References](019-section-reorganization.md#References)
- [Modifications](019-section-reorganization.md#Modifications)

---

