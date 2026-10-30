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

All eleven native PDF pages were pixel-identical at 144 DPI. All PDF and PNG
pages were visually inspected. This is native-output evidence, not Office review;
Word and PowerPoint remain unreviewed.

Observed limitations remain visible in the sources and captures:

- Mixed-sign percentage stacks divide by the positive category total. Negative
  values can exceed -100% and extend outside the plot.
- A left legend can overlap the vertical value-axis title.
- Long data labels wrap inside a fixed-height box and can clip.
- Decimal tick text retains floating-point accumulation digits.
- Native horizontal bars ignore overlap; column overlap changes bar width without
  independently changing the inter-series offset.

These are retained behavior, not layout corrections. Source geometry is not
adjusted to conceal them. Non-finite input and overflowing stacked totals are
separate explicit error tests. Additional native chart kinds are tested in their
own focused collections as support is added.
