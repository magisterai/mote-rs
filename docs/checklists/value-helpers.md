# Value helpers

[Directive index](README.md)

Helpers return values rather than content elements; these contracts do not add
OOXML attributes. Typed lengths feed the existing length mappings, and typed
dates can supply annotation properties.

- [x] `upper(value)` and `lower(value)`: Unicode simple case conversion of the
  value's textual representation, without locale-specific or multi-letter expansion.
- [x] `format_money(amount, currency)` or named `currency`: numeric/string amount,
  default USD; two decimals, comma grouping, minus sign and uppercase currency.
- [x] `length(value, unit)`: numeric/string magnitude, default `pt`; units
  `pt`, `in`, `cm`, `mm`, `twip`. Existing unit helpers remain equivalent.
- [x] `date(value)`: a valid ISO calendar date string or existing date value;
  preserve its type through bindings and Lua; JSON displays the date string.
- [x] Equivalent global Lua and `document` helper calls; invalid arity/type/unit
  and invalid calendar dates report diagnostics.

No locale, timezone, exchange-rate or calendar-formatting options are implied.

Evidence: [language definition](../specification/value-helpers.md),
[contract and output tests](../../tests/value_helpers.rs), and
[frozen captures](../../tests/fixtures/value-helpers/README.md).
