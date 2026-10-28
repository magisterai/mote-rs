# Portable Network Graphics (PNG) Specification (Third Edition)

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Next](002-2-scope.md)

[W3C](https://www.w3.org/)

<a id="title"></a>

# Portable Network Graphics (PNG) Specification (Third Edition)

<a id="w3c-state"></a>

[W3C Recommendation](https://www.w3.org/standards/types#REC) 24 June 2025

More details about this document

This version:
:   [https://www.w3.org/TR/2025/REC-png-3-20250624/](../README.md)

Latest published version:
:   [https://www.w3.org/TR/png-3/](https://www.w3.org/TR/png-3/)

Latest editor's draft:
:   [https://w3c.github.io/png/](https://w3c.github.io/png/)

History:
:   [https://www.w3.org/standards/history/png-3/](https://www.w3.org/standards/history/png-3/)
:   [Commit history](https://github.com/w3c/png/commits/main)

Implementation report:
:   [https://w3c.github.io/png/Implementation\_Report\_3e/](https://w3c.github.io/png/Implementation_Report_3e/)

Editors:
:   [Chris Blume](https://www.programmax.net) (W3C Invited Experts)
:   [Pierre-Anthony Lemieux](mailto:pal@palemieux.com) ([MovieLabs](https://movielabs.com/))
:   [Chris Lilley](https://svgees.us) ([W3C](https://www.w3.org))
:   [Chris Needham](https://chrisneedham.com/about)
:   Leonard Rosenthol ([Adobe Inc.](https://www.adobe.com))
:   Chris Arley Seeger ([NBCUniversal, LLC a subsidiary of Comcast Corporation](https://www.xfinity.com))
:   Simon Thompson ([British Broadcasting Corporation](https://www.bbc.co.uk))
:   Cosmin Truta (W3C Invited Experts)

Former editors:
:   Thomas Boutell
:   Adam M. Costello
:   David Duce
:   Tom Lane
:   Glenn Randers-Pehrson

Authors:
:   [Mark Adler](https://en.wikipedia.org/wiki/Mark_Adler)
:   [Thomas Boutell](https://boutell.dev/)
:   Christian Brunschen
:   [Adam M. Costello](http://www.nicemice.net/amc/)
:   Lee Daniel Crocker
:   [Andreas Dilger](http://www-mddsp.enel.ucalgary.ca/People/adilger/)
:   [Oliver Fromme](https://www.fromme.com/)
:   [Jean-loup Gailly](http://gailly.net/)
:   Phil Harvey
:   Chris Herborth
:   Alex Jakulin
:   Neal Kettler
:   Tom Lane
:   Alexander Lehmann
:   [Chris Lilley](https://svgees.us) ([W3C](https://www.w3.org))
:   Dave Martindale
:   Owen Mortensen
:   Stuart Parmenter
:   Keith S. Pickens
:   Robert P. Poole
:   Glenn Randers-Pehrson
:   [Greg Roelofs](http://www.gregroelofs.com/)
:   Willem van Schaik
:   Guy Schalnat
:   Paul Schmidt
:   Andrew Smith
:   Michael Stokes
:   Vladimir Vukicevic
:   Tim Wegner
:   Jeremy Wohl

Feedback:
:   [GitHub w3c/png](https://github.com/w3c/png/)
    ([pull requests](https://github.com/w3c/png/pulls/),
    [new issue](https://github.com/w3c/png/issues/new/choose),
    [open issues](https://github.com/w3c/png/issues/))
:   [public-png@w3.org](mailto:public-png@w3.org?subject=%5Bpng-3%5D%20YOUR%20TOPIC%20HERE) with subject line `[png-3] … message topic …` ([archives](https://lists.w3.org/Archives/Public/public-png))

Errata:
:   [https://www.w3.org/2025/06/REC-PNG-20250624-errata](https://www.w3.org/2025/06/REC-PNG-20250624-errata)

See also [**translations**](https://www.w3.org/Translations/?technology=png-3).

[Copyright](https://www.w3.org/policies/#copyright)
©
1996-2025
[World Wide Web Consortium](https://www.w3.org/).
W3C<sup>®</sup>
[liability](https://www.w3.org/policies/#Legal_Disclaimer),
[trademark](https://www.w3.org/policies/#W3C_Trademarks) and
[permissive document license](https://www.w3.org/copyright/software-license-2023/ "W3C Software and Document Notice and License") rules apply.

---

<a id="abstract"></a>

## Abstract

<a id="ref-for-dfn-lossless-1"></a>

<a id="ref-for-3indexedColour-1"></a>

<a id="ref-for-dfn-greyscale-1"></a>

<a id="ref-for-3truecolour-1"></a>

This document describes PNG (Portable Network Graphics), an extensible file format for the [lossless](002-2-scope.md#dfn-lossless), portable,
well-compressed storage of static and animated raster images. PNG provides a patent-free replacement for GIF and can also
replace many common uses of TIFF. [Indexed-color](004-4-3-color-spaces.md#3indexedColour), [greyscale](005-4-5-png-image.md#dfn-greyscale), and [truecolor](005-4-5-png-image.md#3truecolour) images are supported, plus an optional alpha
channel. Sample depths range from 1 to 16 bits.

PNG is designed to work well in online viewing applications, such as the World Wide Web, so it is fully streamable with a
progressive display option. PNG is robust, providing both full file integrity checking and simple detection of common
transmission errors. Also, PNG can store color space data for improved color matching on heterogeneous platforms.

This specification defines two Internet Media Types, image/png and image/apng.

<a id="sotd"></a>

## Status of This Document

*This section describes the status of this
document at the time of its publication. A list of current W3C
publications and the latest revision of this technical report can be found
in the
[W3C standards and drafts index](https://www.w3.org/TR/) at
https://www.w3.org/TR/.*

This specification is intended to become an International Standard, but is not yet one. It is inappropriate to refer to this
specification as an International Standard.

This document was published by the [Portable Network Graphics (PNG) Working Group](https://www.w3.org/groups/wg/png) as a Recommendation using the [Recommendation track](https://www.w3.org/policies/process/20231103/#recs-and-notes).

A W3C Recommendation is a specification that, after extensive consensus-building, is endorsed by W3C and its Members, and has commitments from Working Group members to [royalty-free licensing](https://www.w3.org/policies/patent-policy/#sec-Requirements) for implementations.

W3C recommends the wide deployment of this specification as a standard for the Web.

This document was produced by a group
operating under the
[W3C Patent
Policy](https://www.w3.org/policies/patent-policy/).
W3C maintains a
[public list of any patent disclosures](https://www.w3.org/groups/wg/png/ipr)
made in connection with the deliverables of
the group; that page also includes
instructions for disclosing a patent. An individual who has actual
knowledge of a patent which the individual believes contains
[Essential Claim(s)](https://www.w3.org/policies/patent-policy/#def-essential)
must disclose the information in accordance with
[section 6 of the W3C Patent Policy](https://www.w3.org/policies/patent-policy/#sec-Disclosure).

<a id="w3c_process_revision"></a>

This document is governed by the
[03 November 2023 W3C Process Document](https://www.w3.org/policies/process/20231103/).

<a id="introduction"></a>

<a id="Introduction"></a>

## 1. Introduction

The design goals for this specification were:

1. Portability: encoding, decoding, and transmission should be software and hardware platform independent.

<a id="ref-for-3truecolour-2"></a>

<a id="ref-for-3indexedColour-2"></a>

<a id="ref-for-dfn-greyscale-2"></a>

2. Completeness: it should be possible to represent [truecolor](005-4-5-png-image.md#3truecolour), [indexed-color](004-4-3-color-spaces.md#3indexedColour), and [greyscale](005-4-5-png-image.md#dfn-greyscale) images, in each case with
   the option of transparency, color space information, and ancillary information such as textual comments.
3. Serial encode and decode: it should be possible for datastreams to be generated serially and read serially, allowing the
   datastream format to be used for on-the-fly generation and display of images across a serial communication channel.
4. Progressive presentation: it should be possible to transmit datastreams so that an approximation of the whole image can be
   presented initially, and progressively enhanced as the datastream is received.
5. Robustness to transmission errors: it should be possible to detect datastream transmission errors reliably.
6. Losslessness: filtering and compression should preserve all information.
7. Performance: any filtering, compression, and progressive image presentation should be aimed at efficient decoding and
   presentation. Fast encoding is a less important goal than fast decoding. Decoding speed may be achieved at the expense of
   encoding speed.
8. Compression: images should be compressed effectively, consistent with the other design goals.
9. Simplicity: developers should be able to implement the standard easily.
10. Interchangeability: any standard-conforming PNG decoder shall be capable of reading all conforming PNG datastreams.
11. Flexibility: future extensions and private additions should be allowed for without compromising the interchangeability of
    standard PNG datastreams.
12. Freedom from legal restrictions: no algorithms should be used that are not freely available.

