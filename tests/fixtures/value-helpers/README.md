# Value helper captures

`static` and `lua` exercise Unicode casing, money formatting, lengths and typed
dates through bindings and Lua round trips. Captures include source digests,
complete models and DOCX package parts, native text geometry and PNG pages.
Package parts are stored by content hash. PDF rasters matched pixel for pixel at
144 DPI; PNG comparisons retain bounded antialiasing tolerances.

`unicode-casing.json` stores every scalar with a simple case mapping and its upper
and lower results. This checks full Unicode input independently of font coverage
or locale and distinguishes simple mappings from multi-character expansions.

[Tests](../../value_helpers.rs) also check named Lua currency properties, rejected
argument types, duplicate currency arguments, invalid units and leap years.
Office rendering remains unreviewed; these helpers do not imply locale-aware
formatting or currency conversion.
