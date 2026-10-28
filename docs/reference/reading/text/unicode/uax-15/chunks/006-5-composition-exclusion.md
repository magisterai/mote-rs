# 5 Composition Exclusion

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-3-versioning-and-stability.md) · [Next](007-5-2-composition-exclusion-data-files.md)

<a id="Primary_Exclusion_List_Table"></a>

## 5  [Composition Exclusion](006-5-composition-exclusion.md#Primary_Exclusion_List_Table)

The concept of *composition exclusion* is a key part of the Unicode Normalization
Algorithm. For normalization forms NFC and NFKC, which normalize Unicode strings to **C**omposed
forms, where possible, the basic process is first to fully decompose the string, and then
to compose the string, except where blocked or excluded. (See D117, Canonical Composition Algorithm,
in Section 3.11, Normalization Forms in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].)
This section provides information about the types of characters which are excluded from
composition during application of the Unicode Normalization Algorithm, and describes the
data files which provide the definitive lists of those characters.

Composition exclusion characters have an associated binary character property in
the [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]: Composition\_Exclusion. It is a
notable characteristic of the Unicode Normalization Algorithm that no composition
exclusion character can occur in any normalized form of Unicode text: NFD, NFC, NFKD, or NFKC.

<a id="Exclusion_Types"></a>

### 5.1  [Composition Exclusion Types](006-5-composition-exclusion.md#Exclusion_Types)

Four
types of canonically decomposable characters are excluded
from composition in the
Canonical Composition Algorithm. These four types are described and exemplified here.

#### Script-specific Exclusions

The term *script-specific exclusion* refers to certain canonically decomposable characters
whose decomposition includes one of a small set of
combining marks for particular Indian scripts, for Tibetan, or for Hebrew.

The list of such characters cannot be computed from the decomposition mappings
in the Unicode Character Database, and must instead be explicitly listed.

The character U+0958 (क़) DEVANAGARI LETTER QA is an example of a script-specific composition exclusion.

