# usUpperOpticalPointSize

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-sxheight.md)

<a id="ups"></a>

<a id="usupperopticalpointsize"></a>

### usUpperOpticalPointSize

|  |  |
| --- | --- |
| Format: | uint16 |
| Units: | TWIPs |
| Description: | This field is used for fonts with multiple optical styles.    This value is the upper value of the size range for which this font has been designed. The units for this field are TWIPs (one-twentieth of a point, or 1440 per inch). The value is exclusive — meaning that that font was designed to work best below this point size down to the usLowerOpticalPointSize threshold. When used with other optical-size-variant fonts within a typographic family that also specify usLowerOpticalPointSize and usUpperOpticalPointSize values, it would be expected that another font has the usLowerOpticalPointSize field set to the same value as the value in this field, unless this font is designed for the highest size range among the fonts in the family. The largest font in an optical-size set should set this value to 0xFFFF, which is interpreted as infinity. When working across multiple optical-size-variant fonts within a typographic family, there should be no intentional gaps or overlaps left in the ranges.    The usUpperOpticalPointSize value must be greater than usLowerOpticalPointSize. The minimum valid value for this field is 2 (two). The largest possible inclusive point size represented by this field is 3276.65 points; any higher values would be represented as infinity.    For fonts that were not designed for multiple optical-size variants, this field should be set to 0xFFFF, and usLowerOpticalPointSize should be set to 0 (zero).    *Note:* Use of this field has been superseded by the STAT table. See [Recommendations Section](https://learn.microsoft.com/en-us/typography/opentype/spec/recom#OptSize) for more information. |
| Version differences: | **Versions 0 – 4:** This field was not defined in versions 0 – 4. If the size of an OS/2 table extends beyond the last field defined for the given version, this additional data should be ignored.    **Version 5:** This field was defined in version 5 of the OS/2 table. |

<a id="os2-table-and-opentype-font-variations"></a>

## OS/2 Table and OpenType Font Variations

In variable fonts, default line metrics should always be set using the sTypoAscender, sTypoDescender and sTypoLineGap values, and the USE\_TYPO\_METRICS flag in the fsSelection field should be set. The ascender, descender and lineGap fields in the 'hhea' table should be set to the same values as sTypoAscender, sTypoDescender and sTypoLineGap. The usWinAscent and usWinDescent fields should be used to specify a recommended clipping rectangle.

In a variable font, various font-metric values within the OS/2 table might need to be adjusted for different variation instances. Variation data for OS/2 entries can be provided in the [metrics variations (MVAR)](https://learn.microsoft.com/en-us/typography/opentype/spec/mvar) table. Different OS/2 entries are associated with particular variation data in the MVAR table using value tags, as follows:

<a id="cpht"></a>

<a id="hasc"></a>

<a id="hdsc"></a>

<a id="hlgp"></a>

<a id="xhgt"></a>

<a id="hcla"></a>

<a id="hcld"></a>

<a id="stro"></a>

<a id="strs"></a>

<a id="sbxo"></a>

<a id="sbxs"></a>

<a id="sbyo"></a>

<a id="sbys"></a>

<a id="spxo"></a>

<a id="spxs"></a>

<a id="spyo"></a>

<a id="spys"></a>

| OS/2 entry | Tag |
| --- | --- |
| sCapHeight | 'cpht' |
| sTypoAscender | 'hasc' |
| sTypoDescender | 'hdsc' |
| sTypoLineGap | 'hlgp' |
| sxHeight | 'xhgt' |
| usWinAscent | 'hcla' |
| usWinDescent | 'hcld' |
| yStrikeoutPosition | 'stro' |
| yStrikeoutSize | 'strs' |
| ySubscriptXOffset | 'sbxo' |
| ySubScriptXSize | 'sbxs' |
| ySubscriptYOffset | 'sbyo' |
| ySubscriptYSize | 'sbys' |
| ySuperscriptXOffset | 'spxo' |
| ySuperscriptXSize | 'spxs' |
| ySuperscriptYOffset | 'spyo' |
| ySuperscriptYSize | 'spys' |

> *Note:* The usWeightClass and usWidthClass values are not adjusted by variation data since these correspond to 'wght' and 'wdth' variation axes that can be used to define a font’s variation space. Appropriate usWeightClass and usWidthClass values for a variation instance can be derived from 'wght' and 'wdth' user coordinates that are used to select a particular variation instance. For 'wdth' values greater than 200, the usWidthClass value is clamped to 9. See the discussion of the 'wght' and 'wdth' axes in the [OpenType Design-Variation Axis Tag Registry](https://learn.microsoft.com/en-us/typography/opentype/spec/dvaraxisreg) for details on the relationship between these OS/2 fields and the corresponding design axes.

> *Note:* The usLowerOpticalPointSize and usUpperOpticalPointSize values are not adjusted by variation data. These values (now superseded by the STAT table) are used to indicate a range of sizes for which a given font has been designed. It is assumed that variation that targets different sizes will be implemented using the 'opsz' variation axis. If a variable font supports 'opsz' as an axis of variation, then the usLowerOpticalPointSize and usUpperOpticalPointSize fields can be set to the same values as the minValue and maxValue fields for the 'opsz' axis in the 'fvar' table.

To have variable line metrics in a variable font, the 'hasc', 'hdsc' and 'hlgp' value tags should be used in the MVAR table to vary the ascender, descender and line gap values from defaults specified in the sTypoAscender, sTypoDescender and sTypoLineGap fields. The 'hcla' and 'hcld' value tags can be used in addition to vary the size of a clipping region from the default specified in the winAscent and winDescent fields. Other metrics can be varied using value tags listed above.

For general information on OpenType Font Variations, see the chapter, [OpenType Font Variations Overview](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview).
