# Custom layout captures

- `inheritance`: forward references, typed and required slots, explicit optional
  overrides, removal, recursive text/margin properties, slot ordering, images,
  ordinary shapes, layout metadata and slide suppression.
- `backgrounds`: recursive layout background fields, source-relative images,
  whole-value slide overrides and missing-image color fallback.
- `replacement`: stable declaration replacement, forward resolution against the
  latest parent, required titles and typed table slots.
- `lua-json`: JSON geometry and colors, Lua-derived layout metadata and content,
  and a typed editable/native chart.

Frozen sources and assets have hashes. Captures contain evaluated JSON, complete
canonical PPTX parts and workbooks, native geometry and independent 144-DPI PNGs.
All eight PDF pages were compared as raster images. Native raster tolerances cover
text-edge antialiasing; geometry remains numerically checked.

JSON inspection currently omits layout declarations. The declaration model and
invalid contracts therefore have direct tests in `tests/presentation_layouts.rs`.
An empty Lua box round-trip currently inserts text and cannot declare a slot;
the Lua case uses directive slot declarations and a Lua-derived layout.

These captures do not establish Microsoft PowerPoint conformance. Real PowerPoint
review remains pending.
