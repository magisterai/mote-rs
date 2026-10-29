# Document text placement

The [49 source requests](requests.json) contain 47 successful text plans and two
intentional native output rejections in [expected.json](expected.json). They exercise
three page heights, paragraph spacing collapse, heading-start spacing, keep-with-next
chains, keep-together, widow/orphan control, explicit page/column/line breaks,
indentation, justification, mixed font sizes, code, tabs and empty lines.

Run `cargo test --test document_text`. The test compiles complete sources and
compares page counts plus every text fragment's absolute position, font, size,
color, width, advance and missing codepoints. [Capture metadata](capture.json)
records fingerprints and tolerance. It never rewrites inputs or expectations.

Blank paragraphs and empty hard-break lines have distinct geometry. A zero-height
blank paragraph, including one under a zero default font size, is a rejected native
plan; nonempty zero-size text uses the measurement default. The two rejected cases
are marked `invalid_line_height`, separate from successful page comparisons.

This is a single-column text-placement API, not a complete painted document plan.
Lists, block quotes, heading numbering and balanced multiple columns are rejected.
Paragraph borders and fills, custom tab-stop layout, glyph painting and output
serialization remain outside this projection. Microsoft Word review is **unreviewed**.
