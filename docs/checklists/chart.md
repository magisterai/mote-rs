# Chart

[Directive index](README.md) · [Chart properties](attributes/charts.md) ·
[Chart extensions](attributes/chart-extensions.md)

The [chart language](../specification/charts.md) and
[model capture tests](../../tests/charts.rs) cover argument normalization. The complete
retained chart schemas include unsupported kinds and options; their presence does
not introduce Mote attributes. Chart text and appearance also inherit the
[DrawingML text](attributes/drawing-text.md), [shape and image](attributes/images.md)
and [value-domain](attributes/values.md) inventories.

- [x] Named arguments only; required `data` property table; no body or positional
  arguments. Shared `properties` overrides and retained unknown metadata.
- [x] `kind`: column, bar, line, pie, doughnut, stacked_column, stacked_bar,
  percent_stacked_column, percent_stacked_bar, area, stacked_area,
  percent_stacked_area. Omitted kind stays absent in the model; output default pending.
- [x] `title`: string or number normalized to text.
- [x] `width`, `height`: non-negative length normalization.
- [ ] Width and height output defaults and coordinate interpretation.
- [ ] `data.categories`, with `labels` as an alternate key: ordered category
  values converted to text. Generated numeric category labels when absent.
- [ ] `data.series`: ordered property tables with `name` and `values`;
  default series names, numeric conversions and differing series lengths.
- [ ] Single-series shorthand `data.values` and `data.name`.
- [x] `legend`: boolean, position string or property table.
  `legend.enabled` and `legend.position`: right→r, left→l, top→t, bottom→b.
  The short legend positions are output model values, not accepted inputs.
- [ ] Kind- and series-count-dependent legend output defaults.
- [ ] `axis.category.title`/`name` and `axis.value.title`/`name`.
- [x] `colors`: arrays and property tables normalize every color member.
- [ ] `series1`, `series2`, ... lookup; default palette and missing-color behavior.
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
- [ ] Gap and overlap output range handling.
- [x] `style`: retained name; retained metadata alone has no formatting effect.
- [x] Lua `document.chart` uses the shared constructor and typed JSON values.

Data and axis property tables are currently retained unchanged; data interpretation
and output defaults remain pending. Editable chart XML, embedded workbook data,
document flow, native chart geometry and presentation placement remain pending. Output support must be reported
separately from model acceptance. Office render review is unreviewed.

Unsupported standard families remain in the linked inventories: 3D and combination
charts, scatter/bubble/radar/stock/surface charts, chart drawings, arbitrary formulas
and external data, trendlines, error bars, secondary/date axes, per-point overrides,
manual layouts, legend-entry overrides, rich label formatting, modern extended
chart families and Microsoft chart styling extensions. No new authoring names are
assigned to those properties here.
