# Item variation store table

Source: [Glyph definition](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-ligature-caret-list-table.md) · [Next](005-example-6-caretvalueformat3-table.md)

<a id="itemVarStoreTbl"></a>

<a id="item-variation-store-table"></a>

### Item variation store table

The format and processing of the ItemVariationStore table and its constituent formats is described in the chapter, [OpenType Font Variations Common Table Formats](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats). Specification of the interpolation algorithm used to derive values for particular variation instances is given in the chapter, [OpenType Font Variations Overview](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview).

The item variation store contains adjustment-delta values arranged in one or more sets of deltas that are referenced using *delta-set indices*. For values that require variation adjustment, a delta-set index is used to reference the particular variation data needed for that target value. Within the GDEF table, delta-set indices can be provided in [VariationIndex tables](../../chapter2/chunks/009-device-and-variationindex-tables.md#devVarIdxTbls) within caret value format 3 tables. For details on use of VariationIndex tables within the GDEF table, see discussion earlier in this chapter.

As needed, an item variation store within the GDEF table is used to store variation data not only for GDEF caret positions but also for values in the GPOS or JSTF tables that vary. Delta values used for the different layout tables may be interleaved and organized together within the one item variation store in any manner.

<a id="examples"></a>

<a id="gdef-table-examples"></a>

## GDEF table examples

The rest of this chapter describes examples of all the GDEF table formats. All the examples reflect unique parameters described below, but the samples provide a useful reference for building tables specific to other situations.

The examples have three columns showing hex data, source, and comments.

<a id="example-1-gdef-header"></a>

### Example 1: GDEF Header

Example 1 shows a GDEF Header definition with offsets to each of the main tables in GDEF.

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **GDEFHeader**  TheGDEFHeader | GDEFHeader table definition |
| 00010000 | 0x00010000 | major/minor version |
| 000C | GlyphClassDefTable | offset to GlyphClassDef table |
| 0026 | AttachListTable | offset to AttachList table |
| 0040 | LigCaretListTable | offset to LigCaretList table |
| 005A | MarkAttachClassDefTable | offset to Mark Attachment Class Definition Table |

<a id="example-2-glyphclassdef-table"></a>

### Example 2: GlyphClassDef table

The GlyphClassDef table in Example 2 specifies glyphs for each of the glyph classes defined in the GlyphClassDef enumeration.

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **ClassDefFormat2**  GlyphClassDefTable | ClassDef table definition |
| 0002 | 2 | classFormat |
| 0004 | 4 | classRangeCount |
|  | classRangeRecords[0] |  |
| 0024 | iGlyphID | startGlyphID |
| 0024 | iGlyphID | endGlyphID |
| 0001 | 1 | class: base glyphs |
|  | classRangeRecords[1] |  |
| 009F | ffiLigGlyphID | startGlyphID |
| 009F | ffiLigGlyphID | endGlyphID |
| 0002 | 2 | class: ligature glyphs |
|  | classRangeRecords[2] |  |
| 0058 | umlautAccentGlyphID | startGlyphID |
| 0058 | umlautAccentGlyphID | endGlyphID |
| 0003 | 3 | class: mark glyphs |
|  | classRangeRecords[3] |  |
| 018F | CurvedTailComponentGlyphID | startGlyphID |
| 018F | CurvedTailComponentGlyphID | endGlyphID |
| 0004 | 4 | class: component glyphs |

<a id="example-3-attachlist-table"></a>

### Example 3: AttachList table

In Example 3, the AttachList table enumerates the attachment points defined for two glyphs. A Coverage table identifies the glyphs: “a” and “e.” For each covered glyph, an AttachPoint table specifies the attachment point count and point indices: one point for the “a” glyph and two for the “e” glyph.

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **AttachList**  AttachListTable | AttachList table definition |
| 0012 | GlyphCoverage | offset to Coverage table |
| 0002 | 2 | glyphCount |
| 0008 | aAttachPoint | attachPointOffsets[0] |
| 000C | eAttachPoint | attachPointOffsets[1] |
|  | **AttachPoint**  aAttachPoint | AttachPoint table definition |
| 0001 | 1 | pointCount |
| 0012 | 18 | pointIndices[0] |
|  | **AttachPoint**  eAttachPoint | AttachPoint table definition |
| 0002 | 2 | pointCount |
| 000E | 14 | pointIndices[0] |
| 0017 | 23 | pointIndices[1] |
|  | **CoverageFormat1**  GlyphCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat |
| 0002 | 2 | glyphCount |
| 001C | aGlyphID | glyphArray[0] |
| 0020 | eGlyphID | glyphArray[1] |

<a id="example-4-ligcaretlist-ligglyph-and-caretvalueformat1-tables"></a>

### Example 4: LigCaretList, LigGlyph and CaretValueFormat1 tables

Example 4 defines a list of ligature carets. A Coverage table lists all the ligature glyphs that define caret positions. In this example, two ligatures are covered, “ffi” and “fi.” For each covered glyph, a LigGlyph table specifies the number of carets for the ligature and their coordinate values. The “fi” ligature defines one caret, positioned between the “f” and “i” components; the “ffi” ligature defines two, one positioned between the two “f” components and the other positioned between the “f” and “i.” The caret value tables shown here use format 1, in which values are specified in design units only.

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **LigCaretList**  LigCaretListTable | LigCaretList table definition |
| 0008 | LigCoverage | offset to Coverage table |
| 0002 | 2 | ligGlyphCount |
| 0010 | fiLigGlyph | ligGlyphOffsets[0] |
| 0014 | ffiLigGlyph | ligGlyphOffsets[1] |
|  | **CoverageFormat1**  LigCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat |
| 0002 | 2 | glyphCount |
| 009F | ffiLigGlyphID | glyphArray[0] |
| 00A5 | fiLigGlyphID | glyphArray[1] |
|  | **LigGlyph**  fiLigGlyph | LigGlyph table definition |
| 0001 | 1 | caretCount, equals the number of components - 1 |
| 000E | CaretFI | caretValueOffsets[0] |
|  | **LigGlyph**  ffiLigGlyph | LigGlyph table definition |
| 0002 | 2 | caretCount, equals the number of components - 1 |
| 0006 | CaretFFI1 | caretValueOffsets[0] |
| 000E | CaretFFI2 | caretValueOffsets[1] |
|  | **CaretValueFormat1**  CaretFI | CaretValue table definition |
| 0001 | 1 | format: design units only |
| 025B | 603 | coordinate (X or Y value) |
|  | **CaretValueFormat1**  CaretFFI1 | CaretValue table definition |
| 0001 | 1 | format: design units only |
| 025B | 603 | coordinate (X or Y value) |
|  | **CaretValueFormat1**  CaretFFI2 | CaretValue table definition |
| 0001 | 1 | format: design units only |
| 04B6 | 1206 | coordinate (X or Y value) |

<a id="example-5-caretvalueformat2-table"></a>

### Example 5: CaretValueFormat2 table

Example 5 shows a CaretValueFormat2 table that specifies a ligature caret coordinate in terms of a contour point index on a specific glyph. The final position of the caret depends on the location of the contour point on the glyph after hinting.

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **CaretValueFormat2**  Caret1 | CaretValue table definition |
| 0002 | 2 | format: contour point |
| 000D | 13 | caretValuePointIndex |

