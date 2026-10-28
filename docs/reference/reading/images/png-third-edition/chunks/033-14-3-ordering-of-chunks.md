# 14.3 Ordering of chunks

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](032-13-17-histogram-and-suggested-palette-usage.md) · [Next](034-a-internet-media-types.md)

<a id="14Ordering-of-chunks"></a>

<a id="x14-3-ordering-of-chunks"></a>

### 14.3 Ordering of chunks

<a id="14Ordering-of-critical-chunks"></a>

<a id="x14-3-1-ordering-of-critical-chunks"></a>

#### 14.3.1 Ordering of critical chunks

<a id="ref-for-dfn-png-editor-18"></a>

Critical chunks may have arbitrary ordering requirements, because [PNG editors](002-2-scope.md#dfn-png-editor) are required to terminate if they
encounter unknown critical chunks. For example [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) has the specific ordering rule that
it shall always appear first. A PNG editor, or indeed any PNG-writing program, shall know and follow the ordering rules for
any critical chunk type that it can generate.

<a id="14Ordering-of-ancillary-chunks"></a>

<a id="x14-3-2-ordering-of-ancillary-chunks"></a>

#### 14.3.2 Ordering of ancillary chunks

The strictest ordering rules for an ancillary chunk type are:

1. Unsafe-to-copy chunks may have ordering requirements relative to critical chunks.
2. Safe-to-copy chunks may have ordering requirements relative to [IDAT](014-10-3-other-uses-of-compression.md#11IDAT).

The actual ordering rules for any particular ancillary chunk type may be weaker. See for example the ordering rules for
the standard ancillary chunk types in [5.6 Chunk ordering](009-5-5-crc-algorithm.md#5ChunkOrdering).

Decoders shall not assume more about the positioning of any ancillary chunk than is specified by the chunk ordering
rules. In particular, it is never valid to assume that a specific ancillary chunk type occurs with any particular
positioning relative to other ancillary chunks.

<a id="ref-for-dfn-png-editor-19"></a>

EXAMPLE It is unsafe to assume that a particular private ancillary chunk occurs immediately before [IEND](014-10-3-other-uses-of-compression.md#11IEND). Even if it is always written in that position by a particular application, a [PNG editor](002-2-scope.md#dfn-png-editor)
might have inserted some other ancillary chunk after it. But it is safe to assume that the chunk will remain somewhere
between [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) and [IEND](014-10-3-other-uses-of-compression.md#11IEND).

<a id="15Conformance"></a>

<a id="conformance"></a>

<a id="x15-conformance"></a>

## 15. Conformance

As well as sections marked as non-normative, all authoring guidelines, diagrams, examples, and notes in this specification are non-normative. Everything else in this specification is normative.

The key words *MAY*, *MUST*, *SHALL*, *SHOULD*, and *SHOULD NOT* in this document
are to be interpreted as described in
[BCP 14](https://www.rfc-editor.org/info/bcp14)
[[RFC2119](044-g-references.md#bib-rfc2119 "Key words for use in RFCs to Indicate Requirement Levels")] [[RFC8174](044-g-references.md#bib-rfc8174 "Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words")]
when, and only when, they appear in all
capitals, as shown here.

<a id="conformance-0"></a>

<a id="x15-1-conformance"></a>

### 15.1 Conformance

<a id="15ConfIntro"></a>

<a id="x15-2-introduction"></a>

### 15.2 Introduction

<a id="15ConfObjectives"></a>

<a id="x15-2-1-objectives"></a>

#### 15.2.1 Objectives

<a id="ref-for-dfn-png-editor-20"></a>

This clause addresses conformance of PNG datastreams, PNG encoders, PNG decoders, and [PNG editors](002-2-scope.md#dfn-png-editor).

The primary objectives of the specifications in this clause are:

1. to promote interoperability by eliminating arbitrary subsets of, or extensions to, this specification;
2. to promote uniformity in the development of conformance tests;
3. to promote consistent results across PNG encoders, decoders, and editors;
4. to facilitate automated test generation.

<a id="15ConfScope"></a>

<a id="x15-2-2-scope"></a>

#### 15.2.2 Scope

Conformance is defined for PNG datastreams and for PNG encoders, decoders, and editors.

<a id="ref-for-dfn-png-editor-21"></a>

This clause addresses the PNG datastream and implementation requirements including the range of allowable differences
for PNG encoders, PNG decoders, and [PNG editors](002-2-scope.md#dfn-png-editor). This clause does not directly address the environmental,
performance, or resource requirements of the encoder, decoder, or editor.

The scope of this clause is limited to rules for the open interchange of PNG datastreams.

<a id="15ConformanceConf"></a>

<a id="x15-3-conformance-conditions"></a>

### 15.3 Conformance conditions

<a id="15FileConformance"></a>

<a id="x15-3-1-conformance-of-png-datastreams"></a>

#### 15.3.1 Conformance of PNG datastreams

A PNG datastream conforms to this specification if the following conditions are met.

1. The PNG datastream contains a PNG signature as the first content (see [5.2 PNG signature](008-4-10-error-handling.md#5PNG-file-signature)).
2. With respect to the chunk types defined in this International Standard:
   - the PNG datastream contains as its first chunk, an [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk, immediately
     following the PNG signature;
   - the PNG datastream contains as its last chunk, an [IEND](014-10-3-other-uses-of-compression.md#11IEND) chunk.
3. No chunks or other content follow the [IEND](014-10-3-other-uses-of-compression.md#11IEND) chunk.
4. All chunks contained therein match the specification of the corresponding chunk types of this specification. The PNG
   datastream shall obey the relationships among chunk types defined in this specification.
5. The sequence of chunks in the PNG datastream obeys the ordering relationship specified in this International
   Standard.
6. All field values in the PNG datastream obey the relationships specified in this specification producing the structure
   specified in this specification.
7. No chunks appear in the PNG datastream other than those specified in this specification or those defined according to
   the rules for creating new chunk types as defined in this specification.
8. The PNG datastream is encoded according to the rules of this International Standard.

<a id="15ConformanceEncoder"></a>

<a id="x15-3-2-conformance-of-png-encoders"></a>

#### 15.3.2 Conformance of PNG encoders

A PNG encoder conforms to this specification if it satisfies the following conditions.

1. All PNG datastreams that are generated by the PNG encoder are conforming PNG datastreams.
2. When encoding input samples that have a sample depth that cannot be directly represented in PNG, the encoder scales
   the samples up to the next higher sample depth that is allowed by PNG. The data are scaled in such a way that the
   high-order bits match the original data.

<a id="ref-for-dfn-private-field-values-5"></a>

3. [Private field values](010-5-7-defining-chunks.md#dfn-private-field-values) are used when encoding experimental or private definitions of values for any of the method
   or type fields.

<a id="15ConformanceDecoder"></a>

<a id="x15-3-3-conformance-of-png-decoders"></a>

#### 15.3.3 Conformance of PNG decoders

A PNG decoder conforms to this specification if it satisfies the following conditions.

1. It is able to read any PNG datastream that conforms to this International Standard, including both public and private
   chunks whose types may not be recognized.
2. It supports all the standardized critical chunks, and all the standardized compression, filter, and interlace methods
   and types in any PNG datastream that conforms to this International Standard.
3. Unknown chunk types are handled as described in [5.4 Chunk naming conventions](008-4-10-error-handling.md#5Chunk-naming-conventions). An unknown chunk type is
   **not** treated as an error unless it is a critical chunk.
4. Unexpected values in fields of known chunks (for example, an unexpected compression method in the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk) are treated as errors.

<a id="ref-for-3truecolour-33"></a>

<a id="ref-for-dfn-greyscale-18"></a>

<a id="ref-for-dfn-truecolor-with-alpha-8"></a>

<a id="ref-for-dfn-greyscale-with-alpha-4"></a>

<a id="ref-for-3truecolour-34"></a>

5. All types of PNG images (indexed-color, [truecolor](005-4-5-png-image.md#3truecolour), [greyscale](005-4-5-png-image.md#dfn-greyscale), [truecolor with alpha](005-4-5-png-image.md#dfn-truecolor-with-alpha), and [greyscale
   with alpha](005-4-5-png-image.md#dfn-greyscale-with-alpha)) are processed. For example, decoders which are part of viewers running on indexed-color display hardware
   shall reduce [truecolor](005-4-5-png-image.md#3truecolour) images to indexed format for viewing.
6. Encountering an unknown chunk in which the ancillary bit is 0 generates an error if the decoder is attempting to
   extract the image.
7. A chunk type in which the reserved bit is set is treated as an unknown chunk type.

<a id="ref-for-3colourType-42"></a>

8. All valid combinations of bit depth and [color type](010-5-7-defining-chunks.md#3colourType) as defined in [11.2.1 IHDR Image header](014-10-3-other-uses-of-compression.md#11IHDR) are
   supported.

<a id="ref-for-3colourType-43"></a>

<a id="ref-for-3filter-14"></a>

9. An error is reported if an unrecognized value is encountered in the bit depth, [color type](010-5-7-defining-chunks.md#3colourType), compression
   method, [filter method](012-7-3-filtering.md#3filter), or interlace method bytes of the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk.

<a id="ref-for-dfn-greyscale-19"></a>

<a id="ref-for-3truecolour-35"></a>

10. When processing 16-bit [greyscale](005-4-5-png-image.md#dfn-greyscale) or [truecolor](005-4-5-png-image.md#3truecolour) data in the [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk,
    both bytes of the sample values are evaluated to determine whether a pixel is transparent.

<a id="ref-for-dfn-image-data-36"></a>

11. When processing an image compressed by compression method 0, the decoder assumes no more than that the complete
    [image data](002-2-scope.md#dfn-image-data) is represented by a single compressed datastream that is stored in some number of [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks.
12. No assumptions are made concerning the positioning of any ancillary chunk other than those that are specified by the
    chunk ordering rules.

<a id="15ConformanceEditor"></a>

<a id="x15-3-4-conformance-of-png-editors"></a>

#### 15.3.4 Conformance of PNG editors

<a id="ref-for-dfn-png-editor-22"></a>

A [PNG editor](002-2-scope.md#dfn-png-editor) conforms to this specification if it satisfies the following conditions.

1. It conforms to the requirements for PNG encoders.
2. It conforms to the requirements for PNG decoders.
3. It is able to encode all chunks that it decodes.
4. It preserves the ordering of the chunks presented within the rules in [5.6 Chunk ordering](009-5-5-crc-algorithm.md#5ChunkOrdering).
5. It properly processes the safe-to-copy bit information and preserves unknown chunks when the safe-to-copy rules
   permit it.

<a id="ref-for-3colourType-44"></a>

<a id="ref-for-3indexedColour-21"></a>

6. Unless the user specifically permits lossy operations or the editor issues a warning, it preserves all information
   required to reconstruct the reference image exactly, except that the sample depth of the alpha channel need not be
   preserved if it contains only zero and maximum values. Operations such as changing the [color type](010-5-7-defining-chunks.md#3colourType) or rearranging
   the palette in an [indexed-color](004-4-3-color-spaces.md#3indexedColour) datastream are permitted provided that the new datastream losslessly represents the same
   reference image.

