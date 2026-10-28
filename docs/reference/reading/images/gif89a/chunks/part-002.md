# Graphics Interchange Format — part 2

Source: [Graphics Interchange Format](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) · 89a, 1990-07-31.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md) · [Next](part-003.md)

<a id="source-page-1-part-2"></a>

## Source page 1 (continued, part 2)

```text

encoder or a decoder may be compliant with a given version number and not
compliant with some subsequent version.


10. About Recommendations.

Each block section in this document contains an entry labeled Recommendation;
this section lists a set of recommendations intended to guide and organize the
use of the particular blocks. Such recommendations are geared towards making
the functions of encoders and decoders more efficient, as well as making
optimal use of the communications bandwidth.  It is advised that these
recommendations be followed.


11. About Color Tables.

The GIF format utilizes color tables to render raster-based graphics. A color
table can have one of two different scopes: global or local. A Global Color
Table is used by all those graphics in the Data Stream which do not have a
Local Color Table associated with them. The scope of the Global Color Table is
the entire Data Stream. A Local Color Table is always associated with the
graphic that immediately follows it; the scope of a Local Color Table is
limited to that single graphic. A Local Color Table supersedes a Global Color
Table, that is, if a Data Stream contains a Global Color Table, and an image
has a Local Color Table associated with it, the decoder must save the Global
Color Table, use the Local Color Table to render the image, and then restore
the Global Color Table. Both types of color tables are optional, making it
possible for a Data Stream to contain numerous graphics without a color table
at all. For this reason, it is recommended that the decoder save the last
Global Color Table used until another Global Color Table is encountered. In
this way, a Data Stream which does not contain either a Global Color Table or
a Local Color Table may be processed using the last Global Color Table saved.
If a Global Color Table from a previous Stream is used, that table becomes the
Global Color Table of the present Stream. This is intended to reduce the
overhead incurred by color tables. In particular, it is recommended that an
encoder use only one Global Color Table if all the images in related Data
Streams can be rendered with the same table.  If no color table is available at
all, the decoder is free to use a system color table or a table of its own. In
that case, the decoder may use a color table with as many colors as its
hardware is able to support; it is recommended that such a table have black and
white as its first two entries, so that monochrome images can be rendered
adequately.

The Definition of the GIF Format allows for a Data Stream to contain only the
Header, the Logical Screen Descriptor, a Global Color Table and the GIF
Trailer. Such a Data Stream would be used to load a decoder with a Global Color
Table, in preparation for subsequent Data Streams without a color table at all.


12. Blocks, Extensions and Scope.

Blocks can be classified into three groups : Control, Graphic-Rendering and
Special Purpose.  Control blocks, such as the Header, the Logical Screen
Descriptor, the Graphic Control Extension and the Trailer, contain information
used to control the process of the Data Stream or information  used in setting
hardware parameters.  Graphic-Rendering blocks such as the Image Descriptor and







                                                                         5


the Plain Text Extension contain information and data used to render a graphic
on the display device. Special Purpose blocks such as the Comment Extension and
the Application Extension are neither used to control the process of the Data
Stream nor do they contain information or data used to render a graphic on the
display device. With the exception of the Logical Screen Descriptor and the
Global Color Table, whose scope is the entire Data Stream, all other Control
blocks have a limited scope, restricted to the Graphic-Rendering block that
follows them.  Special Purpose blocks do not delimit the scope of any Control
blocks; Special Purpose blocks are transparent to the decoding process.
Graphic-Rendering blocks and extensions are used as scope delimiters for
Control blocks and extensions. The labels used to identify labeled blocks fall
into three ranges : 0x00-0x7F (0-127) are the Graphic Rendering blocks,
excluding the Trailer (0x3B); 0x80-0xF9 (128-249) are the Control blocks;
0xFA-0xFF (250-255) are the Special Purpose blocks. These ranges are defined so
that decoders can handle block scope by appropriately identifying block labels,
even when the block itself cannot be processed.


13. Block Sizes.

The Block Size field in a block, counts the number of bytes remaining in the
block, not counting the Block Size field itself, and not counting the Block
Terminator, if one is to follow.  Blocks other than Data Blocks are intended to
be of fixed length; the Block Size field is provided in order to facilitate
skipping them, not to allow their size to change in the future.  Data blocks
and sub-blocks are of variable length to accommodate the amount of data.


14. Using GIF as an embedded protocol.

As an embedded protocol, GIF may be part of larger application protocols,
within which GIF is used to render graphics.  In such a case, the application
protocol could define a block within which the GIF Data Stream would be
contained. The application program would then invoke a GIF decoder upon
encountering a block of type GIF.  This approach is recommended in favor of
using Application Extensions, which become overhead for all other applications
that do not process them. Because a GIF Data Stream must be processed in
context, the application must rely on some means of identifying the GIF Data
Stream outside of the Stream itself.


15. Data Sub-blocks.

      a. Description. Data Sub-blocks are units containing data. They do not
      have a label, these blocks are processed in the context of control
      blocks, wherever data blocks are specified in the format. The first byte
      of the Data sub-block indicates the number of data bytes to follow. A
      data sub-block may contain from 0 to 255 data bytes. The size of the
      block does not account for the size byte itself, therefore, the empty
      sub-block is one whose size field contains 0x00.

      b. Required Version.  87a.











                                                                         6


      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Block Size                    Byte
     +---------------+
  1  |               |
     +-             -+
  2  |               |
     +-             -+
  3  |               |
     +-             -+
     |               |       Data Values                   Byte
     +-             -+
 up  |               |
     +-   . . . .   -+
 to  |               |
     +-             -+
     |               |
     +-             -+
255  |               |
     +---------------+

            i) Block Size - Number of bytes in the Data Sub-block; the size
            must be within 0 and 255 bytes, inclusive.

            ii) Data Values - Any 8-bit value. There must be exactly as many
            Data Values as specified by the Block Size field.

      d. Extensions and Scope. This type of block always occurs as part of a
      larger unit. It does not have a scope of itself.

      e. Recommendation. None.


16. Block Terminator.

      a. Description. This zero-length Data Sub-block is used to terminate a
      sequence of Data Sub-blocks. It contains a single byte in the position of
      the Block Size field and does not contain data.

      b. Required Version.  87a.

      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Block Size                    Byte
     +---------------+

            i) Block Size - Number of bytes in the Data Sub-block; this field
            contains the fixed value 0x00.

            ii) Data Values - This block does not contain any data.









                                                                         7


      d. Extensions and Scope. This block terminates the immediately preceding
      sequence of Data Sub-blocks. This block cannot be modified by any
      extension.

      e. Recommendation. None.


17. Header.

      a. Description. The Header identifies the GIF Data Stream in context. The
      Signature field marks the beginning of the Data Stream, and the Version
      field identifies the set of capabilities required of a decoder to fully
      process the Data Stream.  This block is REQUIRED; exactly one Header must
      be present per Data Stream.

      b. Required Version.  Not applicable. This block is not subject to a
      version number. This block must appear at the beginning of every Data
      Stream.

      c. Syntax.


      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
   0 |               |       Signature                     3 Bytes
     +-             -+
   1 |               |
     +-             -+
   2 |               |
     +---------------+
   3 |               |       Version                       3 Bytes
     +-             -+
   4 |               |
     +-             -+
   5 |               |
     +---------------+

            i) Signature - Identifies the GIF Data Stream. This field contains
            the fixed value 'GIF'.

            ii) Version - Version number used to format the data stream.
            Identifies the minimum set of capabilities necessary to a decoder
            to fully process the contents of the Data Stream.

            Version Numbers as of 10 July 1990 :       "87a" - May 1987
                                                       "89a" - July 1989

            Version numbers are ordered numerically increasing on the first two
            digits starting with 87 (87,88,...,99,00,...,85,86) and
            alphabetically increasing on the third character (a,...,z).

            iii) Extensions and Scope. The scope of this block is the entire
            Data Stream. This block cannot be modified by any extension.










                                                                         8


      d. Recommendations.

            i) Signature - This field identifies the beginning of the GIF Data
            Stream; it is not intended to provide a unique signature for the
            identification of the data. It is recommended that the GIF Data
            Stream be identified externally by the application. (Refer to
            Appendix G for on-line identification of the GIF Data Stream.)

            ii) Version - ENCODER : An encoder should use the earliest possible
            version number that defines all the blocks used in the Data Stream.
            When two or more Data Streams are combined, the latest of the
            individual version numbers should be used for the resulting Data
            Stream. DECODER : A decoder should attempt to process the data
            stream to the best of its ability; if it encounters a version
            number which it is not capable of processing fully, it should
            nevertheless, attempt to process the data stream to the best of its
            ability, perhaps after warning the user that the data may be
            incomplete.


18. Logical Screen Descriptor.

      a. Description.  The Logical Screen Descriptor contains the parameters
      necessary to define the area of the display device within which the
      images will be rendered.  The coordinates in this block are given with
      respect to the top-left corner of the virtual screen; they do not
      necessarily refer to absolute coordinates on the display device.  This
      implies that they could refer to window coordinates in a window-based
      environment or printer coordinates when a printer is used.

      This block is REQUIRED; exactly one Logical Screen Descriptor must be
      present per Data Stream.

      b. Required Version.  Not applicable. This block is not subject to a
      version number. This block must appear immediately after the Header.

      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Logical Screen Width          Unsigned
     +-             -+
  1  |               |
     +---------------+
  2  |               |       Logical Screen Height         Unsigned
     +-             -+
  3  |               |
     +---------------+
  4  | |     | |     |       <Packed Fields>               See below
     +---------------+
  5  |               |       Background Color Index        Byte
     +---------------+
  6  |               |       Pixel Aspect Ratio            Byte
     +---------------+









                                                                         9


     <Packed Fields>  =      Global Color Table Flag       1 Bit
                             Color Resolution              3 Bits
                             Sort Flag                     1 Bit
                             Size of Global Color Table    3 Bits

            i) Logical Screen Width - Width, in pixels, of the Logical Screen
            where the images will be rendered in the displaying device.

            ii) Logical Screen Height - Height, in pixels, of the Logical
            Screen where the images will be rendered in the displaying device.
```

