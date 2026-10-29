# Image output captures

Each source exercises a separate concern: geometry and natural size; transforms
and border effects; placement fallbacks; column/page flow; images inside quote
and list text; JPEG color spaces; and GIF subframes. Assets are synthetic checker/gradient
patterns in PNG, JPEG and GIF forms.

`captured/<case>/` contains the complete captured DOCX package, model, original
package/source hashes, native 72-DPI PNG pages, and `images.json`. The latter
records image placement and clip rectangles extracted from the captured native
PDF, with decoded image pixels stored separately. The test compares every
package part, including original binary media. XML comparison ignores generation
timestamps and inventories advisory page-break markers; all other XML order,
values, relationships and text remain significant.

Geometry is compared within 0.011 point because captured PDF coordinates have
two decimal places. Transformed image pixels allow one premultiplied channel
step, or three for independent JPEG decoding. Native full-page PNGs allow at
most 48 channel levels and a foreground mean below 2; the text-only contexts
case allows a mean below 10 for its small antialiased glyph edges. These bounds
do not replace the separate image geometry and transformed-pixel checks.

The focused manual comparison additionally checks PDF validity and rasterizes
both native PDFs at 144 DPI. All eight pages match pixel for pixel. These are
native-output captures; Microsoft Word rendering remains **unreviewed**.

Format-specific image behavior remains explicit: native sizing fits the column;
DOCX retains authored extents. Native borders are solid and opaque, with PDF
centered strokes and PNG inside edge bands. DOCX retains dash and opacity. Native
quote/list text uses visible image placeholders, while DOCX keeps image runs.

For a GIF whose first frame is smaller than its logical canvas, natural sizing
uses the canvas until a transform is applied, then uses the decoded frame. This
captured distinction is retained for review; explicit frame sizes avoid it.
