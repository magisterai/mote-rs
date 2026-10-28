# Continuation 7

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-4-1-default-word-boundary-specification.md) · [Next](008-continuation-8.md)

<a id="WB1"></a>

<a id="WB2"></a>

<a id="WB3"></a>

<a id="WB3a"></a>

<a id="WB3b"></a>

<a id="WB3c"></a>

<a id="WB3d"></a>

<a id="WB4"></a>

<a id="WB5"></a>

<a id="WB6"></a>

<a id="WB7"></a>

<a id="WB7a"></a>

<a id="WB7b"></a>

<a id="WB7c"></a>

<a id="WB8"></a>

<a id="WB9"></a>

<a id="WB10"></a>

<a id="WB11"></a>

<a id="WB12"></a>

<a id="WB13"></a>

<a id="WB13a"></a>

<a id="WB13b"></a>

<a id="WB15"></a>

<a id="WB16"></a>

<a id="WB999"></a>

<table class="subtle-nb loose rules">
<tr>
<td class="rule" colspan="4">Break at the start and end of
					text, unless the text is empty.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB1" name="WB1">WB1</a></td>
<td style="text-align: right">sot</td>
<td style="text-align: center">÷</td>
<td>Any</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB2" name="WB2">WB2</a></td>
<td style="text-align: right">Any</td>
<td style="text-align: center">÷</td>
<td>eot</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within CRLF.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB3" name="WB3">WB3</a></td>
<td style="text-align: right">CR</td>
<td style="text-align: center">×</td>
<td>LF</td>
</tr>
<tr>
<td class="rule" colspan="4">Otherwise break before and after
					Newlines (including CR and LF)</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB3a" name="WB3a">WB3a</a></td>
<td style="text-align: right">(Newline | CR | LF)</td>
<td style="text-align: center">÷</td>
<td> </td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB3b" name="WB3b">WB3b</a></td>
<td style="text-align: right"> </td>
<td style="text-align: center">÷</td>
<td>(Newline | CR | LF)</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within emoji zwj
					sequences.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB3c" name="WB3c">WB3c</a></td>
<td style="text-align: right">ZWJ</td>
<td style="text-align: center">×</td>
<td>\p{Extended_Pictographic}</td>
</tr>
<tr>
<td class="rule" colspan="4">Keep horizontal whitespace
					together.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB3d" name="WB3d">WB3d</a></td>
<td style="text-align: right">WSegSpace</td>
<td style="text-align: center">×</td>
<td>WSegSpace</td>
</tr>
<tr>
<td class="rule" colspan="4">Ignore Format and Extend
					characters, except after sot, CR, LF, and Newline. (See Section
					6.2, <a href="011-6-implementation-notes.md#Grapheme_Cluster_and_Format_Rules">Replacing
						Ignore Rules</a>.) This also has the effect of: Any × (Format | Extend
					| ZWJ)
				</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB4" name="WB4">WB4</a></td>
<td style="text-align: right">X (Extend | Format | ZWJ)*</td>
<td style="text-align: center">→</td>
<td>X</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break between most letters.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB5" name="WB5">WB5</a></td>
<td style="text-align: right"><b>AHLetter</b></td>
<td style="text-align: center">×</td>
<td><b>AHLetter</b></td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break letters across
					certain punctuation, such as within “e.g.” or “example.com”.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB6" name="WB6">WB6</a></td>
<td style="text-align: right"><b>AHLetter</b></td>
<td style="text-align: center">×</td>
<td>(MidLetter | <b>MidNumLetQ</b>) <b>AHLetter</b></td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB7" name="WB7">WB7</a></td>
<td style="text-align: right"><b>AHLetter</b> (MidLetter | <b>MidNumLetQ</b>)</td>
<td style="text-align: center">×</td>
<td><b>AHLetter</b></td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB7a" name="WB7a">WB7a</a></td>
<td style="text-align: right">Hebrew_Letter</td>
<td style="text-align: center">×</td>
<td>Single_Quote</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB7b" name="WB7b">WB7b</a></td>
<td style="text-align: right">Hebrew_Letter</td>
<td style="text-align: center">×</td>
<td>Double_Quote Hebrew_Letter</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB7c" name="WB7c">WB7c</a></td>
<td style="text-align: right">Hebrew_Letter Double_Quote</td>
<td style="text-align: center">×</td>
<td>Hebrew_Letter</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within sequences of
					digits, or digits adjacent to letters (“3a”, or “A3”).</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB8" name="WB8">WB8</a></td>
<td style="text-align: right">Numeric</td>
<td style="text-align: center">×</td>
<td>Numeric</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB9" name="WB9">WB9</a></td>
<td style="text-align: right"><b>AHLetter</b></td>
<td style="text-align: center">×</td>
<td>Numeric</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB10" name="WB10">WB10</a></td>
<td style="text-align: right">Numeric</td>
<td style="text-align: center">×</td>
<td><b>AHLetter</b></td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within sequences,
					such as “3.2” or “3,456.789”.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB11" name="WB11">WB11</a></td>
<td style="text-align: right">Numeric (MidNum | <b>MidNumLetQ</b>)
				</td>
<td style="text-align: center">×</td>
<td>Numeric</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB12" name="WB12">WB12</a></td>
<td style="text-align: right">Numeric</td>
<td style="text-align: center">×</td>
<td>(MidNum | <b>MidNumLetQ</b>) Numeric
				</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break between Katakana.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB13" name="WB13">WB13</a></td>
<td style="text-align: right">Katakana</td>
<td style="text-align: center">×</td>
<td>Katakana</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break from extenders.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB13a" name="WB13a">WB13a</a></td>
<td style="text-align: right">(<b>AHLetter</b> | Numeric |
					Katakana | ExtendNumLet)
				</td>
<td style="text-align: center">×</td>
<td>ExtendNumLet</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB13b" name="WB13b">WB13b</a></td>
<td style="text-align: right">ExtendNumLet</td>
<td style="text-align: center">×</td>
<td>(<b>AHLetter</b> | Numeric | Katakana)
				</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within emoji flag
					sequences. That is, do not break between regional indicator (RI)
					symbols if there is an odd number of RI characters before the break
					point.</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB15" name="WB15">WB15</a></td>
<td style="text-align: right">sot (RI RI)* RI</td>
<td style="text-align: center">×</td>
<td>RI</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB16" name="WB16">WB16</a></td>
<td style="text-align: right">[^RI] (RI RI)* RI</td>
<td style="text-align: center">×</td>
<td>RI</td>
</tr>
<tr>
<td class="rule" colspan="4">Otherwise, break everywhere
					(including around ideographs).</td>
</tr>
<tr>
<td><a href="007-continuation-7.md#WB999" name="WB999">WB999</a></td>
<td style="text-align: right">Any</td>
<td style="text-align: center">÷</td>
<td>Any</td>
</tr>
</table>

