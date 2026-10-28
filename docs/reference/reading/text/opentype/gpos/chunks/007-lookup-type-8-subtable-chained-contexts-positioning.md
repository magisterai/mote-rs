# Lookup type 8 subtable: chained contexts positioning

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-lookup-type-6-subtable-mark-to-mark-attachment-positioning.md) · [Next](008-mark-array-table.md)

<a id="CCP"></a>

<a id="lookup-type-8-subtable-chained-contexts-positioning"></a>

### Lookup type 8 subtable: chained contexts positioning

A *chained contexts* positioning subtable describes glyph positioning in context with an ability to look back and/or look ahead in the sequence of glyphs. The design of the chained contexts positioning subtable is parallel to that of the contextual positioning subtable, including the availability of three formats. Each format can describe one or more chained backtrack, input, and lookahead sequence combinations, and one or more positioning adjustments for glyphs in each input sequence.

Positioning adjustments may be specified only for the input sequence context, not for backtrack and lookahead sequences.

Note that backtrack sequences are specified in reverse logical order. See the [Chained Sequence Context Format 1](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt1) section in the OpenType Layout Common Table Formats chapter for details regarding chained backtrack, input, and lookahead sequences.

<a id="CCPF1"></a>

**Chained contexts positioning format 1: simple glyph contexts**

Format 1 defines the context for a glyph substitution as a particular sequence of glyphs. For example, a context could be <xyz>, <holiday>, <!?\*#@>, or any other glyph sequence. Specific glyph sequences are used for input, backtrack or lookahead contexts.

Format 1 chained context substitutions are implemented using a ChainedSequenceContextFormat1 table. See [Chained Sequence Context Format 1: simple glyph contexts](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt1) in the OpenType Layout Common Table Formats chapter for complete details.

<a id="CCPF2"></a>

**Chained contexts positioning format 2: class-based glyph contexts**

