# 2.2 Explicit Directional Overrides

Source: [UAX #9: Unicode Bidirectional Algorithm](https://www.unicode.org/reports/tr9/tr9-51.html) · Unicode 17.0.0, revision 51.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-1-introduction.md) · [Next](004-3-basic-display-algorithm.md)

<a id="Explicit_Directional_Overrides"></a>

### 2.2  [Explicit Directional Overrides](003-2-2-explicit-directional-overrides.md#Explicit_Directional_Overrides)

The following characters allow the bidirectional character types to be overridden when required for
special cases, such as for part numbers.
They are to be avoided wherever possible, because of
security concerns. For more information, see [[UTR36](https://www.unicode.org/reports/tr41/tr41-36.html#UTR36)].
Directional overrides can be nested one inside another, and in embeddings and isolates.

| Abbr. | Code Point | Name | Description |
| --- | --- | --- | --- |
| **LRO** | U+202D | LEFT-TO-RIGHT OVERRIDE | Force following characters to be treated as strong left-to-right characters. |
| **RLO** | U+202E | RIGHT-TO-LEFT OVERRIDE | Force following characters to be treated as strong right-to-left characters. |

The precise meaning of these characters will be made clear in the discussion of the algorithm. The
right-to-left override, for example, can be used to force a part number made of mixed English,
digits and Hebrew letters to be written from right to left.

<a id="Terminating_Explicit_Directional_Embeddings_and_Overrides"></a>
<a id="Terminating_Explicit_Directional_Code"></a>

### 2.3  [Terminating Explicit Directional Embeddings and Overrides](003-2-2-explicit-directional-overrides.md#Terminating_Explicit_Directional_Embeddings_and_Overrides)

The following character terminates the scope of the last
LRE, RLE, LRO, or RLO whose scope has not yet been terminated.

| Abbr. | Code Point | Name | Description |
| --- | --- | --- | --- |
| **PDF** | U+202C | POP DIRECTIONAL FORMATTING | End the scope of the last LRE, RLE, RLO, or LRO. |

The precise meaning of this character will be made clear in the discussion
of the algorithm.

<a id="Explicit_Directional_Isolates"></a>

### 2.4  [Explicit Directional Isolates](003-2-2-explicit-directional-overrides.md#Explicit_Directional_Isolates)

The following characters signal that a piece of text is to be treated as
directionally isolated from its surroundings.
They are very similar to the explicit embedding formatting characters.
However, while an embedding roughly has
the effect of a strong character on the ordering of the surrounding text,
an isolate has the effect of a neutral like U+FFFC OBJECT REPLACEMENT CHARACTER,
and is assigned the corresponding display position in the surrounding text.
Furthermore, the text inside the isolate has no effect on the ordering of the text outside it,
and vice versa.

In addition to allowing the embedding of strongly directional text without unduly affecting
the bidirectional order of its surroundings,
one of the isolate formatting characters also offers an extra feature:
embedding text while inferring its direction heuristically from its constituent characters.

Isolates can be nested one inside another, and in embeddings and overrides.

| Abbr. | Code Point | Name | Description |
| --- | --- | --- | --- |
| **LRI** | U+2066 | LEFT‑TO‑RIGHT ISOLATE | Treat the following text as isolated and left-to-right. |
| **RLI** | U+2067 | RIGHT‑TO‑LEFT ISOLATE | Treat the following text as isolated and right-to-left. |
| **FSI** | U+2068 | FIRST STRONG ISOLATE | Treat the following text as isolated and in the direction of its first strong directional character that is not inside a nested isolate. |

The precise meaning of these characters will be made clear in the discussion of the algorithm.

<a id="Terminating_Explicit_Directional_Isolates"></a>

### 2.5  [Terminating Explicit Directional Isolates](003-2-2-explicit-directional-overrides.md#Terminating_Explicit_Directional_Isolates)

The following character terminates the scope of the last LRI, RLI, or FSI
whose scope has not yet been terminated,
as well as the scopes of any subsequent LREs, RLEs, LROs, or RLOs whose scopes have not yet been terminated.

| Abbr. | Code Point | Name | Description |
| --- | --- | --- | --- |
| **PDI** | U+2069 | POP DIRECTIONAL ISOLATE | End the scope of the last LRI, RLI, or FSI. |

The precise meaning of this character will be made clear in the discussion of the algorithm.

<a id="Implicit_Directional_Marks"></a>

### 2.6  [Implicit Directional Marks](003-2-2-explicit-directional-overrides.md#Implicit_Directional_Marks)

These characters are very light-weight formatting. They act exactly like right-to-left or
left-to-right characters, except that they do not display or have any other semantic effect. Their
use is more convenient than using explicit embeddings or overrides because their scope is
much more local.

| Abbr. | Code Point | Name | Description |
| --- | --- | --- | --- |
| **LRM** | U+200E | LEFT-TO-RIGHT MARK | Left-to-right zero-width character |
| **RLM** | U+200F | RIGHT-TO-LEFT MARK | Right-to-left zero-width non-Arabic character |
| **ALM** | U+061C | ARABIC LETTER MARK | Right-to-left zero-width Arabic character |

There is no special mention of the implicit directional marks in the following algorithm. That
is because their effect on bidirectional ordering is exactly the same as a corresponding strong
directional character; the only difference is that they do not appear in the display.

<a id="Markup_And_Formatting"></a>

### 2.7  [Markup and Formatting Characters](003-2-2-explicit-directional-overrides.md#Markup_And_Formatting)

The explicit formatting characters introduce state into the plain text, which must be maintained when editing or displaying the text. Processes that are modifying the text without being aware of this state may inadvertently affect the rendering of large portions of the text, for example by removing a PDF.

The Unicode Bidirectional Algorithm is designed so that the use of
explicit formatting characters can be equivalently represented by out-of-line information, such as stylesheet
information or markup. Conflicts can arise if markup and explicitly formatting characters are both used in the same paragraph.
Where available, markup should be used instead of the explicit formatting characters: for more information, see [[UnicodeXML](https://www.unicode.org/reports/tr41/tr41-36.html#UnicodeXML)].
However, any alternative representation is only to be defined by reference to the behavior
of the corresponding explicit formatting characters in this algorithm, to ensure conformance with the Unicode Standard.

HTML5 [[HTML5](https://www.unicode.org/reports/tr41/tr41-36.html#HTML5)] and CSS3 [[CSS3Writing](https://www.unicode.org/reports/tr41/tr41-36.html#CSS3Writing)]
provide support for bidi markup as follows:

<table class="subtle">
<tr>
<th>Unicode</th>
<th>Equivalent Markup</th>
<th>Equivalent CSS</th>
<th>Comment</th>
</tr>
<tr>
<td>RLI ... PDI</td>
<td>dir = "rtl"</td>
<td>direction:rtl; <span style="white-space: nowrap;">unicode-bidi:isolate</span></td>
<td>dir attribute on any element</td>
</tr>
<tr>
<td>LRI ... PDI</td>
<td>dir = "ltr"</td>
<td>direction:ltr; <span style="white-space: nowrap;">unicode-bidi:isolate</span></td>
<td>dir attribute on any element</td>
</tr>
<tr>
<td>FSI ... PDI</td>
<td>&lt;bdi&gt;, dir = "auto"</td>
<td><span style="white-space: nowrap;">unicode-bidi:plaintext</span></td>
<td>dir attribute on any element</td>
</tr>
<tr>
<td>RLE ... PDF</td>
<td> </td>
<td>direction:rtl; <span style="white-space: nowrap;">unicode-bidi:embed</span></td>
<td>markup not available in HTML</td>
</tr>
<tr>
<td>LRE ... PDF</td>
<td> </td>
<td>direction:ltr; <span style="white-space: nowrap;">unicode-bidi:embed</span></td>
<td>markup not available in HTML</td>
</tr>
<tr>
<td>RLO ... PDF</td>
<td> </td>
<td>direction:rtl; <span style="white-space: nowrap;">unicode-bidi:bidi-override</span></td>
<td>markup not available in HTML</td>
</tr>
<tr>
<td>LRO ... PDF</td>
<td> </td>
<td>direction:ltr; <span style="white-space: nowrap;">unicode-bidi:bidi-override</span></td>
<td>markup not available in HTML</td>
</tr>
<tr>
<td>FSI RLO . . . PDF PDI</td>
<td>&lt;bdo dir = "rtl"&gt;</td>
<td>direction:rtl; <span style="white-space: nowrap;">unicode-bidi:isolate-override</span></td>
<td> </td>
</tr>
<tr>
<td>FSI LRO . . . PDF PDI</td>
<td>&lt;bdo dir = "ltr"&gt;</td>
<td>direction:ltr; <span style="white-space: nowrap;">unicode-bidi:isolate-override</span></td>
<td> </td>
</tr>
</table>

Unlike HTML4.0, HTML5 does not provide exact equivalents for LRE, RLE, LRO, and RLO, although
the dir attribute and the BDO element as outlined above should in most cases work as well or better than those formatting characters.
When absolutely necessary, CSS can be used to get exact equivalents for LRE, RLE, LRO, and RLO, as well as for LRI, RLI, and FSI.

Whenever plain text is produced from a document containing markup, the equivalent formatting characters should be introduced, so that the correct ordering is not lost. For example, whenever cut and paste results in plain text this transformation should occur.

