# Native pie and doughnut captures

Four frozen sources cover ten pages: default output, four legend positions,
chart/plot fills and borders, single and multiline labels, small sectors, signed
and zero data, absent data, one-value data, multiple series, and category/value
length differences. Ordered geometry compares every sector, text advance,
rectangle and clip within 1e-8 points. Sources and captures have content hashes.

All ten PDF and PNG pages were visually inspected. Nine native PDF pages were
pixel-identical at 144 DPI; the small-sector page differs at one pixel by at most
13 channel levels. Independent PNG limits are fixed per case: maximum channel
differences 48–92, means 0.2–1 on nonwhite pixels, at most one pixel above 64.
The text-only empty-data page permits mean 8.5 because it contains no large filled
areas to dilute text rasterization differences. Office review is unreviewed.

Retained behavior and limits:

- Only the first series supplies sectors; the legend follows category count.
- Nonpositive values have zero sweep, but enabled labels still show their values.
  Their labels can overlap, as can labels on very small sectors.
- Label-position overrides use automatic radial placement and warn. Axis,
  gridline, gap and overlap settings have no native effect and warn.
- The doughnut center is painted after labels and can cover part of a label.
  Its fill uses plot background, then chart background, then white.
- Overflowing positive totals or sector angles report errors before painting.

The fixture geometry remains unchanged so these limitations stay visible.
