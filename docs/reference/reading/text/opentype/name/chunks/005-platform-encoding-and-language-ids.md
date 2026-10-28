# Platform, encoding and language IDs

Source: [Naming table](https://learn.microsoft.com/en-us/typography/opentype/spec/name) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/name).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-name-id-examples.md)

<a id="platformEncodingLanguageIds"></a>

<a id="platform-encoding-and-language-ids"></a>

## Platform, encoding and language IDs

The following sections provide details regarding platform IDs, platform-specific encoding IDs, and platform-specific language IDs used in the 'name' table. For details regarding platform, encoding or language IDs used in the 'cmap' table, see [Encoding records and encodings](../../cmap/chunks/001-table-overview.md#encoding-records-and-encodings) in the 'cmap' table chapter.

<a id="platformIDs"></a>

<a id="platform-ids"></a>

### Platform IDs

The following platform IDs may be used in the 'name' table:

| Platform ID | Platform name | Platform-specific encoding IDs | Language IDs |
| --- | --- | --- | --- |
| 0 | Unicode | [Various](005-platform-encoding-and-language-ids.md#enc0) | None |
| 1 | Macintosh | [Script manager code](005-platform-encoding-and-language-ids.md#enc1) | [Various](005-platform-encoding-and-language-ids.md#lang1) |
| 3 | Windows | [Windows encoding](005-platform-encoding-and-language-ids.md#enc3) | [Various](005-platform-encoding-and-language-ids.md#windows-language-ids) |

Note that other platform IDs have been defined for use only in the 'cmap' table.

<a id="encLangIDs_platID0"></a>

<a id="unicode-platform-platform-id--0"></a>

### Unicode platform (platform ID = 0)

<a id="enc0"></a>

The following encoding IDs for the Unicode platform may be used in the 'name' table:

<a id="encID0_pid0"></a>

<a id="encID1_pid0"></a>

<a id="encID2_pid0"></a>

<a id="encID3_pid0"></a>

<a id="encID4_pid0"></a>

| Encoding ID | Description |
| --- | --- |
| 0 | Unicode 1.0 semantics—*deprecated* |
| 1 | Unicode 1.1 semantics—*deprecated* |
| 2 | ISO/IEC 10646 semantics—*deprecated* |
| 3 | Unicode 2.0 and onwards semantics, Unicode BMP only |
| 4 | Unicode 2.0 and onwards semantics, Unicode full repertoire |

Use of encoding IDs 0, 1 or 2 is deprecated. Either encoding ID 3 or 4 may be used for 'name' entries.

Note that other platform IDs have been defined for use only in the 'cmap' table. Also note that a new encoding ID for the Unicode platform may sometimes be assigned when new 'cmap' subtable formats are defined, and these may also be appropriate for use in the 'name' table. For example, when 'cmap' subtable formats 10 and 12 were added to the specification, encoding ID 4 was added as well.

There are no platform-specific language IDs defined for the Unicode platform. Language ID = 0 may be used for Unicode-platform strings, but this does not indicate any particular language. Language IDs greater than or equal to 0x8000 may be used together with language-tag records, as described above.

Strings for the Unicode platform must be encoded in UTF-16BE.

<a id="encLangIDs_platID1"></a>

<a id="macintosh-platform-platform-id--1"></a>

### Macintosh platform (platform ID = 1)

<a id="enc1"></a>

<a id="macintosh-encoding-ids-script-manager-codes"></a>

#### Macintosh encoding IDs (script manager codes)

The following encoding IDs are defined for use with the Macintosh platform:

<a id="encID0_pid1"></a>

<a id="encID17_pid1"></a>

<a id="encID1_pid1"></a>

<a id="encID18_pid1"></a>

<a id="encID2_pid1"></a>

<a id="encID19_pid1"></a>

<a id="encID3_pid1"></a>

<a id="encID20_pid1"></a>

<a id="encID4_pid1"></a>

<a id="encID21_pid1"></a>

<a id="encID5_pid1"></a>

<a id="encID22_pid1"></a>

<a id="encID6_pid1"></a>

<a id="encID23_pid1"></a>

<a id="encID7_pid1"></a>

<a id="encID24_pid1"></a>

<a id="encID8_pid1"></a>

<a id="encID25_pid1"></a>

<a id="encID9_pid1"></a>

<a id="encID26_pid1"></a>

<a id="encID10_pid1"></a>

<a id="encID27_pid1"></a>

<a id="encID11_pid1"></a>

<a id="encID28_pid1"></a>

<a id="encID12_pid1"></a>

<a id="encID29_pid1"></a>

<a id="encID13_pid1"></a>

<a id="encID30_pid1"></a>

<a id="encID14_pid1"></a>

<a id="encID31_pid1"></a>

<a id="encID15_pid1"></a>

<a id="encID32_pid1"></a>

<a id="encID16_pid1"></a>

| Encoding ID | Script | Encoding ID | Script |
| --- | --- | --- | --- |
| 0 | Roman | 17 | Malayalam |
| 1 | Japanese | 18 | Sinhalese |
| 2 | Chinese (Traditional) | 19 | Burmese |
| 3 | Korean | 20 | Khmer |
| 4 | Arabic | 21 | Thai |
| 5 | Hebrew | 22 | Laotian |
| 6 | Greek | 23 | Georgian |
| 7 | Russian | 24 | Armenian |
| 8 | RSymbol | 25 | Chinese (Simplified) |
| 9 | Devanagari | 26 | Tibetan |
| 10 | Gurmukhi | 27 | Mongolian |
| 11 | Gujarati | 28 | Geez |
| 12 | Odia | 29 | Slavic |
| 13 | Bangla | 30 | Vietnamese |
| 14 | Tamil | 31 | Sindhi |
| 15 | Telugu | 32 | Uninterpreted |
| 16 | Kannada |  |  |

Strings for the Macintosh platform (platform ID 1) use platform-specific single- or double-byte encodings according to the specified encoding ID for a given name record.

<a id="lang1"></a>

<a id="macintosh-language-ids"></a>

#### Macintosh language IDs

For information on Macintosh platform-specific language IDs, consult [Apple’s TrueType Reference Manual](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6name.html).

<a id="windows-platform-platform-id-3"></a>

### Windows platform (platform ID= 3)

<a id="enc3"></a>

<a id="windows-encoding-ids"></a>

#### Windows encoding IDs

The following encoding IDs are defined for use with the Windows platform:

<a id="encID0_pid3"></a>

<a id="encID1_pid3"></a>

<a id="encID2_pid3"></a>

<a id="encID3_pid3"></a>

<a id="encID4_pid3"></a>

<a id="encID5_pid3"></a>

<a id="encID6_pid3"></a>

<a id="encID7_pid3"></a>

<a id="encID8_pid3"></a>

<a id="encID9_pid3"></a>

<a id="encID10_pid3"></a>

| Encoding ID | Description |
| --- | --- |
| 0 | Symbol |
| 1 | Unicode BMP |
| 2 | ShiftJIS |
| 3 | PRC |
| 4 | Big5 |
| 5 | Wansung |
| 6 | Johab |
| 7 | Reserved |
| 8 | Reserved |
| 9 | Reserved |
| 10 | Unicode full repertoire |

Encoding IDs for platform 3 'name' entries should match the encoding IDs used for platform 3 subtables in the 'cmap' table. When building a Unicode font for Windows, the platform ID should be 3 and the encoding ID should be 1. When building a symbol font for Windows, the platform ID should be 3 and the encoding ID should be 0. If a font has records for encoding IDs 3, 4 or 5, the corresponding string data should be encoded using code pages 936, 950 and 949, respectively. Otherwise, all string data for platform 3 must be encoded in UTF-16BE.

> *Note:* Some legacy Traditional Chinese fonts had name entries for platform 3, encoding ID 4 (Big5) with some string data encoded using code page 950 but with the string data for name ID 2 (font subfamily) encoded, instead, in UTF-16BE. For example, this was the case in the MingLi font included in the Traditional Chinese edition of Windows 95. Some older software implementations, including Windows GDI, allow for this exception.

<a id="encLangIDs_platID3"></a>

<a id="lang3"></a>

<a id="windows-language-ids"></a>

#### Windows language IDs

For information on Windows platform-specific language IDs and corresponding BCP 47 language tags, see [[MS-LCID]: Windows Language Code Identifier (LCID) Reference](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f).
