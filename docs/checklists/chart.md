# Chart

[Directive index](README.md) · [Chart properties](attributes/charts.md) ·
[Chart extensions](attributes/chart-extensions.md)

The [chart language](../specification/charts.md) and
[model capture tests](../../tests/charts.rs) cover argument normalization and complete editable packages. The complete
retained chart schemas include unsupported kinds and options; their presence does
not introduce Mote attributes. Chart text and appearance also inherit the
[DrawingML text](attributes/drawing-text.md), [shape and image](attributes/images.md)
and [value-domain](attributes/values.md) inventories.

- [x] Named arguments only; required `data` property table; no body or positional
  arguments. Shared `properties` overrides and retained unknown metadata.
- [x] `kind`: column, bar, line, pie, doughnut, stacked_column, stacked_bar,
  percent_stacked_column, percent_stacked_bar, area, stacked_area,
  percent_stacked_area. Omitted kind stays absent in the model and resolves to column.
- [x] `title`: string or number normalized to text.
- [x] `width`, `height`: non-negative length normalization.
- [x] Width and height output defaults and coordinate interpretation.
- [x] `data.categories`, with `labels` as an alternate key: ordered category
  values converted to text. Generated numeric category labels when absent.
- [x] `data.series`: ordered property tables with `name` and `values`;
  default series names, numeric conversions and differing series lengths.
- [x] Single-series shorthand `data.values` and `data.name`.
- [x] `legend`: boolean, position string or property table.
  `legend.enabled` and `legend.position`: right→r, left→l, top→t, bottom→b.
  The short legend positions are output model values, not accepted inputs.
- [x] Kind- and series-count-dependent legend output defaults.
- [x] `axis.category.title`/`name` and `axis.value.title`/`name`.
- [x] `colors`: arrays and property tables normalize every color member.
- [x] `series1`, `series2`, ... lookup; default palette and missing-color behavior.
- [x] `background`: normalized chart-area color.
- [x] `border`: boolean, color, or property table with enabled, color and width.
- [x] `plot_area`: boolean or property table; background/fill/color normalization
  and nested border properties.
- [x] `gridlines`: boolean or property table with major, minor, color and width.
- [x] `data_labels`: boolean or property table with enabled, value, category,
  series, percent/percentage and position. Position aliases: center/ctr,
  inside_base/inBase, inside_end/inEnd, outside_end/outEnd, best_fit/bestFit,
  left/l, right/r, top/t and bottom/b.
- [x] `gap_width`, `overlap`: signed integer normalization.
- [x] Gap and overlap output range handling.
- [x] `style`: retained name; retained metadata alone has no formatting effect.
- [x] Lua `document.chart` uses the shared constructor and typed JSON values.

Data and axis tables remain unchanged in the evaluated model and resolve through
one shared chart model during output. DOCX contains editable chart XML, cached
values and an embedded workbook, with complete package capture tests across all
twelve kinds. Doughnut label positions are omitted from editable XML so Office
can choose their placement. Editable pie/doughnut charts use the first series; the
workbook retains every series. Workbook rows follow category count, padding short
series with zeros and omitting excess values; caches retain original lengths.
Keep arrays aligned to avoid that editing discrepancy. Native document PDF/PNG supports column and bar kinds, including stacked and
percentage variants, titles, axis titles, legends, fills, borders, gridlines and
data labels. Width fits the current column while preserving aspect ratio; zero
dimensions omit the chart. Native gap is clamped at zero, horizontal bars ignore
overlap, and pie/doughnut charts use positive values for sector angles, radial labels and
category legends. Pie label-position overrides use automatic placement with a
warning; doughnut centers use the plot, chart or default white background. Native line and area kinds share axes,
scales and legends; line charts include circular markers and labels above points.
Area charts omit data labels with a warning. Nonempty line/area series require
at least two categories. Gap and overlap have no native line/area effect. Native charts in table cells are
rejected; flattened running stories omit chart geometry with a warning.
[Bar captures](../../tests/fixtures/chart-native-bars/README.md) and
[line/area captures](../../tests/fixtures/chart-native-curves/README.md), and
[pie/doughnut captures](../../tests/fixtures/chart-native-pie/README.md) separately
record geometry, pixel comparisons and retained layout limitations.
Positioned presentation placement, shared theme defaults, editable workbooks
and native painting have [captured coverage](../../tests/fixtures/presentation-charts/README.md).
Native percentage stacks normalize by the sum of absolute category values and
retain each sign. All-zero categories paint no area. Chart labels fit their
allocated frames; bar labels paint above all series marks. Top/bottom legends
wrap into rows and side legends reserve measured label heights. A left legend
has separate space beside the value-axis title. These behaviors have
[combined captures](../../tests/fixtures/rendering-audit/README.md).
Office render review is unreviewed.

Unsupported standard families remain in the linked inventories: 3D and combination
charts, scatter/bubble/radar/stock/surface charts, chart drawings, arbitrary formulas
and external data, trendlines, error bars, secondary/date axes, per-point overrides,
manual layouts, legend-entry overrides, rich label formatting, modern extended
chart families and Microsoft chart styling extensions. No new authoring names are
assigned to those properties here.

Positioned slide chart output requires box geometry, editable chart parts and
embedded workbooks, all twelve existing chart kinds, and the authored title,
axes, legends, labels, fills, borders, gridlines, palette, gap and overlap fields.
Deck palette and text defaults must also reach native chart layout. Presentation
captures cover these fields separately from the existing document captures.
