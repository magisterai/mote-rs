# DEFLATE Compressed Data Format Specification — part 3

Source: [DEFLATE Compressed Data Format Specification](https://www.rfc-editor.org/rfc/rfc1951.txt) · RFC 1951.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc1951.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-002.md)

<a id="source-page-13-part-1"></a>

## Source page 13

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


         The code lengths are sufficient to generate the actual codes,
         as described above; we show the codes in the table for added
         clarity.  Literal/length values 286-287 will never actually
         occur in the compressed data, but participate in the code
         construction.

         Distance codes 0-31 are represented by (fixed-length) 5-bit
         codes, with possible additional bits as shown in the table
         shown in Paragraph 3.2.5, above.  Note that distance codes 30-
         31 will never actually occur in the compressed data.

      3.2.7. Compression with dynamic Huffman codes (BTYPE=10)

         The Huffman codes for the two alphabets appear in the block
         immediately after the header bits and before the actual
         compressed data, first the literal/length code and then the
         distance code.  Each code is defined by a sequence of code
         lengths, as discussed in Paragraph 3.2.2, above.  For even
         greater compactness, the code length sequences themselves are
         compressed using a Huffman code.  The alphabet for code lengths
         is as follows:

               0 - 15: Represent code lengths of 0 - 15
                   16: Copy the previous code length 3 - 6 times.
                       The next 2 bits indicate repeat length
                             (0 = 3, ... , 3 = 6)
                          Example:  Codes 8, 16 (+2 bits 11),
                                    16 (+2 bits 10) will expand to
                                    12 code lengths of 8 (1 + 6 + 5)
                   17: Repeat a code length of 0 for 3 - 10 times.
                       (3 bits of length)
                   18: Repeat a code length of 0 for 11 - 138 times
                       (7 bits of length)

         A code length of 0 indicates that the corresponding symbol in
         the literal/length or distance alphabet will not occur in the
         block, and should not participate in the Huffman code
         construction algorithm given earlier.  If only one distance
         code is used, it is encoded using one bit, not zero bits; in
         this case there is a single code length of one, with one unused
         code.  One distance code of zero bits means that there are no
         distance codes used at all (the data is all literals).

         We can now define the format of the block:

               5 Bits: HLIT, # of Literal/Length codes - 257 (257 - 286)
               5 Bits: HDIST, # of Distance codes - 1        (1 - 32)
               4 Bits: HCLEN, # of Code Length codes - 4     (4 - 19)



Deutsch                      Informational                     [Page 13]
```

<a id="source-page-14-part-1"></a>

## Source page 14

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


               (HCLEN + 4) x 3 bits: code lengths for the code length
                  alphabet given just above, in the order: 16, 17, 18,
                  0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15

                  These code lengths are interpreted as 3-bit integers
                  (0-7); as above, a code length of 0 means the
                  corresponding symbol (literal/length or distance code
                  length) is not used.

               HLIT + 257 code lengths for the literal/length alphabet,
                  encoded using the code length Huffman code

               HDIST + 1 code lengths for the distance alphabet,
                  encoded using the code length Huffman code

               The actual compressed data of the block,
                  encoded using the literal/length and distance Huffman
                  codes

               The literal/length symbol 256 (end of data),
                  encoded using the literal/length Huffman code

         The code length repeat codes can cross from HLIT + 257 to the
         HDIST + 1 code lengths.  In other words, all code lengths form
         a single sequence of HLIT + HDIST + 258 values.

   3.3. Compliance

      A compressor may limit further the ranges of values specified in
      the previous section and still be compliant; for example, it may
      limit the range of backward pointers to some value smaller than
      32K.  Similarly, a compressor may limit the size of blocks so that
      a compressible block fits in memory.

      A compliant decompressor must accept the full range of possible
      values defined in the previous section, and must accept blocks of
      arbitrary size.

4. Compression algorithm details

   While it is the intent of this document to define the "deflate"
   compressed data format without reference to any particular
   compression algorithm, the format is related to the compressed
   formats produced by LZ77 (Lempel-Ziv 1977, see reference [2] below);
   since many variations of LZ77 are patented, it is strongly
   recommended that the implementor of a compressor follow the general
   algorithm presented here, which is known not to be patented per se.
   The material in this section is not part of the definition of the



Deutsch                      Informational                     [Page 14]
```

<a id="source-page-15-part-1"></a>

## Source page 15

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


   specification per se, and a compressor need not follow it in order to
   be compliant.

   The compressor terminates a block when it determines that starting a
   new block with fresh trees would be useful, or when the block size
   fills up the compressor's block buffer.

   The compressor uses a chained hash table to find duplicated strings,
   using a hash function that operates on 3-byte sequences.  At any
   given point during compression, let XYZ be the next 3 input bytes to
   be examined (not necessarily all different, of course).  First, the
   compressor examines the hash chain for XYZ.  If the chain is empty,
   the compressor simply writes out X as a literal byte and advances one
   byte in the input.  If the hash chain is not empty, indicating that
   the sequence XYZ (or, if we are unlucky, some other 3 bytes with the
   same hash function value) has occurred recently, the compressor
   compares all strings on the XYZ hash chain with the actual input data
   sequence starting at the current point, and selects the longest
   match.

   The compressor searches the hash chains starting with the most recent
   strings, to favor small distances and thus take advantage of the
   Huffman encoding.  The hash chains are singly linked. There are no
   deletions from the hash chains; the algorithm simply discards matches
   that are too old.  To avoid a worst-case situation, very long hash
   chains are arbitrarily truncated at a certain length, determined by a
   run-time parameter.

   To improve overall compression, the compressor optionally defers the
   selection of matches ("lazy matching"): after a match of length N has
   been found, the compressor searches for a longer match starting at
   the next input byte.  If it finds a longer match, it truncates the
   previous match to a length of one (thus producing a single literal
   byte) and then emits the longer match.  Otherwise, it emits the
   original match, and, as described above, advances N bytes before
   continuing.

   Run-time parameters also control this "lazy match" procedure.  If
   compression ratio is most important, the compressor attempts a
   complete second search regardless of the length of the first match.
   In the normal case, if the current match is "long enough", the
   compressor reduces the search for a longer match, thus speeding up
   the process.  If speed is most important, the compressor inserts new
   strings in the hash table only when no match was found, or when the
   match is not "too long".  This degrades the compression ratio but
   saves time since there are both fewer insertions and fewer searches.





Deutsch                      Informational                     [Page 15]
```

<a id="source-page-16-part-1"></a>

## Source page 16

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


5. References

   [1] Huffman, D. A., "A Method for the Construction of Minimum
       Redundancy Codes", Proceedings of the Institute of Radio
       Engineers, September 1952, Volume 40, Number 9, pp. 1098-1101.

   [2] Ziv J., Lempel A., "A Universal Algorithm for Sequential Data
       Compression", IEEE Transactions on Information Theory, Vol. 23,
       No. 3, pp. 337-343.

   [3] Gailly, J.-L., and Adler, M., ZLIB documentation and sources,
       available in ftp://ftp.uu.net/pub/archiving/zip/doc/

   [4] Gailly, J.-L., and Adler, M., GZIP documentation and sources,
       available as gzip-*.tar in ftp://prep.ai.mit.edu/pub/gnu/

   [5] Schwartz, E. S., and Kallick, B. "Generating a canonical prefix
       encoding." Comm. ACM, 7,3 (Mar. 1964), pp. 166-169.

   [6] Hirschberg and Lelewer, "Efficient decoding of prefix codes,"
       Comm. ACM, 33,4, April 1990, pp. 449-459.

6. Security Considerations

   Any data compression method involves the reduction of redundancy in
   the data.  Consequently, any corruption of the data is likely to have
   severe effects and be difficult to correct.  Uncompressed text, on
   the other hand, will probably still be readable despite the presence
   of some corrupted bytes.

   It is recommended that systems using this data format provide some
   means of validating the integrity of the compressed data.  See
   reference [3], for example.

7. Source code

   Source code for a C language implementation of a "deflate" compliant
   compressor and decompressor is available within the zlib package at
   ftp://ftp.uu.net/pub/archiving/zip/zlib/.

8. Acknowledgements

   Trademarks cited in this document are the property of their
   respective owners.

   Phil Katz designed the deflate format.  Jean-Loup Gailly and Mark
   Adler wrote the related software described in this specification.
   Glenn Randers-Pehrson converted this document to RFC and HTML format.



Deutsch                      Informational                     [Page 16]
```

<a id="source-page-17-part-1"></a>

## Source page 17

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


9. Author's Address

   L. Peter Deutsch
   Aladdin Enterprises
   203 Santa Margarita Ave.
   Menlo Park, CA 94025

   Phone: (415) 322-0103 (AM only)
   FAX:   (415) 322-1734
   EMail: <ghost@aladdin.com>

   Questions about the technical content of this specification can be
   sent by email to:

   Jean-Loup Gailly <gzip@prep.ai.mit.edu> and
   Mark Adler <madler@alumni.caltech.edu>

   Editorial comments on this specification can be sent by email to:

   L. Peter Deutsch <ghost@aladdin.com> and
   Glenn Randers-Pehrson <randeg@alumni.rpi.edu>






























Deutsch                      Informational                     [Page 17]
```

<a id="source-page-18-part-1"></a>

## Source page 18

```text

```

