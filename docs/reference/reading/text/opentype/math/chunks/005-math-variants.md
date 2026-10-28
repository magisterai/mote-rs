# Math variants

Source: [Mathematical typesetting table](https://learn.microsoft.com/en-us/typography/opentype/spec/math) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/math).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-mathglyphinfo-table.md) · [Next](006-opentype-layout-tags-used-with-the-math-table.md)

<a id="math-variants"></a>

### Math variants

<a id="mathVariants"></a>

<a id="mathvariants-table"></a>

#### MathVariants table

The MathVariants table solves the following problem: given a particular default glyph shape and a certain width or height, find a variant shape glyph (or construct created by putting several glyphs together) that has the required measurement. This functionality is needed for growing the parentheses to match the height of the expression within, growing the radical sign to match the height of the expression under the radical, stretching accents like tilde when they are put over several characters, for stretching arrows, horizontal curly braces, and so forth.

The MathVariants table consists of the following fields:

- Count and coverage of glyphs that can grow in the vertical direction.
- Count and coverage of glyphs that can grow in the horizontal direction.
- The minConnectorOverlap field defines by how much two glyphs need to overlap with each other when used to construct a larger shape. Each glyph to be used as a building block in constructing extended shapes will have a straight part at either or both ends. This connector part is used to connect that glyph to other glyphs in the assembly. These connectors need to overlap to compensate for rounding errors and hinting corrections at a lower resolution. The minConnectorOverlap value tells how much overlap is necessary for this particular font.
- Two arrays of offsets to MathGlyphConstruction tables: one array for glyphs that grow in the vertical direction, and the other array for glyphs that grow in the horizontal direction. The arrays must be arranged in coverage order and have specified sizes.

<a id="mathVariantsTable"></a>

*MathVariants table*

| Type | Name | Description |
| --- | --- | --- |
| UFWORD | minConnectorOverlap | Minimum overlap of connecting glyphs during glyph construction, in design units. |
| Offset16 | vertGlyphCoverageOffset | Offset to Coverage table, from the beginning of the MathVariants table. |
| Offset16 | horizGlyphCoverageOffset | Offset to Coverage table, from the beginning of the MathVariants table. |
| uint16 | vertGlyphCount | Number of glyphs for which information is provided for vertically growing variants. Must be the same as the number of glyph IDs referenced in the vertical Coverage table. |
| uint16 | horizGlyphCount | Number of glyphs for which information is provided for horizontally growing variants. Must be the same as the number of glyph IDs referenced in the horizontal Coverage table. |
| Offset16 | vertGlyphConstructionOffsets[vertGlyphCount] | Array of offsets to MathGlyphConstruction tables, from the beginning of the MathVariants table, for shapes growing in the vertical direction. |
| Offset16 | horizGlyphConstructionOffsets[horizGlyphCount] | Array of offsets to MathGlyphConstruction tables, from the beginning of the MathVariants table, for shapes growing in the horizontal direction. |

<a id="mathGlyphConstruction"></a>

<a id="mathglyphconstruction-table"></a>

#### MathGlyphConstruction table

The MathGlyphConstruction table provides information on finding or assembling extended variants for one particular glyph. It can be used for shapes that grow in either horizontal or vertical directions.

The first entry is the offset to the GlyphAssembly table that specifies how the shape for this glyph can be assembled from parts found in the glyph set of the font. If no such assembly exists, this offset will be set to NULL.

The MathGlyphConstruction table also contains the count and array of ready-made glyph variants for the specified glyph. Each variant consists of the glyph index and this glyph’s measurement in the direction of extension (vertical or horizontal).

Note that it is quite possible that both the GlyphAssembly table and some variants are defined for a particular glyph. For example, the font may provide several variants of curly braces with different sizes, and also a general mechanism for constructing larger versions of curly braces by stacking parts found in the glyph set. First, an attempt is made to find glyph among provided variants. If the required size is larger than any of the glyph variants provided, however, then the general mechanism can be employed to typeset the curly braces as a glyph assembly.

<a id="mathGlyphConstructionTable"></a>

*MathGlyphConstruction table*

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | glyphAssemblyOffset | Offset to the GlyphAssembly table for this shape, from the beginning of the MathGlyphConstruction table. May be NULL. |
| uint16 | variantCount | Count of glyph growing variants for this glyph. |
| MathGlyphVariantRecord | mathGlyphVariantRecords[variantCount] | MathGlyphVariantRecords for alternative variants of the glyphs. |

<a id="mathGlyphVariantRecordFormat"></a>

*MathGlyphVariantRecord*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | variantGlyph | Glyph ID for the variant. |
| UFWORD | advanceMeasurement | Advance width/height, in design units, of the variant, in the direction of requested glyph extension. |

<a id="glyphAssembly"></a>

<a id="glyphassembly-table"></a>

#### GlyphAssembly table

The GlyphAssembly table specifies how the shape for a particular glyph can be constructed from parts found in the glyph set. The table defines the italics correction of the resulting assembly, and a number of parts that have to be put together to form the required shape. Some glyph parts can be designated as extenders, which can be repeated as needed to obtain a target size.

<a id="glyphAssemblyTable"></a>

*GlyphAssembly table*

| Type | Name | Description |
| --- | --- | --- |
| MathValueRecord | italicsCorrection | Italics correction of this GlyphAssembly. Should not depend on the assembly size. |
| uint16 | partCount | Number of parts in this assembly. |
| GlyphPart | partRecords[partCount] | Array of GlyphPart records, from left to right (for assemblies that extend horizontally) or bottom to top (for assemblies that extend vertically). |

The result of the assembly process is an array of glyphs with an offset specified for each of those glyphs. When drawn consecutively at those offsets, the glyphs should combine correctly and produce the required shape.

The offsets in the direction of growth (advance offsets), as well as the number of extender parts, are determined based on the size requirement for the resulting assembly.

Note that the glyphs comprising the assembly should be designed so that they align properly in the direction that is orthogonal to the direction of growth.

Thus, a GlyphPart record consists of the following fields:

- Glyph ID for the part.
- Lengths of the connectors on each end of the glyph. The connectors are straight parts of the glyph that can be used to link it with the next or previous part. The connectors of neighboring parts can overlap by varying amounts, providing flexibility in how these glyphs can be put together. However, the overlap should not be less than the minConnectorOverlap value defined in the MathVariants tables, and it should not exceed the specified connector length for that end of the glyph. If the part does not have a connector on one of its ends, the corresponding connector length should be set to zero.
- The full advance of the part. It is also used to determine the measurement of the result by using the following formula:  
    
  *Size of Assembly = Offset of the Last Part + Full Advance of the Last Part*
- PartFlags is the last field. It identifies certain parts as *extenders*: those parts that can be repeated (that is, multiple instances of them can be used in place of one) or skipped altogether. Usually, the extenders are vertical or horizontal bars of the appropriate thickness, aligned with the rest of the assembly.

To ensure that the width/height is distributed equally and the symmetry of the shape is preserved, the following steps can be used by the math-layout engine.

1. Assemble all parts with all extenders removed and with connections overlapping by the maximum amount. This gives the smallest possible result.
2. Determine how much extra width/height can be obtained from all existing connections between neighboring parts by using minimal overlaps. If that is enough to achieve the size goal, extend each connection equally by changing overlaps of connectors to finish the job.
3. If all connections have been extended to the minimum overlap and further growth is needed, add one of each extender, and repeat the process from the first step.

Note that for assemblies growing in the vertical direction, the distribution of height between ascent and descent is not defined. The math-layout engine is responsible for positioning the resulting assembly relative to the baseline.

<a id="glyphPartRecord"></a>

*GlyphPart record*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | glyphID | Glyph ID for the part. |
| UFWORD | startConnectorLength | Advance width/ height, in design units, of the straight bar connector material at the start of the glyph in the direction of the extension (the left end for horizontal extension, the bottom end for vertical extension). |
| UFWORD | endConnectorLength | Advance width/ height, in design units, of the straight bar connector material at the end of the glyph in the direction of the extension (the right end for horizontal extension, the top end for vertical extension). |
| UFWORD | fullAdvance | Full advance width/height for this part in the direction of the extension, in design units. |
| uint16 | partFlags | Part qualifiers. PartFlags enumeration currently uses only one bit:  0x0001 EXTENDER\_FLAG: If set, the part can be skipped or repeated.  0xFFFE Reserved. |

