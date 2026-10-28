# GSUB table structures

Source: [Glyph substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-gsub-table-organization.md) · [Next](004-lookup-type-3-subtable-alternate-substitution.md)

<a id="gsub-table-structures"></a>

## GSUB table structures

<a id="header"></a>

<a id="gsub-header"></a>

### GSUB Header

The GSUB table begins with a header that contains a version number for the table and offsets to three tables: ScriptList, FeatureList, and LookupList. For descriptions of each of these tables, see the chapter, OpenType Layout Common Table Formats. [Example 1](006-gsub-structure-examples.md#EX1) at the end of this chapter shows a GSUB Header version 1.0 table definition.

<a id="gsubHeader_10"></a>

*GSUB Header, version 1.0*

<a id="majorVersion_10"></a>

<a id="minorVersion_10"></a>

<a id="scriptListOffset_10"></a>

<a id="featureListOffset_10"></a>

<a id="lookupListOffset_10"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the GSUB table, = 1. |
| uint16 | minorVersion | Minor version of the GSUB table, = 0. |
| Offset16 | scriptListOffset | Offset to ScriptList table, from beginning of GSUB table. |
| Offset16 | featureListOffset | Offset to FeatureList table, from beginning of GSUB table. |
| Offset16 | lookupListOffset | Offset to LookupList table, from beginning of GSUB table. |

<a id="gsubHeader_11"></a>

*GSUB Header, version 1.1*

<a id="majorVersion_11"></a>

<a id="minorVersion_11"></a>

<a id="scriptListOffset_11"></a>

<a id="featureListOffset_11"></a>

<a id="lookupListOffset_11"></a>

<a id="featureVariationsOffset_11"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the GSUB table, = 1. |
| uint16 | minorVersion | Minor version of the GSUB table, = 1. |
| Offset16 | scriptListOffset | Offset to ScriptList table, from beginning of GSUB table. |
| Offset16 | featureListOffset | Offset to FeatureList table, from beginning of GSUB table. |
| Offset16 | lookupListOffset | Offset to LookupList table, from beginning of GSUB table. |
| Offset32 | featureVariationsOffset | Offset to FeatureVariations table, from beginning of the GSUB table (may be NULL). |

<a id="SS"></a>

<a id="lookup-type-1-subtable-single-substitution"></a>

### Lookup type 1 subtable: single substitution

Single substitution (SingleSubst) subtables tell a client to replace a single glyph with another glyph. The subtables can be either of two formats. Both formats require two distinct sets of glyph indices: one that defines input glyphs (specified in the Coverage table), and one that defines the output glyphs. Format 1 requires less space than format 2, but it is less flexible.

<a id="SSF1"></a>

**Single substitution format 1**

Format 1 calculates the indices of the output glyphs, which are not explicitly defined in the subtable. To calculate an output glyph index, format 1 adds a constant delta value to the input glyph index. The input and output glyphs do not need to be in continuous glyph ID ranges, but the delta between input glyph IDs and output glyph IDs need to be constant. This format does not use the Coverage index that is returned from the Coverage table.

The SingleSubstFormat1 subtable begins with a format identifier of 1. An offset references a Coverage table that specifies the indices of the input glyphs. The deltaGlyphID is a constant value added to each input glyph index to calculate the index of the corresponding output glyph. Addition of deltaGlyphID is modulo 65536. If the result after adding deltaGlyphID to the input glyph index is less than zero, add 65536 to obtain a valid glyph ID.

[Example 2](006-gsub-structure-examples.md#EX2) at the end of this chapter uses format 1 to replace standard numerals with lining numerals.

<a id="singleSubst_1"></a>

*SingleSubstFormat1 subtable*

<a id="substFormat_ss1"></a>

<a id="coverageOffset_ss1"></a>

<a id="deltaGlyphID_ss1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier: format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of substitution subtable. |
| int16 | deltaGlyphID | Add to original glyph ID to get substitute glyph ID. |

<a id="SSF2"></a>

**Single substitution format 2**

Format 2 is more flexible than format 1 but requires more space. It provides an array of output glyph indices (substituteGlyphIDs) explicitly matched to the input glyph indices specified in the Coverage table.

The SingleSubstFormat2 subtable specifies a format identifier, an offset to a Coverage table that defines the input glyph indices, and an array of output glyph indices (substituteGlyphIDs).

The substituteGlyphIDs array must contain the same number of glyph indices as the Coverage table, and the glyphs must be ordered to match the order of corresponding input glyphs in the Coverage table. To locate the corresponding output glyph index in the substituteGlyphIDs array, this format uses the Coverage index returned from the Coverage table.

[Example 3](006-gsub-structure-examples.md#EX3) at the end of this chapter uses format 2 to substitute vertically oriented glyphs for horizontally oriented glyphs.

<a id="singleSubst_2"></a>

*SingleSubstFormat2 subtable*

<a id="substFormat_ss2"></a>

<a id="coverageOffset_ss2"></a>

<a id="glyphCount_ss2"></a>

<a id="substituteGlyphIDs_ss2"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier: format = 2. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of substitution subtable. |
| uint16 | glyphCount | Number of glyph IDs in the substituteGlyphIDs array. |
| uint16 | substituteGlyphIDs[glyphCount] | Array of substitute glyph IDs — ordered by Coverage index. |

<a id="MS"></a>

<a id="lookup-type-2-subtable-multiple-substitution"></a>

### Lookup type 2 subtable: multiple substitution

A multiple substitution (MultipleSubst) subtable replaces a single glyph with a sequence of glyphs, as when multiple glyphs replace a single ligature. The subtable has a single format.

<a id="MSF1"></a>

**Multiple substitution format 1**

The MultipleSubstFormat1 subtable specifies a format identifier, an offset to a Coverage table that defines the input glyph indices, and an array of offsets to Sequence tables that define the output glyph indices. The Sequence table offsets are ordered by the Coverage index of the input glyphs.

For each input glyph listed in the Coverage table, a Sequence table defines the output glyphs. Each Sequence table contains a count of the glyphs in the output glyph sequence and an array of output glyph indices.

> *Note:* The order of the output glyph indices depends on the writing direction of the text. For text written left to right, the left-most glyph will be first glyph in the sequence. Conversely, for text written right to left, the right-most glyph will be first.

The use of multiple substitution for deletion of an input glyph is prohibited. The glyphCount value must always be greater than 0.

[Example 4](006-gsub-structure-examples.md#EX4) at the end of this chapter shows how to replace a single ligature with three glyphs.

<a id="multipleSubst_1"></a>

*MultipleSubstFormat1 subtable*

<a id="substFormat_ms1"></a>

<a id="coverageOffset_ms1"></a>

<a id="sequenceCount_ms1"></a>

<a id="sequenceOffsets_ms1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier: format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of substitution subtable. |
| uint16 | sequenceCount | Number of Sequence table offsets in the sequenceOffsets array. |
| Offset16 | sequenceOffsets[sequenceCount] | Array of offsets to Sequence tables. Offsets are from beginning of substitution subtable, ordered by Coverage index. |

<a id="sequenceTable"></a>

*Sequence table*

<a id="glyphCount_seqTbl"></a>

<a id="substituteGlyphIDs_seqTbl"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | glyphCount | Number of glyph IDs in the substituteGlyphIDs array. This must always be greater than 0. |
| uint16 | substituteGlyphIDs[glyphCount] | String of glyph IDs to substitute. |

