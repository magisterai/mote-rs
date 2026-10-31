# Native bar and column chart captures

Four frozen sources cover six chart kinds across eleven pages: grouped, stacked
and percentage-stacked columns and horizontal bars. The cases exercise signed
values, empty data, all four legend positions, titles and axis titles, chart/plot
fills and borders, major/minor gridlines, gap and overlap, four data-label
placements, width fitting, zero-width omission, paragraph spacing and pagination.

`geometry.json` records painting order, text and glyph advances, rules, filled
rectangles, outlines, clips and rotations. A filled and stroked rectangle is
represented by its two painting marks. Geometry compares within 1e-8 points.
Every captured PNG and source has a hash. The independent PNG comparison permits
bounded text rasterization differences: maximum channel difference 64 and mean 2
on nonwhite pixels, with no differences above 64; the rotated axis-title case
permits four edge pixels per page, maximum 88 and mean 0.5.

All eleven native PDF and PNG pages were visually inspected. The manifest records
144-DPI PDF comparison metrics, including small text raster differences after
label fitting. This is native-output evidence; Word and PowerPoint are unreviewed.

Percentage stacks use total category magnitude and preserve each sign. Negative
stacks remain within the percentage scale. Labels fit their allocated frames and
paint above every bar; legends reserve measured rows and the left legend stays
beside the value-axis title. Sources remain unchanged across these corrections.

Observed limitations remain visible in the sources and captures:

- Fitting preserves complete text, but very dense frames can require small labels;
  independent data labels can still collide.
- Decimal tick text retains floating-point accumulation digits.
- Native horizontal bars ignore overlap; column overlap changes bar width without
  independently changing the inter-series offset.

These remaining limits stay visible. Source geometry is not
adjusted to conceal them. Non-finite input and overflowing stacked totals are
separate explicit error tests. Additional native chart kinds are tested in their
own focused collections as support is added.
