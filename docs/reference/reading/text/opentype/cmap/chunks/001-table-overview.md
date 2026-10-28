# Table overview

Source: [Character to glyph index mapping](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-macintosh-platform-platform-id-1.md)

<a id="overview"></a>

<a id="table-overview"></a>

## Table overview

This table defines the mapping of character codes to a default glyph index. Different subtables may be defined that each contain mappings for different character encoding schemes. The table header indicates the character encodings for which subtables are present.

Regardless of the encoding scheme, character codes that do not correspond to any glyph in the font should be mapped to glyph index 0. The glyph at this location must be a special glyph representing a missing character, commonly known as .notdef.

Each subtable is in one of seven possible formats and begins with a format field indicating the format used. The first four formats — formats 0, 2, 4 and 6 — were originally defined prior to Unicode 2.0. These formats allow for 8-bit single-byte, 8-bit multi-byte, and 16-bit encodings. With the introduction of supplementary planes in Unicode 2.0, the Unicode addressable code space extends beyond 16 bits. To accommodate this, three additional formats were added — formats 8, 10 and 12 — that allow for 32-bit encoding schemes.

Other enhancements in Unicode led to the addition of other subtable formats. Subtable format 13 allows for an efficient mapping of many characters to a single glyph; this is useful for “last-resort” fonts that provide fallback rendering for all possible Unicode characters with a distinct fallback glyph for different Unicode ranges. Subtable format 14 provides a unified mechanism for supporting Unicode variation sequences.

Of the seven available formats, not all are commonly used today. Formats 4 or 12 are appropriate for most new fonts, depending on the Unicode character repertoire supported. Format 14 is used in many applications for support of Unicode variation sequences. Some platforms also make use for format 13 for a last-resort fallback font. Other subtable formats are not recommended for use in new fonts. Application developers, however, should anticipate that any of the formats may be used in fonts.

> *Note:* The 'cmap' table version number remains at 0x0000 for fonts that make use of the newer subtable formats.

<a id="cmapHeader"></a>

<a id="cmap-header"></a>

## 'cmap' Header

The Character to Glyph Index Mapping table is organized as follows.

*'cmap' Header:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | version | Table version number (0). |
| uint16 | numTables | Number of encoding tables that follow. |
| EncodingRecord | encodingRecords[numTables] |  |

<a id="encodings"></a>

<a id="encoding-records-and-encodings"></a>

## Encoding records and encodings

The array of encoding records specifies particular encodings and the offset to the subtable for each encoding.

<a id="encodingRec"></a>

*EncodingRecord:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | platformID | Platform ID. |
| uint16 | encodingID | Platform-specific encoding ID. |
| Offset32 | subtableOffset | Byte offset from beginning of table to the subtable for this encoding. |

The platform ID and platform-specific encoding ID in the encoding record are used to specify a particular character encoding. In the case of the Macintosh platform, a language field within the mapping subtable is also used for this purpose.

The encoding record entries in the 'cmap' header must be sorted first by platform ID, then by platform-specific encoding ID, and then by the language field in the corresponding subtable. Each platform ID, platform-specific encoding ID, and subtable language combination may appear only once in the 'cmap' table.

Apart from a format 14 subtable, all other subtables are exclusive: applications should select and use one and ignore the others. If a Unicode subtable is used (platform 0, or platform 3 / encoding 1 or 10), then a format 14 subtable using platform 0/encoding 5 can also be supplemented for mapping Unicode variation sequences.

If a font includes Unicode subtables for both 16-bit encoding (typically, format 4) and also 32-bit encoding (formats 10 or 12), then the characters supported by the subtable for 32-bit encoding should be a superset of the characters supported by the subtable for 16-bit encoding, and the 32-bit encoding should be used by applications.

Fonts should not include 16-bit Unicode subtables using both format 4 and format 6; format 4 should be used. Similarly, fonts should not include 32-bit Unicode subtables using both format 10 and format 12; format 12 should be used.

If a font includes encoding records for Unicode subtables of the same format but with different platform IDs, an application may choose which to select, but should make this selection consistently each time the font is used.

<a id="platform-ids"></a>

### Platform IDs

The following platform IDs are defined:

| Platform ID | Platform name | Platform-specific encoding IDs |
| --- | --- | --- |
| 0 | Unicode | [Various](001-table-overview.md#unicode-platform-platform-id--0) |
| 1 | Macintosh | [Script manager code](002-macintosh-platform-platform-id-1.md#macintosh-platform-platform-id--1) |
| *2* | *ISO [deprecated]* | *[ISO encoding](002-macintosh-platform-platform-id-1.md#iso-platform-platform-id--2) [deprecated]* |
| 3 | Windows | [Windows encoding](002-macintosh-platform-platform-id-1.md#windows-platform-platform-id--3) |
| 4 | Custom | [Custom](002-macintosh-platform-platform-id-1.md#custom-platform-platform-id--4-and-otf-windows-nt-compatibility-mapping) |

Platform ID values 240 through 255 are reserved for user-defined platforms and shall never be assigned to a registered platform.

<a id="unicodePlatform"></a>

<a id="unicode-platform-platform-id--0"></a>

### Unicode platform (platform ID = 0)

The following encoding IDs are defined for use with the Unicode platform:

<a id="encID0_pid0"></a>

<a id="encID1_pid0"></a>

<a id="encID2_pid0"></a>

<a id="encID3_pid0"></a>

<a id="encID4_pid0"></a>

<a id="encID5_pid0"></a>

<a id="encID6_pid0"></a>

| Encoding ID | Description |
| --- | --- |
| 0 | Unicode 1.0 semantics—*deprecated* |
| 1 | Unicode 1.1 semantics—*deprecated* |
| 2 | ISO/IEC 10646 semantics—*deprecated* |
| 3 | Unicode 2.0 and onwards semantics, Unicode BMP only |
| 4 | Unicode 2.0 and onwards semantics, Unicode full repertoire |
| 5 | Unicode variation sequences—for use with subtable format 14 |
| 6 | Unicode full repertoire—for use with subtable format 13 |

Use of encoding IDs 0, 1 or 2 is deprecated.

Encoding ID 3 should be used in conjunction with 'cmap' subtable formats 4 or 6. Encoding ID 4 should be used in conjunction with subtable formats 10 or 12.

Unicode variation sequences supported by the font should be specified in the 'cmap' table using a format 14 subtable. A format 14 subtable must only be used under platform ID 0 and encoding ID 5; and encoding ID 5 should only be used with a format 14 subtable.

Encoding ID 6 should only be used in conjunction with 'cmap' subtable format 13; and subtable format 13 should only be used under platform ID 0 and encoding ID 6.

