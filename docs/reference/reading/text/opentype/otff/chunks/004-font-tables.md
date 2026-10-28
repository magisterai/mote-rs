# Font Tables

Source: [OpenType font file](https://learn.microsoft.com/en-us/typography/opentype/spec/otff) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/otff).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-calculating-checksums.md) · [Next](005-tables-related-to-color-fonts.md)

<a id="ottables"></a>

<a id="font-tables"></a>

## Font Tables

<a id="reqdTables"></a>

<a id="required-tables"></a>

### Required Tables

Whether TrueType or CFF outlines are used in an OpenType font, the following tables are required for the font to function correctly:

| Tag | Name |
| --- | --- |
| ['cmap'](../../cmap/README.md) | Character to glyph mapping |
| ['head'](../../head/README.md) | Font header |
| ['hhea'](../../hhea/README.md) | Horizontal header |
| ['hmtx'](../../hmtx/README.md) | Horizontal metrics |
| ['maxp'](../../maxp/README.md) | Maximum profile |
| ['name'](../../name/README.md) | Naming table |
| [OS/2](../../os2/README.md) | OS/2 and Windows specific metrics |
| ['post'](../../post/README.md) | PostScript information |

<a id="ttTables"></a>

<a id="tables-related-to-truetype-outlines"></a>

### Tables Related to TrueType Outlines

For OpenType fonts based on TrueType outlines, the following tables are used:

| Tag | Name |
| --- | --- |
| ['cvt '](https://learn.microsoft.com/en-us/typography/opentype/spec/cvt) | Control Value Table (optional table) |
| ['fpgm'](https://learn.microsoft.com/en-us/typography/opentype/spec/fpgm) | Font program (optional table) |
| ['glyf'](../../glyf/README.md) | Glyph data |
| ['loca'](../../loca/README.md) | Index to location |
| ['prep'](https://learn.microsoft.com/en-us/typography/opentype/spec/prep) | Control Value Program (optional table) |
| ['gasp'](https://learn.microsoft.com/en-us/typography/opentype/spec/gasp) | Grid-fitting/Scan-conversion (optional table) |

<a id="cffTables"></a>

<a id="tables-related-to-cff-outlines"></a>

### Tables Related to CFF Outlines

For OpenType fonts based on CFF outlines, the following tables are used:

| Tag | Name |
| --- | --- |
| ['CFF '](../../cff/README.md) | Compact Font Format 1.0 |
| [CFF2](https://learn.microsoft.com/en-us/typography/opentype/spec/cff2) | Compact Font Format 2.0 |
| [VORG](https://learn.microsoft.com/en-us/typography/opentype/spec/vorg) | Vertical Origin (optional table) |

It is strongly recommended that CFF OpenType fonts that are used for vertical writing include a [Vertical Origin (VORG) table](https://learn.microsoft.com/en-us/typography/opentype/spec/vorg).

<a id="svgTables"></a>

<a id="table-related-to-svg-outlines"></a>

### Table Related to SVG Outlines

| Tag | Name |
| --- | --- |
| ['SVG '](https://learn.microsoft.com/en-us/typography/opentype/spec/svg) | The SVG (Scalable Vector Graphics) table |

<a id="bitmapTables"></a>

<a id="tables-related-to-bitmap-glyphs"></a>

### Tables Related to Bitmap Glyphs

| Tag | Name |
| --- | --- |
| [EBDT](https://learn.microsoft.com/en-us/typography/opentype/spec/ebdt) | Embedded bitmap data |
| [EBLC](https://learn.microsoft.com/en-us/typography/opentype/spec/eblc) | Embedded bitmap location data |
| [EBSC](https://learn.microsoft.com/en-us/typography/opentype/spec/ebsc) | Embedded bitmap scaling data |
| [CBDT](https://learn.microsoft.com/en-us/typography/opentype/spec/cbdt) | Color bitmap data |
| [CBLC](https://learn.microsoft.com/en-us/typography/opentype/spec/cblc) | Color bitmap location data |
| ['sbix'](https://learn.microsoft.com/en-us/typography/opentype/spec/sbix) | Standard bitmap graphics |

OpenType fonts may also contain bitmaps of glyphs, in addition to outlines. Hand-tuned bitmaps are especially useful in OpenType fonts for representing complex glyphs at very small sizes. If a bitmap for a particular size is provided in a font, it will be used by the system instead of the outline when rendering the glyph.

<a id="otlTables"></a>

<a id="advanced-typographic-tables"></a>

### Advanced Typographic Tables

Several optional tables support advanced typographic functions:

| Tag | Name |
| --- | --- |
| [BASE](https://learn.microsoft.com/en-us/typography/opentype/spec/base) | Baseline data |
| [GDEF](../../gdef/README.md) | Glyph definition data |
| [GPOS](../../gpos/README.md) | Glyph positioning data |
| [GSUB](../../gsub/README.md) | Glyph substitution data |
| [JSTF](https://learn.microsoft.com/en-us/typography/opentype/spec/jstf) | Justification data |
| [MATH](../../math/README.md) | Math layout data |

For information on common table formats, please see [OpenType Layout Common Table Formats](../../chapter2/README.md) .

<a id="vartables"></a>

<a id="tables-used-for-opentype-font-variations"></a>

### Tables used for OpenType Font Variations

| Tag | Name |
| --- | --- |
| ['avar'](https://learn.microsoft.com/en-us/typography/opentype/spec/avar) | Axis variations |
| ['cvar'](https://learn.microsoft.com/en-us/typography/opentype/spec/cvar) | CVT variations (TrueType outlines only) |
| ['fvar'](https://learn.microsoft.com/en-us/typography/opentype/spec/fvar) | Font variations |
| ['gvar'](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar) | Glyph variations (TrueType outlines only) |
| [HVAR](https://learn.microsoft.com/en-us/typography/opentype/spec/hvar) | Horizontal metrics variations |
| [MVAR](https://learn.microsoft.com/en-us/typography/opentype/spec/mvar) | Metrics variations |
| [STAT](https://learn.microsoft.com/en-us/typography/opentype/spec/stat) | Style attributes (required for variable fonts, optional for non-variable fonts) |
| [VVAR](https://learn.microsoft.com/en-us/typography/opentype/spec/vvar) | Vertical metrics variations |

For an overview of OpenType Font Variations and the specification of the interpolation algorithm used for variations, see [OpenType Font Variations Overview](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview). For details regarding which tables are required or optional in variable fonts, see [Variation Data Tables and Miscellaneous Requirements](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview#vartables) in the Overview chapter.

For information on common table formats used for variations, see [OpenType Font Variations Common Table Formats](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats).

Note that some variation-related formats may be used in tables other than the variations-specific tables listed above. In particular, the GDEF, BASE or COLR tables in a variable font can include variation data using common table formats. A CFF2 table in a variable font can also include variation data, though using formats that are specific to the CFF2 table.

