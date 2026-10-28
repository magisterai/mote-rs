# 13.1 Error handling

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](024-12-6-interlacing.md) · [Next](026-13-2-error-checking.md)

<a id="error-handling"></a>

<a id="13Decoders.Errors"></a>

### 13.1 Error handling

Errors in a PNG datastream will fall into two general classes, transmission errors and syntax errors (see [4.10 Error handling](008-4-10-error-handling.md#4Concepts.Errors)).

Examples of transmission errors are transmission in "text" or "ascii" mode, in which byte codes 13 and/or 10 may be added,
removed, or converted throughout the datastream; unexpected termination, in which the datastream is truncated; or a physical
error on a storage device, in which one or more blocks (typically 512 bytes each) will have garbled or random values. Some
examples of syntax errors are an invalid value for a row filter, an invalid compression method, an invalid chunk length, the
absence of a [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk before the first [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk
in an indexed image, or the presence of multiple [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunks. A PNG decoder should handle
errors as follows:

1. Detect errors as early as possible using the PNG signature bytes and CRCs on each chunk. Decoders should verify
   that all eight bytes of the PNG signature are correct. A decoder can have additional confidence in the datastream's
   integrity if the next eight bytes begin an [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk with the correct chunk length. A
   CRC should be checked before processing the chunk data. Sometimes this is impractical, for example when a streaming
   PNG decoder is processing a large [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk. In this case the CRC should be
   checked when the end of the chunk is reached.
2. Recover from an error, if possible; otherwise fail gracefully. Errors that have little or no effect on the processing
   of the image may be ignored, while those that affect critical data shall be dealt with in a manner appropriate to the
   application.
3. Provide helpful messages describing errors, including recoverable errors.

Three classes of PNG chunks are relevant to this philosophy. For the purposes of this classification, an "unknown chunk"
is either one whose type was genuinely unknown to the decoder's author, or one that the author chose to treat as unknown,
because default handling of that chunk type would be sufficient for the program's purposes. Other chunks are called "known
chunks". Given this definition, the three classes are as follows:

1. known chunks, which necessarily includes all of the critical chunks defined in this specification ([IHDR](014-10-3-other-uses-of-compression.md#11IHDR), [PLTE](014-10-3-other-uses-of-compression.md#11PLTE), [IDAT](014-10-3-other-uses-of-compression.md#11IDAT), [IEND](014-10-3-other-uses-of-compression.md#11IEND))
2. unknown critical chunks (bit 5 of the first byte of the chunk type is 0)
3. unknown ancillary chunks (bit 5 of the first byte of the chunk type is 1)

See [5.4 Chunk naming conventions](008-4-10-error-handling.md#5Chunk-naming-conventions) for a description of chunk naming conventions.

PNG chunk types are marked "critical" or "ancillary" according to whether the chunks are critical for the purpose of
extracting a viewable image (as with [IHDR](014-10-3-other-uses-of-compression.md#11IHDR), [PLTE](014-10-3-other-uses-of-compression.md#11PLTE), and
[IDAT](014-10-3-other-uses-of-compression.md#11IDAT)) or critical to understanding the datastream structure (as with [IEND](014-10-3-other-uses-of-compression.md#11IEND)). This is a specific kind of criticality and one that is not necessarily relevant to every
conceivable decoder. For example, a program whose sole purpose is to extract text annotations (for example, copyright
information) does not require a viewable image
but should [decode UTF-8 correctly](https://encoding.spec.whatwg.org/#utf-8-decode).
Another decoder might consider the [tRNS](015-11-3-ancillary-chunks.md#11tRNS)
and [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunks essential to its proper execution.

<a id="ref-for-3truecolour-22"></a>

Syntax errors always involve known chunks because syntax errors in unknown chunks cannot be detected. The PNG decoder has
to determine whether a syntax error is fatal (unrecoverable) or not, depending on its requirements and the situation. For
example, most decoders can ignore an invalid [IEND](014-10-3-other-uses-of-compression.md#11IEND) chunk; a text-extraction program can
ignore the absence of [IDAT](014-10-3-other-uses-of-compression.md#11IDAT); an image viewer cannot recover from an empty [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk in an indexed image but it can ignore an invalid [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) chunk
in a [truecolor](005-4-5-png-image.md#3truecolour) image; and a program that extracts the alpha channel can ignore an invalid [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk, but may consider the presence of two [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunks to be a fatal
error. Anomalous situations other than syntax errors shall be treated as follows:

1. Encountering an unknown ancillary chunk is never an error. The chunk can simply be ignored.
2. Encountering an unknown critical chunk is a fatal condition for any decoder trying to extract the image from the
   datastream. A decoder that ignored a critical chunk could not know whether the image it extracted was the one intended by
   the encoder.
3. A PNG signature mismatch, a CRC mismatch, or an unexpected end-of-stream indicates a corrupted datastream, and
   may be regarded as a fatal error. A decoder could try to salvage something from the datastream, but the extent of the
   damage will not be known.

<a id="ref-for-dfn-image-data-21"></a>

When a fatal condition occurs, the decoder should fail immediately, signal an error to the user if appropriate, and
optionally continue displaying any [image data](002-2-scope.md#dfn-image-data) already visible to the user (i.e. "fail gracefully"). The application as
a whole need not terminate.

When a non-fatal error occurs, the decoder should signal a warning to the user if appropriate, recover from the error, and
continue processing normally.

<a id="oor-index-black"></a>

When decoding an indexed-color PNG, if out-of-range indexes are encountered, decoders have historically varied in their
handling of this error. Displaying the pixel as opaque black is one common error recovery tactic,
and is now required by this specification.
Older implementations will vary, and so the behavior
must not be relied on by encoders.

Decoders that do not compute CRCs should interpret apparent syntax errors as indications of corruption (see also
[13.2 Error checking](026-13-2-error-checking.md#13Error-checking)).

<a id="ref-for-dfn-deflate-16"></a>

Errors in compressed chunks ( [IDAT](014-10-3-other-uses-of-compression.md#11IDAT), [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt),
[iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt), [iCCP](015-11-3-ancillary-chunks.md#11iCCP)) could lead to buffer overruns.
Implementors of [deflate](002-2-scope.md#dfn-deflate) decompressors should guard against this possibility.

APNG is designed to allow incremental display of frames before the entire [datastream](008-4-10-error-handling.md#5DataRep) has been
read. This implies that some errors may not be detected until partway through the animation. It is strongly recommended that
when any error is encountered decoders should discard all subsequent frames, stop the animation, and revert to displaying the
static image. A decoder which detects an error before the animation has started should display the static image. An error
message may be displayed to the user if appropriate.

<a id="ref-for-dfn-png-editor-6"></a>

Decoders shall treat out-of-order APNG chunks as an error. APNG-aware [PNG editors](002-2-scope.md#dfn-png-editor) should restore them to correct
order, using the sequence numbers.

