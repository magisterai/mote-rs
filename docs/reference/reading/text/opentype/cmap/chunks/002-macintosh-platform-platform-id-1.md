# Macintosh platform (platform ID = 1)

Source: [Character to glyph index mapping](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-table-overview.md) · [Next](003-format-4-segment-mapping-to-delta-values.md)

<a id="macPlatform"></a>

<a id="macintosh-platform-platform-id--1"></a>

### Macintosh platform (platform ID = 1)

Older Macintosh versions required fonts to have a 'cmap' subtable for platform ID 1. For current Apple platforms, use of platform ID 1 is discouraged. See the ['name'](../../name/chunks/005-platform-encoding-and-language-ids.md#macintosh-encoding-ids-script-manager-codes) table chapter for details regarding encoding IDs defined for the Macintosh platform.

<a id="isoPlatform"></a>

<a id="iso-platform-platform-id--2"></a>

### ISO platform (platform ID = 2)

Use of this platform ID is deprecated.

The following encoding IDs are defined for use with the ISO platform:

<a id="encID0_pid2"></a>

<a id="encID1_pid2"></a>

<a id="encID2_pid2"></a>

| Code | ISO encoding |
| --- | --- |
| 0 | 7-bit ASCII |
| 1 | ISO 10646 |
| 2 | ISO 8859-1 |

<a id="windowsPlatform"></a>

<a id="windows-platform-platform-id--3"></a>

### Windows platform (platform ID = 3)

The Windows platform supports several encodings. When creating fonts for Windows, Unicode 'cmap' subtables should always be used—platform ID 3 with encoding ID 1 or encoding ID 10. See below for additional details.

The following encoding IDs are supported on the Windows platform:

| Platform ID | Encoding ID | Description |
| --- | --- | --- |
| 3 | 0 | Symbol |
| 3 | 1 | Unicode BMP |
| 3 | 2 | ShiftJIS |
| 3 | 3 | PRC |
| 3 | 4 | Big5 |
| 3 | 5 | Wansung |
| 3 | 6 | Johab |
| 3 | 7 | Reserved |
| 3 | 8 | Reserved |
| 3 | 9 | Reserved |
| 3 | 10 | Unicode full repertoire |

Fonts that support only Unicode BMP characters (U+0000 to U+FFFF) on the Windows platform must use encoding 1 with a format 4 subtable. This encoding must not be used to support Unicode supplementary-plane characters.

Fonts that support Unicode supplementary-plane characters (U+10000 to U+10FFFF) on the Windows platform must use encoding 10 with a format 12 subtable.

The symbol encoding was created to support fonts with arbitrary ornaments or symbols not supported in Unicode or other standard encodings. A format 4 subtable would be used, typically with up to 224 graphic characters assigned at code positions beginning with 0xF020. This corresponds to a sub-range within the Unicode Private-Use Area (PUA), though this is not a Unicode encoding. In legacy usage, some applications would represent the symbol characters in text using a single-byte encoding, and then map 0x20 to the OS/2.usFirstCharIndex value in the font. In new fonts, symbols or characters not in Unicode should be encoded using PUA code points in a Unicode 'cmap' subtable.

See the [Recommendations](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#cmap-table) chapter for additional information.

<a id="customPlatform_OTF"></a>

<a id="custom-platform-platform-id--4-and-otf-windows-nt-compatibility-mapping"></a>

### Custom platform (platform ID = 4) and OTF Windows NT compatibility mapping

Platform ID 4 is a legacy platform that was created to provide compatibility of older applications with OpenType fonts that had been adapted from older Type 1 fonts. This platform is not commonly used today, and should not be used in new fonts.

| ID | Custom encoding |
| --- | --- |
| 0-255 | OTF Windows NT compatibility mapping |

This 'cmap' platform provides a compatibility mechanism for non-Unicode applications that use the font as if it were Windows ANSI encoded. Non-Windows ANSI Type 1 fonts, such as Cyrillic and Central European fonts that Adobe shipped in the past, had “0” (Windows ANSI) recorded in the CharSet field of the .PFM file; Adobe Type Manager for Windows 9x ignored the CharSet altogether. Adobe provides this compatibility 'cmap' encoding in every OpenType font converted from a Type1 font in which the Encoding is not StandardEncoding.

When platform ID 4 is used, the encoding ID must be set to the Windows charset value (in the range 0 to 255, inclusive) present in the .PFM file of the original Type 1 font.

If a platform ID 4, encoding ID 0 – 255 'cmap' encoding is present in an OpenType font with CFF outlines, then the OTF font driver in Windows NT will: (a) superimpose the glyphs encoded at character codes 0-255 in the encoding on the corresponding Windows ANSI (code page 1252) Unicode values in the Unicode encoding it reports to the system; (b) add Windows ANSI (CharSet 0) to the list of CharSets supported by the font; and (c) consider the value of the encoding ID to be a Windows CharSet value and add it to the list of CharSets supported by the font. Note that the 'cmap' subtable needs to use Format 0 or 6 for its subtable, and the encoding needs to be identical to the CFF’s encoding.

<a id="cmap-subtable-formats"></a>

## 'cmap' subtable formats

<a id="language"></a>

<a id="use-of-the-language-field-in-cmap-subtables"></a>

### Use of the language field in 'cmap' subtables

All 'cmap' subtable formats include a *language* field. The language field must be set to zero for all 'cmap' subtables whose platform IDs are other than Macintosh (platform ID 1). For 'cmap' subtables whose platform IDs are Macintosh, set this field to the Macintosh language ID of the 'cmap' subtable plus one, or to zero if the 'cmap' subtable is not language-specific. For example, a Mac OS Turkish 'cmap' subtable must set this field to 18, since the Macintosh language ID for Turkish is 17. A Mac OS Roman 'cmap' subtable must set this field to 0, since Mac OS Roman is not a language-specific encoding.

<a id="format0"></a>

<a id="format-0-byte-encoding-table"></a>

### Format 0: Byte encoding table

Format 0 was the standard mapping subtable used on older Macintosh platforms but is not required on newer Apple platforms.

<a id="subtable_0"></a>

*'cmap' Subtable Format 0:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format number is set to 0. |
| uint16 | length | This is the length in bytes of the subtable. |
| uint16 | language | For requirements on use of the language field, see “[Use of the language field in 'cmap' subtables](002-macintosh-platform-platform-id-1.md#language)” in this document. |
| uint8 | glyphIdArray[256] | An array that maps character codes to glyph index values. |

This is a simple 1 to 1 mapping of character codes to glyph indices. The glyph set is limited to 256. If this format is used to index into a larger glyph set, only the first 256 glyphs will be accessible.

<a id="format2"></a>

<a id="format-2-high-byte-mapping-through-table"></a>

### Format 2: High byte mapping through table

This subtable format was created for “double-byte” encodings following national character code standards used for Japanese, Chinese, and Korean characters. These code standards use a mixed 8-/16-bit encoding. This format is not commonly used today.

In these mixed 8-/16-bit encodings, certain byte values signal the first byte of a 2-byte character. (These byte values are also valid as the second byte of a 2-byte character.) In addition, even for the 2-byte characters, the mapping of character codes to glyph index values depends heavily on the first byte. Consequently, the table begins with an array that maps the first byte to a SubHeader record. For 2-byte character codes, the SubHeader is used to map the second byte’s value into a sub-range of a glyph index array—a *sub-array*—, as described below. When processing mixed 8-/16-bit text, SubHeader 0 is special: it is used for single-byte character codes. When SubHeader 0 is used, a second byte is not needed; the single byte value is mapped through the specified sub-array.

<a id="subtable_2"></a>

*'cmap' Subtable Format 2:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format number is set to 2. |
| uint16 | length | This is the length in bytes of the subtable. |
| uint16 | language | For requirements on use of the language field, see “[Use of the language field in 'cmap' subtables](002-macintosh-platform-platform-id-1.md#language)” in this document. |
| uint16 | subHeaderKeys[256] | Array that maps high bytes into the subHeaders array: value is subHeaders index × 8. |
| SubHeader | subHeaders[ ] | Variable-length array of SubHeader records. |
| uint16 | glyphIdArray[ ] | Variable-length array containing sub-arrays used for mapping the low byte of 2-byte characters. |

The SubHeader record is structured as follows:

<a id="subHeadRec"></a>

*SubHeader Record:*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | firstCode | First valid low byte for this SubHeader. |
| uint16 | entryCount | Number of valid low bytes for this SubHeader. |
| int16 | idDelta | See text below. |
| uint16 | idRangeOffset | See text below. |

The firstCode and entryCount values specify a subrange that begins at firstCode and has a length equal to the value of entryCount. This subrange stays within the 0-255 range of the byte being mapped. Bytes outside of this subrange are mapped to glyph index 0 (missing glyph). The offset of the byte within this subrange is then used as index into a corresponding sub-array of glyphIdArray. This sub-array is also of length entryCount. The value of the idRangeOffset is the number of bytes past the actual location of the idRangeOffset word where the glyphIdArray element corresponding to firstCode appears.

Finally, if the value obtained from the sub-array is not 0 (which indicates the missing glyph), you should add idDelta to it in order to get the glyphIndex. The value idDelta permits the same sub-array to be used for several different subheaders. The idDelta arithmetic is modulo 65536. If the result after adding idDelta to the value from the sub-array is less than zero, add 65536 to obtain a valid glyph ID.

