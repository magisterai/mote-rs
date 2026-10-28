# 4.1 Relation to the Terms “Fullwidth” and “Halfwidth”

Source: [UAX #11: East Asian Width](https://www.unicode.org/reports/tr11/tr11-44.html) · Unicode 17.0.0, revision 44.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://www.unicode.org/versions/Unicode17.0.0/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-2-scope.md) · [Next](004-6-classifications.md)

<a id="Relation"></a>

### 4.1  [Relation to the Terms “Fullwidth” and “Halfwidth”](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#Relation)

When converting a DBCS mixed-width encoding to and from Unicode, the fullwidth characters in such a mixed-width encoding are mapped to the fullwidth compatibility characters in the FFxx block, whereas the corresponding halfwidth characters are mapped to ordinary Unicode characters (for example, ASCII in U+0021..U+007E, plus a few other scattered characters).

In the context of interoperability with DBCS character encodings, this restricted set of Unicode characters in the General Scripts area can be construed as halfwidth, rather than fullwidth. (This applies only to the restricted set of characters that can be paired with the fullwidth compatibility characters.)

In the context of interoperability with DBCS character encodings, all other Unicode characters that are not explicitly marked as halfwidth can be construed as fullwidth.

In any other context, Unicode characters not explicitly marked as being either fullwidth or halfwidth compatibility forms are neither halfwidth nor fullwidth.

Seen in this light, the “halfwidth” and “fullwidth” properties are not unitary character properties in the same sense as “space” or “combining” or “alphabetic.” They are, instead, relational properties of a pair of characters, one of which is explicitly encoded as a halfwidth or fullwidth form for compatibility in mapping to DBCS mixed-width character encodings.

What is “fullwidth” by default today could in theory become “halfwidth” tomorrow by the introduction of another character on the SBCS part of a mixed-width code page somewhere, requiring the introduction of another fullwidth compatibility character to complete the mapping. However, because the single byte part of mixed-width character sets is limited, there are not going to be many candidates and neither the Unicode Technical Committee [[UTC](https://www.unicode.org/reports/tr41/tr41-36.html#UTC)] nor WG2 has any intention to encode additional compatibility characters for this purpose.

<a id="Ambiguous"></a>

### 4.2  [Ambiguous Characters](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#Ambiguous)

Ambiguous width characters originated from characters that can occur as fullwidth characters in any of a number of East Asian legacy character encodings. They have a “resolved” width of either narrow or wide depending on the context of their use. As originally defined, the width of ambiguous characters resolves to narrow, if they are not used in the context of the specific legacy encoding to which they belong. Otherwise, it resolves to fullwidth or halfwidth. The term *context* as used here includes extra information such as explicit markup, knowledge of the source code page, font information, or language and script identification. For example:

- Greek characters resolve to narrow when used with a standard Greek font, because there is no East Asian legacy context.
- Private-use character codes and the replacement character have ambiguous width, because they may stand in for characters of any width.
- Ambiguous quotation marks are generally resolved to wide when they enclose and are adjacent to a wide character, and to narrow otherwise.

The East\_Asian\_Width property does not preserve canonical equivalence, because the base characters of canonical decompositions almost always have a different East\_Asian\_Width than the precomposed characters. The East\_Asian\_Width property is designed for use with legacy character sets so the property value is not designed to respect canonical equivalence.

***Modern Rendering Practice.*** Modern practice is evolving toward rendering ever more of the ambiguous characters with proportionally spaced, narrow forms that rotate with the direction of writing, independent of their treatment in one or more legacy character sets. In other words, context information beyond the choice of font or source character set is employed to resolve the width of the character. This annex does not attempt to track such changes in practice; therefore, the set of characters with mappings to legacy character sets that have been assigned ambiguous width constitute a different set than the set of such characters that may be rendered as wide characters in a given context. In particular, an application might find it useful to treat characters from alphabetic scripts as narrow by default. Conversely, many of the symbols in the Unicode Standard have no mappings to legacy character sets, yet they may be rendered as “wide” characters if they appear in an East Asian context. An implementation might therefore elect to treat them as ambiguous even though they are classified as *neutral* here.

<a id="VariationSequences"></a>

### 4.3  [Variation Sequences](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#VariationSequences)

In some cases, variation sequences have been defined that request a character to be displayed as wide or narrow. These variation sequences are defined in StandardizedVariants.txt [[StandardizedVariants](https://www.unicode.org/reports/tr41/tr41-36.html#StandardizedVariants)] in the [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)], but they are identified via the description of the intended glyph style and not via a formal property of the variation sequence. Nevertheless, the presence of the respective variation selector should be considered a sufficient condition to resolve ambiguous width to either wide or narrow, perhaps subject only to availability in the actual font used. If the font does not have a wide form, a variation sequence requesting the wide form would be ignorable and it makes no sense to treat the character as “Wide” if it is being displayed as “Narrow.”

<a id="Recommendations"></a>

## 5  [Recommendations](003-4-1-relation-to-the-terms-fullwidth-and-halfwidth.md#Recommendations)

*When mapping Unicode to East Asian legacy character encodings*

- Wide Unicode characters *always* map to fullwidth characters.
- Narrow (and neutral) Unicode characters *always* map to halfwidth characters.
- Halfwidth Unicode characters *always* map to halfwidth characters.
- Ambiguous Unicode characters *always* map to fullwidth characters.

*When mapping Unicode to non–East Asian legacy character encodings*

- Wide Unicode characters *do not* map to *non*–East Asian legacy character encodings.
- Narrow (and neutral) Unicode characters *always* map to regular (narrow) characters.
- Halfwidth Unicode characters *do not* map.
- Ambiguous Unicode characters *always* map to regular (narrow) characters.

*When processing or displaying data*

- Wide characters behave like ideographs in important ways, such as layout. Except for certain punctuation characters, they are not rotated when appearing in vertical text runs. In fixed-pitch fonts, they take up one Em of space.
- Halfwidth characters behave like ideographs in some ways, however, they are rotated like narrow characters when appearing in vertical text runs. In fixed-pitch fonts, they take up 1/2 Em of space.
- Narrow characters behave like Western characters, for example, in line breaking. They are rotated sideways, when appearing in vertical text. In fixed-pitch East Asian fonts, they take up 1/2 Em of space, but in rendering, a non–East Asian, proportional font is often substituted.
- Ambiguous characters behave like wide or narrow characters depending on the context (language tag, script identification, associated font, source of data, or explicit markup; all can provide the context). If the context cannot be established reliably, they should be treated as narrow characters by default.

> **Note:** Some variation sequences may specify a narrow or wide presentation behaviour. (This can be true for sequences defined in StandardizedVariants.txt [[StandardizedVariants](https://www.unicode.org/reports/tr41/tr41-36.html#StandardizedVariants)] in the [[UCD](https://www.unicode.org/reports/tr41/tr41-36.html#UCD)] or in emoji-variation-sequences.txt in [[UTS51](https://www.unicode.org/reports/tr41/tr41-36.html#UTS51)].) For such sequences, the width behaviour specified by a variation sequence can differ from the East\_Asian\_Width property of the base character of that variation sequence. The width indication of the variation sequence can specify some aspects of glyph appearance such as advance width, while not specifying other aspects of presentation, such as rotational behavior in vertical text.

