# 13.16 Alpha channel processing

Source: [Portable Network Graphics (PNG) Specification](https://www.w3.org/TR/2025/REC-png-3-20250624/) · 3rd edition, W3C Recommendation 2025-06-24.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2025/REC-png-3-20250624/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](030-13-14-decoder-color-handling.md) · [Next](032-13-17-histogram-and-suggested-palette-usage.md)

<a id="13Alpha-channel-processing"></a>

<a id="x13-16-alpha-channel-processing"></a>

### 13.16 Alpha channel processing

<a id="ref-for-dfn-composited-11"></a>

The alpha channel can be used to [composite](002-2-scope.md#dfn-composited) a foreground image against a background image. The PNG datastream
defines the foreground image and the transparency mask, but not the background image. PNG decoders are **not**
required to support this most general case. It is expected that most will be able to support compositing against a single
background color.

<a id="ref-for-dfn-composited-12"></a>

The equation for computing a [composited](002-2-scope.md#dfn-composited) sample value is:



```
output = alpha * foreground + (1-alpha) * background
```




<a id="ref-for-dfn-gamma-21"></a>

where alpha and the input and output sample values are expressed as fractions in the range 0 to 1. This computation should
be performed with intensity samples (not [gamma](002-2-scope.md#dfn-gamma)-encoded samples). For color images, the computation is done separately
for R, G, and B samples.

<a id="ref-for-dfn-frame-buffer-8"></a>

<a id="ref-for-dfn-gamma-value-25"></a>

The following code illustrates the general case of compositing a foreground image against a background image. It assumes
that the original pixel data are available for the background image, and that output is to a [frame buffer](002-2-scope.md#dfn-frame-buffer) for display.
Other variants are possible; see the comments below the code. The code allows the sample depths and [gamma values](002-2-scope.md#dfn-gamma-value) of
foreground image and background image all to be different and not necessarily suited to the display system. In practice no
assumptions about equality should be made without first checking.

This code is ISO C [[ISO\_9899](044-g-references.md#bib-iso_9899 "ISO/IEC 9899:2018 Information technology — Programming languages — C")], with line numbers added for reference in the comments below.



```
01  int foreground[4];  /* image pixel: R, G, B, A */
02  int background[3];  /* background pixel: R, G, B */
03  int fbpix[3];       /* frame buffer pixel */
04  int fg_maxsample;   /* foreground max sample */
05  int bg_maxsample;   /* background max sample */
06  int fb_maxsample;   /* frame buffer max sample */
07  int ialpha;
08  float alpha, compalpha;
09  float gamfg, linfg, gambg, linbg, comppix, gcvideo;

    /* Get max sample values in data and frame buffer */
10  fg_maxsample = (1 << fg_sample_depth) - 1;
11  bg_maxsample = (1 << bg_sample_depth) - 1;
12  fb_maxsample = (1 << frame_buffer_sample_depth) - 1;
    /*
     * Get integer version of alpha.
     * Check for opaque and transparent special cases;
     * no compositing needed if so.
     *
     * We show the whole gamma decode/correct process in
     * floating point, but it would more likely be done
     * with lookup tables.
     */
13  ialpha = foreground[3];

14  if (ialpha == 0) {
        /*
         * Foreground image is transparent here.
         * If the background image is already in the frame
         * buffer, there is nothing to do.
         */
15      ;
16  } else if (ialpha == fg_maxsample) {
        /*
         * Copy foreground pixel to frame buffer.
         */
17      for (i = 0; i < 3; i++) {
18          gamfg = (float) foreground[i] / fg_maxsample;
19          linfg = pow(gamfg, 1.0 / fg_gamma);
20          comppix = linfg;
21          gcvideo = pow(comppix, 1.0 / display_exponent);
22          fbpix[i] = (int) (gcvideo * fb_maxsample + 0.5);
23      }
24  } else {
        /*
         * Compositing is necessary.
         * Get floating-point alpha and its complement.
         * Note: alpha is always linear; gamma does not
         * affect it.
         */
25      alpha = (float) ialpha / fg_maxsample;
26      compalpha = 1.0 - alpha;

27      for (i = 0; i < 3; i++) {
            /*
             * Convert foreground and background to floating
             * point, then undo gamma encoding.
             */
28          gamfg = (float) foreground[i] / fg_maxsample;
29          linfg = pow(gamfg, 1.0 / fg_gamma);
30          gambg = (float) background[i] / bg_maxsample;
```






```
31          linbg = pow(gambg, 1.0 / bg_gamma);
            /*
             * Composite.
             */
32          comppix = linfg * alpha + linbg * compalpha;
            /*
             * Gamma correct for display.
             * Convert to integer frame buffer pixel.
             */
33          gcvideo = pow(comppix, 1.0 / display_exponent);
34          fbpix[i] = (int) (gcvideo * fb_maxsample + 0.5);
35      }
36  }
```




Variations:

<a id="ref-for-dfn-frame-buffer-9"></a>

1. If output is to another PNG datastream instead of a [frame buffer](002-2-scope.md#dfn-frame-buffer), lines 21, 22, 33, and 34 should be changed
   along the following lines



```
/*
 * Gamma encode for storage in output datastream.
 * Convert to integer sample value.
 */
gamout = pow(comppix, outfile_gamma);
outpix[i] = (int) (gamout * out_maxsample + 0.5);
```




   Also, it becomes necessary to process background pixels when alpha is zero, rather than just skipping pixels. Thus, line 15
   will need to be replaced by copies of lines 17-23, but processing background instead of foreground pixel values.

<a id="ref-for-dfn-gamma-value-26"></a>

<a id="ref-for-dfn-gamma-22"></a>

2. If the sample depths of the output file, foreground file, and background file are all the same, and the three [gamma
   values](002-2-scope.md#dfn-gamma-value) also match, then the no-compositing code in lines 14-23 reduces to copying pixel values from the input file to
   the output file if alpha is one, or copying pixel values from background to output file if alpha is zero. Since alpha is
   typically either zero or one for the vast majority of pixels in an image, this is a significant saving. No [gamma](002-2-scope.md#dfn-gamma)
   computations are needed for most pixels.

<a id="ref-for-dfn-gamma-value-27"></a>

<a id="ref-for-dfn-gamma-23"></a>

<a id="ref-for-dfn-gamma-24"></a>

3. When the sample depths and [gamma values](002-2-scope.md#dfn-gamma-value) all match, it may appear attractive to skip the [gamma](002-2-scope.md#dfn-gamma) decoding
   and encoding (lines 28-31, 33-34) and just perform line 32 using [gamma](002-2-scope.md#dfn-gamma)-encoded sample values. Although this does not
   have too bad an effect on image quality, the time savings are small if alpha values of zero and one are treated as special
   cases as recommended here.

<a id="ref-for-dfn-frame-buffer-10"></a>

<a id="ref-for-dfn-frame-buffer-11"></a>

4. If the original pixel values of the background image are no longer available, only processed [frame buffer](002-2-scope.md#dfn-frame-buffer) pixels
   left by display of the background image, then lines 30 and 31 need to extract intensity from the [frame buffer](002-2-scope.md#dfn-frame-buffer) pixel
   values using code such as



```
/*
 * Convert frame buffer value into intensity sample.
 */
gcvideo = (float) fbpix[i] / fb_maxsample;
linbg = pow(gcvideo, display_exponent);
```




   However, some roundoff error can result, so it is better to have the original background pixels available if at all possible.

<a id="ref-for-dfn-gamma-25"></a>

5. Note that lines 18-22 are performing exactly the same [gamma](002-2-scope.md#dfn-gamma) computation that is done when no alpha channel is
   present. If the no-alpha case is handled with a lookup table, the same lookup table can be used here. Lines 28-31 and 33-34
   can also be done with (different) lookup tables.
6. Integer arithmetic can be used instead of floating point, providing care is taken to maintain sufficient precision
   throughout.

<a id="issue-container-generatedID-22"></a>

<a id="h-note-22"></a>

Note

NOTE In floating point, no overflow or underflow checks are needed, because the input sample values are
guaranteed to be between 0 and 1, and compositing always yields a result that is in between the input values (inclusive).
With integer arithmetic, some roundoff-error analysis might be needed to guarantee no overflow or underflow.

<a id="ref-for-dfn-composited-13"></a>

When displaying a PNG image with full alpha channel, it is important to be able to [composite](002-2-scope.md#dfn-composited) the image against some
background, even if it is only black. Ignoring the alpha channel will cause PNG images that have been converted from an
associated-alpha representation to look wrong. (Of course, if the alpha channel is a separate transparency mask, then
ignoring alpha is a useful option: it allows the hidden parts of the image to be recovered.)

<a id="ref-for-dfn-greyscale-17"></a>

<a id="ref-for-3truecolour-25"></a>

<a id="ref-for-3indexedColour-20"></a>

Even if the decoder does not implement true compositing logic, it is simple to deal with images that contain only zero and
one alpha values. (This is implicitly true for [greyscale](005-4-5-png-image.md#dfn-greyscale) and [truecolor](005-4-5-png-image.md#3truecolour) PNG datastreams that use a [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk; for [indexed-color](004-4-3-color-spaces.md#3indexedColour) PNG datastreams it is easy to check whether the [tRNS](015-11-3-ancillary-chunks.md#11tRNS) chunk contains any values other than 0 and 255.) In this simple case, transparent pixels are replaced by
the background color, while others are unchanged.

If a decoder contains only this much transparency capability, it should deal with a full alpha channel by treating all
nonzero alpha values as fully opaque or by dithering. Neither approach will yield very good results for images converted from
associated-alpha formats, but this is preferable to doing nothing. Dithering full alpha to binary alpha is very much like
dithering greyscale to black-and-white, except that all fully transparent and fully opaque pixels should be left unchanged by
the dither.

