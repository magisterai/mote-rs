# 12. PNG Encoders

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md) · [Next](022-12-2-encoder-color-handling.md)

<a id="ref-for-dfn-image-data-16"></a>

<a id="ref-for-dfn-static-image-7"></a>

The fdAT chunk serves the same purpose for animations as the [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks do for static images;
the set of fdAT chunks
contains the [image data](002-2-scope.md#dfn-image-data) for all frames (or, for animations
which include the [static image](003-4-concepts.md#dfn-static-image) as first frame, for all frames after the first one). It contains:

<a id="fdAT-structure"></a>

[Table 28](021-12-png-encoders.md#fdAT-structure) 
fdAT chunk contents

| Name | Size |
| --- | --- |
| `sequence_number` | 4 bytes |
| `frame_data` | *n* bytes |

At least one fdAT chunk is required for each frame, except for the first frame, if that
frame is represented by an [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk.

The compressed datastream for each frame is then the concatenation,
in ascending [sequence number](007-4-9-apng-frame-based-animation.md#4Concepts.APNGSequence) order,
of the contents of the `frame_data` fields of all the fdAT chunks within a frame.

<a id="ref-for-3colourType-32"></a>

<a id="ref-for-3filter-12"></a>

<a id="ref-for-dfn-static-image-8"></a>

Because of the sequence number, fdAT chunks
[may not be of zero length](008-4-10-error-handling.md#zero-length-data));
however the `frame_data` fields may be of zero length.
When decompressed, the datastream is the complete pixel data of a PNG image,
including the filter byte at the beginning of each scanline, similar to the uncompressed data of all the [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks. It utilizes the same bit depth, [color type](010-5-7-defining-chunks.md#3colourType), compression method, [filter
method](012-7-3-filtering.md#3filter), interlace method, and palette (if any) as the [static image](003-4-concepts.md#dfn-static-image).

Each frame inherits every property specified by any critical or ancillary chunks *before* the first [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk in the file, except the width and height, which come from the fcTL chunk.

<a id="ref-for-dfn-canvas-3"></a>

If the PNG [pHYs](019-11-3-3-4-itxt-international-textual-data.md#11pHYs) chunk is present, the APNG images and their `x_offset` and
`y_offset` values must be scaled in the same way as the main image. Conceptually, such scaling occurs while mapping the
output buffer onto the [canvas](007-4-9-apng-frame-based-animation.md#dfn-canvas).

<a id="issue-container-generatedID-21"></a>

<a id="h-note-21"></a>

Note

For Web compatibility, due to the long time between the development and deployment of this chunk and it's
incorporation into the PNG specification, this chunk name is exceptionally defined as if it were a private chunk.

<a id="12Encoders"></a>

<a id="x12-png-encoders"></a>

## 12. PNG Encoders

<a id="12Introduction"></a>

<a id="introduction-8"></a>

### Introduction

This clause gives requirements and recommendations for encoder behavior. A PNG encoder shall produce a PNG datastream
from a PNG image that conforms to the format specified in the preceding clauses. Best results will usually be achieved by
following the additional recommendations given here.

<a id="12Encoder-gamma-handling"></a>

<a id="x12-1-encoder-gamma-handling"></a>

### 12.1 Encoder gamma handling

<a id="ref-for-dfn-gamma-4"></a>

See [C. Gamma and chromaticity](035-c-gamma-and-chromaticity.md#C-GammaAppendix) for a brief introduction to [gamma](002-2-scope.md#dfn-gamma) issues.

<a id="ref-for-dfn-gamma-5"></a>

PNG encoders capable of full color management will perform more sophisticated calculations than those described here and
may choose to use the [iCCP](015-11-3-ancillary-chunks.md#11iCCP) chunk. If it is known that the image samples conform to the
sRGB specification [[SRGB](044-g-references.md#bib-srgb "Multimedia systems and equipment - Colour measurement and management - Part 2-1: Colour management - Default RGB colour space - sRGB")], encoders are strongly encouraged to write the [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#srgb-standard-colour-space) chunk without performing additional [gamma](002-2-scope.md#dfn-gamma) handling. In both cases it is
recommended that an appropriate [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk be generated for use by PNG decoders that do
not recognize the [iCCP](015-11-3-ancillary-chunks.md#11iCCP) or [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#srgb-standard-colour-space)
chunks.

A PNG encoder has to determine:

1. what value to write in the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk;
2. how to transform the provided image samples into the values to be written in the PNG datastream.

The value to write in the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk is that value which causes a PNG decoder to
behave in the desired way. See [13.13 Decoder gamma handling](029-13-13-decoder-gamma-handling.md#13Decoder-gamma-handling).

<a id="dfn-gamma-encoding"></a>

The transform to be applied depends on the nature of the image samples and their precision. If the samples represent light
intensity in floating-point or high precision integer form (perhaps from a computer graphics renderer), the encoder may
perform gamma encoding (applying a power function with exponent less than 1) before quantizing the data to integer
values for inclusion in the PNG datastream. This results in fewer banding artifacts at a given sample depth, or allows
smaller samples while retaining the same visual quality. An intensity level expressed as a floating-point value in the range
0 to 1 can be converted to a datastream image sample by:

`integer_sample = floor((2<sup>sampledepth</sup>-1) * intensity<sup>encoding_exponent</sup> + 0.5)`

<a id="ref-for-dfn-gamma-value-7"></a>

If the intensity in the equation is the desired output intensity, the encoding exponent is the [gamma value](002-2-scope.md#dfn-gamma-value) to be
used in the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk.

<a id="ref-for-dfn-transfer-function-12"></a>

If the intensity available to the PNG encoder is the original scene intensity, another transformation may be needed. There
is sometimes a requirement for the displayed image to have higher contrast than the original source image. This corresponds
to an end-to-end [transfer function](002-2-scope.md#dfn-transfer-function) from original scene to display output with an exponent greater than 1. In this
case:



```
gamma = encoding_exponent/end_to_end_exponent
```




If it is not known whether the conditions under which the original image was captured or calculated warrant such a
contrast change, it may be assumed that the display intensities are proportional to original scene intensities, i.e. the
end-to-end exponent is 1 and hence:



```
gamma = encoding_exponent
```




<a id="ref-for-dfn-gamma-value-8"></a>

If the image is being written to a datastream only, the encoder is free to choose the encoding exponent. Choosing a value
that causes the [gamma value](002-2-scope.md#dfn-gamma-value) in the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk to be 1/2.2 is often a reasonable
choice because it minimizes the work for a PNG decoder displaying on a typical video monitor.

<a id="ref-for-dfn-gamma-6"></a>

Some image renderers may simultaneously write the image to a PNG datastream and display it on-screen. The displayed pixels
should be [gamma](002-2-scope.md#dfn-gamma) corrected for the display system and viewing conditions in use, so that the user sees a proper
representation of the intended scene.

<a id="ref-for-dfn-gamma-encoding-1"></a>

<a id="ref-for-dfn-transfer-function-13"></a>

If the renderer wants to write the displayed sample values to the PNG datastream, avoiding a separate [gamma
encoding](021-12-png-encoders.md#dfn-gamma-encoding) step for the datastream, the renderer should approximate the [transfer function](002-2-scope.md#dfn-transfer-function) of the display system by a
power function, and write the reciprocal of the exponent into the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk. This will
allow a PNG decoder to reproduce what was displayed on screen for the originator during rendering.

<a id="ref-for-dfn-gamma-encoding-2"></a>

However, it is equally reasonable for a renderer to compute displayed pixels appropriate for the display device, and to
perform separate [gamma encoding](021-12-png-encoders.md#dfn-gamma-encoding) for data storage and transmission, arranging to have a value in the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk more appropriate to the future use of the image.

<a id="ref-for-dfn-gamma-encoding-3"></a>

<a id="ref-for-dfn-gamma-encoding-4"></a>

<a id="ref-for-dfn-gamma-value-9"></a>

<a id="ref-for-dfn-transfer-function-14"></a>

<a id="ref-for-dfn-gamma-value-10"></a>

Computer graphics renderers often do not perform [gamma encoding](021-12-png-encoders.md#dfn-gamma-encoding), instead making sample values directly proportional
to scene light intensity. If the PNG encoder receives sample values that have already been quantized into integer values,
there is no point in doing [gamma encoding](021-12-png-encoders.md#dfn-gamma-encoding) on them; that would just result in further loss of information. The encoder
should just write the sample values to the PNG datastream. This does not imply that the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk should contain a [gamma value](002-2-scope.md#dfn-gamma-value) of 1.0 because the desired end-to-end [transfer function](002-2-scope.md#dfn-transfer-function)
from scene intensity to display output intensity is not necessarily linear. However, the desired [gamma value](002-2-scope.md#dfn-gamma-value) is
probably not far from 1.0. It may depend on whether the scene being rendered is a daylight scene or an indoor scene, etc.

<a id="ref-for-dfn-transfer-function-15"></a>

<a id="ref-for-dfn-gamma-7"></a>

When the sample values come directly from a piece of hardware, the correct [gAMA](015-11-3-ancillary-chunks.md#11gAMA) value
can, in principle, be inferred from the [transfer function](002-2-scope.md#dfn-transfer-function) of the hardware and lighting conditions of the scene. In the
case of video digitizers ("frame grabbers"), the samples are probably in the sRGB color space, because the sRGB
specification was designed to be compatible with modern video standards. Image scanners are less predictable. Their output
samples may be proportional to the input light intensity since CCD sensors themselves are linear, or the scanner hardware may
have already applied a power function designed to compensate for dot gain in subsequent printing (an exponent of about 0.57),
or the scanner may have corrected the samples for display on a monitor. It may be necessary to refer to the scanner's manual
or to scan a calibrated target in order to determine the characteristics of a particular scanner. It should be remembered
that [gamma](002-2-scope.md#dfn-gamma) relates samples to desired display output, not to scanner input.

<a id="ref-for-dfn-gamma-8"></a>

<a id="ref-for-dfn-gamma-value-11"></a>

<a id="ref-for-dfn-gamma-9"></a>

Datastream format converters generally should not attempt to convert supplied images to a different [gamma](002-2-scope.md#dfn-gamma). The data
should be stored in the PNG datastream without conversion, and the [gamma value](002-2-scope.md#dfn-gamma-value) should be deduced from information in
the source datastream if possible. [Gamma](002-2-scope.md#dfn-gamma) alteration at datastream conversion time causes re-quantization of the set of
intensity levels that are represented, introducing further roundoff error with little benefit. It is almost always better to
just copy the sample values intact from the input to the output file.

<a id="ref-for-dfn-gamma-10"></a>

<a id="ref-for-dfn-gamma-value-12"></a>

<a id="ref-for-dfn-gamma-value-13"></a>

<a id="ref-for-dfn-gamma-value-14"></a>

If the source datastream describes the [gamma](002-2-scope.md#dfn-gamma) characteristics of the image, a datastream converter is strongly
encouraged to write a [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk. Some datastream formats specify the display exponent
(the exponent of the function which maps image samples to display output rather than the other direction). If the source
file's [gamma value](002-2-scope.md#dfn-gamma-value) is greater than 1.0, it is probably a display exponent, and the reciprocal of this value should be
used for the PNG [gamma value](002-2-scope.md#dfn-gamma-value). If the source file format records the relationship between image samples and a quantity
other than display output, it will be more complex than this to deduce the PNG [gamma value](002-2-scope.md#dfn-gamma-value).

