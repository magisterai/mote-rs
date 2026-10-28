# Graphics Interchange Format — part 3

Source: [Graphics Interchange Format](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) · 89a, 1990-07-31.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-002.md) · [Next](part-004.md)

<a id="source-page-1-part-3"></a>

## Source page 1 (continued, part 3)

```text

            iii) Global Color Table Flag - Flag indicating the presence of a
            Global Color Table; if the flag is set, the Global Color Table will
            immediately follow the Logical Screen Descriptor. This flag also
            selects the interpretation of the Background Color Index; if the
            flag is set, the value of the Background Color Index field should
            be used as the table index of the background color. (This field is
            the most significant bit of the byte.)

            Values :    0 -   No Global Color Table follows, the Background
                              Color Index field is meaningless.
                        1 -   A Global Color Table will immediately follow, the
                              Background Color Index field is meaningful.

            iv) Color Resolution - Number of bits per primary color available
            to the original image, minus 1. This value represents the size of
            the entire palette from which the colors in the graphic were
            selected, not the number of colors actually used in the graphic.
            For example, if the value in this field is 3, then the palette of
            the original image had 4 bits per primary color available to create
            the image.  This value should be set to indicate the richness of
            the original palette, even if not every color from the whole
            palette is available on the source machine.

            v) Sort Flag - Indicates whether the Global Color Table is sorted.
            If the flag is set, the Global Color Table is sorted, in order of
            decreasing importance. Typically, the order would be decreasing
            frequency, with most frequent color first. This assists a decoder,
            with fewer available colors, in choosing the best subset of colors;
            the decoder may use an initial segment of the table to render the
            graphic.

            Values :    0 -   Not ordered.
                        1 -   Ordered by decreasing importance, most
                              important color first.

            vi) Size of Global Color Table - If the Global Color Table Flag is
            set to 1, the value in this field is used to calculate the number
            of bytes contained in the Global Color Table. To determine that
            actual size of the color table, raise 2 to [the value of the field
            + 1].  Even if there is no Global Color Table specified, set this
            field according to the above formula so that decoders can choose
            the best graphics mode to display the stream in.  (This field is
            made up of the 3 least significant bits of the byte.)

            vii) Background Color Index - Index into the Global Color Table for







                                                                        10


            the Background Color. The Background Color is the color used for
            those pixels on the screen that are not covered by an image. If the
            Global Color Table Flag is set to (zero), this field should be zero
            and should be ignored.

            viii) Pixel Aspect Ratio - Factor used to compute an approximation
            of the aspect ratio of the pixel in the original image.  If the
            value of the field is not 0, this approximation of the aspect ratio
            is computed based on the formula:

            Aspect Ratio = (Pixel Aspect Ratio + 15) / 64

            The Pixel Aspect Ratio is defined to be the quotient of the pixel's
            width over its height.  The value range in this field allows
            specification of the widest pixel of 4:1 to the tallest pixel of
            1:4 in increments of 1/64th.

            Values :        0 -   No aspect ratio information is given.
                       1..255 -   Value used in the computation.

      d. Extensions and Scope. The scope of this block is the entire Data
      Stream. This block cannot be modified by any extension.

      e. Recommendations. None.


19. Global Color Table.

      a. Description. This block contains a color table, which is a sequence of
      bytes representing red-green-blue color triplets. The Global Color Table
      is used by images without a Local Color Table and by Plain Text
      Extensions. Its presence is marked by the Global Color Table Flag being
      set to 1 in the Logical Screen Descriptor; if present, it immediately
      follows the Logical Screen Descriptor and contains a number of bytes
      equal to
                    3 x 2^(Size of Global Color Table+1).

      This block is OPTIONAL; at most one Global Color Table may be present
      per Data Stream.

      b. Required Version.  87a






















                                                                        11


      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +===============+
  0  |               |       Red 0                         Byte
     +-             -+
  1  |               |       Green 0                       Byte
     +-             -+
  2  |               |       Blue 0                        Byte
     +-             -+
  3  |               |       Red 1                         Byte
     +-             -+
     |               |       Green 1                       Byte
     +-             -+
 up  |               |
     +-   . . . .   -+       ...
 to  |               |
     +-             -+
     |               |       Green 255                     Byte
     +-             -+
767  |               |       Blue 255                      Byte
     +===============+


      d. Extensions and Scope. The scope of this block is the entire Data
      Stream. This block cannot be modified by any extension.

      e. Recommendation. None.


20. Image Descriptor.

      a. Description. Each image in the Data Stream is composed of an Image
      Descriptor, an optional Local Color Table, and the image data.  Each
      image must fit within the boundaries of the Logical Screen, as defined
      in the Logical Screen Descriptor.

      The Image Descriptor contains the parameters necessary to process a table
      based image. The coordinates given in this block refer to coordinates
      within the Logical Screen, and are given in pixels. This block is a
      Graphic-Rendering Block, optionally preceded by one or more Control
      blocks such as the Graphic Control Extension, and may be optionally
      followed by a Local Color Table; the Image Descriptor is always followed
      by the image data.

      This block is REQUIRED for an image.  Exactly one Image Descriptor must
      be present per image in the Data Stream.  An unlimited number of images
      may be present per Data Stream.

      b. Required Version.  87a.













                                                                        12


      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Image Separator               Byte
     +---------------+
  1  |               |       Image Left Position           Unsigned
     +-             -+
  2  |               |
     +---------------+
  3  |               |       Image Top Position            Unsigned
     +-             -+
  4  |               |
     +---------------+
  5  |               |       Image Width                   Unsigned
     +-             -+
  6  |               |
     +---------------+
  7  |               |       Image Height                  Unsigned
     +-             -+
  8  |               |
     +---------------+
  9  | | | |   |     |       <Packed Fields>               See below
     +---------------+

     <Packed Fields>  =      Local Color Table Flag        1 Bit
                             Interlace Flag                1 Bit
                             Sort Flag                     1 Bit
                             Reserved                      2 Bits
                             Size of Local Color Table     3 Bits

           i) Image Separator - Identifies the beginning of an Image
           Descriptor. This field contains the fixed value 0x2C.

           ii) Image Left Position - Column number, in pixels, of the left edge
           of the image, with respect to the left edge of the Logical Screen.
           Leftmost column of the Logical Screen is 0.

           iii) Image Top Position - Row number, in pixels, of the top edge of
           the image with respect to the top edge of the Logical Screen. Top
           row of the Logical Screen is 0.

           iv) Image Width - Width of the image in pixels.

           v) Image Height - Height of the image in pixels.

           vi) Local Color Table Flag - Indicates the presence of a Local Color
           Table immediately following this Image Descriptor. (This field is
           the most significant bit of the byte.)


           Values :    0 -   Local Color Table is not present. Use
                             Global Color Table if available.
                       1 -   Local Color Table present, and to follow
                             immediately after this Image Descriptor.








                                                                        13


           vii) Interlace Flag - Indicates if the image is interlaced. An image
           is interlaced in a four-pass interlace pattern; see Appendix E for
           details.

           Values :    0 - Image is not interlaced.
                       1 - Image is interlaced.

            viii) Sort Flag - Indicates whether the Local Color Table is
            sorted.  If the flag is set, the Local Color Table is sorted, in
            order of decreasing importance. Typically, the order would be
            decreasing frequency, with most frequent color first. This assists
            a decoder, with fewer available colors, in choosing the best subset
            of colors; the decoder may use an initial segment of the table to
            render the graphic.

            Values :    0 -   Not ordered.
                        1 -   Ordered by decreasing importance, most
                              important color first.

            ix) Size of Local Color Table - If the Local Color Table Flag is
            set to 1, the value in this field is used to calculate the number
            of bytes contained in the Local Color Table. To determine that
            actual size of the color table, raise 2 to the value of the field
            + 1. This value should be 0 if there is no Local Color Table
            specified. (This field is made up of the 3 least significant bits
            of the byte.)

     d. Extensions and Scope. The scope of this block is the Table-based Image
     Data Block that follows it. This block may be modified by the Graphic
     Control Extension.

     e. Recommendation. None.


21. Local Color Table.

     a. Description. This block contains a color table, which is a sequence of
     bytes representing red-green-blue color triplets. The Local Color Table
     is used by the image that immediately follows. Its presence is marked by
     the Local Color Table Flag being set to 1 in the Image Descriptor; if
     present, the Local Color Table immediately follows the Image Descriptor
     and contains a number of bytes equal to
                          3x2^(Size of Local Color Table+1).
     If present, this color table temporarily becomes the active color table
     and the following image should be processed using it. This block is
     OPTIONAL; at most one Local Color Table may be present per Image
     Descriptor and its scope is the single image associated with the Image
     Descriptor that precedes it.

     b. Required Version.  87a.













                                                                        14


     c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +===============+
  0  |               |       Red 0                         Byte
     +-             -+
  1  |               |       Green 0                       Byte
     +-             -+
  2  |               |       Blue 0                        Byte
     +-             -+
  3  |               |       Red 1                         Byte
     +-             -+
     |               |       Green 1                       Byte
     +-             -+
 up  |               |
     +-   . . . .   -+       ...
 to  |               |
     +-             -+
     |               |       Green 255                     Byte
     +-             -+
767  |               |       Blue 255                      Byte
     +===============+


     d. Extensions and Scope. The scope of this block is the Table-based Image
     Data Block that immediately follows it. This block cannot be modified by
     any extension.

     e. Recommendations. None.


22. Table Based Image Data.

     a. Description. The image data for a table based image consists of a
     sequence of sub-blocks, of size at most 255 bytes each, containing an
     index into the active color table, for each pixel in the image.  Pixel
     indices are in order of left to right and from top to bottom.  Each index
     must be within the range of the size of the active color table, starting
     at 0. The sequence of indices is encoded using the LZW Algorithm with
     variable-length code, as described in Appendix F

     b. Required Version.  87a.

     c. Syntax. The image data format is as follows:

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
     |               |       LZW Minimum Code Size         Byte
     +---------------+

     +===============+
     |               |
     /               /       Image Data                    Data Sub-blocks
     |               |
     +===============+







```

