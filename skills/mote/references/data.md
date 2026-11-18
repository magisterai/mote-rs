# Data, conditions and loops

A **binding** gives a name to a value. A value is a string, number, boolean,
length, color, array or object, and may be read from a JSON or CSV file. A
**condition** includes content when a value is true, and a **loop** repeats
content for each member of an array or object. None of these requires Lua.

## Bindings

```mote
#let(client = "Acme", total = 1234.5, accent = color("#2563eb"), report = { quarter = "Q3", totals = { 10, 20 } })
#let(second = report.totals.1)

#heading(level = 1)[Report for #client]

The #report.quarter total is #span(color = accent)[#format_money(total)], and the second figure is #second.
```

- The form `#name` inserts a value, and `#name.field` a field of an object.
- An array index in a path counts from zero, and is written in an expression,
  as in `report.totals.1`; the inserted form `#name.0` is not an index.
- A binding made inside a directive body is local to that body.

## Files

```mote
#let(sales = json("sales.json"), rows = csv("sales.csv"))

#chart(kind = "line", title = sales.title, data = sales.chart)

#table[
  #row(repeat_header = true)[#cell[**Region**] #cell[**Revenue**]]
  #for region in sales.regions {#row[#cell[#region.name] #cell[#region.revenue]]}
]

#for row in rows {#paragraph[#row.region: #row.revenue million, up #row.change%.]}
```

- The function `json(path)` reads a JSON file, and `csv(path)` reads a CSV file
  as an array of objects, one for each record, keyed by the header row; every
  CSV value is a string. A path is relative to the source file.
- The function `#import("shared.mote")` evaluates another source and makes its
  bindings, defaults, styles and fonts available, without inserting its text.

## Conditions and loops

```mote
#let(sales = json("sales.json"))

#for region in sales.regions {
  #paragraph[#region.name: #if(region.paid)[paid] else [awaiting payment].]
}
```

- The form `#if(value)[content] else [other content]` includes one branch. The
  values `nil`, `false`, zero, an empty string, an empty array or object and a
  zero length are false; every other value, including the string `"false"`, is
  true.
- The form `#for name in value {content}` repeats its content, which is Mote
  markup, for each member. A loop may generate rows, cells, list items and
  paragraphs where the enclosing directive accepts them.

## Value helpers

- `upper(text)` and `lower(text)` change case.
- `format_number(value, decimals)` rounds a number, as in
  `format_number(1234.5, 1, grouping = true)`, which gives `1,234.5`; grouping is
  off by default.
- `format_money(amount, currency)` gives `$1,234.50`, or `EUR 1,234.50` for
  another currency.
- `date("2026-11-04")` is a date, and `length(12, "pt")` a length.

## Dates

A date helper takes a date or a `YYYY-MM-DD` string. The binding `today` holds
the date of the compilation, as in `#days_between(today, finish)`; a document
that must not change from day to day states its own date instead.

```mote
#let(start = date("2026-01-31"), finish = "2026-03-14")

The work runs from #format_date(start) to #format_date(finish, "ddd D MMM"):
#days_between(start, finish) days, reviewed on #add_days(start, 14) and again on
#add_months(start, 1), in week #date_part(finish, "week").
```

- `days_between(start, end)` is a whole number, negative if `end` is earlier.
- `add_days(date, count)` and `add_months(date, count)` give a date; a month
  without the day gives its last day, so the second review above is `2026-02-28`.
- `date_part(date, part)` gives a number; the part is `year`, `month`, `day`,
  `weekday` (1 for Monday to 7 for Sunday), `day_of_year` or `week`.
- `format_date(date, pattern)` gives text, by default `31 January 2026`. The
  tokens are `YYYY`, `YY`, `MMMM` (January), `MMM` (Jan), `MM`, `M`, `DD`, `D`,
  `dddd` (Saturday) and `ddd` (Sat); text in square brackets stands as written.

## Limits

- Arithmetic and comparison, such as `total * 1.2` or `count > 3`, require Lua,
  described in [Lua](lua.md).
- A `#section` generated inside a condition or loop also repeats its body as
  ordinary content; sections belong at the top level.
