# 13.10 Interlacing and progressive display

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](027-13-6-pixel-dimensions.md) · [Next](029-13-13-decoder-gamma-handling.md)

<a id="13Progressive-display"></a>

<a id="x13-10-interlacing-and-progressive-display"></a>

### 13.10 Interlacing and progressive display

Decoders are required to be able to read interlaced images. If the reference image contains fewer than five columns or
fewer than five rows, some passes will be empty. Encoders and decoders shall handle this case correctly. In particular,
filter type bytes are associated only with nonempty scanlines; no filter type bytes are present in an empty reduced
image.

When receiving images over slow transmission links, viewers can improve perceived performance by displaying interlaced
images progressively. This means that as each reduced image is received, an approximation to the complete image is displayed
based on the data received so far. One simple yet pleasing effect can be obtained by expanding each received pixel to fill a
rectangle covering the yet-to-be-transmitted pixel positions below and to the right of the received pixel. This process can
be described by the following ISO C code [[ISO\_9899](044-g-references.md#bib-iso_9899 "ISO/IEC 9899:2018 Information technology — Programming languages — C")]:



```
/*
    variables declared and initialized elsewhere in the code:
        height, width
    functions or macros defined elsewhere in the code:
        visit(), min()
 */

int starting_row[7]  = { 0, 0, 4, 0, 2, 0, 1 };
int starting_col[7]  = { 0, 4, 0, 2, 0, 1, 0 };
int row_increment[7] = { 8, 8, 8, 4, 4, 2, 2 };
int col_increment[7] = { 8, 8, 4, 4, 2, 2, 1 };
int block_height[7]  = { 8, 8, 4, 4, 2, 2, 1 };
int block_width[7]   = { 8, 4, 4, 2, 2, 1, 1 };

int pass;
long row, col;

pass = 0;
while (pass < 7)
{
    row = starting_row[pass];
    while (row < height)
    {
        col = starting_col[pass];
        while (col < width)
        {
            visit(row, col,
                  min(block_height[pass], height - row),
                  min(block_width[pass], width - col));
            col = col + col_increment[pass];
        }
        row = row + row_increment[pass];
    }
    pass = pass + 1;
}
```




The function `visit(row,column,height,width)` obtains the next transmitted pixel and paints a rectangle of the
specified height and width, whose upper-left corner is at the specified row and column, using the color indicated by the
pixel. Note that row and column are measured from 0,0 at the upper left corner.

If the viewer is merging the received image with a background image, it may be more convenient just to paint the received
pixel positions (the `visit()` function sets only the pixel at the specified row and column, not the whole
rectangle). This produces a "fade-in" effect as the new image gradually replaces the old. An advantage of this approach is
that proper alpha or transparency processing can be done as each pixel is replaced. Painting a rectangle as described above
will overwrite background-image pixels that may be needed later, if the pixels eventually received for those positions turn
out to be wholly or partially transparent. This is a problem only if the background image is not stored anywhere
offscreen.

<a id="13Truecolour-image-handling"></a>

<a id="x13-11-truecolor-image-handling"></a>

### 13.11 Truecolor image handling

<a id="ref-for-3indexedColour-18"></a>

<a id="ref-for-3truecolour-23"></a>

<a id="ref-for-dfn-greyscale-16"></a>

<a id="ref-for-3truecolour-24"></a>

To achieve PNG's goal of universal interchangeability, decoders shall accept all types of PNG image: [indexed-color](004-4-3-color-spaces.md#3indexedColour),
[truecolor](005-4-5-png-image.md#3truecolour), and [greyscale](005-4-5-png-image.md#dfn-greyscale). Viewers running on indexed-color display hardware need to be able to reduce
[truecolor](005-4-5-png-image.md#3truecolour) images to indexed-color for viewing. This process is called "color quantization".

A simple, fast method for color quantization is to reduce the image to a fixed palette. Palettes with uniform color
spacing ("color cubes") are usually used to minimize the per-pixel computation. For photograph-like images, dithering is
recommended to avoid ugly contours in what should be smooth gradients; however, dithering introduces graininess that can be
objectionable.

The quality of rendering can be improved substantially by using a palette chosen specifically for the image, since a
color cube usually has numerous entries that are unused in any particular image. This approach requires more work, first in
choosing the palette, and second in mapping individual pixels to the closest available color. PNG allows the encoder to
supply suggested palettes, but not all encoders will do so, and the suggested palettes may be unsuitable in any case (they
may have too many or too few colors). Therefore, high-quality viewers will need to have a palette selection routine at hand.
A large lookup table is usually the most feasible way of mapping individual pixels to palette entries with adequate
speed.

Numerous implementations of color quantization are available. The PNG sample implementation, libpng ([`http://www.libpng.org/pub/png/libpng.html`](http://www.libpng.org/pub/png/libpng.html)), includes code for
the purpose.

<a id="13Sample-depth-rescaling"></a>

<a id="x13-12-sample-depth-rescaling"></a>

### 13.12 Sample depth rescaling

Decoders may wish to scale PNG data to a lesser sample depth (data precision) for display. For example, 16-bit data will
need to be reduced to 8-bit depth for use on most present-day display hardware. Reduction of 8-bit data to 5-bit depth is
also common.

The most accurate scaling is achieved by the linear equation

`output = floor((input * MAXOUTSAMPLE / MAXINSAMPLE) + 0.5)`

where

`MAXINSAMPLE = (2<sup>sampledepth</sup>)-1`  
`MAXOUTSAMPLE = (2<sup>desired_sampledepth</sup>)-1`

<a id="ref-for-dfn-gamma-13"></a>

<a id="ref-for-dfn-gamma-14"></a>

A slightly less accurate conversion is achieved by simply shifting right by `(sampledepth -
desired_sampledepth)` places. For example, to reduce 16-bit samples to 8-bit, the low-order byte can be discarded. In
many situations the shift method is sufficiently accurate for display purposes, and it is certainly much faster. (But if
[gamma](002-2-scope.md#dfn-gamma) correction is being done, sample rescaling can be merged into the [gamma](002-2-scope.md#dfn-gamma) correction lookup table, as is
illustrated in [13.13 Decoder gamma handling](029-13-13-decoder-gamma-handling.md#13Decoder-gamma-handling).)

<a id="ref-for-dfn-frame-buffer-2"></a>

If the decoder needs to scale samples up (for example, if the [frame buffer](002-2-scope.md#dfn-frame-buffer) has a greater sample depth than the PNG
image), it should use linear scaling or left-bit-replication as described in [12.4 Sample depth scaling](023-12-3-alpha-channel-creation.md#12Sample-depth-scaling).

<a id="ref-for-dfn-image-data-25"></a>

When an [sBIT](015-11-3-ancillary-chunks.md#11sBIT) chunk is present, the reference [image data](002-2-scope.md#dfn-image-data) can be recovered by
shifting right to the sample depth specified by [sBIT](015-11-3-ancillary-chunks.md#11sBIT). Note that linear scaling will not
necessarily reproduce the original data, because the encoder is not required to have used linear scaling to scale the data
up. However, the encoder is required to have used a method that preserves the high-order bits, so shifting always works. This
is the only case in which shifting might be said to be more accurate than linear scaling. A decoder need not pay attention to
the [sBIT](015-11-3-ancillary-chunks.md#11sBIT) chunk; the stored image is a valid PNG datastream of the sample depth indicated
by the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk; however, using [sBIT](015-11-3-ancillary-chunks.md#11sBIT) to recover the
original samples before scaling them to suit the display often yields a more accurate display than ignoring [sBIT](015-11-3-ancillary-chunks.md#11sBIT).

<a id="tRNS-compare-exactly"></a>

When comparing pixel values to [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk values to detect transparent pixels, the
comparison shall be done exactly. Therefore, transparent pixel detection shall be done before reducing sample precision.

