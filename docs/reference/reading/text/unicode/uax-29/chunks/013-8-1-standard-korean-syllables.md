# 8.1 Standard Korean Syllables

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-6-5-tailoring.md)

<a id="Standard_Korean_Syllables"></a>

### 8.1  [Standard Korean Syllables](013-8-1-standard-korean-syllables.md#Standard_Korean_Syllables)

- *Standard Korean syllable block:* A sequence of one or
  more L followed by a sequence of one or more V and a sequence of
  zero or more T, or any other sequence that is canonically
  equivalent.

- All precomposed Hangul syllables, which have the form LV or
  LVT, are standard Korean syllable blocks.
- Alternatively, a standard Korean syllable block may be
  expressed as a sequence of a choseong and a jungseong, optionally
  followed by a jongseong.
- A choseong filler may substitute for a missing leading
  consonant, and a jungseong filler may substitute for a missing
  vowel.

Using regular expression notation, a canonically decomposed
standard Korean syllable block is of the following form:

L+ V+ T\*

Arbitrary standard Korean syllable blocks have a somewhat more
complex form because they include any canonically equivalent
sequence, thus including precomposed Korean syllables. The regular
expressions for them have the following form:

(L+ V+ T\*) | (L\* LV V\* T\*) | (L\* LVT T\*)

All standard Korean syllable blocks used in modern Korean are
of the form <L V T> or <L V> and have equivalent,
single-character precomposed forms.

