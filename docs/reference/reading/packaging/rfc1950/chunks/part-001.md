# ZLIB Compressed Data Format Specification — part 1

Source: [ZLIB Compressed Data Format Specification](https://www.rfc-editor.org/rfc/rfc1950.txt) · RFC 1950.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc1950.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text






Network Working Group                                         P. Deutsch
Request for Comments: 1950                           Aladdin Enterprises
Category: Informational                                      J-L. Gailly
                                                                Info-ZIP
                                                                May 1996


         ZLIB Compressed Data Format Specification version 3.3

Status of This Memo

   This memo provides information for the Internet community.  This memo
   does not specify an Internet standard of any kind.  Distribution of
   this memo is unlimited.

IESG Note:

   The IESG takes no position on the validity of any Intellectual
   Property Rights statements contained in this document.

Notices

   Copyright (c) 1996 L. Peter Deutsch and Jean-Loup Gailly

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

   This specification defines a lossless compressed data format.  The
   data can be produced or consumed, even for an arbitrarily long
   sequentially presented input data stream, using only an a priori
   bounded amount of intermediate storage.  The format presently uses
   the DEFLATE compression method but can be easily extended to use
   other compression methods.  It can be implemented readily in a manner
   not covered by patents.  This specification also defines the ADLER-32
   checksum (an extension and improvement of the Fletcher checksum),
   used for detection of data corruption, and provides an algorithm for
   computing it.




Deutsch & Gailly             Informational                      [Page 1]
```

<a id="source-page-2-part-1"></a>

## Source page 2

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


Table of Contents

   1. Introduction ................................................... 2
      1.1. Purpose ................................................... 2
      1.2. Intended audience ......................................... 3
      1.3. Scope ..................................................... 3
      1.4. Compliance ................................................ 3
      1.5.  Definitions of terms and conventions used ................ 3
      1.6. Changes from previous versions ............................ 3
   2. Detailed specification ......................................... 3
      2.1. Overall conventions ....................................... 3
      2.2. Data format ............................................... 4
      2.3. Compliance ................................................ 7
   3. References ..................................................... 7
   4. Source code .................................................... 8
   5. Security Considerations ........................................ 8
   6. Acknowledgements ............................................... 8
   7. Authors' Addresses ............................................. 8
   8. Appendix: Rationale ............................................ 9
   9. Appendix: Sample code ..........................................10

1. Introduction

   1.1. Purpose

      The purpose of this specification is to define a lossless
      compressed data format that:

          * Is independent of CPU type, operating system, file system,
            and character set, and hence can be used for interchange;

          * Can be produced or consumed, even for an arbitrarily long
            sequentially presented input data stream, using only an a
            priori bounded amount of intermediate storage, and hence can
            be used in data communications or similar structures such as
            Unix filters;

          * Can use a number of different compression methods;

          * Can be implemented readily in a manner not covered by
            patents, and hence can be practiced freely.

      The data format defined by this specification does not attempt to
      allow random access to compressed data.







Deutsch & Gailly             Informational                      [Page 2]
```

<a id="source-page-3-part-1"></a>

## Source page 3

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


   1.2. Intended audience

      This specification is intended for use by implementors of software
      to compress data into zlib format and/or decompress data from zlib
      format.

      The text of the specification assumes a basic background in
      programming at the level of bits and other primitive data
      representations.

   1.3. Scope

      The specification specifies a compressed data format that can be
      used for in-memory compression of a sequence of arbitrary bytes.

   1.4. Compliance

      Unless otherwise indicated below, a compliant decompressor must be
      able to accept and decompress any data set that conforms to all
      the specifications presented here; a compliant compressor must
      produce data sets that conform to all the specifications presented
      here.

   1.5.  Definitions of terms and conventions used

      byte: 8 bits stored or transmitted as a unit (same as an octet).
      (For this specification, a byte is exactly 8 bits, even on
      machines which store a character on a number of bits different
      from 8.) See below, for the numbering of bits within a byte.

   1.6. Changes from previous versions

      Version 3.1 was the first public release of this specification.
      In version 3.2, some terminology was changed and the Adler-32
      sample code was rewritten for clarity.  In version 3.3, the
      support for a preset dictionary was introduced, and the
      specification was converted to RFC style.

2. Detailed specification

   2.1. Overall conventions

      In the diagrams below, a box like this:

         +---+
         |   | <-- the vertical bars might be missing
         +---+




Deutsch & Gailly             Informational                      [Page 3]
```

<a id="source-page-4-part-1"></a>

## Source page 4

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


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
      the MOST-significant byte first (at the lower memory address).
      For example, the decimal number 520 is stored as:

             0     1
         +--------+--------+
         |00000010|00001000|
         +--------+--------+
          ^        ^
          |        |
          |        + less significant byte = 8
          + more significant byte = 2 x 256

   2.2. Data format

      A zlib stream has the following structure:

           0   1
         +---+---+
         |CMF|FLG|   (more-->)
         +---+---+








Deutsch & Gailly             Informational                      [Page 4]
```

<a id="source-page-5-part-1"></a>

## Source page 5

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


      (if FLG.FDICT set)

           0   1   2   3
         +---+---+---+---+
         |     DICTID    |   (more-->)
         +---+---+---+---+

         +=====================+---+---+---+---+
         |...compressed data...|    ADLER32    |
         +=====================+---+---+---+---+

      Any data which may appear after ADLER32 are not part of the zlib
      stream.

      CMF (Compression Method and flags)
         This byte is divided into a 4-bit compression method and a 4-
         bit information field depending on the compression method.

            bits 0 to 3  CM     Compression method
            bits 4 to 7  CINFO  Compression info

      CM (Compression method)
         This identifies the compression method used in the file. CM = 8
         denotes the "deflate" compression method with a window size up
         to 32K.  This is the method used by gzip and PNG (see
         references [1] and [2] in Chapter 3, below, for the reference
         documents).  CM = 15 is reserved.  It might be used in a future
         version of this specification to indicate the presence of an
         extra field before the compressed data.

      CINFO (Compression info)
         For CM = 8, CINFO is the base-2 logarithm of the LZ77 window
         size, minus eight (CINFO=7 indicates a 32K window size). Values
         of CINFO above 7 are not allowed in this version of the
         specification.  CINFO is not defined in this specification for
         CM not equal to 8.

      FLG (FLaGs)
         This flag byte is divided as follows:

            bits 0 to 4  FCHECK  (check bits for CMF and FLG)
            bit  5       FDICT   (preset dictionary)
            bits 6 to 7  FLEVEL  (compression level)

         The FCHECK value must be such that CMF and FLG, when viewed as
         a 16-bit unsigned integer stored in MSB order (CMF*256 + FLG),
         is a multiple of 31.




Deutsch & Gailly             Informational                      [Page 5]
```

<a id="source-page-6-part-1"></a>

## Source page 6

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


      FDICT (Preset dictionary)
         If FDICT is set, a DICT dictionary identifier is present
         immediately after the FLG byte. The dictionary is a sequence of
         bytes which are initially fed to the compressor without
         producing any compressed output. DICT is the Adler-32 checksum
         of this sequence of bytes (see the definition of ADLER32
         below).  The decompressor can use this identifier to determine
         which dictionary has been used by the compressor.

      FLEVEL (Compression level)
         These flags are available for use by specific compression
         methods.  The "deflate" method (CM = 8) sets these flags as
         follows:

            0 - compressor used fastest algorithm
            1 - compressor used fast algorithm
            2 - compressor used default algorithm
            3 - compressor used maximum compression, slowest algorithm

         The information in FLEVEL is not needed for decompression; it
         is there to indicate if recompression might be worthwhile.

      compressed data
         For compression method 8, the compressed data is stored in the
         deflate compressed data format as described in the document
         "DEFLATE Compressed Data Format Specification" by L. Peter
         Deutsch. (See reference [3] in Chapter 3, below)

         Other compressed data formats are not specified in this version
         of the zlib specification.

      ADLER32 (Adler-32 checksum)
         This contains a checksum value of the uncompressed data
         (excluding any dictionary data) computed according to Adler-32
         algorithm. This algorithm is a 32-bit extension and improvement
         of the Fletcher algorithm, used in the ITU-T X.224 / ISO 8073
         standard. See references [4] and [5] in Chapter 3, below)

         Adler-32 is composed of two sums accumulated per byte: s1 is
         the sum of all bytes, s2 is the sum of all s1 values. Both sums
         are done modulo 65521. s1 is initialized to 1, s2 to zero.  The
         Adler-32 checksum is stored as s2*65536 + s1 in most-
         significant-byte first (network) order.








Deutsch & Gailly             Informational                      [Page 6]
```

