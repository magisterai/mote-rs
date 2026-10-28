# Composite glyph description

Source: [Glyph data](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-table-organization.md) · [Next](003-continuation-3.md)

<a id="compositeGlyphDescription"></a>

<a id="composite-glyph-description"></a>

## Composite glyph description

If numberOfContours is negative, a composite glyph description is used.

> *Note:* A numberOfContours value of -1 is recommended to indicate a composite glyph.

A composite, or compound, glyph describes an outline indirectly by referencing other glyphs that get incorporated into the composite glyph as components. This is useful when the same contours are needed for multiple glyphs as it provides consistency for contours that are repeated across multiple glyphs and can also provide significant size reduction.

To add clarity in explaining composite glyphs, the terms *parent* and *child* will be used, a composite glyph description being the parent, and the other glyphs referenced as components being the children.

Composite glyphs may be nested within other composite glyphs—that is, a composite glyph parent may include other composite glyphs as child components. Thus, a composite glyph description is a directed graph. This graph must be acyclic, with every path through the graph leading to a simple glyph as a leaf node. The maxComponentDepth field in the ['maxp'](../../maxp/README.md) table is set to indicate the maximum nesting depth across all composite glyphs in a font. There is no minimum nesting depth that must be supported. For fonts to be compatible with the widest range of implementations, nesting of composites should be avoided.

> *Note:* Some PostScript devices (and possibly other implementations) do not correctly render glyphs that have nested composite descriptions. A composite glyph description that has nested composites can be flattened to reference only simple glyphs as child components. This can lose some benefits of de-duplication of information, but can still retain significant size-saving benefits as well as providing broader compatibility.

The data for a composite glyph description is comprised of a sequence of data blocks for each child component glyph. A flag within the data for each component is used to indicate if there are additional components in the sequence. The sequence is processed in the order given, with the contours from each child glyph incorporated into the parent. As the contours from a child are incorporated, its control points are renumbered to follow sequentially after all points previously incorporated into the parent.

Each glyph has an outline positioned within the font design grid based on the x and y coordinates of its control points. When incorporated as a child into a composite glyph, the parent can control placement of the child outline within the parent’s design grid. This can be done in two different ways: by specifying a vector offset added to (x, y) coordinates of the child’s control points, or by specifying one control point from the child’s outline that is aligned with a specified control point in the parent. The second mechanism assumes some outlines have already been incorporated into the parent, so cannot be used for the first component glyph.

The parent can also specify a scale or other affine transform to be applied to a child glyph as it is incorporated into the parent. The transform can affect an offset vector used to position the child glyph; see below for additional details.

Each component glyph can include instructions that apply to its outline. A parent composite glyph description can include instructions that apply to the composite as a whole, after instructions for each child have been performed. Instructions for the parent composite apply to the accumulated contour data from components with points renumbered, as described above.

Before each child is incorporated into the parent, it is processed, with phantom points defined and hinting instructions performed. Thus, if placement of the child is done by alignment of points, the child’s phantom points can be used for this alignment, and instructions in child glyphs that affected their points will already have been performed.

The data block for each child component starts with two uint16 values: a flags field, and a glyph ID. These are followed by two argument fields, though the size and interpretation of the arguments varies according to the flags that are set. Optional fields describing a transformation can follow the arguments, depending on the flags.

<a id="compositeGlyphTable"></a>

*Component Glyph record*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | flags | component flag |
| uint16 | glyphIndex | glyph index of component |
| uint8, int8, uint16 or int16 | argument1 | x-offset for component or point number; type depends on bits 0 and 1 in component flags |
| uint8, int8, uint16 or int16 | argument2 | y-offset for component or point number; type depends on bits 0 and 1 in component flags |
| [transform data] |  | optional transform data—see below |

The C pseudo-code fragment below shows how the sequence of component glyph records is stored and parsed; definitions for the flag bits follow this fragment:



