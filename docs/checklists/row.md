# Row

[Language](../specification/tables.md) · [Tests](../../tests/tables.rs)

[Directive index](README.md) · [Standard properties](attributes/tables.md)

- [x] One required body containing cells and separating whitespace; empty-row warning.
- [x] `repeat_header`: boolean, mapped to a repeating DOCX header flag.
- [x] `height`: nonnegative length; `height_rule`: auto, at_least, exact.
- [x] `cant_split`: boolean, mapped to a DOCX row-splitting request.
- [x] `properties`: flattened overrides and retained metadata.

Row grid omissions, revisions, hidden state, table-property exceptions and other
standard properties remain in the linked inventory without invented Mote names.

Language/model and editable DOCX support are verified by captured packages.
Native table layout is the next implementation step; Microsoft Word visual
review remains unreviewed.
