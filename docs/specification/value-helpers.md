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
1234.5, 1)` accepts named decimals and grouping likewise. Global `format_money` does not interpret such a
table as named arguments. Invalid argument counts, types, units and dates produce
diagnostics.
