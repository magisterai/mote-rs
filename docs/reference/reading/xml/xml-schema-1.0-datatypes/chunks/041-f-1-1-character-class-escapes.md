# F.1.1 Character Class Escapes

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](040-f-1-character-classes.md) · [Next](042-continuation-42.md)

For any [·positive character group·](040-f-1-character-classes.md#dt-poschargroup) or
[·negative character group·](040-f-1-character-classes.md#dt-negchargroup) *G*, and any
[·character class expression·](040-f-1-character-classes.md#dt-charexpr) *C*, *G-C* is a valid
[·character class subtraction·](040-f-1-character-classes.md#dt-subchargroup), identifying the set of all characters in
*C(G)* that are not also in *C(C)*.

<a id="dt-charrange"></a>

[Definition:]
A
**character range** *R* identifies a set of
characters *C(R)* containing all XML characters with UCS
code points in a specified range.

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Character Range</font></strong></td></tr><tr><td><a id="charrange"></a><a id="nt-charRange"></a><a id="serange"></a><a id="nt-seRange"></a><a id="charoresc"></a><a id="nt-charOrEsc"></a><a id="xmlchar"></a><a id="nt-XmlChar"></a><a id="XmlCharIncDash"></a><a id="nt-XmlCharIncDash"></a><table border="0"><tbody><tr valign="baseline"><td><a id="charrange" name="charrange"></a><a id="nt-charRange" name="nt-charRange"></a>[17]   </td><td><code>charRange</code></td><td>   ::=   </td><td><code>
<a href="041-f-1-1-character-class-escapes.md#nt-seRange">seRange</a> |



			
			<a href="041-f-1-1-character-class-escapes.md#nt-XmlCharIncDash">XmlCharIncDash</a>
</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="serange" name="serange"></a><a id="nt-seRange" name="nt-seRange"></a>[18]   </td><td><code>seRange</code></td><td>   ::=   </td><td><code><a href="041-f-1-1-character-class-escapes.md#nt-charOrEsc">charOrEsc</a> '-' <a href="041-f-1-1-character-class-escapes.md#nt-charOrEsc">charOrEsc</a></code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="charoresc" name="charoresc"></a><a id="nt-charOrEsc" name="nt-charOrEsc"></a>[20]   </td><td><code>charOrEsc</code></td><td>   ::=   </td><td><code><a href="041-f-1-1-character-class-escapes.md#nt-XmlChar">XmlChar</a> | <a href="041-f-1-1-character-class-escapes.md#nt-SingleCharEsc">SingleCharEsc</a></code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="xmlchar" name="xmlchar"></a><a id="nt-XmlChar" name="nt-XmlChar"></a>[21]   </td><td><code>XmlChar</code></td><td>   ::=   </td><td><code>[^\#x2D#x5B#x5D]</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="XmlCharIncDash" name="XmlCharIncDash"></a><a id="nt-XmlCharIncDash" name="nt-XmlCharIncDash"></a>[22]   </td><td><code>XmlCharIncDash</code></td><td>   ::=   </td><td><code>[^\#x5B#x5D]</code></td></tr></tbody></table></td></tr></tbody></table>

A single XML character is a [·character range·](041-f-1-1-character-class-escapes.md#dt-charrange) that identifies
the set of characters containing only itself. All XML characters are valid
character ranges, except as follows:

- The `[`, `]`, `-` and `\` characters are not
  valid character ranges;
- The `^` character is only valid at the beginning of a
  [·positive character group·](040-f-1-character-classes.md#dt-poschargroup) if it is part of a
  [·negative character group·](040-f-1-character-classes.md#dt-negchargroup)
- The `-` character is a valid character range only at the beginning
  or end of a [·positive character group·](040-f-1-character-classes.md#dt-poschargroup).

**Note:**The grammar for [·character range·](041-f-1-1-character-class-escapes.md#dt-charrange) as
given above is ambiguous, but the second and third bullets above
together remove the ambiguity.

A [·character range·](041-f-1-1-character-class-escapes.md#dt-charrange) [·may·](003-1-4-terminology.md#dt-may) also be written
in the form *s-e*, identifying the set that contains all XML characters
with UCS code points greater than or equal to the code point
of *s*, but not greater than the code point of *e*.

*s-e* is a valid character range iff:

- *s* is a [·single character escape·](041-f-1-1-character-class-escapes.md#dt-cces1), or an XML character;
- *s* is not `\`
- If s is the first character in a [·character class expression·](040-f-1-character-classes.md#dt-charexpr), then
  *s* is not `^`
- *e* is a [·single character escape·](041-f-1-1-character-class-escapes.md#dt-cces1), or an XML character;
- *e* is not `\` or `[`; and
- The code point of *e* is greater than or equal to the code
  point of *s*;

**Note:**
The code point of a [·single character escape·](041-f-1-1-character-class-escapes.md#dt-cces1) is the code point of the
single character in the set of characters that it identifies.

<a id="cces"></a>

#### F.1.1 Character Class Escapes

<a id="dt-cces"></a>

[Definition:]
A **character class escape** is a short sequence of characters
that identifies predefined character class. The valid character
class escapes are the [·single character escape·](041-f-1-1-character-class-escapes.md#dt-cces1)s, the
[·multi-character escape·](042-continuation-42.md#dt-ccesN)s, and the [·category escape·](041-f-1-1-character-class-escapes.md#dt-ccescat)s (including
the [·block escape·](042-continuation-42.md#dt-ccesblock)s).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Character Class Escape</font></strong></td></tr><tr><td><a id="charclassesc"></a><a id="nt-charClassEsc"></a><table border="0"><tbody><tr valign="baseline"><td><a id="charclassesc" name="charclassesc"></a><a id="nt-charClassEsc" name="nt-charClassEsc"></a>[23]   </td><td><code>charClassEsc</code></td><td>   ::=   </td><td><code>
			(
			<a href="041-f-1-1-character-class-escapes.md#nt-SingleCharEsc">SingleCharEsc</a> |
			<a href="042-continuation-42.md#nt-MultiCharEsc">MultiCharEsc</a> |
			<a href="041-f-1-1-character-class-escapes.md#nt-catEsc">catEsc</a> |
			<a href="041-f-1-1-character-class-escapes.md#nt-complEsc">complEsc</a>
			)
		</code></td></tr></tbody></table></td></tr></tbody></table>

<a id="dt-cces1"></a>

[Definition:]
A
**single character escape** identifies a set containing a only
one character -- usually because that character is difficult or
impossible to write directly into a [·regular expression·](039-e-1-algorithm.md#dt-regex).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Single Character Escape</font></strong></td></tr><tr><td><a id="singlecharesc"></a><a id="nt-SingleCharEsc"></a><table border="0"><tbody><tr valign="baseline"><td><a id="singlecharesc" name="singlecharesc"></a><a id="nt-SingleCharEsc" name="nt-SingleCharEsc"></a>[24]   </td><td><code>SingleCharEsc</code></td><td>   ::=   </td><td><code>'\' [nrt\|.?*+(){}#x2D#x5B#x5D#x5E]</code></td></tr></tbody></table></td></tr></tbody></table>

| The valid [·single character escape·](041-f-1-1-character-class-escapes.md#dt-cces1)s are: | Identifying the set of characters *C(R)* containing: |
| --- | --- |
| `\n` | the newline character (#xA) |
| `\r` | the return character (#xD) |
| `\t` | the tab character (#x9) |
| `\\` | \ |
| `\|` | | |
| `\.` | . |
| `\-` | - |
| `\^` | ^ |
| `\?` | ? |
| `\*` | \* |
| `\+` | + |
| `\{` | { |
| `\}` | } |
| `\(` | ( |
| `\)` | ) |
| `\[` | [ |
| `\]` | ] |

<a id="dt-ccescat"></a>

[Definition:]
[[Unicode Database]](044-h-references.md#UnicodeDB) specifies a number of possible
values for the "General Category" property
and provides mappings from code points to specific character properties.
The set containing all characters that have property `X`,
can be identified with a **category escape** `\p{X}`.
The complement of this set is specified with the
**category escape** `\P{X}`.
(`[\P{X}]` = `[^\p{X}]`).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Category Escape</font></strong></td></tr><tr><td><a id="catesc"></a><a id="nt-catEsc"></a><a id="complesc"></a><a id="nt-complEsc"></a><a id="charprop"></a><a id="nt-charProp"></a><table border="0"><tbody><tr valign="baseline"><td><a id="catesc" name="catesc"></a><a id="nt-catEsc" name="nt-catEsc"></a>[25]   </td><td><code>catEsc</code></td><td>   ::=   </td><td><code>'\p{' <a href="041-f-1-1-character-class-escapes.md#nt-charProp">charProp</a> '}'</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="complesc" name="complesc"></a><a id="nt-complEsc" name="nt-complEsc"></a>[26]   </td><td><code>complEsc</code></td><td>   ::=   </td><td><code>'\P{' <a href="041-f-1-1-character-class-escapes.md#nt-charProp">charProp</a> '}'</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="charprop" name="charprop"></a><a id="nt-charProp" name="nt-charProp"></a>[27]   </td><td><code>charProp</code></td><td>   ::=   </td><td><code><a href="042-continuation-42.md#nt-IsCategory">IsCategory</a> | <a href="042-continuation-42.md#nt-IsBlock">IsBlock</a></code></td></tr></tbody></table></td></tr></tbody></table>

**Note:**
[[Unicode Database]](044-h-references.md#UnicodeDB) is subject to future revision. For example, the
mapping from code points to character properties might be updated.
All [·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors [·must·](003-1-4-terminology.md#dt-must)
support the character properties defined in the version of [[Unicode Database]](044-h-references.md#UnicodeDB)
that is current at the time this specification became a W3C
Recommendation. However, implementors are encouraged to support the
character properties defined in any future version.

The following table specifies the recognized values of the
"General Category" property.

<table align="center" border="1"><tbody><tr><th>Category</th><th>Property</th><th>Meaning</th></tr><tr><td rowspan="6">Letters</td><td align="center">L</td><td>All Letters</td></tr><tr><td align="center">Lu</td><td>uppercase</td></tr><tr><td align="center">Ll</td><td>lowercase</td></tr><tr><td align="center">Lt</td><td>titlecase</td></tr><tr><td align="center">Lm</td><td>modifier</td></tr><tr><td align="center">Lo</td><td>other</td></tr><tr><td colspan="3"> </td></tr><tr><td rowspan="4">Marks</td><td align="center">M</td><td>All Marks</td></tr><tr><td align="center">Mn</td><td>nonspacing</td></tr><tr><td align="center">Mc</td><td>spacing combining</td></tr><tr><td align="center">Me</td><td>enclosing</td></tr><tr><td colspan="3"> </td></tr><tr><td rowspan="4">Numbers</td><td align="center">N</td><td>All Numbers</td></tr><tr><td align="center">Nd</td><td>decimal digit</td></tr><tr><td align="center">Nl</td><td>letter</td></tr><tr><td align="center">No</td><td>other</td></tr><tr><td colspan="3"> </td></tr><tr><td rowspan="8">Punctuation</td><td align="center">P</td><td>All Punctuation</td></tr><tr><td align="center">Pc</td><td>connector</td></tr><tr><td align="center">Pd</td><td>dash</td></tr><tr><td align="center">Ps</td><td>open</td></tr><tr><td align="center">Pe</td><td>close</td></tr><tr><td align="center">Pi</td><td>initial quote
			(may behave like Ps or Pe depending on usage)</td></tr><tr><td align="center">Pf</td><td>final quote
			(may behave like Ps or Pe depending on usage)</td></tr><tr><td align="center">Po</td><td>other</td></tr><tr><td colspan="3"> </td></tr><tr><td rowspan="4">Separators</td><td align="center">Z</td><td>All Separators</td></tr><tr><td align="center">Zs</td><td>space</td></tr><tr><td align="center">Zl</td><td>line</td></tr><tr><td align="center">Zp</td><td>paragraph</td></tr><tr><td colspan="3"> </td></tr><tr><td rowspan="5">Symbols</td><td align="center">S</td><td>All Symbols</td></tr><tr><td align="center">Sm</td><td>math</td></tr><tr><td align="center">Sc</td><td>currency</td></tr><tr><td align="center">Sk</td><td>modifier</td></tr><tr><td align="center">So</td><td>other</td></tr><tr><td colspan="3"> </td></tr><tr><td rowspan="6">Other</td><td align="center">C</td><td>All Others</td></tr><tr><td align="center">Cc</td><td>control</td></tr><tr><td align="center">Cf</td><td>format</td></tr><tr><td align="center">Co</td><td>private use</td></tr><tr><td align="center">Cn</td><td>not assigned</td></tr></tbody></table>

