# DEFLATE Compressed Data Format Specification — part 1

Source: [DEFLATE Compressed Data Format Specification](https://www.rfc-editor.org/rfc/rfc1951.txt) · RFC 1951.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc1951.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text






Network Working Group                                         P. Deutsch
Request for Comments: 1951                           Aladdin Enterprises
Category: Informational                                         May 1996


        DEFLATE Compressed Data Format Specification version 1.3

Status of This Memo

   This memo provides information for the Internet community.  This memo
   does not specify an Internet standard of any kind.  Distribution of
   this memo is unlimited.

IESG Note:

   The IESG takes no position on the validity of any Intellectual
   Property Rights statements contained in this document.

Notices

   Copyright (c) 1996 L. Peter Deutsch

   Permission is granted to copy and distribute this document for any
   purpose and without charge, including translations into other
   languages and incorporation into compilations, provided that the
   copyright notice and this notice are preserved, and that any
   substantive changes or deletions from the original are clearly
   marked.

   A pointer to the latest version of this and related documentation in
   HTML format can be found at the URL
   <ftp://ftp.uu.net/graphics/png/documents/zlib/zdoc-index.html>.

Abstract

   This specification defines a lossless compressed data format that
   compresses data using a combination of the LZ77 algorithm and Huffman
   coding, with efficiency comparable to the best currently available
   general-purpose compression methods.  The data can be produced or
   consumed, even for an arbitrarily long sequentially presented input
   data stream, using only an a priori bounded amount of intermediate
   storage.  The format can be implemented readily in a manner not
   covered by patents.








Deutsch                      Informational                      [Page 1]
```

<a id="source-page-2-part-1"></a>

## Source page 2

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


Table of Contents

   1. Introduction ................................................... 2
      1.1. Purpose ................................................... 2
      1.2. Intended audience ......................................... 3
      1.3. Scope ..................................................... 3
      1.4. Compliance ................................................ 3
      1.5.  Definitions of terms and conventions used ................ 3
      1.6. Changes from previous versions ............................ 4
   2. Compressed representation overview ............................. 4
   3. Detailed specification ......................................... 5
      3.1. Overall conventions ....................................... 5
          3.1.1. Packing into bytes .................................. 5
      3.2. Compressed block format ................................... 6
          3.2.1. Synopsis of prefix and Huffman coding ............... 6
          3.2.2. Use of Huffman coding in the "deflate" format ....... 7
          3.2.3. Details of block format ............................. 9
          3.2.4. Non-compressed blocks (BTYPE=00) ................... 11
          3.2.5. Compressed blocks (length and distance codes) ...... 11
          3.2.6. Compression with fixed Huffman codes (BTYPE=01) .... 12
          3.2.7. Compression with dynamic Huffman codes (BTYPE=10) .. 13
      3.3. Compliance ............................................... 14
   4. Compression algorithm details ................................. 14
   5. References .................................................... 16
   6. Security Considerations ....................................... 16
   7. Source code ................................................... 16
   8. Acknowledgements .............................................. 16
   9. Author's Address .............................................. 17

1. Introduction

   1.1. Purpose

      The purpose of this specification is to define a lossless
      compressed data format that:
          * Is independent of CPU type, operating system, file system,
            and character set, and hence can be used for interchange;
          * Can be produced or consumed, even for an arbitrarily long
            sequentially presented input data stream, using only an a
            priori bounded amount of intermediate storage, and hence
            can be used in data communications or similar structures
            such as Unix filters;
          * Compresses data with efficiency comparable to the best
            currently available general-purpose compression methods,
            and in particular considerably better than the "compress"
            program;
          * Can be implemented readily in a manner not covered by
            patents, and hence can be practiced freely;



Deutsch                      Informational                      [Page 2]
```

<a id="source-page-3-part-1"></a>

## Source page 3

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


          * Is compatible with the file format produced by the current
            widely used gzip utility, in that conforming decompressors
            will be able to read data produced by the existing gzip
            compressor.

      The data format defined by this specification does not attempt to:

          * Allow random access to compressed data;
          * Compress specialized data (e.g., raster graphics) as well
            as the best currently available specialized algorithms.

      A simple counting argument shows that no lossless compression
      algorithm can compress every possible input data set.  For the
      format defined here, the worst case expansion is 5 bytes per 32K-
      byte block, i.e., a size increase of 0.015% for large data sets.
      English text usually compresses by a factor of 2.5 to 3;
      executable files usually compress somewhat less; graphical data
      such as raster images may compress much more.

   1.2. Intended audience

      This specification is intended for use by implementors of software
      to compress data into "deflate" format and/or decompress data from
      "deflate" format.

      The text of the specification assumes a basic background in
      programming at the level of bits and other primitive data
      representations.  Familiarity with the technique of Huffman coding
      is helpful but not required.

   1.3. Scope

      The specification specifies a method for representing a sequence
      of bytes as a (usually shorter) sequence of bits, and a method for
      packing the latter bit sequence into bytes.

   1.4. Compliance

      Unless otherwise indicated below, a compliant decompressor must be
      able to accept and decompress any data set that conforms to all
      the specifications presented here; a compliant compressor must
      produce data sets that conform to all the specifications presented
      here.

   1.5.  Definitions of terms and conventions used

      Byte: 8 bits stored or transmitted as a unit (same as an octet).
      For this specification, a byte is exactly 8 bits, even on machines



Deutsch                      Informational                      [Page 3]
```

<a id="source-page-4-part-1"></a>

## Source page 4

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


      which store a character on a number of bits different from eight.
      See below, for the numbering of bits within a byte.

      String: a sequence of arbitrary bytes.

   1.6. Changes from previous versions

      There have been no technical changes to the deflate format since
      version 1.1 of this specification.  In version 1.2, some
      terminology was changed.  Version 1.3 is a conversion of the
      specification to RFC style.

2. Compressed representation overview

   A compressed data set consists of a series of blocks, corresponding
   to successive blocks of input data.  The block sizes are arbitrary,
   except that non-compressible blocks are limited to 65,535 bytes.

   Each block is compressed using a combination of the LZ77 algorithm
   and Huffman coding. The Huffman trees for each block are independent
   of those for previous or subsequent blocks; the LZ77 algorithm may
   use a reference to a duplicated string occurring in a previous block,
   up to 32K input bytes before.

   Each block consists of two parts: a pair of Huffman code trees that
   describe the representation of the compressed data part, and a
   compressed data part.  (The Huffman trees themselves are compressed
   using Huffman encoding.)  The compressed data consists of a series of
   elements of two types: literal bytes (of strings that have not been
   detected as duplicated within the previous 32K input bytes), and
   pointers to duplicated strings, where a pointer is represented as a
   pair <length, backward distance>.  The representation used in the
   "deflate" format limits distances to 32K bytes and lengths to 258
   bytes, but does not limit the size of a block, except for
   uncompressible blocks, which are limited as noted above.

   Each type of value (literals, distances, and lengths) in the
   compressed data is represented using a Huffman code, using one code
   tree for literals and lengths and a separate code tree for distances.
   The code trees for each block appear in a compact form just before
   the compressed data for that block.










Deutsch                      Informational                      [Page 4]
```

<a id="source-page-5-part-1"></a>

## Source page 5

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


3. Detailed specification

   3.1. Overall conventions In the diagrams below, a box like this:

         +---+
         |   | <-- the vertical bars might be missing
         +---+

      represents one byte; a box like this:

         +==============+
         |              |
         +==============+

      represents a variable number of bytes.

      Bytes stored within a computer do not have a "bit order", since
      they are always treated as a unit.  However, a byte considered as
      an integer between 0 and 255 does have a most- and least-
      significant bit, and since we write numbers with the most-
      significant digit on the left, we also write bytes with the most-
      significant bit on the left.  In the diagrams below, we number the
      bits of a byte so that bit 0 is the least-significant bit, i.e.,
      the bits are numbered:

         +--------+
         |76543210|
         +--------+

      Within a computer, a number may occupy multiple bytes.  All
      multi-byte numbers in the format described here are stored with
      the least-significant byte first (at the lower memory address).
      For example, the decimal number 520 is stored as:

             0        1
         +--------+--------+
         |00001000|00000010|
         +--------+--------+
          ^        ^
          |        |
          |        + more significant byte = 2 x 256
          + less significant byte = 8

      3.1.1. Packing into bytes

         This document does not address the issue of the order in which
         bits of a byte are transmitted on a bit-sequential medium,
         since the final data format described here is byte- rather than



Deutsch                      Informational                      [Page 5]
```

<a id="source-page-6-part-1"></a>

## Source page 6

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


         bit-oriented.  However, we describe the compressed block format
         in below, as a sequence of data elements of various bit
         lengths, not a sequence of bytes.  We must therefore specify
         how to pack these data elements into bytes to form the final
         compressed byte sequence:

             * Data elements are packed into bytes in order of
               increasing bit number within the byte, i.e., starting
               with the least-significant bit of the byte.
             * Data elements other than Huffman codes are packed
               starting with the least-significant bit of the data
               element.
             * Huffman codes are packed starting with the most-
               significant bit of the code.

         In other words, if one were to print out the compressed data as
         a sequence of bytes, starting with the first byte at the
         *right* margin and proceeding to the *left*, with the most-
         significant bit of each byte on the left as usual, one would be
         able to parse the result from right to left, with fixed-width
         elements in the correct MSB-to-LSB order and Huffman codes in
         bit-reversed order (i.e., with the first bit of the code in the
         relative LSB position).

   3.2. Compressed block format

      3.2.1. Synopsis of prefix and Huffman coding

         Prefix coding represents symbols from an a priori known
         alphabet by bit sequences (codes), one code for each symbol, in
         a manner such that different symbols may be represented by bit
         sequences of different lengths, but a parser can always parse
         an encoded string unambiguously symbol-by-symbol.

         We define a prefix code in terms of a binary tree in which the
         two edges descending from each non-leaf node are labeled 0 and
         1 and in which the leaf nodes correspond one-for-one with (are
         labeled with) the symbols of the alphabet; then the code for a
         symbol is the sequence of 0's and 1's on the edges leading from
         the root to the leaf labeled with that symbol.  For example:











Deutsch                      Informational                      [Page 6]
```

