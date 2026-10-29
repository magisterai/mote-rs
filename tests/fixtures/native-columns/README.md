# Native column captures

These 19 source cases freeze independent text geometry and ordered painting
operations for equal-width columns. They cover natural flow, explicit column and
page breaks, spacing, lists crossing columns, keep behavior, unused columns and
separator extents. `capture.json` records the inputs and expectations by hash.

Separators appear only between columns reached by flow and stop at the deepest
used position on that page. PDF paints a centered stroke; PNG retains a filled
one-point-wide vertical band starting at the resolved separator coordinate.
The standalone `../native-output/columns.mote` exercises three physical pages,
including an explicitly started page with a single used column and no separator.

The final implicit document section flows naturally through its columns; it is
not balanced. Continuous sections and balancing between sections are outside the
current language. These captures are native checks, not Office visual review.
