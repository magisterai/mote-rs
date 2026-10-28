# 11.3.2.7 mDCV Mastering Display Color Volume

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](016-11-3-2-5-srgb-standard-rgb-color-space.md) · [Next](018-11-3-2-8-clli-content-light-level-information.md)

<a id="ref-for-dfn-transfer-function-11"></a>

[transfer function](002-2-scope.md#dfn-transfer-function) defined by [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")]:



```
0C 0D 00 01
```




(Four 1-byte unsigned integers, in hexadecimal)

<a id="mDCV-chunk"></a>

<a id="x11-3-2-7-mdcv-mastering-display-color-volume"></a>

##### 11.3.2.7 mDCV Mastering Display Color Volume

The four-byte chunk type field contains the hexadecimal values



```
6D 44 43 56
```




If present, the mDCV chunk characterizes
the Mastering Display Color Volume (mDCV) used at the point of content creation,
as specified in [[SMPTE-ST-2086](044-g-references.md#bib-smpte-st-2086 "Mastering Display Color Volume Metadata Supporting High Luminance and Wide Color Gamut Images")]. The mDCV chunk provides informative static metadata which
allows a target (consumer) display to potentially optimize its tone mapping decisions
on a comparison of its inherent capabilities versus the original mastering display's capabilites.

<a id="ref-for-dfn-pq-3"></a>

<a id="ref-for-dfn-hlg-3"></a>

<a id="ref-for-dfn-sdr-3"></a>

mDCV is typically used with the [PQ](002-2-scope.md#dfn-pq) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] transfer function
and additional [cLLI](018-11-3-2-8-clli-content-light-level-information.md#cLLI-chunk) metadata and is commonly then called
[[HDR10](045-g-2-informative-references.md#bib-hdr10 "HDR10 Media Profile")] (PQ with ST 2086 static metadata, MaxFALL and MaxCLL).
The mDCV chunk may also be included with [HLG](002-2-scope.md#dfn-hlg) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] and
[SDR](002-2-scope.md#dfn-sdr) image formats (for example [[ITU-R-BT.709](044-g-references.md#bib-itu-r-bt.709 "ITU-R BT.709, SERIES BT: BROADCASTING SERVICE (TELEVISION). Parameter values for the HDTV standards for production and international programme exchange")]).

Since mDCV was originally created as supplemental static metadata meant to
optimize the tone-mapping of images on a video display target, a cICP chunk
must accompany the use of mDCV in order to establish the basic characteristics
of the image content. Color Primaries and White Point characteristics
can be derived from cICP chunk formats.
Specific examples of its most common use-cases for images using
both HDR [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] and SDR [[ITU-R-BT.709](044-g-references.md#bib-itu-r-bt.709 "ITU-R BT.709, SERIES BT: BROADCASTING SERVICE (TELEVISION). Parameter values for the HDTV standards for production and international programme exchange")] are available in
[[ITU-T-Series-H-Supplement-19](044-g-references.md#bib-itu-t-series-h-supplement-19 "Series H: Audio Visual and Multimedia Systems - Usage of video signal type code points")]. The basic (cICP) characteristics plus the supplemental
(mDCV) static metadata may provide valuable information to optimize
tone-mapping decisions.

<a id="issue-container-generatedID-12"></a>

<a id="h-note-12"></a>

Note

[Issue #319](https://github.com/w3c/png/issues/319) discusses tone-mapping behavior when
the mDCV chunk is present.

<a id="ref-for-dfn-sdr-4"></a>

<a id="ref-for-dfn-sdr-5"></a>

For [SDR](002-2-scope.md#dfn-sdr) images, if mDCV display min/max luminance are unknown, the default
characteristics can be derived from the values in [[ITU-T-Series-H-Supplement-19](044-g-references.md#bib-itu-t-series-h-supplement-19 "Series H: Audio Visual and Multimedia Systems - Usage of video signal type code points")] Table 11 or from the relevant [SDR](002-2-scope.md#dfn-sdr) specification.
At present, there is no published, standardized method for translating an SDR image signal from its default viewing
condition (display luminance and ambient illumination) to that signalled in the mDCV chunk.

<a id="issue-container-generatedID-13"></a>

<a id="h-note-13"></a>

Note

The 

<a id="ref-for-dfn-hlg-4"></a>

[HLG](002-2-scope.md#dfn-hlg) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] image format does have published methods for translating the image for both changes in display luminance (within [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")]) and
ambient illumination (within the accompanying report [[ITU-R-BT.2390](045-g-2-informative-references.md#bib-itu-r-bt.2390 "High dynamic range television for production and international programme exchange")]). This may be used with SDR images.

The following specifies the syntax of the mDCV chunk:

<a id="mDCV-chunk-syntax"></a>

<table class="numbered simple" id="mDCV-chunk-syntax">
<caption><a class="self-link" href="017-11-3-2-7-mdcv-mastering-display-color-volume.md#mDCV-chunk-syntax">Table <bdi class="tableno">19</bdi></a> <span class="table-title">
              mDCV chunk components
            </span></caption>
<tbody><tr>
<th>Name</th>
<th>Size</th>
<th>Divisor value</th>
</tr>
<tr>
<td>Mastering display color primary chromaticities (CIE 1931 <i>x,y</i> of R,G,B )</td>
<td>12 bytes</td>
<td>0.00002</td>
</tr>
<tr>
<td>Mastering display white point chromaticity (CIE 1931 <i>x,y</i>)</td>
<td>4 bytes</td>
<td>0.00002</td>
</tr>
<tr>
<td>Mastering display maximum luminance (measured in cd/m<sup>2</sup>)</td>
<td>4 bytes</td>
<td>0.0001</td>
</tr>
<tr>
<td>Mastering display minimum luminance (measured in cd/m<sup>2</sup>)</td>
<td>4 bytes</td>
<td>0.0001</td>
</tr>
</tbody></table>

<a id="ref-for-dfn-png-two-byte-unsigned-integer-1"></a>

The color primaries are encoded as
three pairs of [PNG two-byte unsigned integer](002-2-scope.md#dfn-png-two-byte-unsigned-integer)s,
in the order *x* and then *y*,
each representing the x or y primary chromaticity value divided by the divisor value.
They are ordered starting with the primary with the largest x chromaticity,
followed by the primary with the largest y chromaticity,
followed by the remaining primary.
For RGB color spaces, this corresponds to the order R, G, B.

<a id="ref-for-dfn-png-two-byte-unsigned-integer-2"></a>

The white point is encoded as
a pair of [PNG two-byte unsigned integer](002-2-scope.md#dfn-png-two-byte-unsigned-integer)s,
in the order *x* and then *y*,
each representing the x or y whie chromaticity value divided by the divisor value.

<a id="ref-for-dfn-png-four-byte-unsigned-integer-6"></a>

The maximum and minimum luminance values are encoded as
[PNG four-byte unsigned integer](002-2-scope.md#dfn-png-four-byte-unsigned-integer)s,
representing the absolute luminance value in cd/m<sup>2</sup>
divided by the divisor value.

The divisor maps from actual value to stored value. For example, the unitless divisor of 0.00002 for the primaries and
white point would store the chromaticity (0.6800, 0.3200) as {34000, 16000}.

The mDCV chunk *MUST* come before the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) and [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks.

<a id="ref-for-dfn-hdr-3"></a>

Below are mDCV examples for [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")] [HDR](002-2-scope.md#dfn-hdr).

<a id="example-5"></a>

[Example 5](017-11-3-2-7-mdcv-mastering-display-color-volume.md#example-5)

Example mDCV chunk mastering display color primaries for 

<a id="ref-for-dfn-hdr-4"></a>

[HDR](002-2-scope.md#dfn-hdr) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")]:

<a id="mDCV-chunk-hdr-primaries-example"></a>

<table class="numbered simple" id="mDCV-chunk-hdr-primaries-example">
<tbody><tr>
<th>Name</th>
<th>Actual values</th>
<th>Stored Decimal values</th>
<th>Stored Hexadecimal values</th>
</tr>
<tr>
<td rowspan="3">Color primaries specified in [<cite><a class="bibref" data-link-type="biblio" href="045-g-2-informative-references.md#bib-itu-r-bt.2020" title="Parameter values for ultra-high definition television systems for production and international programme exchange">ITU-R-BT.2020</a></cite>]</td>
<td>(0.708, 0.292)</td>
<td>{ 35400, 14600 }</td>
<td>{ 8A 48, 39 08 }</td>
</tr>
<tr>
<td>(0.170, 0.797)</td>
<td>{ 8500, 39850 }</td>
<td>{ 21 34, 9B AA }</td>
</tr>
<tr>
<td>(0.131, 0.046)</td>
<td>{ 6550, 2300 }</td>
<td>{ 19 96, 08 FC }</td>
</tr>
</tbody></table>



<a id="example-6"></a>

[Example 6](017-11-3-2-7-mdcv-mastering-display-color-volume.md#example-6)

Example mDCV chunk mastering display white point for 

<a id="ref-for-dfn-hdr-5"></a>

[HDR](002-2-scope.md#dfn-hdr) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")]:

<a id="mDCV-chunk-hdr-white-point-example"></a>

| Name | Actual values | Stored Decimal values | Stored Hexadecimal values |
| --- | --- | --- | --- |
| Illuminant D65 specified in [[SMPTE-RP-177](045-g-2-informative-references.md#bib-smpte-rp-177 "Derivation of Basic Television Color Equations")] | (0.3127, 0.3290) | { 15635, 16450 } | { 3C 05, 40 42 } |



<a id="example-7"></a>

[Example 7](017-11-3-2-7-mdcv-mastering-display-color-volume.md#example-7)

Example mDCV chunk mastering display maximum luminance for 

<a id="ref-for-dfn-hdr-6"></a>

[HDR](002-2-scope.md#dfn-hdr) [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")]:

<a id="mDCV-chunk-hdr-max-luminance-example"></a>

<table class="numbered simple" id="mDCV-chunk-hdr-max-luminance-example">
<tbody><tr>
<th>Actual value</th>
<th>Stored Decimal value</th>
<th>Stored Hexadecimal value</th>
</tr>
<tr>
<td>4000 cd/m<sup>2</sup></td>
<td>40000000</td>
<td>02 62 5A 00</td>
</tr>
</tbody></table>



<a id="example-8"></a>

[Example 8](017-11-3-2-7-mdcv-mastering-display-color-volume.md#example-8)

Example mDCV chunk mastering display minimum luminance:

<a id="mDCV-chunk-hdr-min-luminance-example"></a>

<table class="numbered simple" id="mDCV-chunk-hdr-min-luminance-example">
<tbody><tr>
<th>Actual value</th>
<th>Stored Decimal value</th>
<th>Stored Hexadecimal value</th>
</tr>
<tr>
<td>0.0005 cd/m<sup>2</sup></td>
<td>5</td>
<td>00 00 00 05</td>
</tr>
</tbody></table>



<a id="ref-for-dfn-sdr-6"></a>

Below are mDCV examples for [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")] [SDR](002-2-scope.md#dfn-sdr).

<a id="example-9"></a>

[Example 9](017-11-3-2-7-mdcv-mastering-display-color-volume.md#example-9)

Example mDCV chunk mastering display color primaries for [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")]:

<a id="mDCV-chunk-sdr-primaries-example"></a>

<table class="numbered simple" id="mDCV-chunk-sdr-primaries-example">
<tbody><tr>
<th>Name</th>
<th>Actual values</th>
<th>Stored Decimal values</th>
<th>Stored Hexadecimal values</th>
</tr>
<tr>
<td rowspan="3">Color primaries specified in [<cite><a class="bibref" data-link-type="biblio" href="044-g-references.md#bib-display-p3" title="Display P3">Display-P3</a></cite>]</td>
<td>(0.68, 0.32)</td>
<td>{ 34000, 16000 }</td>
<td>{ 84 D0, 3E 80 }</td>
</tr>
<tr>
<td>(0.265, 0.69)</td>
<td>{ 13520, 34500 }</td>
<td>{ 34 D0, 86 C4 }</td>
</tr>
<tr>
<td>(0.15, 0.06)</td>
<td>{ 7500, 3000 }</td>
<td>{ 1D 4C, 0B B8 }</td>
</tr>
</tbody></table>



<a id="example-10"></a>

[Example 10](017-11-3-2-7-mdcv-mastering-display-color-volume.md#example-10)

Example mDCV chunk mastering display white point for [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")]:

