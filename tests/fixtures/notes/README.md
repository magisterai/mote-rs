# Note captures

Eleven frozen cases cover inline note references, independent numbering, rich and
empty bodies, hyperlinks, Markdown definitions, page pressure, multiple notes,
endnote continuation, footer reservation, fields, bookmarks, and placement fallbacks.
Each capture includes the complete evaluated model and DOCX package, geometry,
native PNG pages, and source/artifact hashes. Word visual review is unreviewed.

All 19 pages match captured native geometry within 1e-8 points. PNG comparison
allows only the recorded font-edge rasterization tolerance: maximum channel
difference 56/255 and mean foreground difference 11/255. PDF raster comparison at
144 DPI is exact on 18 pages. All native PDF pages were visually inspected.

The oversized-note diagnostic has one explicit PDF raster exception: its geometry
contains negative off-page text coordinates. Painting these signed coordinates
directly avoids interpreting them relative to the page bottom. The capture's PNG
and geometry remain the conformance evidence for that case; the exception is recorded
in its manifest. Oversized notes still warn and can overlap the reference line.

`endnote-flow` retains a visible continuation/footer overlap when the footer has no
reserved height. The independent `endnote-footer` case reserves explicit height.
These diagnostics expose output limits; their content is not adjusted after review.
