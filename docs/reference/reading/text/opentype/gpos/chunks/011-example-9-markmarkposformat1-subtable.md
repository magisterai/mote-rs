# Example 9: MarkMarkPosFormat1 subtable

Source: [Glyph positioning](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gpos).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](010-example-6-cursiveposformat1-subtable.md) · [Next](012-example-12-contextual-positioning-format-3.md)

<a id="example9"></a>

<a id="example-9-markmarkposformat1-subtable"></a>

### Example 9: MarkMarkPosFormat1 subtable

The MarkMarkPosFormat1 subtable in Example 9 defines two Arabic marks glyphs. The hamza mark, the base mark (Mark2), is identified in the Mark2GlyphsCoverage table. The damma mark, the attaching mark (Mark1), is defined in the Mark1GlyphsCoverage table.

Each Mark1 glyph is also listed in the Mark1Array, along with its attachment point data and a mark Class value. The Mark1GlyphsArray defines one mark class, class 0, that consists of marks located above Mark2 base glyphs. The Mark1GlyphsArray contains an offset to a dammaMarkAnchor table to specify the coordinate of the damma mark’s attachment point.

The Mark2GlyphsArray table defines a count and an array of Mark2 records, one for each covered Mark2 base glyph. Each record contains an offset to a Mark2Anchor table for each Mark1 class. One Anchor table, AboveMark2Anchor, specifies a coordinate value for attaching the damma mark above the hamza base mark.

*Example 9*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **MarkMarkPosFormat1**  MarkMarkAttachSubTable | MarkBasePos subtable definition |
| 0001 | 1 | format |
| 000C | Mark1GlyphsCoverage | offset to mark1Coverage table |
| 0012 | Mark2GlyphsCoverage | offset to mark2Coverage table |
| 0001 | 1 | markClassCount |
| 0018 | Mark1GlyphsArray | offset to mark1Array table |
| 0024 | Mark2GlyphsArray | offset to mark2Array table |
|  | **CoverageFormat1**  Mark1GlyphsCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0001 | 1 | glyphCount |
| 0296 | dammaMarkGlyphID | glyphArray[0] |
|  | **CoverageFormat1**  Mark2GlyphsCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0001 | 1 | glyphCount |
| 0289 | hamzaMarkGlyphID | glyphArray[1] |
|  | **MarkArray**  Mark1GlyphsArray | MarkArray table definition |
| 0001 | 1 | markCount |
|  | markRecords[0] | MarkRecords in Coverage index order |
| 0000 | 0 | markClass — for marks above base mark |
| 0006 | dammaMarkAnchor | markAnchorOffset |
|  | **AnchorFormat1**  dammaMarkAnchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 00BD | 189 | xCoordinate |
| FF99 | -103 | yCoordinate |
|  | **Mark2Array**  Mark2GlyphsArray | Mark2Array table definition |
| 0001 | 1 | mark2Count |
|  | mark2Records[0] |  |
| 0004 | AboveMark2Anchor | mark2AnchorOffsets[0] |
|  | **AnchorFormat1**  AboveMark2Anchor | Anchor table definition |
| 0001 | 1 | format: design units only |
| 00DD | 221 | xCoordinate |
| 012D | 301 | yCoordinate |

<a id="example10"></a>

<a id="example-10-contextual-positioning-format-1"></a>

### Example 10: Contextual positioning format 1

