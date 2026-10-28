# 10.3 Other uses of compression

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](013-9-3-filter-type-3-average.md) · [Next](015-11-3-ancillary-chunks.md)

<a id="10CompressionOtherUses"></a>

<a id="x10-3-other-uses-of-compression"></a>

### 10.3 Other uses of compression

<a id="ref-for-dfn-image-data-7"></a>

<a id="ref-for-dfn-zlib-13"></a>

PNG also uses compression method 0 in [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt), [iCCP](015-11-3-ancillary-chunks.md#11iCCP),
and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks. Unlike the [image data](002-2-scope.md#dfn-image-data), such datastreams are not split across
chunks; each such chunk contains an independent [zlib](002-2-scope.md#dfn-zlib) datastream (see [10.1 Compression method 0](013-9-3-filter-type-3-average.md#10CompressionCM0)).

<a id="11Chunks"></a>

<a id="x11-chunk-specifications"></a>

## 11. Chunk specifications

<a id="11Introduction"></a>

<a id="x11-1-general"></a>

### 11.1 General

This clause defines chunk used in this specification.

<a id="11Critical-chunks"></a>

<a id="x11-2-critical-chunks"></a>

### 11.2 Critical chunks

<a id="11CcGen"></a>

<a id="introduction-5"></a>

### Introduction

<a id="3criticalChunk"></a>

A critical chunk is a chunk that is absolutely required in order to successfully decode a PNG image from a PNG
datastream. Extension chunks may be defined as critical chunks (see [14. Editors](032-13-17-histogram-and-suggested-palette-usage.md#14EditorsExt)), though this practice is
strongly discouraged.

A valid PNG datastream shall begin with a PNG signature, immediately followed by an [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk, then one or more [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks, and shall end with an [IEND](014-10-3-other-uses-of-compression.md#11IEND) chunk. Only one [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk and one [IEND](014-10-3-other-uses-of-compression.md#11IEND) chunk are allowed in a PNG datastream.

<a id="11IHDR"></a>

<a id="x11-2-1-ihdr-image-header"></a>

#### 11.2.1 IHDR Image header

The four-byte chunk type field contains the hexadecimal values



```
49 48 44 52
```




The [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk shall be the first chunk in the PNG datastream. It contains:

<a id="IHDR-structure"></a>

|  |  |
| --- | --- |
| Width | 4 bytes |
| Height | 4 bytes |
| Bit depth | 1 byte |
| Color type | 1 byte |
| Compression method | 1 byte |
| Filter method | 1 byte |
| Interlace method | 1 byte |

<a id="ref-for-dfn-png-four-byte-unsigned-integer-3"></a>

Width and height give the image dimensions in pixels. They are [PNG four-byte unsigned integers](002-2-scope.md#dfn-png-four-byte-unsigned-integer). Zero is an
invalid value.

<a id="3bitDepth"></a>

<a id="ref-for-3colourType-9"></a>

Bit depth is a single-byte integer giving the number of bits per sample or per palette index (not per pixel). Valid
values are 1, 2, 4, 8, and 16, although not all values are allowed for all [color types](010-5-7-defining-chunks.md#3colourType). See [6.1 Color types and values](010-5-7-defining-chunks.md#6Colour-values).

<a id="ref-for-3colourType-10"></a>

[Color type](010-5-7-defining-chunks.md#3colourType) is a single-byte integer.

<a id="ref-for-3colourType-11"></a>

Bit depth restrictions for each [color type](010-5-7-defining-chunks.md#3colourType) are imposed to simplify implementations and to prohibit combinations
that do not compress well. The allowed combinations are defined in [Table 12](014-10-3-other-uses-of-compression.md#table111 "Allowed combinations of color type and bit depth").

<a id="table111"></a>

<a id="ref-for-3colourType-12"></a>

[Table 12](014-10-3-other-uses-of-compression.md#table111) 
Allowed combinations of [color type](010-5-7-defining-chunks.md#3colourType) and bit depth

| PNG image type | Color type | Allowed bit depths | Interpretation |
| --- | --- | --- | --- |
| Greyscale | 0 | 1, 2, 4, 8, 16 | Each pixel is a greyscale sample |
| Truecolor | 2 | 8, 16 | Each pixel is an R,G,B triple |
| Indexed-color | 3 | 1, 2, 4, 8 | Each pixel is a palette index; a [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk shall appear. |
| Greyscale with alpha | 4 | 8, 16 | Each pixel is a greyscale sample followed by an alpha sample. |
| Truecolor with alpha | 6 | 8, 16 | Each pixel is an R,G,B triple followed by an alpha sample. |

<a id="ref-for-3indexedColour-12"></a>

<a id="ref-for-3colourType-13"></a>

The sample depth is the same as the bit depth except in the case of [indexed-color](004-4-3-color-spaces.md#3indexedColour) PNG images ([color type](010-5-7-defining-chunks.md#3colourType) 3), in
which the sample depth is always 8 bits (see [4.5 PNG image](005-4-5-png-image.md#4Concepts.PNGImage)).

<a id="ref-for-dfn-image-data-8"></a>

<a id="ref-for-dfn-deflate-11"></a>

Compression method is a single-byte integer that indicates the method used to compress the [image data](002-2-scope.md#dfn-image-data). Only
compression method 0 ([deflate](002-2-scope.md#dfn-deflate) compression with a sliding window of at most 32768 bytes) is defined in this
specification. All conforming PNG images shall be compressed with this scheme.

<a id="ref-for-dfn-image-data-9"></a>

<a id="ref-for-3filter-11"></a>

Filter method is a single-byte integer that indicates the preprocessing method applied to the [image data](002-2-scope.md#dfn-image-data) before
compression. Only [filter method](012-7-3-filtering.md#3filter) 0 (adaptive filtering with five basic filter types) is defined in this
specification. See [9. Filtering](012-7-3-filtering.md#9Filters) for details.

<a id="ref-for-dfn-image-data-10"></a>

Interlace method is a single-byte integer that indicates the transmission order of the [image data](002-2-scope.md#dfn-image-data). Two values are
defined in this specification: 0 (no interlace) or 1 (Adam7 interlace). See [8. Interlacing and pass extraction](012-7-3-filtering.md#8Interlace) for details.

<a id="11PLTE"></a>

<a id="x11-2-2-plte-palette"></a>

#### 11.2.2 PLTE Palette

The four-byte chunk type field contains the hexadecimal values



```
50 4C 54 45
```




The PLTE chunk contains from 1 to 256 palette entries, each a three-byte series of the
form:

<a id="PLTE-structure"></a>

|  |  |
| --- | --- |
| Red | 1 byte |
| Green | 1 byte |
| Blue | 1 byte |

The number of entries is determined from the chunk length. A chunk length not divisible by 3 is an error.

<a id="ref-for-3colourType-14"></a>

<a id="ref-for-3colourType-15"></a>

<a id="ref-for-3colourType-16"></a>

This chunk shall appear for [color type](010-5-7-defining-chunks.md#3colourType) 3, and may appear for [color types](010-5-7-defining-chunks.md#3colourType) 2 and 6; it shall not appear
for [color types](010-5-7-defining-chunks.md#3colourType) 0 and 4. There shall not be more than one PLTE chunk.

<a id="ref-for-3colourType-17"></a>

<a id="ref-for-dfn-image-data-11"></a>

For [color type](010-5-7-defining-chunks.md#3colourType) 3 (indexed-color), the PLTE chunk is required. The first entry in
PLTE is referenced by pixel value 0, the second by pixel value 1, etc. The number of palette
entries shall not exceed the range that can be represented in the image bit depth (for example, 2<sup>4</sup> = 16 for a
bit depth of 4). It is permissible to have fewer entries than the bit depth would allow. In that case, any out-of-range
pixel value found in the [image data](002-2-scope.md#dfn-image-data) is an error.

<a id="ref-for-3colourType-18"></a>

<a id="ref-for-3truecolour-8"></a>

<a id="ref-for-dfn-truecolor-with-alpha-3"></a>

<a id="ref-for-3truecolour-9"></a>

For [color types](010-5-7-defining-chunks.md#3colourType) 2 and 6 ([truecolor](005-4-5-png-image.md#3truecolour) and [truecolor with alpha](005-4-5-png-image.md#dfn-truecolor-with-alpha)), the PLTE chunk
is optional. If present, it provides a suggested set of colors (from 1 to 256) to which the [truecolor](005-4-5-png-image.md#3truecolour) image can be
quantized if it cannot be displayed directly. It is, however, recommended that the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT)
chunk be used for this purpose, rather than the PLTE chunk. If neither PLTE nor [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunks are present and the image cannot be displayed
directly, quantization has to be done by the viewing system. However, it is often preferable for the selection of colors
to be done once by the PNG encoder. (See [12.5 Suggested palettes](023-12-3-alpha-channel-creation.md#12Suggested-palettes).)

<a id="ref-for-3truecolour-10"></a>

Note that the palette uses 8 bits (1 byte) per sample regardless of the image bit depth. In particular, the palette is 8
bits deep even when it is a suggested quantization of a 16-bit [truecolor](005-4-5-png-image.md#3truecolour) image.

There is no requirement that the palette entries all be used by the image, nor that they all be different.

<a id="11IDAT"></a>

<a id="x11-2-3-idat-image-data"></a>

#### 11.2.3 IDAT Image data

The four-byte chunk type field contains the hexadecimal values



```
49 44 41 54
```




<a id="ref-for-dfn-image-data-12"></a>

The IDAT chunk contains the actual [image data](002-2-scope.md#dfn-image-data) which is the output stream of the
compression algorithm. See [9. Filtering](012-7-3-filtering.md#9Filters) and [10. Compression](013-9-3-filter-type-3-average.md#10Compression) for details.

There may be multiple IDAT chunks; if so, they shall appear consecutively with no other
intervening chunks. The compressed datastream is then the concatenation of the contents of the data fields of all the
IDAT chunks
(noting that data fields [may be of zero length](008-4-10-error-handling.md#zero-length-data)).

Some images have unused trailing bytes at the end of the final IDAT chunk. This could happen when an entire buffer is stored rather than just the portion of the buffer which is used. This is undesirable. Preferably, an encoder would not include these unused bytes. If it must, setting the bytes to zero will prevent accidental data sharing. A decoder should ignore these trailing bytes.

<a id="11IEND"></a>

<a id="x11-2-4-iend-image-trailer"></a>

#### 11.2.4 IEND Image trailer

The four-byte chunk type field contains the hexadecimal values



```
49 45 4E 44
```




The IEND chunk marks the end of the PNG datastream. The chunk's data field is empty.

