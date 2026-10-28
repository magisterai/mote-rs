# Lookup type 4 subtable: mark-to-base attachment positioning

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-lookup-type-3-subtable-cursive-attachment-positioning.md) · [Next](006-lookup-type-6-subtable-mark-to-mark-attachment-positioning.md)

<a id="MBP"></a>

<a id="lookup-type-4-subtable-mark-to-base-attachment-positioning"></a>

### Lookup type 4 subtable: mark-to-base attachment positioning

The mark-to-base attachment (MarkBasePos) subtable is used to position combining mark glyphs with respect to base glyphs. For example, the Arabic, Hebrew, and Thai scripts combine vowels, diacritical marks, and tone marks with base glyphs.

In the MarkBasePos subtable, every mark glyph has an anchor point and is also assigned to a mark class. Each base glyph then defines an anchor point for each class of marks it uses. When a mark is combined with a given base, the mark placement is adjusted so that the mark anchor is aligned with the base anchor for the applicable mark class. Placement of the base glyph and advances of both glyphs are not affected.

For example, assume two mark classes: all marks positioned above base glyphs (class 0), and all marks positioned below base glyphs (class 1). In this case, each base glyph that uses these marks would define two anchor points, one for attaching the mark glyphs listed in class 0, and one for attaching the mark glyphs listed in class 1.

To identify the base glyph that combines with a mark, the text-processing client must look backward in the glyph string from the mark to the preceding base glyph. To combine the mark and base glyph, the client aligns their attachment points, positioning the mark with respect to the final pen point (advance) position of the base glyph.

