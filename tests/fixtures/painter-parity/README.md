# Painter parity allowances

Mote draws each native page twice: once as PDF and once as PNG, through two
independent painters. [allowances.json](allowances.json) records, for each review
case and probe, the largest share of a page's inked blocks that may at present
differ between the PNG painter's page and the native PDF rasterized by the hayro
interpreter. `cargo xtask painter-parity` measures the shares and is described in
the [repository task guide](../../../xtask/README.md#painter-parity).

The remaining differences are of three kinds. Thin glyph stems fall on adjacent
pixels in the two rasterizers. The PNG painter applies kerning that the PDF
painter does not apply to TrueType and standard fonts, so glyphs drift apart
along a kerned line. Dashed outlines begin their dash patterns at different
phases. As with the Office allowances, a fix lowers the allowances it improves.
