# GSUB structure examples

Source: [Glyph substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-lookup-type-6-subtable-chained-contexts-substitution.md) · [Next](007-example-6-ligaturesubstformat1-subtable.md)

<a id="examples"></a>

<a id="gsub-structure-examples"></a>

## GSUB structure examples

The rest of this chapter describes and illustrates examples of the various GSUB subtables, including each of the three formats available for contextual substitutions. All the examples reflect unique parameters described below, but the samples provide a useful reference for building subtables specific to other situations.

All the examples have three columns showing hex data, source, and comments.

<a id="EX1"></a>

<a id="example-1-gsub-header"></a>

### Example 1: GSUB Header

Example 1 shows a typical [GSUB Header table](003-gsub-table-structures.md#header) definition.

*Example 1*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **GSUBHeader**  TheGSUBHeader | GSUBHeader table definition |
| 00010000 | 0x00010000 | major/minor version |
| 000A | TheScriptList | offset to ScriptList table |
| 001E | TheFeatureList | offset to FeatureList table |
| 002C | TheLookupList | offset to LookupList table |

<a id="EX2"></a>

<a id="example-2-singlesubstformat1-subtable"></a>

### Example 2: SingleSubstFormat1 subtable

Example 2 illustrates the [SingleSubstFormat1 subtable](003-gsub-table-structures.md#SSF1) , which uses ranges to replace single input glyphs with their corresponding output glyphs. The indices of the output glyphs are calculated by adding a constant delta value to the indices of the input glyphs. In this example, the Coverage table has a format identifier of 2 to indicate the range format, which is used because the input glyph indices are in consecutive order in the font. The Coverage table specifies one range that contains a startGlyphID for the “0” (zero) glyph and an endGlyphID for the “9” glyph.

*Example 2*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SingleSubstFormat1**  LiningNumeralSubtable | SingleSubst subtable definition |
| 0001 | 1 | format: calculated output glyph indices |
| 0006 | LiningNumeralCoverage | offset to Coverage table for input glyphs |
| 00C0 | 192 | deltaGlyphID = 192: add to each input glyph index to produce output glyph index |
|  | **CoverageFormat2**  LiningNumeralCoverage | Coverage table definition |
| 0002 | 2 | coverageFormat: ranges |
|  | 1 | rangeCount |
|  | rangeRecords[0] |  |
| 004E | 78 | Start glyph ID for numeral zero glyph |
| 0058 | 87 | End glyph ID for numeral nine glyph |
| 0000 | 0 | startCoverageIndex: first CoverageIndex = 0 |

<a id="EX3"></a>

<a id="example-3-singlesubstformat2-subtable"></a>

### Example 3: SingleSubstFormat2 subtable

Example 3 uses the [SingleSubstFormat2 subtable](003-gsub-table-structures.md#SSF2) for lists to substitute punctuation glyphs in Japanese text that is written vertically. Horizontally oriented parentheses and square brackets (the input glyphs) are replaced with vertically oriented parentheses and square brackets (the output glyphs).

The Coverage table, format 1, identifies each input glyph index. The number of input glyph indices listed in the Coverage table matches the number of output glyph indices listed in the subtable. For correct substitution, the order of the glyph indices in the Coverage table (input glyphs) must match the order in the Substitute array (output glyphs).

*Example 3*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **SingleSubstFormat2**  VerticalPunctuationSubtable | SingleSubst subtable definition |
| 0002 | 2 | format: lists |
| 000E | VerticalPunctuationCoverage | offset to Coverage table |
| 0004 | 4 | glyphCount — equals glyphCount in Coverage table |
| 0131 | VerticalOpenBracketGlyph | substituteGlyphIDs[0], ordered by Coverage index |
| 0135 | VerticalClosedBracketGlyph | substituteGlyphIDs[1] |
| 013E | VerticalOpenParenthesisGlyph | substituteGlyphIDs[2] |
| 0143 | VerticalClosedParenthesisGlyph | substituteGlyphIDs[3] |
|  | **CoverageFormat1**  VerticalPunctuationCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 0004 | 4 | glyphCount |
| 003C | HorizontalOpenBracketGlyph | glyphArray[0], ordered by glyph ID |
| 0040 | HorizontalClosedBracketGlyph | glyphArray[1] |
| 004B | HorizontalOpenParenthesisGlyph | glyphArray[2] |
| 004F | HorizontalClosedParenthesisGlyph | glyphArray[3] |

<a id="EX4"></a>

<a id="example-4-multiplesubstformat1-subtable"></a>

### Example 4: MultipleSubstFormat1 subtable

Example 4 uses a [MultipleSubstFormat1 subtable](003-gsub-table-structures.md#MSF1) to replace a single “ffi” ligature with three individual glyphs that form the string <ffi>. The subtable defines a format identifier of 1, an offset to a Coverage table that specifies the glyph index of the “ffi” ligature (the input glyph), an offset to a Sequence table that specifies the sequence of glyph indices for the <ffi> string in its substitute array (the output glyph sequence), and a count of Sequence table offsets.

*Example 4*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **MultipleSubstFormat1**  FfiDecompSubtable | MultipleSubst subtable definition |
| 0001 | 1 | format |
| 0008 | FfiDecompCoverage | offset to Coverage table |
| 0001 | 1 | sequenceCount — equals glyphCount in Coverage table |
| 000E | FfiDecompSequence | sequenceOffsets[0] (offset to Sequence table 0) |
|  | **CoverageFormat1**  FfiDecompCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 0001 | 1 | glyphCount |
| 00F1 | ffiGlyphID | ligature glyph |
|  | **Sequence**  FfiDecompSequence | Sequence table definition |
| 0003 | 3 | glyphCount |
| 001A | fGlyphID | first glyph in sequence order |
| 001A | fGlyphID | second glyph |
| 001D | iGlyphID | third glyph |

<a id="EX5"></a>

<a id="example-5-alternatesubstformat-1-subtable"></a>

### Example 5: AlternateSubstFormat 1 subtable

Example 5 uses the [AlternateSubstFormat1 subtable](004-lookup-type-3-subtable-alternate-substitution.md#ASF1) to replace the default ampersand glyph (input glyph) with one of two alternative ampersand glyphs (output glyph).

In this case, the Coverage table specifies the index of a single glyph, the default ampersand, because it is the only glyph covered by this lookup. The AlternateSet table for this covered glyph identifies the alternative glyphs: AltAmpersand1GlyphID and AltAmpersand2GlyphID.

In Example 5, the index position of the AlternateSet table offset in the AlternateSet array is zero (0), which correlates with the index position (also zero) of the default ampersand glyph in the Coverage table.

*Example 5*

| Hex Data | Source | Comments |
| --- | --- | --- |
|  | **AlternateSubstFormat1**  AltAmpersandSubtable | AlternateSubstFormat1 subtable definition |
| 0001 | 1 | format |
| 0008 | AltAmpersandCoverage | offset to Coverage table |
| 0001 | 1 | alternateSetCount — equals glyphCount in Coverage table |
| 000E | AltAmpersandSet | alternateSetOffsets[0] (offset to AlternateSet table 0) |
|  | **CoverageFormat1**  AltAmpersandCoverage | Coverage table definition |
| 0001 | 1 | coverageFormat: lists |
| 0001 | 1 | glyphCount |
| 003A | DefaultAmpersandGlyphID | glyphArray[0] |
|  | **AlternateSet**  AltAmpersandSet | AlternateSet table definition |
| 0002 | 2 | glyphCount |
| 00C9 | AltAmpersand1GlyphID | alternateGlyphIDs[0] — glyphs in arbitrary order |
| 00CA | AltAmpersand2GlyphID | alternateGlyphIDs[1] |

