# 11.3.2.8 cLLI Content Light Level Information

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](017-11-3-2-7-mdcv-mastering-display-color-volume.md) · [Next](019-11-3-3-4-itxt-international-textual-data.md)

<a id="mDCV-chunk-sdr-white-point-example"></a>

| Name | Actual values | Stored Decimal values | Stored Hexadecimal values |
| --- | --- | --- | --- |
| Illuminant D65 specified in [[SMPTE-RP-177](045-g-2-informative-references.md#bib-smpte-rp-177 "Derivation of Basic Television Color Equations")] | (0.3127, 0.3290) | { 15635, 16450 } | { 3D 13, 40 42 } |



<a id="example-11"></a>

[Example 11](018-11-3-2-8-clli-content-light-level-information.md#example-11)

Example mDCV chunk mastering display maximum
luminance for [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")]:

<a id="mDCV-chunk-sdr-max-luminance-example"></a>

<table class="numbered simple" id="mDCV-chunk-sdr-max-luminance-example">
<tbody><tr>
<th>Actual value</th>
<th>Stored Decimal values</th>
<th>Stored Hexadecimal values</th>
</tr>
<tr>
<td>80 cd/m<sup>2</sup></td>
<td>800000</td>
<td>00 0C 35 00</td>
</tr>
</tbody></table>



<a id="example-12"></a>

[Example 12](018-11-3-2-8-clli-content-light-level-information.md#example-12)

Example mDCV chunk mastering display minimum luminance for [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")]:

<a id="mDCV-chunk-sdr-min-luminance-example"></a>

<table class="numbered simple" id="mDCV-chunk-sdr-min-luminance-example">
<tbody><tr>
<th>Actual value</th>
<th>Stored Decimal values</th>
<th>Stored Hexadecimal values</th>
</tr>
<tr>
<td>0.05 cd/m<sup>2</sup></td>
<td>500</td>
<td>00 00 01 F4</td>
</tr>
</tbody></table>

<a id="cLLI-chunk"></a>

<a id="x11-3-2-8-clli-content-light-level-information"></a>

##### 11.3.2.8 cLLI Content Light Level Information

The four-byte chunk type field contains the hexadecimal values



```
63 4C 4C 49
```




<a id="ref-for-dfn-hdr-7"></a>

If present, the cLLI chunk identifies two characteristics of [HDR](002-2-scope.md#dfn-hdr) content:

The cLLI chunk adds static metadata which provides an opportunity
to optimize tone mapping of the associated content to a specific target display. This is
accomplished by tailoring the tone mapping of the content itself to the specific peak brightness
capabilities of the target display to prevent clipping. The method of tone-mapping optimization
is currently subjective.

MaxCLL (Maximum Content Light Level) uses a static metadata value to indicate the maximum light
level of any single pixel (in cd/m<sup>2</sup>, also known as nits) of the entire playback sequence.
There is often an algorithmic filter to eliminate false values occurring from processing or noise
that could adversely affect intended downstream tone mapping.

<a id="ref-for-dfn-frame-2"></a>

MaxFALL (Maximum Frame Average Light Level) uses a static metadata value to indicate the
maximum value of the [frame](002-2-scope.md#dfn-frame) average light level (in cd/m<sup>2</sup>, also known as nits)
of the entire playback sequence. MaxFALL is calculated by first averaging the decoded luminance
values of all the pixels in each frame, and then using the value for the frame with the highest value.

<a id="ref-for-dfn-png-four-byte-unsigned-integer-7"></a>

The MaxCLL and MaxFALL values are encoded as [PNG four-byte unsigned integers](002-2-scope.md#dfn-png-four-byte-unsigned-integer).

<a id="issue-container-generatedID-14"></a>

<a id="h-note-14"></a>

Note

[[CTA-861.3-A](045-g-2-informative-references.md#bib-cta-861.3-a "HDR Static Metadata Extensions (CTA-861.3-A)")] describes the method of calculation for generating the
cLLI values,
but does not specify any filtering.
[[HDR-Static-Meta](045-g-2-informative-references.md#bib-hdr-static-meta "On the Calculation and Usage of HDR Static Content Metadata")] describes an improved method which rejects extreme values from
statistical outliers, noise or ringing from resampling filters,
and is recommended for practical implementations.

<a id="issue-container-generatedID-15"></a>

<a id="h-note-15"></a>

Note

[[SMPTE-ST-2067-21](045-g-2-informative-references.md#bib-smpte-st-2067-21 "Interoperable Master Format — Application #2E")] Section 7.5 adds additional
information in Section 7.5 in the case where the cLLI values are unknown and have not been calculated.

<a id="issue-container-generatedID-16"></a>

<a id="h-note-16"></a>

Note

[Issue #319](https://github.com/w3c/png/issues/319) discusses tone-mapping behavior when
the cLLI chunk is present.

<a id="ref-for-dfn-frame-3"></a>

Each [frame](002-2-scope.md#dfn-frame) is analyzed.

A value of zero for either MaxCLL or MaxFALL means that the value is unknown or not currently calculable.

<a id="issue-container-generatedID-17"></a>

<a id="h-note-17"></a>

Note

An example where this will not be calculable is when creating a live animated PNG stream, when not all frames will be available to compute the values until the stream ends. The encoder may wish to use the value zero initially and replace this with the calculated value when the stream ends.

The following specifies the syntax of the cLLI chunk:

<a id="cLLI-chunk-syntax"></a>

<table class="numbered simple" id="cLLI-chunk-syntax">
<caption><a class="self-link" href="018-11-3-2-8-clli-content-light-level-information.md#cLLI-chunk-syntax">Table <bdi class="tableno">20</bdi></a> <span class="table-title">cLLI chunk components</span></caption>
<tbody><tr>
<th>Name</th>
<th>Size</th>
<th>Divisor value</th>
</tr>
<tr>
<td>Maximum Content Light Level (MaxCLL)</td>
<td>4 bytes</td>
<td>0.0001 cd/m<sup>2</sup></td>
</tr>
<tr>
<td>Maximum Frame-Average Light Level (MaxFALL)</td>
<td>4 bytes</td>
<td>0.0001 cd/m<sup>2</sup></td>
</tr>
</tbody></table>

<a id="example-13"></a>

[Example 13](018-11-3-2-8-clli-content-light-level-information.md#example-13)

Example cLLI chunk Maximum Content Light Level:

<a id="cLLI-chunk-max-content-light-level-example"></a>

<table class="numbered simple" id="cLLI-chunk-max-content-light-level-example">
<tbody><tr>
<th>Actual value</th>
<th>Stored Decimal values</th>
<th>Stored Hexadecimal values</th>
</tr>
<tr>
<td>1000 cd/m<sup>2</sup></td>
<td>10000000</td>
<td>00 98 96 80</td>
</tr>
</tbody></table>



<a id="example-14"></a>

[Example 14](018-11-3-2-8-clli-content-light-level-information.md#example-14)

Example cLLI chunk Maximum Frame-Average Light Level:

<a id="cLLI-chunk-max-frame-avg-luminance-example"></a>

<table class="numbered simple" id="cLLI-chunk-max-frame-avg-luminance-example">
<tbody><tr>
<th>Actual value</th>
<th>Stored Decimal values</th>
<th>Stored Hexadecimal values</th>
</tr>
<tr>
<td>250 cd/m<sup>2</sup></td>
<td>2500000</td>
<td>00 26 25 A0</td>
</tr>
</tbody></table>

<a id="11textinfo"></a>

<a id="x11-3-3-textual-information"></a>

#### 11.3.3 Textual information

<a id="11textIntro"></a>

<a id="introduction-7"></a>

### Introduction

PNG provides the [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt), [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt), and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks for storing text strings associated with the image, such as an image description
or copyright notice. Keywords are used to indicate what each text string represents. Any number of such text chunks may
appear, and more than one with the same keyword is permitted.

<a id="11keywords"></a>

<a id="x11-3-3-1-keywords-and-text-strings"></a>

##### 11.3.3.1 Keywords and text strings

The following keywords are predefined and should be used where appropriate.

<a id="keywords-predefined"></a>

[Table 21](018-11-3-2-8-clli-content-light-level-information.md#keywords-predefined) 
Predefined keywords

| Keyword value | Description |
| --- | --- |
| Title | Short (one line) title or caption for image |
| Author | Name of image's creator |
| Description | Description of image (possibly long) |
| Copyright | Copyright notice |
| Creation Time | Time of original image creation |
| Software | Software used to create the image |
| Disclaimer | Legal disclaimer |
| Warning | Warning of nature of content |
| Source | Device used to create the image |
| Comment | Miscellaneous comment |
| XML:com.adobe.xmp | Extensible Metadata Platform (XMP) information, formatted as required by the XMP specification [[XMP](044-g-references.md#bib-xmp "Extensible metadata platform (XMP) specification -- Part 1")]. The use of [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt), with Compression Flag set to 0, and both Language Tag and Translated Keyword set to the null string, are recommended for XMP compliance. |
| Collection | Name of a collection to which the image belongs. An image may belong to one or more collections, each named by a separate text chunk. |

Other keywords *MAY* be defined by any application for private or general interest.

Keywords *SHOULD* be .

- reasonably self-explanatory, since the aim is to let other human users understand what the chunk contains; and
- chosen to minimize the chance that the same keyword is used for incompatible purposes by different
  applications.

Keywords of general interest *SHOULD* be listed in [[PNG-EXTENSIONS](044-g-references.md#bib-png-extensions "Extensions to the PNG Third Edition Specification, Version 1.6.0")].

Keywords shall contain only printable Latin-1 [[ISO\_8859-1](044-g-references.md#bib-iso_8859-1 "ISO/IEC 8859-1:1998, Information technology — 8-bit single-byte coded graphic character sets — Part 1: Latin alphabet No. 1.")] characters and spaces; that is, only code points 0x20-7E
and 0xA1-FF are allowed. To reduce the chances for human misreading of a keyword, leading spaces, trailing spaces, and
consecutive spaces are not permitted in keywords, nor is U+00A0 NON-BREAKING SPACE since it is visually indistinguishable
from an ordinary space.

Keywords shall be spelled exactly as registered, so that decoders can use simple literal comparisons when looking for
particular keywords. In particular, keywords are considered case-sensitive. Keywords are restricted to 1 to 79 bytes in
length.

For the Creation Time keyword, the date format *SHOULD* be in the RFC 3339 [[rfc3339](044-g-references.md#bib-rfc3339 "Date and Time on the Internet: Timestamps")] date-time format or in the date format defined in section 5.2.14 of RFC 1123 [[rfc1123](044-g-references.md#bib-rfc1123 "Requirements for Internet Hosts - Application and Support")]. The RFC3339 date-time format is preferred. The actual format of this field is undefined.

<a id="ref-for-dfn-zlib-15"></a>

<a id="ref-for-dfn-deflate-13"></a>

The [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunk uses the UTF-8 encoding [[rfc3629](044-g-references.md#bib-rfc3629 "UTF-8, a transformation format of ISO 10646")] and can be used to convey
characters in any language. There is an option to compress text strings in the [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt)
chunk. [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) is recommended for all text strings, as it supports Unicode. There are
also [tEXt](018-11-3-2-8-clli-content-light-level-information.md#11tEXt) and [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) chunks, whose content is
restricted to the printable Latin-1 character set plus U+000A LINE FEED (LF). Text strings in [zTXt](018-11-3-2-8-clli-content-light-level-information.md#11zTXt) are compressed into [zlib](002-2-scope.md#dfn-zlib) datastreams using [deflate](002-2-scope.md#dfn-deflate) compression (see [10.3 Other uses of compression](014-10-3-other-uses-of-compression.md#10CompressionOtherUses)).

<a id="11tEXt"></a>

<a id="x11-3-3-2-text-textual-data"></a>

##### 11.3.3.2 tEXt Textual data

The four-byte chunk type field contains the hexadecimal values



```
74 45 58 74
```




Each tEXt chunk contains a keyword and a text string, in the format:

<a id="tEXt-structure"></a>

|  |  |
| --- | --- |
| Keyword | 1-79 bytes (character string) |
| Null separator | 1 byte (null character) |
| Text string | 0 or more bytes (character string) |

The keyword and text string are separated by a zero byte (null character). Neither the keyword nor the text string may
contain a null character. The text string is **not** null-terminated (the length of the chunk defines the
ending). The text string may be of any length from zero bytes up to the maximum permissible chunk size less the length of
the keyword and null character separator.

The keyword indicates the type of information represented by the text string as described in [11.3.3.1 Keywords and text strings](018-11-3-2-8-clli-content-light-level-information.md#11keywords).

Text is interpreted according to the Latin-1 character set [[ISO\_8859-1](044-g-references.md#bib-iso_8859-1 "ISO/IEC 8859-1:1998, Information technology — 8-bit single-byte coded graphic character sets — Part 1: Latin alphabet No. 1.")]. The text string may contain any Latin-1
character. Newlines in the text string should be represented by a single linefeed character (decimal 10). Characters
other than those defined in Latin-1 plus the linefeed character have no defined meaning in tEXt chunks. Text containing characters outside the repertoire of ISO/IEC 8859-1 should be encoded using
the [iTXt](019-11-3-3-4-itxt-international-textual-data.md#11iTXt) chunk.

<a id="11zTXt"></a>

<a id="x11-3-3-3-ztxt-compressed-textual-data"></a>

##### 11.3.3.3 zTXt Compressed textual data

The four-byte chunk type field contains the hexadecimal values



```
7A 54 58 74
```




