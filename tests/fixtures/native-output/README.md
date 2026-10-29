# Native output captures

These sources exercise core fonts, declared TrueType and CFF fonts, page breaks,
wrapping, justification, whitespace, small caps, highlights and decorations.
The declared fonts reuse the licensed measurement fixtures. The expected native
PNG pages are frozen at 72 DPI; their source and image fingerprints are in
[capture.json](capture.json).

`cargo test --test native_output` checks deterministic generation, PNG dimensions,
resolution metadata, allocation limits, font diagnostics and declared-font raster
comparisons. The raster comparison permits at most 48/255 channel difference and
8/255 mean error across foreground pixels to account for edge coverage differences
between rasterizers. Page dimensions must match exactly. This is separate from
the much tighter numeric geometry checks and does not excuse line or page changes.
Core-font outlines are platform-dependent; use declared fonts for portable raster
comparisons.

PDF validation uses `qpdf --check`, `pdftotext` and `pdftoppm -r 144 -png` on generated
files. The focused PDF pages matched their captured counterparts pixel for pixel.
`blocks.mote` adds lists, task markers, heading numbering, quote text and paragraph
fills/borders; `columns.mote` adds natural column flow, separators, and explicit
breaks. Core-font outlines remain platform dependent, so their PNG review is local.
These are native-output checks; Word and PowerPoint remain unreviewed. Custom tab
stops and broader document structures are outside the current painter scope.
