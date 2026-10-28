# 'hhea' Table and OpenType Font Variations

Source: [Horizontal header](https://learn.microsoft.com/en-us/typography/opentype/spec/hhea) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/hhea).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

<a id="hheaHeader"></a>

This table contains information for horizontal layout. The values in the minRightSidebearing, minLeftSideBearing and xMaxExtent should be computed using *only* glyphs that have contours. Glyphs with no contours should be ignored for the purposes of these calculations. All reserved areas must be set to 0.

<a id="majorVersion"></a>

<a id="minorVersion"></a>

<a id="ascender"></a>

<a id="descender"></a>

<a id="lineGap"></a>

<a id="advanceWidthMax"></a>

<a id="minLeftSideBearing"></a>

<a id="minRightSideBearing"></a>

<a id="xMaxExtent"></a>

<a id="caretSlopeRise"></a>

<a id="caretSlopeRun"></a>

<a id="caretOffset"></a>

<a id="metricDataFormat"></a>

<a id="numberOfHMetrics"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version number of the horizontal header table — set to 1. |
| uint16 | minorVersion | Minor version number of the horizontal header table — set to 0. |
| FWORD | ascender | Typographic ascent—see remarks below. |
| FWORD | descender | Typographic descent—see remarks below. |
| FWORD | lineGap | Typographic line gap. Negative lineGap values are treated as zero in some legacy platform implementations. |
| UFWORD | advanceWidthMax | Maximum advance width value in 'hmtx' table. |
| FWORD | minLeftSideBearing | Minimum left sidebearing value in 'hmtx' table for glyphs with contours (empty glyphs should be ignored). |
| FWORD | minRightSideBearing | Minimum right sidebearing value; calculated as min(aw - (lsb + xMax - xMin)) for glyphs with contours (empty glyphs should be ignored). |
| FWORD | xMaxExtent | Max(lsb + (xMax - xMin)). |
| int16 | caretSlopeRise | Used to calculate the slope of the cursor (rise/run); 1 for vertical. |
| int16 | caretSlopeRun | 0 for vertical. |
| int16 | caretOffset | The amount by which a slanted highlight on a glyph needs to be shifted to produce the best appearance. Set to 0 for non-slanted fonts |
| int16 | (reserved) | set to 0 |
| int16 | (reserved) | set to 0 |
| int16 | (reserved) | set to 0 |
| int16 | (reserved) | set to 0 |
| int16 | metricDataFormat | 0 for current format. |
| uint16 | numberOfHMetrics | Number of hMetric entries in 'hmtx' table |

The ascender, descender and linegap values in this table are Apple specific; see [Apple's specification](http://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6hhea.html) for details regarding Apple platforms. The [sTypoAscender](../../os2/chunks/009-usfirstcharindex.md#stypoascender), [sTypoDescender](../../os2/chunks/009-usfirstcharindex.md#stypodescender) and [sTypoLineGap](../../os2/chunks/009-usfirstcharindex.md#stypolinegap) fields in the OS/2 table are used on the Windows platform, and are recommended for new text-layout implementations. Font developers should evaluate behavior in target applications that may use fields in this table or in the OS/2 table to ensure consistent layout. See the descriptions of the OS/2 fields for additional details.

<a id="fontvar"></a>

<a id="hhea-table-and-opentype-font-variations"></a>

## 'hhea' Table and OpenType Font Variations

In a variable font, various font-metric values within the horizontal header table may need to be adjusted for different variation instances. Variation data for 'hhea' entries can be provided in the [metrics variations (MVAR)](https://learn.microsoft.com/en-us/typography/opentype/spec/mvar) table. Different 'hhea' entries are associated with particular variation data in the MVAR table using value tags, as follows:

<a id="hcof"></a>

<a id="hcrs"></a>

<a id="hcrn"></a>

| 'hhea' entry | Tag |
| --- | --- |
| caretOffset | 'hcof' |
| caretSlopeRise | 'hcrs' |
| caretSlopeRun | 'hcrn' |

For general information on OpenType Font Variations, see the chapter, [OpenType Font Variations Overview](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview).
