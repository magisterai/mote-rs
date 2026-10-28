# 12.6 Interlacing

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](023-12-3-alpha-channel-creation.md) · [Next](025-13-1-error-handling.md)

<a id="12Interlacing"></a>

<a id="x12-6-interlacing"></a>

### 12.6 Interlacing

This specification defines two interlace methods, one of which is no interlacing. Interlacing provides a convenient basis
from which decoders can progressively display an image, as described in [13.10 Interlacing and progressive display](028-13-10-interlacing-and-progressive-display.md#13Progressive-display).

<a id="12Filter-selection"></a>

<a id="x12-7-filter-selection"></a>

### 12.7 Filter selection

<a id="ref-for-3colourType-39"></a>

<a id="ref-for-3indexedColour-17"></a>

<a id="ref-for-3truecolour-19"></a>

For images of [color type](010-5-7-defining-chunks.md#3colourType) 3 (indexed-color), filter type 0 (None) is usually the most effective. Color images
with 256 or fewer colors should almost always be stored in [indexed-color](004-4-3-color-spaces.md#3indexedColour) format; [truecolor](005-4-5-png-image.md#3truecolour) format is likely to be
much larger.

Filter type 0 is also recommended for images of bit depths less than 8. For low-bit-depth greyscale images, in rare cases,
better compression may be obtained by first expanding the image to 8-bit representation and then applying filtering.

<a id="ref-for-3truecolour-20"></a>

<a id="ref-for-dfn-greyscale-14"></a>

For [truecolor](005-4-5-png-image.md#3truecolour) and [greyscale](005-4-5-png-image.md#dfn-greyscale) images, any of the five filters may prove the most effective. If an encoder uses a
fixed filter, the Paeth filter type is most likely to be the best.

<a id="ref-for-3truecolour-21"></a>

<a id="ref-for-dfn-greyscale-15"></a>

For best compression of [truecolor](005-4-5-png-image.md#3truecolour) and [greyscale](005-4-5-png-image.md#dfn-greyscale) images,
and if compression efficiency is valued over speed of compression,
the recommended approach is adaptive filtering in which a
filter type is chosen for each scanline.
Each unique image will have a different set of filters which perform best for it.
An encoder could try every combination of filters to find what compresses best
for a given image. However, when an exhaustive search is unacceptable,
here are some general heuristics which may perform well enough:
compute the output
scanline using all five filters, and select the filter that gives the smallest sum of absolute values of outputs. (Consider
the output bytes as signed differences for this test.)
This method usually outperforms any single fixed filter type choice.

Filtering according to these recommendations is effective in conjunction with either of the two interlace methods defined
in this specification.

<a id="12Compression"></a>

<a id="x12-8-compression"></a>

### 12.8 Compression

The encoder may divide the compressed datastream into [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks however it wishes.
(Multiple [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks are allowed so that encoders may work in a fixed amount of memory;
typically the chunk size will correspond to the encoder's buffer size.) A PNG datastream in which each [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk contains only one data byte is valid, though remarkably wasteful of space. ([Zero-length](008-4-10-error-handling.md#zero-length-data) [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks are also valid, though even more wasteful.)

<a id="12Text-chunk-processing"></a>

<a id="x12-9-text-chunk-processing"></a>

### 12.9 Text chunk processing

A nonempty keyword shall be provided for each text chunk. The generic keyword "Comment" can be used if no better
description of the text is available. If a user-supplied keyword is used, encoders should check that it meets the
restrictions on keywords.

The [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunk uses the UTF-8 encoding of Unicode and thus can store text in any
language. The [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks use the Latin-1
(ISO 8859-1) character encoding, which limits the range of characters that can be used in these chunks. Encoders should
prefer [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) to [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks, in order to allow a wide range of characters without data loss. Encoders must convert characters
that use local [legacy character encodings](https://www.w3.org/TR/i18n-glossary/#dfn-legacy-character-encodings) to the appropriate encoding when storing text.

When creating [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunks,
encoders should follow [UTF-8 encode](https://encoding.spec.whatwg.org/#utf-8-encode) in [Encoding Standard](https://encoding.spec.whatwg.org/).

<a id="ref-for-dfn-image-data-19"></a>

<a id="ref-for-dfn-image-data-20"></a>

Encoders should discourage
the creation of single lines of text longer than 79 Unicode [code points](https://www.w3.org/TR/i18n-glossary/#dfn-code-point), in order to facilitate easy reading. It is
recommended that text items less than 1024 bytes in size should be output using uncompressed text chunks. It is recommended
that the basic title and author keywords be output using uncompressed text chunks. Placing large text chunks after the
[image data](002-2-scope.md#dfn-image-data) (after the [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks) can speed up image display in some situations,
as the decoder will decode the [image data](002-2-scope.md#dfn-image-data) first. It is recommended that small text chunks, such as the image title,
appear before the [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks.

<a id="12Chunk-processing"></a>

<a id="x12-10-chunking"></a>

### 12.10 Chunking

<a id="12Use-of-private-chunks"></a>

<a id="x12-10-1-use-of-private-chunks"></a>

#### 12.10.1 Use of private chunks

Encoders *MAY* use private chunks to carry information that need not be understood by other applications.

<a id="12Private-type-and-method-codes"></a>

<a id="x12-10-2-use-of-non-reserved-field-values"></a>

#### 12.10.2 Use of non-reserved field values

Encoders *MAY* use non-reserved field values for experimental or private use.

<a id="12Ancillary"></a>

<a id="x12-10-3-ancillary-chunks"></a>

#### 12.10.3 Ancillary chunks

All ancillary chunks are optional, encoders need not write them. However, encoders are encouraged to write the standard
ancillary chunks when the information is available.

<a id="13Decoders"></a>

<a id="x13-png-decoders-and-viewers"></a>

## 13. PNG decoders and viewers

<a id="13Introduction"></a>

<a id="introduction-9"></a>

## Introduction

This clause gives some requirements and recommendations for PNG decoder behavior and viewer behavior. A viewer presents
the decoded PNG image to the user. Since viewer and decoder behavior are closely connected, decoders and viewers are treated
together here. The only absolute requirement on a PNG decoder is that it successfully reads any datastream conforming to the
format specified in the preceding chapters. However, best results will usually be achieved by following these additional
recommendations.

<a id="ref-for-3colourType-40"></a>

<a id="ref-for-3filter-13"></a>

PNG decoders shall support all valid combinations of bit depth, [color type](010-5-7-defining-chunks.md#3colourType), compression method, [filter
method](012-7-3-filtering.md#3filter), and interlace method that are explicitly defined in this International Standard.

