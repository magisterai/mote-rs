# Shape text captures

Four frozen sources exercise all ten preset text rectangles; top, center and
bottom alignment; margins; clipping, shrink-to-fit and text-height growth;
headings, named styles, mixed run sizes, highlights, strikes, small caps, code,
tabs, line breaks, links, nested lists, ordered lists and task markers; clockwise,
counterclockwise and right-angle rotation; and readable text through shape flips.

Each capture includes the complete evaluated model and editable DOCX package,
source hash, measured lines, clips, rotation centers, task geometry and independent
72-DPI PNG output. Tests compare coordinates within 1e-8 points and retain
separate raster edge limits. The PNG maximum/foreground-mean/edge-pixel limits
are 80/3/20 for fitting, 108/9/2 for formatting, 56/1/0 for presets, and 64/2/0
for rotation. An edge pixel exceeds 64/255 in any color channel.

All four native PDF pages were inspected at 144 DPI. Fitting, formatting and
rotation were pixel-exact; presets differed at one pixel by 2/255. Every independent
PNG page was inspected too. Microsoft Word review remains unreviewed.

`resize_shape` grows the native text clip, leaving the painted outline unchanged.
Rotated text and its clip share one transform; PDF hyperlink rectangles retain
unrotated positions and trigger a warning. Shape text uses Aptos for default body
and heading faces. These observations describe the captured output behavior.

The earlier six sources in `../shape-output/` were also rendered and inspected.
The origin-position default case retains visible text at negative page coordinates;
its PDF differs from the captured renderer, which omits the first line. Its planned
text geometry agrees. The fixture remains unchanged, and this exception must not
be treated as a pixel-exact PDF result.
