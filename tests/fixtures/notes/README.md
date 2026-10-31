# Note captures

Eleven frozen cases cover inline note references, independent numbering, rich and
empty bodies, hyperlinks, Markdown definitions, page pressure, multiple notes,
endnote continuation, footer reservation, fields, bookmarks, and placement fallbacks.
Each capture includes the complete evaluated model and DOCX package, geometry,
native PNG pages, and source/artifact hashes. Word visual review is unreviewed.

All 19 pages match captured native geometry within 1e-8 points. PNG comparison
allows only the recorded font-edge rasterization tolerance: maximum channel
difference 56/255 and mean foreground difference 11/255. PDF raster comparison at
144 DPI is exact on all 19 pages. All native PDF pages were visually inspected.

The oversized-note diagnostic retains negative off-page text coordinates.
PDF and PNG paint those signed coordinates consistently. Oversized notes still
warn and can overlap the reference line.

`endnote-flow` verifies measured footer reservation when no height is declared.
The independent `endnote-footer` case retains a larger explicit reservation.
Both continue naturally without overlapping footer text. Sources remain unchanged.
