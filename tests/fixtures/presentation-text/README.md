# Positioned presentation text

These fixed sources cover explicit slide boundaries, wide/standard/custom page
sizes, solid backgrounds, theme colors and fonts, editable paragraphs, headings,
styles, lists, task marks, links, code, margins, alignment, clipping, fills and
borders. Empty, zero-size and off-canvas boxes remain visible as boundary cases.
Lua constructs the same model as direct directives.

Each capture includes the evaluated JSON model, a source SHA-256, and a manifest
of every XML package part. Identical parts share content-addressed files. Tests
compare complete canonical XML, including relationships, themes, masters and
layouts; only volatile core timestamps are ignored. Package equality does not
establish PowerPoint layout conformance. These cases are unreviewed in PowerPoint.

Text shrinking and shape resizing, shadows, automatic slide layout, background
images, footer fields, speaker notes and non-text boxes are outside this initial
editable-output slice and fail explicitly when requested.
