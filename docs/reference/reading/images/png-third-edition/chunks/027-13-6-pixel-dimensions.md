# 13.6 Pixel dimensions

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](026-13-2-error-checking.md) · [Next](028-13-10-interlacing-and-progressive-display.md)

<a id="13Pixel-dimensions"></a>

<a id="x13-6-pixel-dimensions"></a>

### 13.6 Pixel dimensions

Non-square pixels can be represented (see [11.3.4.3 pHYs Physical pixel dimensions](019-11-3-3-4-itxt-international-textual-data.md#11pHYs)), but viewers are not required to account for them; a
viewer can present any PNG datastream as though its pixels are square.

Where the pixel aspect ratio of the display differs from the aspect ratio of the physical pixel dimensions defined in the
PNG datastream, viewers are strongly encouraged to rescale images for proper display.

When the [pHYs](019-11-3-3-4-itxt-international-textual-data.md#11pHYs) chunk has a unit specifier of 0 (unit is unknown), the behavior of a
decoder may depend on the ratio of the two pixels-per-unit values, but should not depend on their magnitudes. For example, a
[pHYs](019-11-3-3-4-itxt-international-textual-data.md#11pHYs) chunk containing `(ppuX, ppuY, unit) = (2, 1, 0)` is equivalent to one
containing `(1000, 500, 0)`; both are equally valid indications that the image pixels are twice as tall as they
are wide.

One reasonable way for viewers to handle a difference between the pixel aspect ratios of the image and the display is to
expand the image either horizontally or vertically, but not both. The scale factors could be obtained using the following
floating-point calculations:



```
image_ratio = pHYs_ppuY / pHYs_ppuX
display_ratio = display_ppuY / display_ppuX
scale_factor_X = max(1.0, image_ratio/display_ratio)
scale_factor_Y = max(1.0, display_ratio/image_ratio)
```




Because other methods such as maintaining the image area are also reasonable, and because ignoring the [pHYs](019-11-3-3-4-itxt-international-textual-data.md#11pHYs) chunk is permissible, authors should not assume that all viewing applications will use this scaling
method.

As well as making corrections for pixel aspect ratio, a viewer may have reasons to perform additional scaling both
horizontally and vertically. For example, a viewer might want to shrink an image that is too large to fit on the display, or
to expand images sent to a high-resolution printer so that they appear the same size as they did on the display.

<a id="13Text-chunk-processing"></a>

<a id="x13-7-text-chunk-processing"></a>

### 13.7 Text chunk processing

If practical, PNG decoders should have a way to display to the user all the [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt),
[tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt), and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks found in the datastream. Even
if the decoder does not recognize a particular text keyword, the user might be able to understand it.

When processing [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks, decoders
could encounter characters other than those permitted. Some can be safely displayed (e.g., TAB, FF, and CR, hexadecimal 09,
0C, and 0D, respectively), but others, especially the ESC character (hexadecimal 1B), could pose a security hazard (because
unexpected actions may be taken by display hardware or software). Decoders should not attempt to directly display any
non-Latin-1 characters (except for newline and perhaps TAB, FF, CR) encountered in a [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt)
or [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunk. Instead, they should be ignored or displayed in a visible notation such as
"`\nnn`". See [13.3 Security considerations](026-13-2-error-checking.md#13Security-considerations).

When processing [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunks,
decoders should follow [UTF-8 decode](https://encoding.spec.whatwg.org/#utf-8-decode) in [Encoding Standard](https://encoding.spec.whatwg.org/).

Even though encoders are recommended to represent newlines as linefeed (hexadecimal 0A), it is recommended that decoders
not rely on this; it is best to recognize all the common newline combinations (CR, LF, and CR-LF) and display each as a
single newline. TAB can be expanded to the proper number of spaces needed to arrive at a column multiple of 8.

Decoders running on systems with a non-Latin-1 [legacy character encoding](https://www.w3.org/TR/i18n-glossary/#dfn-legacy-character-encodings) should remap character codes so that
Latin-1 characters are displayed correctly. Unsupported characters should be replaced with a system-appropriate replacement
character (such as U+FFFD REPLACEMENT CHARACTER, U+003F QUESTION MARK, or U+001A SUB) or mapped to a visible notation such as
"`\nnn`". Characters should be only displayed if they are printable characters on the decoding system. Some byte
values may be interpreted by the decoding system as control characters; for security, decoders running on such systems should
not display these control characters.

Decoders should be prepared to display text chunks that contain any number of printing characters between newline
characters, even though it is recommended that encoders avoid creating lines in excess of 79 characters.

<a id="13Decompression"></a>

<a id="x13-8-decompression"></a>

### 13.8 Decompression

The compression technique used in this specification does not require the entire compressed datastream to be available
before decompression can start. Display can therefore commence before the entire decompressed datastream is available. It is
extremely unlikely that any general purpose compression methods in future versions of this specification will not have this
property.

<a id="ref-for-dfn-image-data-22"></a>

<a id="ref-for-dfn-deflate-17"></a>

<a id="ref-for-dfn-image-data-23"></a>

<a id="ref-for-dfn-zlib-18"></a>

It is important to emphasize that [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk boundaries have no semantic significance
and can occur at any point in the compressed datastream. There is no required correlation between the structure of the
[image data](002-2-scope.md#dfn-image-data) (for example, scanline boundaries) and [deflate](002-2-scope.md#dfn-deflate) block boundaries or [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk boundaries. The complete [image data](002-2-scope.md#dfn-image-data) is represented by a single [zlib](002-2-scope.md#dfn-zlib) datastream that is
stored in some number of [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks; a decoder that assumes any more than this is
incorrect. Some encoder implementations may emit datastreams in which some of these structures are indeed related, but
decoders cannot rely on this.

<a id="13Filtering"></a>

<a id="x13-9-filtering"></a>

### 13.9 Filtering

<a id="ref-for-dfn-image-data-24"></a>

To reverse the effect of a filter, the decoder may need to use the decoded values of the prior pixel on the same line, the
pixel immediately above the current pixel on the prior line, and the pixel just to the left of the pixel above. This implies
that at least one scanline's worth of [image data](002-2-scope.md#dfn-image-data) needs to be stored by the decoder at all times. Even though some
filter types do not refer to the prior scanline, the decoder will always need to store each scanline as it is decoded, since
the next scanline might use a filter type that refers to it. See [7.3 Filtering](012-7-3-filtering.md#7Filtering).

