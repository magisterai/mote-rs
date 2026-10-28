# DEFLATE Compressed Data Format Specification — part 2

Source: [DEFLATE Compressed Data Format Specification](https://www.rfc-editor.org/rfc/rfc1951.txt) · RFC 1951.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc1951.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md) · [Next](part-003.md)

<a id="source-page-7-part-1"></a>

## Source page 7

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


                          /\              Symbol    Code
                         0  1             ------    ----
                        /    \                A      00
                       /\     B               B       1
                      0  1                    C     011
                     /    \                   D     010
                    A     /\
                         0  1
                        /    \
                       D      C

         A parser can decode the next symbol from an encoded input
         stream by walking down the tree from the root, at each step
         choosing the edge corresponding to the next input bit.

         Given an alphabet with known symbol frequencies, the Huffman
         algorithm allows the construction of an optimal prefix code
         (one which represents strings with those symbol frequencies
         using the fewest bits of any possible prefix codes for that
         alphabet).  Such a code is called a Huffman code.  (See
         reference [1] in Chapter 5, references for additional
         information on Huffman codes.)

         Note that in the "deflate" format, the Huffman codes for the
         various alphabets must not exceed certain maximum code lengths.
         This constraint complicates the algorithm for computing code
         lengths from symbol frequencies.  Again, see Chapter 5,
         references for details.

      3.2.2. Use of Huffman coding in the "deflate" format

         The Huffman codes used for each alphabet in the "deflate"
         format have two additional rules:

             * All codes of a given bit length have lexicographically
               consecutive values, in the same order as the symbols
               they represent;

             * Shorter codes lexicographically precede longer codes.












Deutsch                      Informational                      [Page 7]
```

<a id="source-page-8-part-1"></a>

## Source page 8

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


         We could recode the example above to follow this rule as
         follows, assuming that the order of the alphabet is ABCD:

            Symbol  Code
            ------  ----
            A       10
            B       0
            C       110
            D       111

         I.e., 0 precedes 10 which precedes 11x, and 110 and 111 are
         lexicographically consecutive.

         Given this rule, we can define the Huffman code for an alphabet
         just by giving the bit lengths of the codes for each symbol of
         the alphabet in order; this is sufficient to determine the
         actual codes.  In our example, the code is completely defined
         by the sequence of bit lengths (2, 1, 3, 3).  The following
         algorithm generates the codes as integers, intended to be read
         from most- to least-significant bit.  The code lengths are
         initially in tree[I].Len; the codes are produced in
         tree[I].Code.

         1)  Count the number of codes for each code length.  Let
             bl_count[N] be the number of codes of length N, N >= 1.

         2)  Find the numerical value of the smallest code for each
             code length:

                code = 0;
                bl_count[0] = 0;
                for (bits = 1; bits <= MAX_BITS; bits++) {
                    code = (code + bl_count[bits-1]) << 1;
                    next_code[bits] = code;
                }

         3)  Assign numerical values to all codes, using consecutive
             values for all codes of the same length with the base
             values determined at step 2. Codes that are never used
             (which have a bit length of zero) must not be assigned a
             value.

                for (n = 0;  n <= max_code; n++) {
                    len = tree[n].Len;
                    if (len != 0) {
                        tree[n].Code = next_code[len];
                        next_code[len]++;
                    }



Deutsch                      Informational                      [Page 8]
```

<a id="source-page-9-part-1"></a>

## Source page 9

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


                }

         Example:

         Consider the alphabet ABCDEFGH, with bit lengths (3, 3, 3, 3,
         3, 2, 4, 4).  After step 1, we have:

            N      bl_count[N]
            -      -----------
            2      1
            3      5
            4      2

         Step 2 computes the following next_code values:

            N      next_code[N]
            -      ------------
            1      0
            2      0
            3      2
            4      14

         Step 3 produces the following code values:

            Symbol Length   Code
            ------ ------   ----
            A       3        010
            B       3        011
            C       3        100
            D       3        101
            E       3        110
            F       2         00
            G       4       1110
            H       4       1111

      3.2.3. Details of block format

         Each block of compressed data begins with 3 header bits
         containing the following data:

            first bit       BFINAL
            next 2 bits     BTYPE

         Note that the header bits do not necessarily begin on a byte
         boundary, since a block does not necessarily occupy an integral
         number of bytes.





