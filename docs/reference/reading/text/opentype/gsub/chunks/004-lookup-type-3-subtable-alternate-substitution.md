# Lookup type 3 subtable: alternate substitution

Source: [Glyph substitution](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/gsub).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-gsub-table-structures.md) · [Next](005-lookup-type-6-subtable-chained-contexts-substitution.md)

<a id="AS"></a>

<a id="lookup-type-3-subtable-alternate-substitution"></a>

### Lookup type 3 subtable: alternate substitution

An alternate substitution (AlternateSubst) subtable identifies any number of aesthetic alternatives from which a user can choose a glyph variant to replace the input glyph. For example, if a font contains four variants of the ampersand symbol, the 'cmap' table will specify the index of one of the four glyphs as the default glyph index, and an AlternateSubst subtable will list the indices of the other three glyphs as alternatives. A text-processing client would then have the option of replacing the default glyph with any of the three alternatives.

The subtable has one format.

<a id="ASF1"></a>

**Alternate substitution format 1**

The AlternateSubstFormat1 subtable contains a format identifier, an offset to a Coverage table containing the indices of glyphs with alternative forms, and an array of offsets to AlternateSet tables.

For each glyph in the Coverage table, an AlternateSet subtable contains a count of the alternative glyphs and an array of their glyph indices. Because all the glyphs are functionally equivalent, they can be in any order in the array.

