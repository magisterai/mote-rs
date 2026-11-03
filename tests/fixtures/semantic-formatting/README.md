# Semantic formatting captures

Three frozen sources exercise document text, slide text and direct slide overrides.
The captures retain evaluated models, complete editable package parts and
independent native PNG pages. Asset and source digests prevent unnoticed changes.

The document covers every underline pattern, underline color and size, both strike
kinds, highlights, small capitals, script baselines and nested formatting. The
slide cases retain authored script sizes in DrawingML and exercise the native
presentation script profile separately.

PDF raster comparisons at 144 DPI are exact for all three cases.
Native PNG differences are predominantly text antialiasing; the mixed slide also
has three fractional dash-edge pixels above a 64-level channel difference. The
per-case limits in capture.json retain those measured bounds.

Native wave underlines use continuous geometry, while none removes the underline.
The changed foreground mix uses mean PNG bounds of 7.65 for the document and
5.93 for the slide; maximum differences and the three existing dash-edge outliers
are unchanged. Word semantic wrappers other than underline reject direct font,
size, color and style attributes without a mapping; surrounding spans supply
that formatting.

Real Word and PowerPoint visual review remains pending. Native render comparisons
and matching editable package structure do not establish Office review.
