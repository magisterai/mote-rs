# Source to paragraph lines

These 273 captures apply the [91 style sources](../layout-styles/requests.json)
to 12-, 72- and 300-point paragraph containers. [Expected geometry](expected.json)
records both indentation frames and every line's breaks, bounds, baseline and
fragments. [Capture metadata](capture.json) fingerprints the sources and results.

`cargo test --test paragraph_text` compiles each source, resolves its styles and
inline runs, measures fonts and plans lines. It compares every numeric field
within 0.000000001 point and every other field exactly. The sources and expected
results remain unchanged during tests.

The narrow cases expose frame clamping, hanging indents and oversized graphemes.
The source cases include zero font sizes: the effective style retains zero, while
line measurement and placement select the 11-point measurement default. Explicit
zero line heights likewise retain the ordinary fallback line advance.

This is paragraph-relative text geometry. Vertical stacking, pagination, numbering,
paragraph borders, tab stops and painters are outside this collection. Microsoft
Word visual review remains **unreviewed**.
