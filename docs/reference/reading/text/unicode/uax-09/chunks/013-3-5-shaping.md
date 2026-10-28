# 3.5 Shaping

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-3-3-6-resolving-implicit-levels.md) · [Next](014-4-1-boundary-neutrals.md)

<a id="Shaping"></a>

### 3.5  [Shaping](013-3-5-shaping.md#Shaping)

Cursively connected scripts, such as Arabic or
Syriac, require the selection of positional character shapes that depend on
adjacent characters (see *Section 9.2, Arabic* of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)]).
Shaping is logically applied *after* Rule [I2](012-3-3-6-resolving-implicit-levels.md#I2) of the Bidirectional Algorithm
and is limited to characters within the same level run.
(Note that there is no practical difference between limiting shaping to a level run
and an isolating run sequence because the isolate initiator and PDI characters are defined to have joining type U, i.e. non-joining.
Thus, the characters before and after a directional isolate will not join across the isolate,
even if the isolate is empty or overflows the depth limit.)
Consider the following example string
of Arabic characters, which is represented in memory as characters 1, 2, 3, and 4, and where the first two characters
are overridden to be LTR. To show both paragraph directions, the next two are embedded, but with
the normal RTL direction.

<table class="simple">
<tr>
<td style="text-align: center"><b>1</b></td>
<td style="text-align: center"><b>2</b></td>
<td style="text-align: center"><b><i>3</i></b></td>
<td style="text-align: center"><b><i>4</i></b></td>
</tr>
<tr>
<td style="text-align: center"><font size="6">ج</font><br/>
<tt>062C<br/>
          JEEM</tt></td>
<td style="text-align: center"><font size="6">ع</font><br/>
<tt>0639<br/>
          AIN</tt></td>
<td style="text-align: center"><font size="6">ل</font><br/>
<tt>0644<br/>
          LAM</tt></td>
<td style="text-align: center"><font size="6">م</font><br/>
<tt>0645<br/>
          MEEM</tt></td>
</tr>
<tr>
<td>
<p align="center"><b>L</b></p></td>
<td>
<p align="center"><b>L</b></p></td>
<td>
<p align="center"><b>R</b></p></td>
<td>
<p align="center"><b>R</b></p></td>
</tr>
</table>

One can use explicit directional formatting characters to achieve this effect in plain text or use markup in HTML, as in the
examples below. (The **bold** text would be for the right-to-left paragraph direction.)

- LRM/**RLM** LRO *JEEM AIN* PDF RLO *LAM MEEM* PDF
- <p dir="ltr"/"**rtl**">LRO *JEEM AIN* PDF RLO *LAM MEEM* PDF</p>
- <p dir="ltr"/"**rtl**"><bdo dir="ltr">*JEEM AIN*</bdo>  
                <bdo dir="rtl">*LAM MEEM*</bdo></p>

The resulting shapes will be the following, according to the paragraph direction:

<table class="subtle">
<tr>
<th style="text-align: center" width="50%">Left-Right Paragraph</th>
<th style="text-align: center" width="50%">Right-Left Paragraph</th>
</tr>
<tr>
<td width="50%">
<table class="simple">
<tr>
<td style="text-align: center"><b>1</b></td>
<td style="text-align: center"><b>2</b></td>
<td style="text-align: center"><i><b>4</b></i></td>
<td style="text-align: center"><i><b>3</b></i></td>
</tr>
<tr>
<td style="text-align: center">
<font size="6">ﺞ</font><tt><br/>JEEM-F</tt></td>
<td style="text-align: center">
<font size="6">ﻋ</font><tt><br/>AIN-I</tt></td>
<td style="text-align: center">
<font size="6">ﻢ</font><tt><br/>MEEM-F</tt></td>
<td style="text-align: center">
<font size="6">ﻟ</font><tt><br/>LAM-I</tt></td>
</tr>
</table>
</td>
<td width="50%">
<table class="simple">
<tr>
<td style="text-align: center">
<i><b>4</b></i></td>
<td style="text-align: center">
<i><b>3</b></i></td>
<td style="text-align: center">
<b>1</b></td>
<td style="text-align: center">
<b>2</b></td>
</tr>
<tr>
<td style="text-align: center">
<font size="6">ﻢ</font><tt><br/>MEEM-F</tt></td>
<td style="text-align: center">
<font size="6">ﻟ</font><tt><br/>LAM-I</tt></td>
<td style="text-align: center">
<font size="6">ﺞ</font><tt><br/>JEEM-F</tt></td>
<td style="text-align: center">
<font size="6">ﻋ</font><tt><br/>AIN-I</tt></td>
</tr>
</table>
</td>
</tr>
</table>

<a id="Shaping_and_line_breaking"></a>

### 3.5.1  [Shaping and Line Breaking](013-3-5-shaping.md#Shaping_and_line_breaking)

The process of breaking a paragraph into one or
more lines that fit within particular bounds is outside the scope of the
Bidirectional Algorithm. Where character shaping is involved, the width
calculations must be based on the shaped glyphs.

Note that the *soft-hyphen* (SHY) works in
cursively connected scripts as it does in other scripts. That is, it
indicates a point where the line could be broken in the middle of a word. If
the rendering system breaks at that point, the display—including shaping—should be what is appropriate for the given language. For more information
on this and other line breaking issues, see Unicode Standard Annex #14, “Line Breaking Properties” [[UAX14](https://www.unicode.org/reports/tr41/tr41-36.html#UAX14)].

<a id="Bidirectional_Conformance"></a>
<a id="Conformance"></a>

## 4  [Bidirectional Conformance](013-3-5-shaping.md#Bidirectional_Conformance)

The Bidirectional Algorithm specifies part of the intrinsic semantics of right-to-left
characters and is thus required for conformance to the Unicode Standard where any such
characters are displayed.

A process that claims conformance to this specification shall satisfy the following
clauses:

<a id="C1"></a>

***[UAX9-C1](013-3-5-shaping.md#C1).**
In the absence of a permissible higher-level protocol, a
process that renders text shall display all visible representations of characters (excluding
formatting characters) in the order described by Section 3,
[Basic Display Algorithm](004-3-basic-display-algorithm.md#Basic_Display_Algorithm), of this annex. In particular, this includes definitions
[BD1](004-3-basic-display-algorithm.md#BD1)–[BD16](006-3-1-3-paired-brackets.md#BD16) and steps*  *[P1](008-3-3-1-the-paragraph-level.md#P1)–[P3](008-3-3-1-the-paragraph-level.md#P3),
[X1](008-3-3-1-the-paragraph-level.md#X1)–[X10](010-3-3-3-preparations-for-implicit-processing.md#X10), [W1](011-3-3-4-resolving-weak-types.md#W1)–[W7](011-3-3-4-resolving-weak-types.md#W7),
[N0](011-3-3-4-resolving-weak-types.md#N0)–[N2](011-3-3-4-resolving-weak-types.md#N2), [I1](012-3-3-6-resolving-implicit-levels.md#I1)–[I2](012-3-3-6-resolving-implicit-levels.md#I2), and
[L1](012-3-3-6-resolving-implicit-levels.md#L1)–[L4](012-3-3-6-resolving-implicit-levels.md#L4).*

As is the case for all other Unicode algorithms, this is a *logical* description—particular implementations can have more efficient mechanisms as long as they produce the same
results. See C18 in *Chapter 3, Conformance* of [[Unicode](https://www.unicode.org/reports/tr41/tr41-36.html#Unicode)], and the notes following.

<a id="C2"></a>

***[UAX9-C2](013-3-5-shaping.md#C2).**
The only permissible higher-level protocols are those
listed in Section 4.3,
[Higher-Level Protocols](014-4-1-boundary-neutrals.md#Higher-Level_Protocols). They are [HL1](014-4-1-boundary-neutrals.md#HL1), [HL2](014-4-1-boundary-neutrals.md#HL2),
[HL3](014-4-1-boundary-neutrals.md#HL3), [HL4](014-4-1-boundary-neutrals.md#HL4), [HL5](014-4-1-boundary-neutrals.md#HL5), and [HL6](014-4-1-boundary-neutrals.md#HL6).*

> **Note:** The use of higher-level protocols introduces interchange problems,
> since the text may be displayed differently as plain text; see Section 6.5,
> [*Conversion to Plain Text*](018-6-4-separating-punctuation-marks.md#Conversion_to_Plain_Text).
> This can have security implications. Higher-level protocols are recommended
> wherever the semantics of segment order are more
> significant than those of displayed order, as is the case for source text. For detailed examples for which use of HL4 would be recommended, see Section 4.3.1,
> [*HL4 Example 1 for XML*](014-4-1-boundary-neutrals.md#HL4Example1) and Section 4.3.2,
> [*HL4 Example 2 for Program Text*](015-4-3-2-hl4-example-2-for-program-text.md#HL4Example2).
> For more information, see Section 4.1, *Bidirectional Ordering*, in Unicode
> Technical Standard #55, “Unicode Source Code Handling”
> [[UTS55](https://www.unicode.org/reports/tr41/tr41-36.html#UTS55)], as well as Unicode Technical Report #36, “Unicode Security
> Considerations” [[UTR36](https://www.unicode.org/reports/tr41/tr41-36.html#UTR36)].

