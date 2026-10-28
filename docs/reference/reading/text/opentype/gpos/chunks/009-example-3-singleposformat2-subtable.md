# Example 3: SinglePosFormat2 subtable

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-mark-array-table.md) · [Next](010-example-6-cursiveposformat1-subtable.md)

<a id="example3"></a>

<a id="example-3-singleposformat2-subtable"></a>

### Example 3: SinglePosFormat2 subtable

This example uses a SinglePosFormat2 subtable to adjust the spacing of three dash glyphs by different amounts. The em dash spacing changes by 10 units, the en dash spacing changes by 25 units, and spacing of the standard dash changes by 50 units.

The DashSpacingSubTable contains one Coverage table with three dash glyph indices, plus an array of ValueRecords, one for each covered glyph. The ValueRecords use the same ValueFormat to modify the xPlacement and xAdvance values of each glyph. The ValueFormat bit setting of 0x0005 is produced by adding the xPlacement and xAdvance bit settings.

*Example 3*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SinglePosFormat2**  DashSpacingSubTable | SinglePos subtable definition |
| 0002 | 2 | format |
| 0014 | DashSpacingCoverage | offset to Coverage table |
| 0005 | 0x0005 | ValueFormat: X\_PLACEMENT || X\_ADVANCE |
| 0003 | 3 | valueCount |
|  | valueRecords[0] | for dash glyph |
| 0032 | 50 | xPlacement |
| 0032 | 50 | xAdvance |
|  | valueRecords[1] | for en dash glyph |
| 0019 | 25 | xPlacement |
| 0019 | 25 | xAdvance |
|  | valueRecords[2] | for em dash glyph |
| 000A | 10 | xPlacement |
| 000A | 10 | xAdvance |
|  | **CoverageFormat1**  DashSpacingCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0003 | 3 | glyphCount |
| 004F | DashGlyphID | glyphArray[0] |
| 0125 | EnDashGlyphID | glyphArray[1] |
| 0129 | EmDashGlyphID | glyphArray[2] |

<a id="example4"></a>

<a id="example-4-pairposformat1-subtable"></a>

### Example 4: PairPosFormat1 subtable

Example 4 uses a PairPosFormat1 subtable to kern two glyph pairs - “Po” and “To” - by adjusting the xAdvance of the first glyph and the xPlacement of the second glyph. Two ValueFormats are defined, one for each glyph. The subtable contains a Coverage table that lists the index of the first glyph in each pair. It also contains an offset to a PairSet table for each covered glyph.

A PairSet table defines an array of PairValue records to specify all the glyph pairs that contain a covered glyph as their first component. In this example, the PairSet table has one PairValue record that identifies the second glyph in the “Po” pair and two ValueRecords, one for the first glyph and one for the second. The PairSet table also has one PairValue record that lists the second glyph in the “To” pair and two ValueRecords, one for each glyph.

*Example 4*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **PairPosFormat1**  PairKerningSubTable | PairPos subtable definition |
| 0001 | 1 | format |
| 001E | PairKerningCoverage | offset to Coverage table |
| 0004 | 0x0004 | valueFormat1: X\_ADVANCE only |
| 0001 | 0x0001 | ValueFormat2: X\_PLACEMENT only |
| 0002 | 2 | pairSetCount |
| 000E | PairSetTable | pairSetOffsets[0] |
| 0016 | PairSetTable | pairSetOffsets[1] |
|  | **PairSetTable**  PairSetTable | PairSet table definition |
| 0001 | 1 | pairValueCount |
|  | pairValueRecords[0] |  |
| 0059 | LowercaseOGlyphID | secondGlyph |
|  | valueRecord1 | ValueRecord for first glyph |
| FFE2 | -30 | xAdvance |
|  | valueRecord2 | ValueRecord for second glyph |
| FFEC | -20 | xPlacement |
|  | **PairSetTable**  PairSetTable | PairSet table definition |
| 0001 | 1 | pairValueCount |
|  | pairValueRecords[0] |  |
| 0059 | LowercaseOGlyphID | secondGlyph |
|  | valueRecord1 | ValueRecord for first glyph |
| FFD8 | -40 | xAdvance |
|  | valueRecord2 | ValueRecord for second glyph |
| FFE7 | -25 | xPlacement |
|  | **CoverageFormat1**  PairKerningCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0002 | 2 | glyphCount |
| 002D | UppercasePGlyphID | glyphArray[0] |
| 0031 | UppercaseTGlyphID | glyphArray[1] |

