# Block break capture

Frozen source, model and XML package parts cover explicit block flow breaks.
A page break that follows a paragraph or a heading ends that paragraph, and a
block line break is an empty paragraph. Column breaks prefix the next paragraph,
heading or quote when possible; pending or consecutive breaks and non-paragraph
blocks retain separate break paragraphs.
Heading spacing and authored breaks remain part of the structural comparison.

Real Microsoft Word layout and pagination review remain unreviewed.

The capture contains no advisory `lastRenderedPageBreak` marker. Authored `w:br`
elements remain compared. This writer does not estimate pagination.
