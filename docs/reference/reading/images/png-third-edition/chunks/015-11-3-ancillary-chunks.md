# 11.3 Ancillary chunks

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](014-10-3-other-uses-of-compression.md) · [Next](016-11-3-2-5-srgb-standard-rgb-color-space.md)

<a id="11Ancillary-chunks"></a>

<a id="x11-3-ancillary-chunks"></a>

### 11.3 Ancillary chunks

<a id="11AcGen"></a>

<a id="introduction-6"></a>

### Introduction

The ancillary chunks defined in this specification are listed in the order in [4.8.2 Chunk types](006-4-7-additional-information.md#4Concepts.FormatTypes).
This is not the order in which they appear in a PNG datastream. Ancillary chunks may be ignored by a decoder. For each
ancillary chunk, the actions described are under the assumption that the decoder is not ignoring the chunk.

<a id="11transinfo"></a>

<a id="x11-3-1-transparency-information"></a>

#### 11.3.1 Transparency information

<a id="11tRNS"></a>

<a id="x11-3-1-1-trns-transparency"></a>

##### 11.3.1.1 tRNS Transparency

The four-byte chunk type field contains the hexadecimal values



```
74 52 4E 53
```




<a id="ref-for-3indexedColour-13"></a>

<a id="ref-for-dfn-greyscale-7"></a>

<a id="ref-for-3truecolour-11"></a>

The tRNS chunk specifies either alpha values that are associated with palette entries (for
[indexed-color](004-4-3-color-spaces.md#3indexedColour) images) or a single transparent color (for [greyscale](005-4-5-png-image.md#dfn-greyscale) and [truecolor](005-4-5-png-image.md#3truecolour) images). The tRNS chunk contains:

<a id="tRNS-structure"></a>

<a id="ref-for-3colourType-19"></a>

<a id="ref-for-3colourType-20"></a>

<a id="ref-for-3colourType-21"></a>

<table class="simple numbered" id="tRNS-structure">
<tbody><tr>
<th colspan="2">
<a class="internalDFN" data-link-type="dfn" href="010-5-7-defining-chunks.md#3colourType" id="ref-for-3colourType-19">Color type</a> 0
              </th>
</tr>
<tr>
<td>Grey sample value</td>
<td>2 bytes</td>
</tr>
<tr>
<th colspan="2">
<a class="internalDFN" data-link-type="dfn" href="010-5-7-defining-chunks.md#3colourType" id="ref-for-3colourType-20">Color type</a> 2
              </th>
</tr>
<tr>
<td>Red sample value</td>
<td>2 bytes</td>
</tr>
<tr>
<td>Green sample value</td>
<td>2 bytes</td>
</tr>
<tr>
<td>Blue sample value</td>
<td>2 bytes</td>
</tr>
<tr>
<th colspan="2">
<a class="internalDFN" data-link-type="dfn" href="010-5-7-defining-chunks.md#3colourType" id="ref-for-3colourType-21">Color type</a> 3
              </th>
</tr>
<tr>
<td>Alpha for palette index 0</td>
<td>1 byte</td>
</tr>
<tr>
<td>Alpha for palette index 1</td>
<td>1 byte</td>
</tr>
<tr>
<td>...etc...</td>
<td>1 byte</td>
</tr>
</tbody></table>

<a id="ref-for-3colourType-22"></a>

For [color type](010-5-7-defining-chunks.md#3colourType) 3 (indexed-color), the tRNS chunk contains a series of one-byte
alpha values, corresponding to entries in the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk. Each entry indicates that
pixels of the corresponding palette index shall be treated as having the specified alpha value. Alpha values have the
same interpretation as in an 8-bit full alpha channel: 0 is fully transparent, 255 is fully opaque, regardless of image
bit depth. The tRNS chunk shall not contain more alpha values than there are palette entries,
but a tRNS chunk may contain fewer values than there are palette entries. In this case, the
alpha value for all remaining palette entries is assumed to be 255. In the common case in which only palette index 0 need
be made transparent, only a one-byte tRNS chunk is needed, and when all palette indices are
opaque, the tRNS chunk may be omitted.

<a id="ref-for-3colourType-23"></a>

For [color types](010-5-7-defining-chunks.md#3colourType) 0 or 2, two bytes per sample are used regardless of the image bit depth (see [7.1 Integers and byte order](011-6-2-alpha-representation.md#7Integers-and-byte-order)). Pixels of the specified grey sample value or RGB sample values are treated as
transparent (equivalent to alpha value 0); all other pixels are to be treated as fully opaque (alpha value
2<sup>bitdepth</sup>-1).
If the image bit depth is less than 16, the least significant bits are used.
Encoders should set the other bits to 0, and decoders must mask the other bits to 0 before the value is used.

<a id="ref-for-3colourType-24"></a>

A tRNS chunk shall not appear for [color types](010-5-7-defining-chunks.md#3colourType) 4 and 6, since a full alpha channel
is already present in those cases.

<a id="issue-container-generatedID-10"></a>

<a id="h-note-10"></a>

Note

<a id="ref-for-dfn-greyscale-8"></a>

<a id="ref-for-3truecolour-12"></a>

NOTE For 16-bit [greyscale](005-4-5-png-image.md#dfn-greyscale) or [truecolor](005-4-5-png-image.md#3truecolour) data,
as described in [13.12 Sample depth rescaling](028-13-10-interlacing-and-progressive-display.md#13Sample-depth-rescaling),
only [pixels matching the entire 16-bit values](028-13-10-interlacing-and-progressive-display.md#tRNS-compare-exactly) in
tRNS chunks are transparent. Decoders have to postpone any sample depth rescaling until after
the pixels have been tested for transparency.

<a id="11addnlcolinfo"></a>

<a id="x11-3-2-color-space-information"></a>

#### 11.3.2 Color space information

<a id="11cHRM"></a>

<a id="x11-3-2-1-chrm-primary-chromaticities-and-white-point"></a>

##### 11.3.2.1 cHRM Primary chromaticities and white point

The four-byte chunk type field contains the hexadecimal values



```
63 48 52 4D
```




<a id="ref-for-3PNGimage-3"></a>

<a id="ref-for-dfn-white-point-2"></a>

The cHRM chunk may be used to specify the 1931 CIE *x,y* chromaticities of the red,
green, and blue display primaries used in the [PNG image](003-4-concepts.md#3PNGimage), and the referenced [white point](002-2-scope.md#dfn-white-point). See [C. Gamma and chromaticity](035-c-gamma-and-chromaticity.md#C-GammaAppendix) for more information. The [iCCP](015-11-3-ancillary-chunks.md#11iCCP), and [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#11sRGB) chunks provide more sophisticated support for color management and control.

The cHRM chunk contains:

<a id="cHRM-structure"></a>

[Table 13](015-11-3-ancillary-chunks.md#cHRM-structure) 
cHRM chunk components

| Name | Size |
| --- | --- |
| White point x | 4 bytes |
| White point y | 4 bytes |
| Red x | 4 bytes |
| Red y | 4 bytes |
| Green x | 4 bytes |
| Green y | 4 bytes |
| Blue x | 4 bytes |
| Blue y | 4 bytes |

<a id="ref-for-dfn-png-four-byte-unsigned-integer-4"></a>

Each value is encoded as a [PNG four-byte unsigned integer](002-2-scope.md#dfn-png-four-byte-unsigned-integer), representing the *x* or *y* value times
100000.

A value of 0.3127 would be stored as the integer 31270.

<a id="ref-for-dfn-greyscale-9"></a>

The cHRM chunk is allowed in all PNG datastreams, although it is of little value for
[greyscale](005-4-5-png-image.md#dfn-greyscale) images.

This chunk is ignored
unless it is the [highest-precedence color chunk](004-4-3-color-spaces.md#color-chunk-precendence)
understood by the decoder.

<a id="11gAMA"></a>

<a id="x11-3-2-2-gama-image-gamma"></a>

##### 11.3.2.2 gAMA Image gamma

The four-byte chunk type field contains the hexadecimal values



```
67 41 4D 41
```




<a id="ref-for-dfn-gamma-value-3"></a>

The [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk specifies a [gamma value](002-2-scope.md#dfn-gamma-value).

In fact specifying the desired display output intensity is insufficient. It is also necessary to specify the viewing
conditions under which the output is desired. For gAMA these are the reference viewing
conditions of the sRGB specification [[SRGB](044-g-references.md#bib-srgb "Multimedia systems and equipment - Colour measurement and management - Part 2-1: Colour management - Default RGB colour space - sRGB")]. Adjustment for different viewing conditions is normally handled by a
Color Management System. If the adjustment is not performed, the error is usually small. Applications desiring high
color fidelity may wish to use an [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#11sRGB), [iCCP](015-11-3-ancillary-chunks.md#11iCCP)
chunk.

The gAMA chunk contains:

<a id="gAMA-structure"></a>

|  |  |
| --- | --- |
| Image gamma | 4 bytes |

<a id="ref-for-dfn-png-four-byte-unsigned-integer-5"></a>

<a id="ref-for-dfn-gamma-value-4"></a>

The value is encoded as a [PNG four-byte unsigned integer](002-2-scope.md#dfn-png-four-byte-unsigned-integer), representing the [gamma value](002-2-scope.md#dfn-gamma-value) times 100000.

<a id="ref-for-dfn-gamma-value-5"></a>

A [gamma value](002-2-scope.md#dfn-gamma-value) of 1/2.2 would be stored as the integer 45455.

See [12.1 Encoder gamma handling](021-12-png-encoders.md#12Encoder-gamma-handling) and [13.13 Decoder gamma handling](029-13-13-decoder-gamma-handling.md#13Decoder-gamma-handling) for more information.

This chunk is ignored
unless it is the [highest-precedence color chunk](004-4-3-color-spaces.md#color-chunk-precendence)
understood by the decoder.

<a id="11iCCP"></a>

<a id="x11-3-2-3-iccp-embedded-icc-profile"></a>

##### 11.3.2.3 iCCP Embedded ICC profile

The four-byte chunk type field contains the hexadecimal values



```
69 43 43 50
```




The iCCP chunk contains:

<a id="iCCP-structure"></a>

|  |  |
| --- | --- |
| Profile name | 1-79 bytes (character string) |
| Null separator | 1 byte (null character) |
| Compression method | 1 byte |
| Compressed profile | n bytes |

<a id="ref-for-dfn-zlib-14"></a>

<a id="ref-for-dfn-deflate-12"></a>

The profile name may be any convenient name for referring to the profile. It is case-sensitive. Profile names shall
contain only printable Latin-1 characters and spaces (only code points 0x20-7E and 0xA1-FF are allowed). Leading,
trailing, and consecutive spaces are not permitted. The only compression method defined in this specification is method 0
([zlib](002-2-scope.md#dfn-zlib) datastream with [deflate](002-2-scope.md#dfn-deflate) compression, see [10.3 Other uses of compression](014-10-3-other-uses-of-compression.md#10CompressionOtherUses)). The compression
method entry is followed by a compressed profile that makes up the remainder of the chunk. Decompression of this
datastream yields the embedded ICC profile.

<a id="ref-for-3colourType-25"></a>

<a id="ref-for-dfn-greyscale-10"></a>

<a id="ref-for-3colourType-26"></a>

If the iCCP chunk is present, the image samples conform to the color space represented by
the embedded ICC profile as defined by the International Color Consortium [[ICC](044-g-references.md#bib-icc "ICC.1:2022 (Profile version 4.4.0.0)")][[ISO\_15076-1](044-g-references.md#bib-iso_15076-1 "ISO 15076-1:2010 Image technology colour management — Architecture, profile format and data structure — Part 1: Based on ICC.1:2010")]. The color space of the
ICC profile shall be an RGB color space for color images ([color types](010-5-7-defining-chunks.md#3colourType) 2, 3, and 6), or a greyscale color space
for [greyscale](005-4-5-png-image.md#dfn-greyscale) images ([color types](010-5-7-defining-chunks.md#3colourType) 0 and 4). A PNG encoder that writes the iCCP chunk
is encouraged to also write [gAMA](015-11-3-ancillary-chunks.md#11gAMA) and [cHRM](015-11-3-ancillary-chunks.md#11cHRM) chunks
that approximate the ICC profile, to provide compatibility with applications that do not use the iCCP chunk.

This chunk is ignored
unless it is the [highest-precedence color chunk](004-4-3-color-spaces.md#color-chunk-precendence)
understood by the decoder.

Unless a [cICP](016-11-3-2-5-srgb-standard-rgb-color-space.md#cICP-chunk) chunk exists, a PNG datastream should contain at most one
embedded profile, whether specified explicitly with an iCCP or implicitly with an [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#srgb-standard-colour-space) chunk.

<a id="11sBIT"></a>

<a id="x11-3-2-4-sbit-significant-bits"></a>

##### 11.3.2.4 sBIT Significant bits

The four-byte chunk type field contains the hexadecimal values



```
73 42 49 54
```




To simplify decoders, PNG specifies that only certain sample depths may be used, and further specifies that sample
values should be scaled to the full range of possible values at the sample depth. The sBIT
chunk defines the original number of significant bits (which can be less than or equal to the sample depth). This allows
PNG decoders to recover the original data losslessly even if the data had a sample depth not directly supported by
PNG.

The sBIT chunk contains:

