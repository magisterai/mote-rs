# Row

[Language](../specification/tables.md) · [Model/DOCX tests](../../tests/tables.rs) · [Native tests](../../tests/table_rendering.rs)

[Directive index](README.md) · [Standard properties](attributes/tables.md)

- [x] One required body containing cells and separating whitespace; empty-row warning.
- [x] `repeat_header`: boolean, mapped to a repeating DOCX header flag.
- [x] `height`: nonnegative length; `height_rule`: auto, at_least, exact.
- [x] `cant_split`: boolean, mapped to a DOCX row-splitting request.
- [x] `properties`: flattened overrides and retained metadata.

Row grid omissions, revisions, hidden state, table-property exceptions and other
standard properties remain in the linked inventory without invented Mote names.

Language/model and editable DOCX support are verified by captured packages.

Native PDF/PNG repeats contiguous leading header rows and honors `cant_split`.
Ordinary rows and merged groups can continue across pages and columns at safe
content boundaries. Oversized atomic rows warn about clipping; an oversized merged
group without a safe split warns and falls back to individual rows.
`height` and `height_rule` affect editable DOCX only and produce native support warnings.

Microsoft Word visual review remains unreviewed.
