# F.7 Changes since the First Public Working Draft of 25 October 2022 (Third Edition)

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](036-e-online-resources.md) · [Next](038-f-9-changes-between-first-and-second-editions.md)

<a id="changes-since-the-first-public-working-draft-of-25-october-2022-third-edition"></a>

<a id="changes-20221025"></a>

### F.7 Changes since the [First Public Working Draft of 25 October 2022 (Third Edition)](https://www.w3.org/TR/2022/WD-png-3-20221025/)

- Explained preferable handling of trailing bytes in the final [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk for encoders and decoders.

<a id="ref-for-dfn-hdr-8"></a>

- Linked to open issue on tone-mapping [HDR](002-2-scope.md#dfn-hdr) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] images in the presence of [mDCV](017-11-3-2-7-mdcv-mastering-display-color-volume.md#mDCV-chunk).
- Follow the Encoding Standard on UTF-8 encode and decode.
- Added definition of a frame.
- Required the Matrix Coefficients in [cICP](016-11-3-2-5-srgb-standard-rgb-color-space.md#cICP-chunk) to be zero (RGB data).
- Added known Privacy issue with recoverable data that only appears to have been redacted.
- Improved advice on choosing filters.
- Added links to color image type definitions.
- Clarified that MaxFALL uses the values of the frame with highest mean luminance.
- Clarified luminance units.
- Prefer RFC 3339 format for Creation Time.
- Improved the definition of [mDCV](017-11-3-2-7-mdcv-mastering-display-color-volume.md#mDCV-chunk), with better descriptions, default values, and reference to SMPTE standards.
- Refactored the terms and definitions, for clarity.
- Improved definitions of source, reference, and PNG images.
- Moved concepts from the terms and definitions section to the main prose.
- Corrected error in [eXIf](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#eXIf) chunk,
  which conflicted with the [chunk ordering](009-5-5-crc-algorithm.md#5ChunkOrdering) section.
- Simplified [Scope](002-2-scope.md#1Scope) section to remove redundant detail described elsewhere.
- Redrew chunk-ordering lattice diagrams to be clearer and more consistent.

<a id="ref-for-dfn-hdr-9"></a>

- Added a new chunk, [cLLI](018-11-3-2-8-clli-content-light-level-information.md#cLLI-chunk),
  to describe the Maximum Single-Pixel and Frame-Average Luminance Levels
  for both static and animated [HDR](002-2-scope.md#dfn-hdr) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] PNG images.
- Updated external links to latest versions, preferring https over http.
- Specified interoperable handling of extra sample bits, beyond the specified bit depth,
  in [tRNS](015-11-3-ancillary-chunks.md#11tRNS) and [bKGD](019-11-3-3-4-itxt-international-textual-data.md#11bKGD) chunks.

<a id="ref-for-dfn-hdr-10"></a>

- Added a new chunk, [mDCV](017-11-3-2-7-mdcv-mastering-display-color-volume.md#mDCV-chunk)
  to describe the color volume of the mastering display used to grade [HDR](002-2-scope.md#dfn-hdr) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] content.
- Used correct Unicode character names.
- Changed chunk type codes to use hexadecimal, rather than decimal.
- Described textual chunk processing more clearly.
- Recommended [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) for new content.
- Clarifications on the language tag field of the [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunk,
  corrected examples to conform to BCP47.
- Updated image/apng registration appendix. APNG MIME type registered with IANA.
- Converted ACII-art figures to more accessible diagrams.

<a id="changes-since-the-w3c-recommendation-of-10-november-2003-png-second-edition"></a>

<a id="changes-20031110"></a>

### F.8 Changes since the [W3C Recommendation of 10 November 2003](https://www.w3.org/TR/2003/REC-PNG-20031110/) (PNG Second Edition)

- The three previously defined, but unofficial, chunks for Animated PNG (APNG) have been added:

  - [acTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#acTL-chunk) Animation Control Chunk
  - [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) Frame Control Chunk
  - [fdAT](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fdAT-chunk) Frame Data Chunk

  This brings the PNG specification into alignment with widely deployed industry practice.
- <a id="ref-for-dfn-hdr-11"></a>

  Added the [cICP](016-11-3-2-5-srgb-standard-rgb-color-space.md#cICP-chunk) chunk, Coding-independent code points for video signal type
  identification, to contain image format metadata defined in [[ITU-T-H.273](044-g-references.md#bib-itu-t-h.273 "ITU-T H.273, SERIES H: AUDIOVISUAL AND MULTIMEDIA SYSTEMS Infrastructure of audiovisual services – Coding of moving video. Coding-independent code points for video signal type identification")] which enables PNG to contain [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] High Dynamic Range
  ([HDR](002-2-scope.md#dfn-hdr)) and Wide Color Gamut (WCG) images.
- For chunks which define the image color space,
  the order of precedence is clearly defined,
  if more than one is present.
- The previously defined [eXIf](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#eXIf) chunk has been moved from the PNG-Extensions document
  [[PNG-EXTENSIONS](044-g-references.md#bib-png-extensions "Extensions to the PNG Third Edition Specification, Version 1.6.0")] into the main body of this specification, to reflect its increasing use.
- To help with tonemapping HDR content, added the [mDCV](017-11-3-2-7-mdcv-mastering-display-color-volume.md#mDCV-chunk) chunk, which contains metadata about the display used in
  mastering, and [cLLI](018-11-3-2-8-clli-content-light-level-information.md#cLLI-chunk), which contains metadata about peak and average light levels. This enabled more accurate color matching on heterogeneous platforms
- Clarified that the [iCCP](015-11-3-ancillary-chunks.md#11iCCP) chunk, which contains an ICC profile, can contain profiles conforming to any version of the ICC.1 specification. PNG Second Edition only referenced the then-current v2 of ICC.1, although it has since become industry practice to also used higher versions.
- Clarified handling of out-of-range indexes, for indexed-color PNG
- Clarified error recovery for unknown and invalid ancillary chunks
- Incorporation of all [PNG Second Edition Errata](https://www.w3.org/2003/11/REC-PNG-20031110-errata). Notably, clarified that PNG images with unknown gamma value, when embedded in formats such as HTML or SVG, must be treated as [untagged images](https://drafts.csswg.org/css-color-4/#untagged)
- Various editorial clarifications in response to community feedback
- References updated to latest versions
- Markup corrections and link fixes
- Document source reformatted to use ReSpec

