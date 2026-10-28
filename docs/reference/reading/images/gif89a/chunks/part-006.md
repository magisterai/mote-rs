# Graphics Interchange Format — part 6

Source: [Graphics Interchange Format](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) · 89a, 1990-07-31.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-005.md)

<a id="source-page-1-part-6"></a>

## Source page 1 (continued, part 6)

```text

Extension Blocks - Extensions are defined using the Extension Introducer code
to mark the beginning of the block, followed by a block label, identifying the
type of extension.  Extension Codes are numbers in the range from 0x00 to 0xFF,
inclusive. Special purpose extensions are transparent to the decoder and may be
omitted when transmitting the Data Stream on-line. The GIF capabilities
dialogue makes the provision for the receiver to request the transmission of
all blocks; the default state in this regard is no transmission of Special
purpose blocks.

Reserved Fields - All Reserved Fields are expected to have each bit set to zero
(off).






















                                                                        29


Appendix
E. Interlaced Images.

The rows of an Interlaced images are arranged in the following order:

      Group 1 : Every 8th. row, starting with row 0.              (Pass 1)
      Group 2 : Every 8th. row, starting with row 4.              (Pass 2)
      Group 3 : Every 4th. row, starting with row 2.              (Pass 3)
      Group 4 : Every 2nd. row, starting with row 1.              (Pass 4)

The Following example illustrates how the rows of an interlaced image are
ordered.

      Row Number                                        Interlace Pass

 0    -----------------------------------------       1
 1    -----------------------------------------                         4
 2    -----------------------------------------                   3
 3    -----------------------------------------                         4
 4    -----------------------------------------             2
 5    -----------------------------------------                         4
 6    -----------------------------------------                   3
 7    -----------------------------------------                         4
 8    -----------------------------------------       1
 9    -----------------------------------------                         4
 10   -----------------------------------------                   3
 11   -----------------------------------------                         4
 12   -----------------------------------------             2
 13   -----------------------------------------                         4
 14   -----------------------------------------                   3
 15   -----------------------------------------                         4
 16   -----------------------------------------       1
 17   -----------------------------------------                         4
 18   -----------------------------------------                   3
 19   -----------------------------------------                         4




























                                                                        30


Appendix
F. Variable-Length-Code LZW Compression.

The Variable-Length-Code LZW Compression is a variation of the Lempel-Ziv
Compression algorithm in which variable-length codes are used to replace
patterns detected in the original data. The algorithm uses a code or
translation table constructed from the patterns encountered in the original
data; each new pattern is entered into the table and its index is used to
replace it in the compressed stream.

The compressor takes the data from the input stream and builds a code or
translation table with the patterns as it encounters them; each new pattern is
entered into the code table and its index is added to the output stream; when a
pattern is encountered which had been detected since the last code table
refresh, its index from the code table is put on the output stream, thus
achieving the data compression.  The expander takes input from the compressed
data stream and builds the code or translation table from it; as the compressed
data stream is processed, codes are used to index into the code table and the
corresponding data is put on the decompressed output stream, thus achieving
data decompression.  The details of the algorithm are explained below.  The
Variable-Length-Code aspect of the algorithm is based on an initial code size
(LZW-initial code size), which specifies the initial number of bits used for
the compression codes.  When the number of patterns detected by the compressor
in the input stream exceeds the number of patterns encodable with the current
number of bits, the number of bits per LZW code is increased by one.

The Raster Data stream that represents the actual output image can be
represented as:

         7 6 5 4 3 2 1 0
        +---------------+
        | LZW code size |
        +---------------+

        +---------------+ ----+
        |  block size   |     |
        +---------------+     |
        |               |     +-- Repeated as many
        |  data bytes   |     |   times as necessary.
        |               |     |
        +---------------+ ----+

        . . .       . . . ------- The code that terminates the LZW
                                  compressed data must appear before
                                  Block Terminator.
        +---------------+
        |0 0 0 0 0 0 0 0|  Block Terminator
        +---------------+

The conversion of the image from a series of pixel values to a transmitted or
stored character stream involves several steps. In brief these steps are:

1. Establish the Code Size - Define the number of bits needed to represent the
actual data.

2. Compress the Data - Compress the series of image pixels to a series of







                                                                        31


compression codes.

3. Build a Series of Bytes - Take the set of compression codes and convert to a
string of 8-bit bytes.

4. Package the Bytes - Package sets of bytes into blocks preceded by character
counts and output.

ESTABLISH CODE SIZE

The first byte of the Compressed Data stream is a value indicating the minimum
number of bits required to represent the set of actual pixel values. Normally
this will be the same as the number of color bits. Because of some algorithmic
constraints however, black & white images which have one color bit must be
indicated as having a code size of 2.
This code size value also implies that the compression codes must start out one
bit longer.

COMPRESSION

The LZW algorithm converts a series of data values into a series of codes which
may be raw values or a code designating a series of values. Using text
characters as an analogy, the output code consists of a character or a code
representing a string of characters.

The LZW algorithm used in GIF matches algorithmically with the standard LZW
algorithm with the following differences:

1.  A special Clear code is defined which resets all compression/decompression
parameters and tables to a start-up state. The value of this code is 2**<code
size>. For example if the code size indicated was 4 (image was 4 bits/pixel)
the Clear code value would be 16 (10000 binary). The Clear code can appear at
any point in the image data stream and therefore requires the LZW algorithm to
process succeeding codes as if a new data stream was starting. Encoders should
output a Clear code as the first code of each image data stream.

2. An End of Information code is defined that explicitly indicates the end of
the image data stream. LZW processing terminates when this code is encountered.
It must be the last code output by the encoder for an image. The value of this
code is <Clear code>+1.

3. The first available compression code value is <Clear code>+2.

4. The output codes are of variable length, starting at <code size>+1 bits per
code, up to 12 bits per code. This defines a maximum code value of 4095
(0xFFF). Whenever the LZW code value would exceed the current code length, the
code length is increased by one. The packing/unpacking of these codes must then
be altered to reflect the new code length.

BUILD 8-BIT BYTES

Because the LZW compression used for GIF creates a series of variable length
codes, of between 3 and 12 bits each, these codes must be reformed into a
series of 8-bit bytes that will be the characters actually stored or
transmitted. This provides additional compression of the image. The codes are
formed into a stream of bits as if they were packed right to left and then







                                                                        32


picked off 8 bits at a time to be output.

Assuming a character array of 8 bits per character and using 5 bit codes to be
packed, an example layout would be similar to:


     +---------------+
  0  |               |    bbbaaaaa
     +---------------+
  1  |               |    dcccccbb
     +---------------+
  2  |               |    eeeedddd
     +---------------+
  3  |               |    ggfffffe
     +---------------+
  4  |               |    hhhhhggg
     +---------------+
           . . .
     +---------------+
  N  |               |
     +---------------+


Note that the physical packing arrangement will change as the number of bits
per compression code change but the concept remains the same.

PACKAGE THE BYTES

Once the bytes have been created, they are grouped into blocks for output by
preceding each block of 0 to 255 bytes with a character count byte. A block
with a zero byte count terminates the Raster Data stream for a given image.
These blocks are what are actually output for the GIF image. This block format
has the side effect of allowing a decoding program the ability to read past the
actual image data if necessary by reading block counts and then skipping over
the data.



FURTHER READING

[1] Ziv, J. and Lempel, A. : "A Universal Algorithm for Sequential Data
Compression", IEEE Transactions on Information Theory, May 1977.
[2] Welch, T. : "A Technique for High-Performance Data Compression", Computer,
June 1984.
[3] Nelson, M.R. : "LZW Data Compression", Dr. Dobb's Journal, October 1989.


















                                                                        33


Appendix
G. On-line Capabilities Dialogue.

NOTE : This section is currently (10 July 1990) under revision; the information
provided here should be used as general guidelines. Code written based on this
information should be designed in a flexible way to accommodate any changes
resulting from the revisions.

The following sequences are defined for use in mediating control between a GIF
sender and GIF receiver over an interactive communications line. These
sequences do not apply to applications that involve downloading of static GIF
files and are not considered part of a GIF file.

GIF CAPABILITIES ENQUIRY

The GIF Capabilities Enquiry sequence is issued from a host and requests an
interactive GIF decoder to return a response message that defines the graphics
parameters for the decoder. This involves returning information about available
screen sizes, number of bits/color supported and the amount of color detail
supported. The escape sequence for the GIF Capabilities Enquiry is defined as:

ESC[>0g           0x1B 0x5B 0x3E 0x30 0x67

GIF CAPABILITIES RESPONSE

The GIF Capabilities Response message is returned by an interactive GIF decoder
and defines the decoder's display capabilities for all graphics modes that are
supported by the software. Note that this can also include graphics printers as
well as a monitor screen. The general format of this message is:

#version;protocol{;dev, width, height, color-bits, color-res}...<CR>


'#'            GIF Capabilities Response identifier character.
version        GIF format version number;  initially '87a'.
protocol='0'   No end-to-end protocol supported by decoder Transfer as direct
               8-bit data stream.
protocol='1'   Can use CIS B+ error correction protocol to transfer GIF data
               interactively from the host directly to the display.
dev = '0'      Screen parameter set follows.
dev = '1'      Printer parameter set follows.
width          Maximum supported display width in pixels.
height         Maximum supported display height in pixels.
color-bits     Number of bits per pixel supported. The number of supported
               colors is therefore 2**color-bits.
color-res      Number of bits per color component supported in the hardware
               color palette. If color-res is '0' then no hardware palette
               table is available.

Note that all values in the GIF Capabilities Response are returned as ASCII
decimal numbers and the message is terminated by a Carriage Return character.

The following GIF Capabilities Response message describes three standard IBM PC
Enhanced Graphics Adapter configurations with no printer; the GIF data stream









                                                                        34


can be processed within an error correcting protocol:

#87a;1;0,320,200,4,0;0,640,200,2,2;0,640,350,4,2<CR>

ENTER GIF GRAPHICS MODE

Two sequences are currently defined to invoke an interactive GIF decoder into
action. The only difference between them is that different output media are
selected. These sequences are:

ESC[>1g     Display GIF image on screen

                  0x1B 0x5B 0x3E 0x31 0x67

ESC[>2g   Display image directly to an attached graphics printer. The image may
optionally be displayed on the screen as well.

                  0x1B 0x5B 0x3E 0x32 0x67

Note that the 'g' character terminating each sequence is in lowercase.

INTERACTIVE ENVIRONMENT

The assumed environment for the transmission of GIF image data from an
interactive application is a full 8-bit data stream from host to micro.  All
256 character codes must be transferrable. The establishing of an 8-bit data
path for communications will normally be taken care of by the host application
programs. It is however up to the receiving communications programs supporting
GIF to be able to receive and pass on all 256 8-bit codes to the GIF decoder
software.
.
```

