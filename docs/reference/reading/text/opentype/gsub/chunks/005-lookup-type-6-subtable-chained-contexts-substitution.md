# Lookup type 6 subtable: chained contexts substitution

Source: [Glyph substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-lookup-type-3-subtable-alternate-substitution.md) · [Next](006-gsub-structure-examples.md)

[Example 9](008-example-8-contextual-substitution-format-2.md#example-9-contextual-substitution-format-3) at the end of this chapter uses SequenceContextFormat3 to substitute swash glyphs for two out of three glyphs in a sequence.

<a id="CC"></a>

<a id="lookup-type-6-subtable-chained-contexts-substitution"></a>

### Lookup type 6 subtable: chained contexts substitution

A chained contexts substitution subtable describes glyph substitutions in context with an ability to look back and/or look ahead in the sequence of glyphs. The design of the chained contexts substitution subtable is parallel to that of the contextual substitution subtable, including the availability of three formats. Each format can describe one or more chained backtrack, input, and lookahead sequence combinations, and one or more substitutions for glyphs in each input sequence.

> *Note:* Substitutions can be specified only for the input sequence context, not for backtrack and lookahead sequences.

See the introduction to the [Contextual substitution](004-lookup-type-3-subtable-alternate-substitution.md#CS) section for general remarks regarding contextual substitutions, which also apply to chained contexts substitutions.

Note that backtrack sequences are specified in reverse logical order. See the [Chained sequence context format 1](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt1) section in the OpenType Layout Common Table Formats chapter for details regarding chained backtrack, input, and lookahead sequences.

<a id="CCF1"></a>

**Chained contexts substitution format 1: simple glyph contexts**

Format 1 defines the context for a glyph substitution as a particular sequence of glyphs. For example, a context could be <xyz>, <holiday>, <!?\*#@>, or any other glyph sequence. Specific glyph sequences are used for input, backtrack or lookahead contexts.

Format 1 chained context substitutions are implemented using a ChainedSequenceContextFormat1 table. See [Chained sequence context format 1: simple glyph contexts](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt1) in the OpenType Layout Common Table Formats chapter for complete details.

<a id="CCF2"></a>

**Chained contexts substitution format 2: class-based glyph contexts**

Format 2 defines contexts for glyph substitutions as patterns expressed in terms of glyph classes. The glyph classes are defined using a [Class Definition table](../../chapter2/chunks/005-coverage-table.md#class-definition-table). Several sequence patterns may be specified, with each pattern specifying a class of glyphs for each sequence position.

To chain contexts, three separate Class Definition tables are used for the backtrack sequence, input sequence, and lookahead sequence.

Format 2 contextual substitutions are implemented using a ChainedSequenceContextFormat2 table. See [Chained sequence context format 2: class-based glyph contexts](../../chapter2/chunks/008-chained-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt2) in the OpenType Layout Common Table Formats chapter for complete details.

<a id="CCF3"></a>

**Chained contexts substitution format 3: coverage-based glyph contexts**

Format 3 defines contexts for glyph substitutions as patterns expressed in terms of Coverage tables. A different Coverage table is defined for each position in a sequence. To chain contexts, three separate sets of Coverage tables are used for the backtrack sequence, input sequence, and lookahead sequence.

Format 3 is like format 2 in that patterns are defined using sets of glyphs. However, with the glyph classes used in format 2, each glyph is in exactly one class. With format 3, any glyph can occur in multiple Coverage tables.

Format 3 contextual substitutions are implemented using a ChainedSequenceContextFormat3 table. See [Chained sequence context format 3: coverage-based glyph contexts](../../chapter2/chunks/008-chained-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt3) in the OpenType Layout Common Table Formats chapter for complete details.

<a id="ES"></a>

<a id="lookup-type-7-subtable-substitution-subtable-extension"></a>

### Lookup type 7 subtable: substitution subtable extension

This lookup type provides a way to access lookup subtables within the GSUB table using 32-bit offsets. This is needed if the total size of the subtables exceeds the 16-bit limits of the various other offsets in the GSUB table. In this specification, the subtable stored at the 32-bit offset location is termed the “extension” subtable.

This subtable type uses one format.

<a id="ESF1"></a>

**Substitution extension format 1**

<a id="extensionSubst_1"></a>

*SubstExtensionFormat1 subtable*

<a id="substFormat_es1"></a>

<a id="extensionLookupType_es1"></a>

<a id="extensionOffset_es1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier. Set to 1. |
| uint16 | extensionLookupType | Lookup type of subtable referenced by extensionOffset (that is, the extension subtable). |
| Offset32 | extensionOffset | Offset to the extension subtable, of lookup type extensionLookupType, relative to the start of the ExtensionSubstFormat1 subtable. |

The extensionLookupType field must be set to any lookup type other than 7. If a lookup table uses extension subtables, then all of the extension subtables must have the same extensionLookupType. All offsets to extension subtables are set in the usual way—that is, relative to start of the ExtensionSubstFormat1 subtable.

When a layout engine encounters a GSUB type 7 Lookup table, it shall:

- Proceed as though the Lookup table’s lookupType field were set to the extensionLookupType of the subtables.
- Proceed as though each extension subtable referenced by extensionOffset replaced the type 7 subtable that referenced it.

<a id="RCCS"></a>

<a id="lookup-type-8-subtable-reverse-chained-contexts-single-substitution"></a>

### Lookup type 8 subtable: reverse chained contexts single substitution

The reverse chaining contextual single substitution subtable (ReverseChainSingleSubst) describes single-glyph substitutions in context with an ability to look back and/or look ahead in the sequence of glyphs. The major difference between this and other lookup types is that processing of input glyph sequence goes from end to start.

Compared to chained contexts substitution (lookup subtable type 6), this format is restricted to only a coverage-based subtable format, input sequences can contain only a single glyph, and only single substitutions are allowed on this glyph. This constraint is integrated into the subtable format.

This lookup type is designed specifically for Arabic script writing styles like Nastaliq in which the shape of the glyph is determined by the following glyph, beginning at the last glyph of the “joor”, or set of connected glyphs.

This subtable type uses one format.

<a id="RCCSF1"></a>

**Reverse chained contexts single substitution format 1: coverage-based glyph contexts**

Format 1 defines a chaining context rule as a sequence of Coverage tables. Each position in the sequence may define a different Coverage table for the set of glyphs that matches the context pattern. With format 1, the glyph sets defined in the different Coverage tables may intersect.

Despite reverse order processing, the order of the Coverage tables listed in the Coverage array must be in logical order (follow the writing direction). The backtrack sequence is as illustrated for the [Chained sequence context format 1](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt1) table, in the OpenType Layout Common Table Formats chapter. The input sequence is one glyph located at *i* in the logical string. The backtrack begins at *i* - 1 and increases in offset value as one moves toward the logical beginning of the string. The lookahead sequence begins at *i* + 1 and increases in offset value as one moves toward the logical end of the string. In processing a reverse chaining substitution, *i* begins at the logical end of the string and moves to the beginning.

The subtable contains a Coverage table for the input glyph and Coverage table arrays for backtrack and lookahead sequences. It also contains an array of substitute glyph indices (substituteGlyphIDs), which are substitutions for glyphs in the Coverage table, and a count of glyphs in the substituteGlyphIDs array. The substituteGlyphIDs array must contain the same number of glyph indices as the Coverage table. To locate the corresponding output glyph index in the substituteGlyphIDs array, this format uses the Coverage index returned from the Coverage table.

[Example 10](009-example-10-reversechainsinglesubstformat1-subtable.md#EX10) at the end of this chapter uses ReverseChainSingleSubstFormat1 to substitute Arabic glyphs with a correct stroke thickness on the left (exit) to match the stroke thickness on the right (entry) of the following glyph (in logical order).

<a id="revChainSingleSubst_1"></a>

*ReverseChainSingleSubstFormat1 subtable*

<a id="substFormat_rcss"></a>

<a id="coverageOffset_rcss"></a>

<a id="backtrackGlyphCount_rcss"></a>

<a id="backtrackCoverageOffsets_rcss"></a>

<a id="lookaheadGlyphCount_rcss"></a>

<a id="lookaheadCoverageOffsets_rcss"></a>

<a id="glyphCount_rcss"></a>

<a id="substituteGlyphIDs_rcss"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier: format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of substitution subtable. |
| uint16 | backtrackGlyphCount | Number of glyphs in the backtrack sequence. |
| Offset16 | backtrackCoverageOffsets[backtrackGlyphCount] | Array of offsets to coverage tables in backtrack sequence, in glyph sequence order. |
| uint16 | lookaheadGlyphCount | Number of glyphs in lookahead sequence. |
| Offset16 | lookaheadCoverageOffsets[lookaheadGlyphCount] | Array of offsets to coverage tables in lookahead sequence, in glyph sequence order. |
| uint16 | glyphCount | Number of glyph IDs in the substituteGlyphIDs array. |
| uint16 | substituteGlyphIDs[glyphCount] | Array of substitute glyph IDs — ordered by Coverage index. |

