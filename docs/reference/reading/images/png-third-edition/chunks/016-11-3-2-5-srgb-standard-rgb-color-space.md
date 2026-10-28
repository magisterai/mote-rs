# 11.3.2.5 sRGB Standard RGB color space

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](015-11-3-ancillary-chunks.md) · [Next](017-11-3-2-7-mdcv-mastering-display-color-volume.md)

<a id="sBIT-structure"></a>

<table class="numbered simple" id="sBIT-structure">
<caption><a class="self-link" href="016-11-3-2-5-srgb-standard-rgb-color-space.md#sBIT-structure">Table <bdi class="tableno">14</bdi></a> <span class="table-title">
              sBIT chunk contents
            </span></caption>
<tbody><tr>
<th colspan="2">Color type 0</th>
</tr>
<tr>
<td>significant greyscale bits</td>
<td>1 byte</td>
</tr>
<tr>
<th colspan="2">Color types 2 and 3</th>
</tr>
<tr>
<td>significant red bits</td>
<td>1 byte</td>
</tr>
<tr>
<td>significant green bits</td>
<td>1 byte</td>
</tr>
<tr>
<td>significant blue bits</td>
<td>1 byte</td>
</tr>
<tr>
<th colspan="2">Color type 4</th>
</tr>
<tr>
<td>significant greyscale bits</td>
<td>1 byte</td>
</tr>
<tr>
<td>significant alpha bits</td>
<td>1 byte</td>
</tr>
<tr>
<th colspan="2">Color type 6</th>
</tr>
<tr>
<td>significant red bits</td>
<td>1 byte</td>
</tr>
<tr>
<td>significant green bits</td>
<td>1 byte</td>
</tr>
<tr>
<td>significant blue bits</td>
<td>1 byte</td>
</tr>
<tr>
<td>significant alpha bits</td>
<td>1 byte</td>
</tr>
</tbody></table>

<a id="ref-for-3indexedColour-14"></a>

<a id="ref-for-3colourType-27"></a>

