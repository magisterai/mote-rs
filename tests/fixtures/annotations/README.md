# Control, annotation and metadata captures

`document` covers standalone and legacy checkboxes, fields, comment identities,
authors and dates, metadata replacement, and mixed formatting. `stories` checks
Lua, nested annotations and running stories. `contexts` covers tables, fields
without native text, notes inside comments and inherited small capitals. `slides`
checks retained and omitted presentation content and core metadata.
`inline-boundaries` checks whitespace on both sides of controls, fields, comments
and equations, including empty comment bodies and preserved spans.

Captures contain frozen source digests, complete models, every DOCX/PPTX part,
font identities and native text geometry, plus PNG pages. Content-addressed parts
are shared within this directory. PDF rasters matched pixel for pixel at 144 DPI;
PNG checks allow bounded antialiasing differences. Geometry comparisons use an
independent 1e-8-point tolerance, including blank cell lines.

[Tests](../../annotations.rs) also enforce unique generated comment/control
identities, metadata merge behavior and attribute/body validation. Native output
retains comment text, omits general fields and uses textual document checkboxes.
Editable package structure is tested separately from those fallbacks. Word and
PowerPoint desktop rendering remain unreviewed.
