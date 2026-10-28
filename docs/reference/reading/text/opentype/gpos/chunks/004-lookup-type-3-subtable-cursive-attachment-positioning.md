# Lookup type 3 subtable: cursive attachment positioning

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-gpos-table-structures.md) · [Next](005-lookup-type-4-subtable-mark-to-base-attachment-positioning.md)

A PairPosFormat2 subtable contains offsets to two class definition tables: one that assigns class values to all the first glyphs in all pairs (classDef1), and one that assigns class values to all the second glyphs in all pairs (classDef2). If both glyphs in a pair use the same class definition, the offset value can be the same for classDef1 as for classDef2, but they are not required to be the same. The subtable also specifies the number of glyph classes defined in classDef1 and in classDef2, including class 0.

For each class identified in the classDef1 table, a Class1 record enumerates all pairs that contain a particular class as a first component. The class1Records array stores all Class1 records according to class value.

> *Note:* Class1 records are not tagged with a class value identifier. Instead, the index value of a Class1 record in the array defines the class value represented by the record. For example, the first Class1 record enumerates pairs that begin with a class 0 glyph, the second Class1 record enumerates pairs that begin with a class 1 glyph, and so on.

<a id="pairPos_2"></a>

*PairPosFormat2 subtable*

<a id="valueFormat1_pp2"></a>

<a id="valueFormat2_pp2"></a>

<a id="classDef1Offset_pp2"></a>

<a id="classDef2Offset_pp2"></a>

<a id="classDef1Count_pp2"></a>

<a id="classDef2Count_pp2"></a>

<a id="class1Records_pp2"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of PairPos subtable. |
| uint16 | valueFormat1 | ValueRecord definition — for the first glyph of the pair (may be zero). |
| uint16 | valueFormat2 | ValueRecord definition — for the second glyph of the pair (may be zero). |
| Offset16 | classDef1Offset | Offset to ClassDef table, from beginning of PairPos subtable — for the first glyph of the pair. |
| Offset16 | classDef2Offset | Offset to ClassDef table, from beginning of PairPos subtable — for the second glyph of the pair. |
| uint16 | class1Count | Number of classes in classDef1 table — includes class 0. |
| uint16 | class2Count | Number of classes in classDef2 table — includes class 0. |
| Class1 | class1Records[class1Count] | Array of Class1 records, ordered by classes in classDef1. |

Each Class1 record contains an array of Class2 records, which also are ordered by class value. One Class2 record must be declared for each class in the classDef2 table, including class 0.

<a id="class1Rec"></a>

*Class1 record*

<a id="class2Records_c1Rec"></a>

| Type | Name | Description |
| --- | --- | --- |
| Class2 | class2Records[class2Count] | Array of Class2 records, ordered by classes in classDef2. |

A Class2 record consists of two ValueRecords, one for the first glyph in a class pair (valueRecord1) and one for the second glyph (valueRecord2). Note that both fields of a Class2 record are optional: If the PairPos subtable has a value of zero (0) for valueFormat1 or valueFormat2, then the corresponding record (valueRecord1 or valueRecord2) will be empty — that is, not present. For example, if valueFormat1 is zero, then the Class2 record will begin with and consist solely of valueRecord2. The text-processing client must be aware of the variable nature of the Class2 record and use the valueFormat1 and valueFormat2 fields to determine the size and content of the Class2 record.

Example 5 at the end of this chapter demonstrates pair kerning with glyph classes in a PairPosFormat2 subtable.

<a id="class2Rec"></a>

*Class2 record*

<a id="value1_c2"></a>

<a id="value2_c2"></a>

| Type | Name | Description |
| --- | --- | --- |
| ValueRecord | valueRecord1 | Positioning for first glyph — empty if valueFormat1 = 0. |
| ValueRecord | valueRecord2 | Positioning for second glyph — empty if valueFormat2 = 0. |

<a id="CAP"></a>

<a id="lookup-type-3-subtable-cursive-attachment-positioning"></a>

### Lookup type 3 subtable: cursive attachment positioning

Some cursive fonts are designed so that adjacent glyphs join when rendered with their default positioning. However, if positioning adjustments are needed to join the glyphs, a cursive attachment positioning (CursivePos) subtable can describe how to connect the glyphs by aligning two anchor points: the designated exit point of a glyph, and the designated entry point of the following glyph.

Anchor alignment can result in horizontal or vertical positioning adjustments, or both. Note that the positioning effects in the text-layout direction (horizontal, for horizontal layout) work differently than for the *cross-stream* direction (vertical, in horizontal layout):

- For adjustments in the line-layout direction, the layout engine adjusts the advance of the first glyph (in logical order). This effectively moves the second glyph relative to the first so that the anchors are aligned in that direction.
- For the cross-stream direction, placement of one glyph is adjusted to make the anchors align. Which glyph is adjusted is determined by the [RIGHT\_TO\_LEFT flag](../../chapter2/chunks/004-lookuplist-table.md#lookupFlags_1) in the parent lookup table: if the RIGHT\_TO\_LEFT flag is clear, the second glyph is adjusted to align anchors with the first glyph; if the RIGHT\_TO\_LEFT flag is set, the first glyph is adjusted to align anchors with the second glyph.

Note that, if the RIGHT\_TO\_LEFT lookup flag is set, then the *last* glyph in the connected sequence keeps its initial position in the cross-stream direction relative to the baseline, and the cross-stream positions of the preceding, connected glyphs are adjusted.

The subtable has one format: CursivePosFormat1.

<a id="CAPF1"></a>

**Cursive attachment positioning format 1**

The CursivePosFormat1 subtable begins with a format identifier and an offset to a Coverage table, which lists all the glyphs that define cursive attachment data. In addition, the subtable contains an array of EntryExit records: one for each glyph listed in the Coverage table, sorted in Coverage index order.

<a id="cursivePos_1"></a>

*CursivePosFormat1 subtable*

<a id="posFormat_cp1"></a>

<a id="coverageOffset_cp1"></a>

<a id="entryExitCount_cp1"></a>

<a id="entryExitRecords_cp1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of CursivePos subtable. |
| uint16 | entryExitCount | Number of EntryExit records. |
| EntryExit | entryExitRecords[entryExitCount] | Array of EntryExit records, in Coverage index order. |

Each EntryExit record consists of two offsets: one to an Anchor table that identifies the entry point on the glyph, and an offset to an Anchor table that identifies the exit point on the glyph. See later in this chapter for a description of the Anchor table.

To position glyphs using the CursivePosFormat1 subtable, a text-processing client aligns the exit anchor point of a glyph with the entry anchor point of the following glyph. If no corresponding anchor point exists, the offset for either the entry or exit Anchor table may be NULL, in which case no positioning adjustment is applied.

At the end of this chapter, Example 6 describes cursive glyph attachment in the Urdu language.

<a id="entryExitRec"></a>

*EntryExit record*

<a id="entryAnchor_ee"></a>

<a id="exitAnchor_ee"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | entryAnchorOffset | Offset to entryAnchor table, from beginning of CursivePos subtable (may be NULL). |
| Offset16 | exitAnchorOffset | Offset to exitAnchor table, from beginning of CursivePos subtable (may be NULL). |

