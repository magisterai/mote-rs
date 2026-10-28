# usFirstCharIndex

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-achvendid.md) · [Next](010-uswindescent.md)

<a id="fci"></a>

<a id="usfirstcharindex"></a>

### usFirstCharIndex

|  |  |
| --- | --- |
| Format: | uint16 |
| Description: | The minimum Unicode index (character code) in this font, according to the 'cmap' subtable for platform ID 3 and platform- specific encoding ID 0 or 1. For most fonts supporting Win-ANSI or other character sets, this value would be 0x0020. This field cannot represent supplementary character values (codepoints greater than 0xFFFF). Fonts that support supplementary characters should set the value in this field to 0xFFFF if the minimum index value is a supplementary character. |

<a id="lci"></a>

<a id="uslastcharindex"></a>

### usLastCharIndex

|  |  |
| --- | --- |
| Format: | uint16 |
| Description: | The maximum Unicode index (character code) in this font, according to the 'cmap' subtable for platform ID 3 and encoding ID 0 or 1. This value depends on which character sets the font supports. This field cannot represent supplementary character values (codepoints greater than 0xFFFF). Fonts that support supplementary characters should set the value in this field to 0xFFFF. |

<a id="sta"></a>

<a id="stypoascender"></a>

### sTypoAscender

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Description: | The typographic ascender for this font. This field should be combined with the sTypoDescender and sTypoLineGap values to determine default line spacing.    This field is similar to the [ascender](../../hhea/chunks/001-hhea-table-and-opentype-font-variations.md#ascender) field in the 'hhea' table as well as to the [usWinAscent](009-usfirstcharindex.md#wa) field in this table. However, legacy platform implementations used those fields with platform-specific behaviors. As a result, those fields are constrained by backward-compatibility requirements, and they do not ensure consistent layout across implementations. The sTypoAscender, sTypoDescender and sTypoLineGap fields are intended to allow applications to lay out documents in a typographically correct and portable fashion.    The USE\_TYPO\_METRICS flag (bit 7) of the fsSelection field is used to choose between using sTypo\* values or usWin\* values for default line metrics. See [fsSelection](008-achvendid.md#fss) for additional details.    It is not a general requirement that sTypoAscender - sTypoDescender be equal to unitsPerEm. These values should be set to provide default line spacing appropriate for the primary languages the font is designed to support.    For CJK (Chinese, Japanese, and Korean) fonts that are intended to be used for vertical (as well as horizontal) layout, the required value for sTypoAscender is that which describes the top of the [ideographic em-box](https://learn.microsoft.com/en-us/typography/opentype/spec/baselinetags#ideoembox). For example, if the ideographic em-box of the font extends from coordinates 0,-120 to 1000,880 (that is, a 1000 × 1000 box set 120 design units below the Latin baseline), then the value of sTypoAscender must be set to 880. Failing to adhere to these requirements will result in incorrect vertical layout.    Also see the [Recommendations Section](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#tad) for more on this field. |

<a id="std"></a>

<a id="stypodescender"></a>

### sTypoDescender

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Description: | The typographic descender for this font. This field should be combined with the sTypoAscender and sTypoLineGap values to determine default line spacing.    This field is similar to the [descender](../../hhea/chunks/001-hhea-table-and-opentype-font-variations.md#descender) field in the 'hhea' table as well as to the [usWinDescent](010-uswindescent.md#wd) field in this table. However, legacy platform implementations used those fields with platform-specific behaviors. As a result, those fields are constrained by backward-compatibility requirements, and they do not ensure consistent layout across implementations. The sTypoAscender, sTypoDescender and sTypoLineGap fields are intended to allow applications to lay out documents in a typographically correct and portable fashion.    The USE\_TYPO\_METRICS flag (bit 7) of the fsSelection field is used to choose between using sTypo\* values or usWin\* values for default line metrics. See [fsSelection](008-achvendid.md#fss) for additional details.    It is not a general requirement that sTypoAscender - sTypoDescender be equal to unitsPerEm. These values should be set to provide default line spacing appropriate for the primary languages the font is designed to support.    For CJK (Chinese, Japanese, and Korean) fonts that are intended to be used for vertical (as well as horizontal) layout, the required value for sTypoDescender is that which describes the bottom of the [ideographic em-box](https://learn.microsoft.com/en-us/typography/opentype/spec/baselinetags#ideoembox). For example, if the ideographic em-box of the font extends from coordinates 0,-120 to 1000,880 (that is, a 1000 × 1000 box set 120 design units below the Latin baseline), then the value of sTypoDescender must be set to -120. Failing to adhere to these requirements will result in incorrect vertical layout.    Also see the [Recommendations Section](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#tad) for more on this field. |

<a id="tlg"></a>

<a id="stypolinegap"></a>

### sTypoLineGap

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Description: | The typographic line gap for this font. This field should be combined with the sTypoAscender and sTypoDescender values to determine default line spacing.    This field is similar to the [lineGap](../../hhea/chunks/001-hhea-table-and-opentype-font-variations.md#lineGap) field in the 'hhea' table. However, legacy platform implementations treat that field with platform-specific behaviors. As a result, that field is constrained by backward-compatibility requirements, and does not ensure consistent layout across implementations. The sTypoAscender, sTypoDescender and sTypoLineGap fields are intended to allow applications to lay out documents in a typographically correct and portable fashion.    The USE\_TYPO\_METRICS flag (bit 7) of the fsSelection field is used to choose between using sTypo\* values or usWin\* values for default line metrics. See [fsSelection](008-achvendid.md#fss) for additional details. |

<a id="wa"></a>

<a id="uswinascent"></a>

### usWinAscent

|  |  |
| --- | --- |
| Format: | UFWORD |
| Units: | Font design units |
| Description: | The “Windows ascender” metric. This should be used to specify the height above the baseline for a clipping region.    This is similar to the [sTypoAscender](009-usfirstcharindex.md#sta) field, and also to the [ascender](../../hhea/chunks/001-hhea-table-and-opentype-font-variations.md#ascender) field in the 'hhea' table. There are important differences between these, however.    In the Windows GDI implementation, the usWinAscent and usWinDescent values have been used to determine the size of the bitmap surface in the TrueType rasterizer. Windows GDI will clip any portion of a TrueType glyph outline that appears above the usWinAscent value. If any clipping is unacceptable, then the value should be set greater than or equal to yMax.    *Note:* This pertains to the default position of glyphs, not their final position in layout after data from the GPOS or 'kern' table has been applied. Also, this clipping behavior interacts with the [VDMX table](https://learn.microsoft.com/en-us/typography/opentype/spec/vdmx): if a VDMX table is present and there is data for the current device aspect ratio and rasterization size, then the VDMX data will supersede the usWinAscent and usWinDescent values.    Some legacy applications use the usWinAscent and usWinDescent values to determine default line spacing. This is strongly discouraged. The sTypo\* fields should be used for this purpose.    Note that some applications use either the usWin\* values or the sTypo\* values to determine default line spacing, depending on whether the USE\_TYPO\_METRICS flag (bit 7) of the fsSelection field is set. This could be useful to provide compatibility with legacy documents using older fonts, while also providing better and more-portable layout using newer fonts. See [fsSelection](008-achvendid.md#fss) for additional details.    Applications that use the sTypo\* fields for default line spacing can use the usWin\* values to determine the size of a clipping region. Some applications use a clipping region for editing scenarios to determine what portion of the display surface to re-draw when text is edited, or how large a selection rectangle to draw when text is selected. This is an appropriate use for the usWin\* values.    Early versions of this specification suggested that the usWinAscent value be computed as the yMax for all characters in the Windows “ANSI” character set. For new fonts, the value should be determined based on the primary languages the font is designed to support, and should take into consideration additional height that could be required to accommodate tall glyphs or mark positioning. |

