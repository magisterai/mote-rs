# 3.3.6 Resolving Implicit Levels

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-3-3-4-resolving-weak-types.md) · [Next](013-3-5-shaping.md)

<a id="Resolving_Implicit_Levels"></a>

### 3.3.6  [Resolving Implicit Levels](012-3-3-6-resolving-implicit-levels.md#Resolving_Implicit_Levels)

In the final phase, the embedding level of text may be increased, based on the resolved
character type. Right-to-left text will always end up with an odd level, and left-to-right and
numeric text will always end up with an even level. In addition, numeric text will always end up
with a higher level than the paragraph level. (Note that it is possible for text to end up at
level max\_depth+1 as a result of this process.) This results in the following rules:

<a id="I1"></a>

***[I1](012-3-3-6-resolving-implicit-levels.md#I1).** For all characters with an even (left-to-right) embedding
level, those of type R go up one level and those of type AN or EN go up two levels.*

<a id="I2"></a>

***[I2](012-3-3-6-resolving-implicit-levels.md#I2).** For all characters with an odd (right-to-left)
embedding level, those of type L, EN or AN go up one level.*

*Table 5* summarizes the results of the implicit algorithm.

<a id="Table_Resolving_Implicit_Levels"></a>

Table 5. [Resolving Implicit Levels](012-3-3-6-resolving-implicit-levels.md#Table_Resolving_Implicit_Levels)

<table class="subtle">
<tr>
<th rowspan="2">Type</th>
<th colspan="2">Embedding Level</th>
</tr>
<tr>
<th>Even</th>
<th>Odd</th>
</tr>
<tr>
<td><b>L</b></td>
<td>EL</td>
<td>EL+1</td>
</tr>
<tr>
<td><b>R</b></td>
<td>EL+1</td>
<td>EL</td>
</tr>
<tr>
<td><b>AN</b></td>
<td>EL+2</td>
<td>EL+1</td>
</tr>
<tr>
<td><b>EN</b></td>
<td>EL+2</td>
<td>EL+1</td>
</tr>
</table>

<a id="Reordering_Resolved_Levels"></a>

### 3.4  [Reordering Resolved Levels](012-3-3-6-resolving-implicit-levels.md#Reordering_Resolved_Levels)

The following rules describe the logical process of finding the correct display order.
As opposed to resolution phases,
these rules act on a per-line basis *and are applied **after** any line wrapping is
applied to the paragraph.*

Logically there are the following steps:

- The levels of the text are determined according to the previous rules.
- The characters are shaped into glyphs according to their context *(taking the embedding
  levels into account for mirroring).*
- The accumulated widths of those glyphs *(in logical order)* are used to determine line
  breaks.
- For each line, rules [L1](012-3-3-6-resolving-implicit-levels.md#L1)–[L4](012-3-3-6-resolving-implicit-levels.md#L4) are used to reorder the characters on that line.
- The glyphs corresponding to the characters on the line are displayed in that order.

<a id="L1"></a>

***[L1](012-3-3-6-resolving-implicit-levels.md#L1).** On each line, reset the embedding level of the following characters to
the paragraph embedding level:*

1. *Segment separators,*
2. *Paragraph separators,*
3. *Any sequence of whitespace characters and/or isolate formatting characters (FSI, LRI, RLI, and PDI)
   preceding a segment separator or paragraph separator, and*
4. *Any sequence of whitespace characters and/or isolate formatting characters (FSI, LRI, RLI, and PDI)
   at the end of the line.*

- The types of characters used here are the *original* types, not those modified by the
  previous phase.
- Because a *paragraph separator* breaks lines, there will be at most one per line, at the end of
  that line.

In combination with the following rule, this means that trailing whitespace will appear at the
visual end of the line (in the paragraph direction). Tabulation will always have a consistent
direction within a paragraph.

<a id="L2"></a>

***[L2](012-3-3-6-resolving-implicit-levels.md#L2).** From the highest level found in the text to the lowest odd level on
each line, including intermediate levels not actually present in the text, reverse
any contiguous sequence of characters that are at that level or higher.*

This rule reverses a progressively larger series of substrings.

The following examples illustrate the reordering, showing the
successive steps in application of Rule [L2](012-3-3-6-resolving-implicit-levels.md#L2).
The original text is shown in the "Storage" row in the
example tables.
The invisible, zero-width formatting characters LRI, RLI, and PDI
are represented with the symbols **>**,
**<**, and **=**, respectively.
The application of the rules from *Section 3.3, [Resolving Embedding Levels](007-3-3-resolving-embedding-levels.md#Resolving_Embedding_Levels)*
and of the Rule
[L1](012-3-3-6-resolving-implicit-levels.md#L1) results in the resolved levels listed in the "Resolved
Levels" row.
(Since these examples only make use of the isolate formatting characters, Rule [X9](010-3-3-3-preparations-for-implicit-processing.md#X9) does not remove any characters.
Note that Example 3 would not work if it used embeddings instead because the two right-to-left phrases would have merged into a single right-to-left run,
together with the neutral punctuation in between.)
Each successive row thereafter shows one pass of reversal from Rule [L2](012-3-3-6-resolving-implicit-levels.md#L2), such as
"Reverse levels 1-2". At each iteration, the underlining shows the text that has been reversed.  
  
The paragraph embedding level for the first, second, and third examples is 0 (left-to-right direction),
and for the fourth example is 1 (right-to-left direction).

**Example 1**. (embedding level = 0)

<table class="subtle-nb">
<tr>
<th style="vertical-align: middle">Storage</th>
<td style="vertical-align: middle"><code>car means CAR.</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Resolved levels     </th>
<td style="vertical-align: middle"><code>00000000001110</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse level 1</th>
<td style="vertical-align: middle"><code>car means <u>RAC</u>.</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Display</th>
<td style="vertical-align: middle">car means RAC.</td>
</tr>
</table>

**Example 2**. (embedding level = 0)

<table class="subtle-nb">
<tr>
<th style="vertical-align: middle">Storage</th>
<td style="vertical-align: middle"><code>&lt;car MEANS CAR.=</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Resolved levels</th>
<td style="vertical-align: middle"><code>0222111111111110</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse level 2</th>
<td style="vertical-align: middle"><code>&lt;<u>rac</u> MEANS CAR.=</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse levels 1-2</th>
<td style="vertical-align: middle"><code>&lt;<u>.RAC SNAEM car</u>=</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Display</th>
<td style="vertical-align: middle">.RAC SNAEM car</td>
</tr>
</table>

**Example 3**. (embedding level = 0)

<table class="subtle-nb">
<tr>
<th style="vertical-align: middle">Storage</th>
<td style="vertical-align: middle"><code>he said “&lt;car MEANS CAR=.” “&lt;IT DOES=,” she agreed.</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Resolved levels</th>
<td style="vertical-align: middle"><code>000000000022211111111110000001111111000000000000000</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse level 2</th>
<td style="vertical-align: middle"><code>he said “&lt;<u>rac</u> MEANS CAR=.” “&lt;IT DOES=,” she agreed.</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse levels 1-2</th>
<td style="vertical-align: middle"><code>he said “&lt;<u>RAC SNAEM car</u>=.” “&lt;<u>SEOD TI</u>=,” she agreed.</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Display</th>
<td style="vertical-align: middle">he said “RAC SNAEM car.” “SEOD TI,” she agreed.</td>
</tr>
</table>

**Example 4**. (embedding level = 1)

<table class="subtle-nb">
<tr>
<th style="vertical-align: middle">Storage</th>
<td style="vertical-align: middle"><code>DID YOU SAY ’&gt;he said “&lt;car MEANS CAR=”=‘?</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Resolved levels</th>
<td style="vertical-align: middle"><code>111111111111112222222222444333333333322111</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse level 4</th>
<td style="vertical-align: middle"><code>DID YOU SAY ’&gt;he said “&lt;<u>rac</u> MEANS CAR=”=‘?</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse levels 3-4</th>
<td style="vertical-align: middle"><code>DID YOU SAY ’&gt;he said “&lt;<u>RAC SNAEM car</u>=”=‘?</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse levels 2-4</th>
<td style="vertical-align: middle"><code>DID YOU SAY ’&gt;<u>”=rac MEANS CAR&lt;“ dias eh</u>=‘?</code></td>
</tr>
<tr>
<th style="vertical-align: middle">Reverse levels 1-4</th>
<td style="vertical-align: middle"><code><u>?‘=he said “&lt;RAC SNAEM car=”&gt;’ YAS UOY DID</u></code></td>
</tr>
<tr>
<th style="vertical-align: middle">Display</th>
<td style="vertical-align: middle">?‘he said “RAC SNAEM car”’ YAS UOY DID</td>
</tr>
</table>

  


<a id="L3"></a>

***[L3](012-3-3-6-resolving-implicit-levels.md#L3).** Combining marks applied to a right-to-left base character will at this
point precede their base character. If the rendering engine expects them to follow the base
characters in the final display process, then the ordering of the marks and the base character
must be reversed.*

Many font designers provide default metrics for combining marks that support rendering by
simple overhang. Because of the reordering for right-to-left characters, it is common practice to
make the glyphs for most combining characters overhang to the left (thus assuming the characters
will be applied to left-to-right base characters) and make the glyphs for combining characters in
right-to-left scripts overhang to the right (thus assuming that the characters will be applied to
right-to-left base characters). With such fonts, the display ordering of the marks and base glyphs
may need to be adjusted when combining marks are applied to “unmatching” base characters. See *Section 5.13, Rendering Nonspacing Marks* of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)], for more
information.

<a id="L4"></a>

***[L4](012-3-3-6-resolving-implicit-levels.md#L4).** A character is
depicted by a mirrored glyph if and only if (a)
the resolved directionality of that character is R,
and (b) the Bidi\_Mirrored
property value of that character is Yes.*

- *The Bidi\_Mirrored property
  is defined by Section 4.7, Bidi Mirrored of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)];
  the property values are specified in [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)].*
- *This rule can be overridden in certain
  cases; see
  [HL6](014-4-1-boundary-neutrals.md#HL6).*

For example, U+0028 LEFT PARENTHESIS—which is interpreted in the Unicode
Standard as an opening parenthesis—appears as “**(**” when its resolved
level is even, and as the mirrored glyph “**)**”
when its resolved level is odd. Note that for
backward compatibility the characters U+FD3E ( ﴾ ) ORNATE LEFT PARENTHESIS
and U+FD3F ( ﴿ ) ORNATE RIGHT PARENTHESIS are not mirrored.

