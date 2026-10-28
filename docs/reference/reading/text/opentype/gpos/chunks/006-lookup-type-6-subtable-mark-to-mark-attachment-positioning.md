# Lookup type 6 subtable: mark-to-mark attachment positioning

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-lookup-type-4-subtable-mark-to-base-attachment-positioning.md) · [Next](007-lookup-type-8-subtable-chained-contexts-positioning.md)

<a id="MMP"></a>

<a id="lookup-type-6-subtable-mark-to-mark-attachment-positioning"></a>

### Lookup type 6 subtable: mark-to-mark attachment positioning

The mark-to-mark attachment (MarkMarkPos) subtable is identical in form to the mark-to-base attachment subtable, although its function is different. Mark-to-mark attachment defines the position of one mark relative to another mark as when, for example, positioning tone marks with respect to vowel diacritical marks in Vietnamese.

The attaching mark is mark1, and the base mark being attached to is mark2. In the MarkMarkPos subtable, every mark1 glyph has an anchor attachment point and is assigned to a class of marks. As for mark glyphs in a mark-to-base attachment, the anchor point and class assignment of mark1 glyphs is defined using a MarkArray table in combination with a mark1 Coverage table. Each mark2 glyph defines an anchor point for each class of mark1 marks.

For example, assume two mark1 classes: all marks positioned to the left of mark2 glyphs (class 0), and all marks positioned to the right of mark2 glyphs (class 1). Each mark2 glyph that uses these marks defines two anchor points: one for attaching the mark1 glyphs listed in class 0, and one for attaching the mark1 glyphs listed in class 1.

The mark2 glyph that combines with a mark1 glyph is the glyph preceding the mark1 glyph in glyph string order (skipping glyphs according to LookupFlags). The subtable applies precisely when that mark2 glyph is covered by the mark2 Coverage. To combine the mark glyphs, the placement of the mark1 glyph is adjusted such that the relevant attachment points coincide. Advance widths are not affected. The input context for mark-to-base, mark-to-ligature and mark-to-mark positioning tables is the mark that is being positioned. If a sequence contains several marks, a lookup may act on it several times, to position them.

The mark-to-mark attachment subtable has one format: MarkMarkPosFormat1.

<a id="MMPF1"></a>

**Mark-to-mark attachment positioning format 1**

The MarkMarkPosFormat1 subtable begins with a format identifier and two offsets to Coverage tables: one that lists all the mark1 glyphs referenced in the subtable (mark1Coverage), and one that lists all the mark2 glyphs referenced in the subtable (mark2Coverage).

The subtable also has an offset to a MarkArray table for the mark1 glyphs. For each mark glyph in the mark1Coverage table, a MarkRecord in the MarkArray table specifies its class and an offset to the Anchor table that describes the mark’s attachment point. The markClassCount field specifies the total number of distinct mark classes defined in all the MarkRecords.

The MarkMarkPosFormat1 subtable also has an offset to a *Mark2Array* table, which defines for each mark2 glyph an array of anchors, one for each mark1 mark class.

<a id="markMarkPos_1"></a>

*MarkMarkPosFormat1 subtable*

<a id="posFormat_mm1"></a>

<a id="mark1CoverageOffset_mm1"></a>

<a id="mark2CoverageOffset_mm1"></a>

<a id="markClassCount_mm1"></a>

<a id="mark1ArrayOffset_mm1"></a>

<a id="mark2ArrayOffset_mm1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | mark1CoverageOffset | Offset to Combining Mark Coverage table, from beginning of MarkMarkPos subtable. |
| Offset16 | mark2CoverageOffset | Offset to Base Mark Coverage table, from beginning of MarkMarkPos subtable. |
| uint16 | markClassCount | Number of Combining Mark classes defined. |
| Offset16 | mark1ArrayOffset | Offset to MarkArray table for mark1, from beginning of MarkMarkPos subtable. |
| Offset16 | mark2ArrayOffset | Offset to Mark2Array table for mark2, from beginning of MarkMarkPos subtable. |

The Mark2Array table contains one Mark2 record for each mark2 glyph listed in the mark2Coverage table. It stores the records in the same order as the mark2Coverage index.

<a id="mark2ArrayTbl"></a>

*Mark2Array table*

<a id="mark2Count_m2a"></a>

<a id="mark2Records_m2a"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | mark2Count | Number of Mark2 records. |
| Mark2 | mark2Records[mark2Count] | Array of Mark2 records, in Coverage order. |

Each Mark2 record contains an array of offsets to Anchor tables (mark2AnchorOffsets). The zero-based array of offsets, measured from the beginning of the Mark2Array table, defines the entire set of mark2 attachment points used to attach mark1 glyphs to a specific mark2 glyph. The Anchor tables referenced in the mark2AnchorOffsets array are ordered by mark1 class value.

A Mark2 record declares one Anchor table for each mark class (including class 0) identified in the MarkRecords of the MarkArray. Each Anchor table specifies one mark2 attachment point used to attach all the mark1 glyphs in a particular class to the mark2 glyph.

Example 9 at the end of the chapter shows a MarkMarkPosFormat1 subtable for attaching one mark to another in the Arabic script.

<a id="mark2Rec"></a>

*Mark2 record*

<a id="mark2AnchorOffsets"></a>

