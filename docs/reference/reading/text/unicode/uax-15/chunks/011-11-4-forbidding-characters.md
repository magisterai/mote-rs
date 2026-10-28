# 11.4 Forbidding Characters

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-11-stability-prior-to-unicode-4-1.md) · [Next](012-13-stream-safe-text-format.md)

<a id="Forbidding_Characters"></a>

### 11.4  [Forbidding Characters](011-11-4-forbidding-characters.md#Forbidding_Characters)

An alternative approach for certain protocols is to forbid characters
that differ in normalization status across versions
prior to Unicode 4.1. The characters and
sequences affected are not in any practical use, so this may be viable for
some implementations. For example, when upgrading from Unicode 3.2 to
Unicode 5.0, there are three relevant corrigenda:

- Corrigendum #3, “[U+F951
  Normalization](https://www.unicode.org/versions/corrigendum3.html)” [[Corrigendum3](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum3)]
- Corrigendum #4, “[Five Unihan
  Canonical Mapping Errors](https://www.unicode.org/versions/corrigendum4.html)” [[Corrigendum4](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum4)]   
  The five characters are U+2F868, U+2F874, U+2F91F, U+2F95F, and U+2F9BF.
- Corrigendum #5, “[Normalization Idempotency](https://www.unicode.org/versions/corrigendum5.html)”
  [[Corrigendum5](https://www.unicode.org/reports/tr41/tr41-36.html#Corrigendum5)]

> The characters in Corrigenda #3 and #4 are
> all extremely rare Han characters. They are compatibility characters
> included only for compatibility with a single East Asian
> character set standard each: U+F951 for a duplicate character
> in KS X 1001, and the other five for CNS 11643-1992. That’s
> why
> they have canonical decomposition mappings in the first place.
>
> The duplicate character in KS X 1001 is a rare character in
> Korean to begin with—in a South Korean standard, where the
> use of Han characters at all is uncommon in actual data. And
> this is a pronunciation duplicate, which even if it were used
> would very likely be inconsistently and incorrectly used by
> end users, because there is no visual way for them to make
> the correct distinctions.
>
> The five characters from CNS 11643-1992 have even less utility.
> They are minor glyphic variants of unified characters—the
> kinds of distinctions that are subsumed already within all
> the unified Han ideographs in the Unicode Standard. They are from Planes 4–15 of
> CNS 11643-1992, which never saw any commercial implementation
> in Taiwan. The IT systems in Taiwan almost all implemented
> Big Five instead, which was a slight variant on Planes 1 and 2
> of CNS 11643-1986, and which included none of the five glyph
> variants in question here.
>
> As for Corrigendum #5, it is important to recognize that none of the
> affected sequences occur in any well-formed text in any language. See
> *Section 11.5, [Corrigendum
> 5 Sequences](011-11-4-forbidding-characters.md#Corrigendum_5_Sequences).*

<a id="Corrigendum_5_Sequences"></a>

### 11.5  [Corrigendum 5 Sequences](011-11-4-forbidding-characters.md#Corrigendum_5_Sequences)

*Table 10* shows all of the problem sequences relevant to Corrigendum 5. *It is important to emphasize that none of these sequences will occur in any
meaningful text, because none of the intervening characters shown in the sequences occur in the
contexts shown in the table.*

<a id="Problem_Sequence_Table"></a>

Table 10. [Problem Sequences](011-11-4-forbidding-characters.md#Problem_Sequence_Table)

<table class="subtle">
<tr>
<th align="left" width="38%">First Character</th>
<th align="left" nowrap="" width="15%">
      Intervening<br/>
		Character(s)</th>
<th align="left" width="41%">Last Character</th>
</tr>
<tr>
<td>09C7 BENGALI VOWEL SIGN E</td>
<td rowspan="13" style="vertical-align:middle">One or more characters 
      with a non-zero
      Canonical Combining Class property value — for example, an acute accent.</td>
<td>09BE BENGALI VOWEL SIGN AA <b>or</b><br/>
      09D7 BENGALI AU LENGTH MARK</td>
</tr>
<tr>
<td>0B47 ORIYA VOWEL SIGN E</td>
<td>0B3E ORIYA VOWEL SIGN AA <b>or</b><br/>
      0B56 ORIYA AI LENGTH MARK <b>or</b><br/>
      0B57 ORIYA AU LENGTH MARK</td>
</tr>
<tr>
<td>0BC6 TAMIL VOWEL SIGN E</td>
<td>0BBE TAMIL VOWEL SIGN AA <b>or</b><br/>
      0BD7 TAMIL AU LENGTH MARK</td>
</tr>
<tr>
<td>0BC7 TAMIL VOWEL SIGN EE</td>
<td>0BBE TAMIL VOWEL SIGN AA</td>
</tr>
<tr>
<td>0B92 TAMIL LETTER O</td>
<td>0BD7 TAMIL AU LENGTH MARK</td>
</tr>
<tr>
<td>0CC6 KANNADA VOWEL SIGN E</td>
<td>0CC2 KANNADA VOWEL SIGN UU <b>or</b><br/>
      0CD5 KANNADA LENGTH MARK <b>or</b><br/>
      0CD6 KANNADA AI LENGTH MARK</td>
</tr>
<tr>
<td>0CBF KANNADA VOWEL SIGN I <b>or</b><br/>
      0CCA KANNADA VOWEL SIGN O</td>
<td>0CD5 KANNADA LENGTH MARK</td>
</tr>
<tr>
<td>0D47 MALAYALAM VOWEL SIGN EE</td>
<td>0D3E MALAYALAM VOWEL SIGN AA</td>
</tr>
<tr>
<td>0D46 MALAYALAM VOWEL SIGN E</td>
<td>0D3E MALAYALAM VOWEL SIGN AA <b>or</b><br/>
      0D57 MALAYALAM AU LENGTH MARK</td>
</tr>
<tr>
<td>1025 MYANMAR LETTER U</td>
<td>102E MYANMAR VOWEL SIGN II</td>
</tr>
<tr>
<td>0DD9 SINHALA VOWEL SIGN KOMBUVA</td>
<td>0DCF SINHALA VOWEL SIGN AELA-PILLA <b>or</b><br/> 
        0DDF SINHALA VOWEL SIGN GAYANUKITTA</td>
</tr>
<tr>
<td>[1100-1112] HANGUL CHOSEONG KIYEOK..HIEUH<br/>(19 instances)</td>
<td>[1161-1175] HANGUL JUNGSEONG A..I<br/>(21 instances)</td>
</tr>
<tr>
<td>[:HangulSyllableType=LV:]</td>
<td>[11A8..11C2] HANGUL JONGSEONG KIYEOK..HIEUH<br/>(27 instances)</td>
</tr>
</table>

> **Note:** This
> table is constructed on the premise that the text is being normalized
> and that the first character has already been
> composed if possible. If the table is used externally to normalization to
> assess whether any problem sequences occur, then the implementation must
> also catch cases that are canonical equivalents. That is only relevant to
> the case [:HangulSyllableType=LV:]; the equivalent sequences of <x,y> where
> x is in [1100..1112] and y is in [1161..1175] must also be detected.

<a id="Stabilized_Strings"></a>

## 12  [Stabilized Strings](011-11-4-forbidding-characters.md#Stabilized_Strings)

In certain protocols, there is a requirement
for a normalization process for *stabilized* strings. The key concept
is that for a given normalization form, once a Unicode string has been successfully normalized according to
the process, it will *never* change if subsequently normalized again,
in any version of Unicode, past or future. To meet this need, the *Normalization Process for Stabilized Strings* (NPSS) is defined. NPSS adds to
regular normalization the requirement that an implementation must abort with
an error if it encounters any characters that are not in the current version
of Unicode.

<a id="Normalization_Process_for_Stabilized_Strings"></a>

### 12.1  [Normalization Process for Stabilized Strings](011-11-4-forbidding-characters.md#Normalization_Process_for_Stabilized_Strings)

The Normalization Process for Stabilized
Strings (NPSS) for a given normalization form (NFD, NFC, NFKD, or NFKC) is the same
as the corresponding process for generating that form, except that:

- The process must be aborted with an error
  if the string contains any code point with the property value
  General\_Category=Unassigned, according to the version of Unicode used
  for the normalization process.

*Examples:*  

<table class="subtle">
<tr>
<th rowspan="2" style="text-align: center">Sample Characters</th>
<th colspan="4" style="text-align: center">Required Behavior for Unicode Version</th>
</tr>
<tr>
<th style="text-align: center">3.2</th>
<th style="text-align: center">4.0</th>
<th style="text-align: center">4.1</th>
<th style="text-align: center">5.0</th>
</tr>
<tr>
<td>
			U+0234 (ȴ) LATIN SMALL LETTER L WITH CURL<br/>
			(added in Unicode 4.0)</td>
<td style="text-align: center; background-color:#c0c0c0">Abort</td>
<td style="text-align: center">Accept</td>
<td style="text-align: center">Accept</td>
<td style="text-align: center">Accept</td>
</tr>
<tr>
<td>
			U+0237 (ȷ) LATIN SMALL LETTER DOTLESS J<br/>
			(added in Unicode 4.1)</td>
<td style="text-align: center; background-color:#c0c0c0">Abort</td>
<td style="text-align: center; background-color:#c0c0c0">Abort</td>
<td style="text-align: center">Accept</td>
<td style="text-align: center">Accept</td>
</tr>
<tr>
<td>
			U+0242 (ɂ) LATIN SMALL LETTER GLOTTAL STOP<br/>
			(added in Unicode 5.0)</td>
<td style="text-align: center; background-color:#c0c0c0">Abort</td>
<td style="text-align: center; background-color:#c0c0c0">Abort</td>
<td style="text-align: center; background-color:#c0c0c0">Abort</td>
<td style="text-align: center">Accept</td>
</tr>
</table>

Once a string has been normalized by the
NPSS for a particular normalization form, it will never change if
renormalized for that same normalization form by an implementation that
supports any version of Unicode, past or future. For example, if an
implementation normalizes a string to NFC, following the constraints of NPSS
(aborting with an error if it encounters any unassigned code point for the
version of Unicode it supports), the resulting normalized string would be
stable: it would remain completely unchanged if renormalized to NFC by any
conformant Unicode normalization implementation supporting a prior or a
future version of the standard.

Note that NPSS defines a process, not another
normalization form. The resulting string is simply in a particular normalization form. If a
different implementation applies the NPSS again to that string, then
depending on the version of Unicode supported by the other implementation,
either the same string will result, or an error will occur. Given a string
that is purported to have been produced by the NPSS for a given
normalization form, what an implementation can determine is one of the
following three conditions:

1. definitely produced by NPSS (it is
   normalized, and contains no unassigned characters)
2. definitely not produced by NPSS (it is not
   normalized)
3. may or may not have been produced by NPSS
   (it contains unassigned characters but is otherwise normalized)

The additional data required for the stable
normalization process can be easily implemented with a compact lookup table.
Most libraries supplying normalization functions also supply the required
property tests, and in those normalization functions it is straightforward
for them to provide an additional parameter which invokes the stabilized process.

NPSS only applies to Unicode 4.1 and later, or
to implementations that apply Corrigenda
#2 through #5 to earlier versions: see *Section 11 [Stability
Prior to Unicode 4.1](010-11-stability-prior-to-unicode-4-1.md#Stability_Prior_to_Unicode41)*. A protocol that requires stability even
across other versions is a *restricted*
protocol. Such a protocol must define and use a *restricted* NPSS, a process that also aborts with an error if encounters
any problematic characters or sequences, as discussed in *Section 11.4 [Forbidding Characters](011-11-4-forbidding-characters.md#Forbidding_Characters)*.

