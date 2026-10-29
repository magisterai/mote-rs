# Native page geometry

The 23 [source requests](requests.json) and [expected geometries](expected.json)
cover named paper sizes, unknown-size fallback, orientation, custom and unitless
sizes, margins, header/footer distances and equal-width columns. The capture
preserves the native minimum top/bottom margin imposed by header/footer distance,
the unused gutter metadata, and the full-content-width fallback for oversized
column gaps. These are observations, not claims of physical Word equivalence.

Run `cargo test --test page_geometry`. [Capture metadata](capture.json) records
fingerprints and numeric tolerance. Invalid physical content areas fail explicitly.
Unequal columns, section transitions and final-column balancing remain outside
this collection. Microsoft Word visual review remains **unreviewed**.
