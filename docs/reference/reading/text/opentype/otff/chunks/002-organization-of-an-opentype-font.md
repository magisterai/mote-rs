# Organization of an OpenType Font

Source: [OpenType font file](https://learn.microsoft.com/en-us/typography/opentype/spec/otff) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/otff).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-filenames.md) · [Next](003-calculating-checksums.md)

<a id="otFontOrganization"></a>

<a id="organization-of-an-opentype-font"></a>

## Organization of an OpenType Font

<a id="tabledirectory"></a>

<a id="offsetTable"></a>

<a id="table-directory"></a>

### Table Directory

A key characteristic of the OpenType format is the TrueType sfnt “wrapper”, which provides organization for a collection of tables in a general and extensible manner.

The OpenType font starts with the table directory, which is a directory of the top-level tables in a font. If the font file contains only one font, the table directory will begin at byte 0 of the file. If the font file is an OpenType Font Collection file (see below), the beginning point of the table directory for each font is indicated in the TTCHeader.

*TableDirectory:*

<a id="sfntVersion"></a>

<a id="numTables"></a>

<a id="searchRange"></a>

<a id="entrySelector"></a>

<a id="rangeShift"></a>

<a id="tableRecords"></a>

<table>
<tbody>
<tr>
<th>Type</th>
<th>Name</th>
<th>Description</th>
</tr>
<tr id="sfntVersion">
<td>uint32</td>
<td>sfntVersion</td>
<td>0x00010000 or 0x4F54544F ('OTTO') — see below.</td>
</tr>
<tr id="numTables">
<td>uint16</td>
<td>numTables</td>
<td>Number of tables.</td>
</tr>
<tr id="searchRange">
<td>uint16</td>
<td>searchRange</td>
<td>Maximum power of 2 less than or equal to numTables, times 16 ((2**floor(log2(numTables))) * 16, where “**” is an exponentiation operator).</td>
</tr>
<tr id="entrySelector">
<td>uint16</td>
<td>entrySelector</td>
<td>Log<sub>2</sub> of the maximum power of 2 less than or equal to numTables (log<sub>2</sub>(searchRange/16), which is equal to floor(log<sub>2</sub>(numTables))).</td>
</tr>
<tr id="rangeShift">
<td>uint16</td>
<td>rangeShift</td>
<td>numTables times 16, minus searchRange ((numTables * 16) - searchRange).</td>
</tr>
<tr id="tableRecords">
<td>TableRecord</td>
<td>tableRecords[numTables]</td>
<td>Table records array—one for each top-level table in the font.</td>
</tr>
</tbody>
</table>

> *Note:* In the preceding table, the symbol “\*\*” is an exponentiation operator, as used in several programming languages.

OpenType fonts that contain TrueType outlines should use the value of 0x00010000 for sfntVersion. OpenType fonts containing CFF data (version 1 or 2) should use 0x4F54544F ('OTTO', when re-interpreted as a Tag) for sfntVersion.

> *Note:* [Apple's TrueType Reference Manual](https://developer.apple.com/fonts/TrueType-Reference-Manual/) allows for 'true' and 'typ1' for sfntVersion. These version tags should not be used for OpenType fonts.

The table directory format allows for a large number of tables. To assist in quick binary searches, the searchRange, entrySelector and rangeShift fields are included as parameters that may be used in configuring search algorithms. Binary search is optimal when the number of entries is a power of two. The searchRange field provides the largest number of items that can be searched with that constraint (maximum power of two). The rangeShift field provides the remaining number of items that would also need to be searched. The entrySelector field indicates the maximum number of levels into the binary tree will need to be entered. Values are multiplied by 16, which is the size of each TableRecord.

In early implementations on devices with limited hardware capabilities, optimizations provided by the searchRange, entrySelector and rangeShift fields were of high importance. They have less importance on modern devices, but may still be used in some implementations. However, incorrect values could potentially be used as an attack vector against some implementations. Since these values can be derived from the numTables field when the file is parsed, it is strongly recommended that parsing implementations not rely on the searchRange, entrySelector and rangeShift fields in the font but derive them independently from numTables. Font files, however, should continue to provide valid values for these fields to maintain compatibility with all existing implementations.

The table directory ends with the tableRecords array. The TableRecord format is as follows.

<a id="tableRecord"></a>

*TableRecord:*

<a id="tblrec_tag"></a>

<a id="tblrec_checkSum"></a>

<a id="tblrec_offset"></a>

<a id="tblrec_length"></a>

| Type | Name | Description |
| --- | --- | --- |
| Tag | tableTag | Table identifier. |
| uint32 | checksum | Checksum for this table. |
| Offset32 | offset | Offset from beginning of font file. |
| uint32 | length | Length of this table. |

Table tags are the names given to tables in the OpenType font file. The tableRecords array makes it possible for a given font to contain only those tables it actually needs. As a result, there is no standard value for numTables. The records in the array must be sorted in ascending order by tag.

For requirements of Tag values, see [Data Types](001-filenames.md#dataTypes), above. Several tags and their associated table formats are defined in this specification. For table tags defined in this specification, a font resource should have at most one table record using a given tag. If a font resource does contain more than one table of a given type, behaviour is unpredictable: apps or platforms may select one of the tables arbitrarily, or may reject the font as invalid.

Platform vendors may define additional tables and associated tags to provide platform-specific functionality See, for example, Apple’s [TrueType Reference Manual](https://developer.apple.com/fonts/TrueType-Reference-Manual/) which defines various tables with associated tags not defined in OpenType. Some font development tools may also define special tables. Fonts that contain such additional tables can still qualify as OpenType fonts if they satisfy the requirements of this specification. For custom tables defined outside this specification, an external specification of such a table may permit multiple tables of that type within a single font resource.

> *Note:* Vendors should notify Microsoft when they define custom tags to ensure forward compatibility in the event of future expansion of OpenType.

> *Note:* [Apple’s specification](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6.html) stipulates that tag names consisting of all lower case letters are reserved for Apple’s use.

All tables must begin on four-byte boundaries, and any remaining space between tables must be padded with zeros. The length of each table should be recorded in the table record with the actual length of data, not the padded length.

> Note: The requirement for four-byte alignment applies to top-level tables only and does not extend to sub-table offsets, records, or fields within tables or records.

Some tables have an internal structure with subtables located at specified offsets, and as a result, it is possible to construct a font with data for different tables interleaved. In general, tables should be arranged contiguously without overlapping the ranges of distinct tables. In any case, however, table lengths measure a contiguous range of bytes that encompasses all of the data for a table. This applies to any subtables as well as to top-level tables.

