# 1.1 Canonical and Compatibility Equivalence

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-unicode-standard-annex-15.md) · [Next](003-1-2-normalization-forms.md)

<a id="Canon_Compat_Equivalence"></a>

### 1.1  [Canonical and Compatibility Equivalence](002-1-1-canonical-and-compatibility-equivalence.md#Canon_Compat_Equivalence)

The Unicode Standard defines two formal types of equivalence between characters:
*canonical equivalence*
and *compatibility equivalence*. Canonical equivalence is a fundamental equivalency between characters or
sequences of characters which represent the same
abstract character, and which when correctly displayed should always
have the same visual appearance and
behavior. *Figure 1* illustrates this type of equivalence
with examples of several subtypes.

<a id="Canonical_Equivalence_Figure"></a>

Figure 1. [Examples of Canonical Equivalence](002-1-1-canonical-and-compatibility-equivalence.md#Canonical_Equivalence_Figure)

<table class="subtle">
<tr>
<th>Subtype</th>
<th colspan="3">Examples</th>
</tr>
<tr>
<td>Combining sequence</td>
<td style="text-align: center"><span class="charSample">Ç</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">↔</span></td>
<td style="text-align: center"><span class="charSample">C+◌̧</span></td>
</tr>
<tr>
<td>Ordering of combining marks</td>
<td style="text-align: center"><span class="charSample">q+◌̇+◌̣</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">↔</span></td>
<td style="text-align: center"><span class="charSample">q+◌̣+◌̇</span></td>
</tr>
<tr>
<td>Hangul &amp; conjoining jamo</td>
<td style="text-align: center"><span class="charSample">가</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">↔</span></td>
<td style="text-align: center"><span class="charSample">ᄀ +ᅡ</span></td>
</tr>
<tr>
<td>Singleton equivalence</td>
<td style="text-align: center"><span class="charSample">Ω</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">↔</span></td>
<td style="text-align: center"><span class="charSample">Ω</span></td>
</tr>
</table>

Compatibility equivalence is a weaker type of equivalence
between characters or sequences of characters which represent the same
abstract character (or sequence of abstract characters), but which may have
distinct visual appearances or behaviors.
The visual appearances of the compatibility equivalent
forms typically constitute a subset of the expected range of visual
appearances of the character (or sequence of characters) they are equivalent to.
However, these variant forms may represent a visual distinction that
is significant in some textual contexts, but not in others. As a result,
greater care is required to determine when use of a compatibility
equivalent is appropriate.
If the visual distinction is stylistic,
then markup or styling could be used to represent the formatting
information. However, some characters with compatibility decompositions are
used in mathematical notation to represent a distinction of a semantic nature;
replacing the use of distinct character codes by formatting in
such contexts may cause problems. *Figure 2*
provides examples of compatibility equivalence.

<a id="Compatibility_Equivalence_Figure"></a>

Figure 2. [Examples of Compatibility Equivalence](002-1-1-canonical-and-compatibility-equivalence.md#Compatibility_Equivalence_Figure)

<table class="subtle">
<tr>
<th>Subtype</th>
<th colspan="3">Examples</th>
</tr>
<tr>
<td rowspan="2">Font variants</td>
<td style="text-align: center"><span class="charSample">ℌ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">H</span></td>
</tr>
<tr>
<td style="text-align: center"><span class="charSample">ℍ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">H</span></td>
</tr>
<tr>
<td>Linebreaking differences</td>
<td style="text-align: center"><span class="charSample">[NBSP]</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">[SPACE]</span></td>
</tr>
<tr>
<td rowspan="4">Positional variant forms</td>
<td style="text-align: center"><span class="charSample">ﻉ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">‌ع‌</span></td>
</tr>
<tr>
<td style="text-align: center"><span class="charSample">ﻊ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">‌ع‌</span></td>
</tr>
<tr>
<td style="text-align: center"><span class="charSample">ﻋ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">‌ع‌</span></td>
</tr>
<tr>
<td style="text-align: center"><span class="charSample">ﻌ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">‌ع‌</span></td>
</tr>
<tr>
<td>Circled variants</td>
<td style="text-align: center"><span class="charSample">①</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">1</span></td>
</tr>
<tr>
<td>Width variants</td>
<td style="text-align: center"><span class="charSample">ｶ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">カ</span></td>
</tr>
<tr>
<td rowspan="2">Rotated variants</td>
<td style="text-align: center"><span class="charSample">︷</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">{</span></td>
</tr>
<tr>
<td style="text-align: center"><span class="charSample">︸</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">}</span></td>
</tr>
<tr>
<td rowspan="2">Superscripts/subscripts</td>
<td style="text-align: center"><span class="charSample">i⁹</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">i9</span></td>
</tr>
<tr>
<td style="text-align: center"><span class="charSample">i₉</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">i9</span></td>
</tr>
<tr>
<td>Squared characters</td>
<td style="text-align: center"><span class="charSample">㌀</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">アパート</span></td>
</tr>
<tr>
<td>Fractions</td>
<td style="text-align: center"><span class="charSample">¼</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">1/4</span></td>
</tr>
<tr>
<td>Other</td>
<td style="text-align: center"><span class="charSample">ǆ</span></td>
<td class="hide-side-borders" style="text-align: center"><span class="charSample">→</span></td>
<td style="text-align: center"><span class="charSample">dž</span></td>
</tr>
</table>

Both canonical and compatibility equivalences are explained in more detail in
*Chapter 2, General Structure*, and *Chapter 3, Conformance,*
in [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