Example 10 illustrates [contextual positioning format 1](006-lookup-type-6-subtable-mark-to-mark-attachment-positioning.md#CPF1) using a [SequenceContextFormat1](../../chapter2/chunks/006-class-definition-format-2.md#seqctxt1) subtable to adjust the spacing between three Arabic glyphs in a word. The context is the glyph sequence (from right to left): heh (initial form), thal (final form), and heh (isolated form). In the rendered word, the first two glyphs are connected, but the last glyph (the isolated form of heh), is separate. This subtable reduces the amount of space between the last glyph and the rest of the word.

The subtable contains a WordCoverage table that lists the first glyph in the word, heh (initial), and one SequenceRuleSet table, called WordPosRuleSet, that defines all contexts beginning with this covered glyph.

The WordPosRuleSet contains one SequenceRule table that describes a word context of three glyphs and identifies the second and third glyphs (the first glyph is identified by the WordPosRuleSet). When a text-processing client locates this context in text, it applies a Single Adjustment Positioning lookup (not shown in the example) at position 2 to reduce the spacing between the glyphs.

*Example 10*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SequenceContextFormat1**  MoveHehInSubtable | SequenceContextFormat1 subtable definition |
| 0001 | 1 | format |
| 0008 | WordCoverage | offset to Coverage table |
| 0001 | 1 | seqRuleSetCount |
| 000E | WordPosRuleSet | seqRuleSetOffsets[0] |
|  | **CoverageFormat1**  WordCoverage | Coverage table offset |
| 0001 | 1 | format: lists |
| 0001 | 1 | glyphCount |
| 02A6 | hehInitialGlyphID | glyphArray[0] |
|  | **SequenceRuleSet**  WordPosRuleSet | SequenceRuleSet table definition |
| 0001 | 1 | seqRuleCount |
| 0004 | WordPosRule | seqRuleOffsets[0] |
|  | **SequenceRule**  WordPosRule | SequenceRule table definition |
| 0003 | 3 | glyphCount |
| 0001 | 1 | seqLookupCount |
| 02DD | thalFinalGlyphID | inputSequence[0] |
| 02C6 | hehIsolatedGlyphID | inputSequence[1] |
|  |  | seqLookupRecords[0] |
| 0002 | 2 | sequenceIndex |
| 0001 | 1 | lookupListIndex |

<a id="example11"></a>

<a id="example-11-contextual-positioning-format-2"></a>

### Example 11: Contextual positioning format 2

Example 11 illustrates [contextual positioning format 2](006-lookup-type-6-subtable-mark-to-mark-attachment-positioning.md#CPF2) using a [SequenceContextFormat2](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt2) subtable with glyph classes to modify accent positions in contexts that have preceding overhanging, uppercase glyphs.

In the example, MoveAccentsSubtable contains a Class Definition table that defines five glyph classes: class 1 consists of uppercase glyphs that overhang and create a wide open space on their right side; class 2 consists of uppercase glyphs that overhang and create a narrow space on their right side; class 3 contains lowercase x-height vowels; and class 4 contains accent glyphs placed over the lowercase vowels. The rest of the glyphs in the font fall into class 0.

The MoveAccentsSubtable defines two similar context strings. The first consists of a class 1 uppercase glyph followed by a class 3 lowercase vowel glyph with a class 4 accent glyph over the vowel. When this context is found in the text, the client lowers the accent glyph over the vowel so that it does not collide with the overhanging glyph shape. The second context consists of a class 2 uppercase glyph, followed by a class 3 lowercase vowel glyph with a class 4 accent glyph over the vowel. When this context is found in the text, the client increases the advance width of the uppercase glyph to expand the space between it and the accented vowel.

MoveAccentsSubtable defines a table, MoveAccentsCoverage, that identifies the first glyphs in the two contexts. It also has offsets to five ClassSequenceRuleSet tables, one for each class defined in the ClassDef table. Since no contexts begin with class 0, class 3, or class 4 glyphs, the offsets to the ClassSequenceRuleSet tables for these classes are NULL. ClassSequenceRuleSet[1] defines all contexts beginning with class 1 glyphs; it is called UCWideOverhangPosClass1Set. ClassSequenceRuleSet[2] defines all contexts beginning with class 2 glyphs, and it is called UCNarrowOverhangPosClass1Set.

Each ClassSequenceRuleSet defines one ClassSequenceRule. The UCWideOverhangPosClass1Set uses the UCWideOverhangPosClassRule to specify the first context. The first class in this context string is identified by the index into the array of offsets to the ClassSequenceRuleSet tables; in this case, it is class 1. The ClassSequenceRule table lists the second and third classes in the context as class 3 and class 4. A Single Adjustment Positioning Lookup (not shown) lowers the accent glyph in position 3 in the context string.

The UCNarrowOverhangPosClass1Set defines the UCNarrowOverhangPosClassRule for the second context. This ClassSequenceRule is identical to the UCWideOverhangPosClassRule, except that the first class in the context string is a class 2 lowercase glyph. A Single Adjustment Positioning Lookup (not shown) increases the advance width of the overhanging uppercase glyph in position 0 in the context string.

*Example 11*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SequenceContextFormat2**  MoveAccentsSubtable | SequenceContextFormat2 subtable definition |
| 0002 | 2 | format |
| 0012 | MoveAccentsCoverage | Offset to Coverage table |
| 0020 | MoveAccentsClassDef | Offset to ClassDef |
| 0005 | 5 | classSeqRuleSetCount |
| 0000 | NULL | classSeqRuleSetOffsets[0] — no contexts begin with class 0 glyphs |
| 0060 | UCWideOverhangPosClass1Set | classSeqRuleSetOffsets[1] — contexts beginning with class 1 glyphs |
| 0070 | UCNarrowOverhangPosClass2Set | classSeqRuleSetOffsets[2] — contexts beginning with class 2 glyphs |
| 0000 | NULL | classSeqRuleSetOffsets[3] — no contexts begin with class 3 glyphs |
| 0000 | NULL | classSeqRuleSetOffsets[4] — no contexts begin with class 4 glyphs |
|  | **CoverageFormat1**  MoveAccentsCoverage | Coverage table definition |
| 0001 | 1 | format: lists |
| 0005 | 5 | glyphCount |
| 0029 | UppercaseFGlyphID | glyphArray[0] |
| 0033 | UppercasePGlyphID | glyphArray[1] |
| 0037 | UppercaseTGlyphID | glyphArray[2] |
| 0039 | UppercaseVGlyphID | glyphArray[3] |
| 003A | UppercaseWGlyphID | glyphArray[4] |
|  | **ClassDefFormat2**  MoveAccentsClassDef | ClassDef table definition defines five classes: 0 (all else), 1 (T, V, W: UCUnderhang), 2 (F, P: UCOverhang), 3 (a, e, I, o, u: LCVowels), 4 (tilde, umlaut) |
| 0002 | 2 | format: ranges |
| 000A | 10 | classRangeCount |
|  | classRangeRecords[0] |  |
| 0029 | UppercaseFGlyphID | startGlyphID |
| 0029 | UppercaseFGlyphID | endGlyphID |
| 0002 | 2 | class |
|  | classRangeRecords[1] |  |
| 0033 | UppercasePGlyphID | startGlyphID |
| 0033 | UppercasePGlyphID | endGlyphID |
| 0002 | 2 | class |
|  | classRangeRecords[2] |  |
| 0037 | UppercaseTGlyphID | startGlyphID |
| 0037 | UppercaseTGlyphID | endGlyphID |
| 0001 | 1 | class |
|  | classRangeRecords[3] |  |
| 0039 | UppercaseVGlyphID | startGlyphID |
| 003A | UppercaseWGlyphID | endGlyphID |
| 0001 | 1 | class |
|  | classRangeRecords[4] |  |
| 0042 | LowercaseAGlyphID | startGlyphID |
| 0042 | LowercaseAGlyphID | endGlyphID |
| 0003 | 3 | class |
|  | classRangeRecords[5] |  |
| 0046 | LowercaseEGlyphID | startGlyphID |
| 0046 | LowercaseEGlyphID | endGlyphID |
| 0003 | 3 | class |
|  | classRangeRecords[6] |  |
| 004A | LowercaseIGlyphID | startGlyphID |
| 004A | LowercaseIGlyphID | endGlyphID |
| 0003 | 3 | class |
|  | classRangeRecords[7] |  |
| 0051 | LowercaseOGlyphID | startGlyphID |
| 0051 | LowercaseOGlyphID | endGlyphID |
| 0003 | 3 | class |
|  | classRangeRecords[8] |  |
| 0056 | LowercaseUGlyphID | startGlyphID |
| 0056 | LowercaseUGlyphID | endGlyphID |
| 0003 | 3 | class |
|  | classRangeRecords[9] |  |
| 00F5 | TildeAccentGlyphID | startGlyphID |
| 00F6 | UmlautAccentGlyphID | endGlyphID |
| 0004 | 4 | class |
|  | **ClassSequenceRuleSet**  UCWideOverhangPosClass1Set | ClassSequenceRuleSet table definition |
| 0001 | 1 | classSeqRuleCount |
| 0004 | UCWideOverhangPosClassRule | classSeqRuleOffsets[0] |
|  | **ClassSequenceRule**  UCWideOverhangPosClassRule | ClassSequenceRule table definition |
| 0003 | 3 | glyphCount |
| 0001 | 1 | seqLookupCount |
| 0003 | 3 | inputClassSequence[0] — lowercase vowel |
| 0004 | 4 | inputClassSequence[1] — accent |
|  | seqLookupRecords[0] |  |
| 0002 | 2 | sequenceIndex |
| 0001 | 1 | lookupListIndex — lower the accent |
|  | **ClassSequenceRuleSet**  UCNarrowOverhangPosClass2Set | ClassSequenceRuleSet table definition |
| 0001 | 1 | classSeqRuleCount |
| 0004 | UCNarrowOverhangPosClassRule | classSeqRuleOffsets[0] |
|  | **ClassSequenceRule**  UCNarrowOverhangPosClassRule | ClassSequenceRule table definition |
| 0003 | 3 | glyphCount |
| 0001 | 1 | seqLookupCount |
| 0003 | 3 | inputClassSequence[0] — lowercase vowel |
| 0004 | 4 | inputClassSequence[1] — accent |
|  | seqLookupRecords[0] |  |
| 0000 | 0 | sequenceIndex |
| 0002 | 2 | lookupListIndex — increase overhang advance width |

