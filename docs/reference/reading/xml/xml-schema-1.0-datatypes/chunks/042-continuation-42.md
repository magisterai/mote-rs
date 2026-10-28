# Continuation 42

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](041-f-1-1-character-class-escapes.md) · [Next](043-g-glossary-non-normative.md)

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Categories</font></strong></td></tr><tr><td><a id="cats"></a><a id="nt-IsCategory"></a><a id="lets"></a><a id="nt-Letters"></a><a id="marks"></a><a id="nt-Marks"></a><a id="nums"></a><a id="nt-Numbers"></a><a id="punc"></a><a id="nt-Punctuation"></a><a id="seps"></a><a id="nt-Separators"></a><a id="syms"></a><a id="nt-Symbols"></a><a id="others"></a><a id="nt-Others"></a><table border="0"><tbody><tr valign="baseline"><td><a id="cats" name="cats"></a><a id="nt-IsCategory" name="nt-IsCategory"></a>[28]   </td><td><code>IsCategory</code></td><td>   ::=   </td><td><code>
<a href="042-continuation-42.md#nt-Letters">Letters</a> |
			<a href="042-continuation-42.md#nt-Marks">Marks</a> |
			<a href="042-continuation-42.md#nt-Numbers">Numbers</a> |
			<a href="042-continuation-42.md#nt-Punctuation">Punctuation</a> |
			<a href="042-continuation-42.md#nt-Separators">Separators</a> |
			<a href="042-continuation-42.md#nt-Symbols">Symbols</a> |
			<a href="042-continuation-42.md#nt-Others">Others</a>
</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="lets" name="lets"></a><a id="nt-Letters" name="nt-Letters"></a>[29]   </td><td><code>Letters</code></td><td>   ::=   </td><td><code>'L' [ultmo]?</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="marks" name="marks"></a><a id="nt-Marks" name="nt-Marks"></a>[30]   </td><td><code>Marks</code></td><td>   ::=   </td><td><code>'M' [nce]?</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="nums" name="nums"></a><a id="nt-Numbers" name="nt-Numbers"></a>[31]   </td><td><code>Numbers</code></td><td>   ::=   </td><td><code>'N' [dlo]?</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="punc" name="punc"></a><a id="nt-Punctuation" name="nt-Punctuation"></a>[32]   </td><td><code>Punctuation</code></td><td>   ::=   </td><td><code>'P' [cdseifo]?</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="seps" name="seps"></a><a id="nt-Separators" name="nt-Separators"></a>[33]   </td><td><code>Separators</code></td><td>   ::=   </td><td><code>'Z' [slp]?</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="syms" name="syms"></a><a id="nt-Symbols" name="nt-Symbols"></a>[34]   </td><td><code>Symbols</code></td><td>   ::=   </td><td><code>'S' [mcko]?</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="others" name="others"></a><a id="nt-Others" name="nt-Others"></a>[35]   </td><td><code>Others</code></td><td>   ::=   </td><td><code>'C' [cfon]?</code></td></tr></tbody></table></td></tr></tbody></table>

**Note:**
The properties mentioned above exclude the `Cs` property.
The `Cs` property identifies "surrogate" characters, which do not
occur at the level of the "character abstraction" that XML instance documents
operate on.

<a id="dt-ccesblock"></a>