```
do {
    uint16 flags;
    uint16 glyphIndex;
    if ( flags & ARG_1_AND_2_ARE_WORDS) {
    (int16 or FWORD) argument1;
    (int16 or FWORD) argument2;
    } else {
        uint16 arg1and2; /* (arg1 << 8) | arg2 */
    }
    if ( flags & WE_HAVE_A_SCALE ) {
        F2DOT14  scale;    /* Format 2.14 */
    } else if ( flags & WE_HAVE_AN_X_AND_Y_SCALE ) {
        F2DOT14  xscale;    /* Format 2.14 */
        F2DOT14  yscale;    /* Format 2.14 */
    } else if ( flags & WE_HAVE_A_TWO_BY_TWO ) {
        F2DOT14  xscale;    /* Format 2.14 */
        F2DOT14  scale01;   /* Format 2.14 */
        F2DOT14  scale10;   /* Format 2.14 */
        F2DOT14  yscale;    /* Format 2.14 */
    }
} while ( flags & MORE_COMPONENTS )
if (flags & WE_HAVE_INSTRUCTIONS){
    uint16 numInstr
    uint8 instr[numInstr]
}
```




The following component glyph flags are defined:

<a id="compositeGlyphFlags"></a>

*Component Glyph flags*

| Mask | Name | Description |
| --- | --- | --- |
| 0x0001 | ARG\_1\_AND\_2\_ARE\_WORDS | Bit 0: If this is set, the arguments are 16-bit (uint16 or int16); otherwise, they are bytes (uint8 or int8). |
| 0x0002 | ARGS\_ARE\_XY\_VALUES | Bit 1: If this is set, the arguments are signed xy values; otherwise, they are unsigned point numbers. |
| 0x0004 | ROUND\_XY\_TO\_GRID | Bit 2: If set and ARGS\_ARE\_XY\_VALUES is also set, the xy values are rounded to the nearest grid line. Ignored if ARGS\_ARE\_XY\_VALUES is not set. |
| 0x0008 | WE\_HAVE\_A\_SCALE | Bit 3: This indicates that there is a simple scale for the component. Otherwise, scale = 1.0. |
| 0x0020 | MORE\_COMPONENTS | Bit 5: Indicates at least one more glyph after this one. |
| 0x0040 | WE\_HAVE\_AN\_X\_AND\_Y\_SCALE | Bit 6: The x direction will use a different scale from the y direction. |
| 0x0080 | WE\_HAVE\_A\_TWO\_BY\_TWO | Bit 7: There is a 2 by 2 transformation that will be used to scale the component. |
| 0x0100 | WE\_HAVE\_INSTRUCTIONS | Bit 8: Following the last component are instructions for the composite glyph. |
| 0x0200 | USE\_MY\_METRICS | Bit 9: If set, this forces the aw and lsb (and rsb) for the composite to be equal to those from this component glyph. This works for hinted and unhinted glyphs. |
| 0x0400 | OVERLAP\_COMPOUND | Bit 10: If set, the components of the compound glyph overlap. Use of this flag is not required — that is, component glyphs may overlap without having this flag set. When used, it must be set on the flag word for the first component. Some rasterizer implementations may require fonts to use this flag to obtain correct behavior — see additional remarks, above, for the similar OVERLAP\_SIMPLE flag used in simple-glyph descriptions. |
| 0x0800 | SCALED\_COMPONENT\_OFFSET | Bit 11: The composite is designed to have the component offset scaled. Ignored if ARGS\_ARE\_XY\_VALUES is not set. |
| 0x1000 | UNSCALED\_COMPONENT\_OFFSET | Bit 12: The composite is designed not to have the component offset scaled. Ignored if ARGS\_ARE\_XY\_VALUES is not set. |
| 0xE010 | Reserved | Bits 4, 13, 14 and 15 are reserved: set to 0. |

The argument1 and argument2 fields of the component glyph record are used to determine the placement of the child component glyph within the parent composite glyph. They are interpreted either as an offset vector or as points from the parent and the child, according to whether the ARGS\_ARE\_XY\_VALUES flag is set. This flag must always be set for the first component of a composite glyph.

