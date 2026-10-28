# 6.5 Tailoring

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-6-implementation-notes.md) · [Next](013-8-1-standard-korean-syllables.md)

<a id="Tailoring"></a>

### 6.5  [Tailoring](012-6-5-tailoring.md#Tailoring)

Rule-based implementation can also be combined with a
code-based or table-based tailoring mechanism. For typical state
machine implementations, for example, a Unicode character is
typically passed to a mapping table that maps characters to boundary
property values. This mapping can use an efficient mechanism such as
a trie. Once a boundary property value is produced, it is passed to
the state machine.

The simplest customization is to adjust the values coming out
of the character mapping table. For example, to mark the appropriate
quotation marks for a given language as having the sentence boundary
property value Close, artificial property values can be introduced
for different quotation marks. A table can be applied after the main
mapping table to map those artificial character property values to
the real ones. To change languages, a different small table is
substituted. The only real cost is then an extra array lookup.

For code-based tailoring a different special range of property
values can be added. The state machine is set up so that any special
property value causes the state machine to halt and return a
particular exception value. When this exception value is detected,
the higher-level process can call specialized code according to
whatever the exceptional value is. This can all be encapsulated so
that it is transparent to the caller.

For example, Thai characters can be mapped to a special
property value. When the state machine halts for one of these values,
then a Thai word break implementation is invoked internally, to
produce boundaries within the subsequent string of Thai characters.
These boundaries can then be cached so that subsequent calls for next
or previous boundaries merely return the cached values. Similarly Lao
characters can be mapped to a different special property value,
causing a different implementation to be invoked.

<a id="Testing"></a>

## 7  [Testing](012-6-5-tailoring.md#Testing)

