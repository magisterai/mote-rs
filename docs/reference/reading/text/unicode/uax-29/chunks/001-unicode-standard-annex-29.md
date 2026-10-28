# Unicode® Standard Annex #29

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-1-1-notation.md)

[Unicode](https://www.unicode.org/) · [Technical Reports](https://www.unicode.org/reports/)

## Unicode® Standard Annex #29

# Unicode Text Segmentation

|  |  |
| --- | --- |
| Version | Unicode 17.0.0 |
| Editors | Josh Hadley ([johadley@adobe.com](mailto:johadley@adobe.com)) |
| Date | 2025-08-17 |
| This Version | [https://www.unicode.org/reports/tr29/tr29-47.html](../README.md) |
| Previous Version | [https://www.unicode.org/reports/tr29/tr29-45.html](https://www.unicode.org/reports/tr29/tr29-45.html) |
| Latest Version | [https://www.unicode.org/reports/tr29/](https://www.unicode.org/reports/tr29/) |
| Latest Proposed Update | [https://www.unicode.org/reports/tr29/proposed.html](https://www.unicode.org/reports/tr29/proposed.html) |
| Revision | [47](013-8-1-standard-korean-syllables.md#Modifications) |

#### Summary

*This annex describes guidelines for determining default
segmentation boundaries between certain significant text elements:
grapheme clusters (“user-perceived characters”), words, and
sentences. For line boundaries, see [[UAX14](https://www.unicode.org/reports/tr41/tr41-36.html#UAX14)]*.

#### Status



*This document has been reviewed by Unicode members and other
interested parties, and has been approved for publication by the Unicode
Consortium. This is a stable document and may be used as reference
material or cited as a normative reference by other specifications.*

> ***A Unicode Standard Annex (UAX)** forms an integral part
> of the Unicode Standard, but is published online as a separate
> document. The Unicode Standard may require conformance to normative
> content in a Unicode Standard Annex, if so specified in the
> Conformance chapter of that version of the Unicode Standard. The
> version number of a UAX document corresponds to the version of the
> Unicode Standard of which it forms a part.*

*Please submit corrigenda and other comments with the online
reporting form [[Feedback](https://www.unicode.org/reporting.html)].
Related information that is useful in understanding this annex is
found in Unicode Standard Annex #41, “[Common
References for Unicode Standard Annexes](https://www.unicode.org/reports/tr41/tr41-36.html).” For the latest version of
the Unicode Standard, see [[Unicode](https://www.unicode.org/versions/latest/)]. For a
list of current Unicode Technical Reports, see [[Reports](https://www.unicode.org/reports/)]. For more
information about versions of the Unicode Standard, see [[Versions](https://www.unicode.org/versions/)]. For any
errata which may apply to this annex, see [[Errata](https://www.unicode.org/errata/)].*

#### Contents

- 1 [Introduction](001-unicode-standard-annex-29.md#Introduction)
  - 1.1 [Notation](002-1-1-notation.md#Notation)
  - 1.2 [Rule
    Constraints](002-1-1-notation.md#Rule_Constraints)
- 2 [Conformance](002-1-1-notation.md#Conformance)
- 3 [Grapheme
  Cluster Boundaries](003-3-grapheme-cluster-boundaries.md#Grapheme_Cluster_Boundaries)
  - 3.1 [Default
    Grapheme Cluster Boundary Specification](004-3-1-default-grapheme-cluster-boundary-specification.md#Default_Grapheme_Cluster_Table)
    - 3.1.1 [Grapheme
      Cluster Boundary Rules](004-3-1-default-grapheme-cluster-boundary-specification.md#Grapheme_Cluster_Boundary_Rules)
- 4 [Word Boundaries](005-4-word-boundaries.md#Word_Boundaries)
  - 4.1 [Default Word
    Boundary Specification](006-4-1-default-word-boundary-specification.md#Default_Word_Boundaries)
    - 4.1.1 [Word Boundary
      Rules](006-4-1-default-word-boundary-specification.md#Word_Boundary_Rules)
  - 4.2 [Name Validation](009-4-2-name-validation.md#Name_Validation)
- 5 [Sentence Boundaries](009-4-2-name-validation.md#Sentence_Boundaries)
  - 5.1 [Default
    Sentence Boundary Specification](009-4-2-name-validation.md#Default_Sentence_Boundaries)
    - 5.1.1 [Sentence
      Boundary Rules](009-4-2-name-validation.md#Sentence_Boundary_Rules)
- 6 [Implementation Notes](011-6-implementation-notes.md#Implementation_Notes)
  - 6.1 [Normalization](011-6-implementation-notes.md#Normalization)
  - 6.2 [Replacing
    Ignore Rules](011-6-implementation-notes.md#Grapheme_Cluster_and_Format_Rules)
  - 6.3 [State
    Machines](011-6-implementation-notes.md#State_Machines)
  - 6.4 [Random Access](011-6-implementation-notes.md#Random_Access)
  - 6.5 [Tailoring](012-6-5-tailoring.md#Tailoring)
- 7 [Testing](012-6-5-tailoring.md#Testing)
- 8 [Hangul
  Syllable Boundary Determination](012-6-5-tailoring.md#Hangul_Syllable_Boundary_Determination)
  - 8.1 [Standard
    Korean Syllables](013-8-1-standard-korean-syllables.md#Standard_Korean_Syllables)
  - 8.2 [Transforming into
    Standard Korean Syllables](013-8-1-standard-korean-syllables.md#Transforming_Into_SKS)
- [Acknowledgments](013-8-1-standard-korean-syllables.md#Acknowledgments)
- [References](013-8-1-standard-korean-syllables.md#References)
- [Modifications](013-8-1-standard-korean-syllables.md#Modifications)

---

<a id="Introduction"></a>

## 1  [Introduction](001-unicode-standard-annex-29.md#Introduction)

This annex describes guidelines for determining default boundaries
between certain significant text elements: user-perceived characters,
words, and sentences. The process of boundary determination is also
called *segmentation*.

A string of Unicode-encoded text often needs to be broken up into
text elements programmatically. Common examples of text elements
include what users think of as characters, words, lines (more
precisely, where line breaks are allowed), and sentences. The precise
determination of text elements may vary according to orthographic
conventions for a given script or language. The goal of matching user
perceptions cannot always be met exactly because the text alone does
not always contain enough information to unambiguously decide
boundaries. For example, the *period* (U+002E FULL STOP) is
used ambiguously, sometimes for end-of-sentence purposes, sometimes
for abbreviations, and sometimes for numbers. In most cases, however,
programmatic text boundaries can match user perceptions quite
closely, although sometimes the best that can be done is not to
surprise the user.

Rather than concentrate on algorithmically searching for text
elements (often called *segments*), a simpler and more useful
computation instead detects the *boundaries* (or *breaks*)
between those text elements. The determination of those boundaries is
often critical to performance, so it is important to be able to make
such a determination as quickly as possible. (For a general
discussion of text elements, see *Chapter 2, General Structure*,
of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].)

The default boundary determination mechanism specified in this annex
provides a straightforward and efficient way to determine some of the
most significant boundaries in text: user-perceived characters,
words, and sentences. Boundaries used in line breaking (also called *word
wrapping*) are defined in [[UAX14](https://www.unicode.org/reports/tr41/tr41-36.html#UAX14)].

The sheer number of characters in the Unicode Standard,
together with its representational power, place requirements on both
the specification of text element boundaries and the underlying
implementation. The specification needs to allow the designation of
large sets of characters sharing the same characteristics (for
example, uppercase letters), while the implementation must provide
quick access and matches to those large sets. The mechanism also must
handle special features of the Unicode Standard, such as nonspacing
marks and conjoining jamos.

The default boundary determination builds upon the uniform
character representation of the Unicode Standard, while handling the
large number of characters and special features such as nonspacing
marks and conjoining jamos in an effective manner. As this mechanism
lends itself to a completely data-driven implementation, it can be
tailored to particular orthographic conventions or user preferences
without recoding.

As in other Unicode algorithms, these specifications provide a *logical*
description of the processes: implementations can achieve the same
results without using code or data that follows these rules
step-by-step. In particular, many production-grade implementations
will use a state-table approach. In that case, the performance does
not depend on the complexity or number of rules. Rather, performance
is only affected by the number of characters that may match *after* the boundary position in a rule that applies.

