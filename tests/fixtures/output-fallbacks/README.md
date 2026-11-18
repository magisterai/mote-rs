# Output fallbacks

These fixed sources cover retained span whitespace, document connectors, named
anchors, chart cells and Lua-constructed connector children. Captures contain
complete evaluated models, DOCX parts and native PNGs with source/part hashes.

The separate PDF comparison used 144-DPI rasterization and was pixel-identical.
PNG uses the established raster edge allowance: maximum channel difference 64,
no pixels above 64, and mean foreground channel difference at most 12. Native
text geometry and package comparison remain independent of that allowance.

[unmapped-names.mote](unmapped-names.mote) gives a name without a Word mapping
to each construct that DOCX checks, and names an undeclared style wherever a
style may be named. DOCX ignores each with a warning and writes the package, as
the [diagnostic tests](../../docx_diagnostics.rs) require. The source has no
capture.

[unmapped-values.mote](unmapped-values.mote) gives a border style, a tab
alignment, a tab leader and a vertical cell alignment that Word does not define,
together with an underline style, a numbered heading kept with its successor,
a radical without a degree, a style with two fills and a shadow at a negative
angle, each of which was once written in a form that the schema rejects. The
[package tests](../../docx.rs) require the fallback of each and its warning, and
the validation test of the [repository tasks](../../../xtask/README.md#package-validation)
requires the package to conform. The source has no capture.

DOCX omits standalone connectors with a warning. Native document output displays
`[line]` plus retained child text. Chart cells display `[chart]` or `[chart: title]`
and retain block paragraph spacing. None of these fallbacks implies native object
painting or an Office visual review.
