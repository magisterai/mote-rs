# Graphics Interchange Format — part 1

Source: [Graphics Interchange Format](https://www.w3.org/Graphics/GIF/spec-gif89a.txt) · 89a, 1990-07-31.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text




    Cover Sheet for the GIF89a Specification


    DEFERRED CLEAR CODE IN LZW COMPRESSION

    There has been confusion about where clear codes can be found in the
    data stream.  As the specification says, they may appear at anytime.  There
    is not a requirement to send a clear code when the string table is full.

    It is the encoder's decision as to when the table should be cleared.  When
    the table is full, the encoder can chose to use the table as is, making no
    changes to it until the encoder chooses to clear it.  The encoder during
    this time sends out codes that are of the maximum Code Size.

    As we can see from the above, when the decoder's table is full, it must
    not change the table until a clear code is received.  The Code Size is that
    of the maximum Code Size.  Processing other than this is done normally.

    Because of a large base of decoders that do not handle the decompression in
    this manner, we ask developers of GIF encoding software to NOT implement
    this feature until at least January 1991 and later if they see that their
    particular market is not ready for it.  This will give developers of GIF
    decoding software time to implement this feature and to get it into the
    hands of their clients before the decoders start "breaking" on the new
    GIF's.  It is not required that encoders change their software to take
    advantage of the deferred clear code, but it is for decoders.

    APPLICATION EXTENSION BLOCK - APPLICATION IDENTIFIER

    There will be a Courtesy Directory file located on CompuServe in the PICS
    forum.  This directory will contain Application Identifiers for Application
    Extension Blocks that have been used by developers of GIF applications.
    This file is intended to help keep developers that wish to create
    Application Extension Blocks from using the same Application Identifiers.
    This is not an official directory; it is for voluntary participation only
    and does not guarantee that someone will not use the same identifier.

    E-Mail can be sent to Larry Wood (forum manager of PICS) indicating the
    request for inclusion in this file with an identifier.









































                        GRAPHICS INTERCHANGE FORMAT(sm)

                                  Version 89a

                            (c)1987,1988,1989,1990

                                   Copyright
                            CompuServe Incorporated
                                Columbus, Ohio






























CompuServe Incorporated                           Graphics Interchange Format
Document Date : 31 July 1990                            Programming Reference










                               Table of Contents

Disclaimer.................................................................  1

Foreword...................................................................  1

Licensing..................................................................  1

About the Document.........................................................  2

General Description........................................................  2

Version Numbers............................................................  2

The Encoder................................................................  3

The Decoder................................................................  3

Compliance.................................................................  3

About Recommendations......................................................  4

About Color Tables.........................................................  4

Blocks, Extensions and Scope...............................................  4

Block Sizes................................................................  5

Using GIF as an embedded protocol..........................................  5

Data Sub-blocks............................................................  5

Block Terminator...........................................................  6

Header.....................................................................  7

Logical Screen Descriptor..................................................  8

Global Color Table......................................................... 10

Image Descriptor........................................................... 11

Local Color Table.......................................................... 13

Table Based Image Data..................................................... 14

Graphic Control Extension.................................................. 15

Comment Extension.......................................................... 17

Plain Text Extension....................................................... 18

Application Extension...................................................... 21

Trailer.................................................................... 23











Quick Reference Table...................................................... 24

GIF Grammar................................................................ 25

Glossary................................................................... 27

Conventions................................................................ 28

Interlaced Images.......................................................... 29

Variable-Length-Code LZW Compression....................................... 30

On-line Capabilities Dialogue.............................................. 33


















































                                                                         1


1. Disclaimer.

The information provided herein is subject to change without notice. In no
event will CompuServe Incorporated be liable for damages, including any loss of
revenue, loss of profits or other incidental or consequential damages arising
out of the use or inability to use the information; CompuServe Incorporated
makes no claim as to the suitability of the information.


2. Foreword.

This document defines the Graphics Interchange Format(sm). The specification
given here defines version 89a, which is an extension of version 87a.

The Graphics Interchange Format(sm) as specified here should be considered
complete; any deviation from it should be considered invalid, including but not
limited to, the use of reserved or undefined fields within control or data
blocks, the inclusion of extraneous data within or between blocks, the use of
methods or algorithms not specifically listed as part of the format, etc. In
general, any and all deviations, extensions or modifications not specified in
this document should be considered to be in violation of the format and should
be avoided.


3. Licensing.

The Graphics Interchange Format(c) is the copyright property of CompuServe
Incorporated. Only CompuServe Incorporated is authorized to define, redefine,
enhance, alter, modify or change in any way the definition of the format.

CompuServe Incorporated hereby grants a limited, non-exclusive, royalty-free
license for the use of the Graphics Interchange Format(sm) in computer
software; computer software utilizing GIF(sm) must acknowledge ownership of the
Graphics Interchange Format and its Service Mark by CompuServe Incorporated, in
User and Technical Documentation. Computer software utilizing GIF, which is
distributed or may be distributed without User or Technical Documentation must
display to the screen or printer a message acknowledging ownership of the
Graphics Interchange Format and the Service Mark by CompuServe Incorporated; in
this case, the acknowledgement may be displayed in an opening screen or leading
banner, or a closing screen or trailing banner. A message such as the following
may be used:

      "The Graphics Interchange Format(c) is the Copyright property of
      CompuServe Incorporated. GIF(sm) is a Service Mark property of
      CompuServe Incorporated."

For further information, please contact :

      CompuServe Incorporated
      Graphics Technology Department
      5000 Arlington Center Boulevard
      Columbus, Ohio  43220
      U. S. A.

CompuServe Incorporated maintains a mailing list with all those individuals and
organizations who wish to receive copies of this document when it is corrected







                                                                         2


or revised. This service is offered free of charge; please provide us with your
mailing address.


4. About the Document.

This document describes in detail the definition of the Graphics Interchange
Format.  This document is intended as a programming reference; it is
recommended that the entire document be read carefully before programming,
because of the interdependence of the various parts. There is an individual
section for each of the Format blocks. Within each section, the sub-section
labeled Required Version refers to the version number that an encoder will have
to use if the corresponding block is used in the Data Stream. Within each
section, a diagram describes the individual fields in the block; the diagrams
are drawn vertically; top bytes in the diagram appear first in the Data Stream.
Bits within a byte are drawn most significant on the left end.  Multi-byte
numeric fields are ordered Least Significant Byte first. Numeric constants are
represented as Hexadecimal numbers, preceded by "0x".  Bit fields within a byte
are described in order from most significant bits to least significant bits.


5. General Description.

The Graphics Interchange Format(sm) defines a protocol intended for the on-line
transmission and interchange of raster graphic data in a way that is
independent of the hardware used in their creation or display.

The Graphics Interchange Format is defined in terms of blocks and sub-blocks
which contain relevant parameters and data used in the reproduction of a
graphic. A GIF Data Stream is a sequence of protocol blocks and sub-blocks
representing a collection of graphics. In general, the graphics in a Data
Stream are assumed to be related to some degree, and to share some control
information; it is recommended that encoders attempt to group together related
graphics in order to minimize hardware changes during processing and to
minimize control information overhead. For the same reason, unrelated graphics
or graphics which require resetting hardware parameters should be encoded
separately to the extent possible.

A Data Stream may originate locally, as when read from a file, or it may
originate remotely, as when transmitted over a data communications line. The
Format is defined with the assumption that an error-free Transport Level
Protocol is used for communications; the Format makes no provisions for
error-detection and error-correction.

The GIF Data Stream must be interpreted in context, that is, the application
program must rely on information external to the Data Stream to invoke the
decoder process.


6. Version Numbers.

The version number in the Header of a Data Stream is intended to identify the
minimum set of capabilities required of a decoder in order to fully process the
Data Stream.  An encoder should use the earliest possible version number that
includes all the blocks used in the Data Stream. Within each block section in
this document, there is an entry labeled Required Version which specifies the







                                                                         3


earliest version number that includes the corresponding block.  The encoder
should make every attempt to use the earliest version number covering all the
blocks in the Data Stream; the unnecessary use of later version numbers will
hinder processing by some decoders.


7. The Encoder.

The Encoder is the program used to create a GIF Data Stream. From raster data
and other information, the encoder produces the necessary control and data
blocks needed for reproducing the original graphics.

The encoder has the following primary responsibilities.

            - Include in the Data Stream all the necessary information to
            reproduce  the graphics.

            - Insure that a Data Stream is labeled with the earliest possible
            Version Number that will cover the definition of all the blocks in
            it; this is to ensure that the largest number of decoders can
            process the Data Stream.

            - Ensure encoding of the graphics in such a way that the decoding
            process is optimized. Avoid redundant information as much as
            possible.

            - To the extent possible, avoid grouping graphics which might
            require resetting hardware parameters during the decoding process.

            - Set to zero (off) each of the bits of each and every field
            designated as reserved. Note that some fields in the Logical Screen
            Descriptor and the Image Descriptor were reserved under Version
            87a, but are used under version 89a.


8. The Decoder.

The Decoder is the program used to process a GIF Data Stream. It processes the
Data Stream sequentially, parsing the various blocks and sub-blocks, using the
control information to set hardware and process parameters and interpreting the
data to render the graphics.

The decoder has the following primary responsibilities.

            - Process each graphic in the Data Stream in sequence, without
            delays other than those specified in the control information.

            - Set its hardware parameters to fit, as closely as possible, the
            control information contained in the Data Stream.


9. Compliance.

An encoder or a decoder is said to comply with a given version of the Graphics
Interchange Format if and only if it fully conforms with and correctly
implements the definition of the standard associated with that version.  An







                                                                         4

```

