# Language conformance examples

`specification.json` declares an outcome for each fenced `mote` example in
`docs/specification/`. Example identifiers use the chapter filename and the
one-based position of its Mote code block.

The expected models record the documented content, structure, and attribute
values. They are reviewable expectations, not automatically regenerated output.
The Markdown and directive versions of the quarterly report share one model.

The conformance test reads the source directly from the chapters. Adding or
removing an example requires updating its outcome here. Deferred examples must
parse and fail for their declared missing feature. When one becomes supported,
replace its deferral with an expected model; a newly successful deferred example
fails the test until that expectation is recorded.

These are language and model checks. They provide no rendered-layout or Office
conformance evidence.
