# GDEF table structures

Source: [Glyph definition](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-overview.md) · [Next](003-ligature-caret-list-table.md)

<a id="gdef-table-structures"></a>

## GDEF table structures

<a id="gdefHeader"></a>

<a id="gdef-header"></a>

### GDEF Header

The GDEF table begins with a header that starts with a version number. Three versions are defined. Version 1.0 contains offsets to glyph class definition, attachment point list, ligature caret list and mark attachment class definition tables. Version 1.2 also includes an offset to a mark glyph sets table. Version 1.3 also includes an offset to an item variation store table.

Example 1 at the end of this chapter shows a GDEF Header table.

<a id="gdefHeader_10"></a>

*GDEF Header, version 1.0*

<a id="majorVersion_10"></a>

<a id="minorVersion_10"></a>

<a id="glyphClassDefOffset_10"></a>

<a id="attachListOffset_10"></a>

<a id="ligCaretListOffset_10"></a>

<a id="markAttachClassDefOffset_10"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the GDEF table, = 1. |
| uint16 | minorVersion | Minor version of the GDEF table, = 0. |
| Offset16 | glyphClassDefOffset | Offset to class definition table for glyph type, from beginning of GDEF header (may be NULL). |
| Offset16 | attachListOffset | Offset to attachment point list table, from beginning of GDEF header (may be NULL). |
| Offset16 | ligCaretListOffset | Offset to ligature caret list table, from beginning of GDEF header (may be NULL). |
| Offset16 | markAttachClassDefOffset | Offset to class definition table for mark attachment type, from beginning of GDEF header (may be NULL). |

<a id="gdefHeader_12"></a>

*GDEF Header, version 1.2*

<a id="majorVersion_12"></a>

<a id="minorVersion_12"></a>

<a id="glyphClassDefOffset_12"></a>

<a id="attachListOffset_12"></a>

<a id="ligCaretListOffset_12"></a>

<a id="markAttachClassDefOffset_12"></a>

<a id="markGlyphSetDefOffset_12"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the GDEF table, = 1. |
| uint16 | minorVersion | Minor version of the GDEF table, = 2. |
| Offset16 | glyphClassDefOffset | Offset to class definition table for glyph type, from beginning of GDEF header (may be NULL). |
| Offset16 | attachListOffset | Offset to attachment point list table, from beginning of GDEF header (may be NULL). |
| Offset16 | ligCaretListOffset | Offset to ligature caret list table, from beginning of GDEF header (may be NULL). |
| Offset16 | markAttachClassDefOffset | Offset to class definition table for mark attachment type, from beginning of GDEF header (may be NULL). |
| Offset16 | markGlyphSetsDefOffset | Offset to the table of mark glyph set definitions, from beginning of GDEF header (may be NULL). |

<a id="gdefHeader_13"></a>

*GDEF Header, version 1.3*

<a id="majorVersion_13"></a>

<a id="minorVersion_13"></a>

<a id="glyphClassDefOffset_13"></a>

<a id="attachListOffset_13"></a>

<a id="ligCaretListOffset_13"></a>

<a id="markAttachClassDefOffset_13"></a>

<a id="markGlyphSetDefOffset_13"></a>

<a id="itemVarStoreOffset_13"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the GDEF table, = 1. |
| uint16 | minorVersion | Minor version of the GDEF table, = 3. |
| Offset16 | glyphClassDefOffset | Offset to class definition table for glyph type, from beginning of GDEF header (may be NULL). |
| Offset16 | attachListOffset | Offset to attachment point list table, from beginning of GDEF header (may be NULL). |
| Offset16 | ligCaretListOffset | Offset to ligature caret list table, from beginning of GDEF header (may be NULL). |
| Offset16 | markAttachClassDefOffset | Offset to class definition table for mark attachment type, from beginning of GDEF header (may be NULL). |
| Offset16 | markGlyphSetsDefOffset | Offset to the table of mark glyph set definitions, from beginning of GDEF header (may be NULL). |
| Offset32 | itemVarStoreOffset | Offset to the item variation store table, from beginning of GDEF header (may be NULL). |

<a id="glyphClassDefTbl"></a>

<a id="glyph-class-definition-table"></a>

### Glyph class definition table

The GSUB and GPOS tables use the glyph class definition table (GlyphClassDef) to identify which glyph classes to adjust with lookups. This table uses the [class definition table](../../chapter2/chunks/005-coverage-table.md#class-definition-table) format (defined in the OpenType Layout Common Table Formats chapter), but with classes assigned to indicate specific glyph types, as follows:

<a id="glyphClassDefEnum"></a>

*GlyphClassDef enumeration*

<a id="glyphClassDefEnum_1"></a>

<a id="glyphClassDefEnum_2"></a>

<a id="glyphClassDefEnum_3"></a>

<a id="glyphClassDefEnum_4"></a>

| Class | Description |
| --- | --- |
| 1 | Base glyph (single character, spacing glyph) |
| 2 | Ligature glyph (multiple character, spacing glyph) |
| 3 | Mark glyph (non-spacing combining glyph) |
| 4 | Component glyph (part of single character, spacing glyph) |

The font developer does not have to classify every glyph in the font, but any glyph not assigned a class value falls into Class zero (0). For instance, class values might be useful for the Arabic glyphs in a font, but not for the Latin glyphs. Then the GlyphClassDef table will list only Arabic glyphs, and-by default-the Latin glyphs will be assigned to Class 0. Component glyphs can be put together to generate ligatures. A ligature can be generated by creating a glyph in the font that references the component glyphs, or outputting the component glyphs in the desired sequence. Component glyphs are not used in defining any GSUB or GPOS formats.

Example 2 at the end of this chapter defines a GlyphClassDef table with a sample glyph for each of the assigned classes.

<a id="attachmentPointListTbl"></a>

<a id="attachment-point-list-table"></a>

### Attachment point list table

The attachment point list table (AttachList) may be used to cache attachment point coordinates along with glyph bitmaps.

The table consists of an offset to a [Coverage](../../chapter2/chunks/005-coverage-table.md#coverageTbl) table listing all glyphs that define attachment points in the GPOS table, a count of the glyphs with attachment points, and an array of offsets to AttachPoint tables. The array lists the AttachPoint tables, one for each glyph in the Coverage table, in the same order as the Coverage Index.

<a id="attachListTable"></a>

*AttachList table*

<a id="coverage_al"></a>

<a id="glyphCount"></a>

<a id="attachPointOffsets"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of AttachList table. |
| uint16 | glyphCount | Number of glyphs with attachment points. |
| Offset16 | attachPointOffsets[glyphCount] | Array of offsets to AttachPoint tables, from beginning of AttachList table, in Coverage Index order. |

An AttachPoint table consists of a count of the attachment points on a single glyph and an array of contour indices of those points, listed in increasing numerical order.

Example 3 at the end of the chapter demonstrates an AttachList table that defines attachment points for two glyphs.

<a id="attachPointTable"></a>

*AttachPoint table*

<a id="pointCount"></a>

<a id="pointIndices"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | pointCount | Number of attachment points on this glyph. |
| uint16 | pointIndices[pointCount] | Array of contour point indices, in increasing numerical order. |

