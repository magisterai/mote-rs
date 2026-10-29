# Styles and fonts reference

This capture covers a declared Normal default, paragraph and character styles,
forward declarations, inheritance, next/link references, flat/nested overrides,
exact and normalized lookup, and two names that sanitize to the same Word style
identifier. Character styles clear inherited direct fields they supply; direct
child overrides win and sibling formatting remains unchanged.

Font declarations map Latin names to `w:ascii`/`w:hAnsi`, East Asian names to
`w:eastAsia`, and complex-script names to `w:cs`. Missing script mappings remain
inherited. Font-table entries use physical typeface names, including declared
script mappings; logical aliases are not written as physical fonts. A declared
`mono` alias takes precedence over the implicit Courier New mapping.
`word_line_height` is retained model metadata and does not change DOCX sizes.

[Tests](../../../docx.rs) compare the model and all captured parts. Font files,
embedding, substitution, native measurement, tables and other block output remain
outside this increment. Missing style references and cycles cause explicit output
errors. Real Word rendering and layout review remain **unreviewed**.
