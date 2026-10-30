# Charts

[Specification index](README.md) · Previous: [Shapes and lines](drawings.md)

A **chart** is a block element that associates categories and numeric series with
a chart kind and appearance properties. `chart` takes named arguments, requires
a `data` property table, and accepts no content body or positional arguments.
The evaluated model retains chart data and normalized formatting attributes.
Output support is recorded separately in the [chart checklist](../checklists/chart.md).

## Data and chart kinds

A **category** labels a position in a series. A **series** has a name and an ordered
array of values. The conventional data form uses `categories` for the category
array and `series` for an array of property tables; each series table contains
`name` and `values`. Values can come from the bindings, JSON resources, or Lua
constructors defined in earlier chapters.

```mote
#chart(kind = "column", title = "Quarterly revenue", data = {
  categories = {"Q1", "Q2"},
  series = {{name = "Revenue", values = {10, 12}}}
})
```

This produces one chart element with no children. Its data retains both labels
and the named values array. Data contents are not normalized during evaluation:
strings remain strings, array lengths remain unchanged, and extra data fields are
retained. Model acceptance alone does not validate that data can be plotted.

The kinds are `column`, `bar`, `line`, `pie`, `doughnut`, `stacked_column`,
`stacked_bar`, `percent_stacked_column`, `percent_stacked_bar`, `area`,
`stacked_area`, and `percent_stacked_area`. Kind names are case-sensitive.
An omitted kind remains absent in the evaluated model and selects `column` when
resolved for output. Data interpretation is separate from argument normalization.

## Resolving data

`data.categories` supplies category labels; `data.labels` is used when categories
is not an array. Category values become text without trimming. When the selected
array is empty or absent, labels `1`, `2`, and so on cover the longest series.

`data.series` supplies ordered series tables. Entries without a `values` array are
skipped. Series names are trimmed; an empty or absent name becomes `Series N`,
where N is the entry's original one-based position. If `series` is not an array,
`data.values` supplies one series named by `data.name`, defaulting to `Series 1`.
An existing series array takes precedence over this shorthand, even when empty.

Numeric values remain numeric; numeric text is parsed after trimming. Other
unparseable values become zero. Non-finite values are rejected during output.
Arrays retain their different lengths: use one value per category when the
editable workbook and chart cache must describe identical data.

## Dimensions and appearance

`title` accepts a string or number and normalizes it to text. `width` and `height`
use the established non-negative length rules; explicit units avoid ambiguity
between output coordinate systems. Chart dimensions default to 6 inches by 3.5
inches; unitless drawing dimensions use EMUs (914,400 per inch).
`background` accepts a hex color.

`colors` accepts an array of colors or a property table of colors. Every member
is normalized, including additional property-table keys. `axis` accepts a property
table and retains its contents unchanged. `style` accepts a string and is retained;
its presence does not establish a formatting effect. Arrays choose colors in
order. Property tables use consecutive `series1`, `series2`, and so on, stopping
at the first missing entry. Remaining colors use the repeating palette `#2563EB`,
`#059669`, `#D97706`, `#DC2626`, `#7C3AED`, `#0891B2`. Pie and doughnut colors
apply to points in the first series. Axis tables use `category.title` and
`value.title`, with `name` as a fallback for an empty title.

`border` accepts a boolean, color, or property table. Its table form normalizes
`enabled` as a boolean, `color` as a color, and `width` as a length. `plot_area`
accepts a boolean or property table; its table normalizes `background`, `fill`,
and `color` as colors, and `border` by the same border rules. Chart and plot areas default to no fill
and no border. An enabled border defaults to one point and `#D1D5DB`. In a plot
area table, the first nonempty background/fill/color supplies its fill. A boolean
plot area has no appearance effect.

`gridlines` accepts a boolean or property table. Its table normalizes `major` and
`minor` as booleans, `color` as a color, and `width` as a length. `gap_width` and
`overlap` accept signed integers or integer strings and normalize to numbers;
evaluation does not apply an output-specific percentage range. Editable bar and
column charts retain authored gap and overlap values without clamping; stacked
variants default to 100% overlap. Omitted gridlines enable major lines and disable
minor lines; a boolean controls major lines.

```mote
#chart(data = {categories = {"A", "B"}, series = {{values = {4, 7}}}},
       width = inch(6), height = inch(3),
       colors = {"#2563eb", "#059669"}, background = "#fff",
       border = {color = "#cbd5e1", width = pt(1)},
       plot_area = {fill = "#f8fafc", border = false},
       gridlines = {major = true, minor = false}, gap_width = "75", overlap = -25)
```

The model retains typed dimensions, normalized colors, the nested property tables,
and numeric gap/overlap values. Additional nested metadata is retained without an
implied formatting effect.

## Legend and labels

`legend` accepts a boolean, the strings `true` or `false`, a position string, or a
property table. Input positions are `right`, `left`, `top`, and `bottom`; they
normalize to `r`, `l`, `t`, and `b`. The shortened forms are model values and are
not accepted as legend input positions. Without a legend attribute, pie and
doughnut charts show a legend, and other charts show one when more than one series
exists. Line and area kinds place it at the top; others use the bottom. A legend
table defaults to enabled and normalizes `enabled` as a
boolean and `position` by the same position rules.

`data_labels` accepts a boolean or property table. Its table normalizes `enabled`,
`value`, `category`, `series`, `percent`, and `percentage` as booleans. Both percentage
keys remain distinct in the model and either can enable percentages. A true
boolean enables value labels. A property table defaults to enabled; when it
selects no value/category/series/percentage component, value labels are enabled.
Position names normalize as follows; each
normalized value is also accepted as label input.

| Label position | Normalized value |
| --- | --- |
| `center` | `ctr` |
| `inside_base` | `inBase` |
| `inside_end` | `inEnd` |
| `outside_end` | `outEnd` |
| `best_fit` | `bestFit` |
| `left`, `right`, `top`, `bottom` | `l`, `r`, `t`, `b` |

```mote
#chart(data = {series = {{name = "Total", values = {4, 7}}}},
       legend = {enabled = true, position = "bottom"},
       data_labels = {value = true, category = true, position = "outside_end"})
```

The legend position becomes `b` and label position becomes `outEnd`. These are
normalized authoring intentions; output support does not follow from storing them.

## Overrides and constructors

As with other content directives, named arguments are type-checked before
`properties` overrides are merged. The resulting designated formatting fields
are then normalized. `data` must be supplied as a named argument even when a
`properties` table also contains it. Data and axis contents remain unchanged by
normalization. An override can therefore retain values that output cannot use.

With Lua enabled, `document.chart` constructs the same element from a property
table. It accepts no body arguments. Its data can use previously loaded JSON
objects without a second parsing or conversion rule.
