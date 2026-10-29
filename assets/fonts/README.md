# Raster fallback fonts

The four bundled Go typefaces are from Bigelow & Holmes, distributed by the
upstream image/font library. They provide regular, bold, italic and bold-italic
outlines when a platform core-font face is unavailable. Declared font files take
precedence. Their [license](LICENSE.txt) must accompany binary distributions;
[sources.json](sources.json) records versioned URLs and SHA-256 fingerprints.

Fallback outlines do not alter the core-font metrics used for pagination. PNG
reports when these fallback outlines are selected. Use declared font files when
identical outlines across platforms are required.
