# E. Online resources

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](035-c-gamma-and-chromaticity.md) · [Next](037-f-7-changes-since-the-first-public-working-draft-of-25-october-2022-third-edition.md)

<a id="E-Resources"></a>

<a id="onlineresources"></a>

## E. Online resources

*This section is non-normative.*

<a id="E-Intro"></a>

<a id="introduction-10"></a>

### Introduction

This annex gives the locations of some Internet resources for PNG software developers. By the nature of the Internet, the
list is incomplete and subject to change.

<a id="E-icc-profile-specs"></a>

<a id="e-1-icc-profile-specifications"></a>

### E.1 ICC profile specifications

ICC profile specifications are available at: [`https://www.color.org/`](https://www.color.org/)

<a id="E-PNG-home-page"></a>

<a id="e-2-png-web-site"></a>

### E.2 PNG web site

There is a World Wide Web site for PNG at [`http://www.libpng.org/pub/png/`](http://www.libpng.org/pub/png/). This page is a central location for current
information about PNG and PNG-related tools.

<a id="ref-for-dfn-deflate-18"></a>

Additional documentation and portable C code for [deflate](002-2-scope.md#dfn-deflate), and an optimized implementation of the CRC
algorithm are available from the zlib web site, [`https://www.zlib.net/`](https://www.zlib.net/).

<a id="E-Sample-implementation"></a>

<a id="e-3-sample-implementation-and-test-images"></a>

### E.3 Sample implementation and test images

A sample implementation in portable C, **libpng**, is available at [`http://www.libpng.org/pub/png/libpng.html`](http://www.libpng.org/pub/png/libpng.html). Sample viewer and
encoder applications of libpng are available at [`http://www.libpng.org/pub/png/book/sources.html`](http://www.libpng.org/pub/png/book/sources.html) and are
described in detail in *PNG: The Definitive Guide* [[ROELOFS](045-g-2-informative-references.md#bib-roelofs "PNG: The Definitive Guide")]. Test images can also be accessed from the PNG web
site.

<a id="F-ChangeList"></a>

<a id="f-changes"></a>

## F. Changes

*This section is non-normative.*

<a id="changes-since-the-proposed-recommendation-of-15-may-2025"></a>

<a id="changes-20250515"></a>

### F.1 Changes since the [Proposed Recommendation of 15 May 2025](https://www.w3.org/TR/2025/PR-png-3-20250515/)

- *none*

<a id="changes-since-the-candidate-recommendation-snapshot-of-13-march-2025"></a>

<a id="changes-20250313"></a>

### F.2 Changes since the [Candidate Recommendation Snapshot of 13 March 2025](https://www.w3.org/TR/2025/CR-png-3-20250313/)

- *none*

<a id="changes-since-the-candidate-recommendation-draft-of-21-january-2025-third-edition"></a>

<a id="changes-20250121/"></a>

### F.3 Changes since the [Candidate Recommendation Draft of 21 January 2025 (Third Edition)](https://www.w3.org/TR/2025/CRD-png-3-20250121/)

- Credited original authors of the eXIf chunk
- Removed pointless constraint that unsigned integers cannot be negative (!)
- Moved "Collection" keyword from extensions document to main specification
- Copied original security note regarding EXIF, from Extensions document
- Updated reference to latest version of ITU H.273

<a id="changes-since-the-candidate-recommendation-draft-of-18-july-2024-third-edition"></a>

<a id="changes-20240718"></a>

### F.4 Changes since the [Candidate Recommendation Draft of 18 July 2024 (Third Edition)](https://www.w3.org/TR/2024/CRD-png-3-20240718/)

- Renamed mDCv to mDCV
- Renamed cLLi to cLLI
- Consistently require that acTL only comes before IDAT, not PLTE

<a id="changes-since-the-candidate-recommendation-snapshot-of-21-september-2023-third-edition"></a>

<a id="changes-20230921"></a>

### F.5 Changes since the [Candidate Recommendation Snapshot of 21 September 2023 (Third Edition)](https://www.w3.org/TR/2023/CR-png-3-20230921/)

- Clarified that bit depth and color type fields can take private values
- Listed both decimal and hexadecimal values in MaxCLL and MaxFALL examples
- Corrected terminology in mDCV section
- Corrected "PNG image" in cHRM section
- Consolidated color chunk precedence information
- Added order of precedence for color space chunks
- Clarified color chunk priorities description
- Mark SMPTE RP 2077 as informative, since it is not freely available
- Added color chunk priority table
- Clarified alpha channels are always full-range
- Added EOTF and OETF definitions
- Added recommendation for handling negative values resulting from narrow-range images using an extended range transfer function
- Clarified "video" from "video full range flag) is used to match H.273 wording but still applies to still images
- Added link to IANA subtag registry
- Fixed links to alhpa compaction and alpha separation sections
- Fixed broken tRNS link
- Added Display P3 cICP example
- Changed "perceived" wording to "measured", as luminance and chromaticity don't define a perceived color
- Clarified which metadata forms part of HDR10
- Updated link to latest version of ITU-R BT.2390
- Noted that currently there is no published standard to adapt an SDR image from default viewing conditions (display luminance and ambient illumination) to those given in mDCV
- Noted that the adaptation of BT.2100 HLG images to differing viewing conditions, given in BT.2390, may also be used with SDR images.
- Noted that tone mapping, to adjust the luminance levels given in cLLI to those of a target display to prevent clipping, is particularly important for formats such as BT.2100 PQ which use absolute luminance.
- Noted that use of full-range BT.709 values, while common, is not part of the BT.709 standard
- Added mention of protected code values for Serial Digital Interface (baseband video) in description of narrow-range and full-range video
- Added reference to ITU-R-BT.2390
- Changed description of mDCV to focus on BT.2100 PQ, as use with other formats is currently rare
- Clarified matching requirements for 16-bit tRNS matching
- Used more precise "primary chromaticities" rather than just "primaries"
- Added requirement that mDCV requires an accompanying cICP to fit with expectations of existing HDR10 workflows
- Better explanation of `Video Full Range Flag`
- Clarified ordering and encoding of sequence numbers, clarified fdAT concatenation is in order of sequence number
- Added missing mentions of cICP in descriptions of related chunks
- Fully specified the ordering and byte-order of fields in mDCV
- Added missing definition of PNG two-byte unsigned integer
- Used Display P3, rather than sRGB, in SDR mDCV examples
- Added Chris Needham as co-author
- Updated examples for mDCV, added reference to SMPTE-ST-2086.
- Added reference to Display P3
- Explicitly mentioned that the concatenated data from IDAT and fdAT may include chunks with zero-length data
- Assorted non-substantive improvements to markup, styling, internal cross-linking, correction of typos, punctuation, grammar, consistent use of US English, etc

<a id="changes-since-the-working-draft-of-20-july-2023-third-edition"></a>

<a id="changes-20230720"></a>

### F.6 Changes since the [Working Draft of 20 July 2023 (Third Edition)](https://www.w3.org/TR/2023/WD-png-3-20230720/)

- Clarified descriptions of mDCV and cLLi
- Added note to Security Considerations about potentially malicious data after IEND.
- Clarified that [cLLI](018-11-3-2-8-clli-content-light-level-information.md#cLLI-chunk) is for HDR content
- Added an informative reference to Smith & Zink "On the Calculation and Usage of HDR Static Content Metadata"
- Added an informative reference to CTA-861.3-A for static HDR metadata
- Fixed broken reference to ITU-R BT.2100
- Updated reference to SMPTE-ST-2067-21
- Added guidance on calculating MaxCLL and MaxFALL values
- Added example (live streaming) where [cLLI](018-11-3-2-8-clli-content-light-level-information.md#cLLI-chunk) could not be pre-calculated
- Added definitions for stop, SDR, HDR, HLG and PQ
- Clarified definition of narrow-range
- Updated ITU-T H Suppl. 19 reference to latest version
- Added Simon Thompson as an author
- Mandated current browser handling of out-of-range palette indices
- Updated "Additional Information" table to add [mDCV](017-11-3-2-7-mdcv-mastering-display-color-volume.md#mDCV-chunk) and [cLLI](018-11-3-2-8-clli-content-light-level-information.md#cLLI-chunk).

