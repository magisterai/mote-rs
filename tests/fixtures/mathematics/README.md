# Mathematics captures

Frozen sources cover inline and display equations, operators, scripts, radicals,
matrices, alignment, raw Office Math, Lua constructors, inherited text formatting,
table cells, running stories, notes and positioned slide text.

Each capture stores the source digest, complete model, every Office package part,
font identities, every native text fragment and a PNG page. Text geometry is
checked within 1e-8 point independently of pixel tolerances. Captured parts are
content-addressed and shared within this directory. Tests do not need an external
implementation or executable.

`expressions`, `edges`, `matrices`, `raw-and-lua`, `stories` are DOCX cases; `slides` is PPTX.
All six PDF rasters matched their capture inputs pixel for pixel at 144 DPI.
Native math intentionally displays italic source text, with warnings; these pages
are not mathematical typesetting evidence. Complete editable math XML is checked
separately. `edges` also covers unmatched delimiters, brace-adjacent operators,
explicit spacing, and whitespace beside inline math. The raw-input negative tests
also require valid enclosing package XML and a visible fallback when a math root
is malformed, duplicated or unrelated.

PNG checks allow bounded antialiasing differences. `slides` has three isolated
Aptos italic intersection pixels above the ordinary 64-channel threshold, with a
maximum of 75/255. Their exact coordinates are recorded, the cap is 80/255, and no
other outlier pixel is allowed. All fragment positions match; document captures
stay below the ordinary threshold. This exception does not change any existing
fixture's tolerance.

[Tests](../../mathematics.rs) also cover argument validation, parser depth,
unsupported-command warnings and the native fallback. Office desktop rendering
remains unreviewed.
