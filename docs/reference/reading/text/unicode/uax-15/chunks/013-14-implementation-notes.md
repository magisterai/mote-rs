# 14 Implementation Notes

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-13-stream-safe-text-format.md) · [Next](014-14-2-code-samples.md)

<table border="0" style="BORDER-COLLAPSE: collapse">
<tr>
<td style="text-align:center" width="30"><font size="5">T</font></td>
<td style="text-align:center" width="30"><font size="5">h</font></td>
<td style="text-align:center" width="30"><font size="5">e</font></td>
<td style="text-align:center" width="30"> </td>
<td style="text-align:center; background-color:#FF8060" width="30"><font size="5">c</font></td>
<td style="text-align:center" width="30"><font size="5">◌́</font></td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="60"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
</tr>
<tr>
<td style="text-align:center" width="30">0</td>
<td style="text-align:center" width="30">1</td>
<td style="text-align:center" width="30">2</td>
<td style="text-align:center" width="30">3</td>
<td style="text-align:center" width="30">4</td>
<td style="text-align:center" width="30">5</td>
<td style="text-align:center" width="30">6</td>
<td style="text-align:center" width="60">...</td>
<td style="text-align:center" width="30">31</td>
<td style="text-align:center" width="30">32</td>
<td style="text-align:center" width="30">33</td>
<td style="text-align:center" width="30">34</td>
<td style="text-align:center" width="30">35</td>
<td style="text-align:center" width="30">36</td>
<td style="text-align:center" width="30">37</td>
<td style="text-align:center" width="30">38</td>
<td style="text-align:center" width="30">39</td>
</tr>
</table>

When the "a" (with QC=Y) is to be appended to the buffer, it is then
safe to compose the "c" and all subsequent characters, and then enter in
the "a", marking it as the last character with QC=Y.

<table border="0" style="BORDER-COLLAPSE: collapse">
<tr>
<td style="text-align:center" width="30"><font size="5">T</font></td>
<td style="text-align:center" width="30"><font size="5">h</font></td>
<td style="text-align:center" width="30"><font size="5">e</font></td>
<td style="text-align:center" width="30"> </td>
<td style="text-align:center" width="30"><font size="5">ć</font></td>
<td style="text-align:center; background-color:#FF8060" width="30"><font size="5">a</font></td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="60"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
</tr>
<tr>
<td style="text-align:center" width="30">0</td>
<td style="text-align:center" width="30">1</td>
<td style="text-align:center" width="30">2</td>
<td style="text-align:center" width="30">3</td>
<td style="text-align:center" width="30">4</td>
<td style="text-align:center" width="30">5</td>
<td style="text-align:center" width="30">6</td>
<td style="text-align:center" width="60">...</td>
<td style="text-align:center" width="30">31</td>
<td style="text-align:center" width="30">32</td>
<td style="text-align:center" width="30">33</td>
<td style="text-align:center" width="30">34</td>
<td style="text-align:center" width="30">35</td>
<td style="text-align:center" width="30">36</td>
<td style="text-align:center" width="30">37</td>
<td style="text-align:center" width="30">38</td>
<td style="text-align:center" width="30">39</td>
</tr>
</table>

<a id="Implementation_Notes"></a>

## 14  [Implementation Notes](013-14-implementation-notes.md#Implementation_Notes)

<a id="Optimization_Strategies"></a>

### 14.1  [Optimization Strategies](013-14-implementation-notes.md#Optimization_Strategies)

There are a number of optimizations that can be made in programs that normalize Unicode strings. This section lists
a few techniques for optimization. See also [[UTN5](https://www.unicode.org/reports/tr41/tr41-36.html#UTN5)] for other
information about possible optimizations.

Any implementation using optimization techniques must be carefully checked
to ensure that it still produces conformant results. In particular, the code must still be able to pass the
the NormalizationTest.txt conformance
test [[Tests15](https://www.unicode.org/reports/tr41/tr41-36.html#Tests15)].

<a id="NFC_QC_Optimization"></a>

#### 14.1.1  [Quick Check for NFC](013-14-implementation-notes.md#NFC_QC_Optimization)

When normalizing to NFC, rather than first decomposing the text fully, a quick check can be made on
each character. If it is already in the proper precomposed form, then no work has to be done. Only
if the current character is a combining mark or is in the
[Composition Exclusion Table](006-5-composition-exclusion.md#Primary_Exclusion_List_Table)
[[Exclusions](https://www.unicode.org/reports/tr41/tr41-36.html#Exclusions)], does a slower code
path need to be invoked. The slower code path will need to look at previous characters, back to the
last starter. See *Section 9,
[Detecting Normalization Forms](008-7-design-goals.md#Detecting_Normalization_Forms)*, for more information.

<a id="NFC_Table_Optimization"></a>

#### 14.1.2  [Optimizing Tables for NFC Composition](013-14-implementation-notes.md#NFC_Table_Optimization)

The majority of the cycles spent in doing composition are spent looking up the appropriate data.
The data lookup for Normalization Form C can be very efficiently implemented, because it has
to look up only pairs of characters, rather than arbitrary strings.
First, a multistage table (also known as a *trie;* see *Chapter 5, Implementation Guidelines*
in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)])
is used to map a character *c* to a small integer *i* in a contiguous range from
0 to *n*. The code for doing this looks like:



```
i = data[index[c >> BLOCKSHIFT] + (c & BLOCKMASK)];
```




Then a pair of these small integers are simply mapped through a two-dimensional array to get a
resulting value. This yields much better performance than a general-purpose string lookup in a
hash table.

<a id="NFD_Table_Optimization"></a>

#### 14.1.3  [Optimizing Tables for NFD Quick Check](013-14-implementation-notes.md#NFD_Table_Optimization)

The values of the Canonical\_Combining\_Class property are constrained by the character encoding stability guarantees
to the range 0..254; the value 255 will never be assigned for a Canonical\_Combining\_Class value. Because of this constraint,
implementations can make use of 255 as an implementation-specific value for optimizing data tables.
For example, one can do a fast and compact table for implementing isNFD(x) by using the value 255 to represent NFKC\_QC=No.

<a id="Hangul_Composition"></a>

#### 14.1.4  [Hangul Decomposition and Composition](013-14-implementation-notes.md#Hangul_Composition)

Because the decompositions
and compositions for Hangul syllables are algorithmic, memory
storage can be significantly reduced if the corresponding operations are
done in code, rather than by simply storing the data in the
general-purpose tables. See
*Section
3.12, Combining Jamo Behavior* in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]
for example code illustrating the Hangul Syllable Decomposition and the
Hangul Syllable Composition algorithms.

