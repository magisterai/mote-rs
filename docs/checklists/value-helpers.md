# Value helpers

[Directive index](README.md)

Helpers return values rather than content elements; these contracts do not add
OOXML attributes. Typed lengths feed the existing length mappings, and typed
dates can supply annotation properties.

- [x] `upper(value)` and `lower(value)`: Unicode simple case conversion of the
  value's textual representation, without locale-specific or multi-letter expansion.
- [x] `format_money(amount, currency)` or named `currency`: numeric/string amount,
  default USD; two decimals, comma grouping, minus sign and uppercase currency.
- [x] `format_number(value, decimals)` or named `decimals`: numeric/string value,
  0 to 15 decimals, default 0; named `grouping` adds comma thousands separators;
  no sign on a value that rounds to zero.
- [x] `length(value, unit)`: numeric/string magnitude, default `pt`; units
  `pt`, `in`, `cm`, `mm`, `twip`. Existing unit helpers remain equivalent.
- [x] `date(value)`: a valid ISO calendar date string or existing date value;
  preserve its type through bindings and Lua; JSON displays the date string.
- [x] `days_between(start, end)`: whole days between two dates, signed.
- [x] `add_days(date, days)` and `add_months(date, months)`: whole, signed
  counts; a missing day of the month gives the last day; years 0000 to 9999.
- [x] `date_part(date, part)`: `year`, `month`, `day`, `weekday` (Monday is 1),
  `day_of_year` and ISO `week`.
- [x] `format_date(date, pattern)` or named `pattern`: default `D MMMM YYYY`;
  tokens `YYYY`, `YY`, `MMMM`, `MMM`, `MM`, `M`, `DD`, `D`, `dddd`, `ddd`; literal
  text in square brackets; English names only.
- [x] The present day: the binding `today` holds the stated date or the local
  date of the compilation; no helper reads a clock.
- [x] Equivalent global Lua and `document` helper calls; invalid arity/type/unit
  and invalid calendar dates report diagnostics.

No locale, timezone or exchange-rate options are implied, and a date is formatted
with English names only.

Evidence: [language definition](../specification/value-helpers.md),
[contract and output tests](../../tests/value_helpers.rs), and
[frozen captures](../../tests/fixtures/value-helpers/README.md).
