# Example 6: CaretValueFormat3 table

Source: [Glyph definition](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gdef).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-item-variation-store-table.md)

<a id="example-6-caretvalueformat3-table"></a>

### Example 6: CaretValueFormat3 table

In Example 6, the CaretValueFormat3 table defines a caret position in design units, but includes a Device table to adjust the X or Y coordinate for the point size and resolution of the output font. Here, the Device table specifies pixel adjustments for font sizes from 12 ppem to 17 ppem.

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **CaretValueFormat3**  Caret3 | CaretValue table definition |
| 0003 | 3 | format: design units plus Device table |
| 04B6 | 1206 | coordinate (X or Y value, design units) |
| 0006 | CaretDevice | offset to Device table |
|  | **DeviceTableFormat2**  CaretDevice | Device Table definition |
| 000C | 12 | startSize |
| 0011 | 17 | endSize |
| 0002 | 2 | deltaFormat |
|  | 1 | increase 12ppm by 1 pixel |
|  | 1 | increase 13ppm by 1 pixel |
|  | 1 | increase 14ppm by 1 pixel |
| 1111 | 1 | increase 15ppm by 1 pixel |
|  | 2 | increase 16ppm by 2 pixels |
| 2200 | 2 | increase 17ppm by 2 pixels |

<a id="example-7-mark-attachment-class-definition-table"></a>

### Example 7: Mark attachment class definition table

In Example 7, a class definition table specifies an attachment class for the each of the glyph ranges predefined in the GlyphClassDef table as marks.

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **ClassDefFormat2**  theMarkAttachClassDefTable | ClassDef table definition |
| 0002 | 2 | classFormat: glyph ranges |
| 0004 | 4 | classRangeCount |
|  | classRangeRecords[0] |  |
| 0268 | graveAccentGlyphID | startGlyphID |
| 026A | circumflexAccentGlyphID | endGlyphID |
| 0001 | 1 | class: top marks  ClassRangeRecord[1] |
|  | classRangeRecords[1] |  |
| 0270 | diaeresisAccentGlyphID | startGlyphID |
| 0272 | acuteAccentGlyphID | endGlyphID |
| 0001 | 1 | class: top marks |
|  | classRangeRecords[2] |  |
| 028C | diaeresisBelowGlyphID | startGlyphID |
| 028F | cedillaGlyphID | endGlyphID |
| 0002 | 2 | class: bottom marks  ClassRangeRecord[3] |
|  | classRangeRecords[3] |  |
| 0295 | circumflexBelowGlyphID | startGlyphID |
| 0295 | circumflexBelowGlyphID | endGlyphID |
| 0002 | 2 | class: bottom marks |
