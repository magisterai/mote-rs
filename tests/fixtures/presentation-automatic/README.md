# Automatic slide layout captures

These frozen sources cover standard, wide and custom canvases; leading headings;
paragraphs, nested lists, task markers, block quotes and hyperlinks; separate image
and table frames; shrinking text; and an explicit empty slide.

Captures include evaluated models, complete canonical PPTX packages, native page
geometry and independent 144-DPI native PNG pages. Source and asset hashes bind
these expectations to their inputs. PDF rasters were also compared independently.
Native PNG comparisons allow bounded text-edge antialiasing differences; geometry
must agree numerically. See `tests/pptx.rs` and `tests/presentation_rendering.rs`.

These captures do not establish Microsoft PowerPoint conformance. Real PowerPoint
review is pending.
