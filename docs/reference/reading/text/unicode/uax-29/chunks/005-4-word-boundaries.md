# 4 Word Boundaries

Source: [UAX #29: Unicode Text Segmentation](https://www.unicode.org/reports/tr29/tr29-47.html) · Unicode 17.0.0, revision 47.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-3-1-default-grapheme-cluster-boundary-specification.md) · [Next](006-4-1-default-word-boundary-specification.md)

<a id="GB1"></a>

<a id="GB2"></a>

<a id="GB3"></a>

<a id="GB4"></a>

<a id="GB5"></a>

<a id="GB6"></a>

<a id="GB7"></a>

<a id="GB8"></a>

<a id="GB9"></a>

<a id="GB9a"></a>

<a id="GB9b"></a>

<a id="GB9c"></a>

<a id="GB11"></a>

<a id="GB12"></a>

<a id="GB13"></a>

<a id="GB999"></a>

<table class="subtle-nb loose rules">
<tr>
<td class="rule" colspan="4">Break at the start and end of
					text, unless the text is empty.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB1" name="GB1">GB1</a></td>
<td style="text-align: right">sot</td>
<td style="text-align: center">÷</td>
<td>Any</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB2" name="GB2">GB2</a></td>
<td style="text-align: right">Any</td>
<td style="text-align: center">÷</td>
<td>eot</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break between a CR and LF.
					Otherwise, break before and after controls.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB3" name="GB3">GB3</a></td>
<td style="text-align: right">CR</td>
<td style="text-align: center">×</td>
<td>LF</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB4" name="GB4">GB4</a></td>
<td style="text-align: right">(Control | CR | LF)</td>
<td style="text-align: center">÷</td>
<td> </td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB5" name="GB5">GB5</a></td>
<td style="text-align: right"></td>
<td style="text-align: center">÷</td>
<td>(Control | CR | LF)</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break Hangul syllable or other conjoining
					sequences.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB6" name="GB6">GB6</a></td>
<td style="text-align: right">L</td>
<td style="text-align: center">×</td>
<td>(L | V | LV | LVT)</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB7" name="GB7">GB7</a></td>
<td style="text-align: right">(LV | V)</td>
<td style="text-align: center">×</td>
<td>(V | T)</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB8" name="GB8">GB8</a></td>
<td style="text-align: right">(LVT | T)</td>
<td style="text-align: center">×</td>
<td>T</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break before extending
					characters or ZWJ.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB9" name="GB9">GB9</a></td>
<td style="text-align: right"> </td>
<td style="text-align: center">×</td>
<td>(Extend | ZWJ)
				</td>
</tr>
<tr>
<td class="rule" colspan="4"><b>The <a href="005-4-word-boundaries.md#GB9a">GB9a</a> and <a href="005-4-word-boundaries.md#GB9b">GB9b</a> rules only apply to extended grapheme
						clusters:
				</b><br/>
				Do not break before SpacingMarks, or after Prepend
					characters.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB9a" name="GB9a">GB9a</a></td>
<td style="text-align: right"> </td>
<td style="text-align: center">×</td>
<td>SpacingMark</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB9b" name="GB9b">GB9b</a></td>
<td style="text-align: right">Prepend</td>
<td style="text-align: center">×</td>
<td> </td>
</tr>
<tr>
<td class="rule" colspan="4">
<b>The <a href="005-4-word-boundaries.md#GB9c">GB9c</a> rule only applies to extended grapheme clusters:</b><br/>
				Do not break within certain combinations with Indic_Conjunct_Break (InCB)=Linker.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB9c" name="GB9c">GB9c</a></td>
<td style="text-align: right">\p{InCB=Consonant} [ \p{InCB=Extend} \p{InCB=Linker} ]* \p{InCB=Linker} [ \p{InCB=Extend} \p{InCB=Linker} ]*</td>
<td style="text-align: center">×</td>
<td>\p{InCB=Consonant}</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within emoji modifier
			  sequences or emoji zwj sequences.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB11" name="GB11">GB11</a></td>
<td style="text-align: right">\p{Extended_Pictographic}
			  Extend* ZWJ</td>
<td style="text-align: center">×</td>
<td>\p{Extended_Pictographic}</td>
</tr>
<tr>
<td class="rule" colspan="4">Do not break within emoji flag
					sequences. That is, do not break between regional indicator (RI)
					symbols if there is an odd number of RI characters before the break
			  point.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB12" name="GB12">GB12</a></td>
<td style="text-align: right">sot (RI RI)* RI</td>
<td style="text-align: center">×</td>
<td>RI</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB13" name="GB13">GB13</a></td>
<td style="text-align: right">[^RI] (RI RI)* RI</td>
<td style="text-align: center">×</td>
<td>RI</td>
</tr>
<tr>
<td class="rule" colspan="4">Otherwise, break everywhere.</td>
</tr>
<tr>
<td><a href="005-4-word-boundaries.md#GB999" name="GB999">GB999</a></td>
<td style="text-align: right">Any</td>
<td style="text-align: center">÷</td>
<td>Any</td>
</tr>
</table>

> **Notes:**
>
> - Grapheme cluster boundaries can be transformed into simple
>   regular expressions. For more information, see *Section 6.3, [State Machines](011-6-implementation-notes.md#State_Machines)*
>   and *Table 1c, [Regex Definitions](004-3-1-default-grapheme-cluster-boundary-specification.md#Regex_Definitions)*.
> - The Grapheme\_Base and Grapheme\_Extend properties predated
>   the development of the Grapheme\_Cluster\_Break property. The set of
>   characters with Grapheme\_Extend=Yes is used to derive the set of
>   characters with Grapheme\_Cluster\_Break=Extend. However, the
>   Grapheme\_Base property proved to be insufficient for determining
>   grapheme cluster boundaries. Grapheme\_Base is no longer used by this
>   specification.
> - Each *emoji sequence* is a single grapheme cluster. See definition ED-17 in Unicode Technical Standard #51, "Unicode Emoji" [[UAX51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)].
> - Similar to Jamo clustering into Hangul Syllables,
>   other characters bind tightly into grapheme clusters, that, unlike
>   combining characters, don't depend on a base character.
>   These characters are said to exhibit *conjoining behavior*.
>   For the purpose of Grapheme\_Cluster\_Break, the property value V has been
>   extended beyond characters of Hangul\_Syllable\_Type=V to cover them.

<a id="Word_Boundaries"></a>

## 4  [Word Boundaries](005-4-word-boundaries.md#Word_Boundaries)

Word boundaries are used in a number of different contexts. The
most familiar ones are selection (double-click mouse selection), cursor movement
(“move to next word” control-arrow keys), and the dialog option “Whole
Word Search” for search and replace. They are also used in database
queries, to determine whether elements are within a certain number of
words of one another. Searching may also use word boundaries in
determining matching items. Word boundaries are not restricted to
whitespace and punctuation. Indeed, some languages do not use spaces
at all.

*Figure 1* gives an example of word boundaries, marked in the
sample text with vertical bars. In the following discussion, search
terms are indicated by enclosing them in square brackets for clarity.
Spaces are indicated with the open-box symbol “␣”, and the matching
parts between the search terms and target text are emphasized in
color.

<a id="Figure_Word_Boundaries"></a>

Figure 1. [Word Boundaries](005-4-word-boundaries.md#Figure_Word_Boundaries)

|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| The |  | quick |  | ( | “ | brown | ” | ) |  | fox |  | can’t |  | jump |  | 32.3 |  | feet | , |  | right | ? |

Boundaries such as those flanking the words in *Figure 1* are
the boundaries that users would expect, for example, when searching
for a term in the target text using Whole Word Search mode. In that
mode there is a match if—in addition to a matching sequence of
characters—there are word boundaries in the target text on both sides
of the search term. In the sample target text in *Figure 1*,
Whole Word Search would have results such as the following:

- The search term [brown] matches
  because there are word boundaries on both sides.
- The search term [brow] does not
  match because there is no word boundary in the target text between
  ‘w’ and the following character, ‘n’.
- The term [“brown”] matches
  because there are word boundaries between the quotation marks and
  the parentheses that enclose them.
- The term [(“brown”)] also
  matches because there are word boundaries between the parentheses
  and the space characters around them.
- Finally, the term [␣(“brown”)␣]
  with spaces included matches as well, because there are word
  boundaries between the space characters and the letters immediately
  before and after them in the target text.

To allow for such matches that users would expect, there are
word breaks by default between most characters that are not normally
considered parts of words, such as punctuation and spaces.

Word boundaries can also be used in intelligent cut and paste.
With this feature, if the user cuts a selection of text on word
boundaries, adjacent spaces are collapsed to a single space. For
example, cutting “quick” from “The␣quick␣fox” would leave
“The␣ ␣fox”. Intelligent cut and paste collapses this text to
“The␣fox”. However, spaces need to be handled separately: cutting the
center space from “The␣ ␣ ␣fox” probably should not
collapse the remaining two spaces to one.

Proximity tests in searching determines whether, for example, “quick”
is within three words of “fox”. That is done with the above
boundaries by ignoring any words that contain only whitespace, punctuation, and similar characters, as in *[Figure 2](005-4-word-boundaries.md#Figure_Extracted_Words)*. Thus, for
proximity, “fox” is within three words of “quick”. This same
technique can be used for “get next/previous word” commands or
keyboard arrow keys. Letters are not the only characters that can be
used to determine the “significant” words; different implementations
may include other types of characters such as digits or perform other
analysis of the characters.

<a id="Figure_Extracted_Words"></a>

Figure 2. [Extracted Words](005-4-word-boundaries.md#Figure_Extracted_Words)

|  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| The | quick | brown | fox | can’t | jump | 32.3 | feet | right |

As with the other default specifications, implementations may
override (tailor) the results to meet the requirements of different
environments or particular languages. For some languages, it may also
be necessary to have different tailored word break rules for
selection versus Whole Word Search.

Whether the default word boundary detection described here is
adequate, and whether word boundaries are related to line breaks, varies
between scripts. The style of context analysis in line breaking (see
[[UAX14](https://www.unicode.org/reports/tr41/tr41-36.html#UAX14),
section 3.1]) used for a script can provide some rough guidance:

- For scripts that use the Western style of context analysis, default
  word boundaries and default line breaks are usually adequate. A default
  line boundary break opportunity is usually a default word boundary,
  but there are exceptions such as a word containing a SHY (soft hyphen):
  it will break across lines, yet is a single word. Tailorings may find
  additional line break opportunities within words due to hyphenation.
  Scripts in this group include Latin, Arabic, Devanagari, and many others;
  they can be identified by having letters with line break class AL.
- For scripts that use the East Asian or Brahmic styles of context
  analysis, the default word boundary detection is not adequate; it
  needs tailoring. The default line breaks, on the other hand, are
  usually adequate. Word boundaries are irrelevant to line breaking.
  Scripts in this group include Chinese, Japanese, Brahmi, Javanese,
  and others; they can be identified by having letters with line break
  class ID, AK, or AS.
- For scripts that use the South East Asian style of context analysis,
  neither the default word boundaries nor the default line breaks are
  adequate. Both need tailoring. The reason is that line breaks should
  only occur at word boundaries, but there’s no demarcation of words.
  Scripts in this group include Thai, Myanmar, Khmer, and others; they
  can be identified by having letters with line break class SA.

Hangul is treated as part of the first group for default
word boundary detection; and as part of the second group for default line breaking.
Some scripts may be treated as being part of the first group only because not
enough information is available for them.

