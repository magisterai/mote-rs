# Gantt charts

A **Gantt chart** draws dated tasks as bars along a time axis. The `gantt`
component of the library that Mote carries draws one from rows, as a group of
shapes that stays editable in Word and PowerPoint. The component is Lua, so a
source that uses it is compiled with `--lua`.

## In a document

```mote lua
#import("components/gantt")
#let(tasks = {
  { task = "Discovery", start = "2026-01-05", ["end"] = "2026-01-23" },
  { task = "Design", start = "2026-01-19", ["end"] = "2026-02-20", color = "#7c3aed" },
  { task = "Design sign-off", start = "2026-02-20" },
  { task = "Build", start = "2026-02-16", ["end"] = "2026-04-24", color = "#059669" }
})

The programme has three stages and one decision.

#gantt(tasks, width = inch(6.5), height = inch(2.2), today = "2026-03-10")

Build is the longest stage.
```

The chart stands in the text between the two paragraphs and takes the stated
size.

## Rows

The first argument is an array of rows. A row is an object with these fields:

- `task`, or `name`: the label of the row.
- `start`: a date or a `YYYY-MM-DD` string; required.
- `end`, or `finish`: the last day of the task. A row without one is a
  **milestone**, drawn as a diamond on its day.
- `color`: the color of the bar or diamond.

The rows may come from data, as `#gantt(json("tasks.json"))` or
`#gantt(csv("tasks.csv"))`, where the fields are the keys or the column names.
In a Lua table the field `end` is written `["end"]`, since `end` is a word of
Lua; `finish` needs no brackets.

## Arguments

- `width` and `height` are the size of the chart. The default width is 6.5
  inches, and the default height gives each row 24 points.
- `x` and `y` place the chart: they are required on a slide, and in a document
  they fix the chart on the page in place of setting it in the text.
- `alignment` is `left`, `center` or `right`, for a chart in the text.
- `today` is a date, at which a red line is drawn; the binding `today` holds
  the date of the compilation.
- `color` is the color of every bar that states none.
- `label_width` is the width of the column of labels, by default about a
  quarter of the chart, and `text_size` the size of the text.

The axis is divided into days, weeks, months, quarters or years, by the span of
the rows, and the text, the rows and the bars are sized to the chart.

## On a slide

```mote lua
#import("components/gantt")
#presentation(size = "wide")
#slide[
  #box(position = { x = inch(0.7), y = inch(0.5), width = inch(12), height = inch(0.9) })[# Delivery plan]
  #gantt({
    { task = "Pilot", start = "2026-03-02", finish = "2026-04-10" },
    { task = "Review", start = "2026-04-13" },
    { task = "Rollout", start = "2026-04-13", finish = "2026-06-26" }
  }, x = inch(0.7), y = inch(1.6), width = inch(12), height = inch(4.5))
]
```

## Limits

- A row has one bar: the component draws no dependency, progress or grouping.
- A chart is not divided between pages.
- A long label is reduced to fit its column in PDF and PNG; Word and PowerPoint
  may let it pass the column, so a wider `label_width` serves a long label.
