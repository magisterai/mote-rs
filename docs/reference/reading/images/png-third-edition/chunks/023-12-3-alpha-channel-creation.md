# 12.3 Alpha channel creation

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](022-12-2-encoder-color-handling.md) · [Next](024-12-6-interlacing.md)

<a id="12Alpha-channel-creation"></a>

<a id="x12-3-alpha-channel-creation"></a>

### 12.3 Alpha channel creation

The alpha channel can be regarded either as a mask that temporarily hides transparent parts of the image, or as a means
for constructing a non-rectangular image. In the first case, the color values of fully transparent pixels should be
preserved for future use. In the second case, the transparent pixels carry no useful data and are simply there to fill out
the rectangular image area required by PNG. In this case, fully transparent pixels should all be assigned the same color
value for best compression.

Image authors should keep in mind the possibility that a decoder will not support transparency control in full (see
[13.16 Alpha channel processing](031-13-16-alpha-channel-processing.md#13Alpha-channel-processing)). Hence, the colors assigned to transparent pixels should be reasonable
background colors whenever feasible.

For applications that do not require a full alpha channel, or cannot afford the price in compression efficiency, the
[tRNS](015-11-3-ancillary-chunks.md#11tRNS) transparency chunk is also available.

If the image has a known background color, this color should be written in the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD)
chunk. Even decoders that ignore transparency may use the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) color to fill unused
screen area.

If the original image has premultiplied (also called "associated") alpha data, it can be converted to PNG's
non-premultiplied format by dividing each sample value by the corresponding alpha value, then multiplying by the maximum
value for the image bit depth, and rounding to the nearest integer. In valid premultiplied data, the sample values never
exceed their corresponding alpha values, so the result of the division should always be in the range 0 to 1. If the alpha
value is zero, output black (zeroes).

<a id="12Sample-depth-scaling"></a>

<a id="x12-4-sample-depth-scaling"></a>

### 12.4 Sample depth scaling

When encoding input samples that have a sample depth that cannot be directly represented in PNG, the encoder shall scale
the samples up to a sample depth that is allowed by PNG. The most accurate scaling method is the linear equation:



```
output = floor((input * MAXOUTSAMPLE / MAXINSAMPLE) + 0.5)
```




where the input samples range from 0 to MAXINSAMPLE and the outputs range from 0 to MAXOUTSAMPLE
(which is 2<sup>sampledepth</sup>-1).

A close approximation to the linear scaling method is achieved by "left bit replication", which is shifting the valid bits
to begin in the most significant bit and repeating the most significant bits into the open bits. This method is often faster
to compute than linear scaling.

Assume that 5-bit samples are being scaled up to 8 bits. If the source sample value is 27 (in the range from
0-31), then the original bits are:



```
4 3 2 1 0
---------
1 1 0 1 1
```




Left bit replication gives a value of 222:



```
7 6 5 4 3  2 1 0
----------------
1 1 0 1 1  1 1 0
|=======|  |===|
    |      Leftmost Bits Repeated to Fill Open Bits
    |
Original Bits
```




which matches the value computed by the linear equation. Left bit replication usually gives the same value as linear
scaling, and is never off by more than one.

A distinctly less accurate approximation is obtained by simply left-shifting the input value and filling the low order
bits with zeroes. This scheme cannot reproduce white exactly, since it does not generate an all-ones maximum value; the net
effect is to darken the image slightly. This method is not recommended in general, but it does have the effect of improving
compression, particularly when dealing with greater-than-8-bit sample depths. Since the relative error introduced by
zero-fill scaling is small at high sample depths, some encoders may choose to use it. Zero-fill shall **not** be
used for alpha channel data, however, since many decoders will treat alpha values of all zeroes and all ones as special
cases. It is important to represent both those values exactly in the scaled data.

When the encoder writes an [sBIT](015-11-3-ancillary-chunks.md#11sBIT) chunk, it is required to do the scaling in such a way
that the high-order bits of the stored samples match the original data. That is, if the [sBIT](015-11-3-ancillary-chunks.md#11sBIT) chunk specifies a sample depth of S, the high-order S bits of the stored data shall agree with the
original S-bit data values. This allows decoders to recover the original data by shifting right. The added low-order bits are
not constrained. All the above scaling methods meet this restriction.

<a id="ref-for-dfn-image-data-17"></a>

When scaling up source [image data](002-2-scope.md#dfn-image-data), it is recommended that the low-order bits be filled consistently for all
samples; that is, the same source value should generate the same sample value at any pixel position. This improves
compression by reducing the number of distinct sample values. This is not a mandatory requirement, and some encoders may
choose not to follow it. For example, an encoder might instead dither the low-order bits, improving displayed image quality
at the price of increasing file size.

In some applications the original source data may have a range that is not a power of 2. The linear scaling equation still
works for this case, although the shifting methods do not. It is recommended that an [sBIT](015-11-3-ancillary-chunks.md#11sBIT)
chunk not be written for such images, since [sBIT](015-11-3-ancillary-chunks.md#11sBIT) suggests that the original data range
was exactly 0..2<sup>S</sup>-1.

<a id="12Suggested-palettes"></a>

<a id="x12-5-suggested-palettes"></a>

### 12.5 Suggested palettes

<a id="ref-for-3truecolour-15"></a>

<a id="ref-for-dfn-image-data-18"></a>

<a id="ref-for-3truecolour-16"></a>

Suggested palettes may appear as [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunks in any PNG datastream, or as a [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk in [truecolor](005-4-5-png-image.md#3truecolour) PNG datastreams. In either case, the suggested palette is not an
essential part of the [image data](002-2-scope.md#dfn-image-data), but it may be used to present the image on indexed-color display hardware.
Suggested palettes are of no interest to viewers running on [truecolor](005-4-5-png-image.md#3truecolour) hardware.

<a id="ref-for-dfn-composited-7"></a>

When an [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk is used to provide a suggested palette, it is recommended that the
encoder use the frequency fields to indicate the relative importance of the palette entries, rather than leave them all zero
(meaning undefined). The frequency values are most easily computed as "nearest neighbor" counts, that is, the approximate
usage of each RGBA palette entry if no dithering is applied. (These counts will often be available "for free" as a
consequence of developing the suggested palette.) Because the suggested palette includes transparency information, it should
be computed for the un-[composited](002-2-scope.md#dfn-composited) image.

<a id="ref-for-3colourType-33"></a>

Even for indexed-color images, [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) can be used to define alternative reduced
palettes for viewers that are unable to display all the colors present in the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE)
chunk. If the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk appears without the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD)
chunk in an image of [color type](010-5-7-defining-chunks.md#3colourType) 6, the circumstances under which the palette was computed are unspecified.

<a id="ref-for-3truecolour-17"></a>

<a id="ref-for-3colourType-34"></a>

<a id="ref-for-dfn-truecolor-with-alpha-5"></a>

<a id="ref-for-dfn-png-editor-4"></a>

<a id="ref-for-3colourType-35"></a>

An older method for including a suggested palette in a [truecolor](005-4-5-png-image.md#3truecolour) PNG datastream uses the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk. If this method is used, the histogram (frequencies) should appear in a separate [hIST](019-11-3-3-4-itxt-international-textual-data.md#11hIST) chunk. The [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk does not include transparency information.
Hence for images of [color type](010-5-7-defining-chunks.md#3colourType) 6 ([truecolor with alpha](005-4-5-png-image.md#dfn-truecolor-with-alpha)), it is recommended that a [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk appear and that the palette and histogram be computed with reference to the image as it would appear
after compositing against the specified background color. This definition is necessary to ensure that useful palette entries
are generated for pixels having fractional alpha values. The resulting palette will probably be useful only to viewers that
present the image against the same background color. It is recommended that [PNG editors](002-2-scope.md#dfn-png-editor) delete or recompute the
palette if they alter or remove the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk in an image of [color type](010-5-7-defining-chunks.md#3colourType) 6.

<a id="ref-for-3colourType-36"></a>

<a id="ref-for-3truecolour-18"></a>

For images of [color type](010-5-7-defining-chunks.md#3colourType) 2 ([truecolor](005-4-5-png-image.md#3truecolour)), it is recommended that the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) and [hIST](019-11-3-3-4-itxt-international-textual-data.md#11hIST) chunks be computed with reference to the RGB data only,
ignoring any transparent-color specification. If the datastream uses transparency (has a [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk), viewers can easily adapt the resulting palette for use with their intended background color (see
[13.17 Histogram and suggested palette usage](032-13-17-histogram-and-suggested-palette-usage.md#13Histogram-and-suggested-palette-usage)).

For providing suggested palettes, the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk is more flexible than the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk in the following ways:

1. With [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) multiple suggested palettes may be provided. A PNG decoder may choose an
   appropriate palette based on name or number of entries.

<a id="ref-for-3colourType-37"></a>

<a id="ref-for-dfn-truecolor-with-alpha-6"></a>

<a id="ref-for-dfn-composited-8"></a>

<a id="ref-for-dfn-composited-9"></a>

2. In a PNG datastream of [color type](010-5-7-defining-chunks.md#3colourType) 6 ([truecolor with alpha](005-4-5-png-image.md#dfn-truecolor-with-alpha) channel), the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk represents a palette already [composited](002-2-scope.md#dfn-composited) against the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) color, so it is useful only for display against that background color. The [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk provides an un-[composited](002-2-scope.md#dfn-composited) palette, which is useful for display against backgrounds chosen
   by the PNG decoder.

<a id="ref-for-dfn-png-editor-5"></a>

3. Since the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk is an ancillary chunk, a [PNG editor](002-2-scope.md#dfn-png-editor) may add or modify
   suggested palettes without being forced to discard unknown unsafe-to-copy chunks.

<a id="ref-for-3colourType-38"></a>

<a id="ref-for-dfn-greyscale-13"></a>

<a id="ref-for-3indexedColour-16"></a>

4. Whereas the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk is allowed in PNG datastreams for [color types](010-5-7-defining-chunks.md#3colourType) 0, 3,
   and 4 ([greyscale](005-4-5-png-image.md#dfn-greyscale) and [indexed-color](004-4-3-color-spaces.md#3indexedColour)), the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk cannot be used to provide reduced
   palettes in these cases.
5. More than 256 entries may appear in the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk.

A PNG encoder that uses the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk may choose to write a suggested palette
represented by [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) and [hIST](019-11-3-3-4-itxt-international-textual-data.md#11hIST) chunks as well, for
compatibility with decoders that do not recognize the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunk.

