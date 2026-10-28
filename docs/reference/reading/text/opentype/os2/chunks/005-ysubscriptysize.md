# ySubscriptYSize

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-fstype.md) · [Next](006-ystrikeoutsize.md)

<a id="subys"></a>

<a id="ysubscriptysize"></a>

### ySubscriptYSize

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Subscript vertical font size. |
| Description: | The recommended vertical size in font design units for subscripts for this font. Should be > 0. |
| Comments: | If a font has two recommended sizes for subscripts, e.g. numerics and other, the numeric sizes should be stressed. This size field maps to the em size of the font being used for a subscript. The vertical font size specifies a font designer’s recommendation for vertical size of subscript glyphs associated with this font. If a font does not include all required subscript glyphs for an application, and the application can substitute glyphs by scaling the glyphs in a font or by substituting glyphs from another font, this field specifies the recommended nominal height for those subscript glyphs.    For example, if the em for a font is 2048 units and ySubScriptYSize is set to 205, then the vertical size for a simulated subscript glyph would be 1/10th the size of the normal glyph.   *Note:* This was implemented in the OS/2 platform as a signed value, even though only values > 0 would be expected. |

<a id="subxo"></a>

<a id="ysubscriptxoffset"></a>

### ySubscriptXOffset

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Subscript x offset. |
| Description: | The recommended horizontal offset in font design units for subscripts for this font. |
| Comments: | The ySubscriptXOffset field specifies a font designer’s recommended horizontal offset — from the glyph origin to the glyph origin of the subscript’s glyph — for subscript glyphs associated with this font. If a font does not include all required subscript glyphs for an application, and the application can substitute glyphs, this field specifies the recommended horizontal position from the glyph escapement point of the last glyph before the first subscript glyph. For upright glyphs, this value is usually zero; however, if the glyphs of a font have an incline (italic or slant), the reference point for subscript glyphs is usually adjusted to compensate for the angle of incline. |

<a id="subyo"></a>

<a id="ysubscriptyoffset"></a>

### ySubscriptYOffset

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Subscript y offset. |
| Description: | The recommended vertical offset in font design units from the baseline for subscripts for this font. |
| Comments: | The ySubscriptYOffset field specifies a font designer’s recommended vertical offset from the glyph baseline to the glyph baseline for subscript glyphs associated with this font. Values are expressed as a positive offset below the glyph baseline. If a font does not include all required subscript glyphs for an application, this field specifies the recommended vertical distance below the glyph baseline for alternate subscript glyphs supplied by the application. |

<a id="supxs"></a>

<a id="ysuperscriptxsize"></a>

### ySuperscriptXSize

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Superscript horizontal font size. |
| Description: | The recommended horizontal size in font design units for superscripts for this font. Should be > 0. |
| Comments: | If a font has two recommended sizes for superscripts, e.g., numerics and other, the numeric sizes should be stressed. This size field maps to the em size of the font being used for a superscript. The horizontal font size specifies a font designer’s recommended horizontal size for superscript glyphs associated with this font. If a font does not include all required superscript glyphs for an application, and the application can substitute glyphs by scaling the glyphs of a font or by substituting glyphs from another font, this field specifies the recommended nominal width for those superscript glyphs.    For example, if the em for a font is 2048 units and ySuperScriptXSize is set to 205, then the horizontal size for a simulated superscript glyph would be 1/10th the size of the normal glyph.  *Note:* This was implemented in the OS/2 platform as a signed value, even though only values > 0 would be expected. |

<a id="supys"></a>

<a id="ysuperscriptysize"></a>

### ySuperscriptYSize

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Superscript vertical font size. |
| Description: | The recommended vertical size in font design units for superscripts for this font. Should be > 0. |
| Comments: | If a font has two recommended sizes for superscripts, e.g., numerics and other, the numeric sizes should be stressed. This size field maps to the em size of the font being used for a superscript. The vertical font size specifies a font designer’s recommended vertical size for superscript glyphs associated with this font. If a font does not include all required superscript glyphs for an application, and the application can substitute glyphs by scaling the glyphs of a font or by substituting glyphs from another font, this field specifies the recommended nominal height for those superscript glyphs.    For example, if the em for a font is 2048 units and ySuperScriptYSize is set to 205, then the vertical size for a simulated superscript glyph would be 1/10th the size of the normal glyph.  *Note:* This was implemented in the OS/2 platform as a signed value, even though only values > 0 would be expected. |

<a id="supxo"></a>

<a id="ysuperscriptxoffset"></a>

### ySuperscriptXOffset

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Superscript x offset. |
| Description: | The recommended horizontal offset in font design units for superscripts for this font. |
| Comments: | The ySuperscriptXOffset field specifies a font designer’s recommended horizontal offset — from the glyph’s origin to the superscript glyph’s origin for the superscript glyphs associated with this font. If a font does not include all required superscript glyphs for an application, this field specifies the recommended horizontal position from the escapement point of the glyph before the first superscript glyph. For upright glyphs, this value is usually zero; however, if the glyphs of a font have an incline (italic or slant), the reference point for superscript glyphs is usually adjusted to compensate for the angle of incline. |

<a id="supyo"></a>

<a id="ysuperscriptyoffset"></a>

### ySuperscriptYOffset

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Superscript y offset. |
| Description: | The recommended vertical offset in font design units from the baseline for superscripts for this font. |
| Comments: | The ySuperscriptYOffset field specifies a font designer’s recommended vertical offset — from the glyph’s baseline to the superscript glyph’s baseline associated with this font. Values for this field are expressed as a positive offset above the glyph baseline. If a font does not include all required superscript glyphs for an application, this field specifies the recommended vertical distance above the glyph baseline for alternate superscript glyphs supplied by the application. |

