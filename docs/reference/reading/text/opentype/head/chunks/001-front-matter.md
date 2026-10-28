# Front matter

Source: [Font header](https://learn.microsoft.com/en-us/typography/opentype/spec/head) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/head).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

<a id="headHeader"></a>

This table gives global information about the font. The bounding box values should be computed using *only* glyphs that have contours. Glyphs with no contours should be ignored for the purposes of these calculations.

<a id="majorVersion"></a>

<a id="minorVersion"></a>

<a id="fontRevision"></a>

<a id="checkSumAdjustment"></a>

<a id="magicNumber"></a>

<a id="flags"></a>

<a id="unitsPerEm"></a>

<a id="created"></a>

<a id="modified"></a>

<a id="xMin"></a>

<a id="yMin"></a>

<a id="xMax"></a>

<a id="yMax"></a>

<a id="macstyle"></a>

<a id="lowestRecPPEM"></a>

<a id="fontDirectionHint"></a>

<a id="indexToLocFormat"></a>

<a id="glyphDataFormat"></a>

<table>
<thead>
<tr>
<th>Type</th>
<th>Name</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td><a id="majorVersion"></a> uint16</td>
<td>majorVersion</td>
<td>Major version number of the font header table — set to 1.</td>
</tr>
<tr>
<td><a id="minorVersion"></a> uint16</td>
<td>minorVersion</td>
<td>Minor version number of the font header table — set to 0.</td>
</tr>
<tr>
<td><a id="fontRevision"></a> Fixed</td>
<td>fontRevision</td>
<td>Set by font manufacturer.</td>
</tr>
<tr>
<td><a id="checkSumAdjustment"></a> uint32</td>
<td>checksumAdjustment</td>
<td>To compute: set it to 0, sum the entire font as uint32, then store 0xB1B0AFBA - sum. If the font is used as a component in a font collection file, the value of this field will be invalidated by changes to the file structure and font <a data-linktype="relative-path" href="../../otff/chunks/002-organization-of-an-opentype-font.md#tabledirectory">table directory</a>, and must be ignored.</td>
</tr>
<tr>
<td><a id="magicNumber"></a> uint32</td>
<td>magicNumber</td>
<td>Set to 0x5F0F3CF5.</td>
</tr>
<tr>
<td><a id="flags"></a> uint16</td>
<td>flags</td>
<td><b>Bit 0:</b> Baseline for font at y=0.<br/><br/><b>Bit 1:</b> Left sidebearing point at x=0 (relevant only for TrueType rasterizers) — see additional information below regarding variable fonts.<br/><br/><b>Bit 2:</b> Instructions may depend on point size.<br/><br/><b>Bit 3:</b> Force ppem to integer values for all internal scaler math; may use fractional ppem sizes if this bit is clear. It is strongly recommended that this be set in hinted fonts.<br/><br/><b>Bit 4:</b> Instructions may alter advance width (the advance widths might not scale linearly).<br/><br/><b>Bit 5:</b> This bit is not used in OpenType, and should not be set in order to ensure compatible behavior on all platforms. If set, it may result in different behavior for vertical layout in some platforms. (See <a data-linktype="external" href="http://developer.apple.com/fonts//TrueType-Reference-Manual/RM06/Chap6head.html"> Apple’s specification</a> for details regarding behavior in Apple platforms.)<br/><br/><b>Bits 6 – 10:</b> These bits are not used in OpenType and should always be cleared. (See <a data-linktype="external" href="http://developer.apple.com/fonts//TrueType-Reference-Manual/RM06/Chap6head.html"> Apple’s specification</a> for details regarding legacy use in Apple platforms.)<br/><br/><b>Bit 11:</b> Font data is “lossless” as a result of having been subjected to optimizing transformation and/or compression (such as compression mechanisms defined by ISO/IEC 14496-18, MicroType® Express, WOFF 2.0, or similar) where the original font functionality and features are retained but the binary compatibility between input and output font files is not guaranteed. As a result of the applied transform, the DSIG table may also be invalidated.<br/><br/><b>Bit 12:</b> Font converted (produce compatible metrics).<br/><br/><b>Bit 13:</b> Font optimized for ClearType®. Note, fonts that rely on embedded bitmaps (EBDT) for rendering should not be considered optimized for ClearType, and therefore should keep this bit cleared.<br/><br/><b>Bit 14:</b> Last Resort font. If set, indicates that the glyphs encoded in the 'cmap' subtables are simply generic symbolic representations of code point ranges and do not truly represent support for those code points. If unset, indicates that the glyphs encoded in the 'cmap' subtables represent proper support for those code points.<br/><br/><b>Bit 15:</b> Reserved, set to 0.</td>
</tr>
<tr>
<td><a id="unitsPerEm"></a> uint16</td>
<td>unitsPerEm</td>
<td>Set to a value from 16 to 16384. Any value in this range is valid. In fonts that have TrueType outlines, a power of 2 is recommended as this allows performance optimization in some rasterizers.</td>
</tr>
<tr>
<td><a id="created"></a> LONGDATETIME</td>
<td>created</td>
<td>Number of seconds since 12:00 midnight that started January 1st, 1904, in GMT/UTC time zone.</td>
</tr>
<tr>
<td><a id="modified"></a> LONGDATETIME</td>
<td>modified</td>
<td>Number of seconds since 12:00 midnight that started January 1st, 1904, in GMT/UTC time zone.</td>
</tr>
<tr>
<td><a id="xMin"></a> int16</td>
<td>xMin</td>
<td>Minimum x coordinate across all glyph bounding boxes.</td>
</tr>
<tr>
<td><a id="yMin"></a> int16</td>
<td>yMin</td>
<td>Minimum y coordinate across all glyph bounding boxes.</td>
</tr>
<tr>
<td><a id="xMax"></a> int16</td>
<td>xMax</td>
<td>Maximum x coordinate across all glyph bounding boxes.</td>
</tr>
<tr>
<td><a id="yMax"></a> int16</td>
<td>yMax</td>
<td>Maximum y coordinate across all glyph bounding boxes.</td>
</tr>
<tr>
<td><a id="macstyle"></a> uint16</td>
<td>macStyle</td>
<td><b>Bit 0:</b> Bold (if set to 1);<br/><b>Bit 1:</b> Italic (if set to 1)<br/><b>Bit 2:</b> Underline (if set to 1)<br/><b>Bit 3:</b> Outline (if set to 1) <br/><b>Bit 4:</b> Shadow (if set to 1)<br/><b>Bit 5:</b> Condensed (if set to 1)<br/><b>Bit 6:</b> Extended (if set to 1)<br/><b>Bits 7 – 15:</b> Reserved (set to 0).</td>
</tr>
<tr>
<td><a id="lowestRecPPEM"></a> uint16</td>
<td>lowestRecPPEM</td>
<td>Smallest readable size in pixels.</td>
</tr>
<tr>
<td><a id="fontDirectionHint"></a> int16</td>
<td>fontDirectionHint <sup>a</sup></td>
<td>Deprecated (Set to 2).<br/>0: Fully mixed directional glyphs;<br/>1: Only strongly left to right;<br/>2: Like 1 but also contains neutrals;<br/>-1: Only strongly right to left;<br/>-2: Like -1 but also contains neutrals.</td>
</tr>
<tr>
<td><a id="indexToLocFormat"></a> int16</td>
<td>indexToLocFormat</td>
<td>0 for short offsets (Offset16), 1 for long (Offset32).</td>
</tr>
<tr>
<td><a id="glyphDataFormat"></a> int16</td>
<td>glyphDataFormat</td>
<td>0 for current format.</td>
</tr>
</tbody>
</table>

<sup>a</sup> The fontDirectionHint field was intended for support of right-to-left scripts. Strong left to right, strong right to left, and neutral are properties of characters. A neutral character has no inherent directionality; it is not a character with zero (0) width. Spaces and punctuation are examples of neutral characters. Non-neutral characters are those with inherent directionality. For example, Roman letters (left-to-right) and Arabic letters (right-to-left) have strong directionality. In a “normal” Roman font where spaces and punctuation are present, the font direction hints should be set to two (2).

The unitsPerEm value determines the granularity of the font’s coordinate grid at which coordinates can be specified for visual elements such as outline control points or mark-attachment anchor positions. See the [TrueType Fundamentals](https://learn.microsoft.com/en-us/typography/opentype/spec/ttch01#funits-and-the-grid) chapter for more information.

A glyph’s bounding box is the smallest rectangle that contains all of the glyph’s control points. (See the ['glyf'](../../glyf/chunks/001-table-organization.md#glyph-headers) table chapter for more information.) The xMin, yMin, xMax and yMax values provide a bounding box that will contain all glyphs from the font. Glyphs without contours are ignored.

The macStyle bits must agree with the fsSelection bits in the [OS/2 table](../../os2/README.md). The fsSelection bits are used over the macStyle bits in Windows. The PANOSE values and 'post' table values are ignored for determining bold or italic fonts.

For historical reasons, the fontRevision value contained in this table is not used by Windows to determine the version of a font. Instead, Windows evaluates the version string (ID 5) in the 'name' table.

The left sidebearing point for each glyph corresponds to the first phantom point (“pp1” below—see [Phantom points](https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructing_glyphs#phantom-points)) and relates to the glyph left sidebearing (lsb) and xMin as follows:

> pp1 = xMin - lsb

If bit 1 of the flags field is set, then pp1 = 0 for all glyphs and the xMin and left sidebearing for each glyph must be equal.

In a variable font with TrueType outlines, the left side bearing for each glyph must equal xMin, and bit 1 in the flags field must be set. Also, bit 5 must be cleared in all variable fonts. For general information on OpenType Font Variations, see the chapter, [OpenType Font Variations Overview](https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview).

Also, in a variable font, the minimum or maximum x or y values of control points can vary, and a tight bounding rectangle containing the outline or all points for an instance of any given glyph can be smaller or larger than for the default instance of that glyph. The xMin, yMin, xMax and yMax values in this table might or might not encompass the derived glyph outlines for an instance. In addition, variation deltas are not provided for these values. If an application requires a bounding rectangle that encompasses the glyphs for a non-default instance of the font, the derived glyph outlines for the instance should be processed to determine a bounding rectangle.
