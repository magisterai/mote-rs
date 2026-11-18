# Value helpers

Helpers return values that can be displayed, bound with `let`, or supplied to
attributes. They do not create formatting elements.

`upper(value)` and `lower(value)` convert the textual value using Unicode simple
case mappings. They preserve one character per input character and do not apply
locale-specific rules or expand a character into several letters. For example,
`upper("Straße")` is `STRAßE`, and `lower("İ")` is `i`.

`format_money(amount, currency)` accepts a finite number or numeric string. The
currency defaults to `USD` and can instead be supplied as named `currency`.
Supplying it both ways is invalid. The result uses two decimal places, comma
thousands separators, and a leading minus sign for negative amounts. USD uses
`$`; other currencies use their uppercase code followed by a space. This helper
formats an amount; it does not convert currencies or use locale conventions.

`format_number(value, decimals)` accepts a finite number or numeric string and
rounds it to `decimals` places, from 0 to 15, by default 0. The decimals can
instead be supplied as named `decimals`; supplying them both ways is invalid.
Named `grouping=true` separates thousands with commas; by default digits are not
grouped, so that years and identifiers print as written. A negative value has a
leading minus sign unless it rounds to zero. Rounding is exact for the stored
binary value, so `format_number(1.005, 2)` is `1.00`.

`length(value, unit)` creates a typed length from a finite number or numeric
string. The unit defaults to `pt`; the other units are `in`, `cm`, `mm`, and
`twip`. This is the general form of the existing unit helpers. Numeric strings
must not contain surrounding whitespace.

`date(value)` accepts a valid `YYYY-MM-DD` calendar date string or an existing
date value. It trims surrounding whitespace and checks month lengths and leap
years. The result retains its date type through bindings and Lua; displaying it
or inspecting JSON produces the ISO string. A date has no time or timezone.

The date helpers below accept a date value or a valid ISO date string wherever a
date is required. None of them reads a clock; the present day is the
[binding `today`](bindings.md).

`days_between(start, end)` is the whole number of days from `start` to `end`,
negative when `end` is the earlier date.

`add_days(date, days)` and `add_months(date, months)` return the date a whole
number of days or calendar months later, or earlier for a negative number. A
month that lacks the day of the original date gives its last day, so one month
after `2026-01-31` is `2026-02-28`. A result outside the years 0000 to 9999 is an
error.

`date_part(date, part)` returns one number of the date. The part is `year`,
`month` (1 to 12), `day` (1 to 31), `weekday` (1 for Monday to 7 for Sunday),
`day_of_year` (1 to 366) or `week`, the ISO 8601 week number.

`format_date(date, pattern)` returns text. The pattern, which may instead be
supplied as named `pattern`, defaults to `D MMMM YYYY`. The tokens are `YYYY` and
`YY` for the year in four or two digits; `MMMM` and `MMM` for the English month
name in full or in three letters; `MM` and `M` for the month number with or
without a leading zero; `DD` and `D` likewise for the day; and `dddd` and `ddd`
for the English weekday name in full or in three letters. The longest token at
each place is replaced. Text in square brackets and every other character stand
as written, so `[Day] D` gives `Day 4`. An unclosed square bracket is an error.

```mote
#let(review_date=date("2024-02-29"))
#paragraph(spacing_after=length(12,"pt"))[
#upper("café") costs #format_money(1234.5,currency="EUR") for
#format_number(1234.5,1,grouping=true) units.
Reviewed #review_date.
]
```

Lua global helpers accept positional arguments. The same helpers are available
under `document`; `document.format_money({currency="EUR"}, 1234.5)` accepts the
named currency property table, and `document.format_number({grouping=true},
1234.5, 1)` accepts named decimals and grouping likewise, as
`document.format_date({pattern="D MMM"}, value)` accepts a named pattern. Global `format_money` does not interpret such a
table as named arguments. Invalid argument counts, types, units and dates produce
diagnostics.