Old Korean characters are represented by a series of conjoining
jamos. While the Unicode Standard allows for two L, V, or T
characters as part of a syllable, KS X 1026-1 only allows single
instances. Implementations that need to conform to KS X 1026-1 can
tailor the default rules in *Section 3.1  [Default Grapheme Cluster
Boundary Specification](004-3-1-default-grapheme-cluster-boundary-specification.md#Default_Grapheme_Cluster_Table)* accordingly.

<a id="Transforming_Into_SKS"></a>

### 8.2  [Transforming into Standard Korean Syllables](013-8-1-standard-korean-syllables.md#Transforming_Into_SKS)

A sequence of jamos that do not all match the regular expression for
a standard Korean syllable block can be transformed into a sequence
of standard Korean syllable blocks by the correct insertion of
choseong fillers (L*<sub>f</sub>* ) and jungseong fillers (V*<sub>f</sub>*
). This transformation of a string of text into standard Korean
syllables is performed by determining the syllable breaks as
explained in the earlier subsection “Hangul Syllable Boundaries,”
then inserting one or two fillers as necessary to transform each
syllable into a standard Korean syllable as shown in [*Figure 6*](013-8-1-standard-korean-syllables.md#Inserting_Fillers).

<a id="Inserting_Fillers"></a>

Figure 6. [Inserting
Fillers](013-8-1-standard-korean-syllables.md#Inserting_Fillers)

<table class="simple">
<tr>
<td>L [^V] → L V<i><sub>f</sub></i> [^V]
					</td>
</tr>
<tr>
<td>[^L] V → [^L] L<i><sub>f</sub></i> V
					</td>
</tr>
<tr>
<td>[^V] T → [^V] L<i><sub>f</sub></i> V<i><sub>f</sub></i> T
					</td>
</tr>
</table>

In *Figure 6*, [^X] indicates a character that is not X, or the
absence of a character.

In [*Table 7*](013-8-1-standard-korean-syllables.md#Korean_Syllable_Break_Examples), the
first row shows syllable breaks in a standard sequence, the second
row shows syllable breaks in a nonstandard sequence, and the third
row shows how the sequence in the second row could be transformed
into standard form by inserting fillers into each syllable. Syllable
breaks are shown by *middle dots* “·”.

<a id="Korean_Syllable_Break_Examples"></a>

Table 7. [Korean Syllable Break
Examples](013-8-1-standard-korean-syllables.md#Korean_Syllable_Break_Examples)

<table class="subtle">
<tbody>
<tr>
<th>No.</th>
<th>Sequence</th>
<th> </th>
<th>Sequence with Syllable Breaks Marked</th>
</tr>
<tr>
<td style="text-align: center">1</td>
<td>LVTLVLVLV<i><sub>f</sub></i> L<i><sub>f</sub></i> VL<i><sub>f</sub></i>
							V<i><sub>f</sub></i> T
						</td>
<td>→</td>
<td>LVT · LV · LV · LV<i><sub>f</sub></i> · L<i><sub>f</sub></i>
							V · L<i><sub>f</sub></i> V<i><sub>f</sub></i> T
						</td>
</tr>
<tr>
<td style="text-align: center">2</td>
<td>LLTTVVTTVVLLVV</td>
<td>→</td>
<td>LL · TT · VVTT · VV · LLVV</td>
</tr>
<tr>
<td style="text-align: center">3</td>
<td>LLTTVVTTVVLLVV</td>
<td>→</td>
<td>LLV<i><sub>f</sub></i> · L<i><sub>f</sub></i> V<i><sub>f</sub></i>
							TT · L<i><sub>f</sub></i> VVTT · L<i><sub>f</sub></i> VV · LLVV
						</td>
</tr>
</tbody>
</table>

<a id="Acknowledgments"></a>

## [Acknowledgments](013-8-1-standard-korean-syllables.md#Acknowledgments)

Mark Davis is the author of the initial version and has added to and
maintained the text of this annex through Version 14.0. Laurențiu Iancu assisted in updating it for
Versions 7.0 through 10.0.

Thanks to Julie Allen, Asmus Freytag, Manish
Goregaokar, Andy Heninger, Ted Hopp, Tsuyoshi
Ito, Martin Hosken, Michael Kaplan, Johan Curcio Lindström, Eric Mader, Otto Stolz, Steve Tolkin, Ken Whistler, and
Karl Williamson for their feedback on this annex, including earlier
versions.

<a id="References"></a>

## [References](013-8-1-standard-korean-syllables.md#References)

For references for this annex, see Unicode Standard Annex #41, “[Common References for Unicode
Standard Annexes](https://www.unicode.org/reports/tr41/tr41-36.html).”

<a id="Modifications"></a>

## [Modifications](013-8-1-standard-korean-syllables.md#Modifications)

The following summarizes modifications from the previous
published version of this annex.

### Revision 47

- **Reissued** for Unicode 17.0.0.
- [Section 4.1, Default Word Boundary Specification](006-4-1-default-word-boundary-specification.md#Default_Word_Boundaries):
  Updated the derivation of [Word\_Break=ALetter](006-4-1-default-word-boundary-specification.md#ALetter) to include
  U+00B8 CEDILLA based on usage in Saanich. [[184-C27](https://www.unicode.org/cgi-bin/GetL2Ref.pl?184-C27)]

Modifications for previous versions are listed in those respective versions.

---

© 2004–2025 Unicode, Inc. This publication is protected by copyright, and permission must be obtained from Unicode, Inc. prior to any reproduction, modification, or other use not permitted by the [Terms of Use](https://www.unicode.org/copyright.html). Specifically, you may make copies of this publication and may annotate and translate it solely for personal or internal business purposes and not for public distribution, provided that any such permitted copies and modifications fully reproduce all copyright and other legal notices contained in the original. You may not make copies of or modifications to this publication for public distribution, or incorporate it in whole or in part into any product or publication without the express written permission of Unicode.

Use of all Unicode Products, including this publication, is governed by the Unicode [Terms of Use](https://www.unicode.org/copyright.html). The authors, contributors, and publishers have taken care in the preparation of this publication, but make no express or implied representation or warranty of any kind and assume no responsibility or liability for errors or omissions or for consequential or incidental damages that may arise therefrom. This publication is provided “AS-IS” without charge as a convenience to users.

Unicode and the Unicode Logo are registered trademarks of Unicode, Inc., in the United States and other countries.
