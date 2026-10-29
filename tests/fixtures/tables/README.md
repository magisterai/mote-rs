# Table output captures

The sixteen frozen sources cover basic flow, property mapping, merges, Markdown,
cell block order, nested lists, inherited styles, physical borders, automatic and
explicit widths, images, nested tables, pagination, columns and oversized content.
Media is shared with the image fixtures through source-relative paths.

`captured/<case>/capture.json` inventories the complete DOCX package and source
hash. `model.json` preserves the evaluated model, and `package/` preserves every
original package part. Regression tests compare every part and relationship.
XML namespace prefixes, attribute order, formatting whitespace and generation
timestamps are normalized; binary media compares exactly. Advisory rendered-page
markers and their same-run text splits are normalized without ignoring authored
breaks, tabs, formatting changes or genuine text-node boundaries.

For fifteen cases, `native-capture.json` inventories independently captured
`geometry.json` and `reference-N.png` pages by SHA-256. Geometry uses core font
metrics with installed-font discovery disabled and compares within 1e-8 points.
It covers every text fragment, physical border and image bound across 25 pages.
The PDF comparison renders both independent PDFs at 144 DPI: every pixel matches.
PDF files and comparison intermediates remain build artifacts.

Native PNG captures use 72 DPI, macOS Helvetica outlines for core-font text, and
installed Arial faces for the explicitly named font in `styles.mote`. Native
raster tests use that same font-discovery mode and require those fonts. Maximum
channel difference is bounded at 56/255 and mean foreground-channel difference
at 9/255 for font-edge rasterization (11/255 for the oversized-merge case, whose
short final line has a foreground dominated by edges). Portable geometry tests run separately with
font discovery disabled; proprietary font files are not copied into the repository.

`columns.mote` is explicitly blocked for native reference comparison: repeated
headers in successive columns produce duplicate text-line identifiers on one
page and fail validation. Its model and DOCX still compare. Independent candidate
tests verify repeated headers in both columns, two-page flow and valid PDF/PNG.
`column-flow.mote` separately verifies comparable column continuation without
repeated headers. The blocked fixture is retained unchanged.

`oversized-merge.mote` deliberately exceeds a complete frame with unsplittable
content. Its warning, individual-row fallback and clipped image are captured
behavior, not an example of desirable document layout. Normal-flow fixtures do
not use authored breaks or fixed row heights to manufacture pagination.

All candidate pages have been visually inspected. No fixture has been rendered
or reviewed in Microsoft Word; its Office status remains unreviewed.
