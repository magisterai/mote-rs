# Native pie and doughnut captures

Four frozen sources cover ten pages: default output, four legend positions,
chart/plot fills and borders, single and multiline labels, small sectors, signed
and zero data, absent data, one-value data, multiple series, and category/value
length differences. Ordered geometry compares every sector, text advance,
rectangle and clip within 1e-8 points. Sources and captures have content hashes.

All ten PDF and PNG pages were visually inspected. Native PDF comparisons at 144 DPI
retain bounded edge differences, recorded per page in the capture manifests. Independent PNG limits are fixed per case: maximum channel
differences 48–92, means 0.2–1.2 on nonwhite pixels, at most one pixel above 64.
The multiline external-label page retains maximum 48 and allows mean 1.2
for the larger share of newly visible glyph edges; geometry stays within 1e-8.
The text-only empty-data page permits mean 8.5 because it contains no large filled
areas to dilute text rasterization differences. Office review is unreviewed.

Retained behavior and limits:

- Only the first series supplies sectors; the legend follows category count.
- Nonpositive values have zero sweep, but enabled labels still show their values.
  Labels that do not fit their sectors move into separated side columns.
  Extreme density can reduce text size to keep every label within its frame.
- Label-position overrides use automatic radial placement and warn. Axis,
  gridline, gap and overlap settings have no native effect and warn.
- The doughnut center paints before labels. Its fill uses plot background, then
  chart background, then white. Leaders route outside the circle.
- Overflowing positive totals report errors before painting. Angle calculation
  divides before multiplying, retaining finite very large single values.

Fixture sources remain unchanged; captured geometry records the corrected placement.
