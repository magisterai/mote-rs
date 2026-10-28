# yStrikeoutSize

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-ysubscriptysize.md) · [Next](007-continuation-7.md)

<a id="ss"></a>

<a id="ystrikeoutsize"></a>

### yStrikeoutSize

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Strikeout size. |
| Description: | Thickness of the strikeout stroke in font design units. Should be > 0. |
| Comments: | This field should normally be the thickness of the em dash for the current font, and should also match the underline thickness, which is specified in the ['post'](../../post/chunks/001-overview.md#underlineThickness) table.  *Note:* This was implemented in the OS/2 platform as a signed value, even though only values > 0 would be expected. |

<a id="sp"></a>

<a id="ystrikeoutposition"></a>

### yStrikeoutPosition

|  |  |
| --- | --- |
| Format: | FWORD |
| Units: | Font design units |
| Title: | Strikeout position. |
| Description: | The position of the top of the strikeout stroke relative to the baseline in font design units. |
| Comments: | Positive values represent distances above the baseline; negative values represent distances below the baseline. Aligning the strikeout position with the em dash is suggested. Note, however, that the strikeout position should not interfere with the recognition of standard characters, and therefore should not line up with crossbars in the font. |

<a id="fc"></a>

<a id="sfamilyclass"></a>

### sFamilyClass

|  |  |
| --- | --- |
| Format: | int16 |
| Title: | Font-family class and subclass. |
| Description: | This field provides a classification of font-family design. |
| Comments: | The family class and subclass are assigned to a font based on the family design. This field is intended for use in selecting an alternate font family when the requested font is not available.   The class and subclass provide a two-level classification. The high byte of this field contains the family class, while the low byte contains the family subclass. The interpretation of subclass values depends on the class value.   Registered class and subclass values were originally defined by IBM. For details on registered values, see [IBM Font Family Classifications](https://learn.microsoft.com/en-us/typography/opentype/spec/ibmfc). |

<a id="pan"></a>

<a id="panose"></a>

### panose

<table>
<tbody>
<tr>
<td>Format:</td>
<td>uint8[10]</td>
</tr>
<tr>
<td>Title:</td>
<td>PANOSE classification number</td>
</tr>
<tr>
<td>International:</td>
<td>Additional specifications are required for PANOSE to classify non-Latin character sets.</td>
</tr>
<tr>
<td>Description:</td>
<td>This 10-byte array of numbers is used to describe the visual characteristics of a given typeface. These characteristics are then used to associate the font with other fonts of similar appearance having different names. The variables for each digit are listed below.</td>
</tr>
<tr>
<td>Comments:</td>
<td>The PANOSE definition contains ten bytes, each of which can have multiple possible values. The PANOSE values are fully described in the <a data-linktype="external" href="http://www.panose.com">PANOSE Classification Metrics Guide</a>. Note that the first byte is used for a high-level classification, “Family Kind”, and that the interpretation of the remaining bytes is contingent on the value of the first byte. For example, if the Family Kind value is 2 (Latin Text), then the next byte specifies “Serif Style”; but if the Family Kind value is 3 (Latin Hand Written), then the next byte specifies “Tool Kind”. Some applications might support only certain Family Kind values. The following table gives the interpretation for the panose array when the Family Kind is Latin Text:
<table>
<tbody>
<tr>
<th>Type</th>
<th>Name</th>
</tr>
<tr>
<td>uint8</td>
<td>bFamilyType</td>
</tr>
<tr>
<td>uint8</td>
<td>bSerifStyle</td>
</tr>
<tr>
<td>uint8</td>
<td>bWeight</td>
</tr>
<tr>
<td>uint8</td>
<td>bProportion</td>
</tr>
<tr>
<td>uint8</td>
<td>bContrast</td>
</tr>
<tr>
<td>uint8</td>
<td>bStrokeVariation</td>
</tr>
<tr>
<td>uint8</td>
<td>bArmStyle</td>
</tr>
<tr>
<td>uint8</td>
<td>bLetterform</td>
</tr>
<tr>
<td>uint8</td>
<td>bMidline</td>
</tr>
<tr>
<td>uint8</td>
<td>bXHeight</td>
</tr>
</tbody>
</table>
<p>Some applications can use the PANOSE values for font selection, to select a font matching certain parameters. For example, Proportion (for Family Kind = Latin Text) might be used to determine if a font is monospaced; or Serif Style might be used to determine if a font falls into generic <em>serif</em> or <em>sans serif</em> classes. Some applications will use Family Kind = 5 (Latin Symbol) to identify symbol fonts, which might affect font selection or fallback behaviors. There are no requirements for how applications should use the PANOSE values.</p>
<p>In a variable font that uses OpenType Font Variation mechanisms, there is no way to represent different PANOSE values for different instances supported by the font. The PANOSE values can be set based on the default instance.</p>
</td>
</tr>
<tr>
<td>Version differences:</td>
<td>Early versions of this specification provided more details regarding PANOSE values. However, the external specification cited above is the normative source and should be referred to for such details.</td>
</tr>
</tbody>
</table>

<a id="ur"></a>

<a id="ulunicoderange"></a>

### ulUnicodeRange\*

ulUnicodeRange1 (bits 0 – 31)  
ulUnicodeRange2 (bits 32 – 63)  
ulUnicodeRange3 (bits 64 – 95)  
ulUnicodeRange4 (bits 96 – 127)

  

