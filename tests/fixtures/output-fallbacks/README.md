# Output fallbacks

These fixed sources cover retained span whitespace, document connectors, named
anchors, chart cells and Lua-constructed connector children. Captures contain
complete evaluated models, DOCX parts and native PNGs with source/part hashes.

The separate PDF comparison used 144-DPI rasterization and was pixel-identical.
PNG uses the established raster edge allowance: maximum channel difference 64,
no pixels above 64, and mean foreground channel difference at most 12. Native
text geometry and package comparison remain independent of that allowance.

DOCX omits standalone connectors with a warning. Native document output displays
`[line]` plus retained child text. Chart cells display `[chart]` or `[chart: title]`
and retain block paragraph spacing. None of these fallbacks implies native object
painting or an Office visual review.
