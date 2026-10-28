# GPOS table structures

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-gpos-table-and-opentype-font-variations.md) · [Next](004-lookup-type-3-subtable-cursive-attachment-positioning.md)

<a id="gpos-table-structures"></a>

## GPOS table structures

<a id="header"></a>

<a id="gpos-header"></a>

### GPOS Header

The GPOS table begins with a header that contains a version number for the table. Two versions are defined. Version 1.0 contains offsets to three tables: ScriptList, FeatureList, and LookupList. Version 1.1 also includes an offset to a FeatureVariations table. For descriptions of these tables, see the chapter, [OpenType Layout Common Table Formats](../../chapter2/README.md) . Example 1 at the end of this chapter shows a GPOS Header version 1.0 table definition.

<a id="gposHeader_10"></a>

*GPOS Header, version 1.0*

<a id="majorVersion_10"></a>

<a id="minorVersion_10"></a>

<a id="scriptListOffset_10"></a>

<a id="featureListOffset_10"></a>

<a id="lookupListOffset_10"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the GPOS table, = 1. |
| uint16 | minorVersion | Minor version of the GPOS table, = 0. |
| Offset16 | scriptListOffset | Offset to ScriptList table, from beginning of GPOS table. |
| Offset16 | featureListOffset | Offset to FeatureList table, from beginning of GPOS table. |
| Offset16 | lookupListOffset | Offset to LookupList table, from beginning of GPOS table. |

<a id="gposHeader_11"></a>

*GPOS Header, version 1.1*

<a id="majorVersion_11"></a>

<a id="minorVersion_11"></a>

<a id="scriptListOffset_11"></a>

<a id="featureListOffset_11"></a>

<a id="lookupListOffset_11"></a>

<a id="featureVariationsOffset_11"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | majorVersion | Major version of the GPOS table, = 1. |
| uint16 | minorVersion | Minor version of the GPOS table, = 1. |
| Offset16 | scriptListOffset | Offset to ScriptList table, from beginning of GPOS table. |
| Offset16 | featureListOffset | Offset to FeatureList table, from beginning of GPOS table. |
| Offset16 | lookupListOffset | Offset to LookupList table, from beginning of GPOS table. |
| Offset32 | featureVariationsOffset | Offset to FeatureVariations table, from beginning of GPOS table (may be NULL). |

<a id="SP"></a>

<a id="lookup-type-1-subtable-single-adjustment-positioning"></a>

### Lookup type 1 subtable: single adjustment positioning

A single adjustment positioning subtable (SinglePos) is used to adjust the placement or advance of a single glyph, such as a subscript or superscript. In addition, a SinglePos subtable is commonly used to implement lookup data for contextual positioning.

A SinglePos subtable will have one of two formats: one that applies the same adjustment to a series of glyphs (format 1), and one that applies a different adjustment for each unique glyph (format 2).

<a id="SPF1"></a>

**Single adjustment positioning format 1: single positioning value**

A SinglePosFormat1 subtable applies the same positioning value or values to each glyph listed in its Coverage table. For instance, when a font uses old-style numerals, this format could be applied to uniformly lower the position of all math operator glyphs.

The format 1 subtable consists of a format identifier, an offset to a Coverage table that defines the glyphs to be adjusted by the positioning values, a ValueRecord, and a ValueFormat identifier that describes the amount and kinds of data in the ValueRecord.

The ValueRecord specifies one or more positioning values to be applied to all covered glyphs. For example, if all glyphs in the Coverage table require both horizontal and vertical adjustments, the ValueRecord will specify values for both xPlacement and yPlacement.

Example 2 at the end of this chapter shows a SinglePosFormat1 subtable used to adjust the placement of subscript glyphs.

<a id="singlePos_1"></a>

*SinglePosFormat1 subtable*

<a id="posFormat_sp1"></a>

<a id="coverageOffset_sp1"></a>

<a id="valueFormat_sp1"></a>

<a id="valueRecord_sp1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of SinglePos subtable. |
| uint16 | valueFormat | Defines the types of data in the ValueRecord. |
| ValueRecord | valueRecord | Defines positioning value(s) — applied to all glyphs in the Coverage table. |

