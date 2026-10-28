# 4. Concepts

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](002-2-scope.md) · [Next](004-4-3-color-spaces.md)

    A decoder calculates the CRC for the received data and checks by comparing it to the CRC calculated by
    the encoder and appended to the data. A mismatch indicates that the data or the CRC were corrupted in transit.

<a id="3CRT"></a>

Cathode Ray Tube

CRT
:   vacuum tube containing one or more electron guns, which emit electron beams that are manipulated to display images on a
    phosphorescent screen.

Electro-Optical Transfer Function

<a id="dfn-eotf"></a>

EOTF

<a id="ref-for-dfn-transfer-function-5"></a>

:   The [transfer function](002-2-scope.md#dfn-transfer-function) between the electrical or digital domain and light energy. It defines the amount of light emitted by a display for a given input signal.

<a id="3LSB"></a>

Least Significant Byte

LSB

<a id="ref-for-dfn-byte-3"></a>

:   Least significant byte of a multi-[byte](002-2-scope.md#dfn-byte) value.

<a id="3MSB"></a>

Most Significant Byte

MSB

<a id="ref-for-dfn-byte-4"></a>

:   Most significant byte of a multi-[byte](002-2-scope.md#dfn-byte) value.

Opto-Electrical Transfer Function

<a id="dfn-oetf"></a>

OETF

<a id="ref-for-dfn-transfer-function-6"></a>

:   The [transfer function](002-2-scope.md#dfn-transfer-function) between light energy and the electrical or digital domain. It defines the amount of light in a scene required to produce a given output signal.

<a id="concepts"></a>

<a id="4Concepts"></a>

## 4. Concepts

<a id="static-and-animated-images"></a>

<a id="x4-1-static-and-animated-images"></a>

### 4.1 Static and Animated images

<a id="dfn-static-image"></a>

All PNG images contain a single static image.

<a id="dfn-animated-png"></a>

<a id="dfn-animated-image"></a>

<a id="ref-for-dfn-static-image-2"></a>

<a id="ref-for-dfn-static-image-3"></a>

Some PNG images — called Animated PNG (APNG) — also
contain a frame-based animation sequence, the animated image. The first frame of this may be — but need not be —
the [static image](003-4-concepts.md#dfn-static-image). Non-animation-capable displays (such as printers) will display the [static image](003-4-concepts.md#dfn-static-image) rather than
the animation sequence.

<a id="ref-for-dfn-static-image-4"></a>

<a id="ref-for-dfn-animated-image-1"></a>

The [static image](003-4-concepts.md#dfn-static-image), and each individual frame of an [animated image](003-4-concepts.md#dfn-animated-image), corresponds to a *reference image*
and is stored as a *PNG image*.

<a id="images"></a>

<a id="4Concepts.Sourceimage"></a>

### 4.2 Images

<a id="ref-for-dfn-png-editor-1"></a>

This specification specifies the PNG datastream, and places some requirements on PNG encoders, which generate PNG
datastreams, PNG decoders, which interpret PNG datastreams, and [PNG editors](002-2-scope.md#dfn-png-editor), which transform one PNG datastream into
another. It does not specify the interface between an application and either a PNG encoder, decoder, or editor. The precise
form in which an image is presented to an encoder or delivered by a decoder is not specified. Four kinds of image are
distinguished.

<a id="3sourceImage"></a>

Source image
:   The *source image* is the image presented to a PNG encoder.

<a id="3referenceImage"></a>

Reference image

<a id="ref-for-3referenceImage-3"></a>

<a id="3pixel"></a>

<a id="3channel"></a>

<a id="3alpha"></a>

<a id="ref-for-3PNGdatastream-5"></a>

<a id="ref-for-3PNGdatastream-6"></a>

:   The [reference image](003-4-concepts.md#3referenceImage), which only exists conceptually, is a rectangular array of rectangular pixels, all having
    the same width and height, and all containing the same number of unsigned integer samples, either three (red, green, blue)
    or four (red, green, blue, alpha). The array of all samples of a particular kind (red, green, blue, or alpha) is called a
    channel. Each channel has a sample depth in the range 1 to 16, which is the number of bits used by every sample in the
    channel. Different channels may have different sample depths. The red, green, and blue samples determine the intensities of
    the red, green, and blue components of the pixel's color; if they are all zero, the pixel is black, and if they all have
    their maximum values (2<sup>sampledepth</sup>-1), the pixel is white. The alpha sample determines a pixel's degree of
    opacity, where zero means fully transparent and the maximum value means fully opaque. In a three-channel reference image
    all pixels are fully opaque. (It is also possible for a four-channel reference image to have all pixels fully opaque; the
    difference is that the latter has a specific alpha sample depth, whereas the former does not.) Each horizontal row of
    pixels is called a scanline. Pixels are ordered from left to right within each scanline, and scanlines are ordered from top
    to bottom. Every reference image can be represented exactly by a [PNG datastream](008-4-10-error-handling.md#3PNGdatastream) and every [PNG datastream](008-4-10-error-handling.md#3PNGdatastream) can be converted into a reference image. A PNG encoder may transform the source image directly into a PNG image, but conceptually it first transforms the
    source image into a reference image, then transforms the reference image into a PNG image. Depending on the type of source
    image, the transformation from the source image to a reference image may require the loss of information. That
    transformation is beyond the scope of this International Standard. The reference image, however, can always be recovered
    exactly from a PNG datastream.

<a id="3PNGimage"></a>

PNG image

<a id="ref-for-3alphaSeparation-1"></a>

<a id="ref-for-3indexing-1"></a>

<a id="ref-for-3RGBmerging-1"></a>

<a id="ref-for-3alphaCompaction-1"></a>

<a id="ref-for-3sampleDepthScaling-1"></a>

:   The *PNG image* is obtained from the reference image by a series of transformations: [alpha separation](004-4-3-color-spaces.md#3alphaSeparation),
    [indexing](004-4-3-color-spaces.md#3indexing), [RGB merging](004-4-3-color-spaces.md#3RGBmerging), [alpha compaction](004-4-3-color-spaces.md#3alphaCompaction), and [sample depth scaling](004-4-3-color-spaces.md#3sampleDepthScaling). Five types of PNG image are defined
    (see [6.1 Color types and values](010-5-7-defining-chunks.md#6Colour-values)). (If the PNG encoder actually transforms the source image directly into the PNG image,
    and the source image format is already similar to the PNG image format, the encoder may be able to avoid doing some of
    these transformations.) Although not all sample depths in the range 1 to 16 bits are explicitly supported in the PNG image,
    the number of significant bits in each channel of the reference image may be recorded. All channels in the PNG image have
    the same sample depth. A PNG encoder generates a PNG datastream from the PNG image. A PNG decoder takes the PNG datastream
    and recreates the PNG image.

<a id="3deliveredImage"></a>

Delivered image

<a id="ref-for-3deliveredImage-2"></a>

<a id="ref-for-3deliveredImage-3"></a>

:   The [delivered image](003-4-concepts.md#3deliveredImage) is constructed from the PNG image obtained by decoding a PNG datastream. No specific format
    is specified for the [delivered image](003-4-concepts.md#3deliveredImage). A viewer presents an image to the user as close to the appearance of the
    original source image as it can achieve.

The relationships between the four kinds of image are illustrated in [Figure 1](003-4-concepts.md#image-relationship "Relationships between source, reference, PNG, and display images").

<a id="image-relationship"></a>

<a id="figure41"></a>

![figure41](../../../../assets/web/32027a064299-image-relationships.svg)



[Figure 1](003-4-concepts.md#image-relationship) 
Relationships between source, reference, PNG, and display images

The relationships between samples, channels, pixels, and sample depth are illustrated in [Figure 2](003-4-concepts.md#sample-pixel-channel-relationship "Relationships between sample, sample depth, pixel, and channel").

<a id="sample-pixel-channel-relationship"></a>

<a id="figure42"></a>

![figure42](../../../../assets/web/7462f9303288-sample-pixel-channel-relationship.svg)



[Figure 2](003-4-concepts.md#sample-pixel-channel-relationship) 
Relationships between sample, sample depth, pixel, and channel

