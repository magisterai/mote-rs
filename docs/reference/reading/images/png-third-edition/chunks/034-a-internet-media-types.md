# A. Internet Media Types

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](033-14-3-ordering-of-chunks.md) · [Next](035-c-gamma-and-chromaticity.md)

<a id="A-Conventions"></a>

<a id="IANA-registrations"></a>

## A. Internet Media Types

<a id="image-png"></a>

<a id="A-Media-type"></a>

### A.1 image/png

This updates the existing image/png Internet Media type, under the image
top level type. This appendix is in conformance with [BCP 13](https://www.rfc-editor.org/info/bcp13) and [W3CRegMedia](https://www.w3.org/2020/01/registering-mediatypes.html).

Media type name:
:   image

Media subtype name:
:   png

Required parameters:
:   None

Optional parameters:
:   None

Encoding considerations:
:   binary

Security considerations:
:   A PNG document is composed of a collection of explicitly typed "chunks". For each of the chunk types defined in the
    PNG specification (except for gIFx), the only effect associated with those chunks is to cause
    an image to be rendered on the recipient's display or printer.

    The gIFx chunk type is used to encapsulate Application Extension data, and some use of that
    data might present security risks, though no risks are known. Likewise, the security risks associated with future chunk
    types cannot be evaluated, particularly unregistered chunks. However, it is the intention of the PNG Working Group to
    disallow chunks containing "executable" data to become registered chunks.

    The text chunks, [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt), [iTXT](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt), contain data that can be displayed in the form of comments, etc. Some operating systems
    or terminals might allow the display of textual data with embedded control characters to perform operations such as
    re-mapping of keys, creation of files, etc. For this reason, the specification recommends that the text chunks be
    filtered for control characters before direct display.

    The PNG format is specifically designed to facilitate early detection of file transmission errors, and makes use of
    cyclical redundancy checks to ensure the integrity of the data contained in its chunks.

Interoperability considerations:
:   Network byte order used throughout.

Published specification:
:   [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/PNG/), [https://www.w3.org/TR/PNG/](https://www.w3.org/TR/PNG/)

Applications which use this media:
:   PNG is widely implemented in all Web browsers, image viewers, and image creation tools

Fragment identifier considerations:
:   N/A

Restrictions on usage:
:   N/A

Provisional registration? (standards tree only):
:   No

Additional information:
:   Deprecated alias names for this type:
    :   N/A

    Magic number(s):
    :   89 50 4E 47 0D 0A 1A 0A

    File extension(s):
    :   .png

    Macintosh file type code:
    :   PNGf

    Object Identifiers:
    :   N/A

General Comments:
:   This registration updates the earlier one:

    1. The old one points to an expired Internet Draft. This updated registration points to a W3C Recommendation.
    2. The old contact person is sadly deceased. The new contact email is a publicly archived W3C mailing list for the PNG
       Working Group.
    3. Change controller is W3C

Person to contact for further information:
:   Name:
    :   PNG Working Group

    Email:
    :   [public-png@w3.org](mailto:public-png@w3.org)

Intended usage:
:   Common

Author/Change controller:
:   W3C

<a id="image-apng"></a>

<a id="a-2-image-apng"></a>

### A.2 image/apng

This appendix is in conformance with [BCP 13](https://www.rfc-editor.org/info/bcp13) and [W3CRegMedia](https://www.w3.org/2020/01/registering-mediatypes.html).

Media type name:
:   image

Media subtype name:
:   apng

Required parameters:
:   N/A

Optional parameters:
:   N/A

Encoding considerations:
:   binary

Security considerations:
:   An APNG document is composed of a collection of explicitly typed "chunks". For each of the chunk types defined in the
    PNG specification (except for gIFx), the only effect associated with those chunks is to cause
    an animated image to be rendered on the recipient's display.

    The gIFx chunk type is used to encapsulate Application Extension data, and some use of that
    data might present security risks, though no risks are known. Likewise, the security risks associated with future chunk
    types cannot be evaluated, particularly unregistered chunks. However, it is the intention of the PNG Working Group to
    disallow chunks containing "executable" data to become registered chunks.

    The text chunks, [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt), [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt), contain data that can be displayed in the form of comments, etc. Some operating systems
    or terminals might allow the display of textual data with embedded control characters to perform operations such as
    re-mapping of keys, creation of files, etc. For this reason, the specification recommends that the text chunks be
    filtered for control characters before direct display.

    The PNG format is specifically designed to facilitate early detection of file transmission errors, and makes use of
    cyclical redundancy checks to ensure the integrity of the data contained in its chunks.

    If one creates an APNG file with unrelated static image and animated image chunks, somebody using a tool not
    supporting the APNG format would only see the static image and be unaware of the additional content. This could be used
    e.g. to bypass moderation.

Interoperability considerations:
:   None

Published specification:
:   [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/png/), [https://www.w3.org/TR/png/](https://www.w3.org/TR/png/)

Applications which use this media:
:   Animated PNG (APNG) is widely implemented in all Web browsers, and is increasingly available in image viewers, and
    animation and image creation tools

Fragment identifier considerations:
:   N/A

Restrictions on usage:
:   N/A

Provisional registration? (standards tree only):
:   No

Additional information:
:   Deprecated alias names for this type:
    :   image/vnd.mozilla.apng

    Magic number(s):
    :   89 50 4E 47 0D 0A 1A 0A

    File extension(s):
    :   .apng

    Object Identifiers:
    :   N/A

General Comments:
:   image/apng has been in widespread, unregistered use since 2015. Animated PNG was not part of the official PNG
    specification until 2022. This registration, plus the PNG specification (3rd Edition) brings official documentation into
    alignment with already widely-deployed reality.

Person to contact for further information:
:   Name:
    :   PNG Working Group

    Email:
    :   [public-png@w3.org](mailto:public-png@w3.org)

Intended usage:
:   Common

Author/Change controller:
:   W3C

<a id="B-NewChunksAppendix"></a>

<a id="newchunks"></a>

## B. Guidelines for private chunk types

*This section is non-normative.*

The following specifies guidelines for the definition of private chunks:

1. Do not define new chunks that redefine the meaning of existing chunks or change the interpretation of an existing
   standardized chunk, e.g., do not add a new chunk to say that RGB and alpha values actually mean CMYK.
2. Minimize the use of private chunks to aid portability.
3. Avoid defining chunks that depend on total datastream contents. If such chunks have to be defined, make them critical
   chunks.
4. For textual information that is representable in Latin-1 avoid defining a new chunk type. Use a [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) or [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunk with a suitable keyword to identify the type of
   information. For textual information that is not representable in Latin-1 but which can be represented in UTF-8, use an
   [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunk with a suitable keyword.
5. Group mutually dependent ancillary information into a single chunk. This avoids the need to introduce chunk ordering
   relationships.
6. Avoid defining private critical chunks.

