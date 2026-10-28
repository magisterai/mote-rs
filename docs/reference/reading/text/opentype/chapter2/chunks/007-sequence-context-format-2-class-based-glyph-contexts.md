# Sequence context format 2: class-based glyph contexts

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-class-definition-format-2.md) · [Next](008-chained-sequence-context-format-2-class-based-glyph-contexts.md)

<a id="seqctxt2"></a>

<a id="sequence-context-format-2-class-based-glyph-contexts"></a>

### Sequence context format 2: class-based glyph contexts

GSUB type 5 format 2 subtables and GPOS type 7 format 2 subtables define input sequences patterns in terms of glyph classes defined using a [Class Definition table](005-coverage-table.md#class-definition-table). Several sequence patterns may be specified, with each pattern specifying a class of glyphs for each input sequence position.

Classes are assigned an integer number, the *class value*. An input sequence pattern is specified as a sequence of class values. For example, a pattern *1, 4, 3* specifies the set of glyph sequences with three glyphs, the first from class 1, the second from class 4, and the third from class 3.

Each pattern is specified in a *ClassSequenceRule* table. Patterns that start with the same class value in the first position are aggregated using a *ClassSequenceRuleSet* table.

The SequenceContextFormat2 table has an offset to a Coverage table. The Coverage table includes all glyph IDs for glyphs that occur as the first glyph in any of the class-based patterns. That is, the Coverage table contains the list of glyph indices for all the glyphs in all classes that are first in any of the specified input sequence patterns. For example, if the patterns begin with either class 1 or class 2, then the Coverage table will list all glyphs in either class 1 or class 2. Glyph IDs in the Coverage table are given once.

> *Note:* Due to the way the class definition table is defined, each glyph ID belongs to exactly one class.

When evaluating a SequenceContextFormat2 subtable for a given position in a glyph sequence, the client searches for the current glyph in the Coverage table. If found, the client then searches in the class definition table to find the class value assigned to the currently glyph. The class value is used as the index into an array of offsets to ClassSequenceRuleSet tables. The ClassSequenceRuleSet table for that class value is retrieved, and the ClassSequenceRule tables for that set are examined to see if the current sequence matches any of the specified patterns.

> *Note:* The formats of the ClassSequenceRuleSet and ClassSequenceRule tables are essentially the same as the formats of the SequenceRuleSet and SequenceRule tables, but the semantics are different: The ClassSequenceRule table has a sequence of glyph class values, while the SequenceRule table has a sequence of glyph IDs; therefore these are distinguished. Accordingly, the ClassSequenceRuleSet and SequenceRuleSet tables are distinguished by the subtables referenced by each.

*SequenceContextFormat2 table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of SequenceContextFormat2 table. |
| Offset16 | classDefOffset | Offset to ClassDef table, from beginning of SequenceContextFormat2 table. |
| uint16 | classSeqRuleSetCount | Number of ClassSequenceRuleSet tables. |
| Offset16 | classSeqRuleSetOffsets[classSeqRuleSetCount] | Array of offsets to ClassSequenceRuleSet tables, from beginning of SequenceContextFormat2 table (may be NULL). |

There is one offset to a ClassSequenceRuleSet subtable for each class defined in the class definition table. The offsets are listed in class value order. If no patterns are defined that begin with a particular class, then the offset for that class value may be set to NULL.

*ClassSequenceRuleSet table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | classSeqRuleCount | Number of ClassSequenceRule tables. |
| Offset16 | classSeqRuleOffsets[classSeqRuleCount] | Array of offsets to ClassSequenceRule tables, from beginning of ClassSequenceRuleSet table. |

Offsets to ClassSequenceRule subtables are ordered according to the desired results. The subtables are evaluated in the order the offsets are listed, and the first class sequence rule that matches the current glyph sequence is used. Rules for longer, more specific sequences are typically ordered before shorter rules.

*ClassSequenceRule table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | glyphCount | Number of glyphs to be matched. |
| uint16 | seqLookupCount | Number of SequenceLookup records. |
| uint16 | inputSequence[glyphCount - 1] | Sequence of classes to be matched to the input glyph sequence, beginning with the second glyph position. |
| SequenceLookup | seqLookupRecords[seqLookupCount] | Array of SequenceLookup records. |

The glyphCount value is the total number of glyph classes in the input sequence pattern, including the first sequence position. The inputSequence array specifies the remaining class values in the input sequence pattern, in order.

The seqLookupRecords array lists the sequence lookup records that specify actions to be taken on glyphs at various positions within the input sequence. These do not have to be ordered in sequence position order; they are ordered according to the desired result. All of the sequence lookup records are processed in order, and each applies to the results of the actions indicated by the preceding record.

<a id="seqctxt3"></a>

<a id="sequence-context-format-3-coverage-based-glyph-contexts"></a>

### Sequence context format 3: coverage-based glyph contexts

GSUB type 5 format 3 subtables and GPOS type 7 format 3 subtables define input sequences patterns in terms of sets of glyphs defined using [Coverage tables](005-coverage-table.md#coverage-table).

The SequenceContextFormat3 table specifies exactly one input sequence pattern. It has an array of offsets to coverage tables. These correspond, in order, to the positions in the input sequence pattern.

*SequenceContextFormat3 table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 3. |
| uint16 | glyphCount | Number of glyphs in the input sequence. |
| uint16 | seqLookupCount | Number of SequenceLookup records. |
| Offset16 | coverageOffsets[glyphCount] | Array of offsets to Coverage tables, from beginning of SequenceContextFormat3 subtable. |
| SequenceLookup | seqLookupRecords[seqLookupCount] | Array of SequenceLookup records. |

The seqLookupRecords array lists the sequence lookup records that specify actions to be taken on glyphs at various positions within the input sequence. These do not have to be ordered in sequence position order; they are ordered according to the desired result. All of the sequence lookup records are processed in order, and each applies to the results of the actions indicated by the preceding record.

<a id="chseqctxt1"></a>

<a id="chained-sequence-context-format-1-simple-glyph-contexts"></a>

### Chained sequence context format 1: simple glyph contexts

GSUB type 6 format 1 and GPOS type 8 format 1 subtables define input sequences as well as the chained backtrack and lookahead sequences in terms of specific glyph IDs; this is the *ChainedSequenceContextFormat1* table. Its subtable formats are similar to those used for the SequenceContextFormat1 table, the key difference being that the *ChainedSequenceRule* table includes the chained backtrack and lookahead sequences.

The first glyphs for the input sequences are specified in a Coverage table. The remaining glyphs in each input sequence as well as the backtrack and lookahead sequences are defined in ChainedSequenceRule tables—one for each combined backtrack + input + lookahead sequence. If multiple sequence combinations have the same initial glyph in the input sequence, that glyph ID must be listed once in the Coverage table, and the corresponding rules are aggregated using a *ChainedSequenceRuleSet* table—one for each initial input sequence glyph specified in the Coverage table.

When evaluating a ChainedSequenceContextFormat1 subtable for a given position in a glyph sequence, the client searches for the current glyph in the Coverage table. If found, the corresponding ChainedSequenceRuleSet table is retrieved, and the ChainedSequenceRule tables for that set are examined to see if the current glyph sequence matches any of the patterns in any of thechained sequence rules. The first matching rule subtable is used.

Matching of the sequence rules with the current glyph sequence requires matching of input, backtrack and lookahead sequences. Lookup flags affect matching in backtrack and lookahead sequences as well as the input sequence.

Note that the backtrack sequence is given in reverse logical order: if the current glyph is at position *i* in the text glyph buffer, the backtrack sequence begins at *i-1* and decrements offset values moving away from *i*. The lookahead sequence begins after the input sequence and increases in logical order.

To clarify the ordering of glyph arrays for input, backtrack and lookahead sequences, the following illustration is provided. Suppose within a logically-ordered glyph sequence the input sequence match begins at index *i* and has a length of 2.

|  |  |  |  |  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Logical order: | a | b | c | d | m | n | w | x | y | z |
| Sequence index: |  |  | … | *i* - 1 | *i* | *i* + 1 | *i* + 2 | … |  |  |
| Input sequence index: |  |  |  |  | 0 | 1 |  |  |  |  |
| Backtrack sequence index: | 3 | 2 | 1 | 0 |  |  |  |  |  |  |
| Lookahead sequence index: |  |  |  |  |  |  | 0 | 1 | 2 | 3 |

Thus, in this example, the input sequence would comprise glyph IDs for “mn”; the backtrack sequence would comprise glyph IDs for “dcba” *in that order*; and the lookahead sequence would comprise glyph IDs for “wxyz”. Actions specified in sequence lookup records can be specified only for glyphs in the input sequence, the glyphs for “mn”.

*ChainedSequenceContextFormat1 table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of ChainSequenceContextFormat1 table. |
| uint16 | chainedSeqRuleSetCount | Number of ChainedSequenceRuleSet tables. |
| Offset16 | chainedSeqRuleSetOffsets[chainedSeqRuleSetCount] | Array of offsets to ChainedSeqRuleSet tables, from beginning of ChainedSequenceContextFormat1 table (may be NULL). |

The Coverage table lists the initial glyphs from all of the supported input glyph sequences. The chainedSeqRuleSetCount should match the number of glyphs in the Coverage table. If these differ, the extra coverage glyphs or extra sequence rule sets are ignored.

There is one ChainedSequenceRuleSet table for each covered glyph. The offsets in the chainedSeqRuleSetOffsets array must be ordered to match the order of glyphs in the Coverage table.

*ChainedSequenceRuleSet table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | chainedSeqRuleCount | Number of ChainedSequenceRule tables. |
| Offset16 | chainedSeqRuleOffsets[chainedSeqRuleCount] | Array of offsets to ChainedSequenceRule tables, from beginning of ChainedSequenceRuleSet table. |

Offsets to ChainedSequenceRule subtables are ordered according to the desired results. The subtables are evaluated in the order the offsets are listed, and the first chained sequence rule that matches the current glyph sequence is used. Rules for longer, more specific sequences are typically ordered before shorter rules.

*ChainedSequenceRule table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | backtrackGlyphCount | Number of glyphs in the backtrack sequence. |
| uint16 | backtrackSequence[backtrackGlyphCount] | Array of backtrack glyph IDs. |
| uint16 | inputGlyphCount | Number of glyphs in the input sequence. |
| uint16 | inputSequence[inputGlyphCount - 1] | Array of input glyph IDs—start with second glyph. |
| uint16 | lookaheadGlyphCount | Number of glyphs in the lookahead sequence. |
| uint16 | lookaheadSequence[lookaheadGlyphCount] | Array of lookahead glyph IDs. |
| uint16 | seqLookupCount | Number of SequenceLookup records. |
| SequenceLookup | seqLookupRecords[seqLookupCount] | Array of SequenceLookup records. |

The inputGlyphCount value is the total number of glyphs in the input sequence, including the first glyph. The inputSequence array specifies the remaining glyphs in the input sequence, in order.

The seqLookupRecords array lists the sequence lookup records that specify actions to be taken on glyphs at various positions within the input sequence. These do not have to be ordered in sequence position order; they are ordered according to the desired result. All of the sequence lookup records are processed in order, and each applies to the results of the actions indicated by the preceding record.

