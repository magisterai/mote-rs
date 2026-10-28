# sxHeight

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-uswindescent.md) · [Next](012-usupperopticalpointsize.md)

<a id="xh"></a>

<a id="sxheight"></a>

### sxHeight

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Description: | This metric specifies the distance between the baseline and the approximate height of non-ascending lowercase letters measured in font design units. This value would normally be specified by a type designer but in situations where that is not possible, for example when a legacy font is being converted, the value may be set equal to the top of the unscaled and unhinted glyph bounding box of the glyph encoded at U+0078 (LATIN SMALL LETTER X). If no glyph is encoded in this position the field should be set to 0.    This metric, if specified, can be used in font substitution: the xHeight value of one font can be scaled to approximate the apparent size of another. |
| Version differences: | **Version 0, version 1:** This field was not defined in version 0 or version 1. If the size of a version 0 OS/2 table extends beyond the usWinDescent field, or if the size of a version 1 OS/2 table extends beyond the code page range fields, this additional data should be ignored.    **Version 2 and later:** This field was defined in version 2 of the OS/2 table. |

<a id="ch"></a>

<a id="scapheight"></a>

### sCapHeight

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Description: | This metric specifies the distance between the baseline and the approximate height of uppercase letters measured in font design units. This value would normally be specified by a type designer but in situations where that is not possible, for example when a legacy font is being converted, the value may be set equal to the top of the unscaled and unhinted glyph bounding box of the glyph encoded at U+0048 (LATIN CAPITAL LETTER H). If no glyph is encoded in this position the field should be set to 0.    This metric, if specified, can be used in systems that specify type size by capital height measured in millimeters. It can also be used as an alignment metric; the top of a drop capital, for instance, can be aligned to the sCapHeight metric of the first line of text. |
| Version differences: | **Version 0, version 1:** This field was not defined in version 0 or version 1. If the size of a version 0 OS/2 table extends beyond the usWinDescent field, or if the size of a version 1 OS/2 table extends beyond the code page range fields, this additional data should be ignored.    **Version 2 and later:** This field was defined in version 2 of the OS/2 table. |

<a id="dc"></a>

<a id="usdefaultchar"></a>

### usDefaultChar

|  |  |
| --- | --- |
| Format: | uint16 |
| Description: | This is the Unicode code point, in UTF-16 encoding, of a character that can be used for a default glyph if a requested character is not supported in the font. If the value of this field is zero, glyph ID 0 is to be used for the default character. This field cannot represent supplementary-plane character values (code points greater than 0xFFFF), and so applications are strongly discouraged from using this field. |
| Version differences: | **Version 0, version 1:** This field was not defined in version 0 or version 1. If the size of a version 0 OS/2 table extends beyond the usWinDescent field, or if the size of a version 1 OS/2 table extends beyond the code page range fields, this additional data should be ignored.    **Version 2 and later:** This field was defined in version 2 of the OS/2 table. |

<a id="bc"></a>

<a id="usbreakchar"></a>

### usBreakChar

|  |  |
| --- | --- |
| Format: | uint16 |
| Description: | This is the Unicode code point, in UTF-16 encoding, of a character that can be used as a default break character. The break character is used to separate words and justify text. Most fonts specify U+0020 SPACE as the break character. This field cannot represent supplementary-plane character values (code points greater than 0xFFFF), and so applications are strongly discouraged from using this field. |
| Version differences: | **Version 0, version 1:** This field was not defined in version 0 or version 1. If the size of a version 0 OS/2 table extends beyond the usWinDescent field, or if the size of a version 1 OS/2 table extends beyond the code page range fields, this additional data should be ignored.    **Version 2 and later:** This field was defined in version 2 of the OS/2 table. |

<a id="mc"></a>

<a id="usmaxcontext"></a>

### usMaxContext

|  |  |
| --- | --- |
| Format: | uint16 |
| Description: | The maximum length of a target glyph context for any feature in this font. For example, a font which has only a pair kerning feature should set this field to 2. If the font also has a ligature feature in which the glyph sequence “f f i” is substituted by the ligature “ffi”, then this field should be set to 3. This field could be useful to sophisticated line-breaking engines in determining how far they should look ahead to test whether something could change that affects the line breaking. For chaining contextual lookups, the length of the string (covered glyph) + (input sequence) + (lookahead sequence) should be considered. |
| Version differences: | **Version 0, version 1:** This field was not defined in version 0 or version 1. If the size of a version 0 OS/2 table extends beyond the usWinDescent field, or if the size of a version 1 OS/2 table extends beyond the code page range fields, this additional data should be ignored.    **Version 2 and later:** This field was defined in version 2 of the OS/2 table. |

<a id="lps"></a>

<a id="usloweropticalpointsize"></a>

### usLowerOpticalPointSize

|  |  |
| --- | --- |
| Format: | uint16 |
| Units: | TWIPs |
| Description: | This field is used for fonts with multiple optical styles.    This value is the lower value of the size range for which this font has been designed. The units for this field are TWIPs (one-twentieth of a point, or 1440 per inch). The value is inclusive — meaning that that font was designed to work best at this point size through, but not including, the point size indicated by usUpperOpticalPointSize. When used with other optical-size-variant fonts within a typographic family that also specify usLowerOpticalPointSize and usUpperOpticalPointSize values, it would be expected that another font has the usUpperOpticalPointSize field set to the same value as the value in this field, unless this font is designed for the lowest size range among the fonts in the family. The smallest font in an optical-size set should set this value to 0. When working across multiple optical-size-variant fonts within a typographic family, there should be no intentional gaps or overlaps in the ranges.    The usLowerOpticalPointSize value must be less than usUpperOpticalPointSize. The maximum valid value is 0xFFFE.    For fonts that were not designed for multiple optical-size variants, this field should be set to 0 (zero), and usUpperOpticalPointSize should be set to 0xFFFF.    *Note:* Use of this field has been superseded by the STAT table. See [Recommendations Section](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#OptSize) for more information. |
| Version differences: | **Versions 0 – 4:** This field was not defined in versions 0 – 4. If the size of an OS/2 table extends beyond the last field defined for the given version, this additional data should be ignored.    **Version 5:** This field was defined in version 5 of the OS/2 table. |

