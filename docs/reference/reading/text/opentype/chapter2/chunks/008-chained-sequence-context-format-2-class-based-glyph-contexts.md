# Chained sequence context format 2: class-based glyph contexts

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-sequence-context-format-2-class-based-glyph-contexts.md) · [Next](009-device-and-variationindex-tables.md)

<a id="chseqctxt2"></a>

<a id="chained-sequence-context-format-2-class-based-glyph-contexts"></a>

### Chained sequence context format 2: class-based glyph contexts

GSUB type 6 format 2 and GPOS type 8 format 2 subtables define input sequence patterns, as well as chained backtrack and lookahead sequence patterns, in terms of glyph classes defined using [Class Definition tables](005-coverage-table.md#class-definition-table). Three separate class definition tables are used to define the classes for the input, backtrack and lookahead sequence patterns. Several combined sequence patterns may be defined, with glyphs for each position with the sequence patterns specified using a class value for the corresponding class definition table.

Each combined pattern is specified in a *ChainedClassSequenceRule* table. Pattern combinations for which the first position of the input sequence uses the same class value are aggregated using a *ChainedClassSequenceRuleSet* table.

The ChainedSequenceContextFormat2 table has an offset to a Coverage table. The Coverage table includes all glyph IDs for glyphs that may occur as the first glyph in any of the class-based input sequence patterns. Glyph IDs in the Coverage table are given once.

When evaluating a ChainedSequenceContextFormat2 subtable for a given position in a glyph sequence, the client searches for the current glyph in the Coverage table. If found, the client then searches in the class definition table to find the class value assigned to the current glyph. The class value is used as the index into an array of offsets to ChainedClassSequenceRuleSet tables. The ChainedClassSequenceRuleSet table for that class value is retrieved, and the ChainedClassSequenceRule tables for that set are examined to see if the current glyph sequence matches any of the specified patterns.

Matching of the sequence rules with the current glyph sequence requires matching of input, backtrack and lookahead sequences. Note that Lookup flags affect matching in backtrack and lookahead sequences as well as the input sequence. Backtrack sequence patterns are specified in reverse logical order. Specification of backtrack, input and lookahead sequences is the same as described above for [Chained sequence context format 1](007-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt1), except that sequences are specified using class values rather than glyph IDs.

*ChainedSequenceContextFormat2 table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of ChainedSequenceContextFormat2 table. |
| Offset16 | backtrackClassDefOffset | Offset to ClassDef table containing backtrack sequence context, from beginning of ChainedSequenceContextFormat2 table. |
| Offset16 | inputClassDefOffset | Offset to ClassDef table containing input sequence context, from beginning of ChainedSequenceContextFormat2 table. |
| Offset16 | lookaheadClassDefOffset | Offset to ClassDef table containing lookahead sequence context, from beginning of ChainedSequenceContextFormat2 table. |
| uint16 | chainedClassSeqRuleSetCount | Number of ChainedClassSequenceRuleSet tables. |
| Offset16 | chainedClassSeqRuleSetOffsets[chainedClassSeqRuleSetCount] | Array of offsets to ChainedClassSequenceRuleSet tables, from beginning of ChainedSequenceContextFormat2 table (may be NULL). |

There is one offset to a ChainedClassSequenceRuleSet subtable for each class defined in the input sequence class definition table. The offsets are listed in class value order. If no patterns are defined with an input sequence beginning with a particular class, then the offset for that class value may be set to NULL.

*ChainedClassSequenceRuleSet table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | chainedClassSeqRuleCount | Number of ChainedClassSequenceRule tables. |
| Offset16 | chainedClassSeqRuleOffsets[chainedClassSeqRuleCount] | Array of offsets to ChainedClassSequenceRule tables, from beginning of ChainedClassSequenceRuleSet. |

Offsets to ChainedClassSequenceRule subtables are ordered according to the desired results. The subtables are evaluated in the order the offsets are listed, and the first chained class sequence rule that matches the current glyph sequence is used. Rules for longer, more specific sequences are typically ordered before shorter rules.

*ChainedClassSequenceRule table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | backtrackGlyphCount | Number of glyphs in the backtrack sequence. |
| uint16 | backtrackSequence[backtrackGlyphCount] | Array of backtrack-sequence classes. |
| uint16 | inputGlyphCount | Total number of glyphs in the input sequence. |
| uint16 | inputSequence[inputGlyphCount - 1] | Array of input sequence classes, beginning with the second glyph position. |
| uint16 | lookaheadGlyphCount | Number of glyphs in the lookahead sequence. |
| uint16 | lookaheadSequence[lookaheadGlyphCount] | Array of lookahead-sequence classes. |
| uint16 | seqLookupCount | Number of SequenceLookup records. |
| SequenceLookup | seqLookupRecords[seqLookupCount] | Array of SequenceLookup records. |

The inputGlyphCount value is the total number of glyph classes in the input sequence pattern, including the first sequence position. The inputSequence array specifies the remaining class values in the input sequence pattern, in order.

The seqLookupRecords array lists the sequence lookup records that specify actions to be taken on glyphs at various positions within the input sequence. These do not have to be ordered in sequence position order; they are ordered according to the desired result. All of the sequence lookup records are processed in order, and each applies to the results of the actions indicated by the preceding record.

<a id="chseqctxt3"></a>

<a id="chained-sequence-context-format-3-coverage-based-glyph-contexts"></a>

### Chained sequence context format 3: coverage-based glyph contexts

GSUB type 6 format 3 subtables and GPOS type 8 format 3 subtables define input sequences patterns, as well as chained backtrack and lookahead sequence patterns, in terms of sets of glyphs defined using [Coverage tables](005-coverage-table.md#coverage-table).

The ChainedSequenceContextFormat3 table specifies exactly one input sequence pattern. It has three arrays of offsets to coverage tables: one for the input sequence pattern, one for the backtrack sequence pattern, and one for the lookahead sequence pattern. For each array, the offsets correspond, in order, to the positions in the sequence pattern.

*ChainedSequenceContextFormat3 table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 3. |
| uint16 | backtrackGlyphCount | Number of glyphs in the backtrack sequence. |
| Offset16 | backtrackCoverageOffsets[backtrackGlyphCount] | Array of offsets to coverage tables for the backtrack sequence. |
| uint16 | inputGlyphCount | Number of glyphs in the input sequence. |
| Offset16 | inputCoverageOffsets[inputGlyphCount] | Array of offsets to coverage tables for the input sequence. |
| uint16 | lookaheadGlyphCount | Number of glyphs in the lookahead sequence. |
| Offset16 | lookaheadCoverageOffsets[lookaheadGlyphCount] | Array of offsets to coverage tables for the lookahead sequence. |
| uint16 | seqLookupCount | Number of SequenceLookup records. |
| SequenceLookup | seqLookupRecords[seqLookupCount] | Array of SequenceLookup records. |

All offsets are from the start of the ChainedSequenceContextFormat3 table.

The seqLookupRecords array lists the sequence lookup records that specify actions to be taken on glyphs at various positions within the input sequence. These do not have to be ordered in sequence position order; they are ordered according to the desired result. All of the sequence lookup records are processed in order, and each applies to the results of the actions indicated by the preceding record.

