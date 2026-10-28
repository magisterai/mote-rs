# Table organization

Source: [Glyph data](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-composite-glyph-description.md)

This table contains information that describes the glyphs in the font in the TrueType outline format. Information regarding the rasterizer (scaler) refers to the TrueType rasterizer. For details regarding scaling, grid-fitting and rasterization of TrueType outlines, see the [TrueType Fundamentals](https://learn.microsoft.com/en-us/typography/opentype/spec/ttch01) chapter.

<a id="organization"></a>

<a id="table-organization"></a>

## Table organization

The 'glyf' table is comprised of a list of glyph data blocks, each of which provides the description for a single glyph. Glyphs are referenced by identifiers (glyph IDs), which are sequential integers beginning at zero. The total number of glyphs is specified by the numGlyphs field in the ['maxp' table](../../maxp/README.md). The 'glyf' table does not include any overall table header or records providing offsets to glyph data blocks. Rather, the ['loca' table](../../loca/README.md) provides an array of offsets, indexed by glyph IDs, which provide the location of each glyph data block within the 'glyf' table. Note that the 'glyf' table must always be used in conjunction with the 'loca' and 'maxp' tables. The size of each glyph data block is inferred from the difference between two consecutive offsets in the 'loca' table (with one extra offset provided to give the size of the last glyph data block). As a result of the 'loca' format, glyph data blocks within the 'glyf' table must be in glyph ID order.

Each glyph description uses one of two formats:

- Simple glyph descriptions specify a glyph outline directly using Bezier control points.
- Composite glyph descriptions specify a glyph outline indirectly by referencing one or more glyph IDs to use as components.

In both cases, the glyph description begins with a glyph header that is common to both types of glyph description.

<a id="glyphHeaders"></a>

<a id="glyph-headers"></a>

## Glyph headers

Each glyph description begins with a header:

<a id="glyphHeaderFormat"></a>

*Glyph Header*

| Type | Name | Description |
| --- | --- | --- |
| int16 | numberOfContours | If the number of contours is greater than or equal to zero, this is a simple glyph. If negative, this is a composite glyph — the value -1 should be used for composite glyphs. |
| int16 | xMin | Minimum x for coordinate data. |
| int16 | yMin | Minimum y for coordinate data. |
| int16 | xMax | Maximum x for coordinate data. |
| int16 | yMax | Maximum y for coordinate data. |

The bounding rectangle from each glyph is defined as the rectangle with a lower left corner of (xMin, yMin) and an upper right corner of (xMax, yMax). These values are obtained directly from the point coordinate data for the glyph, comparing all on-curve and off-curve points. Phantom points computed by the rasterizer are not relevant. Note that the bounding box defined by control points is guaranteed to contain the outline, but might not be tight to the outline.

The scaler will perform better if the glyph coordinates have been created such that the xMin is equal to the lsb. For example, if the lsb is 123, then xMin for the glyph should be 123. If the lsb is -12 then the xMin should be -12. If the lsb is 0 then xMin is 0. If all glyphs are done like this, set bit 1 of flags field in the 'head' table.

> *Note:* The glyph descriptions do not include side bearing information. Left side bearings are provided in the ['hmtx' table](../../hmtx/README.md), and right side bearings are inferred from the advance width (also provided in the 'hmtx' table) and the bounding box coordinates provided in the 'glyf' table. For vertical layout, top side bearings are provided in the ['vmtx' table](../../vmtx/README.md), and bottom side bearings are inferred. The rasterizer will generate a representation of side bearings in the form of “phantom” points, which are added as four additional points at the end of the glyph description, and which can be referenced and manipulated by glyph instructions. See the chapter [Instructing TrueType Glyphs](https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructing_glyphs#phantoms) for more background on phantom points.

In a variable font, the minimum and maximum x or y values of control points can vary, and a tight bounding rectangle containing the outline or all points for an instance could be smaller or larger than for the default instance (that is, for the glyph description in this table). The xMin, yMin, xMax and yMax values might or might not encompass the derived outline for an instance. Also, the ['gvar'](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar) table does not provide deltas for these values. If an application requires a bounding rectangle for a non-default instance of a glyph, the derived point data (with deltas applied) should be processed to determine a bounding rectangle.

<a id="simpleGlyphDescription"></a>

<a id="simple-glyph-description"></a>

## Simple glyph description

This is the information used to describe a glyph if numberOfContours is greater than or equal to zero—that is, if a glyph is not a composite.

If a glyph has zero contours, no additional glyph data beyond the header is required. A glyph with zero contours may have additional data, however; in particular, it may have instructions that operate on phantom points.

Point numbers are base-zero indices numbered sequentially across all of the contours for a glyph. The first point of the first contour has point number zero; the first point number of each additional contour is one greater than the last point number of the preceding contour. Phantom point numbers follow after the last point number of the last contour.

<a id="simpleGlyphTable"></a>

*Simple Glyph table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | endPtsOfContours[numberOfContours] | Array of point indices for the last point of each contour, in increasing numeric order. |
| uint16 | instructionLength | Total number of bytes for instructions. If instructionLength is zero, no instructions are present for this glyph, and this field is followed directly by the flags field. |
| uint8 | instructions[instructionLength] | Array of instruction byte code for the glyph. |
| uint8 | flags[*variable*] | Array of flag elements. See below for details regarding the number of flag array elements. |
| uint8 or int16 | xCoordinates[*variable*] | Contour point x-coordinates. See below for details regarding the number of coordinate array elements. Coordinate for the first point is relative to (0,0); others are relative to previous point. |
| uint8 or int16 | yCoordinates[*variable*] | Contour point y-coordinates. See below for details regarding the number of coordinate array elements. Coordinate for the first point is relative to (0,0); others are relative to previous point. |

> *Note:* In the 'glyf' table, the position of a point is not stored in absolute terms but as a vector relative to the previous point. The delta-x and delta-y vectors represent these (often small) changes in position. Coordinate values are in font design units, as defined by the [unitsPerEm](../../head/README.md) field in the 'head' table. Note that smaller unitsPerEm values will make it more likely that delta-x and delta-y values can fit in a smaller representation (8-bit rather than 16-bit), though with a trade-off in the level or precision that can be used for describing an outline.

Each element in the flags array is a single byte, each of which has multiple flag bits with distinct meanings, as shown below.

In logical terms, there is one flag byte element, one x-coordinate, and one y-coordinate for each point. The number of points is determined by the last entry in the endPtsOfContours array. Note, however, that the flag byte elements and the coordinate arrays use packed representations. In particular, if a logical sequence of flag elements or sequence of x- or y-coordinates is repeated, then the actual flag byte element or coordinate value can be given in a single entry, with special flags used to indicate that this value is repeated for subsequent logical entries. The actual stored size of the flags or coordinate arrays must be determined by parsing the flags array entries. See the flag descriptions below for details.

<a id="simpleGlyphFlags"></a>

*Simple Glyph flags*

| Mask | Name | Description |
| --- | --- | --- |
| 0x01 | ON\_CURVE\_POINT | Bit 0: If set, the point is on the curve; otherwise, it is off the curve. |
| 0x02 | X\_SHORT\_VECTOR | Bit 1: If set, the corresponding x-coordinate is 1 byte long, and the sign is determined by the X\_IS\_SAME\_OR\_POSITIVE\_X\_SHORT\_VECTOR flag. If not set, its interpretation depends on the X\_IS\_SAME\_OR\_POSITIVE\_X\_SHORT\_VECTOR flag: If that other flag is set, the x-coordinate is the same as the previous x-coordinate, and no element is added to the xCoordinates array. If both flags are not set, the corresponding element in the xCoordinates array is two bytes and interpreted as a signed integer. See the description of the X\_IS\_SAME\_OR\_POSITIVE\_X\_SHORT\_VECTOR flag for additional information. |
| 0x04 | Y\_SHORT\_VECTOR | Bit 2: If set, the corresponding y-coordinate is 1 byte long, and the sign is determined by the Y\_IS\_SAME\_OR\_POSITIVE\_Y\_SHORT\_VECTOR flag. If not set, its interpretation depends on the Y\_IS\_SAME\_OR\_POSITIVE\_Y\_SHORT\_VECTOR flag: If that other flag is set, the y-coordinate is the same as the previous y-coordinate, and no element is added to the yCoordinates array. If both flags are not set, the corresponding element in the yCoordinates array is two bytes and interpreted as a signed integer. See the description of the Y\_IS\_SAME\_OR\_POSITIVE\_Y\_SHORT\_VECTOR flag for additional information. |
| 0x08 | REPEAT\_FLAG | Bit 3: If set, the next byte (read as unsigned) specifies the number of additional times this flag byte is to be repeated in the logical flags array — that is, the number of additional logical flag entries inserted after this entry. (In the expanded logical array, this bit is ignored.) In this way, the number of flags listed can be smaller than the number of points in the glyph description. |
| 0x10 | X\_IS\_SAME\_OR\_POSITIVE\_X\_SHORT\_VECTOR | Bit 4: This flag has two meanings, depending on how the X\_SHORT\_VECTOR flag is set. If X\_SHORT\_VECTOR is set, this bit describes the sign of the value, with 1 equaling positive and 0 negative. If X\_SHORT\_VECTOR is not set and this bit is set, then the current x-coordinate is the same as the previous x-coordinate. If X\_SHORT\_VECTOR is not set and this bit is also not set, the current x-coordinate is a signed 16-bit delta vector. |
| 0x20 | Y\_IS\_SAME\_OR\_POSITIVE\_Y\_SHORT\_VECTOR | Bit 5: This flag has two meanings, depending on how the Y\_SHORT\_VECTOR flag is set. If Y\_SHORT\_VECTOR is set, this bit describes the sign of the value, with 1 equaling positive and 0 negative. If Y\_SHORT\_VECTOR is not set and this bit is set, then the current y-coordinate is the same as the previous y-coordinate. If Y\_SHORT\_VECTOR is not set and this bit is also not set, the current y-coordinate is a signed 16-bit delta vector. |
| 0x40 | OVERLAP\_SIMPLE | Bit 6: If set, contours in the glyph description could overlap. Use of this flag is not required — that is, contours may overlap without having this flag set. When used, it must be set on the first flag byte for the glyph. See additional details below. |
| 0x80 | Reserved | Bit 7 is reserved: set to zero. |

The OVERLAP\_SIMPLE flag is not required and ignored by some rasterizer implementations. However, it may be (and is) used by some rasterizers to activate additional logic required when contours overlap to obtain correct rasterization. This flag can be used in fonts to provide broad interoperability across rasterizer implementations.

Variable fonts often make use of overlapping contours. This has implications for tools that generate static-font data for a specific instance of a variable font, if broad interoperability of the derived font is desired: if a glyph has overlapping contours in the given instance, then the tool should either set the OVERLAP\_SIMPLE flag in the derived glyph data, or else should merge contours to remove overlap of separate contours.

> *Note:* The OVERLAP\_COMPOUND flag, described below, has a similar purpose in relation to composite glyphs. The same considerations described for the OVERLAP\_SIMPLE flag also apply to the OVERLAP\_COMPOUND flag.

