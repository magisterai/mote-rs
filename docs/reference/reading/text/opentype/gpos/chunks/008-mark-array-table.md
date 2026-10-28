# Mark array table

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-lookup-type-8-subtable-chained-contexts-positioning.md) · [Next](009-example-3-singleposformat2-subtable.md)

AnchorFormat1 consists of a format identifier and a pair of design-unit coordinates that specify the location of the anchor point. This format has the benefits of small size and simplicity, but the anchor point cannot be hinted to adjust its position for different device resolutions.

Example 15 at the end of this chapter uses AnchorFormat1.

<a id="anchorTbl_1"></a>

*AnchorFormat1 table*

<a id="anchorFormat_a1"></a>

<a id="xCoordinate_a1"></a>

<a id="yCoordinate_a1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| int16 | xCoordinate | Horizontal value, in design units. |
| int16 | yCoordinate | Vertical value, in design units. |

<a id="anchorTable_2"></a>

**Anchor format 2: design units plus contour point**

Like AnchorFormat1, AnchorFormat2 specifies a format identifier and a pair of design unit coordinates for the anchor point.

For fine-tuning the location of the anchor point, AnchorFormat2 also provides an index to a glyph contour point that is on the outline of a glyph. Hinting can be used to move the contour anchor point. In the rendered text, the anchor point will provide the final positioning data for a given ppem size.

Example 16 at the end of this chapter uses AnchorFormat2.

<a id="anchorTbl_2"></a>

*AnchorFormat2 table*

<a id="anchorFormat_a2"></a>

<a id="xCoordinate_a2"></a>

<a id="yCoordinate_a2"></a>

<a id="anchorPoint_a2"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| int16 | xCoordinate | Horizontal value, in design units. |
| int16 | yCoordinate | Vertical value, in design units. |
| uint16 | anchorPoint | Index to glyph contour point. |

<a id="anchorTable_3"></a>

**Anchor format 3: design units plus Device or VariationIndex tables**

Like AnchorFormat1, AnchorFormat3 specifies a format identifier and provides coordinates that locate an anchor point. And, like AnchorFormat2, it permits fine adjustments in variable fonts to the coordinate values. However, AnchorFormat3 uses Device tables, rather than a contour point, for this adjustment.

With a Device table, a client can adjust the position of the anchor point for any font size and device resolution. AnchorFormat3 can specify offsets to separate Device tables for the X and Y coordinates. If only one coordinate requires adjustment, the offset to the Device table for the other coordinate may be set to NULL.

In variable fonts, AnchorFormat3 is required to reference variation data to adjust anchor points for different variation instances, if needed. In this case, AnchorFormat3 specifies an offset to a VariationIndex table, which is a variant of the Device table used for variations. If no VariationIndex table is used for a particular anchor point X or Y coordinate, then that value is used for all variation instances. While separate VariationIndex table references are required for each value that requires variation, two or more values that require the same variation-data values can have offsets that point to the same VariationIndex table, and two or more VariationIndex tables can reference the same variation data entries.

Example 17 at the end of the chapter shows an AnchorFormat3 table.

<a id="anchorTbl_3"></a>

*AnchorFormat3 table*

<a id="anchorFormat_a3"></a>

<a id="xCoordinate_a3"></a>

<a id="yCoordinate_a3"></a>

<a id="xDeviceOffset_a3"></a>

<a id="yDeviceOffset_a3"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 3. |
| int16 | xCoordinate | Horizontal value, in design units. |
| int16 | yCoordinate | Vertical value, in design units. |
| Offset16 | xDeviceOffset | Offset to Device table (non-variable font) / VariationIndex table (variable font) for X coordinate, from beginning of Anchor table (may be NULL). |
| Offset16 | yDeviceOffset | Offset to Device table (non-variable font) / VariationIndex table (variable font) for Y coordinate, from beginning of Anchor table (may be NULL). |

<a id="markArrayTable"></a>

<a id="mark-array-table"></a>

#### Mark array table

The MarkArray table defines the class and the anchor point for a mark glyph. Three GPOS subtable types — mark-to-base attachment, mark-to-ligature attachment, and mark-to-mark attachment — use the MarkArray table to specify data for attaching marks.

The MarkArray table contains an array of MarkRecords. Each MarkRecord defines the class of the mark and an offset to the Anchor table that contains data for the mark.

A class value can be zero (0), but the MarkRecord must explicitly assign that class value. (This differs from the [Class Definition](../../chapter2/chunks/005-coverage-table.md#classDefTbl) table, in which all glyphs not assigned class values automatically belong to class 0.) The GPOS subtables that refer to MarkArray tables use the class assignments for indexing zero-based arrays that contain data for each mark class.

In Example 18 at the end of the chapter, a MarkArray table and two MarkRecords define two mark classes.

<a id="markArrayTbl"></a>

*MarkArray table*

<a id="markCount_ma"></a>

<a id="markRecords_ma"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | markCount | Number of MarkRecords. |
| MarkRecord | markRecords[markCount] | Array of MarkRecords, ordered by corresponding glyphs in the associated mark Coverage table. |

<a id="markRec"></a>

*MarkRecord*

<a id="markClass_mr"></a>

<a id="markAnchorOffset_mr"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | markClass | Class defined for the associated mark. |
| Offset16 | markAnchorOffset | Offset to Anchor table, from beginning of MarkArray table. |

<a id="examples"></a>

<a id="gpos-structure-examples"></a>

## GPOS structure examples

The rest of this chapter describes examples of all the GPOS subtable formats, including each of the three formats available for contextual positioning. All the examples reflect unique parameters described below, but the samples provide a useful reference for building subtables specific to other situations.

All the examples have three columns showing hex data, source, and comments.

<a id="example1"></a>

<a id="example-1-gpos-header"></a>

### Example 1: GPOS Header

Example 1 shows a typical GPOS Header table definition with offsets to a ScriptList, FeatureList, and LookupList.

*Example 1*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **GPOSHeader**  TheGPOSHeader | GPOSHeader table definition |
| 00010000 | 0x00010000 | major / minor version |
| 000A | TheScriptList | offset to ScriptList table |
| 001E | TheFeatureList | offset to FeatureList table |
| 002C | TheLookupList | offset to LookupList table |

<a id="example2"></a>

<a id="example-2-singleposformat1-subtable"></a>

### Example 2: SinglePosFormat1 subtable

Example 2 uses the SinglePosFormat1 subtable to lower the Y placement of subscript glyphs in a font. The LowerSubscriptsSubTable defines one Coverage table, called LowerSubscriptsCoverage, which lists one range of glyph indices for the numeral/numeric subscript glyphs. The subtable’s ValueFormat setting indicates that the ValueRecord specifies only the yPlacement value, lowering each subscript glyph by 80 design units.

*Example 2*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SinglePosFormat1**  LowerSubscriptsSubTable | SinglePos subtable definition |
| 0001 | 1 | format |
| 0008 | LowerSubscriptsCoverage | offset to Coverage table |
| 0002 | 0x0002 | valueFormat: Y\_PLACEMENT |
|  | ValueRecord |
| FFB0 | -80 | move Y position down |
|  | **CoverageFormat2**  LowerSubscriptsCoverage | Coverage table definition |
| 0002 | 2 | format: ranges |
| 0001 | 1 | rangeCount |
|  | rangeRecords[0] |  |
| 01B3 | ZeroSubscriptGlyphID | Start, first glyph ID |
| 01BC | NineSubscriptGlyphID | End, last glyph ID |
| 0000 | 0 | startCoverageIndex |

