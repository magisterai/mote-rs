# 5.2 Composition Exclusion Data Files

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-5-composition-exclusion.md) · [Next](008-7-design-goals.md)

<a id="Exclusion_Data_File"></a>

### 5.2  [Composition Exclusion Data Files](007-5-2-composition-exclusion-data-files.md#Exclusion_Data_File)

The
list of composition exclusion characters
(Composition\_Exclusion = True)
is available as a machine-readable
data file, CompositionExclusions.txt
[[Exclusions](https://www.unicode.org/reports/tr41/tr41-36.html#Exclusions)]
in the Unicode Character
Database [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)].

All four classes of composition exclusion characters are included in this file,
although the singletons and non-starter decompositions are
provided in comment lines,
as they can be computed directly from the decomposition mappings in the Unicode
Character Database.

A derived property containing the complete list of
full composition exclusion characters
(Full\_Composition\_Exclusion = True),
is available separately in the Unicode Character Database [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)]
and is described in Unicode Standard Annex #44, "Unicode Character Database"
[[UAX44](https://www.unicode.org/reports/tr41/tr41-36.html#UAX44)]. Implementations can
avoid having to compute the singleton and non-starter decompositions from the Unicode Character Database
by using the Full\_Composition\_Exclusion property instead.

> Note:
> By definition, the set of characters with Full\_Composition\_Exclusion=True
> is the same as the set of characters with
> [NFC\_Quick\_Check](008-7-design-goals.md#Quick_Check_Table)=No.
> (This can be useful for reducing the size of data in some implementations.)

<a id="Examples"></a>

## 6  [Examples and Charts](007-5-2-composition-exclusion-data-files.md#Examples)

This section provides some detailed examples of the results when each of the
Normalization Forms is applied. The Normalization Charts
[[Charts15](https://www.unicode.org/reports/tr41/tr41-36.html#Charts15)] provide charts of all the characters in Unicode that differ
from at least one of their Normalization Forms (NFC, NFD, NFKC, NFKD).

#### Basic Examples

The
basic examples in *Table 6* do not involve compatibility
decompositions. Therefore, in each case Normalization Forms NFD and NFKD are
identical, and Normalization Forms NFC and NFKC are also identical.

<a id="Basic_Example_Table"></a>

Table 6. [Basic Examples](007-5-2-composition-exclusion-data-files.md#Basic_Example_Table)

<table class="subtle">
<tr>
<th valign="top" width="1"></th>
<th valign="top" width="20%">Original</th>
<th valign="top" width="20%">NFD, NFKD</th>
<th valign="top" width="20%">NFC, NFKC</th>
<th align="LEFT" valign="TOP">Notes</th>
</tr>
<tr>
<th valign="top" width="10">a</th>
<td align="CENTER" valign="TOP" width="20%">D-dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D-dot_above</td>
<td rowspan="2" valign="TOP">Both decomposed and precomposed canonical 
      sequences produce the same result.</td>
</tr>
<tr>
<th valign="top" width="10">b</th>
<td align="CENTER" valign="TOP" width="20%">D + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D-dot_above</td>
</tr>
<tr>
<th valign="top" width="10">c</th>
<td align="CENTER" valign="TOP" width="20%">D-dot_below + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D + dot_below + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D-dot_below + dot_above</td>
<td rowspan="3" valign="TOP">The <i>dot_above</i> cannot be combined 
      with the D because the D has already combined with the intervening <i>dot_below</i>.<p> </p></td>
</tr>
<tr>
<th valign="top" width="10">d</th>
<td align="CENTER" valign="TOP" width="20%">D-dot_above + dot_below</td>
<td align="CENTER" valign="TOP" width="20%">D + dot_below + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D-dot_below + dot_above</td>
</tr>
<tr>
<th valign="top" width="10">e</th>
<td align="CENTER" valign="TOP" width="20%">D + dot_above + dot_below</td>
<td align="CENTER" valign="TOP" width="20%">D + dot_below + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D-dot_below + dot_above</td>
</tr>
<tr>
<th valign="top" width="10">f</th>
<td align="CENTER" valign="TOP" width="20%">D + dot_above + horn + dot_below</td>
<td align="CENTER" valign="TOP" width="20%">D + horn + dot_below + dot_above</td>
<td align="CENTER" valign="TOP" width="20%">D-dot_below + horn + dot_above</td>
<td valign="TOP">There may be intervening combining marks, so long as the 
      result of the combination is canonically equivalent.</td>
</tr>
<tr>
<th valign="top" width="10">g</th>
<td align="CENTER" valign="TOP" width="20%">E-macron-grave</td>
<td align="CENTER" valign="TOP" width="20%">E + macron + grave</td>
<td align="CENTER" valign="TOP" width="20%">E-macron-grave</td>
<td rowspan="2">Multiple combining characters are combined with the base 
      character.</td>
</tr>
<tr>
<th valign="top" width="10">h</th>
<td align="CENTER" valign="TOP" width="20%">E-macron + grave</td>
<td align="CENTER" valign="TOP" width="20%">E + macron + grave</td>
<td align="CENTER" valign="TOP" width="20%">E-macron-grave</td>
</tr>
<tr>
<th valign="top" width="10">i</th>
<td align="CENTER" valign="TOP" width="20%">E-grave + macron</td>
<td align="CENTER" valign="TOP" width="20%">E + grave + macron</td>
<td align="CENTER" valign="TOP" width="20%">E-grave + macron</td>
<td>Characters will <i>not</i> be combined if they would not be canonical 
      equivalents because of their ordering.</td>
</tr>
<tr>
<th valign="top" width="10">j</th>
<td align="CENTER" valign="TOP" width="20%">angstrom_sign</td>
<td align="CENTER" valign="TOP" width="20%">A + ring</td>
<td align="CENTER" valign="TOP" width="20%">A-ring</td>
<td rowspan="2" valign="TOP">Because Å (A-ring) is the preferred composite, it 
      is the form produced for both characters.</td>
</tr>
<tr>
<th valign="top" width="10">k</th>
<td align="CENTER" valign="TOP" width="20%">A-ring</td>
<td align="CENTER" valign="TOP" width="20%">A + ring</td>
<td align="CENTER" valign="TOP" width="20%">A-ring</td>
</tr>
</table>

  

#### Effect of Compatibility Decompositions

The examples in *Table 7* and *Table 8* illustrate the
effect of compatibility decompositions. When text is normalized in forms NFD
and NFC, as in *Table 7*, compatibility-equivalent strings do not
result in the same strings. However, when the same strings are normalized in
forms NFKD and NFKC, as shown in *Table 8*, they do result in the same
strings. The tables also contain an entry showing that Hangul syllables are
maintained under all Normalization Forms.

<a id="NFD_And_NFC_Applied_Table"></a>

Table 7. [NFD and NFC Applied to Compatibility-Equivalent Strings](007-5-2-composition-exclusion-data-files.md#NFD_And_NFC_Applied_Table)

<table class="subtle">
<tr>
<th valign="top" width="1"></th>
<th valign="top" width="20%">Original</th>
<th valign="top" width="20%">NFD</th>
<th valign="top" width="20%">NFC</th>
<th align="LEFT" valign="TOP">Notes</th>
</tr>
<tr>
<th valign="top" width="10">l</th>
<td align="CENTER" valign="top" width="20%">"Äffin"</td>
<td align="CENTER" valign="top" width="20%">"A\u0308ffin"</td>
<td align="CENTER" valign="top" width="20%">"Äffin"</td>
<td rowspan="2" valign="TOP">The <i>ffi_ligature</i> (U+FB03) is <i>not</i> 
      decomposed, because it has a compatibility mapping, not a canonical mapping. (See
      <i>Table 8</i>.)</td>
</tr>
<tr>
<th valign="top" width="10">m</th>
<td align="CENTER" valign="top" width="20%">"Ä\uFB03n"</td>
<td align="CENTER" valign="top" width="20%">"A\u0308\uFB03n"</td>
<td align="CENTER" valign="top" width="20%">"Ä\uFB03n"</td>
</tr>
<tr>
<th valign="top" width="10">n</th>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
<td rowspan="2" valign="TOP">Similarly, the ROMAN NUMERAL IV (U+2163) is <i>
      not</i> decomposed.</td>
</tr>
<tr>
<th valign="top" width="10">o</th>
<td align="CENTER" valign="top" width="20%">"Henry \u2163"</td>
<td align="CENTER" valign="top" width="20%">"Henry \u2163"</td>
<td align="CENTER" valign="top" width="20%">"Henry \u2163"</td>
</tr>
<tr>
<th valign="top" width="10">p</th>
<td align="CENTER" valign="top" width="20%">ga</td>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ga</td>
<td rowspan="5" valign="TOP">Different compatibility equivalents of a single 
      Japanese character will <i>not</i> result in the same string in NFC.</td>
</tr>
<tr>
<th valign="top" width="10">q</th>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ga</td>
</tr>
<tr>
<th valign="top" width="10">r</th>
<td align="CENTER" valign="top" width="20%">hw_ka + hw_ten</td>
<td align="CENTER" valign="top" width="20%">hw_ka + hw_ten</td>
<td align="CENTER" valign="top" width="20%">hw_ka + hw_ten</td>
</tr>
<tr>
<th valign="top" width="10">s</th>
<td align="CENTER" valign="top" width="20%">ka + hw_ten</td>
<td align="CENTER" valign="top" width="20%">ka + hw_ten</td>
<td align="CENTER" valign="top" width="20%">ka + hw_ten</td>
</tr>
<tr>
<th valign="top" width="10">t</th>
<td align="CENTER" valign="top" width="20%">hw_ka + ten</td>
<td align="CENTER" valign="top" width="20%">hw_ka + ten</td>
<td align="CENTER" valign="top" width="20%">hw_ka + ten</td>
</tr>
<tr>
<th valign="top" width="10">u</th>
<td align="CENTER" valign="top" width="20%">kaks</td>
<td align="CENTER" valign="top" width="20%">k<sub>i</sub> + a<sub>m</sub> + ks<sub>f</sub></td>
<td align="CENTER" valign="top" width="20%">kaks</td>
<td align="CENTER" valign="TOP">Hangul syllables are maintained under normalization.</td>
</tr>
</table>

  


<a id="NFKD_And_NFKC_Applied_Table"></a>

Table 8. [NFKD and NFKC Applied to Compatibility-Equivalent Strings](007-5-2-composition-exclusion-data-files.md#NFKD_And_NFKC_Applied_Table)

<table class="subtle">
<tr>
<th valign="top" width="10"></th>
<th valign="top" width="20%">Original</th>
<th valign="top" width="20%">NFKD</th>
<th valign="top" width="20%">NFKC</th>
<th align="LEFT" valign="TOP">Notes</th>
</tr>
<tr>
<th valign="top" width="10">l'</th>
<td align="CENTER" valign="top" width="20%">"Äffin"</td>
<td align="CENTER" valign="top" width="20%">"A\u0308ffin"</td>
<td align="CENTER" valign="top" width="20%">"Äffin"</td>
<td rowspan="2" valign="TOP">The <i>ffi_ligature</i> (U+FB03) <i>is</i> 
      decomposed in NFKC (where it is not in NFC).</td>
</tr>
<tr>
<th valign="top" width="10">m'</th>
<td align="CENTER" valign="top" width="20%">"Ä\uFB03n"</td>
<td align="CENTER" valign="top" width="20%">"A\u0308ffin"</td>
<td align="CENTER" valign="top" width="20%">"Äffin"</td>
</tr>
<tr>
<th valign="top" width="10">n'</th>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
<td rowspan="2" valign="TOP">Similarly, the resulting strings here are 
      identical in NFKC.</td>
</tr>
<tr>
<th valign="top" width="10">o'</th>
<td align="CENTER" valign="top" width="20%">"Henry \u2163"</td>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
<td align="CENTER" valign="top" width="20%">"Henry IV"</td>
</tr>
<tr>
<th valign="top" width="10">p'</th>
<td align="CENTER" valign="top" width="20%">ga</td>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ga</td>
<td rowspan="5" valign="TOP">Different compatibility equivalents of a single 
      Japanese character <i>will</i> result in the same string in NFKC.</td>
</tr>
<tr>
<th valign="top" width="10">q'</th>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ga</td>
</tr>
<tr>
<th valign="top" width="10">r'</th>
<td align="CENTER" valign="top" width="20%">hw_ka + hw_ten</td>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ga</td>
</tr>
<tr>
<th valign="top" width="10">s'</th>
<td align="CENTER" valign="top" width="20%">ka + hw_ten</td>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ga</td>
</tr>
<tr>
<th valign="top" width="10">t'</th>
<td align="CENTER" valign="top" width="20%">hw_ka + ten</td>
<td align="CENTER" valign="top" width="20%">ka + ten</td>
<td align="CENTER" valign="top" width="20%">ga</td>
</tr>
<tr>
<th valign="top" width="10">u'</th>
<td align="CENTER" valign="top" width="20%">kaks</td>
<td align="CENTER" valign="top" width="20%">k<sub>i</sub> + a<sub>m</sub> + ks<sub>f</sub></td>
<td align="CENTER" valign="top" width="20%">kaks</td>
<td align="CENTER" valign="TOP">Hangul syllables are maintained under normalization.*</td>
</tr>
</table>

> \* In earlier versions of Unicode, jamo characters like ks<sub>f</sub> had
> compatibility mappings to k<sub>f</sub> + s<sub>f</sub>. These mappings were removed in Unicode
> 2.1.9 to ensure that Hangul syllables would be maintained.

