# Native shape geometry captures

Three frozen sources cover all ten presets, cloud outline and decoration paths,
rotations, flips, opacity, solid/dashed/dotted/borderless outlines, layered and
unblurred shadows, page anchoring, labels, pending paragraph spacing, page breaks,
and zero dimensions. These cases intentionally contain no shape text: text layout
is a separate capability. Source and PNG hashes guard each capture.

All path coordinates match within 1e-8 points (observed maximum 2.3e-13), with
matching fills, alpha, stroke widths and patterns. All four native PDF pages were
inspected at 144 DPI. Three were pixel-exact; `effects` differed at three pixels,
by at most 4/255. The PDF painter retains five-decimal polygon fill coordinates,
two-decimal stroke coordinates, and three-decimal color/opacity values.

The independent 72-DPI PNG comparison allows rasterizer edge differences. Preset
joins observed a maximum channel difference of 107 and foreground mean 0.427,
with 79 pixels exceeding 64; tests cap these at 112, 1.0 and 90 respectively.
Other cases stay below maximum 56 and foreground mean 12 with no pixels exceeding
64. Every page was also visually inspected. PDF/PNG painting remains independent.

PNG patterned outlines restart their dash phase at polygon segments and use
round capsule ends; PDF outlines retain continuous path dashes. These are separate
captured painter behaviors. Matching these captures does not establish Word
conformance: Microsoft Word render review remains unreviewed.
