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
