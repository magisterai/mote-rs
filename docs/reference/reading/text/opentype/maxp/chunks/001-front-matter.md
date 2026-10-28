# Front matter

Source: [Maximum profile](https://learn.microsoft.com/en-us/typography/opentype/spec/maxp) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/maxp).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

This table establishes the memory requirements for this font. Fonts with CFF or CFF2 outlines must use Version 0.5 of this table, specifying only the numGlyphs field. Fonts with TrueType outlines must use Version 1.0 of this table, where all data is required.

<a id="maxpHeader_05"></a>

*Version 0.5*

| Type | Name | Description |
| --- | --- | --- |
| Version16Dot16 | version | 0x00005000 for version 0.5 |
| uint16 | numGlyphs | The number of glyphs in the font. |

<a id="maxpHeader_10"></a>

*Version 1.0*

| Type | Name | Description |
| --- | --- | --- |
| Version16Dot16 | version | 0x00010000 for version 1.0. |
| uint16 | numGlyphs | The number of glyphs in the font. |
| uint16 | maxPoints | Maximum points in a non-composite glyph. |
| uint16 | maxContours | Maximum contours in a non-composite glyph. |
| uint16 | maxCompositePoints | Maximum points in a composite glyph. |
| uint16 | maxCompositeContours | Maximum contours in a composite glyph. |
| uint16 | maxZones | 1 if instructions do not use the twilight zone (Z0), or 2 if instructions do use Z0; should be set to 2 in most cases. |
| uint16 | maxTwilightPoints | Maximum points used in Z0. |
| uint16 | maxStorage | Number of Storage Area locations. |
| uint16 | maxFunctionDefs | Number of FDEFs, equal to the highest function number + 1. |
| uint16 | maxInstructionDefs | Number of IDEFs. |
| uint16 | maxStackElements | Maximum stack depth across Font Program (['fpgm' table](https://learn.microsoft.com/en-us/typography/opentype/spec/fpgm)), CVT Program (['prep' table](https://learn.microsoft.com/en-us/typography/opentype/spec/prep)) and all glyph instructions (in the ['glyf' table](../../glyf/README.md)). |
| uint16 | maxSizeOfInstructions | Maximum byte count for glyph instructions. |
| uint16 | maxComponentElements | Maximum number of components referenced at “top level” for any composite glyph. |
| uint16 | maxComponentDepth | Maximum levels of recursion; 1 for simple components. |
