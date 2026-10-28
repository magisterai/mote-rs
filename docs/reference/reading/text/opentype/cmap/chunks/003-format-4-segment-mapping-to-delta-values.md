# Format 4: Segment mapping to delta values

Source: [Character to glyph index mapping](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/cmap).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-macintosh-platform-platform-id-1.md) · [Next](004-format-6-trimmed-table-mapping.md)

<a id="format4"></a>

<a id="format-4-segment-mapping-to-delta-values"></a>

### Format 4: Segment mapping to delta values

This is the standard character-to-glyph-index mapping subtable for fonts that support only Unicode Basic Multilingual Plane characters (U+0000 to U+FFFF).

> *Note:* To support Unicode supplementary-plane characters, [format 12](004-format-6-trimmed-table-mapping.md#format-12-segmented-coverage) should be used.

This format is used when the character codes for the characters represented by a font fall into several contiguous ranges, possibly with holes in some or all of the ranges (that is, some of the codes in a range might not have a representation in the font). The format-dependent data is divided into three parts, which must occur in the following order:

1. A four-word header gives parameters for an optimized search of the segment list.
2. Four parallel arrays describe the segments (one segment for each contiguous range of codes).
3. A variable-length array of glyph IDs (unsigned words).

<a id="subtable_4"></a>

*'cmap' Subtable Format 4:*

<table>
<thead>
<tr>
<th>Type</th>
<th>Name</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr>
<td>uint16</td>
<td>format</td>
<td>Format number is set to 4.</td>
</tr>
<tr>
<td>uint16</td>
<td>length</td>
<td>This is the length in bytes of the subtable.</td>
</tr>
<tr>
<td>uint16</td>
<td>language</td>
<td>For requirements on use of the language field, see “<a data-linktype="self-bookmark" href="002-macintosh-platform-platform-id-1.md#language">Use of the language field in 'cmap' subtables</a>” in this document.</td>
</tr>
<tr>
<td>uint16</td>
<td>segCountX2</td>
<td>2 × segCount.</td>
</tr>
<tr>
<td>uint16</td>
<td>searchRange</td>
<td>Maximum power of 2 less than or equal to segCount, times 2<br/>((2**floor(log<sub>2</sub>(segCount))) * 2, where “**” is an exponentiation operator)</td>
</tr>
<tr>
<td>uint16</td>
<td>entrySelector</td>
<td>Log<sub>2</sub> of the maximum power of 2 less than or equal to segCount<br/>(log<sub>2</sub>(searchRange/2), which is equal to floor(log<sub>2</sub>(segCount)))</td>
</tr>
<tr>
<td>uint16</td>
<td>rangeShift</td>
<td>segCount times 2, minus searchRange<br/>((segCount * 2) - searchRange)</td>
</tr>
<tr>
<td>uint16</td>
<td>endCode[segCount]</td>
<td>End characterCode for each segment, last=0xFFFF.</td>
</tr>
<tr>
<td>uint16</td>
<td>reservedPad</td>
<td>Set to 0.</td>
</tr>
<tr>
<td>uint16</td>
<td>startCode[segCount]</td>
<td>Start character code for each segment.</td>
</tr>
<tr>
<td>int16</td>
<td>idDelta[segCount]</td>
<td>Delta for all character codes in segment.</td>
</tr>
<tr>
<td>uint16</td>
<td>idRangeOffset[segCount]</td>
<td>Offsets into glyphIdArray or 0</td>
</tr>
<tr>
<td>uint16</td>
<td>glyphIdArray[ ]</td>
<td>Glyph index array (arbitrary length)</td>
</tr>
</tbody>
</table>

The number of segments is specified by segCount, which is not given directly in the header but is readily derived from segCountX2. All of the other header parameters are derived from it. The searchRange value is twice the largest power of 2 that is less than or equal to segCount. For example, if segCount=39, we have the following:

<table>
<tbody><tr><td>segCountX2</td><td>78</td><td> </td><td> </td><td> </td><td> </td><td> </td></tr>
<tr><td>searchRange</td><td>64 (= 2 × (largest power of 2 &lt;=39))</td><td> </td><td> </td><td> </td><td> </td><td> </td></tr>
<tr><td>entrySelector</td><td>5 (= log<sub>2</sub>32)</td><td> </td><td> </td><td> </td><td> </td><td> </td></tr>
<tr><td>rangeShift</td><td>14 (= 2 × 39 - 64)</td><td> </td><td> </td><td> </td><td> </td><td> </td></tr>
</tbody></table>

To assist in quick binary searches, the searchRange, entrySelector and rangeShift fields are included as parameters that can be used in configuring search algorithms. In particular, binary search is optimal when the number of entries is a power of two. The searchRange field provides the largest number of items that can be searched with that constraint (maximum power of two). The rangeShift field provides the remaining number of items that would also need to be searched. The entrySelector field indicates the maximum number of levels into the binary tree will need to be entered.

In early implementations on devices with limited hardware capabilities, optimizations provided by the searchRange, entrySelector and rangeShift fields were of high importance. They have less importance on modern devices but could still be used in some implementations. However, incorrect values could potentially be used as an attack vector against some implementations. Since these values can be derived from the segCountX2 field when the file is parsed, it is strongly recommended that parsing implementations not rely on the searchRange, entrySelector and rangeShift fields in the font but derive them independently from segCountX2. Font files, however, should continue to provide valid values for these fields to maintain compatibility with all existing implementations.

Each segment is described by a startCode and endCode, along with an idDelta and an idRangeOffset, which are used for mapping the character codes in the segment. The segments are sorted in order of increasing endCode values, and the segment values are specified in four parallel arrays. You search for the first endCode that is greater than or equal to the character code you want to map. If the corresponding startCode is less than or equal to the character code, then you use the corresponding idDelta and idRangeOffset to map the character code to a glyph index (otherwise, the missingGlyph is returned). For the search to terminate, the final startCode and endCode values must be 0xFFFF. This segment need not contain any valid mappings. (It can just map the single character code 0xFFFF to missingGlyph). However, the segment must be present.

If the idRangeOffset value for the segment is not 0, the mapping of character codes relies on glyphIdArray. The character code offset from startCode is added to the idRangeOffset value. This sum is used as an offset from the current location within idRangeOffset itself to index out the correct glyphIdArray value. This obscure indexing trick works because glyphIdArray immediately follows idRangeOffset in the font file. The C expression that yields the glyph index is:



```
glyphId = *(idRangeOffset[i]/2
            + (c - startCode[i])
            + &idRangeOffset[i])
```




The value *c* is the character code in question, and *i* is the segment index in which *c* appears. If the value obtained from the indexing operation is not 0 (which indicates missingGlyph), idDelta[i] is added to it to get the glyph index. The idDelta arithmetic is modulo 65536.

If the idRangeOffset is 0, the idDelta value is added directly to the character code offset (i.e. `idDelta[i] + c`) to get the corresponding glyph index. Again, the idDelta arithmetic is modulo 65536. If the result after adding `idDelta[i] + c` is less than zero, add 65536 to obtain a valid glyph ID.

As an example, the variant part of the table to map characters 10-20, 30-90, and 153-480 onto a contiguous range of glyph indices may look like this:

|  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- |
| segCountX2: | 8 |  |  |  |  |  |
| searchRange: | 8 |  |  |  |  |  |
| entrySelector: | 2 |  |  |  |  |  |
| rangeShift: | 0 |  |  |  |  |  |
| endCode: | 20 90 480 0xffff |  |  |  |  |  |
| reservedPad: | 0 |  |  |  |  |  |
| startCode: | 10 30 153 0xffff |  |  |  |  |  |
| idDelta: | -9 -18 -80 1 |  |  |  |  |  |
| idRangeOffset: | 0 0 0 0 |  |  |  |  |  |

This table yields the following mappings:

> 10 ⇒ 10 - 9 = 1  
> 20 ⇒ 20 - 9 = 11  
> 30 ⇒ 30 - 18 = 12  
> 90 ⇒ 90 - 18 = 72  
> 153 ⇒ 153 - 80 = 73  
> 480 ⇒ 480 - 80 = 400  
> 0xffff ⇒ 0

Note that the delta values could be reworked so as to reorder the segments.

