# Language conformance examples

`specification.json` declares an outcome for each fenced `mote` example in
`docs/specification/`. Example identifiers use the chapter filename and the
one-based position of its Mote code block.

The expected models record the documented content, structure, and attribute
values. They are reviewable expectations, not automatically regenerated output.
The Markdown and directive versions of the quarterly report share one model.

The conformance test reads the source directly from the chapters. Adding or
removing an example requires updating its outcome here. Successful examples must
evaluate to their declared models. Deliberate invalid examples must parse and
fail evaluation for the rule explained in both the chapter and the fixture's
`invalid` field. Their `diagnostic` field identifies the expected problem.

Every concept used in an example must be defined earlier in the reading order or
immediately before the example. A missing implementation is not an intentional
invalid use: do not add deferrals to keep introductory examples passing. The test
rejects deferral entries. Definitions and explanations still require prose review;
successful execution alone cannot establish that an example has been introduced
properly.

These are language and model checks. They provide no rendered-layout or Office
conformance evidence.
