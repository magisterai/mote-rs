# usWinDescent

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](009-usfirstcharindex.md) · [Next](011-sxheight.md)

<a id="wd"></a>

<a id="uswindescent"></a>

### usWinDescent

|  |  |
| --- | --- |
| Format: | UFWORD |
| Units: | Font design units |
| Description: | The “Windows descender” metric. This should be used to specify the vertical extent below the baseline for a clipping region.    This is similar to the [sTypoDescender](009-usfirstcharindex.md#std) field, and also to the [descender](../../hhea/chunks/001-hhea-table-and-opentype-font-variations.md#descender) field in the 'hhea' table. There are important differences between these, however. Some of these differences are described below. In addition, the usWinDescent value treats distances below the baseline as positive values; thus, usWinDescent is usually a positive value, while sTypoDescender and hhea.descender are usually negative.    In the Windows GDI implementation, the usWinDescent and usWinAscent values have been used to determine the size of the bitmap surface in the TrueType rasterizer. Windows GDI will clip any portion of a TrueType glyph outline that appears below (-1 × usWinDescent). If any clipping is unacceptable, then the value should be set greater than or equal to (-yMin).    *Note:* This pertains to the default position of glyphs, not their final position in layout after data from the GPOS or 'kern' table has been applied. Also, this clipping behavior interacts with the [VDMX table](https://learn.microsoft.com/en-us/typography/opentype/spec/vdmx): if a VDMX table is present and there is data for the current device aspect ratio and rasterization size, then the VDMX data will supersede the usWinAscent and usWinDescent values.    Some legacy applications use the usWinAscent and usWinDescent values to determine default line spacing. This is strongly discouraged. The sTypo\* fields should be used for this purpose.    Note that some applications use either the usWin\* values or the sTypo\* values to determine default line spacing, depending on whether the USE\_TYPO\_METRICS flag (bit 7) of the fsSelection field is set. This could be useful to provide compatibility with legacy documents using older fonts, while also providing better and more-portable layout using newer fonts. See [fsSelection](008-achvendid.md#fss) for additional details.    Applications that use the sTypo\* fields for default line spacing can use the usWin\* values to determine the size of a clipping region. Some applications use a clipping region for editing scenarios to determine what portion of the display surface to re-draw when text is edited, or how large a selection rectangle to draw when text is selected. This is an appropriate use for the usWin\* values.    Early versions of this specification suggested that the usWinDescent value be computed as -yMin for all characters in the Windows “ANSI” character set. For new fonts, the value should be determined based on the primary languages the font is designed to support, and should take into consideration additional vertical extent that could be required to accommodate glyphs with low descenders or mark positioning. |

<a id="cpr"></a>

<a id="ulcodepagerange"></a>

### ulCodePageRange\*

ulCodePageRange1 (bits 0 – 31)  
ulCodePageRange2 (bits 32 – 63)

  

<table>
<tbody>
<tr>
<td>Format:</td>
<td>uint32[2] (64 bits total)</td>
</tr>
<tr>
<td>Title:</td>
<td>Code Page Character Range</td>
</tr>
<tr>
<td>Description:</td>
<td>This field is used to specify the code pages encompassed by the font file in the 'cmap' subtable for platform 3, encoding ID 1 (Windows platform, Unicode BMP). If the font file is encoding ID 0, then the Symbol Character Set bit should be set.
<br/><br/>
If a given bit is set (1), then the code page is considered functional. If the bit is clear (0) then the code page is not considered functional. Each of the bits is treated as an independent flag and the bits can be set in any combination. The determination of “functional” is left up to the font designer, although character set selection should attempt to be functional by code pages if at all possible.
<br/><br/>
Symbol character sets have a special meaning. If the symbol bit (31) is set, and the font file contains a 'cmap' subtable for platform of 3 and encoding ID of 1, then all of the characters in the Unicode range 0xF000 - 0xF0FF (inclusive) will be used to enumerate the symbol character set. If the bit is not set, any characters present in that range will not be enumerated as a symbol character set.
<br/><br/>
All reserved fields must be zero. Each uint32 is in Big-Endian form.  
<table>
<tbody>
<tr>
<th>Bit</th>
<th>Code Page</th>
<th>Description</th>
</tr>
<tr>
<td>0</td>
<td>1252</td>
<td>Latin 1</td>
</tr>
<tr>
<td>1</td>
<td>1250</td>
<td>Latin 2: Eastern Europe</td>
</tr>
<tr>
<td>2</td>
<td>1251</td>
<td>Cyrillic</td>
</tr>
<tr>
<td>3</td>
<td>1253</td>
<td>Greek</td>
</tr>
<tr>
<td>4</td>
<td>1254</td>
<td>Turkish</td>
</tr>
<tr>
<td>5</td>
<td>1255</td>
<td>Hebrew</td>
</tr>
<tr>
<td>6</td>
<td>1256</td>
<td>Arabic</td>
</tr>
<tr>
<td>7</td>
<td>1257</td>
<td>Windows Baltic</td>
</tr>
<tr>
<td>8</td>
<td>1258</td>
<td>Vietnamese</td>
</tr>
<tr>
<td>9 – 15</td>
<td></td>
<td>Reserved for Alternate ANSI</td>
</tr>
<tr>
<td>16</td>
<td>874</td>
<td>Thai</td>
</tr>
<tr>
<td>17</td>
<td>932</td>
<td>JIS/Japan</td>
</tr>
<tr>
<td>18</td>
<td>936</td>
<td>Chinese: Simplified chars—PRC and Singapore</td>
</tr>
<tr>
<td>19</td>
<td>949</td>
<td>Korean Wansung</td>
</tr>
<tr>
<td>20</td>
<td>950</td>
<td>Chinese: Traditional chars—Taiwan and Hong Kong SAR</td>
</tr>
<tr>
<td>21</td>
<td>1361</td>
<td>Korean Johab</td>
</tr>
<tr>
<td>22 – 28</td>
<td></td>
<td>Reserved for Alternate ANSI or OEM</td>
</tr>
<tr>
<td>29</td>
<td></td>
<td>Macintosh Character Set (US Roman)</td>
</tr>
<tr>
<td>30</td>
<td></td>
<td>OEM Character Set</td>
</tr>
<tr>
<td>31</td>
<td></td>
<td>Symbol Character Set</td>
</tr>
<tr>
<td>32 – 47</td>
<td></td>
<td>Reserved for OEM</td>
</tr>
<tr>
<td>48</td>
<td>869</td>
<td>IBM Greek</td>
</tr>
<tr>
<td>49</td>
<td>866</td>
<td>MS-DOS Russian</td>
</tr>
<tr>
<td>50</td>
<td>865</td>
<td>MS-DOS Nordic</td>
</tr>
<tr>
<td>51</td>
<td>864</td>
<td>Arabic</td>
</tr>
<tr>
<td>52</td>
<td>863</td>
<td>MS-DOS Canadian French</td>
</tr>
<tr>
<td>53</td>
<td>862</td>
<td>Hebrew</td>
</tr>
<tr>
<td>54</td>
<td>861</td>
<td>MS-DOS Icelandic</td>
</tr>
<tr>
<td>55</td>
<td>860</td>
<td>MS-DOS Portuguese</td>
</tr>
<tr>
<td>56</td>
<td>857</td>
<td>IBM Turkish</td>
</tr>
<tr>
<td>57</td>
<td>855</td>
<td>IBM Cyrillic; primarily Russian</td>
</tr>
<tr>
<td>58</td>
<td>852</td>
<td>Latin 2</td>
</tr>
<tr>
<td>59</td>
<td>775</td>
<td>MS-DOS Baltic</td>
</tr>
<tr>
<td>60</td>
<td>737</td>
<td>Greek; former 437 G</td>
</tr>
<tr>
<td>61</td>
<td>708</td>
<td>Arabic; ASMO 708</td>
</tr>
<tr>
<td>62</td>
<td>850</td>
<td>WE/Latin 1</td>
</tr>
<tr>
<td>63</td>
<td>437</td>
<td>US</td>
</tr>
</tbody>
</table>
</td>
</tr>
<tr>
<td>Version differences:</td>
<td>
<b>Version 0:</b> These fields were not defined in version 0. If the size of a version 0 OS/2 table extends beyond the usWinDescent field, additional data beyond the usWinDescent field should be ignored.
<br/><br/>
<b>Version 1:</b> Bit 8 was not assigned in version 1. All other currently assigned bits were defined in version 1.
<br/><br/>
<b>Version 2 and later:</b> All currently assigned bits were defined in version 2.

</td></tr>
</tbody>
</table>

