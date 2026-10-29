# Tab checklist

[Checklist index](README.md)

## Content and attributes

- [x] Inline `tab` element with no positional arguments or content body.
- [x] Named metadata and shared `properties` merging; no formatting normalization.
- [x] Tab remains in its paragraph, including at its edges and between formatted runs.
- [x] Paragraph tab-stop attributes determine alignment; the tab itself has no stop position.

## Specification inventory

A Word run tab is an empty `w:tab` element (`w:CT_Empty`), with no attributes.
Paragraph stop definitions are fully inventoried in
[Word paragraph tabs](attributes/word-paragraphs.md#w-ct_tabs) and their nested
[tab-stop properties](attributes/word-paragraphs.md#w-ct_tabstop).
The related presentation properties are in
[DrawingML text](attributes/drawing-text.md). They do not grant attributes to `tab`.

## Output and evidence

[Language and output tests](../../tests/tabs.rs) cover the
[definition](../specification/inline-text.md#explicit-tabs), argument boundaries,
metadata, paragraph stops, edge trimming and preserved code. Ordinary edge tabs
are trimmed in DOCX; the model retains them. Output validation runs before edge
trimming so unsupported metadata is still reported. Native layout and real Word
visual review remain unreviewed.
