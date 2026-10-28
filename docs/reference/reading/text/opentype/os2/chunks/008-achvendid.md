# achVendID

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](007-continuation-7.md) · [Next](009-usfirstcharindex.md)

<a id="vendid"></a>

<a id="achvendid"></a>

### achVendID

|  |  |
| --- | --- |
| Format: | Tag |
| Title: | Font Vendor Identification |
| Description: | The four character identifier for the vendor of the given typeface. |
| Comments: | This is not the royalty owner of the original artwork. This is the company responsible for the marketing and distribution of the typeface that is being classified. For example, there can be multiple vendors of ITC Zapf Dingbats, with some vendors providing differentiating benefits in their fonts (more kern pairs, unregularized data, hand hinted, etc.). This identifier will allow for the correct vendor’s type to be used over another, possibly inferior, font file.    A registry of vendor IDs is maintained by Microsoft. Registered IDs must be unique to a single vendor. Non-registered IDs may also be used, but are discouraged: vendors are strongly encouraged to register an ID to ensure that there are no conflicts between different vendors in use of a given ID, and that customers are able to find vendor contact information for a given font. This field can also be left blank (set to null, or a tag comprised of four space characters).    All vendor IDs use the Tag data type, which is equivalent to a four-character string composed of a limited set of ASCII characters. For details regarding the Tag data type, see [Data Types](../../otff/chunks/001-filenames.md#data-types). By convention, only registered tags should be comprised of only uppercase letters (or space).    For a list of registered Vendor IDs, or for details on registering a vendor ID or updating vendor information, see [Registered typography vendors](https://www.microsoft.com/typography/links/vendorlist.aspx). |

<a id="fss"></a>

<a id="fsselection"></a>

### fsSelection

<table>
<tbody>
<tr>
<td>Format:</td>
<td>uint16</td>
</tr>
<tr>
<td>Title:</td>
<td>Font selection flags.</td>
</tr>
<tr>
<td>Description:</td>
<td>
Contains information concerning the nature of the font patterns, as follows:
<a id="fss_bit0"></a><a id="fss_bit1"></a><a id="fss_bit2"></a><a id="fss_bit3"></a><a id="fss_bit4"></a><a id="fss_bit5"></a><a id="fss_bit6"></a><a id="fss_bit7"></a><a id="fss_bit8"></a><a id="fss_bit9"></a><table>
<tbody>
<tr>
<th>Bit #</th>
<th>macStyle bit</th>
<th>C definition</th>
<th>Description</th>
</tr>
<tr id="fss_bit0">
<td>0</td>
<td>bit 1</td>
<td>ITALIC</td>
<td>Font contains italic or oblique glyphs, otherwise they are upright.</td>
</tr>
<tr id="fss_bit1">
<td>1</td>
<td> </td>
<td>UNDERSCORE</td>
<td>Glyphs are underscored.</td>
</tr>
<tr id="fss_bit2">
<td>2</td>
<td> </td>
<td>NEGATIVE</td>
<td>Glyphs have their foreground and background reversed.</td>
</tr>
<tr id="fss_bit3">
<td>3</td>
<td> </td>
<td>OUTLINED</td>
<td>Outline (hollow) glyphs, otherwise they are solid.</td>
</tr>
<tr id="fss_bit4">
<td>4</td>
<td> </td>
<td>STRIKEOUT</td>
<td>Glyphs are overstruck.</td>
</tr>
<tr id="fss_bit5">
<td>5</td>
<td>bit 0</td>
<td>BOLD</td>
<td>Glyphs are emboldened.</td>
</tr>
<tr id="fss_bit6">
<td>6</td>
<td> </td>
<td>REGULAR</td>
<td>Glyphs are in the standard weight/style for the font.</td>
</tr>
<tr id="fss_bit7">
<td>7</td>
<td> </td>
<td>USE_TYPO_METRICS</td>
<td>If set, it is strongly recommended that applications use OS/2.sTypoAscender - OS/2.sTypoDescender + OS/2.sTypoLineGap as the default line spacing for this font.</td>
</tr>
<tr id="fss_bit8">
<td>8</td>
<td> </td>
<td>WWS</td>
<td>The font has 'name' table strings consistent with a weight/width/slope family without requiring use of name IDs 21 and 22. (See a more detailed description below.)</td>
</tr>
<tr id="fss_bit9">
<td>9</td>
<td> </td>
<td>OBLIQUE</td>
<td>Font contains oblique glyphs.</td>
</tr>
<tr>
<td>10–15</td>
<td> </td>
<td>Reserved</td>
<td>Reserved; set to 0.</td>
</tr>
</tbody>
</table>
</td>
</tr>
<tr>
<td>Comments:</td>
<td>
All undefined bits must be zero.
<br/><br/>
The fsSelection field was originally created for compatibility with the OS/2 platform. Some style description bits, such as bit 0, are used by modern applications, while other style description bits are not.
<br/><br/>
<b>Bit 0:</b> The setting of bits 0 must match the setting of bit 1 in the <a data-linktype="relative-path" href="../../head/chunks/001-front-matter.md#macstyle">macStyle</a> field of the 'head' table.
<br/><br/>
<b>Bits 1 – 4:</b> Bits 1 – 4 are rarely used bits that indicate the font is primarily a decorative or special purpose font.
<br/><br/>
<b>Bit 5:</b> The setting of bit 5 must match the settings of bit 0 in the <a data-linktype="relative-path" href="../../head/chunks/001-front-matter.md#macstyle">macStyle</a> field of the 'head' table.
<br/><br/>
<b>Bit 6:</b> 
Bit 6 is not used in widely used in modern applications. If bit 6 is set, then bits 0 and 5 must be clear, else the behavior is undefined. Note that, if both bit 0 and bit 5 are clear, that does not give any indication as to whether or not bit 6 will be clear. For example, Arial Light is not the regular style of Arial and would have all bits cleared. In extended font families, bit 6 does not need to be set for fonts other than the regular style that, nonetheless, use "Regular" for name ID 2. (See <a data-linktype="relative-path" href="../../name/chunks/002-name-ids.md#name-ids">Name IDs</a> for more information.)
<br/><br/>
<b>Bit 7:</b>
<br/>
Bit 7 was defined in version 4 and is used in many modern applications. For new fonts, vendors are encouraged to use a version 4 or later OS/2 table and to have bit 7 set.
<br/><br/>
If a font was created with an earlier version of the OS/2 table and is updated to the current version of the OS/2 table, then setting bit 7 could create potential for reflow of existing documents which use the fonts. To minimize such risk, the bit would be set only if using the OS/2.usWin* metrics for line height would yield significantly inferior results than using the OS/2.sTypo* values.
<br/><br/>
<b>Bit 8:</b>
<br/>
If bit 8 is set, then <a data-linktype="relative-path" href="../../name/README.md">'name' table</a> strings for family and subfamily are provided that are consistent with a weight/width/slope (WWS) family model without requiring the use of name IDs 21 or 22.
<br/><br/>
Bit 8 and name IDs 21 and 22 were introduced for use by applications that assume a WWS family model, given that there exist fonts with style variants that do not fit into one of the three attributes, weight, width or slope.
<br/><br/>
Many typographic families include faces that differ only in one or more of these three attributes. In such cases, name IDs 1 and 2 or IDs 16 and 17 will not present challenges for software that assumes a WWS model. But some typographic families include faces that differ in some <i>other</i> attributes. For example, a family might include variations for “handwriting”, “caption”, “display”, “optical size”, etc. In such cases, a font could incorporate that attribute into a typographic subfamily name (ID 17 string), whereas applications that assume a WWS model would require that attribute to be incorporated into a family name (ID 1 or ID 16).
<br/><br/>
IF a face differs within its family only in terms of weight, width or slope attributes, then bit 8 should be set. This applies even if the face is part of a large family that includes <i>other</i> faces distinguished by other attributes. Setting bit 8 serves as an indicator to apps that assume the WWS model that name IDs 1 and 2 or IDs 16 and 17 are consistent with the WWS model and no further treatment is required, including checking to see whether name IDs 21 and 22 are present. Hence, when bit 8 is set, name IDs 21 and 22 are never required and should not be included.
<br/><br/>
If a face differs within its family from Regular by attributes other than weight, width or slope, then bit 8 should *not* be set, and name IDs 21 and 22 should be included. By having bit 8 not set, that serves as an indicator to apps that assume a WWS model that they should look for name IDs 21 and 22, or that some other treatment of names could be required.
<br/><br/>
The <a data-linktype="relative-path" href="https://learn.microsoft.com/en-us/typography/opentype/spec/stat">STAT</a> table was introduced in a later version of OpenType, after bit 8 and name IDs 21 and 22 were defined. The STAT table was introduced, in part, to solve the same kind of requirement but in a more general way that could be useful for applications assuming other family models. If a face has attributes other than weight, width or slope and does not have name IDs 21 and 22, some applications that assume a WWS model could be able to support that font using information in the STAT table. However, even if a STAT table is present (as is required for variable fonts), the recommendations above regarding bit 8 and name IDs 21 and 22 should still be followed.
<br/><br/>
In this context, “typographic family” is the Microsoft Unicode string for name ID 16, if present, else the Microsoft Unicode string for name ID 1; “weight” is OS/2.usWeightClass; “width” is OS/2.usWidthClass; “slope” is OS/2.fsSelection bit 0 (ITALIC) and bit 9 (OBLIQUE).
<br/><br/>
<b>Bit 9:</b>
<br/>
If bit 9 is set, then this font is to be considered an “oblique” style by processes which make a distinction between oblique and italic styles, such as Cascading Style Sheets font matching. For example, a font created by algorithmically slanting an upright face will set this bit.
<br/><br/>
If a font has a version 4 or later OS/2 table and this bit is not set, then this font is not to be considered an “oblique” style. For example, a font that has a classic italic design will not set this bit.
<br/><br/>
This bit, unlike the ITALIC bit (bit 0), is not related to style-linking in applications that assume a four-member font-family model comprised of regular, italic, bold and bold italic. It may be set or unset independently of the ITALIC bit. In most cases, if OBLIQUE is set, then ITALIC will also be set, though this is not required.
<br/><br/>
<b>Bit 15:</b> Bit 15 is permanently reserved. It has been used in some legacy implementations and could result in special behavior in some implementations. Use of this bit is deprecated.
</td>
</tr>
<tr>
<td>Version differences:</td>
<td>
<b>Versions 0 to 3:</b> Only bit 0 (italic) to bit 6 (regular) are assigned. Bits 7 to 15 are reserved and must be set to 0. Applications should ignore bits 7 to 15 in a font that has a version 0 to version 3 OS/2 table.
<br/><br/>
<b>Version 4 to 5:</b> Bits 7 to 9 were defined in version 4 (OpenType 1.5). Bits 10 to 15 are reserved and must be set to 0. Applications should ignore bits 10 to 15 in a font that has a version 4 or version 5 OS/2 table.
</td>
</tr>
</tbody>
</table>

