# Positioned presentation pictures

Fixed sources exercise stretch, contain and cover; asymmetric crop; both flips;
rotation; brightness, contrast, grayscale and transparency; image and box borders;
links and metadata; source-relative PNG, JPEG and GIF assets; and missing assets.
The format case includes grayscale and CMYK JPEGs and a GIF subframe.

Each capture retains the evaluated model, source and asset hashes, and every
package part. XML is compared canonically, ignoring only volatile core timestamps.
Embedded media bytes are compared exactly. These cases are unreviewed in
PowerPoint; package equality does not establish visual conformance.

PPTX fits the cropped aspect ratio and composes authored cropping with cover
cropping. An image border overrides the box border. Picture metadata preserves
XML-sensitive text. Missing images produce editable placeholder text.

Native captures retain all ordered picture and border operations, fitting frames,
clips, rotations and link targets across five pages. Native crop preparation
preserves the original raster extent, so contain/cover use that extent rather
than the cropped aspect ratio used by PPTX. These differing captured behaviors
remain explicit. Missing pictures are omitted with a diagnostic.

All five native PDF pages matched the captured PDF rasters exactly at 144 DPI.
At 72 DPI, three PNG pages match exactly. The format page differs by at most
three channel levels from JPEG decoding; the rotated page differs by at most
61 levels on antialiased edges, with a mean channel difference below 0.06.
Every PDF and PNG page was inspected. PDF link rectangles track picture rotation.
PowerPoint visual review remains pending.
