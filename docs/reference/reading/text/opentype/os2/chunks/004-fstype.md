# fsType

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](003-os-2-field-details.md) · [Next](005-ysubscriptysize.md)

<a id="fst"></a>

<a id="fstype"></a>

### fsType

<a id="fst_0"></a>

<a id="fst_bit0"></a>

<a id="fst_bit1"></a>

<a id="fst_bit2"></a>

<a id="fst_bit3"></a>

<table>
<tbody>
<tr>
<td>Format:</td>
<td>uint16</td>
</tr>
<tr>
<td>Title:</td>
<td>Type flags.</td>
</tr>
<tr>
<td>Description:</td>
<td>
Indicates font embedding licensing rights for the font. The interpretation of flags is as follows:
<p><a id="fst_0"></a>
<a id="fst_bit0"></a>
<a id="fst_bit1"></a>
<a id="fst_bit2"></a>
<a id="fst_bit3"></a></p><a id="fst_bit8"></a><a id="fst_bit9"></a><table>
<tbody>
<tr>
<th>Bits</th>
<th>Mask</th>
<th>Description</th>
</tr>
<tr>
<td>0 – 3</td>
<td>0x000F</td>
<td>
Usage permissions. Valid fonts must set at most one of bits 1, 2 or 3; bit 0 is permanently reserved and must be zero. Valid values for this sub-field are 0, 2, 4 or 8. The meaning of these values is as follows:
<br/><br/>
<b>0:</b> Installable embedding: the font may be embedded, and may be permanently installed for use on a remote systems, or for use by other users. The user of the remote system acquires the identical rights, obligations and licenses for that font as the original purchaser of the font, and is subject to the same end-user license agreement, copyright, design patent, and/or trademark as was the original purchaser.
<br/><br/>
<b>2:</b> Restricted License embedding: the font must not be modified, embedded or exchanged in any manner without first obtaining explicit permission of the legal owner.
<br/><br/>
<b>4:</b> Preview &amp; Print embedding: the font may be embedded, and may be temporarily loaded on other systems for purposes of viewing or printing the document. Documents containing Preview &amp; Print fonts must be opened “read-only”; no edits may be applied to the document.
<br/><br/>
<b>8:</b> Editable embedding: the font may be embedded, and may be temporarily loaded on other systems. As with Preview &amp; Print embedding, documents containing Editable fonts may be opened for reading. In addition, editing is permitted, including ability to format new text using the embedded font, and changes may be saved.
</td>
</tr>
<tr>
<td>4 – 7</td>
<td>0x00F0</td>
<td>Reserved, must be zero.</td>
</tr>
<tr id="fst_bit8">
<td>8</td>
<td>0x0100</td>
<td>No subsetting: When this bit is set, the font must not be subsetted prior to embedding. Other embedding restrictions specified in bits 0 – 3 and bit 9 also apply.</td>
</tr>
<tr id="fst_bit9">
<td>9</td>
<td>0x0200</td>
<td>Bitmap embedding only: When this bit is set, only bitmaps contained in the font may be embedded. No outline data may be embedded. If there are no bitmaps available in the font, then the font is considered unembeddable and the embedding services will fail. Other embedding restrictions specified in bits 0-3 and 8 also apply.</td>
</tr>
<tr>
<td>10 – 15</td>
<td>0xFC00</td>
<td>Reserved, must be zero.</td>
</tr>
</tbody>
</table>
</td>
</tr>
<tr>
<td>Comments:</td>
<td>
Embeddable fonts may be stored in a document. When a document with embedded fonts is opened on a system that does not have the font installed (the remote system), the embedded font may be loaded for temporary (and in some cases, permanent) use on that system by an embedding-aware application. Embedding licensing rights are granted by the vendor of the font.
<br/><br/>
<em>Applications that implement support for font embedding must not embed fonts which are not licensed to permit embedding. Also, when embedding a font into a document, applications must not modify the embedding permissions and restrictions indicated in this field. In addition, applications loading embedded fonts for temporary use (Preview &amp; Print or Editable embedding) <b>must</b> delete the fonts when the document containing the embedded font is closed.</em>
<br/><br/>
Bits 0 to 3 (the embedding permissions sub-field) are mutually exclusive: fonts should never have more than one of these bits set. Note that, if two or more bits are set, some applications could assume the <em>least-restrictive permission indicated</em>. (See version differences for more discussion.) <em>Caution:</em> Font vendors are responsible to set these bits correctly to obtain the desired application behaviors. For Restricted License embedding to take effect, the Embedding permissions sub-field must have the value 2 (that is, only bit 1 is set).
<br/><br/>
<em>Note:</em> <a data-linktype="external" href="https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6OS2.html">Apple’s TrueType Reference Manual</a> specifies bit 1 of the fsType field, and only bit 1, as having an assigned semantic. This originated from a pre-release draft specification for the OS/2 table. However, the final specification for version 0 of the OS/2 table defined bits 0 to 3. Also, some early font implementations mistakenly used <em>the value 1</em> (bit 0 set), leading to problems of non-interoperability. For this reason, bit 0 was specified as reserved in the final specification for version 0. Bit 0 is permanently reserved, and its use is deprecated.
</td>
</tr>
<tr>
<td>Version differences:</td>
<td><b>Versions 0 to 1:</b> only bits 0 to 3 were assigned. Applications must ignore bits 4 to 15 when reading a version 0 or version 1 table.
<br/><br/>
<b>Versions 0 to 2:</b> The specification for versions 0 to 2 did not specify that bits 0 to 3 must be mutually exclusive. Rather, those specifications stated that, in the event that more than one of bits 0 to 3 are set in a given font, then the <em>least-restrictive permission indicated take precedence</em>. In particular, some fonts using a version 0 to version 2 OS/2 table have both bit 2 and bit 3 set with the intent to indicate both preview/print and edit permissions. Applications are permitted to use this behavior for fonts with a version 0 to version 2 OS/2 table.
<br/><br/>
<b>Versions 3 and later:</b> The specification for version 3, added in OpenType 1.4, introduced the explicit requirement that bits 0 to 3 must be mutually exclusive. 
</td>
</tr>
</tbody>
</table>

<a id="subxs"></a>

<a id="ysubscriptxsize"></a>

### ySubscriptXSize

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Subscript horizontal font size. |
| Description: | The recommended horizontal size in font design units for subscripts for this font. Should be > 0. |
| Comments: | If a font has two recommended sizes for subscripts, e.g., numerics and other, the numeric sizes should be stressed. This size field maps to the em size of the font being used for a subscript. The horizontal font size specifies a font designer’s recommended horizontal size of subscript glyphs associated with this font. If a font does not include all required subscript glyphs for an application, and the application can substitute glyphs by scaling the glyphs of a font or by substituting glyphs from another font, this field specifies the recommended nominal width for those subscript glyphs.    For example, if the em for a font is 2048 units and ySubScriptXSize is set to 205, then the horizontal size for a simulated subscript glyph would be 1/10th the size of the normal glyph.   *Note:* This was implemented in the OS/2 platform as a signed value, even though only values > 0 would be expected. |

