# Example 4: LookupList table and Lookup table

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](011-featuretablesubstitution-table.md)

<a id="example-4-lookuplist-table-and-lookup-table"></a>

### Example 4: LookupList table and Lookup table

A continuation of Example 3, Example 4 shows three ligature lookups in the LookupList table. The first generates the “ffi” and “fi” ligatures; the second produces the “ffl”, “fl”, and “ff” ligatures; and the third generates the eszet ligature. Each lookup table defines an offset to a subtable that contains data for the ligature substitution.

*Example 4*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **LookupList**  TheLookupList | LookupList table definition |
| 0003 | 3 | lookupCount |
| 0008 | FfiFiLookup | offset to lookups[0] table, in design order |
| 0010 | FflFlFfLookup | offset to lookups[1] table |
| 0018 | EszetLookup | offset to lookups[2] table |
|  | **Lookup**  FfiFiLookup | lookups[0] table definition |
| 0004 | 4 | lookupType: ligature subst |
| 000C | 0x000C | lookupFlag: IGNORE\_LIGATURES, IGNORE\_MARKS |
| 0001 | 1 | subTableCount |
| 0018 | FfiFiSubtable | offset to FfiFi ligature substitution subtable |
|  | **Lookup**  FflFlFfLookup | lookups[1] table definition |
| 0004 | 4 | lookupType: ligature subst |
| 000C | 0x000C | lookupFlag: IGNORE\_LIGATURES, IGNORE\_MARKS |
| 0001 | 1 | subTableCount |
| 0028 | FflFlFfSubtable | offset to FflFlFf ligature substitution subtable |
|  | **Lookup**  EszetLookup | lookups[2] table definition |
| 0004 | 4 | lookupType: ligature subst |
| 000C | 0x000C | lookupFlag: IGNORE\_LIGATURES, IGNORE\_MARKS |
| 0001 | 1 | subTableCount |
| 0038 | EszetSubtable | offset to Eszet ligature substitution subtable |

<a id="example-5-coverageformat1-table-glyph-id-list"></a>

### Example 5: CoverageFormat1 table (glyph ID list)

Example 5 illustrates a Coverage table that lists the glyph IDs of all lowercase descender glyphs in a font. The table uses the list format instead of the range format because the glyph IDs for the descender glyphs are not consecutively ordered.

*Example 5*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **CoverageFormat1**  DescenderCoverage | Coverage table definition |
| 0001 | 1 | format: glyph ID list |
| 0005 | 5 | glyphCount |
| 0038 | gGlyphID | glyphArray[0], in glyph ID order |
| 003B | jGlyphID | glyphArray[1] |
| 0041 | pGlyphID | glyphArray[2] |
| 0042 | qGlyphID | glyphArray[3] |
| 004A | yGlyphID | glyphArray[4] |

<a id="example-6-coverageformat2-table-glyph-id-ranges"></a>

### Example 6: CoverageFormat2 table (glyph ID ranges)

Example 6 shows a Coverage table that defines ten numeral glyphs (0 through 9). The table uses the range format instead of the list format because the glyph IDs are ordered consecutively in the font. The StartCoverageIndex of zero (0) indicates that the first glyph ID, for the zero glyph, returns a Coverage Index of 0. The second glyph ID, for the numeral one (1) glyph, returns a Coverage Index of 1, and so on.

*Example 6*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **CoverageFormat2**  NumeralCoverage | Coverage table definition |
| 0002 | 2 | format: glyph ID ranges |
| 0001 | 1 | rangeCount |
|  | rangeRecords[0] |  |
| 004E | 0glyphID | startGlyphID |
| 0057 | 9glyphID | endGlyphID |
| 0000 | 0 | StartCoverageIndex, first CoverageIndex = 0 |

<a id="example-7-classdefformat1-table-class-array"></a>

### Example 7: ClassDefFormat1 table (class array)

The ClassDef table in Example 7 assigns class values to the lowercase glyphs in a font. The x-height glyphs are in Class 0, the ascender glyphs are in Class 1, and the descender glyphs are in Class 2. The array begins with the index for the lowercase “a” glyph.

