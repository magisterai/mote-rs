# Calculating Checksums

Source: [OpenType font file](https://learn.microsoft.com/en-us/typography/opentype/spec/otff) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/otff).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-organization-of-an-opentype-font.md) · [Next](004-font-tables.md)

<a id="checksums"></a>

<a id="calculating-checksums"></a>

### Calculating Checksums

Table checksums are the unsigned sum of the uint32 units of a given table. In C, the following function can be used to determine a checksum:



```
uint32
CalcTableChecksum(uint32 *Table, uint32 Length)
{
uint32 Sum = 0L;
uint32 *EndPtr = Table+((Length+3) & ~3) / sizeof(uint32);
while (Table < EndPtr)
    Sum += *Table++;
return Sum;
}
```




> *Note:* This function assumes that the length of any table is a multiple of four bytes, or that tables are padded with zero to four-byte aligned offsets. Actual table lengths recorded in the table directory should not include padding, however. To accommodate data with a length that is not a multiple of four, the above algorithm must be modified to treat the data as though it contains zero padding to a length that is a multiple of four.

The 'head' table is a special case in checksum calculations, as it includes a checksumAdjustment field that is calculated and written after the table’s checksum is calculated and written into the table directory entry, necessarily invalidating that checksum value.

When generating font data, to calculate and write the 'head' table checksum and checksumAdjustment field, do the following:

1. Set the checksumAdjustment field to 0.
2. Calculate the checksum for all tables including the 'head' table and enter the value for each table into the corresponding record in the table directory.
3. Calculate the checksum for the entire font.
4. Subtract that value from 0xB1B0AFBA.
5. Store the result in the 'head' table checksumAdjustment field.

An application attempting to verify that the 'head' table has not changed should calculate the checksum for that table assuming that the checksumAdjustment value is zero, rather than the actual value in the font, before comparing the result with the 'head' table record in the table directory.

Within a font collection file (see below), table checksums must reflect the tables as they are in the collection file. The checksumAdjustment field in the 'head' table is not used for collection files and may be set to zero.

<a id="collections"></a>

<a id="font-collections"></a>

## Font Collections

An OpenType Font Collection (either TTC or OTC, formerly known as TrueType Collection) is a means of delivering multiple OpenType font resources in a single file structure. The format for font collections allows font tables that are identical between two or more fonts to be shared. Font collections containing outline glyph data (TrueType, CFF, CFF2 or SVG) are most useful when the fonts to be delivered together share many glyphs in common. By allowing multiple fonts to share glyph sets and other common font tables, font collections can result in a significant saving of file space.

For example, a group of Japanese fonts may each have their own designs for the kana glyphs, but share identical designs for the kanji. With ordinary OpenType font files, the only way to include the common kanji glyphs is to copy their glyph data into each font. Since the kanji represent much more data than the kana, this results in a great deal of wasteful duplication of glyph data. Font collections were defined to solve this problem.

> *Note:*
> Even though the original definition of Font Collection (as part of the TrueType specification) was intended to be used with fonts containing TrueType outlines, and this constraint was maintained in earlier OpenType versions, this is no longer a constraint in OpenType. Font collection files may contain various types of outlines (or a mix of them), regardless of whether or not fonts have layout tables present. For backward compatibility and simplicity, the description of the font collection file structure is using the term “TrueType Collection”, or “TTC”, though it is understood that it is used to identify a generic font collection structure containing any type of outline tables.

> *Note:* An OpenType variable font is functionally equivalent to multiple non-variable fonts. Variable fonts do not need to be contained within a collection file. A collection file can include one or even multiple variable fonts, however, and may even combine variable and non-variable fonts.

<a id="collectionFile"></a>

<a id="the-font-collection-file-structure"></a>

### The Font Collection File Structure

A font collection file consists of a single TTC header table, one or more table directories (each corresponding to a different font resource), and a number of OpenType tables. The TTC header must be located at the beginning of the TTC file.

The TTC file must contain a complete table directory for each font resource. The same TableDirectory format is used for each font resource in a collection file as is used in a non-collection file. The table offsets in all table directories within a TTC file are measured from the beginning of the TTC file.

Each OpenType table in a TTC file is referenced through the table directory of each font which uses that table. Some of the OpenType tables must appear multiple times, once for each font included in the TTC; while other tables may be shared by multiple fonts in the TTC.

As an example, consider a TTC file which combines two Japanese fonts (Font1 and Font2). The fonts have different kana designs (Kana1 and Kana2) but use the same design for kanji. The TTC file contains a single 'glyf' table which includes both designs of kana together with the kanji; both fonts’ table directories point to this 'glyf' table. But each font’s table directory points to a different 'cmap' table, which identifies the glyph set to use. Font1’s 'cmap' table points to the Kana1 region of the 'loca' and 'glyf' tables for kana glyphs, and to the kanji region for the kanji. Font2’s 'cmap' table points to the Kana2 region of the 'loca' and 'glyf' tables for kana glyphs, and to the same kanji region for the kanji.

The tables that should have a unique copy per font are those that are used by the system in identifying the font and its character mapping, including 'cmap', 'name', and OS/2. The tables that should be shared by fonts in the TTC are those that define glyph and instruction data or use glyph indices to access data: 'glyf', 'loca', 'hmtx', 'hdmx', LTSH, 'cvt ', 'fpgm', 'prep', EBLC, EBDT, EBSC, 'maxp', and so on. In practice, any tables which have identical data for two or more fonts may be shared.

> *Note:*
> When building a collection file from separate font files, close attention needs to be paid to the issue of glyph renumbering in a font and the side effects that can result in the 'cmap' table and elsewhere. The fonts to be merged also need to have compatible TrueType instructions; that is, their preprograms, function definitions, and control values cannot conflict.

Collection files containing TrueType glyph outlines should use the file extension .TTC. Collection files containing CFF or CFF2 outlines should use the file extension .OTC.

<a id="ttcHeader"></a>

<a id="ttc-header"></a>

### TTC Header

There are two versions of the TTC header: Version 1.0 has been used for TTC files without digital signatures. Version 2.0 can be used for TTC files with *or* without digital signatures — if there’s no signature, then the last three fields of the version 2.0 header are left null.

If a digital signature is used, the DSIG table for the file must be located at the end of the TTC file, following any other font tables. Signatures in a TTC file are expected to be Format 1 signatures.

The purpose of the TTC header table is to locate the different table directories within a TTC file. The TTC header is located at the beginning of the TTC file (offset = 0). It consists of an identification tag, a version number, a count of the number of OpenType fonts in the file, and an array of offsets to each table directory.

<a id="ttcHeader_1"></a>

*TTCHeader Version 1.0:*

<a id="ttcTag_1"></a>

<a id="majorVersion_1"></a>

<a id="minorVersion_1"></a>

<a id="numFonts_1"></a>

<a id="offsetTable_1"></a>

| Type | Name | Description |
| --- | --- | --- |
| Tag | ttcTag | Font Collection ID string: 'ttcf' (used for fonts with CFF or CFF2 outlines, as well as TrueType outlines) |
| uint16 | majorVersion | Major version of the TTCHeader, = 1. |
| uint16 | minorVersion | Minor version of the TTCHeader, = 0. |
| uint32 | numFonts | Number of fonts in TTC |
| Offset32 | tableDirectoryOffsets[numFonts] | Array of offsets to the TableDirectory for each font from the beginning of the file |

<a id="ttcHeader_2"></a>

*TTCHeader Version 2.0:*

<a id="ttcTag_2"></a>

<a id="majorVersion_2"></a>

<a id="minorVersion_2"></a>

<a id="numFonts_2"></a>

<a id="offsetTable_2"></a>

<a id="dsigTag_2"></a>

<a id="dsigLength_2"></a>

<a id="dsigOffset_2"></a>

| Type | Name | Description |
| --- | --- | --- |
| Tag | ttcTag | Font Collection ID string: 'ttcf' |
| uint16 | majorVersion | Major version of the TTCHeader, = 2. |
| uint16 | minorVersion | Minor version of the TTCHeader, = 0. |
| uint32 | numFonts | Number of fonts in TTC |
| Offset32 | tableDirectoryOffsets[numFonts] | Array of offsets to the TableDirectory for each font from the beginning of the file |
| uint32 | dsigTag | Tag indicating that a DSIG table exists, 0x44534947 ('DSIG') (null if no signature) |
| uint32 | dsigLength | The length (in bytes) of the DSIG table (null if no signature) |
| uint32 | dsigOffset | The offset (in bytes) of the DSIG table from the beginning of the TTC file (null if no signature) |