[Definition:]
[[Unicode Database]](044-h-references.md#UnicodeDB) groups code points into a number of blocks
such as Basic Latin (i.e., ASCII), Latin-1 Supplement, Hangul Jamo,
CJK Compatibility, etc.
The set containing all characters that have block name `X`
(with all white space stripped out),
can be identified with a **block escape** `\p{IsX}`.
The complement of this set is specified with the
**block escape** `\P{IsX}`.
(`[\P{IsX}]` = `[^\p{IsX}]`).

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Block Escape</font></strong></td></tr><tr><td><a id="blockesc"></a><a id="nt-IsBlock"></a><table border="0"><tbody><tr valign="baseline"><td><a id="blockesc" name="blockesc"></a><a id="nt-IsBlock" name="nt-IsBlock"></a>[36]   </td><td><code>IsBlock</code></td><td>   ::=   </td><td><code>'Is' [a-zA-Z0-9#x2D]+</code></td></tr></tbody></table></td></tr></tbody></table>

The following table specifies the recognized block names (for more
information, see the "Blocks.txt" file in [[Unicode Database]](044-h-references.md#UnicodeDB)).

| Start Code | End Code | Block Name |  | Start Code | End Code | Block Name |
| --- | --- | --- | --- | --- | --- | --- |
| #x0000 | #x007F | BasicLatin |  | #x0080 | #x00FF | Latin-1Supplement |
| #x0100 | #x017F | LatinExtended-A |  | #x0180 | #x024F | LatinExtended-B |
| #x0250 | #x02AF | IPAExtensions |  | #x02B0 | #x02FF | SpacingModifierLetters |
| #x0300 | #x036F | CombiningDiacriticalMarks |  | #x0370 | #x03FF | Greek |
| #x0400 | #x04FF | Cyrillic |  | #x0530 | #x058F | Armenian |
| #x0590 | #x05FF | Hebrew |  | #x0600 | #x06FF | Arabic |
| #x0700 | #x074F | Syriac |  | #x0780 | #x07BF | Thaana |
| #x0900 | #x097F | Devanagari |  | #x0980 | #x09FF | Bengali |
| #x0A00 | #x0A7F | Gurmukhi |  | #x0A80 | #x0AFF | Gujarati |
| #x0B00 | #x0B7F | Oriya |  | #x0B80 | #x0BFF | Tamil |
| #x0C00 | #x0C7F | Telugu |  | #x0C80 | #x0CFF | Kannada |
| #x0D00 | #x0D7F | Malayalam |  | #x0D80 | #x0DFF | Sinhala |
| #x0E00 | #x0E7F | Thai |  | #x0E80 | #x0EFF | Lao |
| #x0F00 | #x0FFF | Tibetan |  | #x1000 | #x109F | Myanmar |
| #x10A0 | #x10FF | Georgian |  | #x1100 | #x11FF | HangulJamo |
| #x1200 | #x137F | Ethiopic |  | #x13A0 | #x13FF | Cherokee |
| #x1400 | #x167F | UnifiedCanadianAboriginalSyllabics |  | #x1680 | #x169F | Ogham |
| #x16A0 | #x16FF | Runic |  | #x1780 | #x17FF | Khmer |
| #x1800 | #x18AF | Mongolian |  | #x1E00 | #x1EFF | LatinExtendedAdditional |
| #x1F00 | #x1FFF | GreekExtended |  | #x2000 | #x206F | GeneralPunctuation |
| #x2070 | #x209F | SuperscriptsandSubscripts |  | #x20A0 | #x20CF | CurrencySymbols |
| #x20D0 | #x20FF | CombiningMarksforSymbols |  | #x2100 | #x214F | LetterlikeSymbols |
| #x2150 | #x218F | NumberForms |  | #x2190 | #x21FF | Arrows |
| #x2200 | #x22FF | MathematicalOperators |  | #x2300 | #x23FF | MiscellaneousTechnical |
| #x2400 | #x243F | ControlPictures |  | #x2440 | #x245F | OpticalCharacterRecognition |
| #x2460 | #x24FF | EnclosedAlphanumerics |  | #x2500 | #x257F | BoxDrawing |
| #x2580 | #x259F | BlockElements |  | #x25A0 | #x25FF | GeometricShapes |
| #x2600 | #x26FF | MiscellaneousSymbols |  | #x2700 | #x27BF | Dingbats |
| #x2800 | #x28FF | BraillePatterns |  | #x2E80 | #x2EFF | CJKRadicalsSupplement |
| #x2F00 | #x2FDF | KangxiRadicals |  | #x2FF0 | #x2FFF | IdeographicDescriptionCharacters |
| #x3000 | #x303F | CJKSymbolsandPunctuation |  | #x3040 | #x309F | Hiragana |
| #x30A0 | #x30FF | Katakana |  | #x3100 | #x312F | Bopomofo |
| #x3130 | #x318F | HangulCompatibilityJamo |  | #x3190 | #x319F | Kanbun |
| #x31A0 | #x31BF | BopomofoExtended |  | #x3200 | #x32FF | EnclosedCJKLettersandMonths |
| #x3300 | #x33FF | CJKCompatibility |  | #x3400 | #x4DB5 | CJKUnifiedIdeographsExtensionA |
| #x4E00 | #x9FFF | CJKUnifiedIdeographs |  | #xA000 | #xA48F | YiSyllables |
| #xA490 | #xA4CF | YiRadicals |  | #xAC00 | #xD7A3 | HangulSyllables |
|  |  |  |  |  |  |  |
|  |  |  |  | #xE000 | #xF8FF | PrivateUse |
| #xF900 | #xFAFF | CJKCompatibilityIdeographs |  | #xFB00 | #xFB4F | AlphabeticPresentationForms |
| #xFB50 | #xFDFF | ArabicPresentationForms-A |  | #xFE20 | #xFE2F | CombiningHalfMarks |
| #xFE30 | #xFE4F | CJKCompatibilityForms |  | #xFE50 | #xFE6F | SmallFormVariants |
| #xFE70 | #xFEFE | ArabicPresentationForms-B |  | #xFEFF | #xFEFF | Specials |
| #xFF00 | #xFFEF | HalfwidthandFullwidthForms |  | #xFFF0 | #xFFFD | Specials |

**Note:**
The blocks mentioned above exclude the `HighSurrogates`,
`LowSurrogates` and `HighPrivateUseSurrogates` blocks.
These blocks identify "surrogate" characters, which do not
occur at the level of the "character abstraction" that XML instance documents
operate on.

**Note:**
[[Unicode Database]](044-h-references.md#UnicodeDB) is subject to future revision.
For example, the
grouping of code points into blocks might be updated.
All [·minimally conforming·](033-5-conformance.md#dt-minimally-conforming) processors [·must·](003-1-4-terminology.md#dt-must)
support the blocks defined in the version of [[Unicode Database]](044-h-references.md#UnicodeDB)
that is current at the time this specification became a W3C
Recommendation. However, implementors are encouraged to support the
blocks defined in any future version of the Unicode Standard.

For example, the [·block escape·](042-continuation-42.md#dt-ccesblock) for identifying the
ASCII characters is `\p{IsBasicLatin}`.

<a id="dt-ccesN"></a>

[Definition:]
A
**multi-character escape** provides a simple way to identify
a commonly used set of characters:

<table border="1" cellpadding="5" class="scrap" width="100%"><tbody><tr align="left"><td><strong><font color="red">Multi-Character Escape</font></strong></td></tr><tr><td><a id="multicharesc"></a><a id="nt-MultiCharEsc"></a><a id="wildcardesc"></a><a id="nt-WildcardEsc"></a><table border="0"><tbody><tr valign="baseline"><td><a id="multicharesc" name="multicharesc"></a><a id="nt-MultiCharEsc" name="nt-MultiCharEsc"></a>[37]   </td><td><code>MultiCharEsc</code></td><td>   ::=   </td><td><code>'\' [sSiIcCdDwW]</code></td></tr></tbody><tbody><tr valign="baseline"><td><a id="wildcardesc" name="wildcardesc"></a><a id="nt-WildcardEsc" name="nt-WildcardEsc"></a>[37a]   </td><td><code>WildcardEsc</code></td><td>   ::=   </td><td><code>'.'</code></td></tr></tbody></table></td></tr></tbody></table>

| Character sequence | Equivalent [·character class·](040-f-1-character-classes.md#dt-charclass) |
| --- | --- |
| . | [^\n\r] |
| \s | [#x20\t\n\r] |
| \S | [^\s] |
| \i | the set of initial name characters, those [·match·](003-1-4-terminology.md#dt-match)ed by [Letter](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-Letter) | '\_' | ':' |
| \I | [^\i] |
| \c | the set of name characters, those [·match·](003-1-4-terminology.md#dt-match)ed by [NameChar](https://www.w3.org/TR/2000/WD-xml-2e-20000814#NT-NameChar) |
| \C | [^\c] |
| \d | \p{Nd} |
| \D | [^\d] |
| \w | [#x0000-#x10FFFF]-[\p{P}\p{Z}\p{C}] (*all characters except the set of "punctuation", "separator" and "other" characters*) |
| \W | [^\w] |

**Note:**
The [·regular expression·](039-e-1-algorithm.md#dt-regex) language defined here does not
attempt to provide a general solution to "regular expressions" over
UCS character sequences. In particular, it does not easily provide
for matching sequences of base characters and combining marks.
The language is targeted at support of "Level 1" features as defined in
[[Unicode Regular Expression Guidelines]](044-h-references.md#unicodeRegEx). It is hoped that future versions of this
specification will provide support for "Level 2" features.