Format 2 defines contexts for glyph positioning as patterns expressed in terms of glyph classes. The glyph classes are defined using a [Class Definition table](../../chapter2/chunks/005-coverage-table.md#class-definition-table). Several sequence patterns may be specified, with each pattern specifying a class of glyphs for each sequence position.

To chain contexts, three separate Class Definition tables are used for the backtrack sequence, input sequence, and lookahead sequence.

Format 2 contextual substitutions are implemented using a ChainedSequenceContextFormat2 table. See [Chained Sequence Context Format 2: class-based glyph contexts](../../chapter2/chunks/008-chained-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt2) in the OpenType Layout Common Table Formats chapter for complete details.

<a id="CCPF3"></a>

**Chained contexts positioning format 3: coverage-based glyph contexts**

Format 3 defines contexts for glyph positioning as patterns expressed in terms of Coverage tables. A different Coverage table is defined for each position in a sequence. To chain contexts, three separate sets of Coverage tables are used for the backtrack sequence, input sequence, and lookahead sequence.

Format 3 is like format 2 in that patterns are defined using sets of glyphs. However, with the glyph classes used in format 2, each glyph is in exactly one class. With format 3, any glyph can occur in multiple Coverage tables.

Format 3 contextual substitutions are implemented using a ChainedSequenceContextFormat3 table. See [Chained Sequence Context Format 3: coverage-based glyph contexts](../../chapter2/chunks/008-chained-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt3) in the OpenType Layout Common Table Formats chapter for complete details.

<a id="EP"></a>

<a id="lookup-type-9-subtable-positioning-subtable-extension"></a>

### Lookup type 9 subtable: positioning subtable extension

This lookup type provides a way to access lookup subtables within the GPOS table using 32-bit offsets. This is needed if the total size of the subtables exceeds the 16-bit limits of the various other offsets in the GPOS table. In this specification, the subtable stored at the 32-bit offset location is termed the “extension” subtable.

This subtable type uses one format: PosExtensionFormat1.

<a id="EPF1"></a>

**Positioning extension format 1**

<a id="extension_1"></a>

*PosExtensionFormat1 subtable*

<a id="posFormat_ep1"></a>

<a id="extensionLookupType_ep1"></a>

<a id="extensionOffset_ep1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| uint16 | extensionLookupType | Lookup type of subtable referenced by extensionOffset (i.e. the extension subtable). |
| Offset32 | extensionOffset | Offset to the extension subtable, of lookup type extensionLookupType, relative to the start of the PosExtensionFormat1 subtable. |

The extensionLookupType field must be set to any lookup type other than 9. If a lookup table uses extension subtables, then all of the extension subtables must have the same extensionLookupType. All offsets to extension subtables are set in the usual way—that is, relative to the start of the PosExtensionFormat1 subtable.

When a layout engine encounters a GPOS type 9 Lookup table, it shall:

- Proceed as though the Lookup table’s lookupType field were set to the extensionLookupType of the subtables.
- Proceed as though each extension subtable referenced by extensionOffset replaced the type 9 subtable that referenced it.

<a id="sharedTables"></a>

<a id="shared-formats-value-record-anchor-table-mark-array-table"></a>

### Shared formats: value record, anchor table, mark array table

Several lookup subtables described earlier in this chapter refer to one or more of the same formats for positioning data: ValueRecord, Anchor table, and MarkArray table. These shared formats are described here.

<a id="valueRecord"></a>

<a id="value-record"></a>

#### Value record

GPOS subtables use the ValueRecord format to describe all the variables and values used to adjust the position of a glyph or set of glyphs. A ValueRecord may define any combination of X and Y values (in design units) to add to (positive values) or subtract from (negative values) the placement and advance values provided in the font. In non-variable fonts, a ValueRecord may also contain an offset to a Device table for each of the specified values. In a variable font, it may also contain an offset to a VariationIndex table for each of the specified values.

Note that all fields of a ValueRecord are optional: to save space, only the fields that are required need be included in a given instance. Because the GPOS table uses ValueRecords for many purposes, the sizes and contents of ValueRecords may vary from subtable to subtable. A ValueRecord is always accompanied by a ValueFormat flags field that specifies which of the ValueRecord fields is present. If a ValueRecord specifies more than one value, the values must be listed in the order shown in the ValueRecord definition. If the associated ValueFormat flags indicate that a field is not present, then the next present field follows immediately after the last preceding, present field. The text-processing client must be aware of the flexible and variable nature of ValueRecords in the GPOS table.

Example 14 at the end of the chapter uses ValueFormat and a ValueRecord to specify positioning values in GPOS.

<a id="valueRec"></a>

*ValueRecord*

<a id="xPlacement_vr"></a>

<a id="yPlacement_vr"></a>

<a id="xAdvance_vr"></a>

<a id="yAdvance_vr"></a>

<a id="xPlaDevice_vr"></a>

<a id="yPlaDevice_vr"></a>

<a id="xAdvDevice_vr"></a>

<a id="yAdvDevice_vr"></a>

| Type | Name | Description |
| --- | --- | --- |
| int16 | xPlacement | Horizontal adjustment for placement, in design units. |
| int16 | yPlacement | Vertical adjustment for placement, in design units. |
| int16 | xAdvance | Horizontal adjustment for advance, in design units — only used for horizontal layout. |
| int16 | yAdvance | Vertical adjustment for advance, in design units — only used for vertical layout. |
| Offset16 | xPlaDeviceOffset | Offset to Device table (non-variable font) / VariationIndex table (variable font) for horizontal placement, from beginning of the immediate parent table (SinglePos or PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable) — may be NULL. |
| Offset16 | yPlaDeviceOffset | Offset to Device table (non-variable font) / VariationIndex table (variable font) for vertical placement, from beginning of the immediate parent table (SinglePos or PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable) — may be NULL. |
| Offset16 | xAdvDeviceOffset | Offset to Device table (non-variable font) / VariationIndex table (variable font) for horizontal advance, from beginning of the immediate parent table (SinglePos or PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable) — may be NULL. |
| Offset16 | yAdvDeviceOffset | Offset to Device table (non-variable font) / VariationIndex table (variable font) for vertical advance, from beginning of the immediate parent table (SinglePos or PairPosFormat2 lookup subtable, PairSet table within a PairPosFormat1 lookup subtable) — may be NULL. |

> *Note:* Device tables are used only in non-variable fonts, while VariationIndex tables are used only in variable fonts.

In variable fonts, VariationIndex tables are needed to reference variation data for any placement or advance value that requires adjustment for different variation instances.

> *Note:* While a separate reference to a VariationIndex table is required for each value that requires variation, two or more values that require the same variation data can have offsets that point to the same VariationIndex table, and two or more VariationIndex tables can reference the same variation data entries.

> *Note:* If no VariationIndex table is used for a particular placement or advance value, then that value is used for all variation instances.

A ValueFormat flags field defines the types of positioning adjustment data that ValueRecords specify. SinglePos subtables will have ValueRecords for a single glyph position in a glyph sequence; PairPos subtables will have separate ValueRecords for two glyph positions. In a given subtable, the same ValueFormat applies to every ValueRecord for a given glyph position.

The ValueFormat determines whether the ValueRecords:

- Apply to placement, advance, or both.
- Apply to the horizontal position (X coordinate), the vertical position (Y coordinate), or both.
- Could refer to one or more Device tables (in non-variable fonts) or VariationIndex tables (in variable fonts) for any of the specified values.

Each defined bit in the ValueFormat flags corresponds to a field in the ValueRecord and increases the size of the ValueRecord by 2 bytes. A ValueFormat of 0x0000 corresponds to an empty ValueRecord, which indicates no positioning changes.

To identify the fields in each ValueRecord, the ValueFormat flags shown below are used. To specify multiple fields with a ValueFormat, the bit settings of the relevant fields are added with a logical OR operation.

For example, to adjust the left-side bearing of a glyph, the ValueFormat will be 0x0001, and the ValueRecord will define the xPlacement value. To adjust the advance width of a different glyph, the ValueFormat will be 0x0004, and the ValueRecord will describe the xAdvance value. To adjust both the xPlacement and xAdvance of a set of glyphs, the ValueFormat will be 0x0005, and the ValueRecord will specify both values in the order they are listed in the ValueRecord definition.

<a id="valueFormatFlags"></a>

*ValueFormat flags*

<a id="valueFormat_0001"></a>

<a id="valueFormat_0002"></a>

<a id="valueFormat_0004"></a>

<a id="valueFormat_0008"></a>

<a id="valueFormat_0010"></a>

<a id="valueFormat_0020"></a>

<a id="valueFormat_0040"></a>

<a id="valueFormat_0080"></a>

<a id="valueFormat_FF00"></a>

| Mask | Name | Description |
| --- | --- | --- |
| 0x0001 | X\_PLACEMENT | Includes horizontal adjustment for placement. |
| 0x0002 | Y\_PLACEMENT | Includes vertical adjustment for placement. |
| 0x0004 | X\_ADVANCE | Includes horizontal adjustment for advance. |
| 0x0008 | Y\_ADVANCE | Includes vertical adjustment for advance. |
| 0x0010 | X\_PLACEMENT\_DEVICE | Includes Device table (non-variable font) / VariationIndex table (variable font) for horizontal placement. |
| 0x0020 | Y\_PLACEMENT\_DEVICE | Includes Device table (non-variable font) / VariationIndex table (variable font) for vertical placement. |
| 0x0040 | X\_ADVANCE\_DEVICE | Includes Device table (non-variable font) / VariationIndex table (variable font) for horizontal advance. |
| 0x0080 | Y\_ADVANCE\_DEVICE | Includes Device table (non-variable font) / VariationIndex table (variable font) for vertical advance. |
| 0xFF00 | Reserved | For future use (set to zero). |

<a id="anchorTables"></a>

<a id="anchor-table"></a>

#### Anchor table

A GPOS table uses anchor points to position one glyph with respect to another: two glyphs each define an anchor point, and the text-processing client attaches the glyphs by aligning their corresponding anchor points.

To describe an anchor point, an Anchor table can use one of three formats. The first format uses X and Y coordinates, in design units, to specify a location for the anchor point in relation to the location of the outline for a given glyph. The other two formats refine the location of the anchor point using contour points (Format 2) or Device tables (Format 3). In a variable font, the third format uses a VariationIndex table (a variant of a Device table) to reference variation data for adjustment of the anchor position for the current variation instance, as needed.

<a id="anchorTable_1"></a>

**Anchor format 1: design units**

