# 3 Versioning and Stability

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-1-3-description-of-the-normalization-process.md) · [Next](006-5-composition-exclusion.md)

<a id="Versioning"></a>

## 3  [Versioning and Stability](005-3-versioning-and-stability.md#Versioning)

It is crucial that Normalization Forms remain stable over time. That is, if a string that does
not have any unassigned characters is normalized under one version of Unicode, it must remain
normalized under all future versions of Unicode. This is the backward compatibility requirement.
To meet this requirement, a fixed version for the composition process is specified, called
the *composition version.* The composition version is defined to be **Version 3.1.0**
of the Unicode Character Database. For more information, see

- Versions of the Unicode Standard [[Versions](https://www.unicode.org/reports/tr41/tr41-36.html#Versions)]
- Unicode 3.1 [[Unicode3.1](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode3.1)]
- Unicode Character Database [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]

To see what difference the composition version makes, suppose that a future version of Unicode
were to add the composite *Q-caron*. For an implementation that uses that future version of
Unicode, strings in Normalization Form C or KC would continue to contain the sequence *Q + caron,*
and *not* the new character *Q-caron*, because a canonical composition for *Q-caron*
was not defined in the composition version. See *Section 5,
[Composition Exclusion Table](006-5-composition-exclusion.md#Primary_Exclusion_List_Table)*, for more information.

It would be possible to add more compositions in a future version of Unicode, as long as the
backward compatibility requirement is met. It requires that for any new composition XY
→ Z, at
most one of X or Y was defined in a previous version of Unicode. That is, Z must be a new
character, and either X or Y must be a new character. However, the Unicode Consortium strongly
discourages new compositions, even in such restricted cases.

In addition to fixing the composition version, future versions of Unicode must be restricted in
terms of the kinds of changes that can be made to character properties. Because of this, the
Unicode Consortium has a clear policy to guarantee the stability of
Normalization Forms.

The Unicode Consortium has well-defined
policies in place to govern changes that affect backward compatibility.
According to the Unicode policy for Normalization Forms, applicable to
Unicode 4.1 and all later versions, the results of normalizing a string on
one version will always be the same as normalizing it on any other version,
as long as the string contains only assigned characters according to both
versions. For information on these stability policies, especially regarding
normalization, see the Unicode Character Encoding Stability Policy [[Policies](https://www.unicode.org/reports/tr41/tr41-36.html#Policies)].

If an implementation normalizes a string
that contains characters that are **not** assigned in
the version of Unicode that it supports, that string **might not** be in normalized form according to a future
version of Unicode. For example, suppose that a Unicode
5.0 program normalizes a string that contains new Unicode
5.1 characters. That string might not be normalized according
to Unicode 5.1.

Prior to Unicode 4.1, the stability policy was
not quite as strict. For more information, see *Section 11 [Stability Prior to Unicode 4.1](010-11-stability-prior-to-unicode-4-1.md#Stability_Prior_to_Unicode41).*

<a id="Conformance"></a>

## 4  [Conformance](005-3-versioning-and-stability.md#Conformance)

Starting with Unicode 5.2.0, conformance clauses UAX15-C1 and UAX15-C2
have been redirected to point to the formal specification of Unicode Normalization
Forms in *Section 3.11, Normalization Forms* in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]. All of the local clauses have been
retained in this annex, so that any external references to Unicode Standard Annex #15 and
to particular conformance clauses for Unicode Normalization Forms will continue to be valid.
Specific references to any definitions used by the Unicode Normalization Algorithm
also remain valid.

<a id="UAX15-C1"></a>

***[UAX15-C1](005-3-versioning-and-stability.md#UAX15-C1).** A process that produces Unicode text that
purports to be in a Normalization Form shall do so in accordance with the specifications in
Section 3.11, Normalization Forms in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].*

- See C13 in *Chapter 3, Conformance* in
  [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]

<a id="UAX15-C2"></a>

***[UAX15-C2](005-3-versioning-and-stability.md#UAX15-C2).** A process that tests Unicode text to
determine whether it is in a Normalization Form shall do so in accordance with the specifications
in Section 3.11, Normalization Forms in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]*

- See C14 in *Chapter 3, Conformance* in
  [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]

<a id="UAX15-C3"></a>

***[UAX15-C3](005-3-versioning-and-stability.md#UAX15-C3).** A process that purports to transform text
into a Normalization Form must be able to
produce the results of the conformance test specified in
the NormalizationTest.txt data
file [[Test15](https://www.unicode.org/reports/tr41/tr41-36.html#Tests15)].*

- See C15 in *Chapter 3, Conformance* in
  [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]
- The NormalizationText.txt file consists of a series of fields.
  When Normalization Forms are
  applied to the different fields in the test file, the results shall be as specified
  in the header of that file.

<a id="UAX15-C4"></a>

***[UAX15-C4](005-3-versioning-and-stability.md#UAX15-C4).**
A process that purports to transform
text into the [Stream-Safe Text Format](012-13-stream-safe-text-format.md#Stream_Safe_Text_Format) must do so
according to the Stream-Safe Text Process defined in [UAX15-D4](012-13-stream-safe-text-format.md#UAX15-D4).*

<a id="UAX15-C5"></a>

***[UAX15-C5](005-3-versioning-and-stability.md#UAX15-C5).** A process that purports to
transform text according to the
[Normalization
Process for Stabilized Strings](011-11-4-forbidding-characters.md#Normalization_Process_for_Stabilized_Strings) must do so in accordance with the
specifications in this annex.*

The specifications for Normalization Forms are written in terms of a process for
producing a decomposition or composition from an arbitrary Unicode string. This is a
*logical*
description—particular implementations can have more efficient mechanisms as long as they
produce the same result. See C18 in *Chapter 3, Conformance* in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]
and the notes following.

Implementations must be thoroughly tested for conformance to the
normalization specification.
Testing for a particular Normalization Form does not require
directly applying the process of normalization, so long as the result of the test is equivalent to
applying normalization and then testing for binary identity.

