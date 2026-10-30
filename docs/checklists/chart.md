# Chart

[Directive index](README.md) · [Chart properties](attributes/charts.md) ·
[Chart extensions](attributes/chart-extensions.md)

This inventory precedes the language definition and implementation. The complete
retained chart schemas include unsupported kinds and options; their presence does
not introduce Mote attributes. Chart text and appearance also inherit the
[DrawingML text](attributes/drawing-text.md), [shape and image](attributes/images.md)
and [value-domain](attributes/values.md) inventories.

- [ ] Named arguments only; required `data` property table; no body or positional
  arguments. Shared `properties` overrides and retained unknown metadata.
- [ ] `kind`: column, bar, line, pie, doughnut, stacked_column, stacked_bar,
  percent_stacked_column, percent_stacked_bar, area, stacked_area,
  percent_stacked_area. Omitted kind resolves to column in output.
- [ ] `title`: string or number normalized to text.
- [ ] `width`, `height`: length normalization and output defaults.
- [ ] `data.categories`, with `labels` as an alternate key: ordered category
  values converted to text. Generated numeric category labels when absent.
- [ ] `data.series`: ordered property tables with `name` and `values`;
  default series names, numeric conversions and differing series lengths.
- [ ] Single-series shorthand `data.values` and `data.name`.
- [ ] `legend`: boolean, position string or property table.
  `legend.enabled` and `legend.position`: right/r, left/l, top/t, bottom/b;
  kind- and series-count-dependent defaults.
- [ ] `axis.category.title`/`name` and `axis.value.title`/`name`.
- [ ] `colors`: array of colors or `series1`, `series2`, ... property table;
  shared default palette and missing-color behavior.
- [ ] `background`: normalized chart-area color.
- [ ] `border`: boolean, color, or property table with enabled, color and width.
- [ ] `plot_area`: boolean or property table; background/fill/color precedence
  and nested border properties.
- [ ] `gridlines`: boolean or property table with major, minor, color and width.
- [ ] `data_labels`: boolean or property table with enabled, value, category,
  series, percent/percentage and position. Position aliases: center/ctr,
  inside_base/inBase, inside_end/inEnd, outside_end/outEnd, best_fit/bestFit,
  left/l, right/r, top/t and bottom/b.
- [ ] `gap_width`, `overlap`: integer normalization and output range handling.
- [ ] `style`: retained name; retained metadata alone has no formatting effect.
- [ ] Lua `document.chart` uses the shared constructor and typed JSON values.

Editable chart XML, embedded workbook data, document flow, native chart geometry
and presentation placement remain pending. Output support must be reported
separately from model acceptance. Office render review is unreviewed.

Unsupported standard families remain in the linked inventories: 3D and combination
charts, scatter/bubble/radar/stock/surface charts, chart drawings, arbitrary formulas
and external data, trendlines, error bars, secondary/date axes, per-point overrides,
manual layouts, legend-entry overrides, rich label formatting, modern extended
chart families and Microsoft chart styling extensions. No new authoring names are
assigned to those properties here.
