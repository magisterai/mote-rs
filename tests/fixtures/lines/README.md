# Text line geometry

These 342 frozen cases compare native line breaks and fragment geometry in points.
They cover width and first-line indentation, left/center/right/justified alignment,
multiple/exact/minimum spacing, mixed sizes, core and declared fonts, kerning,
small caps, code, tabs, preserved whitespace, CRLF across runs, Unicode line breaks,
combining characters, emoji clusters, Hangul and half-width Katakana.

[requests.json](requests.json) supplies the inputs. [expected.json](expected.json)
records each line's geometry and break kind, then each fragment's prepared text,
position, dimensions, advance, size and missing codepoints. Break kinds are
0 (final), 1 (soft) and 2 (hard). [capture.json](capture.json) records fingerprints,
units and the absolute numeric tolerance; all nonnumeric fields compare exactly.
The two declared fonts reuse the licensed [measurement assets](../measurements/README.md).

Run `cargo test --test line_layout` to compare every field. To independently emit
candidate geometry without reading expectations, run:

```sh
cargo run --example plan_lines -- tests/fixtures/lines target/lines-candidate.json
```

The boundary libraries use different Unicode data versions; these captures prove
the listed cases, not universal agreement for every Unicode revision. Decorations,
paragraph placement, pagination and painted glyphs remain outside this collection.
These captures are **unreviewed in Microsoft Word**.