A mark class is identified by a specific integer. Within the MarkBasePos subtable, the anchor definition of each mark, and the assignment of each mark to a mark class, is provided using a MarkArray table in combination with a mark Coverage table. First, a mark Coverage table specifies all of the mark glyphs covered by the subtable. Then, for every mark in the Coverage table, the MarkArray table has a corresponding MarkRecord that defines the anchor and class assignment for the mark. The [MarkArray](008-mark-array-table.md#markArrayTable) table and MarkRecord are defined later in this chapter.

The MarkToBase Attachment subtable has one format: MarkBasePosFormat1.

<a id="MBPF1"></a>

**Mark-to-base attachment positioning format 1**

The MarkBasePosFormat1 subtable begins with a format identifier and offsets to two Coverage tables: one that lists all the mark glyphs referenced in the subtable, and one that lists all the base glyphs referenced in the subtable.

The MarkBasePosFormat1 subtable also contains an offset to a MarkArray table. For each mark glyph in the mark Coverage table, a MarkRecord in the MarkArray table specifies its class and an offset to the Anchor table that describes the mark’s attachment point. The classCount field specifies the total number of distinct mark classes defined in all the MarkRecords.

The MarkBasePosFormat1 subtable also contains an offset to a BaseArray table, which defines for each base glyph an array of anchors, one for each mark class.

<a id="markBase_1"></a>

*MarkBasePosFormat1 subtable*

<a id="posFormat_mb1"></a>

<a id="markCoverageOffset_mb1"></a>

<a id="baseCoverageOffset_mb1"></a>

<a id="markClassCount_mb1"></a>

<a id="markArrayOffset_mb1"></a>

<a id="baseArrayOffset_mb1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | markCoverageOffset | Offset to markCoverage table, from beginning of MarkBasePos subtable. |
| Offset16 | baseCoverageOffset | Offset to baseCoverage table, from beginning of MarkBasePos subtable. |
| uint16 | markClassCount | Number of classes defined for marks. |
| Offset16 | markArrayOffset | Offset to MarkArray table, from beginning of MarkBasePos subtable. |
| Offset16 | baseArrayOffset | Offset to BaseArray table, from beginning of MarkBasePos subtable. |

The BaseArray table consists of an array of BaseRecords. The array stores the BaseRecords in the same order as the baseCoverage index. Each base glyph in the baseCoverage table has a BaseRecord.

<a id="baseArrayTbl"></a>

*BaseArray table*

<a id="baseCount_ba"></a>

<a id="baseRecords_ba"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | baseCount | Number of BaseRecords. |
| BaseRecord | baseRecords[baseCount] | Array of BaseRecords, in order of baseCoverage Index. |

A BaseRecord declares one Anchor table for each mark class (including class 0) identified in the MarkRecords of the MarkArray table. Each Anchor table specifies one attachment point used to attach all the marks in a particular class to the base glyph. A BaseRecord contains an array of offsets to Anchor tables. The zero-based array of offsets defines the entire set of attachment points each base glyph uses to attach marks. The offsets to Anchor tables are ordered by mark class.

> *Note:* Anchor tables are not tagged with class value identifiers. Instead, the index value of an Anchor table in the array defines the class value represented by the Anchor table.

Example 7 at the end of this chapter defines mark positioning above and below base glyphs with a MarkBasePosFormat1 subtable.

<a id="baseRec"></a>

*BaseRecord*

<a id="baseAnchorOffsets"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | baseAnchorOffsets[markClassCount] | Array of offsets (one per mark class) to Anchor tables. Offsets are from beginning of BaseArray table, ordered by class (offsets may be NULL). |

<a id="MLP"></a>

<a id="lookup-type-5-subtable-mark-to-ligature-attachment-positioning"></a>

### Lookup type 5 subtable: mark-to-ligature attachment positioning

The mark-to-ligature attachment (MarkLigPos) subtable is used to position combining mark glyphs with respect to ligature base glyphs. With mark-to-base attachment, described previously, each base glyph has an attachment point defined for each class of marks. Mark-to-ligature attachment is similar, except that each ligature glyph is defined to have multiple components (in a virtual sense — not actual glyphs), and each component has a separate set of attachment points defined for the different mark classes.

As a result, a ligature glyph may have multiple base attachment points for one class of marks. For a given mark assigned to a particular class, the appropriate base attachment point is determined by which ligature component the mark is associated with. This is dependent on the original character string and subsequent character- or glyph-sequence processing, not the font data alone. While a text-layout client is performing any character-based preprocessing or any glyph-substitution operations using the [GSUB](../../gsub/README.md) table, the text-layout client must keep track of associations of marks to particular ligature-glyph components.

The MarkLigPos subtable can be used to define multiple mark-to-ligature attachments. In the subtable, every mark glyph has an anchor point and is associated with a class of marks. As with MarkToBase attachment, mark anchors and class assignments are defined using a MarkArray table in combination with a mark Coverage table. Every ligature glyph specifies a two-dimensional array of data: for each component in a ligature, an array of anchor points is defined, one for each class of marks.

For example, assume two mark classes: all marks positioned above base glyphs (class 0), and all marks positioned below base glyphs (class 1). In this case, each component of a base ligature glyph may define two anchor points, one for attaching the mark glyphs listed in class 0, and one for attaching the mark glyphs listed in class 1. Alternatively, if the language system does not allow marks on the second component, the first ligature component may define two anchor points, one for each class of marks, and the second ligature component may define no anchor points.

To position a combining mark using a MarkLigPos subtable, the text-processing client must work backward from the mark to the preceding ligature glyph. To correctly access the subtables, the client must keep track of the component associated with the mark. Aligning the attachment points combines the mark and ligature.

As with mark-to-base attachment, when a mark is combined with a given ligature base, the mark placement is adjusted so that the mark anchor is aligned with the applicable base anchor. Placement of the base glyph and advances of both glyphs are not affected.

The MarkToLigature attachment subtable has one format: MarkLigPosFormat1.

<a id="MLPF1"></a>

**Mark-to-ligature attachment positioning format 1**

The MarkLigPosFormat1 subtable begins with a format identifier and two offsets to Coverage tables that list all the mark glyphs and ligature glyphs referenced in the subtable.

The MarkLigPosFormat1 subtable also contains an offset to a MarkArray table. For each mark glyph in the mark Coverage table, a MarkRecord in the MarkArray table specifies its class and an offset to the Anchor table that describes the mark’s attachment point. The markClassCount field specifies the total number of distinct mark classes defined in all the MarkRecords.

The MarkLigPosFormat1 subtable also contains an offset to a LigatureArray table, which defines for each ligature glyph the two-dimensional array of anchor data: one anchor per ligature component per mark class.

<a id="markLigPos_1"></a>

*MarkLigPosFormat1 subtable*

<a id="posFormat_ml1"></a>

<a id="markCoverageOffset_ml1"></a>

<a id="ligatureCoverageOffset_ml1"></a>

<a id="markClassCount_ml1"></a>

<a id="markArrayOffset_ml1"></a>

<a id="ligatureArrayOffset_ml1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | markCoverageOffset | Offset to markCoverage table, from beginning of MarkLigPos subtable. |
| Offset16 | ligatureCoverageOffset | Offset to ligatureCoverage table, from beginning of MarkLigPos subtable. |
| uint16 | markClassCount | Number of defined mark classes. |
| Offset16 | markArrayOffset | Offset to MarkArray table, from beginning of MarkLigPos subtable. |
| Offset16 | ligatureArrayOffset | Offset to LigatureArray table, from beginning of MarkLigPos subtable. |

The LigatureArray table contains an array of offsets to LigatureAttach tables. The ligatureAttachOffsets array lists the offsets to LigatureAttach tables, one for each ligature glyph listed in the ligatureCoverage table, in the same order as the ligatureCoverage index.

<a id="ligatureArrayTbl"></a>

*LigatureArray table*

<a id="ligatureCount_ligArray"></a>

<a id="ligatureAttachOffsets_ligArray"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | ligatureCount | Number of LigatureAttach table offsets. |
| Offset16 | ligatureAttachOffsets[ligatureCount] | Array of offsets to LigatureAttach tables. Offsets are from beginning of LigatureArray table, ordered by ligatureCoverage index. |

Each LigatureAttach table consists of an array of ComponentRecords, which describe the component glyphs in a ligature. The array stores the ComponentRecords in the same order as the components in the ligature. The order of the records also corresponds to the writing direction — that is, the logical direction — of the text. For text written left to right, the first component is on the left; for text written right to left, the first component is on the right.

<a id="ligatureAttachTbl"></a>

*LigatureAttach table*

<a id="componentCount_ligAttach"></a>

<a id="componentRecs_ligAttach"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | componentCount | Number of ComponentRecords in this ligature. |
| ComponentRecord | componentRecords[componentCount] | Array of Component records, ordered in writing direction. |

A ComponentRecord, one for each component in the ligature, contains an array of offsets to the Anchor tables that define all the attachment points used to attach marks to the component. For each mark class (including class 0) identified in the MarkArray records, an Anchor table specifies the point used to attach all the marks in a particular class to the ligature base glyph, relative to the component.

In a ComponentRecord, the zero-based ligatureAnchorOffsets array lists offsets to Anchor tables by mark class. If a component does not define an attachment point for a particular class of marks, then the offset to the corresponding Anchor table will be NULL.

Example 8 at the end of this chapter shows a MarkLigPosFormat1 subtable used to attach mark accents to a ligature glyph in the Arabic script.

<a id="componentRec"></a>

*ComponentRecord*

<a id="ligatureAnchorOffsets"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | ligatureAnchorOffsets[markClassCount] | Array of offsets (one per class) to Anchor tables. Offsets are from beginning of LigatureAttach table, ordered by class (offsets may be NULL). |

