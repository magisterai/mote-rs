# Device and VariationIndex tables

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-chained-sequence-context-format-2-class-based-glyph-contexts.md) · [Next](010-feature-variations.md)

<a id="devVarIdxTbls"></a>

<a id="device-and-variationindex-tables"></a>

## Device and VariationIndex tables

Device tables and VariationIndex tables are used to provide adjustments to font-unit values in GPOS, JSTF, GDEF or BASE tables, such as the X and Y coordinates of an attachment anchor position. Device tables are used only in non-variable fonts. VariationIndex tables are used only in variable fonts and are a variant format of the Device table. When values require adjustment data, the table containing that value will also include an offset to a Device table or VariationIndex table.

> *Note:*
> Because the same fields are used to provide an offset to a Device table or an offset to a VariationIndex table, Device tables and VariationIndex tables cannot both be used for a given positioning value. Device tables should only be used in non-variable fonts; VariationIndex tables can only be used in variable fonts.

Glyphs in a font are defined in design units specified by the font developer. Font scaling increases or decreases a glyph’s size and rounds it to the nearest whole pixel. However, precise glyph positioning often requires adjustment of these scaled and rounded values, particularly at small PPEM sizes. Hinting, applied to points in the glyph outline, is an effective solution to this problem, but it may require the font developer to redesign or re-hint glyphs.

Another solution, used by the GPOS, BASE, JSTF, and GDEF tables in non-variable fonts, is to use a Device table to specify correction values to adjust the scaled design units. A Device table applies the correction values to the range of sizes identified by StartSize and EndSize, which specify the smallest and largest pixel-per-em (ppem) sizes needing adjustment.

Because Device table adjustments often are very small (a pixel or two), the correction can be compressed into a 2-, 4-, or 8-bit representation per size. Two bits can represent a number in the range {-2, -1, 0, or 1}, four bits can represent a number in the range {-8 to 7}, and eight bits can represent a number in the range {-128 to 127}.

In variable fonts, X or Y font-unit values in GPOS, JSTF, or GDEF data may require adjustment for different variation instances within a font’s variation space. The variation data for this is contained in an ItemVariationStore table contained within the GDEF table. Similarly, values in a BASE table may require adjustment, and the variation data for this is contained in an ItemVariationStore table within the BASE table. The format of the ItemVariationStore is described in detail in the chapter, [OpenType Font Variations Common Table Formats](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats). It contains a number of delta values organized into sets that are referenced using a *delta-set index*. Data stored outside the ItemVariationStore provides delta-set indices for each of target items requiring variation. Within the GPOS, JSTF, GDEF and BASE tables, delta-set indices are stored in VariationIndex tables.

The Device and VariationIndex tables contain a DeltaFormat field that identifies the format of data contained. Format values 0x0001 to 0x0003 are used for Device tables, and indicate the format of delta adjustment values contained directly within the device table: signed 2-, 4,- or 8-bit values. A format value of 0x8000 is used for the VariationIndex table, and indicates that a delta-set index is used to reference delta data in an ItemVariationStore table.

<a id="deltaFormatValues"></a>

*DeltaFormat values*

<a id="dfv_1"></a>

<a id="dfv_2"></a>

<a id="dfv_3"></a>

<a id="dfv_8000"></a>

| Mask | Name | Description |
| --- | --- | --- |
| 0x0001 | LOCAL\_2\_BIT\_DELTAS | Signed 2-bit value, 8 values per uint16. |
| 0x0002 | LOCAL\_4\_BIT\_DELTAS | Signed 4-bit value, 4 values per uint16. |
| 0x0003 | LOCAL\_8\_BIT\_DELTAS | Signed 8-bit value, 2 values per uint16. |
| 0x8000 | VARIATION\_INDEX | VariationIndex table, contains a delta-set index pair. |
| 0x7FFC | Reserved | For future use — set to 0. |

The Device table includes an array of uint16 values (deltaValue) that stores the adjustment delta values in a packed representation. The 2-, 4-, or 8-bit signed values are packed into uint16 values starting with the most significant bits first. For example, using a DeltaFormat of 2 (4-bit values), an array of values equal to {1, 2, 3, -1} would be represented by a deltaValue entry 0x123F.

A single Device table provides delta information for one target value at a range of sizes. The deltaValue array lists the number of pixels to adjust specified X or Y values at each ppem size in the targeted range. In the array, the first index position specifies the number of pixels to add or subtract from the coordinate at the smallest ppem size that needs correction, the second index position specifies the number of pixels to add or subtract from the coordinate at the next ppem size, and so on for each ppem size in the range.

There is one delta value represented for each ppem size in the specified range. The number of uint16 elements required will depend on the number of values to be represented and the bit-size used. If the number of delta values times the bit-size used for each value is not a multiple of 16, then remaining low-order bits of the last uint16 element are set to zero.

<a id="deviceTable"></a>

*Device table*

<a id="startSize"></a>

<a id="endSize"></a>

<a id="deltaFormat_dt"></a>

<a id="deltaValues"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | startSize | Smallest size to correct, in ppem. |
| uint16 | endSize | Largest size to correct, in ppem. |
| uint16 | deltaFormat | Format of deltaValue array data: 0x0001, 0x0002, or 0x0003. |
| uint16 | deltaValue[ ] | Array of compressed data. |

Example 9 at the end of this chapter shows a Device table as could be used in specifying the minimum shift for superscripts in the [MATH](../../math/README.md) table.

In a variable font, the ItemVariationStore table uses a two-level organization for variation data: a store can have multiple *ItemVariationData* subtables, and each subtable has multiple delta-set rows. A delta-set index is a two-part index: an outer index that selects a particular item variation data subtable, and an inner index that selects a particular delta-set row within that subtable. A VariationIndex table specifies both the outer and inner portions of the delta-set index.

<a id="varidx"></a>

*VariationIndex table*

<a id="dsOuterIdx"></a>

<a id="dsInnerIdx"></a>

<a id="deltaFormat_vi"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | deltaSetOuterIndex | A delta-set outer index — used to select an item variation data subtable within the item variation store. |
| uint16 | deltaSetInnerIndex | A delta-set inner index — used to select a delta-set row within an item variation data subtable. |
| uint16 | deltaFormat | Format, = 0x8000. |

Note that the VariationIndex table is shorter than the Device table since it does not directly contain an array of delta data. Its format is similar to a Device table with an empty delta array. When applications get an offset to a Device or VariationIndex table, they should begin by reading the first three fields and then testing the DeltaFormat field to determine the interpretation of the first two fields and whether there is additional data to read.