[Example 5](006-gsub-structure-examples.md#EX5) at the end of this chapter shows how to replace the default ampersand glyph with alternative glyphs.

<a id="alternateSubst_1"></a>

*AlternateSubstFormat1 subtable*

<a id="substFormat_as1"></a>

<a id="coverageOffset_as1"></a>

<a id="alternateSetCount_as1"></a>

<a id="alternateSetOffsets_as1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier: format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of substitution subtable. |
| uint16 | alternateSetCount | Number of AlternateSet tables |
| Offset16 | alternateSetOffsets[alternateSetCount] | Array of offsets to AlternateSet tables. Offsets are from beginning of substitution subtable, ordered by Coverage index. |

<a id="alternateSetTbl"></a>

*AlternateSet table*

<a id="glyphCount_altSetTbl"></a>

<a id="alternateGlyphIDs_altSetTbl"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | glyphCount | Number of glyph IDs in the alternateGlyphIDs array. |
| uint16 | alternateGlyphIDs[glyphCount] | Array of alternate glyph IDs, in arbitrary order. |

<a id="LS"></a>

<a id="lookup-type-4-subtable-ligature-substitution"></a>

### Lookup type 4 subtable: ligature substitution

A ligature substitution (LigatureSubst) subtable identifies ligature substitutions where a single glyph replaces multiple glyphs. One LigatureSubst subtable can specify any number of ligature substitutions. The subtable has one format.

<a id="LSF1"></a>

**Ligature substitution format 1**

The LigatureSubstFormat1 subtable contains a format identifier, a Coverage table offset, and an array of offsets to LigatureSet tables. The Coverage table specifies only the index of the first glyph component of each ligature set.

[Example 6](007-example-6-ligaturesubstformat1-subtable.md#EX6) at the end of this chapter shows how to replace a string of glyphs with a single ligature.

<a id="ligatureSubst_1"></a>

*LigatureSubstFormat1 subtable*

<a id="substFormat_ls1"></a>

<a id="coverageOffset_ls1"></a>

<a id="ligatureSetCount_ls1"></a>

<a id="ligatureSetOffsets_ls1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier: format = 1. |
| Offset16 | coverageOffset | Offset to Coverage table, from beginning of substitution subtable. |
| uint16 | ligatureSetCount | Number of LigatureSet tables. |
| Offset16 | ligatureSetOffsets[ligatureSetCount] | Array of offsets to LigatureSet tables. Offsets are from beginning of substitution subtable, ordered by Coverage index. |

A LigatureSet table, one for each covered glyph, specifies all the ligature sequences that begin with the covered glyph. For example, if the Coverage table lists the glyph index for a lowercase “f,” then a LigatureSet table will define ligature that begin with “f”, such as the “ffl”, “fl”, “ffi”, “fi” and “ff” ligatures. If the Coverage table also lists the glyph index for a lowercase “e”, then a different LigatureSet table will define ligatures that begin with “e”, such as the “etc” ligature.

A LigatureSet table consists of a count of the ligatures that begin with the covered glyph and an array of offsets to Ligature tables, which define the glyphs in each ligature. The order in the Ligature offset array defines the preference for using the ligatures. For example, if the “ffl” ligature is preferable to the “ff” ligature, then the Ligature array would list the offset to the “ffl” Ligature table before the offset to the “ff” Ligature table.

<a id="ligatureSetTbl"></a>

*LigatureSet table*

<a id="ligatureCount_ligSetTbl"></a>

<a id="ligatureOffsets_litSetTbl"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | ligatureCount | Number of Ligature tables. |
| Offset16 | ligatureOffsets[LigatureCount] | Array of offsets to Ligature tables. Offsets are from beginning of LigatureSet table, ordered by preference. |

For each ligature in the set, a Ligature table specifies the glyph ID of the output ligature glyph; a count of the total number of component glyphs in the ligature, including the first component; and an array of glyph IDs for the components. The array starts with the second component glyph in the ligature (input glyph sequence index = 1, componentGlyphIDs array index = 0) because the first component glyph is specified in the Coverage table.

> *Note:* The componentGlyphIDs array lists glyph IDs according to the writing direction — that is, the logical order — of the text. For text written right to left, the right-most glyph will be first. Conversely, for text written left to right, the left-most glyph will be first.

<a id="ligTbl"></a>

*Ligature table*

<a id="ligatureGlyph_ligTbl"></a>

<a id="componentCount_ligTbl"></a>

<a id="componentGlyphIDs_ligTbl"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | ligatureGlyph | Glyph ID of ligature to substitute. |
| uint16 | componentCount | Number of components in the ligature. |
| uint16 | componentGlyphIDs[componentCount - 1] | Array of component glyph IDs — start with the second component, ordered in writing direction. |

<a id="CS"></a>

<a id="lookup-type-5-subtable-contextual-substitution"></a>

### Lookup type 5 subtable: contextual substitution

A contextual substitution subtable describes glyph substitutions in context that replace one or more glyphs within a certain pattern of glyphs.

Contextual substitution subtables can use any of three formats that are common to the GSUB and GPOS tables. These define input sequence patterns to be matched against the text glyph sequence, and then actions to be applied to glyphs within the input sequence. The actions are specified as “nested” lookups, and each is applied to a particular sequence position within the input sequence.

Each sequence position + nested lookup combination is specified in a [SequenceLookupRecord](../../chapter2/chunks/006-class-definition-format-2.md#sequence-lookup-record). Examples [7](007-example-6-ligaturesubstformat1-subtable.md#EX7), [8](008-example-8-contextual-substitution-format-2.md#EX8), and [9](008-example-8-contextual-substitution-format-2.md#EX9) at the end of this chapter illustrate use of sequence lookup records within the GSUB table.

While the subtable formats are common between the GSUB and GPOS tables, the lookups referenced by sequence lookup records within the GSUB table are referenced by index into the GSUB LookupList table. In this way, actions specified by a GSUB contextual lookup can only be substitutions.

An input sequence pattern is matched against the current glyph sequence before any substitution actions are performed. The substitutions may change the current glyph sequence, but that has no effect on the initial matching operation. For a given lookup subtable, there may be multiple sequence lookup records, and these are processed in the specified order. Each substitution action on the glyph sequence applies to the results from the preceding sequence lookup records. Note in particular that the sequence position index in each sequence lookup record is relative to the glyph sequence as modified by the actions of preceding SequenceLookupRecords.

For example, consider a contextual lookup specifying an input glyph sequence of four glyphs. Suppose that no substitution is performed on the first glyph, but that the middle two glyphs will be replaced with a ligature, and a single glyph will replace the fourth glyph. Suppose also that the actions are listed in that order.

- The first glyph is at sequence position 0. No SequenceLookupRecord is specified for sequence index 0.
- The first SequenceLookupRecord specifies sequence position 1 and gives a LookupList index referencing a ligature substitution lookup. The first glyph specified in the nested lookup will be the glyph at sequence position 1; the second glyph specified in the nested lookup will be the glyph at sequence position 2. After the nested substitution has been performed, there will be three glyphs in the sequence context, not four.
- The last SequenceLookupRecord is defined in terms of the modified sequence context, specifying sequence position 2, not position 3. The nested single-substitution lookup will specify the glyph at position 2 as its input glyph.

<a id="CSF1"></a>

**Contextual substitution format 1: simple glyph contexts**

Format 1 defines the context for a glyph substitution as a particular sequence of glyphs. For example, a context could be <xyz>, <holiday>, <!?\*#@>, or any other glyph sequence.

For example, suppose the glyph string <abc> is to be replaced with its reverse glyph string <cba>. The input context would be defined as the glyph sequence, <abc>. Two single-substitution actions can be specified: the “a” at sequence position 0 is substituted by “c”, and the “c” at sequence position 2 is substituted by “a”.

Format 1 contextual substitutions are implemented using a SequenceContextFormat1 table. See [Sequence context format 1: simple glyph contexts](../../chapter2/chunks/006-class-definition-format-2.md#seqctxt1) in the OpenType Layout Common Table Formats chapter for complete details.

[Example 7](007-example-6-ligaturesubstformat1-subtable.md#EX7) at the end of the chapter uses a SequenceContextFormat1 table to replace a sequence of three glyphs with a sequence preferred for the French language system.

<a id="CSF2"></a>

**Contextual substitution format 2: class-based glyph contexts**

Format 2 defines contexts for glyph substitutions as input sequence patterns, with patterns expressed in terms of glyph classes. The glyph classes are defined using a [Class Definition table](../../chapter2/chunks/005-coverage-table.md#class-definition-table). Several sequence patterns may be specified, with each pattern specifying a class of glyphs for each input sequence position.

For example, suppose that a swash capital glyph should replace each uppercase letter glyph that is preceded by a space glyph and followed by a lowercase letter glyph (a glyph sequence of space - uppercase - lowercase). The set of uppercase glyphs would constitute one glyph class (class 1), the set of lowercase glyphs would constitute a second class (class 2), and the space glyph would constitute a third class (class 3). The input context might be specified as a pattern of one glyph from class 3, followed by one glyph from class 1, followed by one glyph from class 2.

Format 2 contextual substitutions are implemented using a SequenceContextFormat2 table. See [Sequence context format 2: class-based glyph contexts](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt2) in the OpenType Layout Common Table Formats chapter for complete details.

[Example 8](008-example-8-contextual-substitution-format-2.md#EX8) at the end of this chapter uses a SequenceContextFormat2 table to substitute Arabic mark glyphs for base glyphs of different heights.

<a id="CSF3"></a>

**Contextual substitution format 3: coverage-based glyph contexts**

Format 3 defines a context for glyph substitutions as an input sequence pattern, with the pattern expressed in terms of Coverage tables. A different Coverage table is defined for each sequence position.

Format 3 is like format 2 in that patterns are defined using sets of glyphs. However, with the glyph classes used in format 2, each glyph is in exactly one class. With format 3, any glyph can occur in multiple Coverage tables.

Unlike Formats 1 and 2, however, this format can define only one context.

For example, consider an input context that contains a lowercase glyph (position 0), followed by an uppercase glyph (position 1), either a lowercase or numeral glyph (position 2), and then either a lowercase or uppercase vowel (position 3). This context requires four Coverage tables, one for each position:

- For position 0, the Coverage table lists the set of lowercase glyphs.
- For position 1, the Coverage table lists the set of uppercase glyphs.
- For position 2, the Coverage table lists the set of lowercase and numeral glyphs, a superset of the glyphs defined in the Coverage table for position 0.
- For position 3, the Coverage table lists the set of lowercase and uppercase vowels, a subset of the glyphs defined in the Coverage tables for both positions 0 and 1.

Format 3 contextual substitutions are implemented using a SequenceContextFormat3 table. See [Sequence context format 3: coverage-based glyph contexts](../../chapter2/chunks/007-sequence-context-format-2-class-based-glyph-contexts.md#seqctxt3) in the OpenType Layout Common Table Formats chapter for complete details.

