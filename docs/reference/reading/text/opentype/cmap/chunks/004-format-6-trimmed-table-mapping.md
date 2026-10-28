# Format 6: Trimmed table mapping

Source: [Character to glyph index mapping](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-format-4-segment-mapping-to-delta-values.md) · [Next](005-format-13-many-to-one-range-mappings.md)

<a id="format6"></a>

<a id="format-6-trimmed-table-mapping"></a>

### Format 6: Trimmed table mapping

Format 6 was designed to map 16-bit characters to glyph indexes when the character codes for a font fall into a single contiguous range.

<a id="subtable_6"></a>

*'cmap' Subtable Format 6:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format number is set to 6. |
| uint16 | length | This is the length in bytes of the subtable. |
| uint16 | language | For requirements on use of the language field, see “[Use of the language field in 'cmap' subtables](002-macintosh-platform-platform-id-1.md#language)” in this document. |
| uint16 | firstCode | First character code of subrange. |
| uint16 | entryCount | Number of character codes in subrange. |
| uint16 | glyphIdArray[entryCount] | Array of glyph index values for character codes in the range. |

The firstCode and entryCount values specify a subrange (beginning at firstCode, length = entryCount) within the range of possible character codes. Codes outside of this subrange are mapped to glyph index 0. The offset of the code (from the first code) within this subrange is used as index to the glyphIdArray, which provides the glyph index value.

<a id="format8"></a>

<a id="format-8-mixed-16-bit-and-32-bit-coverage"></a>

### Format 8: mixed 16-bit and 32-bit coverage

Subtable format 8 was designed to support Unicode supplementary-plane characters in UTF-16 encoding, though it is not commonly used. Format 8 is similar to format 2 in that it provides for mixed-length character codes. Instead of allowing for 8- and 16-bit character codes, however, it allows for 16- and 32-bit character codes.

If a font contains Unicode supplementary-plane characters (U+10000 to U+10FFFF), then it’s likely that it will also include Unicode BMP characters (U+0000 to U+FFFF) as well. Hence, there is a need to map a mixture of 16-bit and 32-bit character codes. A simplifying assumption is made: namely, that there are no 32-bit character codes which share the same first 16 bits as any 16-bit character code. (Since the Unicode code space extends only to U+10FFFF, a potential conflict exists only for characters U+0000 to U+0010, which are non-printing control characters.) This means that the determination as to whether a particular 16-bit value is a standalone character code or the start of a 32-bit character code can be made by looking at the 16-bit value directly, with no further information required.

<a id="subtable_8"></a>

*'cmap' Subtable Format 8:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Subtable format; set to 8. |
| uint16 | reserved | Reserved; set to 0 |
| uint32 | length | Byte length of this subtable (including the header) |
| uint32 | language | For requirements on use of the language field, see “[Use of the language field in 'cmap' subtables](002-macintosh-platform-platform-id-1.md#language)” in this document. |
| uint8 | is32[8192] | Tightly packed array of bits (8K bytes total) indicating whether the particular 16-bit (index) value is the start of a 32-bit character code |
| uint32 | numGroups | Number of groupings which follow |
| SequentialMapGroup | groups[numGroups] | Array of SequentialMapGroup records. |

Each sequential map group record specifies a character range and the starting glyph ID mapped from the first character. Glyph IDs for subsequent characters follow in sequence.

<a id="seqMapGrpRec_8"></a>

*SequentialMapGroup Record:*

| Type | Name | Description |
| --- | --- | --- |
| uint32 | startCharCode | First character code in this group; note that if this group is for one or more 16-bit character codes (which is determined from the is32 array), this 32-bit value will have the high 16-bits set to zero |
| uint32 | endCharCode | Last character code in this group; same condition as listed above for the startCharCode |
| uint32 | startGlyphID | Glyph index corresponding to the starting character code |

A few notes here. The endCharCode is used, rather than a count, because comparisons for group matching are usually done on an existing character code, and having the endCharCode be there explicitly saves the necessity of an addition per group. Groups must be sorted by increasing startCharCode. A group’s endCharCode must be less than the startCharCode of the following group, if any.

To determine if a particular word (cp) is the first half of 32-bit code points, one can use an expression such as ( is32[ cp / 8 ] & ( 1 << ( 7 - ( cp % 8 ) ) ) ). If this is non-zero, then the word is the first half of a 32-bit code point.

0 is not a special value for the high word of a 32-bit code point. A font must not have both a glyph for the code point 0x0000 and glyphs for code points with a high word of 0x0000.

The presence of the packed array of bits indicating whether a particular 16-bit value is the start of a 32-bit character code is useful even when the font contains no glyphs for a particular 16-bit start value. This is because the system software often needs to know how many bytes ahead the next character begins, even if the current character maps to the missing glyph. By including this information explicitly in this table, no “secret” knowledge needs to be encoded into the OS.

Although this format was created to support Unicode supplementary-plane characters, it is not widely supported or used. Also, no character encoding other than Unicode uses mixed 16-/32-bit characters. The use of this format is discouraged.

<a id="format10"></a>

<a id="format-10-trimmed-array"></a>

### Format 10: Trimmed array

Subtable format 10 was designed to support Unicode supplementary-plane characters, though it is not commonly used. Format 10 is similar to format 6 in that it defines a trimmed array for a tight range of character codes. It differs, however, in that it uses 32-bit character codes.

<a id="subtable_10"></a>

*'cmap' Subtable Format 10:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Subtable format; set to 10. |
| uint16 | reserved | Reserved; set to 0 |
| uint32 | length | Byte length of this subtable (including the header) |
| uint32 | language | For requirements on use of the language field, see “[Use of the language field in 'cmap' subtables](002-macintosh-platform-platform-id-1.md#language)” in this document. |
| uint32 | startCharCode | First character code covered |
| uint32 | numChars | Number of character codes covered |
| uint16 | glyphIdArray[] | Array of glyph indices for the character codes covered |

This format is not widely used and is not supported on Windows platforms. It would be most suitable for fonts that support only a contiguous range of Unicode supplementary-plane characters, but such fonts are rare.

<a id="format12"></a>

<a id="format-12-segmented-coverage"></a>

### Format 12: Segmented coverage

This is the standard character-to-glyph-index mapping subtable for fonts supporting Unicode character repertoires that include supplementary-plane characters (U+10000 to U+10FFFF).

Fonts that include a format 12 subtable can also include a format 4 subtable for compatibility with older applications. This is not required, however. See the [Recommendations](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#cmap-table) chapter for additional information.

Format 12 is similar to format 4 in that it defines segments for sparse representation. It differs, however, in that it uses 32-bit character codes.

<a id="subtable_12"></a>

*'cmap' Subtable Format 12:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Subtable format; set to 12. |
| uint16 | reserved | Reserved; set to 0 |
| uint32 | length | Byte length of this subtable (including the header) |
| uint32 | language | For requirements on use of the language field, see “[Use of the language field in 'cmap' subtables](002-macintosh-platform-platform-id-1.md#language)” in this document. |
| uint32 | numGroups | Number of groupings which follow |
| SequentialMapGroup | groups[numGroups] | Array of SequentialMapGroup records. |

The sequential map group record is the same format as is used for the format 8 subtable. The qualifications regarding 16-bit character codes does not apply here, however, since characters codes are uniformly 32-bit.

<a id="seqMapGrpRec_12"></a>

*SequentialMapGroup Record:*

| Type | Name | Description |
| --- | --- | --- |
| uint32 | startCharCode | First character code in this group |
| uint32 | endCharCode | Last character code in this group |
| uint32 | startGlyphID | Glyph index corresponding to the starting character code |

Groups must be sorted by increasing startCharCode. A group’s endCharCode must be less than the startCharCode of the following group, if any. The endCharCode is used, rather than a count, because comparisons for group matching are usually done on an existing character code, and having the endCharCode be there explicitly saves the necessity of an addition per group.

