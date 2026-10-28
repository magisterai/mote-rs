# Continuation 10

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-4-2-name-validation.md) · [Next](011-6-implementation-notes.md)

<a id="SB1"></a>

<a id="SB2"></a>

<a id="SB3"></a>

<a id="SB4"></a>

<a id="SB5"></a>

<a id="SB6"></a>

<a id="SB7"></a>

<a id="SB8"></a>

<a id="SB8a"></a>

<a id="SB9"></a>

<a id="SB10"></a>

<a id="SB11"></a>

<a id="SB998"></a>

<table class="subtle-nb loose">
<tr>
<td class="rule" colspan="4">Break at the start and end of
					text, unless the text is empty.</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB1" name="SB1">SB1</a></td>
<td style="text-align: right">sot</td>
<td style="text-align: center">÷</td>
<td>Any</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB2" name="SB2">SB2</a></td>
<td style="text-align: right">Any</td>
<td style="text-align: center">÷</td>
<td>eot</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within CRLF.</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB3" name="SB3">SB3</a></td>
<td style="text-align: right">CR</td>
<td style="text-align: center">×</td>
<td>LF</td>
</tr>
<tr>
<td class="rule" colspan="4">Break after paragraph separators.</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB4" name="SB4">SB4</a></td>
<td style="text-align: right"><b>ParaSep</b></td>
<td style="text-align: center">÷</td>
<td> </td>
</tr>
<tr>
<td class="rule" colspan="4">Ignore Format and Extend
					characters, except after sot, <b>ParaSep</b>, and within CRLF. (See
					Section 6.2, <a href="011-6-implementation-notes.md#Grapheme_Cluster_and_Format_Rules">Replacing
						Ignore Rules</a>.) This also has the effect of: Any × (Format |
					Extend)
				</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB5" name="SB5">SB5</a></td>
<td style="text-align: right">X (Extend | Format)*</td>
<td style="text-align: center">→</td>
<td>X</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break after full stop in
					certain contexts. [See note below.]</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB6" name="SB6">SB6</a></td>
<td style="text-align: right">ATerm</td>
<td style="text-align: center">×</td>
<td>Numeric</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB7" name="SB7">SB7</a></td>
<td style="text-align: right">(Upper | Lower) ATerm</td>
<td style="text-align: center">×</td>
<td>Upper</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB8" name="SB8">SB8</a></td>
<td style="text-align: right">ATerm Close* Sp*</td>
<td style="text-align: center">×</td>
<td>( ¬(OLetter | Upper | Lower | <b>ParaSep</b> | <b>SATerm</b>)
					)* Lower
				</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB8a" name="SB8a">SB8a</a></td>
<td style="text-align: right"><b>SATerm</b> Close* Sp*</td>
<td style="text-align: center">×</td>
<td>(SContinue | <b>SATerm</b>)
				</td>
</tr>
<tr>
<td class="rule" colspan="4">Break after sentence terminators,
					but include closing punctuation, trailing spaces, and any paragraph
					separator. [See note below.]</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB9" name="SB9">SB9</a></td>
<td style="text-align: right"><b>SATerm</b> Close*</td>
<td style="text-align: center">×</td>
<td>(Close | Sp | <b>ParaSep</b>)
				</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB10" name="SB10">SB10</a></td>
<td style="text-align: right"><b>SATerm</b> Close* Sp*</td>
<td style="text-align: center">×</td>
<td>(Sp | <b>ParaSep</b>)
				</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB11" name="SB11">SB11</a></td>
<td style="text-align: right"><b>SATerm</b> Close* Sp* <b>ParaSep</b>?</td>
<td style="text-align: center">÷</td>
<td> </td>
</tr>
<tr>
<td class="rule" colspan="4">Otherwise, do not break.</td>
</tr>
<tr>
<td><a href="010-continuation-10.md#SB998" name="SB998">SB998</a></td>
<td style="text-align: right">Any</td>
<td style="text-align: center">×</td>
<td>Any</td>
</tr>
</table>

> **Notes:**
>
> - Rules [SB6](010-continuation-10.md#SB6)–[SB8](010-continuation-10.md#SB8) are
>   designed to forbid breaks after ambiguous terminators (primarily
>   U+002E FULL STOP) within strings such as those shown in [*Figure 3*](010-continuation-10.md#ForbiddenSB). The contexts which forbid
>   breaks include occurrence directly before a number, between
>   uppercase letters, when followed by a lowercase letter (optionally
>   after certain punctuation), or when followed by certain continuation
>   punctuation such as a comma, colon, or semicolon. These rules permit
>   breaks in strings such as those shown in [*Figure
>   4*](010-continuation-10.md#AllowedSB). They cannot detect cases such as “...Mr. Jones...”; more
>   sophisticated tailoring would be required to detect such cases.
> - Rules [SB9](010-continuation-10.md#SB9)–[SB11](010-continuation-10.md#SB11) are
>   designed to allow breaks after sequences of the following form, but
>   not within them:
>   - (STerm | ATerm) Close\* Sp\* (Sep | CR | LF)?
> - Note that in unusual cases, a word segment (determined
>   according to *Section 4 [Word
>   Boundaries](005-4-word-boundaries.md#Word_Boundaries)*) may span a sentence break (according to *Section
>   5 [Sentence Boundaries](009-4-2-name-validation.md#Sentence_Boundaries)*). Inconsistencies between word and sentence boundaries can be
>   reduced by customizing [SB11](010-continuation-10.md#SB11) to take account of
>   whether a period is followed by a character from a script that does
>   not normally require spaces between words.
> - Users can run experiments in an interactive [online demo](https://util.unicode.org/UnicodeJsps/breaks.jsp) to
>   observe default word and sentence boundaries in a given piece of
>   text.

<a id="ForbiddenSB"></a>

Figure 3. [Forbidden
Breaks on “.”](010-continuation-10.md#ForbiddenSB)

<table class="simple">
<tr>
<td style="text-align: right; padding-right: 0px">c.</td>
<td style="text-align: left; padding-left: 0px">d</td>
</tr>
<tr>
<td style="text-align: right; padding-right: 0px">3.</td>
<td style="text-align: left; padding-left: 0px">4</td>
</tr>
<tr>
<td style="text-align: right; padding-right: 0px">U.</td>
<td style="text-align: left; padding-left: 0px">S.</td>
</tr>
<tr>
<td style="text-align: right; padding-right: 0px">... the
						resp.</td>
<td style="text-align: left; padding-left: 0px"> leaders
						are ...</td>
</tr>
<tr>
<td style="text-align: right; padding-right: 0px">...
						etc.)’ </td>
<td style="text-align: left; padding-left: 0px">‘(the ...</td>
</tr>
</table>

<a id="AllowedSB"></a>

Figure 4. [Allowed Breaks on
“.”](010-continuation-10.md#AllowedSB)

<table class="simple">
<tr>
<td style="text-align: right">She said “See spot run.”</td>
<td style="text-align: left"> John shook his head. ...</td>
</tr>
<tr>
<td style="text-align: right">... etc.</td>
<td style="text-align: left">它们指...</td>
</tr>
<tr>
<td style="text-align: right">...理数字.</td>
<td style="text-align: left">它们指...</td>
</tr>
</table>

