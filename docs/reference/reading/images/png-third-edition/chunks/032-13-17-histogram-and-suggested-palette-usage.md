# 13.17 Histogram and suggested palette usage

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](031-13-16-alpha-channel-processing.md) · [Next](033-14-3-ordering-of-chunks.md)

<a id="13Histogram-and-suggested-palette-usage"></a>

<a id="x13-17-histogram-and-suggested-palette-usage"></a>

### 13.17 Histogram and suggested palette usage

<a id="ref-for-3truecolour-26"></a>

<a id="ref-for-dfn-frame-buffer-12"></a>

For viewers running on indexed-color hardware attempting to display a [truecolor](005-4-5-png-image.md#3truecolour) image, or an indexed-color image
whose palette is too large for the [frame buffer](002-2-scope.md#dfn-frame-buffer), the encoder may have provided one or more suggested palettes in
[sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunks. If one of these is found to be suitable, based on size and perhaps name, the
PNG decoder can use that palette. Suggested palettes with a sample depth different from what the decoder needs can be
converted using sample depth rescaling (see [13.12 Sample depth rescaling](028-13-10-interlacing-and-progressive-display.md#13Sample-depth-rescaling)).

<a id="ref-for-dfn-composited-14"></a>

When the background is a solid color, the viewer should [composite](002-2-scope.md#dfn-composited) the image and the suggested palette against that
color, then quantize the resulting image to the resulting RGB palette. When the image uses transparency and the background
is not a solid color, no suggested palette is likely to be useful.

<a id="ref-for-3truecolour-27"></a>

For [truecolor](005-4-5-png-image.md#3truecolour) images, a suggested palette might also be provided in a [PLTE](014-10-3-other-uses-of-compression.md#11PLTE)
chunk. If the image has a [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk and the background is a solid color, the viewer
will need to adapt the suggested palette for use with its desired background color. To do this, the palette entry closest to
the [tRNS](015-11-3-ancillary-chunks.md#11tRNS) color should be replaced with the desired background color; or alternatively a
palette entry for the background color can be added, if the viewer can handle more colors than there are [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) entries.

<a id="ref-for-3colourType-41"></a>

<a id="ref-for-dfn-truecolor-with-alpha-7"></a>

For images of [color type](010-5-7-defining-chunks.md#3colourType) 6 ([truecolor with alpha](005-4-5-png-image.md#dfn-truecolor-with-alpha)), any [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk
should have been designed for display of the image against a uniform background of the color specified by the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk. Viewers should probably ignore the palette if they intend to use a different
background, or if the [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunk is missing. Viewers can use a suggested palette for
display against a different background than it was intended for, but the results may not be very good.

<a id="ref-for-3truecolour-28"></a>

<a id="ref-for-dfn-composited-15"></a>

<a id="ref-for-3truecolour-29"></a>

<a id="ref-for-3truecolour-30"></a>

If the viewer presents a transparent [truecolor](005-4-5-png-image.md#3truecolour) image against a background that is more complex than a uniform
color, it is unlikely that the suggested palette will be optimal for the [composite](002-2-scope.md#dfn-composited) image. In this case it is best to
perform a [truecolor](005-4-5-png-image.md#3truecolour) compositing step on the [truecolor](005-4-5-png-image.md#3truecolour) PNG image and background image, then color-quantize
the resulting image.

<a id="ref-for-3truecolour-31"></a>

In [truecolor](005-4-5-png-image.md#3truecolour) PNG datastreams, if both [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) and [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunks appear, the PNG decoder may choose from among the palettes suggested by both, bearing in mind the
different transparency semantics described above.

<a id="ref-for-dfn-image-data-32"></a>

The frequencies in the [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) and [hIST](019-11-3-3-4-itxt-international-textual-data.md#11hIST) chunks are
useful when the viewer cannot provide as many colors as are used in the palette in the PNG datastream. If the viewer has a
shortfall of only a few colors, it is usually adequate to drop the least-used colors from the palette. To reduce the number
of colors substantially, it is best to choose entirely new representative colors, rather than trying to use a subset of the
existing palette. This amounts to performing a new color quantization step; however, the existing palette and histogram can
be used as the input data, thus avoiding a scan of the [image data](002-2-scope.md#dfn-image-data) in the [IDAT](014-10-3-other-uses-of-compression.md#11IDAT)
chunks.

<a id="ref-for-dfn-image-data-33"></a>

If no suggested palette is provided, a decoder can develop its own, at the cost of an extra pass over the [image
data](002-2-scope.md#dfn-image-data) in the [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks. Alternatively, a default palette (probably a color cube)
can be used.

See also [12.5 Suggested palettes](023-12-3-alpha-channel-creation.md#12Suggested-palettes).

<a id="14EditorsExt"></a>

<a id="x14-editors"></a>

## 14. Editors

<a id="14Additional-chunk-types"></a>

<a id="x14-1-additional-chunk-types"></a>

### 14.1 Additional chunk types

Authors are encouraged to look existing chunk types in both this specification and [[PNG-EXTENSIONS](044-g-references.md#bib-png-extensions "Extensions to the PNG Third Edition Specification, Version 1.6.0")] before considering
introducing a new chunk types. The chunk types at [[PNG-EXTENSIONS](044-g-references.md#bib-png-extensions "Extensions to the PNG Third Edition Specification, Version 1.6.0")] are expected to be less widely supported than those
defined in this specification.

<a id="14Ordering"></a>

<a id="x14-2-behavior-of-png-editors"></a>

### 14.2 Behavior of PNG editors

<a id="ref-for-dfn-png-editor-7"></a>

<a id="ref-for-3truecolour-32"></a>

<a id="ref-for-dfn-png-editor-8"></a>

Two examples of [PNG editors](002-2-scope.md#dfn-png-editor) are a program that adds or modifies text chunks, and a program that adds a suggested
palette to a [truecolor](005-4-5-png-image.md#3truecolour) PNG datastream. Ordinary image editors are not [PNG editors](002-2-scope.md#dfn-png-editor) because they usually discard
all unrecognized information while reading in an image.

<a id="ref-for-dfn-png-editor-9"></a>

To allow new chunk types to be added to PNG, it is necessary to establish rules about the ordering requirements for all
chunk types. Otherwise a [PNG editor](002-2-scope.md#dfn-png-editor) does not know what to do when it encounters an unknown chunk.

<a id="ref-for-dfn-png-editor-10"></a>

EXAMPLE Consider a hypothetical new ancillary chunk type that is safe-to-copy and is required to appear after [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) if [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) is present. If a program attempts to add a
[PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk and does not recognize the new chunk, it may insert the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk in the wrong place, namely after the new chunk. Such problems could be prevented by requiring [PNG
editors](002-2-scope.md#dfn-png-editor) to discard all unknown chunks, but that is a very unattractive solution. Instead, PNG requires ancillary chunks
not to have ordering restrictions like this.

<a id="ref-for-dfn-png-editor-11"></a>

To prevent this type of problem while allowing for future extension, constraints are placed on both the behavior of
[PNG editors](002-2-scope.md#dfn-png-editor) and the allowed ordering requirements for chunks. The safe-to-copy bit defines the proper handling of
unrecognized chunks in a datastream that is being modified.

<a id="ref-for-dfn-png-editor-12"></a>

1. If a chunk's safe-to-copy bit is 1, the chunk may be copied to a modified PNG datastream whether or not the [PNG
   editor](002-2-scope.md#dfn-png-editor) recognizes the chunk type, and regardless of the extent of the datastream modifications.

<a id="ref-for-dfn-image-data-34"></a>

2. If a chunk's safe-to-copy bit is 0, it indicates that the chunk depends on the [image data](002-2-scope.md#dfn-image-data). If the program has
   made **any** changes to **critical** chunks, including addition, modification, deletion, or
   reordering of critical chunks, then unrecognized unsafe chunks shall **not** be copied to the output PNG
   datastream. (Of course, if the program **does** recognize the chunk, it can choose to output an appropriately
   modified version.)

<a id="ref-for-dfn-png-editor-13"></a>

3. A [PNG editor](002-2-scope.md#dfn-png-editor) is always allowed to copy all unrecognized ancillary chunks if it has only added, deleted,
   modified, or reordered **ancillary** chunks. This implies that it is not permissible for ancillary chunks to
   depend on other ancillary chunks.

<a id="ref-for-dfn-png-editor-14"></a>

4. [PNG editors](002-2-scope.md#dfn-png-editor) shall terminate on encountering an unrecognized critical chunk type, because there is no way to be
   certain that a valid datastream will result from modifying a datastream containing such a chunk. (Simply discarding the
   chunk is not good enough, because it might have unknown implications for the interpretation of other chunks.) The
   safe/unsafe mechanism is intended for use with ancillary chunks. The safe-to-copy bit will always be 0 for critical
   chunks.

The rules governing ordering of chunks are as follows.

<a id="ref-for-dfn-png-editor-15"></a>

1. When copying an unknown **unsafe-to-copy** ancillary chunk, a [PNG editor](002-2-scope.md#dfn-png-editor) shall not move the chunk
   relative to any critical chunk. It may relocate the chunk freely relative to other ancillary chunks that occur between the
   same pair of critical chunks. (This is well defined since the editor shall not add, delete, modify, or reorder critical
   chunks if it is preserving unknown unsafe-to-copy chunks.)

<a id="ref-for-dfn-png-editor-16"></a>

2. When copying an unknown **safe-to-copy** ancillary chunk, a [PNG editor](002-2-scope.md#dfn-png-editor) shall not move the chunk
   from before [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) to after [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) or vice versa. (This
   is well defined because [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) is always present.) Any other reordering is permitted.
3. When copying a **known** ancillary chunk type, an editor need only honour the specific chunk ordering
   rules that exist for that chunk type. However, it may always choose to apply the above general rules instead.

These rules are expressed in terms of copying chunks from an input datastream to an output datastream, but they apply in
the obvious way if a PNG datastream is modified in place.

See also [5.4 Chunk naming conventions](008-4-10-error-handling.md#5Chunk-naming-conventions).

<a id="ref-for-dfn-png-editor-17"></a>

<a id="ref-for-dfn-image-data-35"></a>

[PNG editors](002-2-scope.md#dfn-png-editor) that do not change the [image data](002-2-scope.md#dfn-image-data) should not change the [tIME](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#11tIME)
chunk. The Creation Time keyword in the [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt), [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt),
and [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunks may be used for a user-supplied time.

