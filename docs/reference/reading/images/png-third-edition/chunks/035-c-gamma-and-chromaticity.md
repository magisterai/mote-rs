# C. Gamma and chromaticity

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](034-a-internet-media-types.md) · [Next](036-e-online-resources.md)

<a id="C-GammaAppendix"></a>

<a id="gammachromaticity"></a>

## C. Gamma and chromaticity

*This section is non-normative.*

<a id="ref-for-dfn-gamma-value-28"></a>

<a id="ref-for-dfn-transfer-function-20"></a>

<a id="ref-for-dfn-gamma-value-29"></a>

A [gamma value](002-2-scope.md#dfn-gamma-value) is a numerical parameter used to describe approximations to certain non-linear [transfer
functions](002-2-scope.md#dfn-transfer-function) encountered in image capture and reproduction. The [gamma value](002-2-scope.md#dfn-gamma-value) is the exponent in a power law function.
For example the function:

`intensity = (voltage + constant)<sup>exponent</sup>`

which is used to model the non-linearity of CRT displays. It is often assumed, as in this International Standard,
that the constant is zero.

<a id="ref-for-dfn-transfer-function-21"></a>

For the purposes of this specification, it is convenient to consider five places in a general image pipeline at which
non-linear [transfer functions](002-2-scope.md#dfn-transfer-function) may occur and which may be modelled by power laws. The characteristic exponent associated
with each is given a specific name.

<a id="gamma-pipeline"></a>

<a id="ref-for-dfn-transfer-function-22"></a>

<a id="ref-for-dfn-transfer-function-23"></a>

<a id="ref-for-dfn-image-data-37"></a>

<a id="ref-for-dfn-transfer-function-24"></a>

<a id="ref-for-dfn-frame-buffer-13"></a>

|  |  |
| --- | --- |
| input\_exponent | the exponent of the image sensor. |
| encoding\_exponent | the exponent of any [transfer function](002-2-scope.md#dfn-transfer-function) performed by the process or device writing the datastream. |
| decoding\_exponent | the exponent of any [transfer function](002-2-scope.md#dfn-transfer-function) performed by the software reading the [image data](002-2-scope.md#dfn-image-data)stream. |
| LUT\_exponent | the exponent of the [transfer function](002-2-scope.md#dfn-transfer-function) applied between the [frame buffer](002-2-scope.md#dfn-frame-buffer) and the display device (typically this is applied by a Look Up Table). |
| output\_exponent | the exponent of the display device. For a CRT, this is typically a value close to 2.2. |

<a id="ref-for-dfn-transfer-function-25"></a>

It is convenient to define some additional entities that describe some composite [transfer functions](002-2-scope.md#dfn-transfer-function), or combinations
of stages.

<a id="gamma-composite"></a>

<a id="ref-for-dfn-transfer-function-26"></a>

<a id="ref-for-dfn-frame-buffer-14"></a>

|  |  |
| --- | --- |
| display\_exponent | exponent of the [transfer function](002-2-scope.md#dfn-transfer-function) applied between the [frame buffer](002-2-scope.md#dfn-frame-buffer) and the display surface of the display device.  `display_exponent = LUT_exponent * output_exponent` |
| gamma | exponent of the function mapping display output intensity to samples in the PNG datastream.  `gamma = 1.0 / (decoding_exponent * display_exponent)` |
| end\_to\_end\_exponent | the exponent of the function mapping image sensor input intensity to display output intensity. This is generally a value in the range 1.0 to 1.5. |

<a id="ref-for-dfn-gamma-value-30"></a>

The PNG [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk is used to record the [gamma value](002-2-scope.md#dfn-gamma-value). This information may be
used by decoders together with additional information about the display environment in order to achieve, or approximate, the
desired display output.

Additional information about this subject may be found [[GAMMA-FAQ](045-g-2-informative-references.md#bib-gamma-faq "Gamma FAQ")].

Additional information on the impact of color space on image encoding may be found in [[Kasson](045-g-2-informative-references.md#bib-kasson "An Analysis of Selected Computer Interchange Color Spaces")] and [[Hill](045-g-2-informative-references.md#bib-hill "Comparative analysis of the quantization of color spaces on the basis of the CIELAB color-difference formula")].

<a id="ref-for-dfn-chromaticity-5"></a>

Background information about [chromaticity](002-2-scope.md#dfn-chromaticity) and color spaces may be found in [[COLOR-FAQ](045-g-2-informative-references.md#bib-color-faq "Color FAQ")].

<a id="D-CRCAppendix"></a>

<a id="samplecrc"></a>

## D. Sample CRC implementation

The following sample code — which is informative — represents a practical implementation of the CRC (Cyclic
Redundancy Check) employed in PNG chunks. (See also ISO 3309 [[ISO-3309](044-g-references.md#bib-iso-3309 "ISO/IEC 3309:1993, Information Technology — Telecommunications and information exchange between systems — High-level data link control (HDLC) procedures — Frame structure.")] or ITU-T V.42 [[ITU-T-V.42](044-g-references.md#bib-itu-t-v.42 "ITU-T V.42, SERIES V: DATA COMMUNICATION OVER THE TELEPHONE NETWORK. Error-correcting procedures for DCEs using asynchronous-to-synchronous conversion")] for a formal
specification.)

The sample code is in the ISO C [[ISO\_9899](044-g-references.md#bib-iso_9899 "ISO/IEC 9899:2018 Information technology — Programming languages — C")] programming language. The hints in [Table 31](035-c-gamma-and-chromaticity.md#D-tabled1 "Hints for reading ISO C code") may help non-C
users to read the code more easily.

<a id="D-tabled1"></a>

[Table 31](035-c-gamma-and-chromaticity.md#D-tabled1) 
Hints for reading ISO C code

| Operator | Description |
| --- | --- |
| `&` | Bitwise AND operator. |
| `^` | Bitwise exclusive-OR operator. |
| `>>` | Bitwise right shift operator. When applied to an unsigned quantity, as here, right shift inserts zeroes at the left. |
| `!` | Logical NOT operator. |
| `++` | "`n++`" increments the variable n. In "for" loops, it is applied after the variable is tested. |
| `0xNNN` | `0x` introduces a hexadecimal (base 16) constant. Suffix `L` indicates a long value (at least 32 bits). |

---



```
/* Table of CRCs of all 8-bit messages. */
unsigned long crc_table[256];

/* Flag: has the table been computed? Initially false. */
int crc_table_computed = 0;

/* Make the table for a fast CRC. */
void make_crc_table(void)
{
  unsigned long c;
  int n, k;

  for (n = 0; n < 256; n++) {
    c = (unsigned long) n;
    for (k = 0; k < 8; k++) {
      if (c & 1)
        c = 0xedb88320L ^ (c >> 1);
      else
        c = c >> 1;
    }
    crc_table[n] = c;
  }
  crc_table_computed = 1;
}
```






```
/* Update a running CRC with the bytes buf[0..len-1]--the CRC
   should be initialized to all 1's, and the transmitted value
   is the 1's complement of the final running CRC (see the
   crc() routine below). */

unsigned long update_crc(unsigned long crc, unsigned char *buf,
                         int len)
{
  unsigned long c = crc;
  int n;

  if (!crc_table_computed)
    make_crc_table();
  for (n = 0; n < len; n++) {
    c = crc_table[(c ^ buf[n]) & 0xff] ^ (c >> 8);
  }
  return c;
}

/* Return the CRC of the bytes buf[0..len-1]. */
unsigned long crc(unsigned char *buf, int len)
{
  return update_crc(0xffffffffL, buf, len) ^ 0xffffffffL;
}
```