<a id="SPF2"></a>

**Single adjustment positioning format 2: array of positioning values**

A SinglePosFormat2 subtable provides an array of ValueRecords that contains one positioning value for each glyph in the Coverage table. This format is more flexible than format 1, but it requires more space in the font file.

For example, assume that the Cyrillic script will be used in left-justified text. For all glyphs, format 2 could define position adjustments for left side bearings to align the left edges of the paragraphs. To achieve this, the Coverage table would list every glyph in the script, and the SinglePosFormat2 subtable would define a ValueRecord for each covered glyph. Correspondingly, each ValueRecord would specify an xPlacement adjustment value for the left side bearing.

> *Note:* A single ValueFormat applies to all ValueRecords defined in a SinglePos subtable. In this example, if xPlacement is the only value that a ValueRecord needs to optically align the glyphs, then the X\_PLACEMENT flag will be the only flag set in the valueFormat field of the subtable.

As in format 1, the format 2 subtable consists of a format identifier, an offset to a Coverage table that defines the glyphs to be adjusted by the positioning values, and the ValueFormat flags field that describes the amount and kinds of data in the ValueRecords. In addition, the format 2 subtable includes:

- A count of the ValueRecords. One ValueRecord is defined for each glyph in the Coverage table.
- An array of ValueRecords that specify positioning values. Because the array follows the Coverage Index order, the first ValueRecord applies to the first glyph listed in the Coverage table, and so on.

Example 3 at the end of this chapter shows how to adjust the spacing of three dash glyphs with a SinglePosFormat2 subtable.

<a id="singlePos_2"></a>

*SinglePosFormat2 subtable*

<a id="posFormat_sp2"></a>

<a id="coverageOffset_sp2"></a>

<a id="valueFormat_sp2"></a>

<a id="valueCount_sp2"></a>

<a id="valueRecords_sp2"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of SinglePos subtable. |
| uint16 | valueFormat | Defines the types of data in the ValueRecords. |
| uint16 | valueCount | Number of ValueRecords — must equal glyphCount in the Coverage table. |
| ValueRecord | valueRecords[valueCount] | Array of ValueRecords — positioning values applied to glyphs. |

<a id="PP"></a>

<a id="lookup-type-2-subtable-pair-adjustment-positioning"></a>

### Lookup type 2 subtable: pair adjustment positioning

A pair adjustment positioning subtable (PairPos) is used to adjust the placement or advances of two glyphs in relation to one another — for instance, to specify kerning data for pairs of glyphs. Compared to a typical kerning table, however, a PairPos subtable offers more flexibility and precise control over glyph positioning. The PairPos subtable can adjust each glyph in a pair independently in both the X and Y directions, and it can explicitly describe the particular type of adjustment applied to each glyph.

PairPos subtables can be either of two formats: one that identifies glyphs individually by index (format 1), and one that identifies glyphs by class (format 2).

<a id="PPF1"></a>

**Pair adjustment positioning format 1: adjustments for glyph pairs**

Format 1 uses glyph indices to access positioning data for one or more specific pairs of glyphs. All pairs are specified in the order determined by the layout direction of the text.

> *Note:* For text written from right to left, the right-most glyph will be the first glyph in a pair; conversely, for text written from left to right, the left-most glyph will be first.

A PairPosFormat1 subtable contains a format identifier and two ValueFormat fields:

- valueFormat1 applies to the ValueRecords for the first glyph in each pair. The single ValueFormat field applies to ValueRecords for all first glyphs. If valueFormat1 is set to zero (0), the corresponding glyph has no ValueRecord and, therefore, should not be repositioned.
- valueFormat2 applies to the ValueRecords for the second glyph in each pair. The single ValueFormat field applies to ValueRecords for all second glyphs. If valueFormat2 is set to 0, then the second glyph of the pair is the “next” glyph for which a lookup should be performed.

A PairPos subtable also defines an offset to a Coverage table that lists the indices of the first glyphs in each pair. More than one pair can have the same first glyph, but the Coverage table will list that glyph only once.

The subtable also contains an array of offsets to PairSet tables (pairSetOffsets) and a count of the defined tables. The pairSetOffsets array contains one offset for each glyph listed in the Coverage table and uses the same order as the Coverage Index.

