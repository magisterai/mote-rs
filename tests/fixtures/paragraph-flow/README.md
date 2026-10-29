# Paragraph pagination

The 672 [requests](requests.json) and [expected placements](expected.json) cover
initial, top, middle and bottom cursors; one or two columns; empty, single,
mixed-height, zero-height and oversized lines; leading spacing; keep-together;
widow/orphan control; and minimum space reserved beside a preceding block.
Every transition, line placement, final cursor and minimum-start height is compared.

Run `cargo test --test paragraph_flow`. [Capture metadata](capture.json) records
fingerprints and the numeric tolerance. Inputs and expectations remain frozen.

The planner consumes measured heights and frames. It does not measure text,
collapse adjacent paragraph spacing, decide keep-with-next chains, balance final
columns or paint output. Callers supply each page's available area. Invalid areas
and successive areas that cannot make progress return an error; they do not permit
an unbounded loop. Page counts, cursor columns and finite geometry are checked.
Microsoft Word visual review remains **unreviewed**.
