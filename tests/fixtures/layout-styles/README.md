# Native style resolution

The 91 [source requests](requests.json) freeze the mapping from Mote source to
effective paragraph properties and merged inline runs in [expected.json](expected.json).
They cover document defaults, implicit and declared styles, inheritance, normalized
style names, direct overrides, headings, spans, strong/emphasis/code, font aliases,
Word line-height calibration, colors, decorations, indentation and spacing.
[Capture metadata](capture.json) records fingerprints, units and numeric tolerance.

Run `cargo test --test layout_styles`. Every effective property and run is compared;
source fixtures are compiled during the test. The test does not rewrite expectations.

Unitless font sizes intentionally preserve a cross-format discrepancy: native style
resolution treats `size = 12` as 0.6 points, while DOCX emits 12 points. An explicit
`size = pt(12)` gives 12 points in both. Do not silently normalize the captured
behavior while reviewing another change. Zero sizes remain zero in the style;
font measurement has its separate default-size policy.

This collection verifies style resolution, including retained pagination requests.
It does not establish pagination, borders, tab-stop placement, decoration painting
or PDF/PNG generation. Microsoft Word visual review remains **unreviewed**.
