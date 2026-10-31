# Mathematics captures

Frozen sources cover inline and display equations, operators, scripts, radicals,
matrices, alignment, raw Office Math, Lua constructors, inherited text formatting,
table cells, running stories, notes and positioned slide text.

Each capture stores the source digest, complete model, every Office package part,
font identities, every native text fragment and a PNG page. Text geometry is
checked within 1e-8 point independently of pixel tolerances. Captured parts are
content-addressed and shared within this directory. Tests do not need an external
implementation or executable.

`expressions`, `edges`, `matrices`, `raw-and-lua`, `stories` and `recovery` are
DOCX cases; `slides` and `recovery-slides` are PPTX. All eight PDF rasters matched
their capture inputs pixel for pixel at 144 DPI.
Native math intentionally displays italic source text, with warnings; these pages
are not mathematical typesetting evidence. Complete editable math XML is checked
separately. `edges` also covers unmatched delimiters, brace-adjacent operators,
explicit spacing, and whitespace beside inline math. The raw-input negative tests
also require valid enclosing package XML and a visible fallback when a math root
is malformed, duplicated or unrelated.

`recovery` and `recovery-slides` cover incomplete delimiter pairs, nested pairs,
arrow commands beside delimiters, malformed raw XML and unknown namespace
prefixes. Incomplete equations retain their full source in an ordinary-text math
run; malformed XML becomes escaped text outside the math structure. The `edges`
capture updates only its incomplete equation's Office XML; its source, model,
geometry and raster stay fixed.

PNG checks allow bounded antialiasing differences. `slides` has three isolated
Aptos italic intersection pixels above the ordinary 64-channel threshold, with a
maximum of 75/255. Their exact coordinates are recorded, the cap is 80/255, and no
other outlier pixel is allowed. All fragment positions match; document captures
stay below the ordinary threshold. This exception does not change any existing
fixture's tolerance.

`recovery-slides` has three isolated glyph-intersection pixels above 64, with a
maximum of 89/255 and a cap of 96/255. Its capture likewise records every allowed
coordinate. Its PDF raster is identical and every text-fragment position matches.

[Tests](../../mathematics.rs) also cover argument validation, parser depth,
unsupported-command warnings and the native fallback. Office desktop rendering
remains unreviewed.