If ARGS\_ARE\_XY\_VALUES is set, then argument1 and argument2 are interpreted as units in the design coordinate system and an offset vector (x, y) = (argument1, argument2) is added to the coordinates of each control point of the component glyph. In a variable font, the offset vector can be modified by deltas in the 'gvar' table; see [Point numbers and processing for composite glyphs](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#point-numbers-and-processing-for-composite-glyphs) in the 'gvar' chapter for details. If a scale or transform matrix is provided, the offset vector might or might not be subject to the transformation; see the discussion below of the SCALED\_COMPONENT\_OFFSET and UNSCALED\_COMPONENT\_OFFSET flags for details.

If ARGS\_ARE\_XY\_VALUES is set and the ROUND\_XY\_TO\_GRID flag is also set, the offset vector (after any transformation and variation deltas are applied) is grid-fitted, with the x and y values rounded to the nearest pixel grid line.

If ARGS\_ARE\_XY\_VALUES is not set, then argument1 is a point number in the parent glyph (from contours incoporated and re-numbered from previous component glyphs); and argument2 is a point number (prior to re-numbering) from the child component glyph. Phantom points from the parent or the child may be referenced. The child component glyph is positioned within the parent glyph by aligning the two points. If a scale or transform matrix is provided, the transformation is applied to the child’s point before the points are aligned.

In a variable font, when a component is positioned by alignment of points, deltas are applied to component glyphs before this alignment is done. Any deltas specified for the parent composite glyph to be applied to components positioned by point alignment are ignored. See [Point numbers and processing for composite glyphs](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#point-numbers-and-processing-for-composite-glyphs) in the 'gvar' chapter for details.

The WE\_HAVE\_A\_SCALE, WE\_HAVE\_AN\_X\_AND\_Y\_SCALE and WE\_HAVE\_A\_TWO\_BY\_TWO flags are mutually exclusive: no more than one of these may be set. If WE\_HAVE\_A\_SCALE is set, one additional F2DOT14 value is appended to the component glyph data; if WE\_HAVE\_AN\_X\_AND\_Y\_SCALE is set, two F2DOT14 values are appended; if WE\_HAVE\_A\_TWO\_BY\_TWO, four F2DOT14 values are appended. The appended values define an affine transformation, as described below. The child component glyph is transformed as it is incorporated into the parent composite glyph, prior to grid-fitting of the parent. The transform can affect an offset vector used to position the child; see discussion below of the SCALED\_COMPONENT\_OFFSET and UNSCALED\_COMPONENT\_OFFSET flags for details.

For defining the affine transformation, the general case is handled by the WE\_HAVE\_A\_TWO\_BY\_TWO flag. Four values are appended in the component glyph description: in order, *xscale*, *scale01*, *scale10*, and *yscale*. These form a two-by-two matrix used to transform x and y coordinates of points in the component glyph. For a pre-transformation position *(x, y)*, the post-transformation position *(x′, y′)* is calculated as follows:

*x′* = *xscale \* x + scale10 \* y*  
*y′* = *scale01 \* x + yscale \* y*

> *Note:* It can be helpful to understand a matrix transformation by the effect on *x* and *y* basis vectors *î = (1, 0)* and *ĵ = (0, 1)*. The transform described by the four values maps the basis vectors to *î′ = (xscale, scale01)* and *ĵ′ = (scale10, yscale)*.

When the WE\_HAVE\_AN\_X\_AND\_Y\_SCALE flag is used, the appended *xscale* and *yscale* values are used as above, but the transformation is constrained by *scale01* and *scale10* both being set to zero. When the WE\_HAVE\_A\_SCALE flag is used, the transformation is even further constrained by *xscale* and *yscale* both being set to the single appended value, *scale*.

The WE\_HAVE\_INSTRUCTIONS flag is used to indicate that the parent composite glyph has instructions, in addition to instructions for any of the child component glyphs. If the flag is set on any component glyph, then a uint16 value is read immediately after the last component glyph to get the byte length for instructions.

The purpose of USE\_MY\_METRICS is to force the lsb and rsb to take on values obtained from the component glyph. For example, an i-circumflex (U+00EF) is often composed of the circumflex and a dotless-i. In order to force the composite to have the same metrics as the dotless-i, set USE\_MY\_METRICS for the dotless-i component of the composite. Without this bit, the rsb and lsb would be calculated from the 'hmtx' entry for the composite (or would need to be explicitly set with TrueType instructions).

Note that the behavior of the USE\_MY\_METRICS operation is undefined for rotated component glyphs.

