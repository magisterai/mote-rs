# Block break capture

Frozen source, model and XML package parts cover explicit block flow breaks.
Column breaks prefix the next paragraph, heading or quote when possible; pending
or consecutive breaks and non-paragraph blocks retain separate break paragraphs.
Heading spacing and authored breaks remain part of the structural comparison.

Real Microsoft Word layout and pagination review remain unreviewed.

The capture contains 2 advisory `lastRenderedPageBreak` markers, recorded in
`capture.json` and excluded from comparison. Authored `w:br` elements remain
compared. This writer does not estimate pagination.
