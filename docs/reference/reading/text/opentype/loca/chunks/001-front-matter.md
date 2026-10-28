# Front matter

Source: [Index to location](https://learn.microsoft.com/en-us/typography/opentype/spec/loca) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/loca).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

The index to location ('loca') table stores an array of offsets to the locations of glyph descriptions in the ['glyf'](../../glyf/README.md) table, relative to the beginning of that table. Offsets in the array are referenced by corresponding glyph IDs.

Offsets must be in ascending order, with loca[*n*] <= loca[*n+1*]. The length of each glyph description is determined by the difference between two consecutive entries: loca[*n+1*] - loca[*n*]. To compute the length of the last glyph description, there is an extra entry in the offsets array after the entry for the last valid glyph ID. Thus, the number of elements in the offsets array is numGlyphs + 1.

By definition, glyph index zero points to the “missing character” glyph, which is the glyph that is displayed if a character is not found in the font’s ['cmap'](../../cmap/README.md) table. A missing character is commonly represented by a blank box or a space. If the font does not contain an outline for this glyph, then the first and second offsets should have the same value. This also applies to any other glyphs without an outline, such as the glyph for the space character: if a glyph has no outline or instructions, then loca[*n*] = loca[*n+1*].

There are two formats of this table: a short format, and a long format. The format is specified by the indexToLocFormat entry in the ['head'](../../head/README.md) table.

*Short format*

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | offsets[numGlyphs + 1] | The local offset divided by 2 is stored. |

*Long format*

| Type | Name | Description |
| --- | --- | --- |
| Offset32 | offsets[numGlyphs + 1] | The actual local offset is stored. |
