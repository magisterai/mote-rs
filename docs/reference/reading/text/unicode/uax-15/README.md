# UAX #15: Unicode Normalization Forms

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Topic guide](../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

14 reading chunks. Headings, examples, tables, mathematical markup, figures, and source anchors are retained. Publisher logos and navigation images are replaced by text; subsection arrows are omitted from headings.

- [Unicode® Standard Annex #15](chunks/001-unicode-standard-annex-15.md)
  - Unicode Normalization Forms
  - Summary
  - Status
  - Contents
  - 1 Introduction
- [1.1 Canonical and Compatibility Equivalence](chunks/002-1-1-canonical-and-compatibility-equivalence.md)
- [1.2 Normalization Forms](chunks/003-1-2-normalization-forms.md)
- [1.3 Description of the Normalization Process](chunks/004-1-3-description-of-the-normalization-process.md)
  - 1.4 Concatenation of Normalized Strings
  - 2 Notation
- [3 Versioning and Stability](chunks/005-3-versioning-and-stability.md)
  - 4 Conformance
- [5 Composition Exclusion](chunks/006-5-composition-exclusion.md)
  - 5.1 Composition Exclusion Types
  - Script-specific Exclusions
  - Post Composition Version Exclusions
  - Singleton Exclusions
  - Non-starter Decomposition Exclusions
- [5.2 Composition Exclusion Data Files](chunks/007-5-2-composition-exclusion-data-files.md)
  - 6 Examples and Charts
  - Basic Examples
  - Effect of Compatibility Decompositions
- [7 Design Goals](chunks/008-7-design-goals.md)
  - Goal 1: Uniqueness
  - Goal 2: Stability
  - Goal 3: Efficiency
  - 8 Legacy Encodings
  - 9 Detecting Normalization Forms
- [9.1 Stable Code Points](chunks/009-9-1-stable-code-points.md)
  - 9.2 Normalization Contexts Requiring Care in Optimization
  - 10 Respecting Canonical Equivalence
- [11 Stability Prior to Unicode 4.1](chunks/010-11-stability-prior-to-unicode-4-1.md)
  - 11.1 Stability of Normalized Forms
  - 11.2 Stability of the Normalization Process
  - 11.3 Guaranteeing Process Stability
- [11.4 Forbidding Characters](chunks/011-11-4-forbidding-characters.md)
  - 11.5 Corrigendum 5 Sequences
  - 12 Stabilized Strings
  - 12.1 Normalization Process for Stabilized Strings
- [13 Stream-Safe Text Format](chunks/012-13-stream-safe-text-format.md)
  - 13.1 Buffering with Unicode Normalization
- [14 Implementation Notes](chunks/013-14-implementation-notes.md)
  - 14.1 Optimization Strategies
  - 14.1.1 Quick Check for NFC
  - 14.1.2 Optimizing Tables for NFC Composition
  - 14.1.3 Optimizing Tables for NFD Quick Check
  - 14.1.4 Hangul Decomposition and Composition
- [14.2 Code Samples](chunks/014-14-2-code-samples.md)
  - Appendix A: Intellectual Property Considerations
  - Acknowledgments
  - References
  - Modifications
  - Revision 57 \[KW\]
