# 4.9 APNG: frame-based animation

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](006-4-7-additional-information.md) · [Next](008-4-10-error-handling.md)

<a id="apng-frame-based-animation"></a>

<a id="x4-9-apng-frame-based-animation"></a>

### 4.9 APNG: frame-based animation

<a id="introduction-0"></a>

<a id="introduction-3"></a>

### Introduction

<a id="ref-for-dfn-frame-1"></a>

Animated PNG (APNG) extends the original, static-only PNG format,
adding support for [frame](002-2-scope.md#dfn-frame)-based animated images. It is intended to
be a replacement for simple animated images that have traditionally used the GIF format [[GIF](045-g-2-informative-references.md#bib-gif "Graphics Interchange Format")], while adding support for
24-bit images and 8-bit transparency, which GIF lacks.

<a id="ref-for-dfn-static-image-6"></a>

APNG is backwards-compatible with earlier versions of PNG; a non-animated PNG decoder will ignore the ancillary
APNG-specific chunks and display the [static image](003-4-concepts.md#dfn-static-image).

<a id="structure"></a>

<a id="x4-9-1-structure"></a>

#### 4.9.1 Structure

An APNG stream is a normal PNG stream as defined in previous versions of the PNG Specification, with three additional
chunk types describing the animation and providing additional frame data.

To be recognized as an APNG, an [acTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#acTL-chunk) chunk must appear in the stream before any
[IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks. The [acTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#acTL-chunk) structure is [described below](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#animation-information).

<a id="ref-for-dfn-output-buffer-1"></a>

<a id="ref-for-dfn-fully-transparent-black-1"></a>

Conceptually, at the beginning of each play the [output buffer](007-4-9-apng-frame-based-animation.md#dfn-output-buffer) shall be completely initialized to a [fully
transparent black](002-2-scope.md#dfn-fully-transparent-black) rectangle, with width and height dimensions from the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR)
chunk.

The static image may be included as the first frame of the animation by the presence of a single [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) chunk before [IDAT](014-10-3-other-uses-of-compression.md#11IDAT). Otherwise, the static image is not part of
the animation.

Subsequent frames are encoded in [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) chunks, which have the same structure as
[IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks, except preceded by a [sequence
number](007-4-9-apng-frame-based-animation.md#4Concepts.APNGSequence). Information for each frame about placement and rendering is stored in [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) chunks. The full layout of [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) and [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) chunks is [described below](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#animation-information).

<a id="ref-for-dfn-fully-transparent-black-2"></a>

The boundaries of the entire animation are specified by the width and height parameters of the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk, regardless of whether the default image is part of the animation. The default image should be
appropriately padded with [fully transparent black](002-2-scope.md#dfn-fully-transparent-black) pixels if extra space will be needed for later frames.

Each frame is identical for each play, therefore it is safe for applications to cache the frames.

<a id="4Concepts.APNGSequence"></a>

<a id="x4-9-2-sequence-numbers"></a>

#### 4.9.2 Sequence numbers

The [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) and [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) chunks have a
zero-based, 4 byte sequence number. Both chunk types share the sequence. The purpose of this number is to detect (and
optionally correct) sequence errors in an Animated PNG, since this specification does not impose ordering restrictions
on ancillary chunks.

The first [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) chunk shall contain sequence number 0, and the sequence numbers
in the remaining [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) and [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) chunks shall
be in ascending order, with no gaps or duplicates.

The tables below illustrate the use of sequence numbers for images with more than one frame, and more than one [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) chunk for the second frame. ([IHDR](014-10-3-other-uses-of-compression.md#11IHDR) and [IEND](014-10-3-other-uses-of-compression.md#11IEND) chunks omitted in these tables, for clarity).

<a id="table-if-the-static-image-is-also-the-first-frame"></a>

[Table 3](007-4-9-apng-frame-based-animation.md#table-if-the-static-image-is-also-the-first-frame) 
If the static image is also the first frame

| Sequence number | Chunk |
| --- | --- |
| (none) | [acTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#acTL-chunk) |
| 0 | [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) first frame |
| (none) | [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) first frame / static image |
| 1 | [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) second frame |
| 2 | first [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) for second frame |
| 3 | second [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) for second frame |

<a id="table-if-the-static-image-is-not-part-of-the-animation"></a>

[Table 4](007-4-9-apng-frame-based-animation.md#table-if-the-static-image-is-not-part-of-the-animation) 
If the static image is not part of the animation

| Sequence number | Chunk |
| --- | --- |
| (none) | [acTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#acTL-chunk) |
| (none) | [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) static image |
| 0 | [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) first frame |
| 1 | first [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) for first frame |
| 2 | second [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) for first frame |
| 3 | [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) second frame |
| 4 | first [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) for second frame |
| 5 | second [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) for second frame |

<a id="apng-output-buffer"></a>

<a id="x4-9-3-output-buffer"></a>

#### 4.9.3 Output buffer

<a id="dfn-output-buffer"></a>

<a id="ref-for-dfn-composited-2"></a>

<a id="ref-for-dfn-canvas-1"></a>

<a id="ref-for-dfn-canvas-2"></a>

The output buffer is a pixel array with dimensions specified by the width and height parameters of the PNG [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk. Conceptually, each frame is constructed in the output buffer before being
[composited](002-2-scope.md#dfn-composited) onto the [canvas](007-4-9-apng-frame-based-animation.md#dfn-canvas). The contents of the output buffer are available to the decoder. The corners of
the output buffer are mapped to the corners of the [canvas](007-4-9-apng-frame-based-animation.md#dfn-canvas).

<a id="apng-canvas"></a>

<a id="x4-9-4-canvas"></a>

#### 4.9.4 Canvas

<a id="dfn-canvas"></a>

The canvas is the area on the output device on which the frames are to be displayed. The contents of the
canvas are not necessarily available to the decoder. If a [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk exists, it may be
used to fill the canvas if there is no preferable background.

