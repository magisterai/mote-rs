# Images

[Specification](README.md) · [Checklist](../checklists/image.md)

An image is a block containing external raster media. `#image("diagram.png")`
produces an `image` element with `src = "diagram.png"` and no children. Unlike
inline text, an image starts its own flow block. `src` is a required nonempty
string, positional or named; supplying both is invalid. The directive accepts
no content body. A positional source overrides `properties.src`.

`![Diagram](diagram.png "Overview")` produces an image with `src`, `alt =
"Diagram"`, and `title = "Overview"`. The alternative text is the evaluated
inline label flattened to text. File paths resolve relative to the source file;
custom asset resolvers can supply named bytes without filesystem access. The
filesystem resolver does not fetch network URLs. PNG, JPEG and GIF are supported;
GIF uses its first frame for native rendering.

`width` and `height` accept nonnegative lengths. With neither, the natural size
is the pixel size at 96 DPI. With one positive dimension, the other follows the
source aspect ratio. With both, the image stretches into that frame. Use explicit
units: `#image("diagram.png", width = pt(144))` produces an image with a
144-point width. The asset is resolved during output, not language evaluation.

`alignment` is left, center, right, or justify. `spacing = { before = pt(4),
after = pt(8) }` sets block spacing. `placement.width`/`height` are fallbacks for
omitted direct dimensions; direct alignment wins over `placement.align`, then
`placement.horizontal.alignment`. Native document images fit proportionally to
the column width. Placement metadata does not create floating layout.

`placement` also accepts lengths `x`, `y`, and `margin` fields; `type` (inline,
anchor, floating), `float` (none, top, bottom), `wrap` (inline, square, tight,
through, top_bottom, behind, in_front), `anchor` (page, margin, paragraph,
character), booleans `allow_overlap`, `behind_text`, `layout_in_cell`, `locked`,
and nonnegative integer `relative_height`. Its horizontal/vertical objects accept
`offset`, `alignment` (left, center, right, top, bottom, inside, outside), and
`relative_to`. Horizontal references are margin, page, column, character;
vertical references are margin, page, paragraph, line. Unsupported positioning
retains its metadata and reports that ordinary flow is used.

`crop` has left, top, right and bottom percentages. Cropping fills the original
frame; it does not change the frame's aspect ratio. `rotation` is a number or a
string with optional `deg`. `flip_horizontal` and `flip_vertical` are booleans.
The transform order is crop, flips, effects, then rotation. Native rotation
expands the raster around the frame but clips vertically to its authored band.

`effects` accepts `brightness`, `contrast` and `transparency` percentages and
`grayscale`. Native effects clamp brightness/contrast to -100..100 and
transparency/crop to 0..100. Percentage strings may end in `%`.

`border` accepts a length `width`, hex `color`, `opacity` from 0 to 1 or a
percentage (a number above 1 or a string ending in `%`), and `style`: single, solid, dashed, dotted, or none. DOCX
retains border style/opacity; native document borders paint a solid opaque line.

`name`, `alt`, `description`, `title`, `style`, and `link` are strings. The Word
object name prefers name, then title, then its generated media filename. Its
description prefers description, alt, title, then src. `link` makes the image
clickable in DOCX/PDF; PNG has no interactive destination. `style` is retained
metadata for images. `properties` follows the established override rules.

Missing or unsupported assets produce a warning and visible placeholder text.
In quote or list text, native output uses the alternative text in square brackets
and reports that fallback; DOCX retains the editable image. Image allocation limits and output-format
mechanics are documented alongside implementation tests.

A zero dimension remains authored metadata in DOCX; native sizing treats it as
unspecified. Prefer positive dimensions when sizing an image.
