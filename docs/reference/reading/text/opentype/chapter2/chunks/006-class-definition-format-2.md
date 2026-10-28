# Class definition format 2

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-coverage-table.md) · [Next](007-sequence-context-format-2-class-based-glyph-contexts.md)

<a id="classDefFormat2"></a>

<a id="class-definition-format-2"></a>

### Class definition format 2

The second class definition format (ClassDefFormat2) defines multiple groups of glyph indices that belong to the same class. Each group consists of a range of glyph indices in consecutive order. The glyph ranges must not overlap.

The ClassDefFormat2 table contains a format identifier and an array of ClassRange records that specify a range of glyph IDs and the class to which they are assigned. The records must be sorted by the first glyph ID in each range.

Any glyph not covered by a ClassRange record is assigned to Class 0.

Example 8 at the end of this chapter uses format 2 to assign class values to four types of glyphs in the Arabic script.

<a id="classDef2"></a>

*ClassDefFormat2 table: Class ranges*

<a id="classFormat_cdef2"></a>

<a id="classRangeCount"></a>

<a id="classRangeRecs"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| uint16 | classRangeCount | Number of ClassRange records. |
| ClassRange | classRangeRecords[classRangeCount] | Array of ClassRangeRecords — ordered by startGlyphID. |

<a id="classRangeRec"></a>

*ClassRange record*

<a id="startGlyphId_clsRngRec"></a>

<a id="endGlyphId_clsRngRec"></a>

<a id="class"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | startGlyphID | First glyph ID in the range. |
| uint16 | endGlyphID | Last glyph ID in the range. |
| uint16 | class | Applied to all glyphs in the range. |

<a id="ctxtlookupformats"></a>

<a id="common-formats-for-contextual-lookup-subtables"></a>

## Common formats for contextual lookup subtables

The GSUB and GPOS tables each use different lookup types for various substitution and positioning operations. For both GSUB and GPOS, a *contextual* lookup type is defined:

- GSUB type 5: contextual substitution
- GPOS type 7: contextual positioning

The contextual lookup types support specifying *input glyph sequences* that are acted upon, as well as a list of actions to be taken on any glyph within the sequence. Actions are specified as references to separate *nested* lookups (an index into the LookupList). The actions are specified for each glyph position, but the entire sequence must be matched, and so the actions are specified in a context-sensitive manner.

