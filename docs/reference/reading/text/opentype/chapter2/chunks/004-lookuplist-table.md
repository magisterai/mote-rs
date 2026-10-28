# LookupList table

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-features-and-lookups.md) · [Next](005-coverage-table.md)

<a id="lulTbl"></a>

<a id="lookuplist-table"></a>

### LookupList table

The headers of the GSUB and GPOS tables contain offsets to LookupList tables for glyph substitution and glyph positioning lookups. The LookupList table contains an array of offsets to Lookup tables. The font developer defines the order of offsets in the array to control the order in which a text-processing client processes lookup data to perform glyph substitution or positioning operations.

Example 4 at the end of this chapter shows three ligature lookups in a LookupList table.

<a id="lookupList"></a>

*LookupList table*

<a id="lookupCount_lul"></a>

<a id="lookups"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | lookupCount | Number of elements in the lookupOffsets array. |
| Offset16 | lookupOffsets[lookupCount] | Array of offsets to Lookup tables, from beginning of LookupList. |

<a id="lookupTbl"></a>

<a id="lookup-table"></a>

### Lookup table

A Lookup table defines the specific conditions, type, and results of substitution or positioning actions that are used to implement a feature. For example, a substitution operation requires a list of target glyph indices to be replaced, a list of replacement glyph indices, and a description of the type of substitution action.

The data describing the actions of a lookup are contained in one or more lookup subtables. Different lookup types support different types of operation; for example, positioning adjustment on a single glyph versus positioning adjustments on pairs of glyphs. The type of operation determines the information that needs to be included in the lookup subtables. A given lookup table may support only one type of operation, and so may contain only subtables of the same lookup type.

The GSUB table supports eight lookup types; the GPOS table supports nine lookup types. See the [GSUB](../../gsub/README.md) and [GPOS](../../gpos/README.md) chapters for details about the various types of substitution and positioning lookups.

For each lookup type, one or more subtable formats are defined. Each format is determined by the content of the information required for an operation and by required storage efficiency. When glyph information is best presented in more than one format, a single lookup may contain more than one subtable, as long as all the subtables are the same lookup type. For example, within a given lookup, a glyph index array format may best represent one set of target glyphs, whereas a glyph index range format may be better for another set of target glyphs.

During text processing, a client applies a feature to some sequence of glyphs for a string. It then processes the lookups referenced by that feature in their lookup list order. For each lookup, the client processes that lookup over each glyph in the sequence to which the feature has been applied. After that lookup has been processed over the entire glyph sequence, it then processes the next lookup referenced by the feature in the same manner. As each lookup is processed, it acts on the substitution or positioning results of the previous lookups. This continues until all lookups referenced by the feature have been processed.

An application may process lookups for multiple features simultaneously. In this case, the list of lookups is the union of lookups referenced by all of those features, and these are all processed in their lookup list order. If the different features have been applied to different glyph sub-sequences for a string, each lookup is applied only to the sub-sequence to which the feature that referenced that lookup was applied.

