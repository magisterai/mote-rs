# Coverage table

Source: [OpenType layout common table formats](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](004-lookuplist-table.md) · [Next](006-class-definition-format-2.md)

Once a sequence is matched against the pattern in the main lookup, the nested lookups are then processed on glyphs in the sequence. Lookup flags in the main lookup table are not considered while the nested lookups are being processed. Instead, lookup flags in the nested lookups are considered. Note that the flags in the nested lookup can result in a secondary level of filtering over sequences initially matched by the main lookup table.

<a id="coverageTbl"></a>

<a id="coverage-table"></a>

## Coverage table

Each subtable in a Lookup table (except an Extension lookup type subtable) references a Coverage table, which specifies all the glyphs affected by a substitution or positioning operation described in the subtable. The GSUB, GPOS, and GDEF tables rely on this notion of coverage. If a glyph does not appear in a Coverage table, the client can skip that subtable and move immediately to the next subtable.

A Coverage table identifies glyphs by glyph IDs in either of two ways:

- As a list of individual glyph IDs in the glyph set.
- As ranges of consecutive glyph IDs. The range format gives one or more start and end glyph ID pairs to denote the consecutive glyphs covered by the table.

In a Coverage table, a format field specifies the format as an integer: 1 = lists, and 2 = ranges.

A Coverage table defines a unique index value, the *Coverage Index*, for each covered glyph. The Coverage Indexes are sequential, from 0 to the number of covered glyphs minus 1. This unique value specifies the position of the covered glyph in the Coverage table. The client uses the Coverage Index to look up values in the subtable for each glyph.

<a id="coverageFormat1"></a>

<a id="coverage-format-1"></a>

### Coverage format 1

Coverage format 1 consists of a format field and a count of covered glyphs, followed by an array of glyph indices (glyphArray). The glyph indices must be in numerical order for binary searching of the list. When a glyph is found in the Coverage table, its position in the glyphArray determines the Coverage Index that is returned — the first glyph has a Coverage Index = 0, and the last glyph has a Coverage Index = GlyphCount -1.

Example 5 at the end of this chapter shows a Coverage table that uses format 1 to list the glyph IDs of all lowercase descender glyphs in a font.

<a id="coverage1"></a>

*CoverageFormat1 table: Individual glyph indices*

<a id="coverageFormat_cov1"></a>

<a id="glyphCount_cov1"></a>

<a id="glyphArray"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| uint16 | glyphCount | Number of glyphs in the glyph array. |
| uint16 | glyphArray[glyphCount] | Array of glyph IDs — in numerical order. |

<a id="coverageFormat2"></a>

<a id="coverage-format-2"></a>

### Coverage format 2

Format 2 consists of a format field and a count of glyph index ranges, followed by an array of records (rangeRecords). Each RangeRecord consists of a start glyph index, an end glyph index, and the Coverage Index associated with the range’s start glyph. Ranges must be in startGlyphID order, and they must be distinct, with no overlapping.

The Coverage Indexes for the first range begin with zero (0) and increase sequentially to (endGlyphId - startGlyphId). For each successive range, the starting Coverage Index is one greater than the ending Coverage Index of the preceding range. Thus, startCoverageIndex for each non-initial range must equal the length of the preceding range (endGlyphID - startGlyphID + 1) added to the startCoverageIndex of the preceding range. This allows for a quick calculation of the Coverage Index for any glyph in any range using the formula: Coverage Index (glyphID) = startCoverageIndex + glyphID - startGlyphID.

Example 6 at the end of this chapter shows a Coverage table that uses format 2 to identify a range of numeral glyphs in a font.

<a id="coverage2"></a>

*CoverageFormat2 table: Range of glyphs*

<a id="coverageFormat_cov2"></a>

<a id="rangeCount"></a>

<a id="rangeRecs"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 2. |
| uint16 | rangeCount | Number of RangeRecords. |
| RangeRecord | rangeRecords[rangeCount] | Array of glyph ranges — ordered by startGlyphID. |

<a id="rangeRec"></a>

*RangeRecord*

<a id="startGlyphId_rngRec"></a>

<a id="endGlyphId_rngRec"></a>

<a id="startCoverageIndex"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | startGlyphID | First glyph ID in the range. |
| uint16 | endGlyphID | Last glyph ID in the range. |
| uint16 | startCoverageIndex | Coverage Index of first glyph ID in range. |

<a id="classDefTbl"></a>

<a id="CDT"></a>

<a id="class-definition-table"></a>

## Class definition table

For efficiency and ease of representation in lookups, a font developer can group glyphs into glyph *classes*. Classes can be used in GSUB and GPOS lookups for various purposes including describing glyph contexts or sets of marks to be processed or ignored.

Consider a substitution action that replaces only the lowercase ascender glyphs in a glyph string. To describe the appropriate context for the substitution more easily, the font developer could divide the font’s lowercase glyphs into two classes, one that contains the ascenders and one that contains the glyphs without ascenders.

A font developer can assign any glyph to any class, each identified with an integer. A class definition table (ClassDef) assigns glyphs into classes, beginning with Class 1, then Class 2, and so on. All glyphs not assigned to a class fall into Class 0. Within a given class definition table, each glyph in the font belongs to exactly one class.

The ClassDef table can have either of two formats: one that assigns a range of consecutive glyph indices to different classes, or one that puts groups of consecutive glyph indices into the same class.

<a id="classDefFormat1"></a>

<a id="class-definition-format-1"></a>

### Class definition format 1

The first class definition format (ClassDefFormat1) specifies a range of consecutive glyph indices and a list of corresponding glyph class values. This table is useful for assigning each glyph to a different class because the glyph indices in each class are not grouped together.

A ClassDefFormat1 table begins with a format identifier. The range of glyph IDs covered by the table is identified by two values: the glyph ID of the first glyph (startGlyphID) and the number of consecutive glyph IDs (including the first one) that will be assigned class values. An array of integers lists the class value assigned to each glyph ID, starting with the class value for startGlyphID and following the same order as the glyph IDs. Any glyph not included in the range of covered glyph IDs is assigned to Class 0.

Example 7 at the end of this chapter uses format 1 to assign class values to the lowercase, x-height, ascender, and descender glyphs in a font.

<a id="classDef1"></a>

*ClassDefFormat1 table: Class array*

<a id="classFormat_cdef1"></a>

<a id="startGlyphId_cdef1"></a>

<a id="glyphCount_cdef1"></a>

<a id="classValueArray_cdef1"></a>

| Type | Name | Description |
| --- | --- | --- |
| uint16 | format | Format identifier — format = 1. |
| uint16 | startGlyphID | First glyph ID assigned to a class. |
| uint16 | glyphCount | Number of elements in the classValues array. |
| uint16 | classValues[glyphCount] | Array of class values — one per glyph ID. |

