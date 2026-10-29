# Section and running-story captures

Fourteen frozen cases cover default story changes, local overrides and empty
resets; explicit/implicit section boundaries; natural and authored column flow;
mixed physical page sizes; numbering restarts; body and story fields; multiline
spacing and empty lines; overflow; story links and images; and retained output
limits. `geometry` tests orientation metadata on an already wide canvas;
`page-sizes` separately tests actual portrait/landscape dimension changes.

Each `captured/<case>/capture.json` inventories every original DOCX package part
and the source hash. Models and complete packages compare with only the shared
normalizations: numeric representation, XML formatting, volatile timestamps, and
advisory rendered-page markers. Story relationships and binary media compare too.
`native-capture.json` records independent geometry and PNG hashes. Tests compare
all line fragments, borders and image bounds within 1e-8 points. Independent PDFs
were rendered at 144 DPI: all 41 pages match every pixel.

Native PNG uses installed Helvetica outlines at 72 DPI. Its font-edge tolerance
is 56/255 maximum channel difference and 11/255 mean foreground difference;
observed maxima are 52/255 and less than 11/255 respectively. Portable geometry
checks disable installed-font discovery. No proprietary font files are included.
All 41 native PDF pages were visually inspected. Microsoft Word review remains
unreviewed; these captures do not establish Office conformance.

The deliberate limit cases remain intact: native page fields use decimal text,
vertical section alignment is ignored, and parity/next-column section starts
advance to a page. Story images use visible placeholders. Overflow removes whole
trailing lines with a diagnostic; `continuous` has less than one line of reserved
story height, so its header/footer are omitted. DOCX retains the editable mappings.
