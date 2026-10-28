# Graphics Interchange Format — part 4

Source: [Graphics Interchange Format](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) · 89a, 1990-07-31.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-003.md) · [Next](part-005.md)

<a id="source-page-1-part-4"></a>

## Source page 1 (continued, part 4)

```text

                                                                        15


            i) LZW Minimum Code Size.  This byte determines the initial number
            of bits used for LZW codes in the image data, as described in
            Appendix F.

     d. Extensions and Scope. This block has no scope, it contains raster
     data. Extensions intended to modify a Table-based image must appear
     before the corresponding Image Descriptor.

     e. Recommendations. None.


23. Graphic Control Extension.

      a. Description. The Graphic Control Extension contains parameters used
      when processing a graphic rendering block. The scope of this extension is
      the first graphic rendering block to follow. The extension contains only
      one data sub-block.

      This block is OPTIONAL; at most one Graphic Control Extension may precede
      a graphic rendering block. This is the only limit to the number of
      Graphic Control Extensions that may be contained in a Data Stream.

      b. Required Version.  89a.

      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Extension Introducer          Byte
     +---------------+
  1  |               |       Graphic Control Label         Byte
     +---------------+

     +---------------+
  0  |               |       Block Size                    Byte
     +---------------+
  1  |     |     | | |       <Packed Fields>               See below
     +---------------+
  2  |               |       Delay Time                    Unsigned
     +-             -+
  3  |               |
     +---------------+
  4  |               |       Transparent Color Index       Byte
     +---------------+

     +---------------+
  0  |               |       Block Terminator              Byte
     +---------------+


      <Packed Fields>  =     Reserved                      3 Bits
                             Disposal Method               3 Bits
                             User Input Flag               1 Bit
                             Transparent Color Flag        1 Bit

            i) Extension Introducer - Identifies the beginning of an extension







                                                                        16


            block. This field contains the fixed value 0x21.

            ii) Graphic Control Label - Identifies the current block as a
            Graphic Control Extension. This field contains the fixed value
            0xF9.

            iii) Block Size - Number of bytes in the block, after the Block
            Size field and up to but not including the Block Terminator.  This
            field contains the fixed value 4.

            iv) Disposal Method - Indicates the way in which the graphic is to
            be treated after being displayed.

            Values :    0 -   No disposal specified. The decoder is
                              not required to take any action.
                        1 -   Do not dispose. The graphic is to be left
                              in place.
                        2 -   Restore to background color. The area used by the
                              graphic must be restored to the background color.
                        3 -   Restore to previous. The decoder is required to
                              restore the area overwritten by the graphic with
                              what was there prior to rendering the graphic.
                     4-7 -    To be defined.

            v) User Input Flag - Indicates whether or not user input is
            expected before continuing. If the flag is set, processing will
            continue when user input is entered. The nature of the User input
            is determined by the application (Carriage Return, Mouse Button
            Click, etc.).

            Values :    0 -   User input is not expected.
                        1 -   User input is expected.

            When a Delay Time is used and the User Input Flag is set,
            processing will continue when user input is received or when the
            delay time expires, whichever occurs first.

            vi) Transparency Flag - Indicates whether a transparency index is
            given in the Transparent Index field. (This field is the least
            significant bit of the byte.)

            Values :    0 -   Transparent Index is not given.
                        1 -   Transparent Index is given.

            vii) Delay Time - If not 0, this field specifies the number of
            hundredths (1/100) of a second to wait before continuing with the
            processing of the Data Stream. The clock starts ticking immediately
            after the graphic is rendered. This field may be used in
            conjunction with the User Input Flag field.

            viii) Transparency Index - The Transparency Index is such that when
            encountered, the corresponding pixel of the display device is not
            modified and processing goes on to the next pixel. The index is
            present if and only if the Transparency Flag is set to 1.

            ix) Block Terminator - This zero-length data block marks the end of







                                                                        17

            the Graphic Control Extension.

      d. Extensions and Scope. The scope of this Extension is the graphic
      rendering block that follows it; it is possible for other extensions to
      be present between this block and its target. This block can modify the
      Image Descriptor Block and the Plain Text Extension.

      e. Recommendations.

            i) Disposal Method - The mode Restore To Previous is intended to be
            used in small sections of the graphic; the use of this mode imposes
            severe demands on the decoder to store the section of the graphic
            that needs to be saved. For this reason, this mode should be used
            sparingly.  This mode is not intended to save an entire graphic or
            large areas of a graphic; when this is the case, the encoder should
            make every attempt to make the sections of the graphic to be
            restored be separate graphics in the data stream. In the case where
            a decoder is not capable of saving an area of a graphic marked as
            Restore To Previous, it is recommended that a decoder restore to
            the background color.

            ii) User Input Flag - When the flag is set, indicating that user
            input is expected, the decoder may sound the bell (0x07) to alert
            the user that input is being expected.  In the absence of a
            specified Delay Time, the decoder should wait for user input
            indefinitely.  It is recommended that the encoder not set the User
            Input Flag without a Delay Time specified.


24. Comment Extension.

      a. Description. The Comment Extension contains textual information which
      is not part of the actual graphics in the GIF Data Stream. It is suitable
      for including comments about the graphics, credits, descriptions or any
      other type of non-control and non-graphic data.  The Comment Extension
      may be ignored by the decoder, or it may be saved for later processing;
      under no circumstances should a Comment Extension disrupt or interfere
      with the processing of the Data Stream.

      This block is OPTIONAL; any number of them may appear in the Data Stream.

      b. Required Version.  89a.






















                                                                        18


      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Extension Introducer          Byte
     +---------------+
  1  |               |       Comment Label                 Byte
     +---------------+

     +===============+
     |               |
  N  |               |       Comment Data                  Data Sub-blocks
     |               |
     +===============+

     +---------------+
  0  |               |       Block Terminator              Byte
     +---------------+

            i) Extension Introducer - Identifies the beginning of an extension
            block. This field contains the fixed value 0x21.

            ii) Comment Label - Identifies the block as a Comment Extension.
            This field contains the fixed value 0xFE.

            iii) Comment Data - Sequence of sub-blocks, each of size at most
            255 bytes and at least 1 byte, with the size in a byte preceding
            the data.  The end of the sequence is marked by the Block
            Terminator.

            iv) Block Terminator - This zero-length data block marks the end of
            the Comment Extension.

      d. Extensions and Scope. This block does not have scope. This block
      cannot be modified by any extension.

      e. Recommendations.

            i) Data - This block is intended for humans.  It should contain
            text using the 7-bit ASCII character set. This block should
            not be used to store control information for custom processing.

            ii) Position - This block may appear at any point in the Data
            Stream at which a block can begin; however, it is recommended that
            Comment Extensions do not interfere with Control or Data blocks;
            they should be located at the beginning or at the end of the Data
            Stream to the extent possible.


25. Plain Text Extension.

      a. Description. The Plain Text Extension contains textual data and the
      parameters necessary to render that data as a graphic, in a simple form.
      The textual data will be encoded with the 7-bit printable ASCII
      characters.  Text data are rendered using a grid of character cells








                                                                        19


      defined by the parameters in the block fields. Each character is rendered
      in an individual cell. The textual data in this block is to be rendered
      as mono-spaced characters, one character per cell, with a best fitting
      font and size. For further information, see the section on
      Recommendations below. The data characters are taken sequentially from
      the data portion of the block and rendered within a cell, starting with
      the upper left cell in the grid and proceeding from left to right and
      from top to bottom. Text data is rendered until the end of data is
      reached or the character grid is filled.  The Character Grid contains an
      integral number of cells; in the case that the cell dimensions do not
      allow for an integral number, fractional cells must be discarded; an
      encoder must be careful to specify the grid dimensions accurately so that
      this does not happen. This block requires a Global Color Table to be
      available; the colors used by this block reference the Global Color Table
      in the Stream if there is one, or the Global Color Table from a previous
      Stream, if one was saved. This block is a graphic rendering block,
      therefore it may be modified by a Graphic Control Extension.  This block
      is OPTIONAL; any number of them may appear in the Data Stream.

      b. Required Version.  89a.











































                                                                        20


      c. Syntax.

      7 6 5 4 3 2 1 0        Field Name                    Type
     +---------------+
  0  |               |       Extension Introducer          Byte
     +---------------+
  1  |               |       Plain Text Label              Byte
     +---------------+

     +---------------+
  0  |               |       Block Size                    Byte
     +---------------+
  1  |               |       Text Grid Left Position       Unsigned
     +-             -+
  2  |               |
     +---------------+
  3  |               |       Text Grid Top Position        Unsigned
     +-             -+
  4  |               |
     +---------------+
  5  |               |       Text Grid Width               Unsigned
     +-             -+
  6  |               |
     +---------------+
  7  |               |       Text Grid Height              Unsigned
     +-             -+
  8  |               |
     +---------------+
  9  |               |       Character Cell Width          Byte
     +---------------+
 10  |               |       Character Cell Height         Byte
     +---------------+
 11  |               |       Text Foreground Color Index   Byte
     +---------------+
 12  |               |       Text Background Color Index   Byte
     +---------------+

     +===============+
     |               |
  N  |               |       Plain Text Data               Data Sub-blocks
     |               |
     +===============+

     +---------------+
  0  |               |       Block Terminator              Byte
     +---------------+

            i) Extension Introducer - Identifies the beginning of an extension
            block. This field contains the fixed value 0x21.

            ii) Plain Text Label - Identifies the current block as a Plain Text
            Extension. This field contains the fixed value 0x01.

            iii) Block Size - Number of bytes in the extension, after the Block
            Size field and up to but not including the beginning of the data
            portion. This field contains the fixed value 12.







                                                                        21


            iv) Text Grid Left Position - Column number, in pixels, of the left
            edge of the text grid, with respect to the left edge of the Logical
            Screen.
```