Deutsch                      Informational                      [Page 9]
```

<a id="source-page-10-part-1"></a>

## Source page 10

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


         BFINAL is set if and only if this is the last block of the data
         set.

         BTYPE specifies how the data are compressed, as follows:

            00 - no compression
            01 - compressed with fixed Huffman codes
            10 - compressed with dynamic Huffman codes
            11 - reserved (error)

         The only difference between the two compressed cases is how the
         Huffman codes for the literal/length and distance alphabets are
         defined.

         In all cases, the decoding algorithm for the actual data is as
         follows:

            do
               read block header from input stream.
               if stored with no compression
                  skip any remaining bits in current partially
                     processed byte
                  read LEN and NLEN (see next section)
                  copy LEN bytes of data to output
               otherwise
                  if compressed with dynamic Huffman codes
                     read representation of code trees (see
                        subsection below)
                  loop (until end of block code recognized)
                     decode literal/length value from input stream
                     if value < 256
                        copy value (literal byte) to output stream
                     otherwise
                        if value = end of block (256)
                           break from loop
                        otherwise (value = 257..285)
                           decode distance from input stream

                           move backwards distance bytes in the output
                           stream, and copy length bytes from this
                           position to the output stream.
                  end loop
            while not last block

         Note that a duplicated string reference may refer to a string
         in a previous block; i.e., the backward distance may cross one
         or more block boundaries.  However a distance cannot refer past
         the beginning of the output stream.  (An application using a



Deutsch                      Informational                     [Page 10]
```

<a id="source-page-11-part-1"></a>

## Source page 11

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


         preset dictionary might discard part of the output stream; a
         distance can refer to that part of the output stream anyway)
         Note also that the referenced string may overlap the current
         position; for example, if the last 2 bytes decoded have values
         X and Y, a string reference with <length = 5, distance = 2>
         adds X,Y,X,Y,X to the output stream.

         We now specify each compression method in turn.

      3.2.4. Non-compressed blocks (BTYPE=00)

         Any bits of input up to the next byte boundary are ignored.
         The rest of the block consists of the following information:

              0   1   2   3   4...
            +---+---+---+---+================================+
            |  LEN  | NLEN  |... LEN bytes of literal data...|
            +---+---+---+---+================================+

         LEN is the number of data bytes in the block.  NLEN is the
         one's complement of LEN.

      3.2.5. Compressed blocks (length and distance codes)

         As noted above, encoded data blocks in the "deflate" format
         consist of sequences of symbols drawn from three conceptually
         distinct alphabets: either literal bytes, from the alphabet of
         byte values (0..255), or <length, backward distance> pairs,
         where the length is drawn from (3..258) and the distance is
         drawn from (1..32,768).  In fact, the literal and length
         alphabets are merged into a single alphabet (0..285), where
         values 0..255 represent literal bytes, the value 256 indicates
         end-of-block, and values 257..285 represent length codes
         (possibly in conjunction with extra bits following the symbol
         code) as follows:
















Deutsch                      Informational                     [Page 11]
```

<a id="source-page-12-part-1"></a>

## Source page 12

```text

RFC 1951      DEFLATE Compressed Data Format Specification      May 1996


                 Extra               Extra               Extra
            Code Bits Length(s) Code Bits Lengths   Code Bits Length(s)
            ---- ---- ------     ---- ---- -------   ---- ---- -------
             257   0     3       267   1   15,16     277   4   67-82
             258   0     4       268   1   17,18     278   4   83-98
             259   0     5       269   2   19-22     279   4   99-114
             260   0     6       270   2   23-26     280   4  115-130
             261   0     7       271   2   27-30     281   5  131-162
             262   0     8       272   2   31-34     282   5  163-194
             263   0     9       273   3   35-42     283   5  195-226
             264   0    10       274   3   43-50     284   5  227-257
             265   1  11,12      275   3   51-58     285   0    258
             266   1  13,14      276   3   59-66

         The extra bits should be interpreted as a machine integer
         stored with the most-significant bit first, e.g., bits 1110
         represent the value 14.

                  Extra           Extra               Extra
             Code Bits Dist  Code Bits   Dist     Code Bits Distance
             ---- ---- ----  ---- ----  ------    ---- ---- --------
               0   0    1     10   4     33-48    20    9   1025-1536
               1   0    2     11   4     49-64    21    9   1537-2048
               2   0    3     12   5     65-96    22   10   2049-3072
               3   0    4     13   5     97-128   23   10   3073-4096
               4   1   5,6    14   6    129-192   24   11   4097-6144
               5   1   7,8    15   6    193-256   25   11   6145-8192
               6   2   9-12   16   7    257-384   26   12  8193-12288
               7   2  13-16   17   7    385-512   27   12 12289-16384
               8   3  17-24   18   8    513-768   28   13 16385-24576
               9   3  25-32   19   8   769-1024   29   13 24577-32768

      3.2.6. Compression with fixed Huffman codes (BTYPE=01)

         The Huffman codes for the two alphabets are fixed, and are not
         represented explicitly in the data.  The Huffman code lengths
         for the literal/length alphabet are:

                   Lit Value    Bits        Codes
                   ---------    ----        -----
                     0 - 143     8          00110000 through
                                            10111111
                   144 - 255     9          110010000 through
                                            111111111
                   256 - 279     7          0000000 through
                                            0010111
                   280 - 287     8          11000000 through
                                            11000111



Deutsch                      Informational                     [Page 12]
```

