# Graphics Interchange Format — part 5

Source: [Graphics Interchange Format](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) · 89a, 1990-07-31.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-004.md) · [Next](part-006.md)

<a id="source-page-1-part-5"></a>

## Source page 1 (continued, part 5)

```text

            v) Text Grid Top Position - Row number, in pixels, of the top edge
            of the text grid, with respect to the top edge of the Logical
            Screen.

            vi) Image Grid Width - Width of the text grid in pixels.

            vii) Image Grid Height - Height of the text grid in pixels.

            viii) Character Cell Width - Width, in pixels, of each cell in the
            grid.

            ix) Character Cell Height - Height, in pixels, of each cell in the
            grid.

            x) Text Foreground Color Index - Index into the Global Color Table
            to be used to render the text foreground.

            xi) Text Background Color Index - Index into the Global Color Table
            to be used to render the text background.

            xii) Plain Text Data - Sequence of sub-blocks, each of size at most
            255 bytes and at least 1 byte, with the size in a byte preceding
            the data.  The end of the sequence is marked by the Block
            Terminator.

            xiii) Block Terminator - This zero-length data block marks the end
            of the Plain Text Data Blocks.

      d. Extensions and Scope. The scope of this block is the Plain Text Data
      Block contained in it. This block may be modified by the Graphic Control
      Extension.

      e. Recommendations. The data in the Plain Text Extension is assumed to be
      preformatted. The selection of font and size is left to the discretion of
      the decoder.  If characters less than 0x20 or greater than 0xf7 are
      encountered, it is recommended that the decoder display a Space character
      (0x20). The encoder should use grid and cell dimensions such that an
      integral number of cells fit in the grid both horizontally as well as
      vertically.  For broadest compatibility, character cell dimensions should
      be around 8x8 or 8x16 (width x height); consider an image for unusual
      sized text.


26. Application Extension.

      a. Description. The Application Extension contains application-specific
      information; it conforms with the extension block syntax, as described
      below, and its block label is 0xFF.

      b. Required Version.  89a.









                                                                        22


      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Extension Introducer          Byte
     +---------------+
  1  |               |       Extension Label               Byte
     +---------------+

     +---------------+
  0  |               |       Block Size                    Byte
     +---------------+
  1  |               |
     +-             -+
  2  |               |
     +-             -+
  3  |               |       Application Identifier        8 Bytes
     +-             -+
  4  |               |
     +-             -+
  5  |               |
     +-             -+
  6  |               |
     +-             -+
  7  |               |
     +-             -+
  8  |               |
     +---------------+
  9  |               |
     +-             -+
 10  |               |       Appl. Authentication Code     3 Bytes
     +-             -+
 11  |               |
     +---------------+

     +===============+
     |               |
     |               |       Application Data              Data Sub-blocks
     |               |
     |               |
     +===============+

     +---------------+
  0  |               |       Block Terminator              Byte
     +---------------+

            i) Extension Introducer - Defines this block as an extension. This
            field contains the fixed value 0x21.

            ii) Application Extension Label - Identifies the block as an
            Application Extension. This field contains the fixed value 0xFF.

            iii) Block Size - Number of bytes in this extension block,
            following the Block Size field, up to but not including the
            beginning of the Application Data. This field contains the fixed
            value 11.







                                                                        23


            iv) Application Identifier - Sequence of eight printable ASCII
            characters used to identify the application owning the Application
            Extension.

            v) Application Authentication Code - Sequence of three bytes used
            to authenticate the Application Identifier. An Application program
            may use an algorithm to compute a binary code that uniquely
            identifies it as the application owning the Application Extension.


      d. Extensions and Scope. This block does not have scope. This block
      cannot be modified by any extension.

      e. Recommendation. None.


27. Trailer.

      a. Description. This block is a single-field block indicating the end of
      the GIF Data Stream.  It contains the fixed value 0x3B.

      b. Required Version.  87a.

      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       GIF Trailer                   Byte
     +---------------+

      d. Extensions and Scope. This block does not have scope, it terminates
      the GIF Data Stream. This block may not be modified by any extension.

      e. Recommendations. None.





























                                                                        24


Appendix
A. Quick Reference Table.

Block Name                  Required   Label       Ext.   Vers.
Application Extension       Opt. (*)   0xFF (255)  yes    89a
Comment Extension           Opt. (*)   0xFE (254)  yes    89a
Global Color Table          Opt. (1)   none        no     87a
Graphic Control Extension   Opt. (*)   0xF9 (249)  yes    89a
Header                      Req. (1)   none        no     N/A
Image Descriptor            Opt. (*)   0x2C (044)  no     87a (89a)
Local Color Table           Opt. (*)   none        no     87a
Logical Screen Descriptor   Req. (1)   none        no     87a (89a)
Plain Text Extension        Opt. (*)   0x01 (001)  yes    89a
Trailer                     Req. (1)   0x3B (059)  no     87a

Unlabeled Blocks
Header                      Req. (1)   none        no     N/A
Logical Screen Descriptor   Req. (1)   none        no     87a (89a)
Global Color Table          Opt. (1)   none        no     87a
Local Color Table           Opt. (*)   none        no     87a

Graphic-Rendering Blocks
Plain Text Extension        Opt. (*)   0x01 (001)  yes    89a
Image Descriptor            Opt. (*)   0x2C (044)  no     87a (89a)

Control Blocks
Graphic Control Extension   Opt. (*)   0xF9 (249)  yes    89a

Special Purpose Blocks
Trailer                     Req. (1)   0x3B (059)  no     87a
Comment Extension           Opt. (*)   0xFE (254)  yes    89a
Application Extension       Opt. (*)   0xFF (255)  yes    89a

legend:           (1)   if present, at most one occurrence
                  (*)   zero or more occurrences
                  (+)   one or more occurrences

Notes : The Header is not subject to Version Numbers.
(89a) The Logical Screen Descriptor and the Image Descriptor retained their
syntax from version 87a to version 89a, but some fields reserved under version
87a are used under version 89a.






















                                                                        25


Appendix
B. GIF Grammar.

A Grammar is a form of notation to represent the sequence in which certain
objects form larger objects.  A grammar is also used to represent the number of
objects that can occur at a given position.  The grammar given here represents
the sequence of blocks that form the GIF Data Stream. A grammar is given by
listing its rules.  Each rule consists of the left-hand side, followed by some
form of equals sign, followed by the right-hand side.  In a rule, the
right-hand side describes how the left-hand side is defined. The right-hand
side consists of a sequence of entities, with the possible presence of special
symbols. The following legend defines the symbols used in this grammar for GIF.

Legend:           <>    grammar word
                  ::=   defines symbol
                  *     zero or more occurrences
                  +     one or more occurrences
                  |     alternate element
                  []    optional element

Example:

<GIF Data Stream> ::= Header <Logical Screen> <Data>* Trailer

This rule defines the entity <GIF Data Stream> as follows. It must begin with a
Header. The Header is followed by an entity called Logical Screen, which is
defined below by another rule. The Logical Screen is followed by the entity
Data, which is also defined below by another rule. Finally, the entity Data is
followed by the Trailer.  Since there is no rule defining the Header or the
Trailer, this means that these blocks are defined in the document.  The entity
Data has a special symbol (*) following it which means that, at this position,
the entity Data may be repeated any number of times, including 0 times. For
further reading on this subject, refer to a standard text on Programming
Languages.


The Grammar.

<GIF Data Stream> ::=     Header <Logical Screen> <Data>* Trailer

<Logical Screen> ::=      Logical Screen Descriptor [Global Color Table]

<Data> ::=                <Graphic Block>  |
                          <Special-Purpose Block>

<Graphic Block> ::=       [Graphic Control Extension] <Graphic-Rendering Block>

<Graphic-Rendering Block> ::=  <Table-Based Image>  |
                               Plain Text Extension

<Table-Based Image> ::=   Image Descriptor [Local Color Table] Image Data

<Special-Purpose Block> ::=    Application Extension  |
                               Comment Extension









                                                                        26


NOTE : The grammar indicates that it is possible for a GIF Data Stream to
contain the Header, the Logical Screen Descriptor, a Global Color Table and the
GIF Trailer. This special case is used to load a GIF decoder with a Global
Color Table, in preparation for subsequent Data Streams without color tables at
all.


























































                                                                        27


Appendix
C. Glossary.

Active Color Table - Color table used to render the next graphic. If the next
graphic is an image which has a Local Color Table associated with it, the
active color table becomes the Local Color Table associated with that image.
If the next graphic is an image without a Local Color Table, or a Plain Text
Extension, the active color table is the Global Color Table associated with the
Data Stream, if there is one; if there is no Global Color Table in the Data
Stream, the active color table is a color table saved from a previous Data
Stream, or one supplied by the decoder.

Block - Collection of bytes forming a protocol unit. In general, the term
includes labeled and unlabeled blocks, as well as Extensions.

Data Stream - The GIF Data Stream is composed of blocks and sub-blocks
representing images and graphics, together with control information to render
them on a display device. All control and data blocks in the Data Stream must
follow the Header and must precede the Trailer.

Decoder - A program capable of processing a GIF Data Stream to render the
images and graphics contained in it.

Encoder - A program capable of capturing and formatting image and graphic
raster data, following the definitions of the Graphics Interchange Format.

Extension - A protocol block labeled by the Extension Introducer 0x21.

Extension Introducer - Label (0x21) defining an Extension.

Graphic - Data which can be rendered on the screen by virtue of some algorithm.
The term graphic is more general than the term image; in addition to images,
the term graphic also includes data such as text, which is rendered using
character bit-maps.

Image - Data representing a picture or a drawing; an image is represented by an
array of pixels called the raster of the image.

Raster - Array of pixel values representing an image.
























                                                                        28


Appendix
D. Conventions.

Animation - The Graphics Interchange Format is not intended as a platform for
animation, even though it can be done in a limited way.

Byte Ordering - Unless otherwise stated, multi-byte numeric fields are ordered
with the Least Significant Byte first.

Color Indices - Color indices always refer to the active color table, either
the Global Color Table or the Local Color Table.

Color Order - Unless otherwise stated, all triple-component RGB color values
are specified in Red-Green-Blue order.

Color Tables - Both color tables, the Global and the Local, are optional; if
present, the Global Color Table is to be used with every image in the Data
Stream for which a Local Color Table is not given; if present, a Local Color
Table overrides the Global Color Table.  However, if neither color table is
present, the application program is free to use an arbitrary color table. If
the graphics in several Data Streams are related and all use the same color
table, an encoder could place the color table as the Global Color Table in the
first Data Stream and leave subsequent Data Streams without a Global Color
Table or any Local Color Tables; in this way, the overhead for the table is
eliminated.  It is recommended that the decoder save the previous Global Color
Table to be used with the Data Stream that follows, in case it does not contain
either a Global Color Table or any Local Color Tables. In general, this allows
the application program to use past color tables, significantly reducing
transmission overhead.
```

