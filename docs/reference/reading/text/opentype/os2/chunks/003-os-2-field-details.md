# OS/2 Field Details

Source: [OS/2 and Windows metrics](https://learn.microsoft.com/en-us/typography/opentype/spec/os2) · OpenType 1.9.1 website snapshot.

[Document index](../README.md) · [Topic guide](../../../../../README.md) · [Publisher](https://learn.microsoft.com/en-us/typography/opentype/spec/os2).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-version-3.md) · [Next](004-fstype.md)

<a id="os2-field-details"></a>

## OS/2 Field Details

This section provides details covering all versions of the OS/2 table.

<a id="ver"></a>

<a id="version"></a>

### version

|  |  |
| --- | --- |
| Format: | uint16 |
| Units: | n/a |
| Title: | OS/2 table version number. |
| Description: | The version number for the OS/2 table: 0x0000 to 0x0005. |
| Comments: | The version number allows for identification of the precise contents and layout for the OS/2 table. |

<a id="acw"></a>

<a id="xavgcharwidth"></a>

### xAvgCharWidth

<table>
<tbody>
<tr>
<td>Format:</td>
<td>FWORD</td>
</tr>
<tr>
<td>Units:</td>
<td>Font design units</td>
</tr>
<tr>
<td>Title:</td>
<td>Average weighted escapement.</td>
</tr>
<tr>
<td>Description:</td>
<td>The Average Character Width field specifies the arithmetic average of the escapement (width) of all non-zero width glyphs in the font.</td>
</tr>
<tr>
<td>Comments:</td>
<td>The value for xAvgCharWidth is calculated by obtaining the arithmetic average of the width of all non-zero width glyphs in the font. It is strongly recommended that implementers do not rely on this value for computing layout for lines of text, especially for cases where complex scripts are used.<br/><br/>
<em>Note:</em> Some fonts have xAvgCharWidth values that do not conform to this calculation. Fonts could have been originally created with a version 0 to version 2 OS/2 table using the legacy calculation described below, then later updated with a newer OS/2 table version but without changing the xAvgCharWidth value. In some CJK fonts, the xAvgCharWidth can be almost half the value that would be expected by the above calculation. Applications should not use xAvgCharWidth for determining actual glyph advance widths.
</td>
</tr>
<tr>
<td>Version differences:</td>
<td>Versions 0 to 2: When first defined, the specification was biased toward Basic Latin characters, and it was thought that the xAvgCharWidth value could be used to estimate the average length of lines of text. The following formula for calculating xAvgCharWidth was provided: For characters given in the table below, sum the width of each character times the weight factor indicated, then divide the total by 1000. This method of calculating the value of this field was superseded with the introduction of version 3 of the OS/2 table and is deprecated.
<!-- Embedded table -->
<table>
<tbody>
<tr>
<th>Letter</th>
<th>Weight factor</th>
<th>Letter</th>
<th>Weight factor</th>
</tr>
<tr>
<td>a</td>
<td>64</td>
<td>o</td>
<td>56</td>
</tr>
<tr>
<td>b</td>
<td>14</td>
<td>p</td>
<td>17</td>
</tr>
<tr>
<td>c</td>
<td>27</td>
<td>q</td>
<td>4</td>
</tr>
<tr>
<td>d</td>
<td>35</td>
<td>r</td>
<td>49</td>
</tr>
<tr>
<td>e</td>
<td>100</td>
<td>s</td>
<td>56</td>
</tr>
<tr>
<td>f</td>
<td>20</td>
<td>t</td>
<td>71</td>
</tr>
<tr>
<td>g</td>
<td>14</td>
<td>u</td>
<td>31</td>
</tr>
<tr>
<td>h</td>
<td>42</td>
<td>v</td>
<td>10</td>
</tr>
<tr>
<td>i</td>
<td>63</td>
<td>w</td>
<td>18</td>
</tr>
<tr>
<td>j</td>
<td>3</td>
<td>x</td>
<td>3</td>
</tr>
<tr>
<td>k</td>
<td>6</td>
<td>y</td>
<td>18</td>
</tr>
<tr>
<td>l</td>
<td>35</td>
<td>z</td>
<td>2</td>
</tr>
<tr>
<td>m</td>
<td>20</td>
<td>space</td>
<td>166</td>
</tr>
<tr>
<td>n</td>
<td>56</td>
<td> </td>
<td> </td>
</tr>
</tbody>
</table>
<!-- End of embedded table -->
</td>
</tr>
</tbody>
</table>

<a id="wtc"></a>

<a id="usweightclass"></a>

### usWeightClass

<table>
<tbody>
<tr>
<td>Format:</td>
<td>uint16</td>
</tr>
<tr>
<td>Title:</td>
<td>Weight class.</td>
</tr>
<tr>
<td>Description:</td>
<td>Indicates the visual weight (degree of blackness or thickness of strokes) of the characters in the font. Values from 1 to 1000 are valid.</td>
</tr>
<tr>
<td>Comments:</td>
<td>usWeightClass values use the same scale as the 'wght' axis that is used in the <a data-linktype="relative-path" href="https://learn.microsoft.com/en-us/typography/opentype/spec/fvar">'fvar'</a> table of variable fonts and in the <a data-linktype="relative-path" href="https://learn.microsoft.com/en-us/typography/opentype/spec/stat">STAT</a> table. While integer values from 1 to 1000 are supported, some legacy platforms could have limitations on supported values. The following are commonly used values: 
<table>
<tbody>
<tr>
<th>Value</th>
<th>Description</th>
<th>C Definition (from windows.h)</th>
</tr>
<tr>
<td>100</td>
<td>Thin</td>
<td>FW_THIN</td>
</tr>
<tr>
<td>200</td>
<td>Extra-light (Ultra-light)</td>
<td>FW_EXTRALIGHT</td>
</tr>
<tr>
<td>300</td>
<td>Light</td>
<td>FW_LIGHT</td>
</tr>
<tr>
<td>400</td>
<td>Normal (Regular)</td>
<td>FW_NORMAL</td>
</tr>
<tr>
<td>500</td>
<td>Medium</td>
<td>FW_MEDIUM</td>
</tr>
<tr>
<td>600</td>
<td>Semi-bold (Demi-bold)</td>
<td>FW_SEMIBOLD</td>
</tr>
<tr>
<td>700</td>
<td>Bold</td>
<td>FW_BOLD</td>
</tr>
<tr>
<td>800</td>
<td>Extra-bold (Ultra-bold)</td>
<td>FW_EXTRABOLD</td>
</tr>
<tr>
<td>900</td>
<td>Black (Heavy)</td>
<td>FW_BLACK</td>
</tr>
</tbody>
</table>
</td>
</tr>
</tbody>
</table>

<a id="wdc"></a>

<a id="uswidthclass"></a>

### usWidthClass

<table>
<tbody>
<tr>
<td>Format:</td>
<td>uint16</td>
</tr>
<tr>
<td>Title:</td>
<td>Width class.</td>
</tr>
<tr>
<td>Description:</td>
<td>Indicates a relative change from the normal aspect ratio (width to height ratio) as specified by a font designer for the glyphs in a font.</td>
</tr>
<tr>
<td>Comments:</td>
<td>
<p>Although every glyph in a font can have a different numeric aspect ratio, each glyph in a font of normal width is considered to have a relative aspect ratio of one. When a new type style is created of a different width class (either by a font designer or by some automated means) the relative aspect ratio of the glyphs in the new font is some percentage greater or less than the corresponding glyphs in the normal font — it is this difference that this field specifies.
<br/><br/>
The valid usWidthClass values are shown in the following table. Note that the usWidthClass values are related to but distinct from the scale for the 'wdth' axis that is used in the <a data-linktype="relative-path" href="https://learn.microsoft.com/en-us/typography/opentype/spec/fvar">'fvar'</a> table of variable fonts and in the <a data-linktype="relative-path" href="https://learn.microsoft.com/en-us/typography/opentype/spec/stat">STAT</a> table. The “% of normal” column in the following table provides a mapping from usWidthClass values 1 – 9 to 'wdth' values.</p>
<table>
<tbody>
<tr>
<th>Value</th>
<th>Description</th>
<th>C Definition</th>
<th>% of normal</th>
</tr>
<tr>
<td>1</td>
<td>Ultra-condensed</td>
<td>FWIDTH_ULTRA_CONDENSED</td>
<td>50</td>
</tr>
<tr>
<td>2</td>
<td>Extra-condensed</td>
<td>FWIDTH_EXTRA_CONDENSED</td>
<td>62.5</td>
</tr>
<tr>
<td>3</td>
<td>Condensed</td>
<td>FWIDTH_CONDENSED</td>
<td>75</td>
</tr>
<tr>
<td>4</td>
<td>Semi-condensed</td>
<td>FWIDTH_SEMI_CONDENSED</td>
<td>87.5</td>
</tr>
<tr>
<td>5</td>
<td>Medium (normal)</td>
<td>FWIDTH_NORMAL</td>
<td>100</td>
</tr>
<tr>
<td>6</td>
<td>Semi-expanded</td>
<td>FWIDTH_SEMI_EXPANDED</td>
<td>112.5</td>
</tr>
<tr>
<td>7</td>
<td>Expanded</td>
<td>FWIDTH_EXPANDED</td>
<td>125</td>
</tr>
<tr>
<td>8</td>
<td>Extra-expanded</td>
<td>FWIDTH_EXTRA_EXPANDED</td>
<td>150</td>
</tr>
<tr>
<td>9</td>
<td>Ultra-expanded</td>
<td>FWIDTH_ULTRA_EXPANDED</td>
<td>200</td>
</tr>
</tbody>
</table>
</td>
</tr>
</tbody>
</table>

