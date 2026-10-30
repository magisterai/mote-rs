# Positioned slide tables

Five fixed sources cover table placement, explicit column and row sizes, merged
cells, uneven rows, conditional style precedence, text margins, wrapping,
vertical text, and single-owner borders including explicit suppression.

Captures contain source and font hashes, evaluated models, every editable PPTX
package part, ordered native geometry and independent native PNGs. Identical
parts share content-addressed files. All five native PDFs match the captured
144-DPI raster exactly. PNG differences are limited to text rasterization, with
per-case channel and mean bounds and no difference above 48 channel levels.
These cases remain unreviewed in Microsoft PowerPoint.

A slide table occupies its box without document pagination. Unspecified row
heights divide the box equally; explicit row heights replace their shares without
redistribution. Fully specified cell widths can determine the column grid.
Content clips inside cell margins. PowerPoint always wraps table-cell text, so
`no_wrap` warns. Row height rules and pagination flags have no slide effect.
Native table paragraphs use uniform cell frames; editable paragraphs retain their
indentation. Table-style paragraph defaults other than alignment affect editable
text only. Physical borders prefer the upper/left declaration when specified,
including explicit `none`, and omit internal segments in merged cells.