Because contextual lookup tables link to “nested” lookup tables that describe the substitution actions to be performed, there are lookupFlag fields in each nested lookup table as well as in the main lookup table. See the [Lookup table](004-lookuplist-table.md#lookup-table) section above for details regarding the effect of lookup flags in the main and nested lookup tables.

> *Note:* Glyph sequences are given in logical order. For text written from right to left, the right-most glyph will be first; conversely, for text written from left to right, the left-most glyph will be first.

For both GSUB type 5 and GPOS type 7, there are three subtable formats defined, which describe the input sequences in different ways:

1. In terms of specific glyph IDs (“simple glyph contexts”).
2. In terms of sets of glyphs defined using Class Definition tables (“class-based glyph contexts”).
3. In terms of sets of glyphs defined using Coverage tables (“coverage-based glyph contexts”).

The three different subtable formats use different structures, but for each subtable format the structures are common to both the GSUB and GPOS tables.

Also, for both GSUB and GPOS, a *chained context* lookup type is defined:

- GSUB type 6: chained contexts substitution
- GPOS type 8: chained contexts positioning

The chained contextual lookups are functionally similar to the contextual lookups, but add chained glyph-sequence contexts: a *backtrack* glyph sequence that precedes the input sequence, and a *lookahead* sequence that follows the input sequence. (The backtrack and lookahead sequences are described in greater detail below: see [Chained sequence context format 1: simple glyph contexts](007-sequence-context-format-2-class-based-glyph-contexts.md#chseqctxt1).) Actions can be specified only for glyphs in the input sequence, but backtrack, input and lookahead sequence must match the current glyph sequence being processed for the lookup to apply. Once the specified actions are completed, the client advances to the glyph position immediately following the matched input sequence (with special consideration in the case that a nested lookup is a GPOS type 2, paired positioning, lookup); in particular, the client does *not* advance past the matched lookahead sequence.

Three formats are defined for chained contextual lookups, analogous to the three formats for the contextual lookups. The structures differ from those used for the contextual lookup types because they incorporate the chained contexts. But the chained context structures are common to both the GSUB and GPOS tables.

> *Note:* While substitution and positioning operations are distinct between GSUB and GPOS lookups, for contextual lookup types that difference is reflected in the nested lookups. For instance, the actions specified by a GSUB contextual lookup are specified by reference to a nested lookup *within the GSUB table*. In this way, the contextual lookup subtable structures used in the GSUB and GPOS tables can be identical, but the resulting operations that are specified will still be distinct.

For both contextual and chained contextual lookup types, input sequence patterns are defined, and actions on matching glyph sequences are performed. Note that patterns are matched against the current glyph sequence before any actions are performed. Within the GSUB table, substitution actions will change the current glyph sequence, but this does not affect the initial matching operation.

<a id="seqlookuprec"></a>

<a id="sequence-lookup-record"></a>

### Sequence lookup record

For all formats for both contextual and chained contextual lookups, a common record format is used to specify an action—a nested lookup—to be applied to a glyph at a particular sequence position within the input sequence.

*SequenceLookup record*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | sequenceIndex | Index (zero-based) into the input glyph sequence. |
| uint16 | lookupListIndex | Index (zero-based) into the LookupList. |

The lookupListIndex field indicates the Lookup table to apply to the position in the input glyph sequence indicated by sequenceIndex.

<a id="seqctxt1"></a>

<a id="sequence-context-format-1-simple-glyph-contexts"></a>

### Sequence context format 1: simple glyph contexts

GSUB type 5 format 1 subtables and GPOS type 7 format 1 subtables define input sequences in terms of specific glyph IDs. Several sequences may be specified, but each is specified using glyph IDs.

The first glyph for each sequence is specified in a Coverage table. The remaining glyphs in each sequence are defined in *SequenceRule* tables—one for each sequence. If multiple sequences start with the same glyph, that glyph ID must be listed once in the Coverage table, and the corresponding sequence rules are aggregated using a *SequenceRuleSet table*—one for each initial glyph specified in the Coverage table.

When evaluating a SequenceContextFormat1 subtable for a given position in a glyph sequence, the client searches for the current glyph in the Coverage table. If found, the corresponding SequenceRuleSet table is retrieved, and the SequenceRule tables for that set are examined to see if the current glyph sequence matches the input sequence pattern in any of the sequence rules. The first matching rule subtable is used.

*SequenceContextFormat1 table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of SequenceContextFormat1 table. |
| uint16 | seqRuleSetCount | Number of SequenceRuleSet tables. |
| Offset16 | seqRuleSetOffsets[seqRuleSetCount] | Array of offsets to SequenceRuleSet tables, from beginning of SequenceContextFormat1 table (offsets may be NULL). |

The Coverage table lists the initial glyphs from all of the supported input glyph sequences. The seqRuleSetCount should match the number of glyphs in the Coverage table. If these differ, the extra coverage glyphs or extra sequence rule sets are ignored.

There is one SequenceRuleSet table for each covered glyph. The offsets in the seqRuleSetOffsets array must be ordered to match the order of glyphs in the Coverage table.

*SequenceRuleSet table—all contexts beginning with the same glyph*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | seqRuleCount | Number of SequenceRule tables. |
| Offset16 | seqRuleOffsets[seqRuleCount] | Array of offsets to SequenceRule tables, from beginning of the SequenceRuleSet table. |

Offsets to SequenceRule subtables are ordered according to the desired results. The subtables are evaluated in the order the offsets are listed, and the first sequence rule that matches the current glyph sequence is used. Rules for longer, more specific sequences are typically ordered before shorter rules.

> *Note:* If a rule specifies a sequence that is an initial sub-sequence of a longer sequence specified in another rule and the shorter is ordered before the longer, the rule for the longer sequence will never be used. For example, consider two contexts <abc> and <abcd>: if <abc> is first in the sequence rule array, all instances of <abc> in the text, including all instances of <abcd>, will be matched. The second sequence rule, for <abcd>, will never be used.

*SequenceRule table*

| Type | Name | Description |
| --- | --- | --- |
| uint16 | glyphCount | Number of glyphs in the input glyph sequence. |
| uint16 | seqLookupCount | Number of SequenceLookup. |
| uint16 | inputSequence[glyphCount - 1] | Array of input glyph IDs—starting with the second glyph. |
| SequenceLookup | seqLookupRecords[seqLookupCount] | Array of sequence lookup records. |

The glyphCount value is the total number of glyphs in the input sequence, including the first glyph. The inputSequence array specifies the *remaining* glyphs in the input sequence, in order. (The glyph at inputSequence index 0 corresponds to glyph sequence index 1.)

The seqLookupRecords array lists the sequence lookup records that specify actions to be taken on glyphs at various positions within the input sequence. These do not have to be ordered in sequence position order; they are ordered according to the desired result. All of the sequence lookup records are processed in order, and each applies to the results of the actions indicated by the preceding record.

