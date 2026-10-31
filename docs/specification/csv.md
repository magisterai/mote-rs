# CSV data

`csv("data/people.csv")` reads comma-separated records through the selected
source-relative asset resolver. Like `json`, it returns a value that can be bound,
iterated or passed to a directive. It accepts one required string path, the
optional named argument `heading`, and no content body.

The first record supplies column names. `heading = true` skips one title record
before reading those names; it does not enable headers. The default is `false`.
Boolean strings use the same rules as other boolean properties.

Names are trimmed. An empty name becomes `column_1`, `column_2`, and so on by its
one-based column position. Repeated names receive `_2`, `_3`, and later suffixes.
A supplied name that collides with such a generated name can overwrite the earlier
field in the row object; use distinct headers when preserving every field matters.

The result is an array of row objects whose values are strings. Missing fields
become empty strings; fields beyond the header width are ignored. Empty input,
a header without data, or a skipped title without a header produces an empty
array. Blank lines are ignored outside quoted fields; whitespace-only lines are
records.

Double quotes enclose fields containing commas or newlines. Inside a quoted
field, doubled quotes represent one quote. CRLF becomes LF, including in quoted
fields. Quotes in unquoted fields, an unclosed quote, or content between a closing
quote and the separator are errors. Values are not trimmed or converted to numbers.
CSV values passed to chart directives follow those directives' existing conversion
rules.

With Lua enabled, `csv("people.csv")` and
`document.csv({heading = true}, "people.csv")` use the same loader. Files are
snapshotted for the compilation, including when requested by an imported function.

See the [support checklist](../checklists/csv.md).

For `people.csv` containing `Name,Role` followed by `Ada,Author` and
`Lin,Reviewer`, iterate over the returned objects:

```mote
#let(rows = csv("people.csv"))
#for person in rows {
  #paragraph[#person.Name: #person.Role]
}
```

With Lua enabled, `titled.csv` may begin with a `Team roster` record before
the `Name,Role` header and an `Ada,Author` row:

```mote
#let(rows = document.csv({ heading = true }, "titled.csv"))
#for person in rows {
  #paragraph[#person.Name: #person.Role]
}
```