| Type | Name | Description |
| --- | --- | --- |
| Offset16 | mark2AnchorOffsets[markClassCount] | Array of offsets (one per class) to Anchor tables. Offsets are from beginning of Mark2Array table, in class order (offsets may be NULL). |

<a id="CP"></a>

<a id="lookup-type-7-subtable-contextual-positioning"></a>

### Lookup type 7 subtable: contextual positioning

A contextual positioning subtable describes glyph positioning in context so a text-processing client can adjust the position of one or more glyphs within a certain pattern of glyphs.

Contextual positioning subtables can use any of three formats that are common to the GSUB and GPOS tables. These define input sequence patterns to be matched against the text glyph sequence, and then actions to be applied to glyphs within the input sequence. The actions are specified as “nested” lookups, and each is applied to a particular sequence position within the input sequence.

Each sequence position + nested lookup combination is specified in a [SequenceLookupRecord](../../chapter2/chunks/006-class-definition-format-2.md#sequence-lookup-record). Examples [10](011-example-9-markmarkposformat1-subtable.md#example10), [11](011-example-9-markmarkposformat1-subtable.md#example11), [12](012-example-12-contextual-positioning-format-3.md#example12), and [13](012-example-12-contextual-positioning-format-3.md#example13) at the end of this chapter illustrate the use of sequence lookup records within the GPOS table.

While the subtable formats are common between the GSUB and GPOS tables, the lookups referenced by sequence lookup records within the GPOS table are referenced by index into the GPOS LookupList table. In this way, actions specified by a GPOS contextual lookup can only be positioning adjustments.

<a id="CPF1"></a>

**Contextual positioning format 1: simple glyph contexts**

Format 1 defines the context for a glyph positioning operation as a particular sequence of glyphs. For example, a context could be <To>, <xyzabc>, <!?\*#@>, or any other glyph sequence.

For example, suppose that accent mark glyphs above lowercase x-height vowel glyphs need to be lowered when an overhanging capital letter glyph precedes the vowel. When the client locates this context in the text, the subtable identifies the position of the accent mark and a lookup index. A lookup specifies a positioning action that lowers the accent mark over the vowel so that it does not collide with the overhanging capital.

Format 1 contextual positioning is implemented using a SequenceContextFormat1 table. See [Sequence Context Format 1: simple glyph contexts](../../chapter2/chunks/006-class-definition-format-2.md#seqctxt1) in the OpenType Layout Common Table Formats chapter for complete details.

[Example 10](011-example-9-markmarkposformat1-subtable.md#example10) at the end of this chapter demonstrates glyph kerning in context with a SequenceContextFormat1 subtable.

<a id="CPF2"></a>

**Contextual positioning format 2: class-based glyph contexts**

Format 2 defines contexts for glyph positioning as input sequence patterns, with patterns expressed in terms of glyph classes. The glyph classes are defined using a [Class Definition table](../../chapter2/chunks/005-coverage-table.md#class-definition-table). Several sequence patterns may be specified, with each pattern specifying a class of glyphs for each input sequence position.

To clarify the notion of class-based context rules, suppose that certain sequences of three glyphs need special kerning. The glyph sequences consist of an uppercase glyph that overhangs on the right side, a punctuation mark glyph, and then a quote glyph. In this case, the set of uppercase glyphs would constitute one glyph class (class 1), the set of punctuation mark glyphs would constitute a second glyph class (class 2), and the set of quote mark glyphs would constitute a third glyph class (class 3). The input context might be specified as a pattern of one glyph from class 1, followed by one glyph from class 2, followed by one glyph from class 3.

Format 2 contextual positioning is implemented using a SequenceContextFormat2 table. See [Sequence Context Format 2: class-based glyph contexts](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt2) in the OpenType Layout Common Table Formats chapter for complete details.

[Example 11](011-example-9-markmarkposformat1-subtable.md#example11) at the end of this chapter demonstrates a SequenceContextFormat2 subtable that uses glyph classes to modify accent positions in glyph strings.

<a id="CPF3"></a>

**Contextual positioning format 3: coverage-based glyph contexts**

Format 3 defines a context for glyph positioning as an input sequence pattern expressed in terms of Coverage tables. A different Coverage table is defined for each sequence position.

Format 3 is like format 2 in that patterns are defined using sets of glyphs. However, with the glyph classes used in format 2, each glyph is in exactly one class. With format 3, any glyph can occur in multiple Coverage tables.

Unlike Formats 1 and 2, however, this format can define only one context.

For example, consider an input context that contains an uppercase glyph (position 0), followed by any narrow uppercase glyph (position 1), and then another uppercase glyph (position 2). This context requires three Coverage tables, one for each position:

- For position 0, the first position, the Coverage table lists the set of all uppercase glyphs.
- For position 1, the second position, the Coverage table lists the set of all narrow uppercase glyphs, which is a subset of the glyphs listed in the Coverage table for position 0.
- For position 2, the Coverage table lists the set of all uppercase glyphs again.

> *Note:* Both position 0 and position 2 can use the same Coverage table.

Format 3 contextual positioning is implemented using a SequenceContextFormat3 table. See [Sequence Context Format 3: coverage-based glyph contexts](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt3) in the OpenType Layout Common Table Formats chapter for complete details.

[Example 12](012-example-12-contextual-positioning-format-3.md#example12) at the end of this chapter illustrates the use of a SequenceContextFormat3 subtable to change the positions of sign glyphs in math equations.

