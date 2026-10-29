# Inline decoration geometry

The 90 [source requests](requests.json) and [expected operations](expected.json)
cover highlight rectangles, underlines, strikes and double strikes across three
font sizes, multiple/exact/minimum line spacing, two widths, justified lines and
mixed typefaces. The expectations include operation order, merged adjacent areas,
colors and stroke thickness. [Capture metadata](capture.json) records fingerprints.

Run `cargo test --test text_decorations`. Every operation is compared numerically
within 0.000000001 point, with exact colors and kinds. An additional test checks
that highlights and strokes move with text into absolute page coordinates.

These are painter-ready geometric operations. Rasterization, PDF serialization,
paragraph shading/borders and additional underline styles remain outside this
collection. Microsoft Word visual review remains **unreviewed**.
