# Format 13: Many-to-one range mappings

Source: [Character to glyph index mapping](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-format-6-trimmed-table-mapping.md)

<a id="format13"></a>

<a id="format-13-many-to-one-range-mappings"></a>

### Format 13: Many-to-one range mappings

This subtable provides for situations in which the same glyph is used for hundreds or even thousands of consecutive characters spanning across multiple ranges of the code space. This subtable format may be useful for “last resort” fonts, although these fonts may use other suitable subtable formats as well. (For “last resort” fonts, see also the ['head' table](../../head/README.md) flags, bit 14.)

> *Note:* Subtable format 13 has the same structure as format 12; it differs only in the interpretation of the startGlyphID/glyphID fields.

<a id="subtable_13"></a>

*'cmap' Subtable Format 13:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Subtable format; set to 13. |
| uint16 | reserved | Reserved; set to 0 |
| uint32 | length | Byte length of this subtable (including the header) |
| uint32 | language | For requirements on use of the language field, see “[Use of the language field in 'cmap' subtables](002-macintosh-platform-platform-id-1.md#language)” in this document. |
| uint32 | numGroups | Number of groupings which follow |
| ConstantMapGroup | groups[numGroups] | Array of ConstantMapGroup records. |

The constant map group record has the same structure as the sequential map group record, with start and end character codes and a mapped glyph ID. However, the same glyph ID applies to all characters in the specified range rather than sequential glyph IDs.

<a id="constMapGrpRec"></a>

*ConstantMapGroup Record:*

| Type | Name | Description |
| --- | --- | --- |
| uint32 | startCharCode | First character code in this group |
| uint32 | endCharCode | Last character code in this group |
| uint32 | glyphID | Glyph index to be used for all the characters in the group’s range. |

<a id="format14"></a>

<a id="format-14-unicode-variation-sequences"></a>

### Format 14: Unicode variation sequences

Subtable format 14 specifies the Unicode variation sequences (UVSes) supported by the font. A variation sequence, according to the Unicode Standard, comprises a base character followed by a variation selector. For example, <U+82A6, U+E0101>.

This subtable format must only be used under platform ID 0 and encoding ID 5.

The subtable partitions the UVSes supported by the font into two categories: “default” and “non-default” UVSes. Given a UVS, if the glyph obtained by looking up the base character of that sequence in the Unicode 'cmap' subtable (i.e., the BMP subtable or BMP + supplementary-planes subtable) is the glyph to use for that sequence, then the sequence is a “default” UVS. Otherwise, it is a “non-default” UVS, and the glyph to use for that sequence is specified in the format 14 subtable itself.

The example at the bottom of the page shows how a font vendor can use format 14 for a JIS-2004-aware font.

<a id="subtable_14"></a>

*'cmap' Subtable Format 14:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Subtable format; set to 14. |
| uint32 | length | Byte length of this subtable (including this header) |
| uint32 | numVarSelectorRecords | Number of variation Selector Records |
| VariationSelector | varSelector[numVarSelectorRecords] | Array of VariationSelector records. |

Each VariationSelector record specifies a variation selector character, and offsets to “default” and “non-default” tables used to map variation sequences using that variation selector.

<a id="varSelRec"></a>

*VariationSelector Record:*

| Type | Name | Description |
| --- | --- | --- |
| uint24 | varSelector | Variation selector |
| Offset32 | defaultUVSOffset | Offset from the start of the format 14 subtable to Default UVS table. May be 0. |
| Offset32 | nonDefaultUVSOffset | Offset from the start of the format 14 subtable to Non-Default UVS table. May be 0. |

The VariationSelector records are sorted in increasing order of varSelector. No two records may have the same varSelector value.

A VariationSelector record and the data its offsets point to specify those UVSes supported by the font for which the variation selector is the varSelector value of the record. The base characters of the UVSes are stored in the tables pointed to by the offsets. The UVSes are partitioned by whether they are default or non-default UVSes.

Glyph IDs to be used for non-default UVSes are specified in the Non-Default UVS table.

<a id="subtable_dfltUVS"></a>

<a id="default-uvs-table"></a>

#### Default UVS table

A Default UVS table is simply a range-compressed list of Unicode scalar values, representing the base characters of the default UVSes which use the varSelector of the associated VariationSelector record.

<a id="dfltUVS"></a>

*DefaultUVS Table:*

| Type | Name | Description |
| --- | --- | --- |
| uint32 | numUnicodeValueRanges | Number of Unicode character ranges. |
| UnicodeRange | ranges[numUnicodeValueRanges] | Array of UnicodeRange records. |

Each Unicode range record specifies a contiguous range of Unicode values.

<a id="unicRngRec"></a>

*UnicodeRange Record:*

| Type | Name | Description |
| --- | --- | --- |
| uint24 | startUnicodeValue | First value in this range |
| uint8 | additionalCount | Number of additional values in this range |

For example, the range U+4E4D – U+4E4F (3 values) will set startUnicodeValue to 0x004E4D and additionalCount to 2. A singleton range will set additionalCount to 0.

The sum (startUnicodeValue + additionalCount) must not exceed 0xFFFFFF.

The Unicode Value Ranges are sorted in increasing order of startUnicodeValue. The ranges must not overlap; i.e., (startUnicodeValue + additionalCount) must be less than the startUnicodeValue of the following range (if any).

All code points listed in the ranges array should have corresponding entries in a Unicode 'cmap' subtable. Applications could encounter fonts in which this is not the case, however.

<a id="subtable_nonDfltUVS"></a>

<a id="non-default-uvs-table"></a>

#### Non-Default UVS table

A Non-Default UVS table is a list of pairs of Unicode scalar values and glyph IDs. The Unicode values represent the base characters of all non-default UVSes which use the varSelector of the associated VariationSelector record, and the glyph IDs specify the glyph IDs to use for the UVSes.

<a id="nonDfltUVS"></a>

*NonDefaultUVS Table:*

| Type | Name | Description |
| --- | --- | --- |
| uint32 | numUVSMappings | Number of UVS Mappings that follow |
| UVSMapping | uvsMappings[numUVSMappings] | Array of UVSMapping records. |

Each UVSMapping record provides a glyph ID mapping for one base Unicode character, when that base character is used in a variation sequence with the current variation selector.

<a id="uvsMapRec"></a>

*UVSMapping Record:*

| Type | Name | Description |
| --- | --- | --- |
| uint24 | unicodeValue | Base Unicode value of the UVS |
| uint16 | glyphID | Glyph ID of the UVS |

The UVS Mappings are sorted in increasing order of unicodeValue. No two mappings in this table may have the same unicodeValue values.

Typically, code points listed in the uvsMappings array will have corresponding entries in a Unicode 'cmap' subtable. This is not required, however. For example, it might not be the case if a font is intended for use with content in which a given Unicode character only occurs in a variation sequence.

<a id="format14Example"></a>

<a id="example"></a>

#### Example

Here is an example of how a format 14 'cmap' subtable may be used in a font that is aware of JIS-2004 variant glyphs. The CIDs (character IDs) in this example refer to those in the Adobe Character Collection “Adobe-Japan1” and can be assumed to be identical to the glyph IDs in the font in our example.

JIS-2004 changed the default glyph variants for some of its code points. For example:

JIS-90: U+82A6 ⇒ CID 1142  
JIS-2004: U+82A6 ⇒ CID 7961

Both of these glyph variants are supported through the use of Unicode variation sequences, as the following examples from Unicode’s UVS registry show:

U+82A6 U+E0100 ⇒ CID 1142  
U+82A6 U+E0101 ⇒ CID 7961

If the font wants to support the JIS-2004 variants by default, it will:

- encode glyph ID 7961 at U+82A6 in the Unicode 'cmap' subtable;
- specify <U+82A6, U+E0101> in the UVS 'cmap' subtable’s Default UVS table (varSelector will be 0x0E0101 and defaultUVSOffset will point to data containing a 0x0082A6 Unicode value);
- specify <U+82A6, U+E0100> ⇒ glyph ID 1142 in the UVS 'cmap' subtable’s Non-Default UVS table (varSelector will be 0x0E0100 and nonDefaultBaseUVSOffset will point to data containing a unicodeValue 0x0082A6 and glyphID 1142).

If, however, the font wants to support the JIS-90 variants by default, it will:

- encode glyph ID 1142 at U+82A6 in the Unicode 'cmap' subtable;
- specify <U+82A6, U+E0100> in the UVS 'cmap' subtable’s Default UVS table;
- specify <U+82A6, U+E0101> ⇒ glyph ID 7961 in the UVS 'cmap' subtable’s Non-Default UVS table.
