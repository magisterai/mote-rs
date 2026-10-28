# 11.3.4.5 eXIf Exchangeable Image File (Exif) Profile

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](019-11-3-3-4-itxt-international-textual-data.md) · [Next](021-12-png-encoders.md)

<a id="ref-for-dfn-composited-5"></a>

Each frequency value is proportional to the fraction of the pixels in the image for which that palette entry is the
closest match in RGBA space, before the image has been [composited](002-2-scope.md#dfn-composited) against any background. The exact scale factor
is chosen by the PNG encoder; it is recommended that the resulting range of individual values reasonably fills the range
0 to 65535. A PNG encoder may artificially inflate the frequencies for colors considered to be "important", for example
the colors used in a logo or the facial features of a portrait. Zero is a valid frequency meaning that the color is
"least important" or that it is rarely, if ever, used. When all the frequencies are zero, they are meaningless, that is
to say, nothing may be inferred about the actual frequencies with which the colors appear in the PNG image.

Multiple sPLT chunks are permitted, but each shall have a different palette name.

<a id="eXIf"></a>

<a id="x11-3-4-5-exif-exchangeable-image-file-exif-profile"></a>

##### 11.3.4.5 eXIf Exchangeable Image File (Exif) Profile

The four-byte chunk type field contains the hexadecimal values



```
65 58 49 66
```




The data segment of the eXIf chunk contains an Exif profile in the format specified in
"4.7.2 Interoperability Structure of APP1 in Compressed Data" of [[CIPA-DC-008](044-g-references.md#bib-cipa-dc-008 "Exchangeable image file format for digital still cameras: Exif Version 2.32")] except that the JPEG APP1 marker,
length, and the "Exif ID code" described in 4.7.2(C), i.e., "Exif", NULL, and padding byte, are not included.

The
eXIf chunk size is constrained only by the maximum of 2<sup>31</sup>-1 bytes imposed by the
PNG specification. Only one eXIf chunk is allowed in a PNG datastream.

<a id="ref-for-dfn-image-data-13"></a>

<a id="ref-for-dfn-image-data-14"></a>

The eXIf chunk contains metadata concerning the original [image data](002-2-scope.md#dfn-image-data). If the image
has been edited subsequent to creation of the Exif profile, this data might no longer apply to the PNG [image data](002-2-scope.md#dfn-image-data).
It is recommended that unless a decoder has independent knowledge of the validity of the Exif data, the data should be
considered to be of historical value only. It is beyond the scope of this specification to resolve potential conflicts
between data in the eXIf chunk and in other PNG chunks.

<a id="exif-general-recommendations"></a>

<a id="x11-3-4-5-1-exif-general-recommendations"></a>

###### 11.3.4.5.1 eXIf General Recommendations

While the PNG specification allows the chunk size to be as large as 2<sup>31</sup>-1 bytes, application authors
should be aware that, if the Exif profile is going to be written to a JPEG [[JPEG](044-g-references.md#bib-jpeg "JPEG File Interchange Format")] datastream, the total length of the
eXIf chunk data may need to be adjusted to not exceed 2<sup>16</sup>-9 bytes, so it can fit
into a JPEG APP1 marker (Exif) segment.

<a id="exif-recommendations-for-decoders"></a>

<a id="x11-3-4-5-2-exif-recommendations-for-decoders"></a>

###### 11.3.4.5.2 eXIf Recommendations for Decoders

The first two bytes of data are either "II" for little-endian (Intel) or "MM" for big-endian (Motorola) byte order.
Decoders should check the first four bytes to ensure that they have the following hexadecimal values:



```
49 49 2A 00 (ASCII "II", 16-bit little-endian integer 42)
```




or



```
4D 4D 00 2A (ASCII "MM", 16-bit big-endian integer 42)
```




All other values are reserved for possible future definition.

<a id="exif-recommendations-for-encoders"></a>

<a id="x11-3-4-5-3-exif-recommendations-for-encoders"></a>

###### 11.3.4.5.3 eXIf Recommendations for Encoders

Image editing applications should consider Paragraph E.3 of the Exif Specification [[CIPA-DC-008](044-g-references.md#bib-cipa-dc-008 "Exchangeable image file format for digital still cameras: Exif Version 2.32")], which discusses
requirements for updating Exif data when the image is changed. Encoders should follow those requirements, but decoders
should not assume that it has been accomplished.

While encoders may choose to update them, there is no expectation that any thumbnails present in the Exif profile
have (or have not) been updated if the main image was changed.

<a id="11timestampinfo"></a>

<a id="x11-3-5-time-stamp-information"></a>

#### 11.3.5 Time stamp information

<a id="11tIME"></a>

<a id="x11-3-5-1-time-image-last-modification-time"></a>

##### 11.3.5.1 tIME Image last-modification time

The four-byte chunk type field contains the hexadecimal values



```
74 49 4D 45
```




The tIME chunk gives the time of the last image modification (**not** the time
of initial image creation). It contains:

<a id="tIME-structure"></a>

[Table 26](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#tIME-structure) 
tIME chunk contents

| Name | Size |
| --- | --- |
| Year | 2 bytes (complete; for example, 1995, **not** 95) |
| Month | 1 byte (1-12) |
| Day | 1 byte (1-31) |
| Hour | 1 byte (0-23) |
| Minute | 1 byte (0-59) |
| Second | 1 byte (0-60) (to allow for leap seconds) |

Universal Time (UTC) should be specified rather than local time.

<a id="ref-for-dfn-image-data-15"></a>

The tIME chunk is intended for use as an automatically-applied time stamp that is updated
whenever the [image data](002-2-scope.md#dfn-image-data) are changed.

<a id="animation-information"></a>

<a id="x11-3-6-animation-information"></a>

#### 11.3.6 Animation information

<a id="acTL-chunk"></a>

<a id="x11-3-6-1-actl-animation-control-chunk"></a>

##### 11.3.6.1 acTL Animation Control Chunk

The four-byte chunk type field contains the hexadecimal values



```
61 63 54 4C
```




The acTL chunk declares that this is an animated PNG image, gives the number of frames, and
the number of times to loop. It contains:

<a id="acTL-structure"></a>

|  |  |
| --- | --- |
| `num_frames` | 4 bytes |
| `num_plays` | 4 bytes |

<a id="ref-for-dfn-png-four-byte-unsigned-integer-10"></a>

Each value is encoded as a [PNG four-byte unsigned integer](002-2-scope.md#dfn-png-four-byte-unsigned-integer).

`num_frames` indicates the total number of frames in the animation. This must equal the number of [fcTL](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-chunk) chunks. 0 is not a valid value. 1 is a valid value, for a single-frame PNG. If this value
does not equal the actual number of frames it should be treated as an error.

`num_plays` indicates the number of times that this animation should play; if it is 0, the animation should play
indefinitely. If nonzero, the animation should come to rest on the final frame at the end of the last play.

The acTL chunk must appear before the first [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunk
within a valid PNG stream.

<a id="issue-container-generatedID-19"></a>

<a id="h-note-19"></a>

Note

For Web compatibility, due to the long time between the development and deployment of this chunk and it's
incorporation into the PNG specification, this chunk name is exceptionally defined as if it were a private chunk.

<a id="fcTL-chunk"></a>

<a id="x11-3-6-2-fctl-frame-control-chunk"></a>

##### 11.3.6.2 fcTL Frame Control Chunk

The four-byte chunk type field contains the hexadecimal values



```
66 63 54 4C
```




The fcTL chunk defines the dimensions, position, delay and disposal of an individual frame.
Exactly one fcTL chunk chunk is required for each frame. It contains:

<a id="fcTL-structure"></a>

[Table 27](020-11-3-4-5-exif-exchangeable-image-file-exif-profile.md#fcTL-structure) 
fcTL chunk contents

| Name | Size |
| --- | --- |
| `sequence_number` | 4 bytes |
| `width` | 4 bytes |
| `height` | 4 bytes |
| `x_offset` | 4 bytes |
| `y_offset` | 4 bytes |
| `delay_num` | 2 bytes |
| `delay_den` | 2 bytes |
| `dispose_op` | 1 byte |
| `blend_op` | 1 byte |

<a id="ref-for-dfn-png-four-byte-unsigned-integer-11"></a>

`sequence_number` defines the [sequence number](007-4-9-apng-frame-based-animation.md#4Concepts.APNGSequence) of the animation chunk, starting from 0. It is encoded as a [PNG
four-byte unsigned integer](002-2-scope.md#dfn-png-four-byte-unsigned-integer).

<a id="ref-for-dfn-png-four-byte-unsigned-integer-12"></a>

`width` and `height` define the width and height of the following frame. They are encoded as [PNG four-byte unsigned
integers](002-2-scope.md#dfn-png-four-byte-unsigned-integer). They must be greater than zero.

<a id="ref-for-dfn-png-four-byte-unsigned-integer-13"></a>

`x_offset` and `y_offset` define the x and y position of the following frame. They are encoded as [PNG four-byte
unsigned integers](002-2-scope.md#dfn-png-four-byte-unsigned-integer). Zero is a valid value.

The frame must be rendered within the region defined by `x_offset`, `y_offset`, `width`, and `height`. This region may
not fall outside of the default image; thus `x_offset` plus `width` must not be greater than the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) width; similarly `y_offset` plus `height` must not be greater than the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) height.

`delay_num` and `delay_den` define the numerator and denominator of the delay fraction; indicating the time to display
the current frame, in seconds. If the denominator is 0, it is to be treated as if it were 100 (that is, `delay_num` then
specifies 1/100ths of a second). If the the value of the numerator is 0 the decoder should render the next frame as
quickly as possible, though viewers may impose a reasonable lower bound. They are encoded as two-byte unsigned
integers.

Frame timings should be independent of the time required for decoding and display of each frame, so that animations
will run at the same rate regardless of the performance of the decoder implementation.

`dispose_op` defines the type of frame area disposal to be done after rendering this frame; in other words, it
specifies how the output buffer should be changed at the end of the delay (before rendering the next frame). It is
encoded as a one-byte unsigned integer.

Valid values for `dispose_op` are:

<a id="dispose-op-values"></a>

|  |  |
| --- | --- |
| 0 | `APNG_DISPOSE_OP_NONE` |
| 1 | `APNG_DISPOSE_OP_BACKGROUND` |
| 2 | `APNG_DISPOSE_OP_PREVIOUS` |

`APNG_DISPOSE_OP_NONE`
:   no disposal is done on this frame before rendering the next; the contents of the output buffer are left as is.

`APNG_DISPOSE_OP_BACKGROUND`
:   the frame's region of the output buffer is to be cleared to fully transparent black before rendering the next
    frame.

`APNG_DISPOSE_OP_PREVIOUS`
:   the frame's region of the output buffer is to be reverted to the previous contents before rendering the next
    frame.

If the first fcTL chunk uses a `dispose_op` of `APNG_DISPOSE_OP_PREVIOUS` it should be
treated as `APNG_DISPOSE_OP_BACKGROUND`.

`blend_op` specifies whether the frame is to be alpha blended into the current output buffer content, or whether it
should completely replace its region in the output buffer. It is encoded as a one-byte unsigned integer.

Valid values for `blend_op` are:

<a id="blend-op-values"></a>

|  |  |
| --- | --- |
| 0 | `APNG_BLEND_OP_SOURCE` |
| 1 | `APNG_BLEND_OP_OVER` |

<a id="ref-for-dfn-composited-6"></a>

If `blend_op` is `APNG_BLEND_OP_SOURCE` all color components of the frame, including alpha, overwrite the current
contents of the frame's output buffer region. If `blend_op` is `APNG_BLEND_OP_OVER` the frame should be [composited](002-2-scope.md#dfn-composited)
onto the output buffer based on its alpha, using a simple OVER operation as described in [Alpha Channel Processing](031-13-16-alpha-channel-processing.md#13Alpha-channel-processing). Note that the second variation of the sample code is
applicable.

Note that for the first frame, the two blend modes are functionally equivalent due to the clearing of the output
buffer at the beginning of each play.

The fcTL chunk corresponding to the default image, if it exists, has these
restrictions:

- The `x_offset` and `y_offset` fields must be 0.
- The `width` and `height` fields must equal the corresponding fields from the [IHDR](014-10-3-other-uses-of-compression.md#11IHDR) chunk.

As noted earlier, the output buffer must be completely initialized to fully transparent black at the beginning of each
play. This is to ensure that each play of the animation will be identical. Decoders are free to avoid an explicit clear
step as long as the result is guaranteed to be identical. For example, if the default image is included in the animation,
and uses a `blend_op` of `APNG_BLEND_OP_SOURCE`, clearing is not necessary because the entire output buffer will be
overwritten.

<a id="issue-container-generatedID-20"></a>

<a id="h-note-20"></a>

Note

For Web compatibility, due to the long time between the development and deployment of this chunk and it's
incorporation into the PNG specification, this chunk name is exceptionally defined as if it were a private chunk.

<a id="fdAT-chunk"></a>

<a id="x11-3-6-3-fdat-frame-data-chunk"></a>

##### 11.3.6.3 fdAT Frame Data Chunk

The four-byte chunk type field contains the hexadecimal values



```
66 64 41 54
```




