# UAX #9: Unicode Bidirectional Algorithm

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Topic guide](../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

19 reading chunks. Headings, examples, tables, mathematical markup, figures, and source anchors are retained. Publisher logos and navigation images are replaced by text; subsection arrows are omitted from headings.

- [Unicode® Standard Annex #9](chunks/001-unicode-standard-annex-9.md)
  - Unicode Bidirectional Algorithm
  - Summary
  - Status
  - *Contents*
- [1 Introduction](chunks/002-1-introduction.md)
  - 2 Directional Formatting Characters
  - 2.1 Explicit Directional Embeddings
- [2.2 Explicit Directional Overrides](chunks/003-2-2-explicit-directional-overrides.md)
  - 2.3 Terminating Explicit Directional Embeddings and Overrides
  - 2.4 Explicit Directional Isolates
  - 2.5 Terminating Explicit Directional Isolates
  - 2.6 Implicit Directional Marks
  - 2.7 Markup and Formatting Characters
- [3 Basic Display Algorithm](chunks/004-3-basic-display-algorithm.md)
  - 3.1 Definitions
  - 3.1.1 Basics
  - Example
- [3.1.2 Matching Explicit Directional Formatting Characters](chunks/005-3-1-2-matching-explicit-directional-formatting-characters.md)
  - Example 1
  - Example 2
  - Example 3
- [3.1.3 Paired Brackets](chunks/006-3-1-3-paired-brackets.md)
  - Examples of bracket pairs
  - 3.1.4 Additional Abbreviations
  - 3.2 Bidirectional Character Types
- [3.3 Resolving Embedding Levels](chunks/007-3-3-resolving-embedding-levels.md)
- [3.3.1 The Paragraph Level](chunks/008-3-3-1-the-paragraph-level.md)
  - 3.3.2 Explicit Levels and Directions
  - Explicit Embeddings
  - Explicit Overrides
  - Isolates
- [Non-formatting characters](chunks/009-non-formatting-characters.md)
  - Terminating Isolates
  - Terminating Embeddings and Overrides
  - End of Paragraph
- [3.3.3 Preparations for Implicit Processing](chunks/010-3-3-3-preparations-for-implicit-processing.md)
- [3.3.4 Resolving Weak Types](chunks/011-3-3-4-resolving-weak-types.md)
  - 3.3.5 Resolving Neutral and Isolate Formatting Types
- [3.3.6 Resolving Implicit Levels](chunks/012-3-3-6-resolving-implicit-levels.md)
  - 3.4 Reordering Resolved Levels
- [3.5 Shaping](chunks/013-3-5-shaping.md)
  - 3.5.1 Shaping and Line Breaking
  - 4 Bidirectional Conformance
- [4.1 Boundary Neutrals](chunks/014-4-1-boundary-neutrals.md)
  - 4.2 Explicit Formatting Characters
  - 4.3 Higher-Level Protocols
  - 4.3.1 HL4 Example 1 for XML
- [4.3.2 HL4 Example 2 for Program Text](chunks/015-4-3-2-hl4-example-2-for-program-text.md)
  - 4.3.3 HL4 Example 3 for URLs
- [4.4 Bidirectional Conformance Testing](chunks/016-4-4-bidirectional-conformance-testing.md)
  - 5 Implementation Notes
  - 5.1 Reference Code
  - 5.2 Retaining BNs and Explicit Formatting Characters
  - 6 Usage
  - 6.1 Joiners
- [6.2 Vertical Text](chunks/017-6-2-vertical-text.md)
  - 6.3 Formatting
- [6.4 Separating Punctuation Marks](chunks/018-6-4-separating-punctuation-marks.md)
  - 6.5 Conversion to Plain Text
  - 7 Mirroring
  - Migration Issues
- [Section Reorganization](chunks/019-section-reorganization.md)
  - Acknowledgments
  - References
  - Modifications
  - Revision 51
