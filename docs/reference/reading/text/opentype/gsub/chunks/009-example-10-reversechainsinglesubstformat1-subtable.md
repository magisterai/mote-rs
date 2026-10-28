# Example 10: ReverseChainSingleSubstFormat1 subtable

Source: [Glyph substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](008-example-8-contextual-substitution-format-2.md)

<a id="EX10"></a>

<a id="example-10-reversechainsinglesubstformat1-subtable"></a>

### Example 10: ReverseChainSingleSubstFormat1 subtable

Example 10 uses a [ReverseChainSingleSubstFormat1 subtable](005-lookup-type-6-subtable-chained-contexts-substitution.md#RCCSF1) to substitute glyphs with a form that has a thick connection to the left (thick exit). This allows the glyph to correctly connect to the letter form to the left of it.

The ThickExitCoverage table is the listing of glyphs to be matched for substitution.

The LookaheadCoverage table, labeled ThickEntryCoverage, lists four glyph IDs for the glyph following a substitution coverage glyph. This lookahead coverage attempts to match the context that will cause the substitution to take place.

The substituteGlyphIDs array provides the glyphs to replace glyphs that correspond in order in the ThickExitCoverage table.

*Example 10*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **ReverseChainSingleSubstFormat1**  ThickConnect | ReverseChainSingleSubstFormat1 subtable definition |
| 0001 | 1 | format |
| 0068 | ThickExitCoverage | offset to Coverage table |
| 0000 | 0 | backtrackGlyphCount |
| 0000 | null - not used | backtrackCoverageOffsets[0] |
| 0001 | 1 | lookaheadGlyphCount |
| 0026 | ThickEntryCoverage | lookaheadCoverageOffsets[0] |
| 000C | 12 | glyphCount |
| 00A7 | BEm2 | substituteGlyphIDs[0] — substitute glyphs ordered by Coverage index |
| 00B9 | BEi3 | substituteGlyphIDs[1] |
| 00C5 | JIMm3 | substituteGlyphIDs[2] |
| 00D4 | JIMi2 | substituteGlyphIDs[3] |
| 00EA | SINm2 | substituteGlyphIDs[4] |
| 00F2 | SINi2 | substituteGlyphIDs[5] |
| 00FD | SADm2 | substituteGlyphIDs[6] |
| 010D | SADi2 | substituteGlyphIDs[7] |
| 011B | TOEm3 | substituteGlyphIDs[8] |
| 012B | TOEi3 | substituteGlyphIDs[9] |
| 013B | AINm2 | substituteGlyphIDs[10] |
| 0141 | AINi2 | substituteGlyphIDs[11] |
|  | **CoverageFormat1**  ThickEntryCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 001F | 31 | glyphCount |
| 00A5 | ALEFf1 | glyphArray[0] — glyphs in glyph ID order |
| 00A9 | BEm4 | glyphArray[1] |
| 00AA | BEm5 | glyphArray[2] |
| 00E2 | DALf1 | glyphArray[3] |
| 0167 | KAFf1 | glyphArray[4] |
| 0168 | KAFfs1 | glyphArray[5] |
| 0169 | KAFm1 | glyphArray[6] |
| 016D | KAFm5 | glyphArray[7] |
| 016E | KAFm6 | glyphArray[8] |
| 0170 | KAFm8 | glyphArray[9] |
| 0183 | GAFf1 | glyphArray[10] |
| 0184 | GAFfs1 | glyphArray[11] |
| 0185 | GAFm1 | glyphArray[12] |
| 0189 | GAFm5 | glyphArray[13] |
| 018A | GAFm6 | glyphArray[14] |
| 018C | GAFm8 | glyphArray[15] |
| 019F | LAMf1 | glyphArray[16] |
| 01A0 | LAMm1 | glyphArray[17] |
| 01A1 | LAMm2 | glyphArray[18] |
| 01A2 | LAMm3 | glyphArray[19] |
| 01A3 | LAMm4 | glyphArray[20] |
| 01A4 | LAMm5 | glyphArray[21] |
| 01A5 | LAMm6 | glyphArray[22] |
| 01A6 | LAMm7 | glyphArray[23] |
| 01A7 | LAMm8 | glyphArray[24] |
| 01A8 | LAMm9 | glyphArray[25] |
| 01A9 | LAMm10 | glyphArray[26] |
| 01AA | LAMm11 | glyphArray[27] |
| 01AB | LAMm12 | glyphArray[28] |
| 01AC | LAMm13 | glyphArray[29] |
| 01EC | HAYf2 | glyphArray[30] |
|  | **CoverageFormat1**  ThickExitCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 000C | 12 | glyphCount |
| 00A6 | BEm1 | glyphArray[0] |
| 00B7 | BEi1 | glyphArray[1] |
| 00C3 | JIMm1 | glyphArray[2] |
| 00D2 | JIMi1 | glyphArray[3] |
| 00E9 | SINm1 | glyphArray[4] |
| 00F1 | SINi1 | glyphArray[5] |
| 00FC | SADm1 | glyphArray[6] |
| 010C | SADi1 | glyphArray[7] |
| 0119 | TOEm1 | glyphArray[8] |
| 0129 | TOEi1 | glyphArray[9] |
| 013A | AINm1 | glyphArray[10] |
| 0140 | AINi1 | glyphArray[11] |