Each depth specified in sBIT shall be greater than zero and less than or equal to the
sample depth (which is 8 for [indexed-color](004-4-3-color-spaces.md#3indexedColour) images, and the bit depth given in [IHDR](014-10-3-other-uses-of-compression.md#11IHDR)
for other [color types](010-5-7-defining-chunks.md#3colourType)). Note that sBIT does not provide a sample depth for the alpha
channel that is implied by a [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk; in that case, all of the sample bits of the
alpha channel are to be treated as significant. If the sBIT chunk is not present, then all of
the sample bits of all channels are to be treated as significant.

<a id="11sRGB"></a>

<a id="srgb-standard-colour-space"></a>

##### 11.3.2.5 sRGB Standard RGB color space

The four-byte chunk type field contains the hexadecimal values



```
73 52 47 42
```




If the sRGB chunk is present, the image samples conform to the sRGB color space [[SRGB](044-g-references.md#bib-srgb "Multimedia systems and equipment - Colour measurement and management - Part 2-1: Colour management - Default RGB colour space - sRGB")]
and should be displayed using the specified rendering intent defined by the International Color Consortium [[ICC](044-g-references.md#bib-icc "ICC.1:2022 (Profile version 4.4.0.0)")] or
[[ICC-2](044-g-references.md#bib-icc-2 "Specification ICC.2:2019 (Profile version 5.0.0 - iccMAX)")].

The sRGB chunk contains:

<a id="sRGB-structure"></a>

[Table 15](016-11-3-2-5-srgb-standard-rgb-color-space.md#sRGB-structure) 
sRGB chunk contents

| Name | Size |
| --- | --- |
| Rendering intent | 1 byte |

The following values are defined for rendering intent:

<a id="rendering-intent-structure"></a>

<a id="ref-for-dfn-white-point-3"></a>

[Table 16](016-11-3-2-5-srgb-standard-rgb-color-space.md#rendering-intent-structure) 
Rendering intent values

| Value | Name | Description |
| --- | --- | --- |
| 0 | Perceptual | for images preferring good adaptation to the output device gamut at the expense of colorimetric accuracy, such as photographs. |
| 1 | Relative colorimetric | for images requiring color appearance matching (relative to the output device [white point](002-2-scope.md#dfn-white-point)), such as logos. |
| 2 | Saturation | for images preferring preservation of saturation at the expense of hue and lightness, such as charts and graphs. |
| 3 | Absolute colorimetric | for images requiring preservation of absolute colorimetry, such as previews of images destined for a different output device (proofs). |

It is recommended that a PNG encoder that writes the sRGB chunk also write a [gAMA](015-11-3-ancillary-chunks.md#11gAMA) chunk (and optionally a [cHRM](015-11-3-ancillary-chunks.md#11cHRM) chunk) for compatibility
with decoders that do not use the sRGB chunk. Only the following values shall be used.

<a id="sRGB-gAMA-cHRM"></a>

<table class="numbered simple" id="sRGB-gAMA-cHRM">
<caption><a class="self-link" href="016-11-3-2-5-srgb-standard-rgb-color-space.md#sRGB-gAMA-cHRM">Table <bdi class="tableno">17</bdi></a> <span class="table-title">
              gAMA and cHRM values for sRGB
            </span></caption>
<tbody><tr>
<th colspan="2">gAMA</th>
</tr>
<tr>
<td>Gamma</td>
<td>45455</td>
</tr>
<tr>
<th colspan="2">cHRM</th>
</tr>
<tr>
<td>White point x</td>
<td>31270</td>
</tr>
<tr>
<td>White point y</td>
<td>32900</td>
</tr>
<tr>
<td>Red x</td>
<td>64000</td>
</tr>
<tr>
<td>Red y</td>
<td>33000</td>
</tr>
<tr>
<td>Green x</td>
<td>30000</td>
</tr>
<tr>
<td>Green y</td>
<td>60000</td>
</tr>
<tr>
<td>Blue x</td>
<td>15000</td>
</tr>
<tr>
<td>Blue y</td>
<td>6000</td>
</tr>
</tbody></table>

This chunk is ignored
unless it is the [highest-precedence color chunk](004-4-3-color-spaces.md#color-chunk-precendence)
understood by the decoder.

It is recommended that the [sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#11sRGB) and [iCCP](015-11-3-ancillary-chunks.md#11iCCP)
chunks do not appear simultaneously in a PNG datastream.

<a id="cICP-chunk"></a>

<a id="x11-3-2-6-cicp-coding-independent-code-points-for-video-signal-type-identification"></a>

##### 11.3.2.6 cICP Coding-independent code points for video signal type identification

The four-byte chunk type field contains the hexadecimal values



```
63 49 43 50
```




If present, the cICP chunk specifies the color space (primaries), transfer function, matrix
coefficients and scaling factor of the image using the code points specified in [[ITU-T-H.273](044-g-references.md#bib-itu-t-h.273 "ITU-T H.273, SERIES H: AUDIOVISUAL AND MULTIMEDIA SYSTEMS Infrastructure of audiovisual services – Coding of moving video. Coding-independent code points for video signal type identification")]. The video format signaling *SHOULD* be used
when processing the image, including by a decoder or when rendering the image.

The cICP chunk consists of four 1-byte unsigned integers to identify the characteristics described above.

The following specifies the syntax of the cICP chunk:

<a id="cICP-chunk-syntax"></a>

[Table 18](016-11-3-2-5-srgb-standard-rgb-color-space.md#cICP-chunk-syntax) 
cICP chunk components

| Name | Size |
| --- | --- |
| Color Primaries | 1 byte |
| Transfer Function | 1 byte |
| Matrix Coefficients | 1 byte |
| Video Full Range Flag | 1 byte |

Each of the fields of the cICP chunk corresponds to the parameter of the same name in
[[ITU-T-H.273](044-g-references.md#bib-itu-t-h.273 "ITU-T H.273, SERIES H: AUDIOVISUAL AND MULTIMEDIA SYSTEMS Infrastructure of audiovisual services – Coding of moving video. Coding-independent code points for video signal type identification")].

RGB is currently the only supported color model in PNG, and as such `Matrix Coefficients` shall be set to `0`.

<a id="issue-container-generatedID-11"></a>

<a id="h-note-11"></a>

Note

If `Video Full Range Flag` value is `1`, then the image is a 

<a id="ref-for-dfn-full-range-image-1"></a>

[full-range image](002-2-scope.md#dfn-full-range-image). Typically,
images in the RGB color representation are stored in the full-range signal quantization, therefore the vast
majority of computer graphics and web images, including those used in traditional PNG workflows, are 

<a id="ref-for-dfn-full-range-image-2"></a>

[full-range
images](002-2-scope.md#dfn-full-range-image). If `Video Full Range Flag` value is `0`, then the image is a 

<a id="ref-for-dfn-narrow-range-image-1"></a>

[narrow-range
image](002-2-scope.md#dfn-narrow-range-image). Narrow range images are found in video workflows where there are sample values below reference
black (0% signal level) or above nominal peak (100% signal level). For example, [[ITU-R-BT.709](044-g-references.md#bib-itu-r-bt.709 "ITU-R BT.709, SERIES BT: BROADCASTING SERVICE (TELEVISION). Parameter values for the HDTV standards for production and international programme exchange")] specifies that, for
10-bit coding, reference black (called black level) corresponds to a code value of 64 and nominal peak to a
code value of 940. In narrow range, momentary excursions defined as overshoots and undershoots exist below
reference black and above nominal peak in order to preserve processing artifacts caused by filtering/compression
or by uncontrolled lighting without clipping. This can improve image quality during additional stages of processing
and compression. The use of undershoot/overshoot has also been used to preserve additional color volume (both light
and color) as described in [[EBU-R-103](045-g-2-informative-references.md#bib-ebu-r-103 "Video Signal Tolerance in Digital Television Systems")]. [[SMPTE-RP-2077](045-g-2-informative-references.md#bib-smpte-rp-2077 "Full-Range Image Mapping")] describes full range in more detail and includes the mapping
from 

<a id="ref-for-dfn-full-range-image-3"></a>

[full-range images](002-2-scope.md#dfn-full-range-image) to 

<a id="ref-for-dfn-narrow-range-image-2"></a>

[narrow-range images](002-2-scope.md#dfn-narrow-range-image) and describes protected code values for SDI (baseband)
carriage.

<a id="ref-for-dfn-narrow-range-image-3"></a>

<a id="ref-for-dfn-eotf-1"></a>

<a id="ref-for-dfn-oetf-1"></a>

If `Video Full Range Flag` is `0`
(a [narrow-range image](002-2-scope.md#dfn-narrow-range-image)), recommended practice
is to define transfer functions
such as [EOTF](003-4-concepts.md#dfn-eotf) or inverse [OETF](003-4-concepts.md#dfn-oetf)
over the extended range,
so as to include negative values.
This is done as follows:



```
out = sign(in) * TransferFunction(abs(in))
```




The cICP chunk *MUST* come before the [PLTE](014-10-3-other-uses-of-compression.md#11PLTE) and [IDAT](014-10-3-other-uses-of-compression.md#11IDAT) chunks.

This chunk, if understood by the decoder, is the
[highest-precedence color chunk](004-4-3-color-spaces.md#color-chunk-precendence).

<a id="ex-cICP-BT.2100-PQ-full"></a>

[Example 1](016-11-3-2-5-srgb-standard-rgb-color-space.md#ex-cICP-BT.2100-PQ-full)

cICP chunk field values for a 

<a id="ref-for-dfn-full-range-image-4"></a>

[full-range image](002-2-scope.md#dfn-full-range-image) that uses the color primaries and the

<a id="ref-for-dfn-pq-2"></a>

[PQ](002-2-scope.md#dfn-pq) 

<a id="ref-for-dfn-transfer-function-8"></a>

[transfer function](002-2-scope.md#dfn-transfer-function) specified at [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")]:



```
09 10 00 01
```




(Four 1-byte unsigned integers, in hexadecimal)



<a id="ex-cICP-BT.2100-HLG-full"></a>

[Example 2](016-11-3-2-5-srgb-standard-rgb-color-space.md#ex-cICP-BT.2100-HLG-full)

cICP chunk field values for a 

<a id="ref-for-dfn-full-range-image-5"></a>

[full-range image](002-2-scope.md#dfn-full-range-image) that uses the color primaries and the

<a id="ref-for-dfn-hlg-2"></a>

[HLG](002-2-scope.md#dfn-hlg) 

<a id="ref-for-dfn-transfer-function-9"></a>

[transfer function](002-2-scope.md#dfn-transfer-function) specified at [[ITU-R-BT.2100](044-g-references.md#bib-itu-r-bt.2100 "ITU-R BT.2100, SERIES BT: BROADCASTING SERVICE (TELEVISION). Image parameter values for high dynamic range television for use in production and international programme exchange")]:



```
09 12 00 01
```




(Four 1-byte unsigned integers, in hexadecimal)



<a id="ex-cICP-BT.709-narrow"></a>

[Example 3](016-11-3-2-5-srgb-standard-rgb-color-space.md#ex-cICP-BT.709-narrow)

cICP chunk field values for a 

<a id="ref-for-dfn-narrow-range-image-4"></a>

[narrow-range image](002-2-scope.md#dfn-narrow-range-image)
that uses the color primaries and the 

<a id="ref-for-dfn-transfer-function-10"></a>

[transfer function](002-2-scope.md#dfn-transfer-function) defined at [[ITU-R-BT.709](044-g-references.md#bib-itu-r-bt.709 "ITU-R BT.709, SERIES BT: BROADCASTING SERVICE (TELEVISION). Parameter values for the HDTV standards for production and international programme exchange")]:



```
01 01 00 00
```




(Four 1-byte unsigned integers, in hexadecimal)



<a id="display-p3"></a>

In a similar way to the use of the
[sRGB](016-11-3-2-5-srgb-standard-rgb-color-space.md#srgb-standard-colour-space) chunk
to compactly signal an sRGB image, cICP can be used
to compactly signal a Display P3 image [[Display-P3](044-g-references.md#bib-display-p3 "Display P3")].

<a id="ex-cICP-Display-P3-full"></a>

[Example 4](016-11-3-2-5-srgb-standard-rgb-color-space.md#ex-cICP-Display-P3-full)

cICP chunk field values for a 

<a id="ref-for-dfn-full-range-image-6"></a>

[full-range image](002-2-scope.md#dfn-full-range-image)
that uses the color primaries and the 