<a id="pairPos_1"></a>

*PairPosFormat1 subtable*

<a id="posFormat_pp1"></a>

<a id="coverageOffset_pp2"></a>

<a id="valueFormat1_pp1"></a>

<a id="valueFormat2_pp1"></a>

<a id="pairSetCount_pp1"></a>

<a id="pairSetOffsets_pp1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of PairPos subtable. |
| uint16 | valueFormat1 | Defines the types of data in valueRecord1 — for the first glyph in the pair (may be zero). |
| uint16 | valueFormat2 | Defines the types of data in valueRecord2 — for the second glyph in the pair (may be zero). |
| uint16 | pairSetCount | Number of PairSet tables. |
| Offset16 | pairSetOffsets[pairSetCount] | Array of offsets to PairSet tables. Offsets are from beginning of PairPos subtable, ordered by Coverage Index. |

A PairSet table enumerates all the glyph pairs that begin with a covered glyph. An array of PairValue records contains one record for each pair and lists the records sorted by the glyph ID of the second glyph in each pair. The pairValueCount field specifies the number of PairValue records in the set.

<a id="pairSetTbl"></a>

*PairSet table*

<a id="pairValueCount_ps"></a>

<a id="pairValueRecords_ps"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | pairValueCount | Number of PairValue records. |
| PairValue | pairValueRecords[pairValueCount] | Array of PairValue records, ordered by glyph ID of the second glyph. |

A PairValue record specifies the second glyph in a pair and defines a ValueRecord for each glyph (valueRecord1 and valueRecord2). If valueFormat1 in the PairPos subtable is set to zero (0), valueRecord1 will be empty; similarly, if valueFormat2 is 0, valueRecord2 will be empty.

Example 4 at the end of this chapter shows a PairPosFormat1 subtable that defines two cases of pair kerning.

<a id="pairValueRec"></a>

*PairValue record*

<a id="secondGlyph_pvr"></a>

<a id="valueRecord1_pvr"></a>

<a id="valueRecord2_pvr"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | secondGlyph | Glyph ID of second glyph in the pair (first glyph is listed in the Coverage table). |
| ValueRecord | valueRecord1 | Positioning data for the first glyph in the pair. |
| ValueRecord | valueRecord2 | Positioning data for the second glyph in the pair. |

<a id="PPF2"></a>

**Pair adjustment positioning format 2: class pair adjustment**

Format 2 defines a pair as a set of two glyph classes and modifies the positions of all the glyphs in a class. For example, this format is useful in Japanese scripts that apply specific kerning operations to all glyph pairs that contain punctuation glyphs. One class would be defined as all glyphs that may be coupled with punctuation marks, and the other classes would be groups of similar punctuation glyphs.

In a PairPosFormat2 subtable, glyph classes are defined using a [Class Definition](../../chapter2/chunks/005-coverage-table.md#classDefTbl) table, defined in the OpenType Layout Common Table Formats chapter.

The PairPosFormat2 subtable begins with a format identifier and an offset to a Coverage table, measured from the beginning of the PairPos subtable. The Coverage table lists the indices of the first glyphs that may appear in each glyph pair. More than one pair may begin with the same glyph, but the Coverage table lists the glyph index only once.

A PairPosFormat2 subtable also includes two ValueFormat fields:

- valueFormat1 applies to the ValueRecords for the first glyph in each pair. The single ValueFormat field applies to ValueRecords for all first glyphs. If valueFormat1 is set to zero (0), then the ValueRecords for the first glyph will be empty and, therefore, the first glyph is not repositioned.
- valueFormat2 applies to the ValueRecords for the second glyph in each pair. The single ValueFormat field applies to ValueRecords for all second glyphs. If valueFormat2 is set to 0, then the ValueRecords for the second glyph of the pair will be empty, the second glyph is not repositioned, and it becomes the “next” glyph for which a lookup is performed.

PairPosFormat2 requires that each glyph in all pairs be assigned to a class, which is identified by an integer called a class value. Pairs are then represented in a two-dimensional array as sequences of two class values. Multiple pairs can be represented in one format 2 subtable.

