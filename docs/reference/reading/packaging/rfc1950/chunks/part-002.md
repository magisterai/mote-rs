# ZLIB Compressed Data Format Specification — part 2

Source: [ZLIB Compressed Data Format Specification](https://www.rfc-editor.org/rfc/rfc1950.txt) · RFC 1950.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc1950.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md)

<a id="source-page-7-part-1"></a>

## Source page 7

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


   2.3. Compliance

      A compliant compressor must produce streams with correct CMF, FLG
      and ADLER32, but need not support preset dictionaries.  When the
      zlib data format is used as part of another standard data format,
      the compressor may use only preset dictionaries that are specified
      by this other data format.  If this other format does not use the
      preset dictionary feature, the compressor must not set the FDICT
      flag.

      A compliant decompressor must check CMF, FLG, and ADLER32, and
      provide an error indication if any of these have incorrect values.
      A compliant decompressor must give an error indication if CM is
      not one of the values defined in this specification (only the
      value 8 is permitted in this version), since another value could
      indicate the presence of new features that would cause subsequent
      data to be interpreted incorrectly.  A compliant decompressor must
      give an error indication if FDICT is set and DICTID is not the
      identifier of a known preset dictionary.  A decompressor may
      ignore FLEVEL and still be compliant.  When the zlib data format
      is being used as a part of another standard format, a compliant
      decompressor must support all the preset dictionaries specified by
      the other format. When the other format does not use the preset
      dictionary feature, a compliant decompressor must reject any
      stream in which the FDICT flag is set.

3. References

   [1] Deutsch, L.P.,"GZIP Compressed Data Format Specification",
       available in ftp://ftp.uu.net/pub/archiving/zip/doc/

   [2] Thomas Boutell, "PNG (Portable Network Graphics) specification",
       available in ftp://ftp.uu.net/graphics/png/documents/

   [3] Deutsch, L.P.,"DEFLATE Compressed Data Format Specification",
       available in ftp://ftp.uu.net/pub/archiving/zip/doc/

   [4] Fletcher, J. G., "An Arithmetic Checksum for Serial
       Transmissions," IEEE Transactions on Communications, Vol. COM-30,
       No. 1, January 1982, pp. 247-252.

   [5] ITU-T Recommendation X.224, Annex D, "Checksum Algorithms,"
       November, 1993, pp. 144, 145. (Available from
       gopher://info.itu.ch). ITU-T X.244 is also the same as ISO 8073.







Deutsch & Gailly             Informational                      [Page 7]
```

<a id="source-page-8-part-1"></a>

## Source page 8

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


4. Source code

   Source code for a C language implementation of a "zlib" compliant
   library is available at ftp://ftp.uu.net/pub/archiving/zip/zlib/.

5. Security Considerations

   A decoder that fails to check the ADLER32 checksum value may be
   subject to undetected data corruption.

6. Acknowledgements

   Trademarks cited in this document are the property of their
   respective owners.

   Jean-Loup Gailly and Mark Adler designed the zlib format and wrote
   the related software described in this specification.  Glenn
   Randers-Pehrson converted this document to RFC and HTML format.

7. Authors' Addresses

   L. Peter Deutsch
   Aladdin Enterprises
   203 Santa Margarita Ave.
   Menlo Park, CA 94025

   Phone: (415) 322-0103 (AM only)
   FAX:   (415) 322-1734
   EMail: <ghost@aladdin.com>


   Jean-Loup Gailly

   EMail: <gzip@prep.ai.mit.edu>

   Questions about the technical content of this specification can be
   sent by email to

   Jean-Loup Gailly <gzip@prep.ai.mit.edu> and
   Mark Adler <madler@alumni.caltech.edu>

   Editorial comments on this specification can be sent by email to

   L. Peter Deutsch <ghost@aladdin.com> and
   Glenn Randers-Pehrson <randeg@alumni.rpi.edu>






Deutsch & Gailly             Informational                      [Page 8]
```

<a id="source-page-9-part-1"></a>

## Source page 9

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


8. Appendix: Rationale

   8.1. Preset dictionaries

      A preset dictionary is specially useful to compress short input
      sequences. The compressor can take advantage of the dictionary
      context to encode the input in a more compact manner. The
      decompressor can be initialized with the appropriate context by
      virtually decompressing a compressed version of the dictionary
      without producing any output. However for certain compression
      algorithms such as the deflate algorithm this operation can be
      achieved without actually performing any decompression.

      The compressor and the decompressor must use exactly the same
      dictionary. The dictionary may be fixed or may be chosen among a
      certain number of predefined dictionaries, according to the kind
      of input data. The decompressor can determine which dictionary has
      been chosen by the compressor by checking the dictionary
      identifier. This document does not specify the contents of
      predefined dictionaries, since the optimal dictionaries are
      application specific. Standard data formats using this feature of
      the zlib specification must precisely define the allowed
      dictionaries.

   8.2. The Adler-32 algorithm

      The Adler-32 algorithm is much faster than the CRC32 algorithm yet
      still provides an extremely low probability of undetected errors.

      The modulo on unsigned long accumulators can be delayed for 5552
      bytes, so the modulo operation time is negligible.  If the bytes
      are a, b, c, the second sum is 3a + 2b + c + 3, and so is position
      and order sensitive, unlike the first sum, which is just a
      checksum.  That 65521 is prime is important to avoid a possible
      large class of two-byte errors that leave the check unchanged.
      (The Fletcher checksum uses 255, which is not prime and which also
      makes the Fletcher check insensitive to single byte changes 0 <->
      255.)

      The sum s1 is initialized to 1 instead of zero to make the length
      of the sequence part of s2, so that the length does not have to be
      checked separately. (Any sequence of zeroes has a Fletcher
      checksum of zero.)








Deutsch & Gailly             Informational                      [Page 9]
```

<a id="source-page-10-part-1"></a>

## Source page 10

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


9. Appendix: Sample code

   The following C code computes the Adler-32 checksum of a data buffer.
   It is written for clarity, not for speed.  The sample code is in the
   ANSI C programming language. Non C users may find it easier to read
   with these hints:

      &      Bitwise AND operator.
      >>     Bitwise right shift operator. When applied to an
             unsigned quantity, as here, right shift inserts zero bit(s)
             at the left.
      <<     Bitwise left shift operator. Left shift inserts zero
             bit(s) at the right.
      ++     "n++" increments the variable n.
      %      modulo operator: a % b is the remainder of a divided by b.

      #define BASE 65521 /* largest prime smaller than 65536 */

      /*
         Update a running Adler-32 checksum with the bytes buf[0..len-1]
       and return the updated checksum. The Adler-32 checksum should be
       initialized to 1.

       Usage example:

         unsigned long adler = 1L;

         while (read_buffer(buffer, length) != EOF) {
           adler = update_adler32(adler, buffer, length);
         }
         if (adler != original_adler) error();
      */
      unsigned long update_adler32(unsigned long adler,
         unsigned char *buf, int len)
      {
        unsigned long s1 = adler & 0xffff;
        unsigned long s2 = (adler >> 16) & 0xffff;
        int n;

        for (n = 0; n < len; n++) {
          s1 = (s1 + buf[n]) % BASE;
          s2 = (s2 + s1)     % BASE;
        }
        return (s2 << 16) + s1;
      }

      /* Return the adler32 of the bytes buf[0..len-1] */




Deutsch & Gailly             Informational                     [Page 10]
```

<a id="source-page-11-part-1"></a>

## Source page 11

```text

RFC 1950       ZLIB Compressed Data Format Specification        May 1996


      unsigned long adler32(unsigned char *buf, int len)
      {
        return update_adler32(1L, buf, len);
      }















































Deutsch & Gailly             Informational                     [Page 11]
```

<a id="source-page-12-part-1"></a>

## Source page 12

```text

```

