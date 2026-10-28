# 13.13 Decoder gamma handling

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](028-13-10-interlacing-and-progressive-display.md) · [Next](030-13-14-decoder-color-handling.md)

<a id="13Decoder-gamma-handling"></a>

<a id="x13-13-decoder-gamma-handling"></a>

### 13.13 Decoder gamma handling

<a id="ref-for-dfn-gamma-15"></a>

See [C. Gamma and chromaticity](035-c-gamma-and-chromaticity.md#C-GammaAppendix) for a brief introduction to [gamma](002-2-scope.md#dfn-gamma) issues.

Viewers capable of full color management will perform more sophisticated calculations than those described here.

<a id="ref-for-dfn-transfer-function-17"></a>

For an image display program to produce correct tone reproduction, it is necessary to take into account the relationship
between samples and display output, and the [transfer function](002-2-scope.md#dfn-transfer-function) of the display system. This can be done by
calculating:

`sample = integer_sample / (2<sup>sampledepth</sup> - 1.0)  
display_output = sample<sup>1.0/gamma</sup>  
display_input = inverse_display_transfer(display_output)  
framebuf_sample = floor((display_input * MAX_FRAMEBUF_SAMPLE)+0.5)`

<a id="ref-for-dfn-frame-buffer-3"></a>

<a id="ref-for-dfn-frame-buffer-4"></a>

<a id="ref-for-dfn-transfer-function-18"></a>

<a id="ref-for-dfn-frame-buffer-5"></a>

where integer\_sample is the sample value from the datastream, framebuf\_sample is the value to write
into the [frame buffer](002-2-scope.md#dfn-frame-buffer), and MAX\_FRAMEBUF\_SAMPLE is the maximum value of a [frame buffer](002-2-scope.md#dfn-frame-buffer) sample (255
for 8-bit, 31 for 5-bit, etc). The first line converts an integer sample into a normalized floating point value (in the range
0.0 to 1.0), the second converts to a value proportional to the desired display output intensity, the third accounts for the
display system's [transfer function](002-2-scope.md#dfn-transfer-function), and the fourth converts to an integer [frame buffer](002-2-scope.md#dfn-frame-buffer) sample. Zero raised to
any positive power is zero.

A step could be inserted between the second and third to adjust display\_output to account for the difference
between the actual viewing conditions and the reference viewing conditions. However, this adjustment requires accounting for
veiling glare, black mapping, and color appearance models, none of which can be well approximated by power functions. Such
calculations are not described here. If viewing conditions are ignored, the error will usually be small.

<a id="ref-for-dfn-transfer-function-19"></a>

The display [transfer function](002-2-scope.md#dfn-transfer-function) can typically be approximated by a power function with exponent
display\_exponent, in which case the second and third lines can be merged into:

`display_input = sample<sup>1.0/(gamma * display_exponent)</sup> = sample<sup>decoding_exponent</sup>`

so as to perform only one power calculation. For color images, the entire calculation is performed separately for R, G,
and B values.

<a id="ref-for-dfn-gamma-value-18"></a>

<a id="ref-for-dfn-gamma-value-19"></a>

The [gamma value](002-2-scope.md#dfn-gamma-value) can be taken directly from the [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk. Alternatively, an
application may wish to allow the user to adjust the appearance of the displayed image by influencing the [gamma value](002-2-scope.md#dfn-gamma-value).
For example, the user could manually set a parameter user\_exponent which defaults to 1.0, and the application
could set:



```
gamma = gamma_from_file / user_exponent
decoding_exponent = 1.0 / (gamma * display_exponent)
   = user_exponent / (gamma_from_file * display_exponent)
```




The user would set user\_exponent greater than 1 to darken the mid-level tones, or less than 1 to lighten
them.

A [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk containing zero is meaningless but could appear by mistake. Decoders
should ignore it, and editors may discard it and issue a warning to the user.

It is **not** necessary to perform a transcendental mathematical computation for every pixel. Instead, a
lookup table can be computed that gives the correct output value for every possible sample value. This requires only 256
calculations per image (for 8-bit accuracy), not one or three calculations per pixel. For an indexed-color image, a one-time
correction of the palette is sufficient, unless the image uses transparency and is being displayed against a nonuniform
background.

<a id="ref-for-dfn-gamma-16"></a>

If floating-point calculations are not possible, [gamma](002-2-scope.md#dfn-gamma) correction tables can be computed using integer arithmetic
and a precomputed table of logarithms. Example code appears in [[PNG-EXTENSIONS](044-g-references.md#bib-png-extensions "Extensions to the PNG Third Edition Specification, Version 1.6.0")].

<a id="ref-for-dfn-gamma-value-20"></a>

<a id="ref-for-dfn-gamma-value-21"></a>

<a id="ref-for-dfn-gamma-value-22"></a>

<a id="ref-for-dfn-gamma-value-23"></a>

When the incoming image has unknown [gamma value](002-2-scope.md#dfn-gamma-value) ([gAMA](015-11-3-ancillary-chunks.md#11gAMA), [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#srgb-standard-colour-space), and [iCCP](015-11-3-ancillary-chunks.md#11iCCP) all absent), standalone image viewers
should choose a likely default [gamma value](002-2-scope.md#dfn-gamma-value), but allow the user to select a new one if the result proves too dark or
too light. The default [gamma value](002-2-scope.md#dfn-gamma-value) may depend on other knowledge about the image, for example whether it came from the
Internet or from the local system. For consistency, viewers for document formats such as HTML, or vector graphics such as
SVG, should treat embedded or linked PNG images with unknown [gamma value](002-2-scope.md#dfn-gamma-value) in the same way that they treat other
untagged images.

<a id="ref-for-dfn-gamma-17"></a>

In practice, it is often difficult to determine what value of display exponent should be used. In systems with no built-in
[gamma](002-2-scope.md#dfn-gamma) correction, the display exponent is determined entirely by the CRT. A display exponent of 2.2 should be
used unless detailed calibration measurements are available for the particular CRT used.

<a id="ref-for-dfn-frame-buffer-6"></a>

<a id="ref-for-dfn-gamma-18"></a>

<a id="ref-for-dfn-gamma-value-24"></a>

Many modern [frame buffers](002-2-scope.md#dfn-frame-buffer) have lookup tables that are used to perform [gamma](002-2-scope.md#dfn-gamma) correction, and on these systems
the display exponent value should be the exponent of the lookup table and CRT combined. It may not be possible to find
out what the lookup table contains from within the viewer application, in which case it may be necessary to ask the user to
supply the display system's exponent value. Unfortunately, different manufacturers use different ways of specifying what
should go into the lookup table, so interpretation of the system [gamma value](002-2-scope.md#dfn-gamma-value) is system-dependent.

<a id="ref-for-dfn-frame-buffer-7"></a>

The response of real displays is actually more complex than can be described by a single number (the display exponent). If
actual measurements of the monitor's light output as a function of voltage input are available, the third and fourth lines of
the computation above can be replaced by a lookup in these measurements, to find the actual [frame buffer](002-2-scope.md#dfn-frame-buffer) value that
most nearly gives the desired brightness.

