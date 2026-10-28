# Filenames

Source: [OpenType font file](https://learn.microsoft.com/en-us/typography/opentype/spec/otff) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/otff).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-organization-of-an-opentype-font.md)

An OpenType font file contains data, in table format, used for rendering of text. Portions of the data are used by applications to calculate the layout of text using the font; that is, the selection of glyphs and their placement within a line. Other data provide descriptions of glyphs as TrueType or Compact Font Format (CFF) outlines. Still other data can provide monochromatic or color bitmaps, SVG documents or binary, 2D vector graphics compositions as alternate glyph descriptions. The font data also includes meta-information, such as name strings that can be used to present the font as an available choice in a font-picker user interface. Each of these types of data is stored in one or more tables, each designed for a particular purpose.

<a id="filenames"></a>

## Filenames

OpenType font files may have the extension .OTF, .TTF, .OTC or .TTC. (The extension may be upper or lower case.) The extensions .OTC and .TTC should only be used for [font collection](003-calculating-checksums.md#collections) files. For additional information on filename extension conventions, see “Filenames” in [Recommendations for OpenType fonts](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#filenames).

<a id="dataTypes"></a>

<a id="data-types"></a>

## Data Types

The following data types are used in the OpenType font file. All OpenType fonts use big-endian (network) byte order:

<a id="dt_uint8"></a>

<a id="dt_int8"></a>

<a id="dt_uint16"></a>

<a id="dt_int16"></a>

<a id="dt_uint24"></a>

<a id="dt_uint32"></a>

<a id="dt_int32"></a>

<a id="dt_Fixed"></a>

<a id="dt_FWORD"></a>

<a id="dt_UFWORD"></a>

<a id="dt_F2DOT14"></a>

<a id="dt_LONGDATETIME"></a>

<a id="dt_Tag"></a>

<a id="dt_offset8"></a>

<a id="dt_Offset16"></a>

<a id="dt_Offset24"></a>

<a id="dt_Offset32"></a>

| Data Type | Description |
| --- | --- |
| uint8 | 8-bit unsigned integer. |
| int8 | 8-bit signed integer. |
| uint16 | 16-bit unsigned integer. |
| int16 | 16-bit signed integer. |
| uint24 | 24-bit unsigned integer. |
| uint32 | 32-bit unsigned integer. |
| int32 | 32-bit signed integer. |
| Fixed | 32-bit signed fixed-point number (16.16) |
| FWORD | int16 that describes a quantity in font design units. |
| UFWORD | uint16 that describes a quantity in font design units. |
| F2DOT14 | 16-bit signed fixed number with the low 14 bits of fraction (2.14). |
| LONGDATETIME | Date and time represented in number of seconds since 12:00 midnight, January 1, 1904, UTC. The value is represented as a signed 64-bit integer. |
| Tag | Array of four uint8s (length = 32 bits) used to identify a table, design-variation axis, script, language system, feature, or baseline. |
| Offset8 | 8-bit offset to a table, same as uint8, NULL offset = 0x00 |
| Offset16 | Short offset to a table, same as uint16, NULL offset = 0x0000 |
| Offset24 | 24-bit offset to a table, same as uint24, NULL offset = 0x000000 |
| Offset32 | Long offset to a table, same as uint32, NULL offset = 0x00000000 |
| Version16Dot16 | Packed 32-bit value with major and minor version numbers. (See [Table Version Numbers](001-filenames.md#table-version-numbers).) |

The F2DOT14 format consists of a signed, 2’s complement integer and an unsigned fraction. To compute the actual value, take the integer and add the fraction. Examples of 2.14 values are:

<a id="examplesF2DOT14"></a>

| Decimal Value | Hex Value | Integer | Fraction |
| --- | --- | --- | --- |
| 1.999939 | 0x7fff | 1 | 16383/16384 |
| 1.75 | 0x7000 | 1 | 12288/16384 |
| 0.000061 | 0x0001 | 0 | 1/16384 |
| 0.0 | 0x0000 | 0 | 0/16384 |
| -0.000061 | 0xffff | -1 | 16383/16384 |
| -2.0 | 0x8000 | -2 | 0/16384 |

A Tag value is a uint8 array. Each byte within the array must have a value in the range 0x20 to 0x7E. This corresponds to the range of values of Unicode Basic Latin characters in UTF-8 encoding, which is the same as the printable ASCII characters. As a result, a Tag value can be re-interpreted as a four-character sequence, which is conventionally how they are referred to. Formally, however, the value is a byte array. When re-interpreted as characters, the Tag value is case sensitive. It must have one to four non-space characters, padded with trailing spaces (byte value 0x20). A space character must not be followed by a non-space character.

Within this specification, many structures are defined in terms of the data types listed above. Structures are characterized as either *records* or *tables*. The distinction between records and tables is based on these general criteria:

- Tables are referenced by offsets. If a table contains an offset to a sub-structure, the offset is normally from the start of that table.
- Records occur sequentially within a parent structure, either within a sequence of table fields or within an array of records of a given type. If a record contains an offset to a sub-structure, that structure is logically a subtable of the record’s parent table and the offset is normally from the start of the parent table.

In some cases, fields for subtable offsets are documented as permitting NULL values when the given subtable is optional. For example, in the [BASE table header](https://learn.microsoft.com/en-us/typography/opentype/spec/base#base-header), the horizAxisOffset and vertAxisOffset fields may be NULL, meaning that either subtable (or both) is optional. A NULL subtable offset always indicates that the given subtable is not present. Applications must never interpret a NULL offset value as the offset to subtable data. For cases in which subtable offset fields are not documented as permitting NULL values, font compilers must include a subtable of the indicated format, even if it is a header stub without further data (for example, a [coverage](../../chapter2/chunks/005-coverage-table.md#coverage-table) table with no glyph IDs). Applications parsing font data should, however, anticipate non-conformant font data that has a NULL subtable offset where only a non-NULL value is expected.

<a id="versionNumbers"></a>

<a id="table-version-numbers"></a>

## Table Version Numbers

Most tables have version numbers, and the version number for the entire font is contained in the [table directory](002-organization-of-an-opentype-font.md#tabledirectory). Note that there are five different table version number types, each with its own numbering scheme.

- A single uint16 field. This is used in a number of tables, usually with versions starting at zero (0).
- Separate, uint16 major and minor version fields. This is used in a number of tables, usually with versions starting at 1.0.
- A uint32 field with enumerated values.
- A uint32 field with a numeric value. This is used only in the [DSIG](https://learn.microsoft.com/en-us/typography/opentype/spec/dsig) and ['meta'](https://learn.microsoft.com/en-us/typography/opentype/spec/meta) tables.
- A Version16Dot16 field for major/minor version numbers. This is used only in the ['maxp'](../../maxp/README.md), ['post'](../../post/README.md) and ['vhea'](../../vhea/README.md) tables.

The Version16Dot16 type is used for the version field of certain tables, and only for reasons of backward compatibility. (In earlier versions, these fields were documented as using a Fixed value, but had minor version numbers that did not follow the definition of the Fixed type.) Version16Dot16 is a packed value: the upper 16 bits comprise a major version number, and the lower 16 bits, a minor version. Non-zero minor version numbers are represented using digits 0 to 9 in the highest-order nibbles of the lower 16 bits. For example, the version field of 'maxp' table version 0.5 is 0x00005000, and that of 'vhea' table version 1.1 is 0x00011000. This type is used only in the 'maxp', 'post' and 'vhea' tables, and will not be used for any other tables in the future.

The table directory uses a uint32 field, sfntVersion, with an enumeration of defined values, some of which represent four-character tags; see the section below, “Organization of an OpenType Font”, for details.

Implementations reading tables must include code to check version numbers so that, if and when the format and version number changes, older implementations will handle newer versions gracefully.

Minor version number changes always imply format changes that are compatible extensions. If an implementation understands a major version number, then it can safely proceed with reading the table. If the minor version is greater than the latest version recognized by the implementation, then the extension fields will be undetectable to the implementation.

For purposes of compatibility, version numbers that are represented using a single uint16 or uint32 value are treated like a minor version number, and any format changes are compatible extensions.

Note that some field values that were undefined or reserved in an earlier revision may be assigned meanings in a minor version change. Implementations should never make assumptions regarding reserved or unassigned values or bits in bit fields, and can ignore them if encountered. When writing font data, tools should always write zero for reserved fields or bits. Validators should warn of any non-zero values for fields or bits that are not defined for the given version against which data is being validated.

If the major version is not recognized, the implementation must not read the table as it can make no assumptions regarding interpretation of the binary data. The implementation should treat the table as missing.

