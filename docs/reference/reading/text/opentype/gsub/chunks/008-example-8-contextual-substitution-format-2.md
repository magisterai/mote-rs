# Example 8: Contextual substitution format 2

Source: [Glyph substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-example-6-ligaturesubstformat1-subtable.md) · [Next](009-example-10-reversechainsinglesubstformat1-subtable.md)

<a id="EX8"></a>

<a id="example-8-contextual-substitution-format-2"></a>

### Example 8: Contextual substitution format 2

Example 8 illustrates a [format 2 contextual substitution](004-lookup-type-3-subtable-alternate-substitution.md#CSF2) using a [SequenceContextFormat2](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt2) subtable with glyph classes to replace default mark glyphs with their alternative forms. Glyph alternatives are selected depending upon the height of the base glyph that they combine with; that is, the mark glyph used above a high base glyph differs from the mark glyph above a very high base glyph.

In the example, SetMarksHighSubtable contains a Class Definition table that defines four glyph classes: default mark glyphs (class 1), high base glyphs (class 2), very high base glyphs (class 3), and all remaining glyphs, including medium-height base glyphs. The subtable also contains a Coverage table that lists each base glyph that functions as a first component in a context, ordered by glyph index.

Two ClassSequenceRuleSet tables are defined, one for substituting high marks and one for very high marks. No ClassSequenceRuleSets are specified for class 0 and class 1 glyphs because no contexts begin with glyphs from these classes. The classSeqRuleSetOffsets lists offsets to the ClassSequenceRuleSet tables in class value order, so the offset for ClassSequenceRuleSet for class 2 precedes that for class 3.

Within each ClassSequenceRuleSet, a ClassSequencRule is defined. In SetMarksHighSubClassSet2, corresponding to contexts that begin with a glyph in class 2, the ClassSequenceRule table specifies an input sequence with two glyphs: the first glyph in class 2 (a high glyph), and the second in class 1 (a mark glyph). The SequenceLookupRecord specifies applying SubstituteHighMarkLookup at the second position in the sequence—that is, a high mark glyph will replace the default mark glyph.

In SetMarksVeryHighSubClassSet3, corresponding to contexts that begin with a glyph in class 3, the ClassSequencRule specifies an input sequence with two glyphs: the first in class 3 (a very high glyph), and the second in class 1 (a mark glyph). The SequenceLookupRecord specifies applying SubstituteVeryHighMarkLookup at the second position in the sequence—that is, a very high mark glyph will replace the default mark glyph.

*Example 8*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SequenceContextFormat2**  SetMarksHighSubtable | SequenceContextFormat2 subtable definition |
| 0002 | 2 | format |
| 0010 | SetMarksHighCoverage | offset to Coverage table |
| 001C | SetMarksHighClassDef | offset to Class Def table |
| 0004 | 4 | classSeqRuleSetCount |
| 0000 | NULL | classSeqRuleSetOffsets[0] — NULL: no contexts that begin with Class 0 glyphs are defined |
| 0000 | NULL | classSeqRuleSetOffsets[1] — no contexts that begin with Class 1 glyphs are defined |
| 0032 | SetMarksHighSubClassSet2 | classSeqRuleSetOffsets[2] — offset to ClassSequencRuleSet table for contexts that begin with Class 2 glyphs (high base glyphs) |
| 0040 | SetMarksVeryHighSubClassSet3 | classSeqRuleSetOffsets[3] — offset to ClassSequencRuleSet table for contexts that begin with Class 3 glyphs (very high base glyphs) |
|  | **CoverageFormat1**  SetMarksHighCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 0004 | 4 | glyphCount |
| 0030 | tahGlyphID | glyphArray[0], high base glyph |
| 0031 | dhahGlyphID | glyphArray[1], high base glyph |
| 0040 | cafGlyphID | glyphArray[2], very high base glyph |
| 0041 | gafGlyphID | glyphArray[3], very high base glyph |
|  | **ClassDefFormat2**  SetMarksHighClassDef | Class table definition |
| 0002 | 2 | classFormat: ranges |
| 0003 | 3 | classRangeCount |
|  | classRangeRecords[0] | ClassRangeRecords ordered by startGlyphID; record for Class 2, high base glyphs |
| 0030 | tahGlyphID | Start, first Glyph ID in range |
| 0031 | dhahGlyphID | End, last Glyph ID in range |
| 0002 | 2 | class: 2 |
|  | classRangeRecords[1] | ClassRangeRecord for Class 3, very high base glyphs |
| 0040 | cafGlyphID | Start, first Glyph ID in the range |
| 0041 | gafGlyphID | End, last Glyph ID in the range |
| 0003 | 3 | class: 3  ClassRange[2] for Class 1, mark gyphs |
|  | classRangeRecords[2] | ClassRangeRecord for Class 1, mark glyphs |
| 00D2 | fathatanDefaultGlyphID | Start, first Glyph ID in range default fathatan mark |
| 00D3 | dammatanDefaultGlyphID | End, last Glyph ID in the range default dammatan mark |
| 0001 | 1 | class: 1 |
|  | **ClassSequencRuleSet**  SetMarksHighSubClassSet2 | ClassSequencRuleSet[2] table definition— all contexts that begin with Class 2 glyphs |
| 0001 | 1 | classSeqRuleCount |
| 0004 | SetMarksHighSubClassRule2 | classSeqRuleOffsets[0] (offset to ClassSequenceRule table 0) — ClassSequenceRule tables ordered by preference |
|  | **ClassSequenceRule**  SetMarksHighSubClassRule2 | ClassSequenceRule[0] table definition, Class 2 glyph (high base) glyph followed by a Class 1 glyph (mark) |
| 0002 | 2 | glyphCount |
| 0001 | 1 | seqLookupCount |
| 0001 | 1 | inputSequence[0] — input sequence beginning with the second Class in the input context sequence; Class 1, mark glyphs |
|  | seqLookupRecords[0] | seqLookupRecords array in design order |
| 0001 | 1 | sequenceIndex — apply substitution to position 2, a mark |
| 0001 | 1 | lookupListIndex |
|  | **ClassSequencRuleSet**  SetMarksVeryHighSubClassSet3 | ClassSequencRuleSet[3] table definition — all contexts that begin with Class 3 glyphs |
| 0001 | 1 | classSeqRuleCount |
| 0004 | SetMarksVeryHighSubClassRule3 | classSeqRuleOffsets[0] |
|  | **ClassSequenceRule**  SetMarksVeryHighSubClassRule3 | ClassSequenceRule[0] table definition — Class 3 glyph (very high base glyph) followed by a Class 1 glyph (mark) |
| 0002 | 2 | glyphCount |
| 0001 | 1 | seqLookupCount |
| 0001 | 1 | inputSequence[0] — input sequence beginning with the second Class in the input context sequence; Class 1, mark glyphs |
|  | seqLookupRecords[0] | seqLookupRecords array in design order |
| 0001 | 1 | sequenceIndex — apply substitution to position 2, second glyph class (mark) |
| 0002 | 2 | lookupListIndex |

<a id="EX9"></a>

<a id="example-9-contextual-substitution-format-3"></a>

### Example 9: Contextual substitution format 3

Example 9 illustrates a [format 3 contextual substitution](004-lookup-type-3-subtable-alternate-substitution.md#CSF3), using a [SequenceContextFormat3](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt3) subtable with Coverage tables to describe a context sequence of three lowercase glyphs in the pattern: any ascender or descender glyph in position 0 (zero), any x-height glyph in position 1, and any descender glyph in position 2. The overlapping sets of covered glyphs for positions 0 and 2 make Format 3 better for this context than the class-based Format 2.

In positions 0 and 2, swash versions of the glyphs replace the default glyphs. The contextual-substitution lookup is SwashLookup (LookupList index = 0), and its subtable is SwashSubtable. The SwashSubtable defines three Coverage tables: AscenderDescenderCoverage, XheightCoverage, and DescenderCoverage-one for each glyph position in the context sequence, respectively.

The SwashSubtable also defines two SequenceLookupRecords: one that applies to position 0, and one for position 2. (No substitutions are applied to position 1.) The record for position 0 uses a single substitution lookup called AscDescSwashLookup to replace the current ascender or descender glyph with a swash ascender or descender glyph. The record for position 2 uses a single substitution lookup called DescSwashLookup to replace the current descender glyph with a swash descender glyph.

*Example 9*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SequenceContextFormat3**  SwashSubtable | SequenceContextFormat3 subtable definition |
| 0003 | 3 | format |
| 0003 | 3 | glyphCount — number in input glyph sequence |
| 0002 | 2 | seqLookupCount |
| 0014 | AscenderDescenderCoverage | coverageOffsets[0] — offsets to Coverage tables, in context sequence order |
| 0030 | XheightCoverage | coverageOffsets[1] |
| 0052 | DescenderCoverage | coverageOffsets[2] |
|  | seqLookupRecords[0] | SequenceLookupRecords in glyph position order |
| 0000 | 0 | sequenceIndex |
| 0001 | 1 | lookupListIndex — single substitution to output ascender or descender swash |
|  | seqLookupRecords[1] |  |
| 0002 | 2 | sequenceIndex |
| 0002 | 2 | lookupListIndex — single substitution to output descender swash |
|  | **CoverageFormat1**  AscenderDescenderCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 000C | 12 | glyphCount |
| 0033 | bGlyphID | glyphArray[0] — glyphs in glyph ID order |
| 0035 | dGlyphID | glyphArray[1] |
| 0037 | fGlyphID | glyphArray[2] |
| 0038 | gGlyphID | glyphArray[3] |
| 0039 | hGlyphID | glyphArray[4] |
| 003B | jGlyphID | glyphArray[5] |
| 003C | kGlyphID | glyphArray[6] |
| 003D | lGlyphID | glyphArray[7] |
| 0041 | pGlyphID | glyphArray[8] |
| 0042 | qGlyphID | glyphArray[9] |
| 0045 | tGlyphID | glyphArray[10] |
| 004A | yGlyphID | glyphArray[11] |
|  | **CoverageFormat1**  XheightCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 000F | 15 | glyphCount |
| 0032 | aGlyphID | glyphArray[0] |
| 0034 | cGlyphID | glyphArray[1] |
| 0036 | eGlyphID | glyphArray[2] |
| 003A | iGlyphID | glyphArray[3] |
| 003E | mGlyphID | glyphArray[4] |
| 003F | nGlyphID | glyphArray[5] |
| 0040 | oGlyphID | glyphArray[6] |
| 0043 | rGlyphID | glyphArray[7] |
| 0044 | sGlyphID | glyphArray[8] |
| 0045 | tGlyphID | glyphArray[9] |
| 0046 | uGlyphID | glyphArray[10] |
| 0047 | vGlyphID | glyphArray[11] |
| 0048 | wGlyphID | glyphArray[12] |
| 0049 | xGlyphID | glyphArray[13] |
| 004B | zGlyphID | GlyphArray[14] |
|  | **CoverageFormat1**  DescenderCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 0005 | 5 | glyphCount |
| 0038 | gGlyphID | glyphArray[0] |
| 003B | jGlyphID | glyphArray[1] |
| 0041 | pGlyphID | glyphArray[2] |
| 0042 | qGlyphID | glyphArray[3] |
| 004A | yGlyphID | glyphArray[4] |