The list of script-specific composition exclusions constituted
a one-time adjustment to the Unicode Normalization Algorithm, defined at the time
of the [composition version](005-3-versioning-and-stability.md#Versioning) in 2001 and unchanged
since that version. The list can be
divided into the following three general groups, all added to the Unicode Standard before
Version 3.1:

- Many precomposed characters using a *nukta* diacritic in the Bangla/Bengali, Devanagari, Gurmukhi,
  or Odia/Oriya scripts, mostly consisting of additions to the core set of letters for those scripts.
- Tibetan letters and subjoined letters with decompositions that include either U+0FB7 TIBETAN SUBJOINED LETTER HA
  or U+0FB5 TIBETAN SUBJOINED LETTER SSA,
  and two two-part Tibetan vowel signs involving top and bottom pieces.
- A large collection of compatibility precomposed characters for Hebrew involving *dagesh* and/or
  other combining marks.

Although, in principle,
the list of script-specific composition exclusions could be expanded
to add newly encoded characters in
future versions of the Unicode Standard, it is very unlikely to be extended for
such characters, because the normalization forms of sequences are now taken
into account *before* new characters are encoded.

#### Post Composition Version Exclusions

The term *post composition version exclusion* refers to certain
canonical decomposable characters which were added after the
[composition version](005-3-versioning-and-stability.md#Versioning), and which meet certain criteria for
exclusion.

The list of such characters cannot be computed from the decomposition mappings
in the Unicode Character Database, and must instead be explicitly listed.

A canonical decomposable character *must* be added to the list of post
composition version exclusions when its decomposition mapping is defined
to contain two characters, both of which were already encoded in an earlier version
of the Unicode Standard. This criterion is required to maintain normalization stability.
Without the composition exclusion, any previously existing sequence of the two characters
would change to the newly encoded character in NFC, destabilizing the normalized
form of pre-existing text.

A canonical decomposable character *may* be added to the list of post
composition version exclusions when its decomposition mapping is defined
to contain just
one character which was already encoded in an earlier version
of the Unicode Standard. Under these circumstances,
a composition exclusion is not required for normalization stability, but could be
optionally specified by the UTC if there were a determination that the maximally
decomposed sequence was preferred in all normalization forms.

An example of such a post composition version exclusion is
U+2ADC (⫝̸) FORKING. To date, that one character, encoded in Unicode 3.2,
is the *only* character added to the list of composition exclusions based
on the criterion of its decomposition mapping containing a
single prior-encoded character.

A canonical decomposable character *may* also be added to the list of post
composition version exclusions when its decomposition mapping is defined
to contain only characters which are first encoded in same version
of the Unicode Standard as the canonical decomposable character, itself.

An example of such a post composition version exclusion is
U+1D15F (𝅘𝅥) MUSICAL SYMBOL QUARTER NOTE. To date, that character and a related
set of musical note symbols, encoded in Unicode 3.1, are the *only*
characters added to the list of composition exclusions based on the
criterion of their decomposition mappings containing only characters encoded
in the same version of the Unicode Standard. Note that, techically, the encoding
of those particular musical symbols did not formally postdate the
[composition version](005-3-versioning-and-stability.md#Versioning), but that fact is now a historical
oddity resulting from early uncertainty as to whether the composition version
would be fixed at Unicode 3.0 or Unicode 3.1.

In principle, future canonical decomposable characters could
be added to the list of post composition version exclusions, if the UTC
determines that their preferred representation is a decomposed sequence.
In practice, this situation has not actually occurred since the publication
of Unicode 3.1, and is unlikely to occur in the future, given current
practice for assigning decomposition mappings for newly encoded characters.

#### Singleton Exclusions

A singleton decomposition is defined as a canonical
decomposition mapping from a character to different single character.
(See D110 in Section 3.11, Normalization Forms in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].)
Characters which have single decompositions are automatically excluded from
composition in the Canonical Composition Algorithm.

The list of characters with singleton decompositions is
directly derivable from the list of decomposition mappings in the
Unicode Character Database. For information, that list is also provided
in comment lines in CompositionExclusions.txt in the UCD.

An example of a singleton exclusion is
U+2126 (Ω) OHM SIGN.

There are cases where two characters
have the same canonical decomposition in the Unicode Character Database.
*Table 5* shows an example.

<a id="Same_Decomposition_Table"></a>

Table 5. [Same Canonical Decomposition](006-5-composition-exclusion.md#Same_Decomposition_Table)

<table class="subtle">
<tr>
<th>Character</th>
<th>Full Decomposition</th>
</tr>
<tr>
<td>212B (Å) ANGSTROM SIGN</td>
<td rowspan="2">0041 (A) LATIN CAPITAL LETTER A + 030A 
		(°) COMBINING RING ABOVE</td>
</tr>
<tr>
<td>00C5 (Å) LATIN CAPITAL LETTER A WITH RING ABOVE</td>
</tr>
</table>

In such a case, the practice is to assign a singleton decomposition
for one character to the other. The full decomposition for both characters then
is derived from the decomposition mapping for the second character. In this particular
case U+212B ANGSTROM SIGN has a singleton decomposition to
U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE. Instances of characters with
such singleton decompositions occur in the Unicode Standard for compatibility
with certain pre-existing character encoding standards.

#### Non-starter Decomposition Exclusions

A non-starter decomposition is defined as an expanding
canonical decomposition which is not a starter decomposition.
(See D110b and D111 in Section 3.11, Normalization Forms in
[[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].)
Characters which have non-starter decompositions are automatically excluded from
composition in the Canonical Composition Algorithm.

The list of characters with non-starter decompositions is
directly derivable from the list of decomposition mappings in the
Unicode Character Database. For information, that list is also provided
in comment lines in CompositionExclusions.txt in the UCD.

An example of a non-starter decomposition exclusion is
U+0344 (◌̈́) COMBINING GREEK DIALYTIKA TONOS.

