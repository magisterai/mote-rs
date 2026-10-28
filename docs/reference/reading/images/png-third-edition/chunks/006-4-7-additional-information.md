# 4.7 Additional information

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](005-4-5-png-image.md) · [Next](007-4-9-apng-frame-based-animation.md)

<a id="4Concepts.AncillInfo"></a>

<a id="x4-7-additional-information"></a>

### 4.7 Additional information

Ancillary information may be associated with an image. Decoders may ignore all or some of the ancillary information. The
types of ancillary information provided are described in [Table 2](006-4-7-additional-information.md#table41 "Types of ancillary information").

<a id="table41"></a>

<a id="ref-for-dfn-static-image-5"></a>

<a id="ref-for-dfn-transfer-function-7"></a>

<a id="ref-for-dfn-sdr-2"></a>

<a id="ref-for-dfn-hdr-2"></a>

<a id="ref-for-dfn-luminance-2"></a>

<a id="ref-for-dfn-gamma-value-2"></a>

<a id="ref-for-dfn-chromaticity-3"></a>

[Table 2](006-4-7-additional-information.md#table41) 
Types of ancillary information

| Type of information | Description |
| --- | --- |
| Animation information | An animated image, defined as a series of frames with associated timing, position and handling information, to be displayed if the viewer is capable of doing so. For other cases such as printers, the [static image](003-4-concepts.md#dfn-static-image) will be displayed instead. |
| Background color | Solid background color to be used when presenting the image if no better option is available. |
| Coding-independent code points | Identifies the color space by enumerating metadata such as the [transfer function](002-2-scope.md#dfn-transfer-function) and color primaries. Originally for [SDR](002-2-scope.md#dfn-sdr) and [HDR](002-2-scope.md#dfn-hdr) video, also used for still and animated images. |
| Content Light Level Information | [Luminance](002-2-scope.md#dfn-luminance) of the brightest pixel in the image (or image sequence) and the average luminance level of the brightest frame in the sequence. |
| EXIF information | Exchangeable image file format metadata such as shutter speed, aperture, and orientation |
| Gamma and chromaticity | [Gamma value](002-2-scope.md#dfn-gamma-value) of the image with respect to the desired output intensity, and [chromaticity](002-2-scope.md#dfn-chromaticity) characteristics of the RGB values used in the image. |
| ICC profile | Description of the color space (in the form of an International Color Consortium (ICC) profile) to which the samples in the image conform. |
| Image histogram | Estimates of how frequently the image uses each palette entry. |
| Mastering Display Color Volume | Describes the absolute three-dimensional color gamut volume of the display used to prepare the content, including the lightest and darkest colors the mastering display can reproduce. This helps to present the image on the display device. |
| Physical pixel dimensions | Intended pixel size and aspect ratio to be used in presenting the PNG image. |
| Significant bits | The number of bits that are significant in the samples. |
| sRGB color space | A rendering intent (as defined by the International Color Consortium) and an indication that the image samples conform to this color space. |
| Suggested palette | A reduced palette that may be used when the display device is not capable of displaying the full range of colors in the image. |
| Textual data | Textual information (which may be compressed) associated with the image. |
| Time | The time when the PNG image was last modified. |
| Transparency | Alpha information that allows the reference image to be reconstructed when the alpha channel is not retained in the PNG image. |

<a id="4Concepts.Format"></a>

<a id="x4-8-png-datastream"></a>

### 4.8 PNG datastream

<a id="4Concepts.FormatChunks"></a>

<a id="x4-8-1-chunks"></a>

#### 4.8.1 Chunks

The PNG datastream consists of a PNG signature (see [5.2 PNG signature](008-4-10-error-handling.md#5PNG-file-signature)) followed by a sequence of
chunks (see [11. Chunk specifications](014-10-3-other-uses-of-compression.md#11Chunks)). Each chunk has a chunk type which specifies its function.

<a id="4Concepts.FormatTypes"></a>

<a id="x4-8-2-chunk-types"></a>

#### 4.8.2 Chunk types

Chunk types are four-byte sequences chosen so that they correspond to readable labels when interpreted in the ISO
646.IRV:1991 [[ISO646](044-g-references.md#bib-iso646 "Information technology — ISO 7-bit coded character set for information interchange")] character set. The first four are termed critical chunks, which shall be understood and correctly
interpreted according to the provisions of this specification. These are:

1. [IHDR](014-10-3-other-uses-of-compression.md#11IHDR): image header, which is the first chunk in a PNG datastream.
2. [PLTE](014-10-3-other-uses-of-compression.md#11PLTE): palette table associated with indexed PNG images.
3. [IDAT](014-10-3-other-uses-of-compression.md#11IDAT): image data chunks.
4. [IEND](014-10-3-other-uses-of-compression.md#11IEND): image trailer, which is the last chunk in a PNG datastream.

<a id="3ancillaryChunk"></a>

The remaining chunk types are termed ancillary chunk types, which encoders may generate and decoders may interpret.

1. Transparency information: [tRNS](015-11-3-ancillary-chunks.md#11tRNS) (see [11.3.1 Transparency information](015-11-3-ancillary-chunks.md#11transinfo)).
2. Color space information: [cHRM](015-11-3-ancillary-chunks.md#11cHRM), [gAMA](015-11-3-ancillary-chunks.md#11gAMA),
   [iCCP](015-11-3-ancillary-chunks.md#11iCCP), [sBIT](015-11-3-ancillary-chunks.md#11sBIT), [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#11sRGB), [cICP](016-11-3-2-5-srgb-standard-rgb-color-space.md#cICP-chunk), [mDCV](017-11-3-2-7-mdcv-mastering-display-color-volume.md#mDCV-chunk) (see
   [11.3.2 Color space information](015-11-3-ancillary-chunks.md#11addnlcolinfo)).
3. Textual information: [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt), [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt), [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) (see [11.3.3 Textual information](018-11-3-2-8-clli-content-light-level-information.md#11textinfo)).
4. Miscellaneous information: [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD), [hIST](019-11-3-3-4-itxt-international-textual-data.md#11hIST),
   [pHYs](019-11-3-3-4-itxt-international-textual-data.md#11pHYs), [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT), [eXIf](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#eXIf) (see [11.3.4 Miscellaneous information](019-11-3-3-4-itxt-international-textual-data.md#11addnlsiinfo)).
5. Time information: [tIME](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#11tIME) (see [11.3.5 Time stamp information](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#11timestampinfo)).
6. Animation information: [acTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#acTL-chunk), [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk),
   [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) (see [11.3.6 Animation information](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#animation-information)).

