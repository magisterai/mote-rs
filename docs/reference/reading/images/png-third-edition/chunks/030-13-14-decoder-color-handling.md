# 13.14 Decoder color handling

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](029-13-13-decoder-gamma-handling.md) · [Next](031-13-16-alpha-channel-processing.md)

<a id="13Decoder-colour-handling"></a>

<a id="x13-14-decoder-color-handling"></a>

### 13.14 Decoder color handling

See [C. Gamma and chromaticity](035-c-gamma-and-chromaticity.md#C-GammaAppendix) for references to color issues.

<a id="ref-for-dfn-image-data-26"></a>

<a id="ref-for-dfn-gamma-19"></a>

<a id="ref-for-dfn-gamma-20"></a>

In many cases, the [image data](002-2-scope.md#dfn-image-data) in PNG datastreams will be treated as device-dependent RGB values and displayed
without modification (except for appropriate [gamma](002-2-scope.md#dfn-gamma) correction). This provides the fastest display of PNG images. But
unless the viewer uses exactly the same display hardware as that used by the author of the original image, the colors will
not be exactly the same as those seen by the original author, particularly for darker or near-neutral colors. The [cHRM](015-11-3-ancillary-chunks.md#11cHRM) chunk provides information that allows closer color matching than that provided by
[gamma](002-2-scope.md#dfn-gamma) correction alone.

<a id="ref-for-dfn-image-data-27"></a>

The [cHRM](015-11-3-ancillary-chunks.md#11cHRM) data can be used to transform the [image data](002-2-scope.md#dfn-image-data) from RGB to XYZ and
thence into a perceptually linear color space such as CIE LAB. The colors can be partitioned to generate an optimal
palette, because the geometric distance between two colors in CIE LAB is strongly related to how different those colors
appear (unlike, for example, RGB or XYZ spaces). The resulting palette of colors, once transformed back into RGB color
space, could be used for display or written into a [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk.

<a id="ref-for-dfn-image-data-28"></a>

Decoders that are part of image processing applications might also transform [image data](002-2-scope.md#dfn-image-data) into CIE LAB space for
analysis.

<a id="ref-for-dfn-image-data-29"></a>

In applications where color fidelity is critical, such as product design, scientific visualization, medicine,
architecture, or advertising, PNG decoders can transform the [image data](002-2-scope.md#dfn-image-data) from source RGB to the display RGB space of
the monitor used to view the image. This involves calculating the matrix to go from source RGB to XYZ and the matrix to go
from XYZ to display RGB, then combining them to produce the overall transformation. The PNG decoder is responsible for
implementing gamut mapping.

<a id="ref-for-dfn-image-data-30"></a>

Decoders running on platforms that have a Color Management System (CMS) can pass the [image data](002-2-scope.md#dfn-image-data), [gAMA](015-11-3-ancillary-chunks.md#11gAMA), and [cHRM](015-11-3-ancillary-chunks.md#11cHRM) values to the CMS for display or further
processing.

<a id="ref-for-dfn-image-data-31"></a>

PNG decoders that provide color printing facilities can use the facilities in Level 2 PostScript to specify [image
data](002-2-scope.md#dfn-image-data) in calibrated RGB space or in a device-independent color space such as XYZ. This will provide better color
fidelity than a simple RGB to CMYK conversion. The PostScript Language Reference manual [[PostScript](045-g-2-informative-references.md#bib-postscript "PostScript Language Reference Manual")] gives examples. Such
decoders are responsible for implementing gamut mapping between source RGB (specified in the [cHRM](015-11-3-ancillary-chunks.md#11cHRM) chunk) and the target printer. The PostScript interpreter is then responsible for producing the required
colors.

PNG decoders can use the [cHRM](015-11-3-ancillary-chunks.md#11cHRM) data to calculate an accurate greyscale representation
of a color image. Conversion from RGB to grey is simply a case of calculating the Y (luminance) component of XYZ, which is a
weighted sum of R, G, and B values. The weights depend upon the monitor type, i.e. the values in the [cHRM](015-11-3-ancillary-chunks.md#11cHRM) chunk. PNG decoders may wish to do this for PNG datastreams with no [cHRM](015-11-3-ancillary-chunks.md#11cHRM) chunk. In this case, a reasonable default would be the CCIR 709 primaries [[ITU-R-BT.709](044-g-references.md#bib-itu-r-bt.709 "ITU-R BT.709, SERIES BT: BROADCASTING SERVICE (TELEVISION). Parameter values for the HDTV standards for production and international programme exchange")]. The original
NTSC primaries should **not** be used unless the PNG image really was color-balanced for such a monitor.

<a id="13Background-colour"></a>

<a id="x13-15-background-color"></a>

### 13.15 Background color

The background color given by the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk will typically be used to fill unused
screen space around the image, as well as any transparent pixels within the image. (Thus, [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) is valid and useful even when the image does not use transparency.) If no [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk is present, the viewer will need to decide upon a suitable background color. When no other
information is available, a medium grey such as 153 in the 8-bit sRGB color space would be a reasonable choice. Transparent
black or white text and dark drop shadows, which are common, would all be legible against this background.

Viewers that have a specific background against which to present the image (such as web browsers) should ignore the
[bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk, in effect overriding [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) with their
preferred background color or background image.

<a id="ref-for-3indexedColour-19"></a>

<a id="ref-for-dfn-composited-10"></a>

The background color given by the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk is not to be considered transparent,
even if it happens to match the color given by the [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk (or, in the case of an
[indexed-color](004-4-3-color-spaces.md#3indexedColour) image, refers to a palette index that is marked as transparent by the [tRNS](015-11-3-ancillary-chunks.md#11tRNS)
chunk). Otherwise one would have to imagine something "behind the background" to [composite](002-2-scope.md#dfn-composited) against. The background
color is either used as background or ignored; it is not an intermediate layer between the PNG image and some other
background.

Indeed, it will be common that the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) and [tRNS](015-11-3-ancillary-chunks.md#11tRNS)
chunks specify the same color, since then a decoder that does not implement transparency processing will give the intended
display, at least when no partially-transparent pixels are present.

