# Front matter

Source: [Compact Font Format version 1](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/cff).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

This table contains a Compact Font Format (CFF) font representation and is structured according to [Adobe Technical Note #5176: “The Compact Font Format Specification,”](http://partners.adobe.com/public/developer/en/font/5176.CFF.pdf) and [Adobe Technical Note #5177: “Type 2 Charstring Format.”](http://partners.adobe.com/public/developer/en/font/5177.Type2.pdf)

> *Note:* See Adobe Technical Note #5176 for details regarding the CFF INDEX, DICT and FontSet structures mentioned below.

The CFF font format existed as an independent font format prior to the development of OpenType. When incorporated into the OpenType format, certain essential font information continued to be represented using mechanisms defined in the CFF format, duplicating other data representations available within OpenType for use in fonts with TrueType outlines. For example, CFF incorporates glyph widths into glyph outline descriptions, though these could otherwise be provided in the ['hmtx' table](../../hmtx/README.md). OpenType fonts with TrueType outlines use a glyph index in conjunction with offsets in the ['loca' table](../../loca/README.md) to access glyph data within the 'glyf' table. This concept is retained in OpenType CFF fonts, except that glyph data is accessed through a *CharStrings INDEX* structure within the 'CFF ' table.

See [Comparison of 'glyf', 'CFF ' and CFF2 tables](https://learn.microsoft.com/en-us/typography/opentype/spec/glyphformatcomparison) for a summary of significant differences between the 'glyf', 'CFF ' and CFF2 tables.

The Name INDEX in the 'CFF ' table must contain only one entry; that is, there must be only one font in the CFF FontSet. It is not a requirement that this name be the same as name ID 6 entries in the ['name' table](../../name/README.md). Note that, in an OpenType font collection file, a single 'CFF ' table can be shared across multiple fonts; names used by applications must be those provided in the 'name' table, not the Name INDEX entry.

The CFF Top DICT must specify a CharstringType value of 2.

The numGlyphs field in the ['maxp' table](../../maxp/README.md) must be the same as the number of entries in the CFF’s CharStrings INDEX. The OpenType font glyph index is the same as the CFF glyph index for all glyphs in the font.
