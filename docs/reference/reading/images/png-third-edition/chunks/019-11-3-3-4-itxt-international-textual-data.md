# 11.3.3.4 iTXt International textual data

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](018-11-3-2-8-clli-content-light-level-information.md) · [Next](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md)

The zTXt and [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) chunks are semantically equivalent,
but the zTXt chunk is recommended for storing large blocks of text.

A zTXt chunk contains:

<a id="zTXt-structure"></a>

|  |  |
| --- | --- |
| Keyword | 1-79 bytes (character string) |
| Null separator | 1 byte (null character) |
| Compression method | 1 byte |
| Compressed text datastream | n bytes |

<a id="ref-for-dfn-deflate-14"></a>

<a id="ref-for-dfn-zlib-16"></a>

The keyword and null character are the same as in the [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) chunk. The keyword is not compressed. The compression method entry defines the compression method used. The
only value defined in this International Standard is 0 ([deflate](002-2-scope.md#dfn-deflate) compression). Other values are reserved for future
standardization. The compression method entry is followed by the compressed text datastream that makes up the remainder
of the chunk. For compression method 0, this datastream is a [zlib](002-2-scope.md#dfn-zlib) datastream with deflate compression (see
[10.3 Other uses of compression](014-10-3-other-uses-of-compression.md#10CompressionOtherUses)). Decompression of this datastream yields Latin-1 text that is identical to the
text that would be stored in an equivalent [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) chunk.

<a id="11iTXt"></a>

<a id="x11-3-3-4-itxt-international-textual-data"></a>

##### 11.3.3.4 iTXt International textual data

The four-byte chunk type field contains the hexadecimal values



```
69 54 58 74
```




An iTXt chunk contains:

<a id="iTXt-structure"></a>

|  |  |
| --- | --- |
| Keyword | 1-79 bytes (character string) |
| Null separator | 1 byte (null character) |
| Compression flag | 1 byte |
| Compression method | 1 byte |
| Language tag | 0 or more bytes (character string) |
| Null separator | 1 byte (null character) |
| Translated keyword | 0 or more bytes |
| Null separator | 1 byte (null character) |
| Text | 0 or more bytes |

The keyword is described in [11.3.3.1 Keywords and text strings](018-11-3-2-8-clli-content-light-level-information.md#11keywords).

<a id="ref-for-dfn-zlib-17"></a>

<a id="ref-for-dfn-deflate-15"></a>

The compression flag is 0 for uncompressed text, 1 for compressed text. Only the text field may be compressed. The
compression method entry defines the compression method used. The only compression method defined in this specification
is 0 ([zlib](002-2-scope.md#dfn-zlib) datastream with [deflate](002-2-scope.md#dfn-deflate) compression, see [10.3 Other uses of compression](014-10-3-other-uses-of-compression.md#10CompressionOtherUses)). For
uncompressed text, encoders shall set the compression method to 0, and decoders shall ignore it.

The language tag is a well-formed language tag defined by [[BCP47](044-g-references.md#bib-bcp47 "Tags for Identifying Languages")]. Unlike the keyword, the language tag is
case-insensitive. Subtags must appear in the
[IANA language subtag registry](https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry).
If the language tag is empty, the language is
unspecified. Examples of language tags include: `en`, `en-GB`, `es-419`,
`zh-Hans`, `zh-Hans-CN`, `tlh-Cyrl-AQ`, `ar-AE-u-nu-latn`, and
`x-private`.

The translated keyword and text both use the UTF-8 encoding [[rfc3629](044-g-references.md#bib-rfc3629 "UTF-8, a transformation format of ISO 10646")], and neither shall contain a zero byte (null
character). The text, unlike other textual data in this chunk, is not null-terminated; its length is derived from the
chunk length.

Line breaks should not appear in the translated keyword. In the text, a newline should be represented by a single
linefeed character (hexadecimal 0A). The remaining control characters (01-09, 0B-1F, 7F-9F) are discouraged in both the
translated keyword and text. In UTF-8 there is a difference between the characters 80-9F (which are discouraged) and the
bytes 80-9F (which are often necessary).

The translated keyword, if not empty, should contain a translation of the keyword into the language indicated by the
language tag, and applications displaying the keyword should display the translated keyword in addition.

<a id="11addnlsiinfo"></a>

<a id="x11-3-4-miscellaneous-information"></a>

#### 11.3.4 Miscellaneous information

<a id="11bKGD"></a>

<a id="x11-3-4-1-bkgd-background-color"></a>

##### 11.3.4.1 bKGD Background color

The four-byte chunk type field contains the hexadecimal values



```
62 4B 47 44
```




The bKGD chunk specifies a default background color to present the image against. If there
is any other preferred background, either user-specified or part of a larger page (as in a browser), the bKGD chunk should be ignored. The bKGD chunk contains:

<a id="bKGD-structure"></a>

<table class="numbered simple" id="bKGD-structure">
<caption><a class="self-link" href="019-11-3-3-4-itxt-international-textual-data.md#bKGD-structure">Table <bdi class="tableno">22</bdi></a> <span class="table-title">
              bKGD chunk contents
            </span></caption>
<tbody><tr>
<th colspan="2">Color types 0 and 4</th>
</tr>
<tr>
<td>Greyscale</td>
<td>2 bytes</td>
</tr>
<tr>
<th colspan="2">Color types 2 and 6</th>
</tr>
<tr>
<td>Red</td>
<td>2 bytes</td>
</tr>
<tr>
<td>Green</td>
<td>2 bytes</td>
</tr>
<tr>
<td>Blue</td>
<td>2 bytes</td>
</tr>
<tr>
<th colspan="2">Color type 3</th>
</tr>
<tr>
<td>Palette index</td>
<td>1 byte</td>
</tr>
</tbody></table>

<a id="ref-for-3colourType-28"></a>

<a id="ref-for-3indexedColour-15"></a>

For [color type](010-5-7-defining-chunks.md#3colourType) 3 ([indexed-color](004-4-3-color-spaces.md#3indexedColour)), the value is the palette index of the color to be used as background.

<a id="ref-for-3colourType-29"></a>

<a id="ref-for-dfn-greyscale-11"></a>

<a id="ref-for-dfn-greyscale-with-alpha-3"></a>

<a id="ref-for-3colourType-30"></a>

<a id="ref-for-3truecolour-13"></a>

<a id="ref-for-dfn-truecolor-with-alpha-4"></a>

For [color types](010-5-7-defining-chunks.md#3colourType) 0 and 4 ([greyscale](005-4-5-png-image.md#dfn-greyscale), [greyscale with alpha](005-4-5-png-image.md#dfn-greyscale-with-alpha)), the value is the grey level to be used as
background in the range 0 to (2<sup>bitdepth</sup>)-1. For [color types](010-5-7-defining-chunks.md#3colourType) 2 and 6 ([truecolor](005-4-5-png-image.md#3truecolour), [truecolor
with alpha](005-4-5-png-image.md#dfn-truecolor-with-alpha)), the values are the color to be used as background, given as RGB samples in the range 0 to
(2<sup>bitdepth</sup>)-1. In each case, for consistency, two bytes per sample are used regardless of the image bit depth.
If the image bit depth is less than 16, the least significant bits are used.
Encoders should set the other bits to 0, and decoders must mask the other bits to 0 before the value is used.

<a id="11hIST"></a>

<a id="x11-3-4-2-hist-image-histogram"></a>

##### 11.3.4.2 hIST Image histogram

The four-byte chunk type field contains the hexadecimal values



```
68 49 53 54
```




The hIST chunk contains a series of two-byte unsigned integers:

<a id="hIST-structure"></a>

|  |  |
| --- | --- |
| Frequency | 2 bytes (unsigned integer) |
| ...etc... |  |

The hIST chunk gives the approximate usage frequency of each color in the palette. A
histogram chunk can appear only when a [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk appears. If a viewer is unable to
provide all the colors listed in the palette, the histogram may help it decide how to choose a subset of the colors for
display.

There shall be exactly one entry for each entry in the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk. Each entry is
proportional to the fraction of pixels in the image that have that palette index; the exact scale factor is chosen by the
encoder.

Histogram entries are approximate, with the exception that a zero entry specifies that the corresponding palette entry
is not used at all in the image. A histogram entry shall be nonzero if there are any pixels of that color.

<a id="issue-container-generatedID-18"></a>

<a id="h-note-18"></a>

Note

<a id="ref-for-3truecolour-14"></a>

NOTE When the palette is a suggested quantization of a [truecolor](005-4-5-png-image.md#3truecolour) image, the histogram is
necessarily approximate, since a decoder may map pixels to palette entries differently than the encoder did. In this
situation, zero entries should not normally appear, because any entry might be used.

<a id="11pHYs"></a>

<a id="x11-3-4-3-phys-physical-pixel-dimensions"></a>

##### 11.3.4.3 pHYs Physical pixel dimensions

The four-byte chunk type field contains the hexadecimal values



```
70 48 59 73
```




The pHYs chunk specifies the intended pixel size or aspect ratio for display of the image.
It contains:

<a id="pHYs-structure"></a>

<a id="ref-for-dfn-png-four-byte-unsigned-integer-8"></a>

<a id="ref-for-dfn-png-four-byte-unsigned-integer-9"></a>

[Table 23](019-11-3-3-4-itxt-international-textual-data.md#pHYs-structure) 
pHYs chunk contents

| Name | Size |
| --- | --- |
| Pixels per unit, X axis | 4 bytes ([PNG four-byte unsigned integer](002-2-scope.md#dfn-png-four-byte-unsigned-integer)) |
| Pixels per unit, Y axis | 4 bytes ([PNG four-byte unsigned integer](002-2-scope.md#dfn-png-four-byte-unsigned-integer)) |
| Unit specifier | 1 byte |

The following values are defined for the unit specifier:

<a id="unit-specifiers"></a>

[Table 24](019-11-3-3-4-itxt-international-textual-data.md#unit-specifiers) 
Unit specifier values

| Value | Description |
| --- | --- |
| 0 | unit is unknown |
| 1 | unit is the metre |

When the unit specifier is 0, the pHYs chunk defines pixel aspect ratio only; the actual
size of the pixels remains unspecified.

If the pHYs chunk is not present, pixels are assumed to be square, and the physical size of
each pixel is unspecified.

<a id="11sPLT"></a>

<a id="x11-3-4-4-splt-suggested-palette"></a>

##### 11.3.4.4 sPLT Suggested palette

The four-byte chunk type field contains the hexadecimal values



```
73 50 4C 54
```




The sPLT chunk contains:

<a id="sPLT-structure"></a>

[Table 25](019-11-3-3-4-itxt-international-textual-data.md#sPLT-structure) 
sPLT chunk contents

| Name | Size |
| --- | --- |
| Palette name | 1-79 bytes (character string) |
| Null separator | 1 byte (null character) |
| Sample depth | 1 byte |
| Red | 1 or 2 bytes |
| Green | 1 or 2 bytes |
| Blue | 1 or 2 bytes |
| Alpha | 1 or 2 bytes |
| Frequency | 2 bytes |
| ...etc... |  |

Each palette entry is six bytes or ten bytes containing five unsigned integers (red, blue, green, alpha, and
frequency).

There may be any number of entries. A PNG decoder determines the number of entries from the length of the chunk
remaining after the sample depth byte. This shall be divisible by 6 if the sPLT sample depth
is 8, or by 10 if the sPLT sample depth is 16. Entries shall appear in decreasing order of
frequency. There is no requirement that the entries all be used by the image, nor that they all be different.

The palette name can be any convenient name for referring to the palette (for example "256 color including Macintosh
default", "256 color including Windows-3.1 default", "Optimal 512"). The palette name may aid the choice of the
appropriate suggested palette when more than one appears in a PNG datastream.

The palette name is case-sensitive, and subject to the same restrictions as the keyword parameter for the [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) chunk. Palette names shall contain only printable Latin-1 characters and spaces (only
code points 0x20-7E and 0xA1-FF are allowed). Leading, trailing, and consecutive spaces are not permitted.

The sPLT sample depth shall be 8 or 16.

<a id="ref-for-3colourType-31"></a>

<a id="ref-for-dfn-gamma-value-6"></a>

<a id="ref-for-dfn-chromaticity-4"></a>

<a id="ref-for-dfn-greyscale-12"></a>

The red, green, blue, and alpha samples are either one or two bytes each, depending on the sPLT sample depth, regardless of the image bit depth. The color samples are not premultiplied by alpha,
nor are they precomposited against any background. An alpha value of 0 means fully transparent. An alpha value of 255
(when the sPLT sample depth is 8) or 65535 (when the sPLT sample
depth is 16) means fully opaque. The sPLT chunk may appear for any [color type](010-5-7-defining-chunks.md#3colourType). Entries
in sPLT use the same [gamma value](002-2-scope.md#dfn-gamma-value) and [chromaticity](002-2-scope.md#dfn-chromaticity) values as the PNG image, but
may fall outside the range of values used in the color space of the PNG image; for example, in a [greyscale](005-4-5-png-image.md#dfn-greyscale) PNG image,
each sPLT entry would typically have equal red, green, and blue values, but this is not
required. Similarly, sPLT entries can have non-opaque alpha values even when the PNG image
does not use transparency.