A lookup specifies one or more *input sequence* patterns, each with one or more glyphs. When the lookup is processed over a glyph sequence within a string, the client starts with the first glyph in the sequence, testing for a match with the input sequence patterns specified by the lookup. If the glyph sequence does not match any of the lookup input sequence patterns, processing of the lookup is finished for that glyph, and the client advances to the next glyph in the glyph sequence. If the glyph sequence *does* match a lookup input sequence pattern, then the corresponding substitution or position operation is performed on the matched input sequence. A lookup is finished for a glyph after the client has performed the substitution or positioning operation. To move to the “next” glyph, the client skips all the glyphs that participated in the lookup operation: glyphs that were substituted/positioned as well as any other glyphs in the matched input sequence. However, an exception to this rule is made in the case of pair positioning operations (for example, kerning): the “next” glyph in a sequence may be the second glyph of the input sequence pair, rather than skipping over the entire input sequence. (See the [pair positioning lookup type](../../gpos/chunks/003-gpos-table-structures.md#lookup-type-2-subtable-pair-adjustment-positioning) for details.)

If a Lookup table has multiple subtables, the subtables are processed in order, testing the glyph sequence at the current glyph position for a match with the input sequence patterns specified by each subtable in turn. If there is no match with the pattern of a subtable, then processing moves to the next subtable. If the glyph sequence does not match the patterns of any lookup subtable, processing of the lookup is finished for that glyph position. If the glyph sequence does match the pattern of a subtable, then the operations of that subtable are performed and processing of the lookup is finished—no further subtables are processed for that position in the glyph sequence.

Both the GPOS and GSUB tables include lookup types that allow *chained contexts*: [GSUB lookup type 6](../../gsub/chunks/005-lookup-type-6-subtable-chained-contexts-substitution.md#lookup-type-6-subtable-chained-contexts-substitution), and [GPOS lookup type 8](../../gpos/chunks/007-lookup-type-8-subtable-chained-contexts-positioning.md#lookup-type-8-subtable-chained-contexts-positioning). Chained context lookup types support specifying *backtrack* and *lookahead* sequence patterns that precede and follow the input sequence pattern. These must also be matched in the glyph sequence for a lookup to apply at a given position in the glyph sequence. Unlike the input sequence pattern, matching of the backtrack and lookahead sequences is not restricted to the glyph sequence to which the associated feature was applied. A chained context lookup can specify actions for glyphs in the matched input sequence, but not in the backtrack or lookahead sequences. After a chained context lookup has been processed, the client sets the “next” glyph by skipping over the input sequence, but not the lookahead sequence.

A Lookup table specifies a lookup type that defines the type of information stored in lookup subtables. A lookup flag specifies lookup qualifiers that indicate to a text-processing client certain processing options to use when substituting or positioning glyphs. An array of offsets provides offsets from the start of the Lookup table to one or more lookup subtables of the specified lookup type. An optional field provides an additional qualifier on mark glyphs.

<a id="lookupTable"></a>

*Lookup table*

<a id="lookupType"></a>

<a id="lookupFlag"></a>

<a id="subTableCount"></a>

<a id="subTables"></a>

<a id="markFilteringSet"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | lookupType | Different enumerations for GSUB and GPOS. |
| uint16 | lookupFlag | Lookup qualifiers. |
| uint16 | subTableCount | Number of elements in the subtableOffsets array. |
| Offset16 | subtableOffsets[subTableCount] | Array of offsets to lookup subtables, from beginning of Lookup table. |
| uint16 | markFilteringSet | Index (base 0) into GDEF mark glyph sets structure. This field is only present if the USE\_MARK\_FILTERING\_SET lookup flag is set. |

The order of offsets in the subtableOffsets array determines the order in which lookup subtables will be processed.

The lookup flag uses two bytes of data:

- Each of the first four low-order bits can be set in order to specify additional instructions for applying a lookup to a glyph sequence. The table below provides details about the use of these bits.
- The fifth bit indicates the presence of a markFilteringSet field in the Lookup table.
- The next three bits are reserved for future use.
- The high byte can be used to specify a class of mark glyphs as a glyph filter.

<a id="lookupFlags"></a>

*LookupFlag bit enumeration*

<a id="lookupFlags_1"></a>

<a id="lookupFlags_2"></a>

<a id="lookupFlags_4"></a>

<a id="lookupFlags_8"></a>

<a id="lookupFlags_10"></a>

<a id="lookupFlags_E0"></a>

<a id="lookupFlags_FF00"></a>

| Mask | Name | Description |
| --- | --- | --- |
| 0x0001 | RIGHT\_TO\_LEFT | This bit is used only in relation to cursive attachment positioning (GPOS lookup type 3). When this bit is set, the last glyph in a matched input sequence will be positioned on the baseline. |
| 0x0002 | IGNORE\_BASE\_GLYPHS | If set, skips over base glyphs |
| 0x0004 | IGNORE\_LIGATURES | If set, skips over ligatures |
| 0x0008 | IGNORE\_MARKS | If set, skips over all combining marks |
| 0x0010 | USE\_MARK\_FILTERING\_SET | If set, indicates that the Lookup table structure includes the markFilteringSet field. The layout engine skips over all mark glyphs not in the mark filtering set indicated. |
| 0x00E0 | reserved | For future use (Set to zero) |
| 0xFF00 | MARK\_ATTACHMENT\_CLASS\_FILTER | If not zero, skips over all marks not in the specified mark attachment class. |

The RIGHT\_TO\_LEFT flag is used only for GPOS type 3 lookups and is ignored otherwise. It is not used by client software in determining text direction.

As described above, lookups are processed for each glyph in a glyph sequence to which a feature has been applied. Each lookup type specifies an input sequence pattern to be matched: single glyphs, or sequences of glyphs, depending upon the lookup type. The current glyph in the lookup processing loop is always tested against the first glyph in a lookup's input glyph sequence pattern. Lookup flags affect pattern matching for other glyphs in the input sequence but not the current glyph. For chained context lookups, the flags also affect matching of backtrack and lookahead sequences.

IGNORE\_BASE\_GLYPHS, IGNORE\_LIGATURES, or IGNORE\_MARKS refer to base glyphs, ligatures and marks as defined in the Glyph Class Definition table in the GDEF table. If any of these flags are set, a Glyph Class Definition table must be present. If any of these bits is set, then lookups must ignore glyphs of the respective type; that is, the other glyphs must be processed just as though these glyphs were not present in the glyph sequence.

If MARK\_ATTACHMENT\_CLASS\_FILTER is non-zero, then mark attachment classes must be defined in the Mark Attachment Class Definition table in the GDEF table. When processing glyph sequences, a lookup must ignore any mark glyphs that are not in the specified mark attachment class; only marks in the specified class are processed.

If any lookup has the USE\_MARK\_FILTERING\_SET flag set, then the Lookup header must include the markFilteringSet field and a MarkGlyphSets table must be present in GDEF table. The lookup must ignore any mark glyphs that are not in the specified mark glyph set; only glyphs in the specified mark glyph set are processed.

If a mark filtering set is specified, this supersedes any mark attachment class indication in the lookup flag. If the IGNORE\_MARKS bit is set, this supersedes any mark filtering set or mark attachment class indications.

For example, in Arabic text, a character string might have the pattern *base mark base*. That string could be converted into a ligature composed of two components, one for each base character, with the combining mark glyph over the first component. To produce this ligature, the font developer would set the IGNORE\_MARKS bit of the ligature substitution lookup to tell the client to ignore the mark, substitute the ligature glyph first, and then position the mark glyph over the ligature glyph in a subsequent GPOS lookup. Alternatively, a substitution lookup which did not set the IGNORE\_MARKS bit could be used to describe a three-component ligature glyph, composed of the first base glyph, the mark glyph, and the second base glyph.

For another example, a lookup that creates a ligature of a base glyph with an above mark could skip over all below marks by specifying a mark attachment class that includes only above marks.

Contextual lookup types support a nested organization of lookup data. In this structure, a lookup subtable specifies an input sequence pattern for glyph sequences that can be modified, and then references one or more “nested” lookup tables in the LookupList that describe the actions to be applied to individual glyphs within a matching sequence. In these cases, there is a lookupFlag field in the main lookup table, and separate lookupFlag fields in the nested lookups.

GPOS lookup type 7 and type 8 have this nature, as well as GSUB lookup type 5, type 6, and type 8. For these lookup types, the effect of the main lookup is to filter glyph sequences to which they apply, but not directly to modify glyphs in a matched sequence. The lookup flags in the main lookup table will affect this initial matching process. For example, the IGNORE\_MARKS flag will cause mark glyphs to be ignored when evaluating if a glyph sequence matches the pattern specified by the lookup. Note that the RIGHT\_TO\_LEFT flag is never used in the main lookup.

