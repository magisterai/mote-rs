# Example 18: MarkArray table and MarkRecord

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](012-example-12-contextual-positioning-format-3.md)

<a id="example18"></a>

<a id="example-18-markarray-table-and-markrecord"></a>

### Example 18: MarkArray table and MarkRecord

Example 18 shows a MarkArray table with class and attachment point data for two accent marks, a grave and a cedilla. Two MarkRecords are defined, one for each covered mark glyph. The first MarkRecord assigns a mark class value of 0 to accents placed above base glyphs, such as the grave, and has an offset to a graveMarkAnchor table. The second MarkRecord assigns a mark class value of 1 for all accents positioned below base glyphs, such as the cedilla, and has an offset to a cedillaMarkAnchor table.

*Example 18*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **MarkArray**  MarkGlyphsArray | MarkArray table definition |
| 0002 | 2 | markCount |
|  | markRecords[0] | for first mark in markCoverage table: grave |
| 0000 | 0 | markClass — for marks placed above base glyphs |
| 000A | graveMarkAnchor | markAnchorOffset |
|  | markRecords[1] | for second mark in markCoverage table: cedilla |
| 0001 | 1 | markClass — for marks placed below base glyphs |
| 0010 | cedillaMarkAnchor | markAnchorOffset |
