# Markdown deck captures

These frozen sources cover heading and divider boundaries, preamble content,
lower-level headings, automatic tables, task lists, and explicit slides between
implicit segments. `headings` and `separators` contain no presentation declaration:
the caller selects presentation mode. `mixed` retains two level-one headings
inside a single explicit slide.

Consecutive `---` lines without a blank line follow underlined-heading precedence;
the first is the visible text of a level-two heading. Separate divider lines with
blank lines to express repeated boundaries. Empty segments emit no extra slides.

Captures bind source/assets by hash and include the evaluated model, canonical
PPTX package, native geometry and independent 144-DPI PNGs. PDF rasters were also
compared. Text-edge raster tolerances do not relax geometry comparisons. These
captures do not establish PowerPoint conformance; real PowerPoint review is pending.
