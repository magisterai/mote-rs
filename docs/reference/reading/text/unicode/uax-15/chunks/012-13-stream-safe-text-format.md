# 13 Stream-Safe Text Format

Source: [UAX #15: Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-57.html) · Unicode 17.0.0, revision 57.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-11-4-forbidding-characters.md) · [Next](013-14-implementation-notes.md)

<a id="Stream_Safe_Text_Format"></a>

## 13  [Stream-Safe Text Format](012-13-stream-safe-text-format.md#Stream_Safe_Text_Format)

There are certain protocols that would benefit from using normalization, but
that have
implementation constraints. For example, a protocol may require buffered serialization,
in which only a portion of a string may be available at a given time. Consider the
extreme case of a string containing a *digit 2*
followed by 10,000 *umlauts*
followed by one *dot-below*,
then a *digit 3*.
As part of normalization, the *dot-below*
at the end must be reordered to immediately after the *digit 2*,
which means that 10,003 characters need to be considered before
the result can be output.

Such extremely long sequences of combining marks are not illegal, even
though for all practical purposes they are not meaningful. However, the
possibility of encountering such sequences
forces a conformant, serializing implementation to provide large buffer
capacity or to provide a special exception mechanism just for such degenerate
cases. The Stream-Safe Text Format specification addresses this situation.

<a id="UAX15-D3"></a>

***[UAX15-D3.](012-13-stream-safe-text-format.md#UAX15-D3)***
*Stream-Safe Text Format:*
A Unicode string is said to be
in Stream-Safe Text Format
if it would not contain any sequences of non-starters longer than 30
characters in length when normalized to NFKD.

- Such a string can be normalized in buffered serialization with
  a buffer size of 32 characters, which would require no more than 128 bytes
  in any Unicode Encoding Form.
- Incorrect buffer handling can introduce subtle errors in the
  results. Any buffered implementation should be carefully checked against
  the normalization test data.
- The value of 30 is chosen to
  be significantly beyond what is required for any linguistic or technical
  usage. While it would have been feasible to chose a smaller number, this
  value provides a very wide margin, yet is well within the buffer size
  limits of practical implementations.
- NFKD was chosen for the definition because it produces the potentially longest sequences of non-starters from the same text.

<a id="UAX15-D4"></a>

***[UAX15-D4.](012-13-stream-safe-text-format.md#UAX15-D4)***
*Stream-Safe Text Process* is the process of producing a
Unicode string in Stream-Safe Text Format by processing that string from
start to finish, inserting U+034F COMBINING GRAPHEME
JOINER (CGJ) within long sequences of non-starters.
The exact position of the inserted CGJs are determined according to the
following algorithm, which describes the generation of an output string from
an input string:

1. If the input string is
   empty, return an empty output string.
2. Set nonStarterCount to zero.
3. For each code point C in the
   input string:
   1. Produce the NFKD
      decomposition S.
   2. If nonStarterCount plus
      the number of initial non-starters in S is greater than 30, append a
      CGJ to the output string and set the nonStarterCount to zero.
   3. Append C to the output
      string.
   4. If there are no starters in S, increment nonStarterCount by the
      number of code points in S; otherwise, set nonStarterCount to the number of trailing non-starters in S (which
      may be zero).
4. Return the output string.

The Stream-Safe Text Process ensures not only that the resulting text is in Stream-Safe Text Format,
but that any normalization of the result is also in Stream-Safe Text Format.
This is true for any input string that does not contain unassigned code
points. The Stream-Safe Text Process preserves all of the four
normalization forms defined in this annex (NFC, NFD, NFKC, NFKD). However, normalization and
the Stream-Safe Text Process do not commute. That is, normalizing an arbitrary text to NFC,
followed by applying the Stream-Safe Text Process, is not guaranteed to produce the same result
as applying the Stream-Safe Text Process to that arbitrary text, followed by normalization to
NFC.

It is important to realize that if the Stream-Safe Text Process does modify
the input text by insertion of CGJs, the result will *not*
be canonically equivalent to the original. The Stream-Safe Text Format is designed for use in
protocols and systems that accept the limitations on the text imposed by the format,
just as they may impose their own limitations, such as removing certain control
codes.

However, the Stream-Safe Text Format
will not modify ordinary texts. Where it modifies an exceptional text, the resulting string would no
longer be canonically equivalent to the original, but the modifications are
minor and do not disturb any meaningful content. The modified text contains
all of the content of the original, with the only difference being that
reordering is blocked across long groups of non-starters. Any text in Stream-Safe Text Format can be normalized with very small buffers using any of the standard
Normalization Forms.

Implementations can optimize this specification as long as they produce the same results. In particular,
the information used in Step 3 can be precomputed: it does not require the
actual normalization of the character. For efficient processing, the Stream-Safe Text Process can be
implemented in the same implementation pass as normalization. In such a case, the choice of whether to apply the
Stream-Safe Text Process can be controlled by an input parameter.

<a id="Buffering_with_Unicode_Normalization"></a>

### 13.1  [Buffering with Unicode Normalization](012-13-stream-safe-text-format.md#Buffering_with_Unicode_Normalization)

Using buffers for normalization requires that characters be emptied from
the buffer correctly. That is, as decompositions are appended to the
buffer, periodically the end of the buffer will be reached. At that
time, the characters in the buffer up to *but not including* the
last character with the property value Quick\_Check=Yes (QC=Y) must be
canonically ordered (and if NFC and NFKC are being generated, must also
be composed), and only then flushed. For more information on the
Quick\_Check property, see *Section 9 [Detecting Normalization Forms](008-7-design-goals.md#Detecting_Normalization_Forms)*.

Consider the following example. Text is being normalized into NFC
with a buffer size of 40. The buffer has been successively filled with
decompositions, and has two remaining slots. The decomposition takes
three characters, and wouldn't fit. The last character with QC=Y is the
"s", highlighted in color below.

**Buffer**

<table border="0" style="BORDER-COLLAPSE: collapse">
<tr>
<td style="text-align: center" width="30"><font size="5">T</font></td>
<td style="text-align: center" width="30"><font size="5">h</font></td>
<td style="text-align: center" width="30"><font size="5">e</font></td>
<td style="text-align: center" width="30"> </td>
<td style="text-align: center" width="30"><font size="5">c</font></td>
<td style="text-align: center" width="30"><font size="5">◌́</font></td>
<td style="text-align: center" width="30"><font size="5">a</font></td>
<td style="text-align: center" width="60"><font size="5">...</font></td>
<td style="text-align: center" width="30"><font size="5">p</font></td>
<td style="text-align: center" width="30"><font size="5">◌̃</font></td>
<td style="text-align: center" width="30"><font size="5">q</font></td>
<td style="text-align: center" width="30"><font size="5">r</font></td>
<td style="text-align: center" width="30"><font size="5">◌́</font></td>
<td style="text-align: center; background-color:#FF8060" width="30"><font size="5">s</font></td>
<td style="text-align: center" width="30"><font size="5">◌́</font></td>
<td style="text-align: center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align: center; background-color:#c0c0c0" width="30"> </td>
</tr>
<tr>
<td style="text-align: center" width="30">0</td>
<td style="text-align: center" width="30">1</td>
<td style="text-align: center" width="30">2</td>
<td style="text-align: center" width="30">3</td>
<td style="text-align: center" width="30">4</td>
<td style="text-align: center" width="30">5</td>
<td style="text-align: center" width="30">6</td>
<td style="text-align: center" width="60">...</td>
<td style="text-align: center" width="30">31</td>
<td style="text-align: center" width="30">32</td>
<td style="text-align: center" width="30">33</td>
<td style="text-align: center" width="30">34</td>
<td style="text-align: center" width="30">35</td>
<td style="text-align: center" width="30">36</td>
<td style="text-align: center" width="30">37</td>
<td style="text-align: center" width="30">38</td>
<td style="text-align: center" width="30">39</td>
</tr>
</table>

**Decomposition**

<table border="0" style="BORDER-COLLAPSE: collapse">
<tr>
<td style="text-align: center" width="30"><font size="5">u</font></td>
<td style="text-align: center" width="30"><font size="5">◌̃ </font></td>
<td style="text-align: center" width="30"><font size="5">◌́</font></td>
</tr>
<tr>
<td style="text-align: center" width="30">0</td>
<td style="text-align: center" width="30">1</td>
<td style="text-align: center" width="30">2</td>
</tr>
</table>

Thus the buffer up to but not including "s" needs to be composed, and
flushed. Once this is done, the decomposition can be appended, and the
buffer is left in the following state:

<table border="0" style="BORDER-COLLAPSE: collapse">
<tr>
<td style="text-align:center" width="30"><font size="5">s</font></td>
<td style="text-align:center" width="30"><font size="5">◌́</font></td>
<td style="text-align:center" width="30"><font size="5">u</font></td>
<td style="text-align:center" width="30"><font size="5">◌̃ </font></td>
<td style="text-align:center" width="30"><font size="5">◌́</font></td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="30"> </td>
<td style="text-align:center; background-color:#c0c0c0" width="60"><font size="5">...</font></td>
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

Implementations may also canonically order (and compose) the contents
of the buffer as they go; the key requirement is that they cannot
compose a sequence until a following character with the property QC=Y is
encountered. For example, if that had been done in the above example,
then during the course of filling the buffer, we would have had the
following state, where "c" is the last character with QC=Y.

