# JSON data

[Directive index](README.md)

`json` is a value helper without an OOXML attribute family. Data feeds the
attribute inventories of the receiving directives.

- [x] One required positional string path; no named arguments or body.
- [x] Objects, arrays, strings, numbers, booleans and null become typed values.
- [x] Duplicate object keys use the last value; non-finite numbers are rejected.
- [x] Relative paths resolve against the referring source; standard input uses the working directory.
- [x] Custom resolvers supply bytes and logical names without filesystem fallback.
- [x] Repeated reads use one compilation snapshot; conflicting bytes for one logical name fail.
- [x] Resolution and JSON errors identify the source expression and data resource.
- [x] Direct helper content uses the same textual representation as a bound value.

JSON object keys remain data keys; this helper does not invent attribute mappings
or bypass the receiving directive's validation. No network fetch is implied by a
URI; external resources require a custom resolver.

Evidence: [language](../specification/json-data.md),
[resource evaluation](../../src/evaluation/data.rs), and
[resolver, CLI and captured-output tests](../../tests/json_data.rs).
The model and DOCX captures do not establish Office visual review.
