# 11 Stability Prior to Unicode 4.1

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-9-1-stable-code-points.md) · [Next](011-11-4-forbidding-characters.md)

<a id="Stability_Prior_to_Unicode41"></a>

## 11  [Stability Prior to Unicode 4.1](010-11-stability-prior-to-unicode-4-1.md#Stability_Prior_to_Unicode41)

For versions prior to Unicode 4.1 (that do not apply Corrigenda
#2 through #5), slightly weaker stability policies are in effect. For information on these stability policies, especially regarding
normalization, see the Unicode Character Encoding Stability
Policy [[Policies](https://www.unicode.org/reports/tr41/tr41-36.html#Policies)].

These policies still guaranteed, in
particular, that:

> *Once a character is encoded, its canonical combining class and decomposition mapping will
> not be changed in a way that will destabilize normalization.*

What this means is:

> *If a string contains only characters from a given version of the Unicode Standard (for
> example, Unicode 3.1.1), and it is put into a normalized form in accordance with that
> version of Unicode, then it will be in normalized form according to any future version
> of Unicode.*

This guarantee has been in place for Unicode 3.1 and after. It has been necessary to correct
the decompositions of a small number of characters since Unicode 3.1, as listed in the
Normalization Corrections data file [[Corrections](https://www.unicode.org/reports/tr41/tr41-36.html#Corrections)], but such corrections
are in accordance with the above principles: all text normalized on old systems will test as
normalized in future systems. All text normalized in future systems will test as normalized on
past systems. Prior to Unicode 4.1, what may change for those few characters, is that *unnormalized* text may
normalize differently on past and future systems.

<a id="Stability_of_Normalized_Forms"></a>

### 11.1  [Stability of Normalized Forms](010-11-stability-prior-to-unicode-4-1.md#Stability_of_Normalized_Forms)

For all versions, even prior to
Unicode 4.1, the following policy is followed:

*A normalized string is guaranteed to be stable; that
is, once normalized, a string is normalized according to all future versions of Unicode.*

More precisely, if a string has been normalized according to a particular
version of Unicode *and* contains only characters allocated in that version, it
will qualify as normalized according to any future version of Unicode.

<a id="Stability_of_the_Normalization_Process"></a>

### 11.2  [Stability of the Normalization Process](010-11-stability-prior-to-unicode-4-1.md#Stability_of_the_Normalization_Process)

For all versions, even prior to Unicode 4.1, the *process* of producing a normalized string from an
unnormalized string has the same results under each version of Unicode, except for certain edge
cases addressed in the following corrigenda:

- Three corrigenda correct certain data mappings for a total of
  seven characters:

  |  |
  | --- |
  | Corrigendum #2, “[U+FB1D Normalization](https://www.unicode.org/versions/corrigendum2.html)” [[Corrigendum2](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum2)] |
  | Corrigendum #3, “[U+F951 Normalization](https://www.unicode.org/versions/corrigendum3.html)” [[Corrigendum3](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum3)] |
  | Corrigendum #4, “[Five Unihan Canonical Mapping Errors](https://www.unicode.org/versions/corrigendum4.html)” [[Corrigendum4](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum4)] |
- Corrigendum #5, “[Normalization Idempotency](https://www.unicode.org/versions/corrigendum5.html)”
  [[Corrigendum5](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum5)], fixed a problem in the description of the
  normalization process for some instances of particular sequences. *Such instances never occur
  in meaningful text.*

<a id="Guaranteeing_Process_Stability"></a>

### 11.3  [Guaranteeing Process Stability](010-11-stability-prior-to-unicode-4-1.md#Guaranteeing_Process_Stability)

The Unicode Standard provides a
mechanism for those implementations that require
not only normalized strings, *but also the normalization process*, to be absolutely stable
between two versions even prior to Unicode 4.1 (including the edge cases mentioned in *Section 11.2,
[Stability of the Normalization Process](010-11-stability-prior-to-unicode-4-1.md#Stability_of_the_Normalization_Process)*). This, of course,
is true only where the repertoire of characters is limited to those characters present in the
earlier version of Unicode.

To have the newer implementation produce the same results as the
older version (for characters defined as of the older version):

1. Premap a maximum of seven (rare) characters according to whatever
   corrigenda came between the two versions (see [[Errata](https://www.unicode.org/reports/tr41/tr41-36.html#Errata)]).
   - For example, for a Unicode 4.0 implementation to produce the
     same results as Unicode 3.2, the five characters mentioned in
     [[Corrigendum4](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum4)] are premapped
     to the *old* values given in version 4.0 of the UCD data file [[Corrections](https://www.unicode.org/reports/tr41/tr41-36.html#Corrections)].
2. Apply the later version of normalization.
   - Handle any code points that were not defined in the earlier version
     as if they were unassigned: such code points will not decompose or compose,
     and their Canonical\_Combining\_Class value will be zero.
   - The Derived\_Age property in the Unicode Character Database can be used
     to determine whether a code point is assigned for any particular version
     of the standard.
3. If the earlier version is before Unicode 4.1 and the later version
   is 4.1 or later and if the normalization is to forms
   NFC or NFKC, perform the following steps:
   - Reorder the sequences listed in *Table 10* of *Section 11.5,
     [Corrigendum 5 Sequences](011-11-4-forbidding-characters.md#Corrigendum_5_Sequences)*, as follows:
     > <table class="simple">
     > <tr>
     > <td><b>From:</b></td>
     > <td style="text-align: center">first_character</td>
     > <td style="text-align: center">intervening_character(s)</td>
     > <td style="text-align: center"><i>last_character</i></td>
     > </tr>
     > <tr>
     > <td><b>To:</b></td>
     > <td style="text-align: center">first_character</td>
     > <td style="text-align: center"><i>last_character</i></td>
     > <td style="text-align: center">intervening_character(s)</td>
     > </tr>
     > </table>
   - Replace the first\_character and last\_character sequence with the canonically
     equivalent composed character, according to the Canonical Composition Algorithm.

> Note: For step 3, in most implementations it is actually more
> efficient (and much simpler) to parameterize the code to provide for both pre- and post-Unicode
> 4.1 behavior. This typically takes only one additional conditional statement.

Implementations of the Unicode Normalization Algorithm prior to
version 4.1 were not all consistent with each other. Some followed the letter of
the specification; because of the defect in the specification addressed by Corrigendum #5
[[Corrigendum5](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum5)], such implementations
were not idempotent, and their normalization results for
the edge cases addressed by the corrigendum were not always well-defined. Other
implementations followed the intent of the specification and implemented
based on the normalization examples and reference code; those implementations behave *as if*
Corrigendum #5 had already been applied. When developing a current implementation
to guarantee process stability even for earlier versions of the standard, it
is important to know which type of earlier Unicode implementation of normalization
is being targeted. Step 3 outlined above only needs to be applied to guarantee
process stability for interoperating with early implementations that followed
the letter of the specification prior to version 4.1. Step 3 can be omitted when
interoperating with implementations that behaved as if Corrigendum #5 had already
been applied.

