# Native line and area captures

Three frozen sources cover line, area, stacked area and percentage-stacked area
charts across six pages. They exercise signed and empty data, zero and short
series, four legend positions, titles, axis titles, fills, borders, gridlines,
markers, line labels and positive percentage stacks.

The ordered geometry includes paths, text/glyph advances, clips, rotations,
rectangles and rules. Every coordinate compares within 1e-8 points. Source,
geometry and independent PNG files have hashes. Native PNG differences are
bounded by case: maximum channel differences 64–92, mean differences 0.6–2.5 on
nonwhite pixels, and at most four edge pixels above 64. All six PDF and PNG pages
were visually inspected. Five native PDF pages were pixel-identical at 144 DPI;
the positive percentage area page differs at one pixel by at most three channel
levels. Office render review remains unreviewed.

The captures retain mixed-sign percentage overflow, left legend/axis-title overlap
and long decimal tick text, as described in the [bar captures](../chart-native-bars/README.md).
Line labels use a fixed box above each marker; other requested positions warn.
Area labels are omitted with a warning. Gap and overlap have no native effect on
these kinds. The separate `unsupported` sources retain one-category line and area
cases: editable packages support their data, but native planning reports the
minimum-category limit explicitly. Fixtures are not changed to hide differences.
