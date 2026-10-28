# Ligature caret list table

Source: [Glyph definition](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-gdef-table-structures.md) · [Next](004-item-variation-store-table.md)

<a id="ligatureCaretListTbl"></a>

<a id="ligature-caret-list-table"></a>

### Ligature caret list table

The ligature caret list table (LigCaretList) defines caret positions for ligature glyphs in a font. The table consists of an offset to a Coverage table that lists all the ligature glyphs, a count of the defined ligatures, and an array of offsets to LigGlyph tables. The array lists the LigGlyph tables, one for each ligature in the Coverage table, in the same order as the Coverage Index.

Example 4 at the end of this chapter shows a LigCaretList table.

<a id="ligCaretListTable"></a>

*LigCaretList table*

<a id="coverage_lcl"></a>

<a id="ligGlyphCount"></a>

<a id="ligGlyphOffsets"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of LigCaretList table. |
| uint16 | ligGlyphCount | Number of ligature glyphs. |
| Offset16 | ligGlyphOffsets[ligGlyphCount] | Array of offsets to LigGlyph tables, from beginning of LigCaretList table, in Coverage index order. |

<a id="ligatureGlyphTbl"></a>

A ligature glyph table (LigGlyph) contains the caret coordinates for a single ligature glyph. The number of coordinate values, each defined in a separate table, equals the number of components in the ligature minus one.

The LigGlyph table consists of a count of the number of caret value tables defined for the ligature and an array of offsets to caret value tables.

Example 4 at the end of the chapter shows a LigGlyph table.

<a id="ligGlyphTable"></a>

*LigGlyph table*

<a id="caretCount"></a>

<a id="caretValueOffsets"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | caretCount | Number of caret value tables for this ligature (components - 1). |
| Offset16 | caretValueOffsets[caretCount] | Array of offsets to caret value tables, from beginning of LigGlyph table, in increasing coordinate order. |

<a id="caretValueTbls"></a>

Three caret value table formats are defined. One format uses design units to define the caret position. The other two formats use a contour point or (in non-variable fonts) a Device table to fine-tune a caret’s position at specific font sizes and device resolutions. In a variable font, the third format uses a [VariationIndex table](../../chapter2/chunks/009-device-and-variationindex-tables.md#device-and-variationindex-tables) (a variant of a Device table) to reference variation data for adjustment of the caret position for the current variation instance, as needed. Caret coordinates are either X or Y values, depending upon the text direction.

<a id="caretValueTbl1"></a>

The first format (CaretValueFormat1) consists of a format identifier followed by a single coordinate for the caret position, given in design units. This format has the benefits of small size and simplicity, but the coordinate value cannot be hinted for fine adjustments at different device resolutions.

Example 4 at the end of this chapter shows a CaretValueFormat1 table.

<a id="caretValueTable_1"></a>

*CaretValueFormat1 table: Design units only*

<a id="caretValuesFormat_1"></a>

<a id="coordinate_cv1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| int16 | coordinate | X or Y value, in design units. |

<a id="caretValueTbl2"></a>

The second format (CaretValueFormat2) specifies the caret coordinate in terms of a contour point index on a specific glyph. During font hinting, the contour point on the glyph outline could move. The point’s final position after hinting provides the final value for rendering a given font size.

The table contains a format identifier and a contour point index.

Example 5 at the end of this chapter demonstrates a CaretValueFormat2 table.

<a id="caretValueTable_2"></a>

*CaretValueFormat2 table: Contour point*

<a id="caretValuesFormat_2"></a>

<a id="caretValuePoint"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| uint16 | caretValuePointIndex | Contour point index on glyph. |

<a id="caretValueTbl3"></a>

The third format (CaretValueFormat3) also specifies the value in design units but, in non-variable fonts, it uses a Device table rather than a contour point to adjust the value. This format offers the advantage of fine-tuning the coordinate value for any device resolution. (For more information about Device tables, see [OpenType Layout Common Table Formats](../../chapter2/chunks/009-device-and-variationindex-tables.md#devVarIdxTbls).)

In variable fonts, CaretValueFormat3 is required to reference variation data to adjust caret positions for different variation instances, if needed. In this case, CaretValueFormat3 specifies an offset to a VariationIndex table, which is a variant of the Device table used for variations. If no VariationIndex table is used for a particular caret position value, then that value is used for all variation instances.

> *Note:* While separate VariationIndex table references are required for each value that requires variation, two or more values that require the same variation-data values can have offsets that point to the same VariationIndex table, and two or more VariationIndex tables can reference the same variation data entries.

The format consists of a format identifier, an X or Y value, and an offset to a Device or VariationIndex table.

Example 6 at the end of this chapter shows a CaretValueFormat3 table.

<a id="caretValueTable_3"></a>

*CaretValueFormat3 table: Design units plus Device or VariationIndex table*

<a id="caretValuesFormat_3"></a>

<a id="coordinate_cv3"></a>

<a id="devVarIdxOffset"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 3. |
| int16 | coordinate | X or Y value, in design units. |
| Offset16 | deviceOffset | Offset to Device table (non-variable font) / VariationIndex table (variable font) for X or Y value-from beginning of CaretValue table. |

<a id="markAttachClassDefTbl"></a>

<a id="mark-attachment-class-definition-table"></a>

### Mark attachment class definition table

A mark attachment class definition table is a [class definition table](../../chapter2/chunks/005-coverage-table.md#class-definition-table) (defined in the OpenType Layout Common Table Formats chapter) used to assign marks into different classes that can be used to control the effect of GSUB or GPOS lookups.

Example 7 in this document shows a mark attachment class definition table.

<a id="markGlyphSetsTbl"></a>

<a id="mark-glyph-sets-table"></a>

### Mark glyph sets table

Mark glyph sets are used in GSUB and GPOS lookups to filter which marks in a glyph sequence are considered or ignored. Mark glyph sets are defined in a MarkGlyphSets table, which contains offsets to individual sets each represented by a standard [Coverage](../../chapter2/chunks/005-coverage-table.md#coverageTbl) table.

<a id="markGlyphSetsTable"></a>

*MarkGlyphSets table*

<a id="markGlyphSetTableFormat"></a>

<a id="markGlyphSetCount"></a>

<a id="coverageOffsets"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| uint16 | markGlyphSetCount | Number of mark glyph sets defined. |
| Offset32 | coverageOffsets[markGlyphSetCount] | Array of offsets to mark glyph set coverage tables, from the start of the MarkGlyphSets table. |

Mark glyph sets are used for the same purpose as mark attachment classes, which is as filters for GSUB and GPOS lookups. Mark glyph sets differ from mark attachment classes, however, in that mark glyph sets may intersect as needed by the font developer. As for mark attachment classes, only one mark glyph set can be referenced in any given lookup.

> *Note:* The array of offsets for the Coverage tables uses Offset32, not Offset16.

