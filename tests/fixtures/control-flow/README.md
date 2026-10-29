# Control-flow captures

Seven frozen cases cover branch selection, inline and block flow, collection
order, local scopes, nested loops, generated tables/lists/notes, Markdown
boundaries, running stories, and section capture. Each retains a complete model
and editable package. Capture manifests fingerprint the authored source.

The `stories` case preserves the current generated-section limitation: explicit
sections are followed by an implicit section containing their returned bodies.
This is a compatibility observation, not recommended authoring practice.
These captures do not establish real-Word visual review.

Native captures retain geometry in points and 72-DPI PNGs. Geometry tolerance is
1e-8; PNG comparison permits a maximum channel difference of 56/255 and mean
foreground difference of 15/255 for font rasterization. Ten independently painted
PDF pages compare exactly at 144 DPI. Every page was visually inspected.
