# ZIP File Format Specification — part 7

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-006.md) · [Next](part-008.md)

<a id="source-page-1-part-7"></a>

## Source page 1 (continued, part 7)

```text
    into the ZIP file and it MAY include a defined directory path.

    4.7.3 Manifest files MAY be compressed or encrypted as needed for
    application processing of the files inside the ZIP files.

    Manifest files are outside of the scope of this specification.


5.0 Explanation of compression methods
--------------------------------------


5.1 UnShrinking - Method 1
--------------------------

    5.1.1 Shrinking is a Dynamic Ziv-Lempel-Welch compression algorithm
    with partial clearing.  The initial code size is 9 bits, and the 
    maximum code size is 13 bits.  Shrinking differs from conventional 
    Dynamic Ziv-Lempel-Welch implementations in several respects:

    5.1.2 The code size is controlled by the compressor, and is 
    not automatically increased when codes larger than the current
    code size are created (but not necessarily used).  When
    the decompressor encounters the code sequence 256
    (decimal) followed by 1, it SHOULD increase the code size
    read from the input stream to the next bit size.  No
    blocking of the codes is performed, so the next code at
    the increased size SHOULD be read from the input stream
    immediately after where the previous code at the smaller
    bit size was read.  Again, the decompressor SHOULD NOT
    increase the code size used until the sequence 256,1 is
    encountered.

    5.1.3 When the table becomes full, total clearing is not
    performed.  Rather, when the compressor emits the code
    sequence 256,2 (decimal), the decompressor SHOULD clear
    all leaf nodes from the Ziv-Lempel tree, and continue to
    use the current code size.  The nodes that are cleared
    from the Ziv-Lempel tree are then re-used, with the lowest
    code value re-used first, and the highest code value
    re-used last.  The compressor can emit the sequence 256,2
    at any time.

5.2 Expanding - Methods 2-5
---------------------------

    5.2.1 The Reducing algorithm is actually a combination of two
    distinct algorithms.  The first algorithm compresses repeated
    byte sequences, and the second algorithm takes the compressed
    stream from the first algorithm and applies a probabilistic
    compression method.

    5.2.2 The probabilistic compression stores an array of 'follower
    sets' S(j), for j=0 to 255, corresponding to each possible
    ASCII character.  Each set contains between 0 and 32
    characters, to be denoted as S(j)[0],...,S(j)[m], where m<32.
    The sets are stored at the beginning of the data area for a
    Reduced file, in reverse order, with S(255) first, and S(0)
    last.

    5.2.3 The sets are encoded as { N(j), S(j)[0],...,S(j)[N(j)-1] },
    where N(j) is the size of set S(j).  N(j) can be 0, in which
    case the follower set for S(j) is empty.  Each N(j) value is
    encoded in 6 bits, followed by N(j) eight bit character values
    corresponding to S(j)[0] to S(j)[N(j)-1] respectively.  If
    N(j) is 0, then no values for S(j) are stored, and the value
    for N(j-1) immediately follows.

    5.2.4 Immediately after the follower sets, is the compressed data
    stream.  The compressed data stream can be interpreted for the
    probabilistic decompression as follows:

    let Last-Character <- 0.
    loop until done
        if the follower set S(Last-Character) is empty then
            read 8 bits from the input stream, and copy this
            value to the output stream.
        otherwise if the follower set S(Last-Character) is non-empty then
            read 1 bit from the input stream.
            if this bit is not zero then
                read 8 bits from the input stream, and copy this
                value to the output stream.
            otherwise if this bit is zero then
                read B(N(Last-Character)) bits from the input
                stream, and assign this value to I.
                Copy the value of S(Last-Character)[I] to the
                output stream.

        assign the last value placed on the output stream to
        Last-Character.
    end loop

    B(N(j)) is defined as the minimal number of bits required to
    encode the value N(j)-1.

    5.2.5 The decompressed stream from above can then be expanded to
    re-create the original file as follows:

    let State <- 0.

    loop until done
        read 8 bits from the input stream into C.
        case State of
            0:  if C is not equal to DLE (144 decimal) then
                   copy C to the output stream.
                 otherwise if C is equal to DLE then
                   let State <- 1.

            1:  if C is non-zero then
                   let V <- C.
                   let Len <- L(V)
                   let State <- F(Len).
                 otherwise if C is zero then
                   copy the value 144 (decimal) to the output stream.
                   let State <- 0

            2:  let Len <- Len + C
                    let State <- 3.

            3:  move backwards D(V,C) bytes in the output stream
                    (if this position is before the start of the output
                    stream, then assume that all the data before the
                    start of the output stream is filled with zeros).
                    copy Len+3 bytes from this position to the output stream.
                    let State <- 0.
          end case
    end loop

    The functions F,L, and D are dependent on the 'compression
    factor', 1 through 4, and are defined as follows:

    For compression factor 1:
        L(X) equals the lower 7 bits of X.
        F(X) equals 2 if X equals 127 otherwise F(X) equals 3.
        D(X,Y) equals the (upper 1 bit of X) * 256 + Y + 1.
    For compression factor 2:
        L(X) equals the lower 6 bits of X.
        F(X) equals 2 if X equals 63 otherwise F(X) equals 3.
        D(X,Y) equals the (upper 2 bits of X) * 256 + Y + 1.
    For compression factor 3:
        L(X) equals the lower 5 bits of X.
        F(X) equals 2 if X equals 31 otherwise F(X) equals 3.
        D(X,Y) equals the (upper 3 bits of X) * 256 + Y + 1.
    For compression factor 4:
        L(X) equals the lower 4 bits of X.
        F(X) equals 2 if X equals 15 otherwise F(X) equals 3.
        D(X,Y) equals the (upper 4 bits of X) * 256 + Y + 1.

5.3 Imploding - Method 6
------------------------

    5.3.1 The Imploding algorithm is actually a combination of two 
    distinct algorithms.  The first algorithm compresses repeated byte
    sequences using a sliding dictionary.  The second algorithm is
    used to compress the encoding of the sliding dictionary output,
    using multiple Shannon-Fano trees.

    5.3.2 The Imploding algorithm can use a 4K or 8K sliding dictionary
    size. The dictionary size used can be determined by bit 1 in the
    general purpose flag word; a 0 bit indicates a 4K dictionary
    while a 1 bit indicates an 8K dictionary.

    5.3.3 The Shannon-Fano trees are stored at the start of the 
    compressed file. The number of trees stored is defined by bit 2 in 
    the general purpose flag word; a 0 bit indicates two trees stored, 
    a 1 bit indicates three trees are stored.  If 3 trees are stored,
    the first Shannon-Fano tree represents the encoding of the
    Literal characters, the second tree represents the encoding of
    the Length information, the third represents the encoding of the
    Distance information.  When 2 Shannon-Fano trees are stored, the
    Length tree is stored first, followed by the Distance tree.

    5.3.4 The Literal Shannon-Fano tree, if present is used to represent
    the entire ASCII character set, and contains 256 values.  This
    tree is used to compress any data not compressed by the sliding
    dictionary algorithm.  When this tree is present, the Minimum
    Match Length for the sliding dictionary is 3.  If this tree is
    not present, the Minimum Match Length is 2.

    5.3.5 The Length Shannon-Fano tree is used to compress the Length 
    part of the (length,distance) pairs from the sliding dictionary
    output.  The Length tree contains 64 values, ranging from the
    Minimum Match Length, to 63 plus the Minimum Match Length.

    5.3.6 The Distance Shannon-Fano tree is used to compress the Distance
    part of the (length,distance) pairs from the sliding dictionary
    output. The Distance tree contains 64 values, ranging from 0 to
    63, representing the upper 6 bits of the distance value.  The
    distance values themselves will be between 0 and the sliding
    dictionary size, either 4K or 8K.

    5.3.7 The Shannon-Fano trees themselves are stored in a compressed
    format. The first byte of the tree data represents the number of
    bytes of data representing the (compressed) Shannon-Fano tree
    minus 1.  The remaining bytes represent the Shannon-Fano tree
    data encoded as:

        High 4 bits: Number of values at this bit length + 1. (1 - 16)
        Low  4 bits: Bit Length needed to represent value + 1. (1 - 16)

    5.3.8 The Shannon-Fano codes can be constructed from the bit lengths
    using the following algorithm:

    1)  Sort the Bit Lengths in ascending order, while retaining the
        order of the original lengths stored in the file.

    2)  Generate the Shannon-Fano trees:

        Code <- 0
        CodeIncrement <- 0
        LastBitLength <- 0
        i <- number of Shannon-Fano codes - 1   (either 255 or 63)

        loop while i >= 0
            Code = Code + CodeIncrement
            if BitLength(i) <> LastBitLength then
                LastBitLength=BitLength(i)
                CodeIncrement = 1 shifted left (16 - LastBitLength)
            ShannonCode(i) = Code
            i <- i - 1
        end loop

    3)  Reverse the order of all the bits in the above ShannonCode()
        vector, so that the most significant bit becomes the least
        significant bit.  For example, the value 0x1234 (hex) would
        become 0x2C48 (hex).

    4)  Restore the order of Shannon-Fano codes as originally stored
        within the file.

    Example:

        This example will show the encoding of a Shannon-Fano tree
        of size 8.  Notice that the actual Shannon-Fano trees used
        for Imploding are either 64 or 256 entries in size.

    Example:   0x02, 0x42, 0x01, 0x13

        The first byte indicates 3 values in this table.  Decoding the
        bytes:
                0x42 = 5 codes of 3 bits long
                0x01 = 1 code  of 2 bits long
                0x13 = 2 codes of 4 bits long

        This would generate the original bit length array of:
        (3, 3, 3, 3, 3, 2, 4, 4)

        There are 8 codes in this table for the values 0 thru 7.  Using 
        the algorithm to obtain the Shannon-Fano codes produces:

                                      Reversed     Order     Original
    Val  Sorted   Constructed Code      Value     Restored    Length
    ---  ------   -----------------   --------    --------    ------
    0:     2      1100000000000000        11       101          3
    1:     3      1010000000000000       101       001          3
    2:     3      1000000000000000       001       110          3
    3:     3      0110000000000000       110       010          3
    4:     3      0100000000000000       010       100          3
    5:     3      0010000000000000       100        11          2
    6:     4      0001000000000000      1000      1000          4
    7:     4      0000000000000000      0000      0000          4

    The values in the Val, Order Restored and Original Length columns
    now represent the Shannon-Fano encoding tree that can be used for
    decoding the Shannon-Fano encoded data.  How to parse the
    variable length Shannon-Fano values from the data stream is beyond
    the scope of this document.  (See the references listed at the end of
    this document for more information.)  However, traditional decoding
    schemes used for Huffman variable length decoding, such as the
    Greenlaw algorithm, can be successfully applied.

    5.3.9 The compressed data stream begins immediately after the
    compressed Shannon-Fano data.  The compressed data stream can be
    interpreted as follows:

    loop until done
        read 1 bit from input stream.

        if this bit is non-zero then       (encoded data is literal data)
            if Literal Shannon-Fano tree is present
                read and decode character using Literal Shannon-Fano tree.
            otherwise
                read 8 bits from input stream.
            copy character to the output stream.
        otherwise              (encoded data is sliding dictionary match)
            if 8K dictionary size
                read 7 bits for offset Distance (lower 7 bits of offset).
            otherwise
                read 6 bits for offset Distance (lower 6 bits of offset).

            using the Distance Shannon-Fano tree, read and decode the
              upper 6 bits of the Distance value.

            using the Length Shannon-Fano tree, read and decode
              the Length value.

            Length <- Length + Minimum Match Length

            if Length = 63 + Minimum Match Length
                read 8 bits from the input stream,
                add this value to Length.

            move backwards Distance+1 bytes in the output stream, and
            copy Length characters from this position to the output
            stream.  (if this position is before the start of the output
            stream, then assume that all the data before the start of
            the output stream is filled with zeros).
    end loop

5.4 Tokenizing - Method 7
-------------------------

```