*Example 7*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **ClassDefFormat1**  LowercaseClassDef | ClassDef table definition |
| 0001 | 1 | format: class array |
| 0032 | aGlyphID | startGlyph |
| 001A | 26 | glyphCount |
|  | classValueArray |  |
| 0000 | 0 | aGlyph, Xheight Class 0 |
| 0001 | 1 | bGlyph, Ascender Class 1 |
| 0000 | 0 | cGlyph, Xheight Class 0 |
| 0001 | 1 | dGlyph, Ascender Class 1 |
| 0000 | 0 | eGlyph, Xheight Class 0 |
| 0001 | 1 | fGlyph, Ascender Class 1 |
| 0002 | 2 | gGlyph, Descender Class 2 |
| 0001 | 1 | hGlyph, Ascender Class 1 |
| 0000 | 0 | iGlyph, Ascender Class 1 |
| 0002 | 2 | jGlyph, Descender Class 2 |
| 0001 | 1 | kGlyph, Ascender Class 1 |
| 0001 | 1 | lGlyph, Ascender Class 1 |
| 0000 | 0 | mGlyph, Xheight Class 0 |
| 0000 | 0 | nGlyph, Xheight Class 0 |
| 0000 | 0 | oGlyph, Xheight Class 0 |
| 0002 | 2 | pGlyph, Descender Class 2 |
| 0002 | 2 | qGlyph, Descender Class 2 |
| 0000 | 0 | rGlyph, Xheight Class 0 |
| 0000 | 0 | sGlyph, Xheight Class 0 |
| 0001 | 1 | tGlyph, Ascender Class 1 |
| 0000 | 0 | uGlyph, Xheight Class 0 |
| 0000 | 0 | vGlyph, Xheight Class 0 |
| 0000 | 0 | wGlyph, Xheight Class 0 |
| 0000 | 0 | xGlyph, Xheight Class 0 |
| 0002 | 2 | yGlyph, Descender Class 2 |
| 0000 | 0 | zGlyph, Xheight Class 0 |

<a id="example-8-classdefformat2-table-class-ranges"></a>

### Example 8: ClassDefFormat2 table (class ranges)

In Example 8, the ClassDef table assigns class values to four types of glyphs in the Arabic script: medium-height base glyphs, high base glyphs, very high base glyphs, and default mark glyphs. The table lists only Class 1, Class 2, and Class 3; all glyphs not explicitly assigned a class fall into Class 0.

The table uses the range format because the glyph IDs in each class are ordered consecutively in the font. In the ClassRange array, ClassRange definitions are ordered by the Start glyph index in each range. The indices of the high base glyphs, defined in ClassRange[0], are first in the font and have a class value of 2. ClassRange[1] defines all the very high base glyphs and assigns a class value of 3. ClassRange[2] contains all default mark glyphs; the class value is 1. Class 0 consists of all the medium-height base glyphs, which are not explicitly assigned a class value.

*Example 8*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **ClassDefFormat2**  GlyphHeightClassDef | Class table definition |
| 0002 | 2 | format: ranges |
| 0003 | 3 | classRangeCount |
|  | classRangeRecords[0] | ordered by startGlyphID |
| 0030 | tahGlyphID | startGlyphID — first glyph ID in the range |
| 0031 | dhahGlyphID | endGlyphID — last glyph ID in the range |
| 0002 | 2 | class: high base glyphs |
|  | classRangeRecords[1] |  |
| 0040 | cafGlyphID | startGlyphID |
| 0041 | gafGlyphID | endGlyphID |
| 0003 | 3 | class: very high base glyphs |
|  | classRangeRecords[2] |  |
| 00D2 | fathatanDefaultGlyphID | startGlyphID |
| 00D3 | dammatanDefaultGlyphID | endGlyphID |
| 0001 | 1 | class: default marks |

<a id="example-9-device-table"></a>

### Example 9: Device table

Example 9 defines the minimum extent value for a math script, using a Device table to adjust the value according to the size of the output font. Here, the Device table defines single-pixel adjustments for font sizes from 11 ppem to 15 ppem. The DeltaFormat is 1, which signifies a packed array of signed 2-bit values, eight values per uint16.

*Example 9*

| Hex Data | Source | Comment |
| --- | --- | --- |
|  | **DeviceTableFormat1**  MinCoordDeviceTable | Device Table definition |
| 000B | 11 | startSize: 11 ppem |
| 000F | 15 | endSize: 15 ppem |
| 0001 | 1 | deltaFormat: signed 2 bit value (8 values per uint16) |
|  | 1 | increase 11ppem by 1 pixel |
|  | 1 | increase 12ppem by 1 pixel |
|  | 1 | increase 13ppem by 1 pixel |
|  | 1 | increase 14ppem by 1 pixel |
| 5540 | 1 | increase 15ppem by 1 pixel |