There is no requirement that Unicode-conformant implementations
implement these default boundaries. As with the other default
specifications, implementations are also free to override (tailor)
the results to meet the requirements of different environments or
particular languages. For those who do implement the default
boundaries as specified in this annex, and wish to check that that
their implementation matches that specification, three test files
have been made available in [[Tests29](https://www.unicode.org/reports/tr41/tr41-36.html#Tests29)].

These tests cannot be exhaustive, because of the large number
of possible combinations; but they do provide samples that test all
pairs of property values, using a representative character for each
value, plus certain other sequences.

A sample HTML file is also available for each that shows various
combinations in chart form, in [[Charts29](https://www.unicode.org/reports/tr41/tr41-36.html#Charts29)]. The header cells
of the chart show the property value.
The body cells in the chart show
the  *break status*: whether a break occurs between the row
property value and the column property value. If the browser supports
tool-tips, then hovering the mouse over a header cell
will show a sample character,
plus its abbreviated general category and script.
Hovering over the break status will display the
number of the rule responsible for that status.

> **Note:** Testing two adjacent
> characters is insufficient for determining a boundary.

The chart may be followed by some test cases. These test cases
consist of various strings with the break status between each pair of
characters shown by blue lines for breaks and by whitespace for
non-breaks. Hovering over each character (with tool-tips enabled)
shows the character name and property value; hovering over the break
status shows the number of the rule responsible for that status.

Due to the way they have been mechanically processed for
generation, the test rules do not match the rules in this annex
precisely. In particular:

1. The rules are cast into a more regex-style.
2. The rules “sot ÷”, “÷ eot”, and “÷ Any” are added
   mechanically and have artificial numbers.
3. The rules are given decimal numbers without prefix, so rules
   such as WB13a are given a number using tenths, such as 13.1.
4. Where a rule has multiple parts (lines), each one is
   numbered using hundredths, such as
   - 21.01) × $BA
   - 21.02) × $HY
   - ...
5. Any “treat as” or “ignore” rules are handled as discussed in
   this annex, and thus reflected in a transformation of the rules not
   visible in the tests.

The mapping from the rule numbering in this annex to the numbering
for the test rules is summarized in *[Table 5](012-6-5-tailoring.md#Table_Numbering_of_Rules).*

<a id="Table_Numbering_of_Rules"></a>

Table 5. [Numbering of Rules](012-6-5-tailoring.md#Table_Numbering_of_Rules)

<table class="subtle">
<tr>
<th>Rule in This Annex</th>
<th>Test Rule</th>
<th>Comment</th>
</tr>
<tr>
<td>xx1</td>
<td>0.2</td>
<td>sot (start of text)</td>
</tr>
<tr>
<td>xx2</td>
<td>0.3</td>
<td>eot (end of text)</td>
</tr>
<tr>
<td>SB8a</td>
<td>8.1</td>
<td rowspan="3" style="vertical-align: middle">Letter style</td>
</tr>
<tr>
<td>WB13a</td>
<td>13.1</td>
</tr>
<tr>
<td>WB13b</td>
<td>13.2</td>
</tr>
<tr>
<td>GB999</td>
<td rowspan="2" style="vertical-align: middle">999.0</td>
<td rowspan="2" style="vertical-align: middle">Any</td>
</tr>
<tr>
<td>WB999</td>
</tr>
</table>

> **Note:** Rule numbers may change
> between versions of this annex.

<a id="Hangul_Syllable_Boundary_Determination"></a>

## 8  [Hangul Syllable Boundary Determination](012-6-5-tailoring.md#Hangul_Syllable_Boundary_Determination)

In rendering, a sequence of jamos is displayed as a series of
syllable blocks. The following rules specify how to divide up an
arbitrary sequence of jamos (including nonstandard sequences) into
these syllable blocks. The symbols L, V, T, LV, LVT represent the
corresponding Hangul\_Syllable\_Type property values; the symbol M for
combining marks.

The precomposed Hangul syllables are of two types: LV or LVT.
In determining the syllable boundaries, the LV behave as if they were
a sequence of jamo L V, and the LVT behave as if they were a sequence
of jamo L V T.

Within any sequence of characters, a syllable break never occurs
between the pairs of characters shown in [*Table 6*](012-6-5-tailoring.md#Hangul_Syllable_No_Break_Rules). In all
cases other than those shown in *Table 6*, a syllable break
occurs before and after any jamo or precomposed Hangul syllable. As
for other characters, any combining mark between two conjoining jamos
prevents the jamos from forming a syllable block.

<a id="Hangul_Syllable_No_Break_Rules"></a>

Table 6. [Hangul Syllable No-Break
Rules](012-6-5-tailoring.md#Hangul_Syllable_No_Break_Rules)

<table class="subtle">
<tr>
<th colspan="2" style="text-align: center">Do Not Break
						Between</th>
<th>Examples</th>
</tr>
<tr>
<td valign="top">L</td>
<td valign="top">L, V, LV or LVT</td>
<td valign="top">L × L<br/> L × V<br/> L × LV<br/>
						L × LVT
					</td>
</tr>
<tr>
<td valign="top">V or LV</td>
<td valign="top">V or T</td>
<td valign="top">V × V<br/> V × T<br/> LV × V<br/>
						LV × T
					</td>
</tr>
<tr>
<td valign="top">T or LVT</td>
<td valign="top">T</td>
<td valign="top">T × T<br/> LVT × T
					</td>
</tr>
<tr>
<td valign="top">Jamo, LV or LVT</td>
<td valign="top">Combining marks</td>
<td valign="top">L × M<br/> V × M<br/> T × M<br/>
						LV × M<br/> LVT × M
					</td>
</tr>
</table>

Even in Normalization Form NFC, a syllable block may contain a
precomposed Hangul syllable in the middle. An example is L LVT T.
Each well-formed modern Hangul syllable, however, can be represented
in the form L V T? (that is one L, one V and optionally one T) and
consists of a single encoded character in NFC.

For information on the behavior of Hangul compatibility jamos in
syllables, see *Section 18.6, Hangul* of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)].

