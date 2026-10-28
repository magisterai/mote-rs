# 13.2 Error checking

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](025-13-1-error-handling.md) · [Next](027-13-6-pixel-dimensions.md)

<a id="13Error-checking"></a>

<a id="x13-2-error-checking"></a>

### 13.2 Error checking

The PNG error handling philosophy is described in [13.1 Error handling](025-13-1-error-handling.md#13Decoders.Errors).

An unknown chunk type is **not** to be treated as an error unless it is a critical chunk.

The chunk type can be checked for plausibility by seeing whether all four bytes are in the range codes 41-5A and 61-7A
(hexadecimal); note that this need be done only for unrecognized chunk types. If the total datastream size is known (from
file system information, HTTP protocol, etc), the chunk length can be checked for plausibility as well. If CRCs are
not checked, dropped/added data bytes or an erroneous chunk length can cause the decoder to get out of step and misinterpret
subsequent data as a chunk header.

For known-length chunks, such as [IHDR](014-10-3-other-uses-of-compression.md#11IHDR), decoders should treat an unexpected chunk
length as an error. Future extensions to this specification will not add new fields to existing chunks; instead, new chunk
types will be added to carry new information.

Unexpected values in fields of known chunks (for example, an unexpected compression method in the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk) shall be checked for and treated as errors. However, it is recommended that unexpected field values
be treated as fatal errors only in **critical** chunks. An unexpected value in an ancillary chunk can be handled
by ignoring the whole chunk as though it were an unknown chunk type. (This recommendation assumes that the chunk's CRC
has been verified. In decoders that do not check CRCs, it is safer to treat any unexpected value as indicating a
corrupted datastream.)

Standard PNG images shall be compressed with compression method 0. The compression method field of the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk is provided for possible future standardization or proprietary variants. Decoders shall check
this byte and report an error if it holds an unrecognized code. See [10. Compression](013-9-3-filter-type-3-average.md#10Compression) for details.

<a id="13Security-considerations"></a>

<a id="x13-3-security-considerations"></a>

### 13.3 Security considerations

A PNG datastream is composed of a collection of explicitly typed chunks.
Chunks whose contents are defined by the
specification could actually contain anything, including malicious code.
Similarly there could be data after the [IEND](014-10-3-other-uses-of-compression.md#11IEND) chunk
which could contain anything, including malicious code.
There is no known risk that such malicious code
could be executed on the recipient's computer *as a result of decoding the PNG image*.
However, a malicious application might hide such code inside an innocent-looking image file
and then execute it.

The possible security risks associated with future chunk types cannot be specified at this time. Security issues will be
considered when defining future public chunks. There is no additional security risk associated with unknown or unimplemented
chunk types, because such chunks will be ignored, or at most be copied into another PNG datastream.

The [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt), [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt), and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks contain keywords and data that are meant to be displayed as plain text. The [iCCP](015-11-3-ancillary-chunks.md#11iCCP) and [sPLT](019-11-3-3-4-itxt-international-textual-data.md#11sPLT) chunks contain keywords that are meant to be displayed as
plain text. It is possible that if the decoder displays such text without filtering out control characters, especially the
ESC (escape) character, certain systems or terminals could behave in undesirable and insecure ways. It is recommended that
decoders filter out control characters to avoid this risk; see [13.7 Text chunk processing](027-13-6-pixel-dimensions.md#13Text-chunk-processing).

For the [eXIf](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#eXIf) chunk, the Exif Specification [[CIPA-DC-008](044-g-references.md#bib-cipa-dc-008 "Exchangeable image file format for digital still cameras: Exif Version 2.32")] does not contain an express
requirement that tag "value offset" pointers must actually point to a valid address within the file.
This requirement is merely implied. (See Paragraph 4.6.2, which describes the Exif IFD structure.)
Regardless, decoders should be prepared to encounter invalid pointers and to handle them appropriately.

Every chunk begins with a length field, which makes it easier to write decoders that are invulnerable to fraudulent chunks
that attempt to overflow buffers. The CRC at the end of every chunk provides a robust defence against accidentally
corrupted data. The PNG signature bytes provide early detection of common file transmission errors.

A decoder that fails to check CRCs could be subject to data corruption. The only likely consequence of such
corruption is incorrectly displayed pixels within the image. Worse things might happen if the CRC of the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk is not checked and the width or height fields are corrupted. See [13.2 Error checking](026-13-2-error-checking.md#13Error-checking).

A poorly written decoder might be subject to buffer overflow, because chunks can be extremely large, up to
2<sup>31</sup>-1 bytes long. But properly written decoders will handle large chunks without difficulty.

<a id="Privacy-considerations"></a>

<a id="x13-4-privacy-considerations"></a>

### 13.4 Privacy considerations

Some image editing tools have historically performed redaction
by merely setting the alpha channel of the redacted area to zero,
without also removing the actual image data.
Users who rely solely on the visual appearance of such images
run a privacy risk
because the actual image data can be easily recovered.

Similarly, some image editing tools have historically performed clipping
by rewriting the width and height in IHDR
without re-encoding the image data,
which thus extends beyond the new width and height and may be recovered.

Images with eXIf chunks
may contain automatically-included data,
such as photographic GPS coordinates,
which could be a privacy risk if the user is unaware that the PNG image contains this data.
(Other image formats that contain EXIF, such as JPEG/JFIF, have the same privacy risk).

<a id="13Chunking"></a>

<a id="x13-5-chunking"></a>

### 13.5 Chunking

Decoders shall recognize chunk types by a simple four-byte literal comparison; it is incorrect to perform case conversion
on chunk types. A decoder encountering an unknown chunk in which the ancillary bit is 1 may safely ignore the chunk and
proceed to display the image. A decoder trying to extract the image, upon encountering an unknown chunk in which the
ancillary bit is 0, indicating a critical chunk, shall indicate to the user that the image contains information it cannot
safely interpret.

Decoders should test the properties of an unknown chunk type by numerically testing the specified bits. Testing whether a
character is uppercase or lowercase is inefficient, and even incorrect if a locale-specific case definition is used.

Decoders should not flag an error if the reserved bit is set to 1, however, as some future version of the PNG
specification could define a meaning for this bit. It is sufficient to treat a chunk with this bit set in the same way as any
other unknown chunk type.

Decoders do not need to test the chunk type private bit, since it has no functional significance and is used to avoid
conflicts between chunks defined by W3C and those defined privately.

All ancillary chunks are optional; decoders may ignore them. However, decoders are encouraged to interpret these chunks
when appropriate and feasible.

