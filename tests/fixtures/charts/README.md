# Chart captures

Eleven frozen sources cover all twelve chart kinds, typed and retained data,
category and series tables, every label-position alias, legend normalization,
colors and default palette, chart/plot styling, gridlines, integer gap/overlap,
property overrides and Lua constructors fed by a resolved JSON resource.
They include empty and unequal arrays, duplicate labels, signed/exponent values,
XML escaping, 28 series crossing the Z/AA workbook-column boundary, and charts
in headers, footers, notes, figures and table cells alongside other drawings.

Each capture contains the evaluated model and a manifest of every DOCX part,
including every part inside each embedded workbook. Identical XML parts share
content-addressed files under `parts`; no comparisons omit workbook cells,
formulas, caches, colors, styles or relationships. Source, asset and part hashes
protect the inputs. XML comparisons ignore namespace prefixes, attribute order,
formatting whitespace and volatile core timestamps, following the package tests.

Compatibility observations retained by the captures:

- Workbook rows follow category count. Short series are padded with zero cells;
  values beyond the last category have no cells. Chart caches and formula ranges
  retain the original series lengths.
- Shared-string `count` currently equals unique count, including duplicate labels.
- Doughnut label positions are omitted; other label flags remain editable.
- Unitless dimensions and border widths use EMUs. Chart defaults are 6 by 3.5 inches.
- Word chart tick/legend text has a -10000 DrawingML baseline adjustment.
- Authored gap and overlap values are retained without percentage-range clamping.

Non-finite numeric data is separately tested as an output error. These captures
establish model and package agreement; native chart geometry and Office visual
review are separate. Every case remains unreviewed in Word and PowerPoint.
