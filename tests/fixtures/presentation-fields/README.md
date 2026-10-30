# Slide fields and speaker notes

Twelve fixed slides cover inherited and local footer text, fixed dates,
one-based slide numbers, independent false overrides, footer=true, whitespace,
XML escaping, fractional canvas sizes, constrained footer text, plain multiline
notes, slides without notes and Lua construction. The mixed case combines image
backgrounds, foreground pictures, tables, shapes, connectors, a chart, footer
fields and notes to exercise layering, shape IDs and package relationships.

Each source retains a model, source/font/image hashes, every canonical PPTX part
(including a nested chart workbook), ordered native geometry and independent PNG
captures. Geometry matches within 1e-8. All twelve native PDFs match their
144-DPI captures exactly; PNG differences have fixed per-case channel/mean bounds.
Every native page was inspected. PowerPoint visual review remains unreviewed.

Automatic dates have separate fixed-calendar unit tests and an end-to-end test
for editable datetime4 fields, unique field identifiers and native text. A current
date comparison also matched complete packages and three native PDF pages.
Calendar-dependent captures are not permanent golden files. Notes retain text in
notes-slide parts with a shared notes master; a test verifies that removing notes
does not change native slide geometry. Empty notes create no package parts.

Editable field coordinates use integer drawing units; native field layout uses
point coordinates before conversion. The fractional canvas fixture preserves
that rounding boundary. Long footer text clips inside its fixed-height frame.
