# Overview

Source: [PostScript information](https://learn.microsoft.com/en-us/typography/opentype/spec/post) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/post).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-version-3-0.md)

<a id="overview"></a>

## Overview

This table contains additional information needed to use OpenType™ fonts on PostScript printers. This includes certain data found in the FontInfo dictionary of Type 1 fonts and the PostScript names of all the glyphs. For more information about PostScript names, see the [Adobe Glyph List Specification](https://github.com/adobe-type-tools/agl-specification).

Versions 1.0, 2.0, and 2.5 of the 'post' table are used only in fonts with TrueType or CFF version 2 outline data. Fonts with TrueType or CFF version 2 data may also use a version 3.0 'post' table. OpenType fonts with CFF version 1 outline data must only use a version 3.0 'post' table.

<a id="header"></a>

## Header

The table begins as follows:

<a id="version"></a>

<a id="italicAngle"></a>

<a id="underlinePosition"></a>

<a id="underlineThickness"></a>

<a id="isFixedPitch"></a>

<a id="minMemType42"></a>

<a id="maxMemType42"></a>

<a id="minMemType1"></a>

<a id="maxMemType1"></a>

| Type | Name | Description |
| --- | --- | --- |
| Version16Dot16 | version | 0x00010000 for version 1.0 0x00020000 for version 2.0 0x00025000 for version 2.5 *(deprecated)* 0x00030000 for version 3.0 |
| Fixed | italicAngle | Italic angle in counter-clockwise degrees from the vertical. Zero for upright text, negative for text that leans to the right (forward). |
| FWORD | underlinePosition | Suggested y-coordinate of the top of the underline. |
| FWORD | underlineThickness | Suggested values for the underline thickness. In general, the underline thickness should match the thickness of the underscore character (U+005F LOW LINE), and should also match the strikeout thickness, which is specified in the [OS/2](../../os2/chunks/006-ystrikeoutsize.md#ss) table. |
| uint32 | isFixedPitch | Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally spaced (i.e. monospaced). |
| uint32 | minMemType42 | Minimum memory usage when an OpenType font is downloaded. |
| uint32 | maxMemType42 | Maximum memory usage when an OpenType font is downloaded. |
| uint32 | minMemType1 | Minimum memory usage when an OpenType font is downloaded as a Type 1 font. |
| uint32 | maxMemType1 | Maximum memory usage when an OpenType font is downloaded as a Type 1 font. |

> *Note:* The PostScript language defines the UnderLinePosition FontInfo key as specifying the distance from the baseline to the center of the underline. This definition is not used in the 'post' table.

The last four entries in the table are present because PostScript drivers can do better memory management if the virtual memory (VM) requirements of a downloadable OpenType font are known before the font is downloaded. This information should be supplied if known. If it is not known, set the value to zero. The driver will still work but will be less efficient.

Maximum memory usage is minimum memory usage plus maximum runtime memory use. Maximum runtime memory use depends on the maximum band size of any bitmap potentially rasterized by the font scaler. Runtime memory usage could be calculated by rendering characters at different point sizes and comparing memory use.

If the version is 1.0 or 3.0, the table ends here. The additional entries for versions 2.0 and 2.5 are shown below. A version 4.0 is defined by Apple for use in their platforms (see [Apple’s specification](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html)), but is not supported in OpenType.

<a id="version-10"></a>

## Version 1.0

This version is used in order to supply PostScript glyph names when the font file contains exactly the 258 glyphs in the standard Macintosh TrueType font file (see 'post' Format 1 in [Apple’s specification](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html) for a list of the 258 Macintosh glyph names), and the font does not otherwise supply glyph names. As a result, the glyph names are taken from the system with no storage required by the font.

<a id="version-20"></a>

## Version 2.0

Version 2.0 is used for fonts that use glyph names that are not in the set of Macintosh glyph names. A given font may map some of its glyphs to the standard Macintosh glyph names, and some to its own custom names. A version 2.0 'post' table can be used in fonts with TrueType or CFF version 2 outlines.

For version 2.0, the following fields are appended at the end of the header:

| Type | Name | Description |
| --- | --- | --- |
| uint16 | numGlyphs | Number of glyphs (this should be the same as numGlyphs in 'maxp' table). |
| uint16 | glyphNameIndex[numGlyphs] | Array of indices into the string data. See below for details. |
| uint8 | stringData[variable] | Storage for the string data. |

This font file contains glyphs not in the standard Macintosh set, or the ordering of the glyphs in the font file differs from the standard Macintosh set. The glyphNameIndex array maps glyph IDs to a glyph name index. If the glyph name index is between 0 and 257 (inclusive), treat that index as a glyph index in the Macintosh standard glyph set and use the Macintosh glyph name. If the glyph name index is between 258 and 65535, then subtract 258 and use that to index into the list of Pascal strings at the end of the table.

For example, suppose glyphNameIndex[302] (for glyph ID 302) is 217: since that glyph name index is less than 258, the glyph name is the Macintosh glyph name for glyph ID 217. Suppose glyphNameIndex[408] is 262: subtracting 258, the difference is 4; the glyph name for that glyph is the fifth string (index 4, base 0) in the string data.

The string data extends from the last version 2.0 header field to the end of the table. Strings are in Pascal string format, meaning that the first byte of a given string is a length: the number of characters in that string. The length byte is not included; for example, a length byte of 8 indicates that the 8 bytes following the length byte comprise the string character data. To find the string for a given glyph name index, start with the first length byte, advance that number of bytes to find the length byte for the next string entry, and so on.

Glyph name strings are encoded in ASCII. Valid characters are limited to A–Z, a–z, 0–9, “.” (FULL STOP), and “\_” (LOW LINE). Names must be no longer than 63 characters; some older implementations can assume a length limit of 31 characters.

If you do not want to associate a PostScript name with a particular glyph, use 0, which refers to the name *.notdef*, as the glyphNameIndex entry for that glyph ID.

<a id="version-25"></a>

## Version 2.5

Version 2.5 of the 'post' table is deprecated.

This version provides a space-saving table for fonts containing TrueType outlines which contain a pure subset of, or a simple reordering of, the standard Macintosh glyph set.

| Type | Name | Description |
| --- | --- | --- |
| uint16 | numGlyphs | Number of glyphs. |
| int8 | offset[numGlyphs] | Difference between the glyph index and the standard order of the glyph. |

This version has been used in some legacy fonts with TrueType outlines that contain only glyphs in the standard Macintosh glyph set but that have those glyphs arranged in a non-standard order or that are missing some glyphs. The table contains one byte for each glyph in the font file. The byte is treated as a signed offset that maps the glyph index used in the font into the standard glyph index. For example, assuming that the font contains the three glyphs A, B, and C which are the 37th, 38th, and 39th glyphs in the standard ordering, the 'post' table would contain the bytes +36, +36, +36.

