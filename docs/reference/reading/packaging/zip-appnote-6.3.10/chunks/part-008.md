# ZIP File Format Specification — part 8

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-007.md) · [Next](part-009.md)

<a id="source-page-1-part-8"></a>

## Source page 1 (continued, part 8)

```text
    5.4.1 This method is not used by PKZIP.

5.5 Deflating - Method 8
------------------------

    5.5.1 The Deflate algorithm is similar to the Implode algorithm using
    a sliding dictionary of up to 32K with secondary compression
    from Huffman/Shannon-Fano codes.

    5.5.2 The compressed data is stored in blocks with a header describing
    the block and the Huffman codes used in the data block.  The header
    format is as follows:

       Bit 0: Last Block bit     This bit is set to 1 if this is the last
                                 compressed block in the data.
       Bits 1-2: Block type
          00 (0) - Block is stored - All stored data is byte aligned.
                   Skip bits until next byte, then next word = block 
                   length, followed by the ones compliment of the block
                   length word. Remaining data in block is the stored 
                   data.

          01 (1) - Use fixed Huffman codes for literal and distance codes.
                   Lit Code    Bits             Dist Code   Bits
                   ---------   ----             ---------   ----
                     0 - 143    8                 0 - 31      5
                   144 - 255    9
                   256 - 279    7
                   280 - 287    8

                   Literal codes 286-287 and distance codes 30-31 are 
                   never used but participate in the huffman construction.

          10 (2) - Dynamic Huffman codes.  (See expanding Huffman codes)

          11 (3) - Reserved - Flag a "Error in compressed data" if seen.

    5.5.3 Expanding Huffman Codes
    
    If the data block is stored with dynamic Huffman codes, the Huffman
    codes are sent in the following compressed format:

       5 Bits: # of Literal codes sent - 256 (256 - 286)
               All other codes are never sent.
       5 Bits: # of Dist codes - 1           (1 - 32)
       4 Bits: # of Bit Length codes - 3     (3 - 19)

    The Huffman codes are sent as bit lengths and the codes are built as
    described in the implode algorithm.  The bit lengths themselves are
    compressed with Huffman codes.  There are 19 bit length codes:

       0 - 15: Represent bit lengths of 0 - 15
           16: Copy the previous bit length 3 - 6 times.
               The next 2 bits indicate repeat length (0 = 3, ... ,3 = 6)
                  Example:  Codes 8, 16 (+2 bits 11), 16 (+2 bits 10) will
                            expand to 12 bit lengths of 8 (1 + 6 + 5)
           17: Repeat a bit length of 0 for 3 - 10 times. (3 bits of length)
           18: Repeat a bit length of 0 for 11 - 138 times (7 bits of length)

    The lengths of the bit length codes are sent packed 3 bits per value
    (0 - 7) in the following order:

       16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15

    The Huffman codes SHOULD be built as described in the Implode algorithm
    except codes are assigned starting at the shortest bit length, i.e. the
    shortest code SHOULD be all 0's rather than all 1's.  Also, codes with
    a bit length of zero do not participate in the tree construction.  The
    codes are then used to decode the bit lengths for the literal and 
    distance tables.

    The bit lengths for the literal tables are sent first with the number
    of entries sent described by the 5 bits sent earlier.  There are up
    to 286 literal characters; the first 256 represent the respective 8
    bit character, code 256 represents the End-Of-Block code, the remaining
    29 codes represent copy lengths of 3 thru 258.  There are up to 30
    distance codes representing distances from 1 thru 32k as described
    below.

                                 Length Codes
                                 ------------
          Extra             Extra              Extra              Extra
     Code Bits Length  Code Bits Lengths  Code Bits Lengths  Code Bits Length(s)
     ---- ---- ------  ---- ---- -------  ---- ---- -------  ---- ---- ---------
      257   0     3     265   1   11,12    273   3   35-42    281   5  131-162
      258   0     4     266   1   13,14    274   3   43-50    282   5  163-194
      259   0     5     267   1   15,16    275   3   51-58    283   5  195-226
      260   0     6     268   1   17,18    276   3   59-66    284   5  227-257
      261   0     7     269   2   19-22    277   4   67-82    285   0    258
      262   0     8     270   2   23-26    278   4   83-98
      263   0     9     271   2   27-30    279   4   99-114
      264   0    10     272   2   31-34    280   4  115-130

                                Distance Codes
                                --------------
          Extra           Extra             Extra               Extra
     Code Bits Dist  Code Bits  Dist   Code Bits Distance  Code Bits Distance
     ---- ---- ----  ---- ---- ------  ---- ---- --------  ---- ---- --------
       0   0    1      8   3   17-24    16    7  257-384    24   11  4097-6144
       1   0    2      9   3   25-32    17    7  385-512    25   11  6145-8192
       2   0    3     10   4   33-48    18    8  513-768    26   12  8193-12288
       3   0    4     11   4   49-64    19    8  769-1024   27   12 12289-16384
       4   1   5,6    12   5   65-96    20    9 1025-1536   28   13 16385-24576
       5   1   7,8    13   5   97-128   21    9 1537-2048   29   13 24577-32768
       6   2   9-12   14   6  129-192   22   10 2049-3072
       7   2  13-16   15   6  193-256   23   10 3073-4096

    5.5.4 The compressed data stream begins immediately after the
    compressed header data.  The compressed data stream can be
    interpreted as follows:

    do
       read header from input stream.

       if stored block
          skip bits until byte aligned
          read count and 1's compliment of count
          copy count bytes data block
       otherwise
          loop until end of block code sent
             decode literal character from input stream
             if literal < 256
                copy character to the output stream
             otherwise
                if literal = end of block
                   break from loop
                otherwise
                   decode distance from input stream

                   move backwards distance bytes in the output stream, and
                   copy length characters from this position to the output
                   stream.
          end loop
    while not last block

    if data descriptor exists
       skip bits until byte aligned
       read crc and sizes
    endif

5.6 Enhanced Deflating - Method 9
---------------------------------

    5.6.1 The Enhanced Deflating algorithm is similar to Deflate but uses 
    a sliding dictionary of up to 64K. Deflate64(tm) is supported
    by the Deflate extractor. 

5.7 BZIP2 - Method 12
---------------------

    5.7.1 BZIP2 is an open-source data compression algorithm developed by 
    Julian Seward.  Information and source code for this algorithm
    can be found on the internet.

5.8 LZMA - Method 14 
---------------------

    5.8.1 LZMA is a block-oriented, general purpose data compression 
    algorithm developed and maintained by Igor Pavlov.  It is a derivative 
    of LZ77 that utilizes Markov chains and a range coder.  Information and 
    source code for this algorithm can be found on the internet.  Consult 
    with the author of this algorithm for information on terms or 
    restrictions on use.

    Support for LZMA within the ZIP format is defined as follows:   

    5.8.2 The Compression method field within the ZIP Local and Central 
    Header records will be set to the value 14 to indicate data was
    compressed using LZMA. 

    5.8.3 The Version needed to extract field within the ZIP Local and 
    Central Header records will be set to 6.3 to indicate the minimum 
    ZIP format version supporting this feature.

    5.8.4 File data compressed using the LZMA algorithm MUST be placed 
    immediately following the Local Header for the file.  If a standard 
    ZIP encryption header is required, it will follow the Local Header 
    and will precede the LZMA compressed file data segment.  The location 
    of LZMA compressed data segment within the ZIP format will be as shown:

        [local header file 1]
        [encryption header file 1]
        [LZMA compressed data segment for file 1]
        [data descriptor 1]
        [local header file 2]

    5.8.5 The encryption header and data descriptor records MAY
    be conditionally present.  The LZMA Compressed Data Segment 
    will consist of an LZMA Properties Header followed by the 
    LZMA Compressed Data as shown:

        [LZMA properties header for file 1]
        [LZMA compressed data for file 1]

    5.8.6 The LZMA Compressed Data will be stored as provided by the 
    LZMA compression library.  Compressed size, uncompressed size and 
    other file characteristics about the file being compressed MUST be 
    stored in standard ZIP storage format.

    5.8.7 The LZMA Properties Header will store specific data required 
    to decompress the LZMA compressed Data.  This data is set by the 
    LZMA compression engine using the function WriteCoderProperties() 
    as documented within the LZMA SDK. 
         
    5.8.8 Storage fields for the property information within the LZMA 
    Properties Header are as follows:

         LZMA Version Information 2 bytes
         LZMA Properties Size 2 bytes
         LZMA Properties Data variable, defined by "LZMA Properties Size"

       5.8.8.1 LZMA Version Information - this field identifies which version 
       of the LZMA SDK was used to compress a file.  The first byte will 
       store the major version number of the LZMA SDK and the second 
       byte will store the minor number.  

       5.8.8.2 LZMA Properties Size - this field defines the size of the 
       remaining property data.  Typically this size SHOULD be determined by 
       the version of the SDK.  This size field is included as a convenience
       and to help avoid any ambiguity arising in the future due
       to changes in this compression algorithm. 

       5.8.8.3 LZMA Property Data - this variable sized field records the 
       required values for the decompressor as defined by the LZMA SDK.  
       The data stored in this field SHOULD be obtained using the 
       WriteCoderProperties() in the version of the SDK defined by 
       the "LZMA Version Information" field.  

       5.8.8.4 The layout of the "LZMA Properties Data" field is a function of 
       the LZMA compression algorithm.  It is possible that this layout MAY be
       changed by the author over time.  The data layout in version 4.3 of the 
       LZMA SDK defines a 5 byte array that uses 4 bytes to store the dictionary 
       size in little-endian order. This is preceded by a single packed byte as 
       the first element of the array that contains the following fields:

         PosStateBits
         LiteralPosStateBits
         LiteralContextBits

       Refer to the LZMA documentation for a more detailed explanation of 
       these fields.  

    5.8.9 Data compressed with method 14, LZMA, MAY include an end-of-stream
    (EOS) marker ending the compressed data stream.  This marker is not
    required, but its use is highly recommended to facilitate processing
    and implementers SHOULD include the EOS marker whenever possible.
    When the EOS marker is used, general purpose bit 1 MUSY be set.  If
    general purpose bit 1 is not set, the EOS marker is not present.

5.9 WavPack - Method 97
-----------------------

    5.9.1 Information describing the use of compression method 97 is 
    provided by WinZIP International, LLC.  This method relies on the
    open source WavPack audio compression utility developed by David Bryant.  
    Information on WavPack is available at www.wavpack.com.  Please consult 
    with the author of this algorithm for information on terms and 
    restrictions on use.

    5.9.2 WavPack data for a file begins immediately after the end of the
    local header data.  This data is the output from WavPack compression
    routines.  Within the ZIP file, the use of WavPack compression is
    indicated by setting the compression method field to a value of 97 
    in both the local header and the central directory header.  The Version 
    needed to extract and version made by fields use the same values as are 
    used for data compressed using the Deflate algorithm.

    5.9.3 An implementation note for storing digital sample data when using 
    WavPack compression within ZIP files is that all of the bytes of
    the sample data SHOULD be compressed.  This includes any unused
    bits up to the byte boundary.  An example is a 2 byte sample that
    uses only 12 bits for the sample data with 4 unused bits.  If only
    12 bits are passed as the sample size to the WavPack routines, the 4 
    unused bits will be set to 0 on extraction regardless of their original 
    state.  To avoid this, the full 16 bits of the sample data size
    SHOULD be provided. 

5.10 PPMd - Method 98
---------------------

    5.10.1 PPMd is a data compression algorithm developed by Dmitry Shkarin
    which includes a carryless rangecoder developed by Dmitry Subbotin.
    This algorithm is based on predictive phrase matching on multiple
    order contexts.  Information and source code for this algorithm
    can be found on the internet. Consult with the author of this
    algorithm for information on terms or restrictions on use.

    5.10.2 Support for PPMd within the ZIP format currently is provided only 
```

