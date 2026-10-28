# Example 12: Contextual positioning format 3

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-example-9-markmarkposformat1-subtable.md) · [Next](013-example-18-markarray-table-and-markrecord.md)

<a id="example12"></a>

<a id="example-12-contextual-positioning-format-3"></a>

### Example 12: Contextual positioning format 3

Example 12 illustrates [contextual positioning format 3](006-lookup-type-6-subtable-mark-to-mark-attachment-positioning.md#CPF3) using a [SequenceContextFormat3](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt3) subtable to lower the position of math signs in math equations consisting of a lowercase descender or x-height glyph, a math sign glyph, and any lowercase glyph. Format 3 is better to use for this context than the class-based Format 2 because the sets of covered glyphs for positions 0 and 2 overlap.

The LowerMathSignsSubtable contains offsets to three Coverage tables (XhtDescLCCoverage, MathSignCoverage, and LCCoverage), one for each position in the context glyph string. When the client finds the context in the text stream, it applies the SequenceLookupRecord data at position 1 and repositions the math sign.

*Example 12*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SequenceContextFormat3**  LowerMathSignsSubtable | SequenceContextFormat3 subtable definition |
| 0003 | 3 | format |
| 0003 | 3 | glyphCount |
| 0001 | 1 | seqLookupCount |
| 0010 | XhtDescLCCoverage | coverageOffsets[0] |
| 003C | MathSignCoverage | coverageOffsets[1] |
| 0044 | LCCoverage | coverageOffsets[2] |
|  | seqLookupRecords[0] |  |
| 0001 | 1 | sequenceIndex |
| 0001 | 1 | lookupListIndex |
|  | **CoverageFormat1**  XhtDescLCCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0014 | 20 | glyphCount |
| 0033 | LCaGlyphID | glyphArray[0] |
| 0035 | LCcGlyphID | glyphArray[1] |
| 0037 | LCeGlyphID | glyphArray[2] |
| 0039 | LCgGlyphID | glyphArray[3] |
| 003B | LCiGlyphID | glyphArray[4] |
| 003C | LCjGlyphID | glyphArray[5] |
| 003F | LCmGlyphID | glyphArray[6] |
| 0040 | LCnGlyphID | glyphArray[7] |
| 0041 | LCoGlyphID | glyphArray[8] |
| 0042 | LCpGlyphID | glyphArray[9] |
| 0043 | LCqGlyphID | glyphArray[10] |
| 0044 | LCrGlyphID | glyphArray[11] |
| 0045 | LCsGlyphID | glyphArray[12] |
| 0046 | LCtGlyphID | glyphArray[13] |
| 0047 | LCuGlyphID | glyphArray[14] |
| 0048 | LCvGlyphID | glyphArray[15] |
| 0049 | LCwGlyphID | glyphArray[16] |
| 004A | LCxGlyphID | glyphArray[17] |
| 004B | LCyGlyphID | glyphArray[18] |
| 004C | LCzGlyphID | glyphArray[19] |
|  | **CoverageFormat1**  MathSignCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0002 | 2 | glyphCount |
| 011E | EqualsSignGlyphID | glyphArray[0] |
| 012D | PlusSignGlyphID | glyphArray[1] |
|  | **CoverageFormat2**  LCCoverage | Coverage table definition |
| 0002 | 2 | format: ranges |
| 0001 | 1 | rangeCount |
|  | rangeRecords[0] |  |
| 0033 | LCaGlyphID | startGlyphID |
| 004C | LCzGlyphID | endGlyphID |
| 0000 | 0 | startCoverageIndex |

<a id="example13"></a>

<a id="example-13-sequencelookup-record"></a>

### Example 13: SequenceLookup record

The SequenceLookup record in Example 13 identifies a lookup to apply at the second glyph position in a context glyph string.

*Example 13*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SequenceLookupRecord**  seqLookupRecords[0] | SequenceLookupRecord definition |
| 0001 | 1 | sequenceIndex — for second glyph position |
| 0001 | 1 | lookupListIndex — apply this lookup to second glyph position |

<a id="example14"></a>

<a id="example-14-valueformat-and-valuerecord"></a>

### Example 14: ValueFormat and ValueRecord

Example 14 demonstrates how to specify positioning values in the GPOS table. Here, a SinglePosFormat1 subtable defines the ValueFormat and ValueRecord. The ValueFormat bit setting of 0x0099 says that the corresponding ValueRecord contains values for a glyph’s xPlacement and yAdvance. Device tables specify pixel adjustments for these values at font sizes from 11 ppem to 15 ppem.

*Example 14*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SinglePosFormat1**  OnesSubtable | SinglePos subtable definition |
| 0001 | 1 | format |
| 000E | Cov | Offset to Coverage table |
| 0099 | 0x0099 | valueFormat: X\_PLACEMENT + Y\_ADVANCE + X\_PLACEMENT\_DEVICE, Y\_ADVANCE\_DEVICE |
|  | valueRecord |  |
| 0050 | 80 | xPlacement |
| 00D2 | 210 | yAdvance |
| 0018 | XPlaDeviceTable | xPlaDeviceOffset |
| 0020 | YAdvDeviceTable | yAdvDeviceOffset |
|  | **CoverageFormat2**  Cov | Coverage table definition |
| 0002 | 2 | format: ranges |
| 0001 | 1 | rangeCount |
|  | rangeRecords[0] |  |
| 00C8 | 200 | startGlyphID |
| 00D1 | 209 | endGlyphID |
| 0000 | 0 | startCoverageIndex |
|  | **DeviceTableFormat1**  XPlaDeviceTable | Device Table definition |
| 000B | 11 | startSize |
| 000F | 15 | endSize — five delta values (sizes 11 to 15) |
| 0001 | 1 | deltaFormat: LOCAL\_2\_BIT\_DELTAS |
|  | 1 | deltaValue[0]: increase 11ppem by 1 pixel |
|  | 1 | deltaValue[1]: increase 12ppem by 1 pixel |
|  | 1 | deltaValue[2]: increase 13ppem by 1 pixel |
|  | 1 | deltaValue[3]: increase 14ppem by 1 pixel |
| 5540 | 1 | deltaValue[4]: increase 15ppem by 1 pixel |
|  | **DeviceTableFormat1**  YAdvDeviceTable | Device Table definition |
| 000B | 11 | startSize |
| 000F | 15 | endSize — five delta values (sizes 11 to 15) |
| 0001 | 1 | deltaFormat: LOCAL\_2\_BIT\_DELTAS |
|  | 1 | deltaValue[0]: increase 11ppem by 1 pixel |
|  | 1 | deltaValue[1]: increase 12ppem by 1 pixel |
|  | 1 | deltaValue[2]: increase 13ppem by 1 pixel |
|  | 1 | deltaValue[3]: increase 14ppem by 1 pixel |
| 5540 | 1 | deltaValue[4]: increase 15ppem by 1 pixel |

<a id="example15"></a>

<a id="example-15-anchorformat1-table"></a>

### Example 15: AnchorFormat1 table

Example 15 illustrates an Anchor table for the damma mark glyph in the Arabic script. Format1 is used to specify X and Y coordinate values in design units.

*Example 15*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **AnchorFormat1**  dammaMarkAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 00BD | 189 | xCoordinate |
| FF99 | -103 | yCoordinate |

<a id="example16"></a>

<a id="example-16-anchorformat2-table"></a>

### Example 16: AnchorFormat2 table

Example 16 shows an AnchorFormat2 table for an attachment point placed above a base glyph. With this format, the coordinate value for the Anchor depends on the final position of a specific contour point on the base glyph after hinting. The coordinates are specified in design units.

*Example 16*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **AnchorFormat2**  AboveBaseAnchor | Anchor table definition |
| 0002 | 2 | format: design units plus contour point |
| 0142 | 322 | xCoordinate |
| 0384 | 900 | yCoordinate |
| 000D | 13 | anchorPoint — glyph contour point index |

<a id="example17"></a>

<a id="example-17-anchorformat3-table"></a>

### Example 17: AnchorFormat3 table

Example 17 shows an AnchorFormat3 table that specifies an attachment point above a base glyph. Device tables modify the X and Y coordinates of the Anchor for the point size and resolution of the output font. Here, the Device tables define pixel adjustments for font sizes from 12 ppem to 17 ppem.

*Example 17*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **AnchorFormat3**  AboveBaseAnchor | Anchor table definition |
| 0003 | 3 | format: design units plus device table |
| 0117 | 279 | xCoordinate |
| 0515 | 1301 | yCoordinate |
| 000A | XDevice | xDeviceOffset (may be NULL) |
| 0014 | YDevice | yDeviceOffset (may be NULL) |
|  | **DeviceTableFormat2**  XDevice | Device Table definition |
| 000C | 12 | startSize |
| 0011 | 17 | endSize — six delta values (sizes 12 to 17) |
| 0002 | 2 | deltaFormat: LOCAL\_4\_BIT\_DELTAS |
|  | 1 | deltaValue[0]: increase 12ppem by 1 pixel |
|  | 1 | deltaValue[1]: increase 13ppem by 1 pixel |
|  | 1 | deltaValue[2]: increase 14ppem by 1 pixel |
| 1111 | 1 | deltaValue[3]: increase 15ppem by 1 pixel |
|  | 2 | deltaValue[4]: increase 16ppem by 1 pixel |
| 2200 | 2 | deltaValue[5]: increase 17ppem by 1 pixel |
|  | **DeviceTableFormat2**  YDevice | Device Table definition |
| 000C | 12 | startSize |
| 0011 | 17 | endSize — six delta values (sizes 12 to 17) |
| 0002 | 2 | deltaFormat: LOCAL\_4\_BIT\_DELTAS |
|  | 1 | deltaValue[0]: increase 12ppem by 1 pixel |
|  | 1 | deltaValue[1]: increase 13ppem by 1 pixel |
|  | 1 | deltaValue[2]: increase 14ppem by 1 pixel |
| 1111 | 1 | deltaValue[3]: increase 15ppem by 1 pixel |
|  | 2 | deltaValue[4]: increase 16ppem by 1 pixel |
| 2200 | 2 | deltaValue[5]: increase 17ppem by 1 pixel |

