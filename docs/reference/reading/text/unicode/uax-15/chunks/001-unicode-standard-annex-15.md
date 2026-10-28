# Unicode® Standard Annex #15

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-1-1-canonical-and-compatibility-equivalence.md)

[Unicode](https://www.unicode.org/) · [Technical Reports](https://www.unicode.org/reports/)

## Unicode® Standard Annex #15

# Unicode Normalization Forms

|  |  |
| --- | --- |
| Version | Unicode 17.0.0 |
| Editors | Ken Whistler |
| Date | 2025-07-30 |
| This Version | [https://www.unicode.org/reports/tr15/tr15-57.html](../README.md) |
| Previous Version | [https://www.unicode.org/reports/tr15/tr15-56.html](https://www.unicode.org/reports/tr15/tr15-56.html) |
| Latest Version | [https://www.unicode.org/reports/tr15/](https://www.unicode.org/reports/tr15/) |
| Latest Proposed Update | [https://www.unicode.org/reports/tr15/proposed.html](https://www.unicode.org/reports/tr15/proposed.html) |
| Revision | [57](014-14-2-code-samples.md#Modifications) |

#### Summary

*This annex describes normalization forms for Unicode text.
When implementations keep strings in a normalized form, they can be assured that equivalent
strings have a unique binary representation.
This annex also provides examples, additional specifications
regarding normalization of Unicode text, and information about conformance
testing for Unicode normalization forms.*

#### Status



*This document has been reviewed by Unicode members and other interested
parties, and has been approved for publication by the Unicode Consortium.
This is a stable document and may be used as reference material or cited as
a normative reference by other specifications.*

> ***A Unicode Standard Annex (UAX)** forms an integral part of the Unicode Standard, but
> is published online as a separate document. The Unicode Standard may require conformance to normative
> content in a Unicode Standard Annex, if so specified in the Conformance chapter of that version
> of the Unicode Standard. The version number of a UAX document corresponds to the version of the Unicode Standard of which it forms a part.*

*Please submit corrigenda and other comments with the online reporting
form [[Feedback](https://www.unicode.org/reporting.html)].
Related information that is useful in understanding this annex is found in Unicode Standard Annex #41,
“[Common References for Unicode Standard Annexes](https://www.unicode.org/reports/tr41/tr41-36.html).”
For the latest version of the Unicode Standard, see [[Unicode](https://www.unicode.org/versions/latest/)].
For a list of current Unicode Technical Reports, see [[Reports](https://www.unicode.org/reports/)].
For more information about versions of the Unicode Standard, see [[Versions](https://www.unicode.org/versions/)].
For any errata which may apply to this annex, see [[Errata](https://www.unicode.org/errata/)].*

#### Contents

- 1 [Introduction](001-unicode-standard-annex-15.md#Introduction)
  - 1.1 [Canonical and Compatibility Equivalence](002-1-1-canonical-and-compatibility-equivalence.md#Canon_Compat_Equivalence)
  - 1.2 [Normalization Forms](003-1-2-normalization-forms.md#Norm_Forms)
  - 1.3 [Description of the Normalization Process](004-1-3-description-of-the-normalization-process.md#Description_Norm)
  - 1.4 [Concatenation of Normalized Strings](004-1-3-description-of-the-normalization-process.md#Concatenation)
- 2 [Notation](004-1-3-description-of-the-normalization-process.md#Notation)
- 3 [Versioning and Stability](005-3-versioning-and-stability.md#Versioning)
- 4 [Conformance](005-3-versioning-and-stability.md#Conformance)
- 5 [Composition Exclusion](006-5-composition-exclusion.md#Primary_Exclusion_List_Table)
  - 5.1 [Composition
    Exclusion Types](006-5-composition-exclusion.md#Exclusion_Types)
  - 5.2 [Composition Exclusion Data Files](007-5-2-composition-exclusion-data-files.md#Exclusion_Data_File)
- 6 [Examples and Charts](007-5-2-composition-exclusion-data-files.md#Examples)
- 7 [Design Goals](008-7-design-goals.md#Design_Goals)
- 8 [Legacy Encodings](008-7-design-goals.md#Legacy_Encodings)
- 9 [Detecting Normalization Forms](008-7-design-goals.md#Detecting_Normalization_Forms)
  - 9.1 [Stable Code Points](009-9-1-stable-code-points.md#Stable_Code_Points)
  - 9.2 [Normalization Contexts Requiring Care in Optimization](009-9-1-stable-code-points.md#Contexts_Care)
- 10 [Respecting Canonical Equivalence](009-9-1-stable-code-points.md#Canonical_Equivalence)
- 11 [Stability Prior to Unicode 4.1](010-11-stability-prior-to-unicode-4-1.md#Stability_Prior_to_Unicode41)
  - 11.1
    [Stability of Normalized Forms](010-11-stability-prior-to-unicode-4-1.md#Stability_of_Normalized_Forms)
  - 11.2
    [Stability of the Normalization Process](010-11-stability-prior-to-unicode-4-1.md#Stability_of_the_Normalization_Process)
  - 11.3
    [Guaranteeing Process Stability](010-11-stability-prior-to-unicode-4-1.md#Guaranteeing_Process_Stability)
  - 11.4 [Forbidding Characters](011-11-4-forbidding-characters.md#Forbidding_Characters)
  - 11.5 [Corrigendum 5 Sequences](011-11-4-forbidding-characters.md#Corrigendum_5_Sequences)
- 12 [Stabilized Strings](011-11-4-forbidding-characters.md#Stabilized_Strings)
  - 12.1
    [Normalization Process for Stabilized Strings](011-11-4-forbidding-characters.md#Normalization_Process_for_Stabilized_Strings)
- 13 [Stream-Safe Text Format](012-13-stream-safe-text-format.md#Stream_Safe_Text_Format)
  - 13.1
    [Buffering with Unicode Normalization](012-13-stream-safe-text-format.md#Buffering_with_Unicode_Normalization)
- 14 [Implementation Notes](013-14-implementation-notes.md#Implementation_Notes)
  - 14.1 [Optimization Strategies](013-14-implementation-notes.md#Optimization_Strategies)
  - 14.2 [Code Samples](014-14-2-code-samples.md#Code_Sample)
- Appendix A: [Intellectual Property Considerations](014-14-2-code-samples.md#Intellectual_Property_Annex)
- [Acknowledgments](014-14-2-code-samples.md#Acknowledgments)
- [References](014-14-2-code-samples.md#References)
- [Modifications](014-14-2-code-samples.md#Modifications)

  


---

<a id="Introduction"></a>

## 1  [Introduction](001-unicode-standard-annex-15.md#Introduction)

This annex provides subsidiary information about
Unicode normalization. It describes canonical and compatibility equivalence
and the four normalization forms, providing
examples, and elaborates on the formal specification of Unicode normalization,
with further explanations and implementation notes.

This document also provides the formal specification
of the Stream-Safe Text Format and of the Normalization Process for Stabilized
Strings.

For the formal specification of the Unicode Normalization
Algorithm, see *Section 3.11, Normalization Forms* in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

For a general introduction to the topic of equivalent
sequences for Unicode strings and the need for normalization, see
*Section 2.12, Equivalent Sequences and Normalization* in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