<a id="example5"></a>

<a id="example-5-pairposformat2-subtable"></a>

### Example 5: PairPosFormat2 subtable

The PairPosFormat2 subtable in this example defines pairs composed of two glyph classes. Two ClassDef tables are defined, one for each glyph class. The first glyph in each pair is in a class of lowercase glyphs with diagonal shapes (v, w, y), defined class 1 in the LowercaseClassDef table. The second glyph in each pair is in a class of punctuation glyphs (comma and period), defined in class 1 in the PunctuationClassDef table. The Coverage table only lists the indices of the glyphs in the LowercaseClassDef table since they occupy the first position in the pairs.

The subtable defines two Class1 records for the classes defined in LowecaseClassDef, including Class0. Each record, in turn, defines a Class2 record for each class defined in PunctuationClassDef, including class 0. The Class2 records specify the positioning adjustments for the glyphs.

The pairs are kerned by reducing the xAdvance of the first glyph by 50 design units. Because no positioning change applies to the second glyph, its ValueFormat2 field is set to 0, to indicate that Value2 is empty for each pair.

Since no pairs begin with class 0 or class 2 glyphs, all the ValueRecords referenced in class1Records[0] contain values of 0 or are empty. However, class1Records[1] does define an xAdvance value in its class2Records[1] for kerning all pairs that contain a class 1 glyph followed by a class 2 glyph.

*Example 5*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **PairPosFormat2**  PunctKerningSubTable | PairPos subtable definition |
| 0002 | 2 | format |
| 0018 | PunctKerningCoverage | offset to Coverage table |
| 0004 | 0x0004 | valueFormat1: X\_ADVANCE only |
| 0000 | 0 | ValueFormat2: no ValueRecord for second glyph |
| 0022 | LowercaseClassDef | offset to ClassDef1 table, for first class in pair |
| 0032 | PunctuationClassDef | offset to ClassDef2 table, for second class in pair |
| 0002 | 2 | Class1Count |
| 0002 | 2 | Class2Count |
|  | class1Records[0] | First Class1 record, for contexts beginning with class 0 |
|  | class2Records[0] | First Class2 record for class1Records[0]; valueFormat2 is zero, so no valueRecord2. |
|  | valueRecord1 |  |
| 0000 | 0 | xAdvance: no change for first glyph |
|  | class2Records[1] | no valueRecord2 |
|  | valueRecord1 |  |
| 0000 | 0 | xAdvance: no change for first glyph |
|  | class1Records[1] | for contexts beginning with class 1 |
|  | class2Records[0] | no contexts with class 0 as second glyph; no valueRecord2 |
|  | valueRecord1 |  |
| 0000 | 0 | xAdvance: no change for first glyph |
|  | class2Records[1] | contexts with class 1 as second glyph; no valueRecord2 |
|  | valueRecord1 |  |
| FFCE | -50 | xAdvance: move punctuation glyph left |
|  | **CoverageFormat1**  PunctKerningCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0003 | 3 | glyphCount |
| 0046 | LowercaseVGlyphID | glyphArray[0] |
| 0047 | LowercaseWGlyphID | glyphArray[1] |
| 0049 | LowercaseYGlyphID | glyphArray[2] |
|  | **ClassDefFormat2**  LowercaseClassDef | ClassDef table definition |
| 0002 | 2 | format: ranges |
| 0002 | 2 | classRangeCount |
|  | classRangeRecords[0] |  |
| 0046 | LowercaseVGlyphID | startGlyphID |
| 0047 | LowercaseWGlyphID | endGlyphID |
| 0001 | 1 | class |
|  | classRangeRecords[1] |  |
| 0049 | LowercaseYGlyphID | startGlyphID |
| 0049 | LowercaseYGlyphID | endGlyphID |
| 0001 | 1 | class |
|  | **ClassDefFormat2**  PunctuationClassDef | ClassDef table definition |
| 0002 | 2 | format: ranges |
| 0001 | 1 | classRangeCount |
|  | classRangeRecords[0] |  |
| 006A | PeriodPunctGlyphID | startGlyphID |
| 006B | CommaPunctGlyphID | endGlyphID |
| 0001 | 1 | class |

