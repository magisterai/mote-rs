# Example 6: LigatureSubstFormat1 subtable

Source: [Glyph substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-gsub-structure-examples.md) · [Next](008-example-8-contextual-substitution-format-2.md)

<a id="EX6"></a>

<a id="example-6-ligaturesubstformat1-subtable"></a>

### Example 6: LigatureSubstFormat1 subtable

Example 6 shows a [LigatureSubstFormat1 subtable](004-lookup-type-3-subtable-alternate-substitution.md#LSF1) that defines data to replace a string of glyphs with a single ligature glyph. Because a LigatureSubstFormat1 subtable can specify glyph substitutions for more than one ligature, this subtable defines three ligatures: “etc”, “ffi”, and “fi.”

The sample subtable contains a format identifier (4) and an offset to a Coverage table. The Coverage table, which lists an index for each first glyph in the ligatures, lists indices for the “e” and “f” glyphs. The Coverage table range format is used here because the “e” and “f” glyph indices are numbered consecutively.

In the LigatureSubst subtable, ligatureSetCount specifies two LigatureSet tables, one for each covered glyph, and the ligatureSetOffsets array stores offsets to them. In this array, the “e” LigatureSet precedes the “f” LigatureSet, matching the order of the corresponding first-glyph components in the Coverage table.

Each LigatureSet table identifies all ligatures that begin with a covered glyph. The sample LigatureSet table defined for the “e” glyph contains only one ligature, “etc.” A LigatureSet table defined for the “f” glyph contains two ligatures, “ffi” and “fi.”

The sample FLigaturesSet table has offsets to two Ligature tables, one for “ffi” and one for “fi.” The ligatureOffsets array lists the “ffi” Ligature table first to indicate that the “ffi” ligature is preferred to the “fi” ligature.

*Example 6*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **LigatureSubstFormat1**  LigaturesSubtable | LigatureSubstFormat1 subtable definition |
| 0001 | 1 | format |
| 000A | LigaturesCoverage | offset to Coverage table |
| 0002 | 2 | ligatureSetCount |
| 0014 | ELigaturesSet | ligatureSetOffsets[0] (offset to LigatureSet table 0) — LigatureSet tables in Coverage index order |
| 0020 | FLigaturesSet | ligatureSetOffsets[1] |
|  | **CoverageFormat2**  LigaturesCoverage | Coverage table definition |
| 0002 | 2 | coverageFormat: ranges |
| 0001 | 1 | rangeCount |
|  | rangeRecords[0] |  |
| 0019 | eGlyphID | Start, first glyph ID |
| 001A | fGlyphID | End, last glyph ID in range |
| 0000 | 0 | startCoverageIndex: coverage index of start glyph ID = 0 |
|  | **LigatureSet**  ELigaturesSet | LigatureSet table definition — all ligatures that start with e |
| 0001 | 1 | ligatureCount |
| 0004 | etcLigature | ligatureOffsets[0] (offset to Ligature table 0) |
|  | **Ligature**  etcLigature | Ligature table definition |
| 015B | etcGlyphID | ligatureGlyph — output glyph ID |
| 0003 | 3 | componentCount |
| 0028 | tGlyphID | componentGlyphIDs[0] — second component in ligature |
| 0017 | cGlyphID | componentGlyphIDs[1] — third component in ligature |
|  | **LigatureSet**  FLigaturesSet | LigatureSet table definition all ligatures start with f |
| 0002 | 2 | ligatureCount |
| 0006 | ffiLigature | ligatureOffsets[0] — listed first because ffi ligature is preferred to fi ligature |
| 000E | fiLigature | ligatureOffsets[1] |
|  | **Ligature**  ffiLigature | Ligature table definition |
| 00F1 | ffiGlyphID | ligatureGlyph — output glyph ID |
| 0003 | 3 | componentCount |
| 001A | fGlyphID | componentGlyphIDs[0] — second component in ligature |
| 001D | iGlyphID | componentGlyphIDs[1] — third component in ligature |
|  | **Ligature**  fiLigature | Ligature table definition |
| 00F0 | fiGlyphID | ligatureGlyph — output glyph ID |
| 0002 | 2 | componentCount |
| 001D | iGlyphID | componentGlyphIDs[0] — second component in ligature |

<a id="EX7"></a>

<a id="example-7-contextual-substitution-format-1"></a>

### Example 7: Contextual substitution format 1

Example 7 illustrates [format 1 contextual substitution](004-lookup-type-3-subtable-alternate-substitution.md#CSF1), using a [SequenceContextFormat1](../../chapter2/chunks/006-class-definition-format-2.md#seqctxt1) subtable to replace a string of three glyphs with another string. For the French language system, the subtable defines a contextual substitution that replaces the input sequence, space-dash-space, with the output sequence, thin space-dash-thin space.

The contextual substitution, called Dash Lookup in this example, contains one SequenceContextFormat1 subtable called the DashSubtable. The subtable specifies two contexts: a SpaceGlyph followed by a DashGlyph, and a DashGlyph followed by a SpaceGlyph. In each sequence, a single substitution replaces the SpaceGlyph with a ThinSpaceGlyph.

The Coverage table, labeled DashCoverage, lists two glyph IDs for the first glyphs in the SpaceGlyph and DashGlyph sequences. One SequenceRuleSet table is defined for each covered glyph.

SpaceAndDashSubRuleSet lists all the contexts that begin with a SpaceGlyph. It contains an offset to one SequenceRule table (SpaceAndDashSubRule), which specifies two glyphs in the context sequence, the second of which is a DashGlyph. The SequenceRule table contains a SequenceLookupRecord that lists the position in the sequence where the glyph substitution should occur (position 0) and the index of the SpaceToThinSpaceLookup applied there to replace the SpaceGlyph with a ThinSpaceGlyph. DashAndSpaceSubRuleSet lists all the contexts that begin with a DashGlyph. An offset points to a SequenceRule table (DashAndSpaceSubRule), which specifies two glyphs in the context sequence, and the second one is a SpaceGlyph. The SequenceRule table contains a SequenceLookupRecord that lists the position in the sequence where the glyph substitution should occur, and an index to the same lookup used in the SpaceAndDashSubRule. The lookup replaces the SpaceGlyph with a ThinSpaceGlyph.

*Example 7*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SequenceContextFormat1**  DashSubtable | SequenceContextFormat1 subtable definition for Lookup[0], DashLookup |
| 0001 | 1 | format |
| 000A | DashCoverage | offset to Coverage table |
| 0002 | 2 | seqRuleSetCount |
| 0012 | SpaceAndDashSubRuleSet | seqRuleSetOffsets[0] (offset to SequenceRuleSet table 0) — SequenceRuleSets ordered by Coverage index |
| 0020 | DashAndSpaceSubRuleSet | seqRuleSetOffsets[1] |
|  | **CoverageFormat1**  DashCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 0002 | 2 | glyphCount |
| 0028 | SpaceGlyph | glyphArray[0] — glyphs in numeric order |
| 005D | DashGlyph | glyphArray[1], dash glyph ID |
|  | **SequenceRuleSet**  SpaceAndDashSubRuleSet | SequenceRuleSet[0] table definition |
| 0001 | 1 | seqRuleCount |
| 0004 | SpaceAndDashSubRule | seqRuleOffsets[0] (offset to SequenceRule table 0) — SequenceRule tables ordered by preference |
|  | **SequenceRule**  SpaceAndDashSubRule | SequenceRule[0] table definition |
| 0002 | 2 | glyphCount — number in input sequence |
| 0001 | 1 | seqLookupCount |
| 005D | DashGlyph | inputSequence[0], starting with second glyph — SpaceGlyph, in Coverage table, is first glyph |
|  | seqLookupRecords[0] |  |
| 0000 | 0 | sequenceIndex — substitution at first glyph position (0) |
| 0001 | 1 | lookupListIndex — index for SpaceToThinSpaceLookup in LookupList |
|  | **SequenceRuleSet**  DashAndSpaceSubRuleSet | SequenceRuleSet[1] table definition |
| 0001 | 1 | seqRuleCount |
| 0004 | DashAndSpaceSubRule | seqRuleOffsets[0] (offset to SequenceRule table 0) — SequenceRule tables ordered by preference |
|  | **SequenceRule**  DashAndSpaceSubRule | SequenceRule[0] table definition |
| 0002 | 2 | glyphCount — number in the input glyph sequence |
| 0001 | 1 | seqLookupCount |
| 0028 | SpaceGlyph | inputSequence[0] — starting with second glyph |
|  | seqLookupRecords[0] |  |
| 0001 | 1 | sequenceIndex — substitution at second glyph position (glyph sequence index = 1) |
| 0001 | 1 | lookupListIndex — index for SpaceToThinSpaceLookup |

