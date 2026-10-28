# 2. Scope

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](001-portable-network-graphics-png-specification-third-edition.md) · [Next](003-4-concepts.md)

<a id="scope"></a>

<a id="1Scope"></a>

## 2. Scope

<a id="ref-for-dfn-lossless-2"></a>

This specification specifies a datastream and an associated file format, Portable Network Graphics (PNG, pronounced "ping"),
for a [lossless](002-2-scope.md#dfn-lossless), portable, compressed individual computer graphics image or frame-based animation, transmitted across the
Internet.

<a id="terms-definitions-and-abbreviated-terms"></a>

<a id="3Defsandabbrevs"></a>

## 3. Terms, definitions, and abbreviated terms

For the purposes of this specification the following definitions apply.

<a id="3byte"></a>

<a id="dfn-byte"></a>

byte

octet
:   8-bit binary integer in the range [0, 255] where the most significant bit is bit 7 and the least significant
    bit is bit 0.

<a id="3byteOrder"></a>

<a id="dfn-byte-order"></a>

byte order

<a id="ref-for-dfn-byte-1"></a>

:   ordering of [bytes](002-2-scope.md#dfn-byte) for multi-byte data values.

<a id="3chromaticity"></a>

<a id="dfn-chromaticity"></a>

chromaticity

<a id="issue-container-generatedID"></a>

<a id="h-note"></a>

:   pair of *x* and *y* values in the *xyY* space specified at [[COLORIMETRY](044-g-references.md#bib-colorimetry "Colorimetry, Fourth Edition. CIE 015:2018")].

    Note

    Chromaticity is a measure of the quality of a color regardless of its luminance.

<a id="3composite"></a>

<a id="dfn-composited"></a>

composite (verb)

<a id="issue-container-generatedID-0"></a>

<a id="h-note-0"></a>

:   form an image by merging a foreground image and a background image, using transparency information to determine
    where and to what extent the background should be visible.

    Note

    <a id="ref-for-dfn-composited-1"></a>

    The foreground image is said to be [composited](002-2-scope.md#dfn-composited) against the background.

<a id="3datastream"></a>

<a id="dfn-datastream"></a>

datastream

<a id="ref-for-dfn-byte-2"></a>

:   sequence of [bytes](002-2-scope.md#dfn-byte).

<a id="3deflate"></a>

<a id="dfn-deflate"></a>

deflate

<a id="ref-for-3LZ77-1"></a>

:   member of the [LZ77](002-2-scope.md#3LZ77) family of compression methods.

    SOURCE: [[RFC1951](044-g-references.md#bib-rfc1951 "DEFLATE Compressed Data Format Specification version 1.3")]

<a id="dfn-frame"></a>

frame

<a id="ref-for-dfn-static-image-1"></a>

:   For static PNG, the [static image](003-4-concepts.md#dfn-static-image)
    is considered to be the first (and only) frame.
    For animated PNG, each image that forms part
    of the [frame-based animation](007-4-9-apng-frame-based-animation.md#apng-frame-based-animation) sequence
    is a frame.
    Thus, for animated PNG, when the static image is not the first frame,
    the static image is not considered to be a frame.

<a id="3frameBuffer"></a>

<a id="dfn-frame-buffer"></a>

frame buffer

<a id="issue-container-generatedID-1"></a>

<a id="h-note-1"></a>

:   the final digital storage area for the image shown by most types of computer display.

    Note

    <a id="ref-for-dfn-frame-buffer-1"></a>

    Software causes an image to appear on screen by loading the image into the [frame buffer](002-2-scope.md#dfn-frame-buffer).

<a id="dfn-fully-transparent-black"></a>

fully transparent black
:   pixel where the red, green, blue and alpha components are all equal to zero.

<a id="dfn-gamma-value"></a>

gamma value

<a id="ref-for-dfn-gamma-1"></a>

<a id="ref-for-dfn-transfer-function-1"></a>

:   value of the exponent of a [gamma](002-2-scope.md#dfn-gamma) [transfer function](002-2-scope.md#dfn-transfer-function).

<a id="3gamma"></a>

<a id="dfn-gamma"></a>

gamma

<a id="ref-for-dfn-transfer-function-2"></a>

:   power-law [transfer function](002-2-scope.md#dfn-transfer-function).

<a id="dfn-hdr"></a>

high dynamic range (HDR)

<a id="ref-for-dfn-stop-1"></a>

<a id="ref-for-dfn-hdr-1"></a>

<a id="ref-for-dfn-hlg-1"></a>

<a id="ref-for-dfn-pq-1"></a>

:   an image format capable of storing images with a relatively high dynamic range similar to or in excess of the human visual system's instantaneous dynamic range (~12-14 [stops](002-2-scope.md#dfn-stop)). PNG allows the use of two [HDR](002-2-scope.md#dfn-hdr) formats, [HLG](002-2-scope.md#dfn-hlg) and [PQ](002-2-scope.md#dfn-pq) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")].

<a id="dfn-hlg"></a>

hybrid log-gamma (HLG)

<a id="ref-for-dfn-transfer-function-3"></a>

:   [transfer function](002-2-scope.md#dfn-transfer-function) defined in [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] Table 5. (A relative scene-referred system.)

<a id="dfn-full-range-image"></a>

full-range image
:   image where reference black and white correspond, respectively, to sample values `0` and `2<sup>bit depth</sup> -
    1`.

<a id="3imageData"></a>

<a id="dfn-image-data"></a>

image data

<a id="ref-for-dfn-scanline-1"></a>

:   1-dimensional array of [scanlines](002-2-scope.md#dfn-scanline) within an image.

<a id="3interlacedPNGimage"></a>

<a id="dfn-interlaced-png-image"></a>

interlaced PNG image

<a id="ref-for-3reducedImage-1"></a>

<a id="ref-for-3PNGimage-1"></a>

<a id="ref-for-3passExtraction-1"></a>

:   sequence of [reduced images](005-4-5-png-image.md#3reducedImage) generated from the [PNG image](003-4-concepts.md#3PNGimage) by [pass extraction](005-4-5-png-image.md#3passExtraction).

<a id="3losslessCompression"></a>

<a id="dfn-lossless"></a>

lossless
:   method of data compression that permits reconstruction of the original data exactly, bit-for-bit.

<a id="3luminance"></a>

<a id="dfn-luminance"></a>

luminance

<a id="issue-container-generatedID-2"></a>

<a id="h-note-2"></a>

:   an objective measurement of the visible light intensity, taking into account the sensitivity of the human eye to different wavelengths.

    Note

    <a id="ref-for-dfn-chromaticity-1"></a>

    Luminance and [chromaticity](002-2-scope.md#dfn-chromaticity) together fully define a measured color. For a formal definition, see [[COLORIMETRY](044-g-references.md#bib-colorimetry "Colorimetry, Fourth Edition. CIE 015:2018")].

<a id="3LZ77"></a>

LZ77
:   data compression algorithm described in [[Ziv-Lempel](044-g-references.md#bib-ziv-lempel "A Universal Algorithm for Sequential Data Compression, IEEE Transactions on Information Theory, vol. IT-23, no. 3, pp. 337 - 343")].

<a id="dfn-narrow-range-image"></a>

narrow-range image
:   Image where reference black and white do not correspond, respectively, to sample values `0` and
    `2<sup>bit depth</sup> - 1`.

<a id="3networkByteOrder"></a>

<a id="dfn-network-byte-order"></a>

network byte order

<a id="ref-for-dfn-byte-order-1"></a>

:   [byte order](002-2-scope.md#dfn-byte-order) in which the most significant byte comes first, then the less significant bytes in descending order of
    significance (MSB LSB for two-byte integers, MSB B2 B1 LSB for four-byte integers).

<a id="dfn-pq"></a>

perceptual quantizer (PQ)

<a id="ref-for-dfn-transfer-function-4"></a>

<a id="issue-container-generatedID-3"></a>

<a id="h-note-3"></a>

:   [transfer function](002-2-scope.md#dfn-transfer-function) defined in [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] Table 4. (An absolute display-referred system.)

    Note

    Only RGB may be used in PNG, ICtCp is NOT supported.

<a id="3PNGdecoder"></a>

PNG decoder

<a id="ref-for-3referenceImage-1"></a>

<a id="ref-for-3PNGdatastream-1"></a>

<a id="ref-for-3deliveredImage-1"></a>

:   process or device that reconstructs the [reference image](003-4-concepts.md#3referenceImage) from a [PNG datastream](008-4-10-error-handling.md#3PNGdatastream) and generates a
    corresponding [delivered image](003-4-concepts.md#3deliveredImage).

<a id="3PNGeditor"></a>

<a id="dfn-png-editor"></a>

PNG editor

<a id="ref-for-3PNGdatastream-2"></a>

<a id="ref-for-3chunk-1"></a>

:   process or device that creates a modification of an existing [PNG datastream](008-4-10-error-handling.md#3PNGdatastream), preserving unmodified ancillary
    information wherever possible, and obeying the [chunk](008-4-10-error-handling.md#3chunk) ordering rules, even for unknown chunk types.

<a id="3PNGencoder"></a>

PNG encoder

<a id="ref-for-3referenceImage-2"></a>

<a id="ref-for-3sourceImage-1"></a>

<a id="ref-for-3PNGdatastream-3"></a>

:   process or device which constructs a [reference image](003-4-concepts.md#3referenceImage) from a [source image](003-4-concepts.md#3sourceImage), and generates a [PNG
    datastream](008-4-10-error-handling.md#3PNGdatastream) representing the reference image.

<a id="3PNGfile"></a>

PNG file

<a id="ref-for-3PNGdatastream-4"></a>

:   [PNG datastream](008-4-10-error-handling.md#3PNGdatastream) stored as a file.

<a id="3PNGfourByteUnSignedInteger"></a>

<a id="dfn-png-four-byte-unsigned-integer"></a>

PNG four-byte unsigned integer

<a id="issue-container-generatedID-4"></a>

<a id="h-note-4"></a>

:   a four-byte unsigned integer limited to the range 0 to 2<sup>31</sup>-1.

    Note

    The restriction is imposed in order to accommodate languages that have difficulty with unsigned four-byte
    values.

<a id="3PNGtwoByteUnSignedInteger"></a>

<a id="dfn-png-two-byte-unsigned-integer"></a>

PNG two-byte unsigned integer
:   a two-byte unsigned integer in network byte order.

<a id="3sample"></a>

<a id="dfn-sample"></a>

sample

<a id="ref-for-3channel-1"></a>

<a id="ref-for-3pixel-1"></a>

:   intersection of a [channel](003-4-concepts.md#3channel) and a [pixel](003-4-concepts.md#3pixel) in an image.

<a id="3sampleDepth"></a>

sample depth

<a id="ref-for-dfn-sample-1"></a>

:   number of bits used to represent a [sample](002-2-scope.md#dfn-sample) value.

<a id="3scanline"></a>

<a id="dfn-scanline"></a>

scanline

<a id="ref-for-3pixel-2"></a>

<a id="ref-for-dfn-interlaced-png-image-1"></a>

:   row of [pixels](003-4-concepts.md#3pixel) within an image or [interlaced PNG image](002-2-scope.md#dfn-interlaced-png-image).

<a id="dfn-sdr"></a>

standard dynamic range (SDR)

<a id="ref-for-dfn-stop-2"></a>

<a id="issue-container-generatedID-5"></a>

<a id="h-note-5"></a>

:   an image format capable of storing images with a relatively low dynamic range of 5-8 [stops](002-2-scope.md#dfn-stop). Examples include [[SRGB](044-g-references.md#bib-srgb "Multimedia systems and equipment - Colour measurement and management - Part 2-1: Colour management - Default RGB colour space - sRGB")], [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")], [[ITU-R-BT.709](044-g-references.md#bib-itu-r-bt.709 "ITU-R BT.709, SERIES BT: BROADCASTING SERVICE (TELEVISION). Parameter values for the HDTV standards for production and international programme exchange")].

    Note

    <a id="ref-for-dfn-sdr-1"></a>

    Standard dynamic range is independent of the primaries and hence, gamut. Wide color gamut [SDR](002-2-scope.md#dfn-sdr) formats are supported by PNG.

<a id="dfn-stop"></a>

stop
:   a change in scene light luminance of a factor of 2.

<a id="dfn-transfer-function"></a>

transfer function
:   function relating image luminance with image samples.

<a id="3whitePoint"></a>

<a id="dfn-white-point"></a>

white point

<a id="ref-for-dfn-chromaticity-2"></a>

:   [chromaticity](002-2-scope.md#dfn-chromaticity) of a computer display's nominal white value.

<a id="3zlib"></a>

<a id="dfn-zlib"></a>

zlib

<a id="issue-container-generatedID-6"></a>

<a id="h-note-6"></a>

:   <a id="ref-for-dfn-deflate-1"></a>

    [deflate](002-2-scope.md#dfn-deflate)-style compression method.

    SOURCE: [[rfc1950](044-g-references.md#bib-rfc1950 "ZLIB Compressed Data Format Specification version 3.3")]

    Note

    Also refers to the name of a library containing a sample implementation of this method.

<a id="3CRC"></a>

Cyclic Redundancy Code

CRC

<a id="issue-container-generatedID-7"></a>

<a id="h-note-7"></a>

:   type of check value designed to detect most transmission errors.

    Note

